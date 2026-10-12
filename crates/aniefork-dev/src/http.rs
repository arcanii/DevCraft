//! Reading one HTTP request off a loopback connection: what the bridge and the Development
//! server share. Written on the standard library, with every size bounded; a connection carries
//! one request.

use std::io::Read;
use std::net::TcpStream;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

/// The longest request line and headers read (bytes).
pub(crate) const MAX_HEAD: usize = 16 * 1024;
pub(crate) const MAX_HEADERS: usize = 100;

pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// 64 hexadecimal digits from the operating system's source of randomness.
pub(crate) fn token() -> Result<String, String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|e| format!("the system gave no random numbers ({e})"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

/// Whether two texts are the same, taking as long whichever byte differs.
pub(crate) fn same(a: &str, b: &str) -> bool {
    a.len() == b.len() && a.bytes().zip(b.bytes()).fold(0u8, |differs, (x, y)| differs | (x ^ y)) == 0
}

pub(crate) struct Incoming {
    pub method: String,
    pub target: String,
    /// Names in lower case.
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Incoming {
    /// The header's value; an error when it was sent more than once, which no honest client does.
    pub fn header(&self, name: &str) -> Result<Option<&str>, ()> {
        let mut values = self.headers.iter().filter(|(n, _)| n == name).map(|(_, v)| v.as_str());
        let first = values.next();
        if values.next().is_some() { Err(()) } else { Ok(first) }
    }
}

/// Why a request was not read, as the answer to give for it.
pub(crate) struct Refusal {
    pub status: u16,
    pub reason: &'static str,
    pub text: &'static str,
}

impl Refusal {
    pub fn bad() -> Refusal {
        Refusal { status: 400, reason: "Bad Request", text: "That request makes no sense." }
    }
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[allow(clippy::disallowed_methods)]
fn now() -> Instant {
    Instant::now()
}

/// Reads one request, with a body of at most `max_body` bytes, all of it within `within`: a
/// client that sends a byte now and then holds a connection no longer than that. `Err(None)`:
/// the connection gave nothing to answer.
pub(crate) fn read(stream: &mut TcpStream, max_body: usize, within: Duration) -> Result<Incoming, Option<Refusal>> {
    let began = now();
    let late = || now().saturating_duration_since(began) > within;
    let mut buffer = Vec::with_capacity(2048);
    let mut chunk = [0u8; 4096];
    let end = loop {
        if late() {
            return Err(None);
        }
        let found = find(&buffer, b"\r\n\r\n");
        if found.unwrap_or(buffer.len()) > MAX_HEAD {
            return Err(Some(Refusal { status: 431, reason: "Request Header Fields Too Large", text: "That request's headers are too long." }));
        }
        if let Some(at) = found {
            break at;
        }
        let n = stream.read(&mut chunk).map_err(|_| None)?;
        if n == 0 {
            return Err(None);
        }
        buffer.extend_from_slice(chunk.get(..n).unwrap_or(&[]));
    };
    let head = std::str::from_utf8(buffer.get(..end).unwrap_or(&[])).map_err(|_| Some(Refusal::bad()))?;
    let mut lines = head.split("\r\n");
    let mut first = lines.next().unwrap_or("").split(' ');
    let (method, target, version) = (first.next().unwrap_or(""), first.next().unwrap_or(""), first.next().unwrap_or(""));
    if method.is_empty() || !target.starts_with('/') || !version.starts_with("HTTP/1.") || first.next().is_some() {
        return Err(Some(Refusal::bad()));
    }
    let mut headers = Vec::new();
    for line in lines {
        let Some((name, value)) = line.split_once(':') else { return Err(Some(Refusal::bad())) };
        if name.is_empty() || name.contains(char::is_whitespace) || headers.len() >= MAX_HEADERS {
            return Err(Some(Refusal::bad()));
        }
        headers.push((name.to_ascii_lowercase(), value.trim().to_string()));
    }
    let mut request = Incoming { method: method.to_string(), target: target.to_string(), headers, body: Vec::new() };
    // A body is taken by its stated length and no other way.
    if request.header("transfer-encoding") != Ok(None) {
        return Err(Some(Refusal::bad()));
    }
    let length = match request.header("content-length") {
        Ok(None) => 0,
        Ok(Some(text)) => text.parse::<usize>().map_err(|_| Some(Refusal::bad()))?,
        Err(()) => return Err(Some(Refusal::bad())),
    };
    if length > max_body {
        return Err(Some(Refusal { status: 413, reason: "Content Too Large", text: "That call is too large." }));
    }
    let mut body = buffer.get(end + 4..).unwrap_or(&[]).to_vec();
    body.truncate(length);
    while body.len() < length {
        if late() {
            return Err(None);
        }
        let n = stream.read(&mut chunk).map_err(|_| None)?;
        if n == 0 {
            return Err(None);
        }
        let room = length - body.len();
        body.extend_from_slice(chunk.get(..n.min(room)).unwrap_or(&[]));
    }
    request.body = body;
    Ok(request)
}
