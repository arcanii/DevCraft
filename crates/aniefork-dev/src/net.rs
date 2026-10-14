//! Carrying a C program's packets: networking, stage N2 (the owner, 2026-10-14: "the user can turn
//! on networking explicitly", and "Then on to networking").
//!
//! A page can't send the kind of packets games send (UDP). GridCraft can, on its behalf: the
//! program hands its loader a packet and where it goes ([`HEADER`], `<gridcraft/net.h>`), the
//! loader hands it to the bridge with the page's other calls, and the bridge sends it and keeps
//! what comes back for the loader to collect. This is the part that sends and takes in: a
//! [`Carrier`], one for each project whose network is on ([`crate::bridge`] decides that).
//!
//! What it holds a program to:
//!
//! - **It sends only where the project names** (`[network] allow`, as the person turned it on for
//!   them), and back to an address that wrote to it, which can only be one of those hosts.
//! - **A name is checked by the number it turns out to be.** A name that is not one of the person's
//!   own machine or network ([`crate::manifest::Address::is_local`]) and stands for a number that
//!   is, is not sent to: a name can be pointed anywhere, and the person was told the address was
//!   out on the internet.
//! - **It takes in only from the hosts it has sent to**, and, once its program asks to listen
//!   (`net_listen`), on the port the project names, from the hosts the project names: what a
//!   program that hosts a game needs. Whatever else arrives is dropped.
//! - **So much and no more**: a packet is at most [`MAX_PACKET`] bytes, at most [`MAX_A_SECOND`]
//!   packets and [`MAX_BYTES_A_SECOND`] bytes go out in a second, and at most [`MAX_A_REPLY`]
//!   come back to the loader at a time. What is over is dropped, as a network drops packets.
//!
//! Each refusal is said once, for the Developer console. Packets travel between the loader and
//! the bridge as [`to_bytes`] writes them.

use std::collections::HashSet;
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

use crate::manifest::Address;

/// The most bytes in one packet.
pub const MAX_PACKET: usize = 4096;
/// The most packets sent in a second.
pub const MAX_A_SECOND: u32 = 400;
/// The most bytes sent in a second.
pub const MAX_BYTES_A_SECOND: usize = 512 * 1024;
/// The most packets handed back to the loader in one exchange; the rest wait for the next.
pub const MAX_A_REPLY: usize = 256;
/// The most packets the loader hands over in one exchange.
pub const MAX_A_REQUEST: usize = 256;
/// The longest address a packet may be sent to, as text.
const MAX_ADDRESS: usize = 300;
/// How long what a name stands for is kept before it is looked up again.
const NAMES_KEPT: Duration = Duration::from_secs(60);
/// The most refusals kept to be said.
const MAX_SAID: usize = 32;

/// The clock, read only where the bridge runs (off wasm, where reading it traps).
#[allow(clippy::disallowed_methods)]
pub(crate) fn now() -> Instant {
    Instant::now()
}

/// `<gridcraft/net.h>`: what a program asks of its loader for the network. Supplied to every
/// build, as `<GLES3/gl3.h>` is.
pub const HEADER: &str = r#"/*
 * GridCraft's C Development Kit: the network, as GridCraft carries it for a program.
 *
 * A page can't send the packets games send (UDP); GridCraft can, on the program's behalf. A
 * project asks for "network" and names where it would reach in its project.toml:
 *
 *   permissions = ["network"]
 *   [network]
 *   allow = ["quake.example.org:26000"]
 *
 * and the person turns its network on for those addresses, apart from trusting it. GridCraft
 * then sends a packet only to those addresses, or back to an address that wrote to it, and
 * takes in only what comes from the hosts it has sent to. A packet is at most NET_PACKET bytes.
 *
 * None of these waits: a packet is sent soon after net_send, and one that has come is there for
 * net_receive the next time it is asked, as a game asks once a frame. The file is GridCraft's,
 * written for this kit, and is not a project's to change: GridCraft supplies it when it builds.
 */
#ifndef GRIDCRAFT_NET_H
#define GRIDCRAFT_NET_H

#ifdef __cplusplus
extern "C" {
#endif

/* The most bytes in one packet. */
#define NET_PACKET 4096

#define GRIDCRAFT_NET(name) __attribute__((import_module("net"), import_name(#name)))

/* Whether the network is on: 1 it is, -1 it is off (or the project names no address), 0 not
   known yet (GridCraft has not answered). */
GRIDCRAFT_NET(net_status) int net_status(void);

/* Sends `length` bytes to `address`, "host:port" as the project names it, or as net_receive
   said who wrote. 0 when it is on its way; -1 when the network is off; -2 when it is too long,
   or too much is waiting to go. Whether it may go there is GridCraft's to decide: a packet to
   anywhere else is dropped, and the Developer console says so. */
GRIDCRAFT_NET(net_send) int net_send(const char *address, const void *data, int length);

/* Takes in packets on the port the project names to listen on ([network] listen), from the
   hosts it names, before the program has written to them: what a program that hosts a game
   asks for. Ask once a frame until it says: 1 GridCraft listens, and sends from that port too;
   0 not known yet; -1 it can't (the project names no port, the network is off, or the port is
   taken: the Developer console says which). */
GRIDCRAFT_NET(net_listen) int net_listen(void);

/* The next packet that came: its bytes into `data` (room for `room`), and who sent it into
   `from` ("number:port", closed with a zero; room for `from_room`). Its length, which may be 0;
   -1 when none has come; -2 when it was longer than `room`, and is dropped. */
GRIDCRAFT_NET(net_receive) int net_receive(char *from, int from_room, void *data, int room);

#undef GRIDCRAFT_NET

#ifdef __cplusplus
}
#endif

#endif
"#;

/// A packet and the address it goes to or came from, as text.
pub type Packet = (String, Vec<u8>);

/// Reads the packets of an exchange: each is its address's length (two bytes, the high one
/// first), the address, its data's length (two bytes), the data.
pub fn packets(bytes: &[u8]) -> Result<Vec<Packet>, String> {
    /// The next `length` bytes, from `at` on.
    fn take<'a>(bytes: &'a [u8], at: &mut usize, length: usize) -> Result<&'a [u8], String> {
        let part = bytes.get(*at..at.saturating_add(length)).ok_or("the packets end too soon")?;
        *at += length;
        Ok(part)
    }
    /// A length: two bytes, the high one first.
    fn length(bytes: &[u8], at: &mut usize) -> Result<usize, String> {
        let two = take(bytes, at, 2)?;
        Ok(usize::from(u16::from_be_bytes([two.first().copied().unwrap_or(0), two.get(1).copied().unwrap_or(0)])))
    }
    let mut found = Vec::new();
    let mut at = 0;
    while at < bytes.len() {
        if found.len() >= MAX_A_REQUEST {
            return Err(format!("more than {MAX_A_REQUEST} packets at once"));
        }
        let address_length = length(bytes, &mut at)?;
        let address = String::from_utf8(take(bytes, &mut at, address_length)?.to_vec()).map_err(|_| "an address is not text")?;
        let data_length = length(bytes, &mut at)?;
        let data = take(bytes, &mut at, data_length)?.to_vec();
        found.push((address, data));
    }
    Ok(found)
}

/// Writes packets as [`packets`] reads them. A packet whose address or data is too long to be
/// written so is left out.
pub fn to_bytes(packets: &[Packet]) -> Vec<u8> {
    let mut out = Vec::new();
    for (address, data) in packets {
        let (Ok(address_length), Ok(data_length)) = (u16::try_from(address.len()), u16::try_from(data.len())) else { continue };
        out.extend_from_slice(&address_length.to_be_bytes());
        out.extend_from_slice(address.as_bytes());
        out.extend_from_slice(&data_length.to_be_bytes());
        out.extend_from_slice(data);
    }
    out
}

/// Whether a number is one of the person's own machine or network: the machine itself, an
/// address kept for a home or office network, one a network gives itself, or none at all.
pub fn is_local_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => {
            v4.is_loopback() || v4.is_private() || v4.is_link_local() || v4.is_unspecified() || v4.is_broadcast() || v4.octets()[0] == 0
        }
        IpAddr::V6(v6) => {
            let first = v6.segments()[0];
            v6.is_loopback()
                || v6.is_unspecified()
                || (first & 0xfe00) == 0xfc00 // unique local
                || (first & 0xffc0) == 0xfe80 // link local
                || v6.to_ipv4_mapped().is_some_and(|v4| is_local_ip(&IpAddr::V4(v4)))
        }
    }
}

/// Looks up what a host stands for: the system's own lookup.
pub fn system_lookup(host: &str, port: u16) -> Vec<SocketAddr> {
    use std::net::ToSocketAddrs;
    (host, port).to_socket_addrs().map(Iterator::collect).unwrap_or_default()
}

/// The kinds of socket a carrier sends from: the machine's own address can be reached only from
/// a socket on it, and the rest from one on every address.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Bound {
    Here4,
    Here6,
    Out4,
    Out6,
}

impl Bound {
    fn of(to: &SocketAddr) -> Bound {
        match (to.ip().is_loopback(), to.is_ipv4()) {
            (true, true) => Bound::Here4,
            (true, false) => Bound::Here6,
            (false, true) => Bound::Out4,
            (false, false) => Bound::Out6,
        }
    }

    fn bind(self) -> std::io::Result<UdpSocket> {
        let ip = match self {
            Bound::Here4 => IpAddr::V4(Ipv4Addr::LOCALHOST),
            Bound::Here6 => IpAddr::V6(Ipv6Addr::LOCALHOST),
            Bound::Out4 => IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            Bound::Out6 => IpAddr::V6(Ipv6Addr::UNSPECIFIED),
        };
        let socket = UdpSocket::bind(SocketAddr::new(ip, 0))?;
        socket.set_nonblocking(true)?;
        Ok(socket)
    }
}

/// One project's network, as GridCraft carries it.
pub struct Carrier {
    allowed: Vec<Address>,
    /// What each name stood for when it was last looked up, and when.
    looked_up: Vec<(Address, Vec<SocketAddr>, Instant)>,
    /// The hosts it has sent to: only they may write to it.
    sent_to: HashSet<IpAddr>,
    /// Who wrote to it: it may answer them.
    heard: HashSet<SocketAddr>,
    sockets: Vec<(Bound, UdpSocket)>,
    /// The port the project names to listen on; and, once the program asked, the socket on it,
    /// whether it is on this machine alone, and the hosts that may write to it.
    listen: Option<u16>,
    listener: Option<(u16, UdpSocket, bool)>,
    welcome: HashSet<IpAddr>,
    /// This second's sending: when it began, packets, bytes.
    second: (Instant, u32, usize),
    /// What was refused and has not been said yet; and what has been said, so it is said once.
    to_say: Vec<String>,
    said: HashSet<String>,
}

impl Carrier {
    /// A carrier for the addresses a project names, and the port it may listen on, as its
    /// network was turned on for them.
    pub fn new(allowed: Vec<Address>, listen: Option<u16>) -> Carrier {
        Carrier {
            allowed,
            listen,
            listener: None,
            welcome: HashSet::new(),
            looked_up: Vec::new(),
            sent_to: HashSet::new(),
            heard: HashSet::new(),
            sockets: Vec::new(),
            second: (now(), 0, 0),
            to_say: Vec::new(),
            said: HashSet::new(),
        }
    }

    /// The addresses it carries for.
    pub fn allowed(&self) -> &[Address] {
        &self.allowed
    }

    /// The port it may listen on.
    pub fn listen_port(&self) -> Option<u16> {
        self.listen
    }

    /// Takes in packets on the port the project names, from the hosts it names, before the
    /// program has written to them; and sends from that port from then on, so that its answers
    /// come from where it was written to. On this machine alone when every host it names is this
    /// machine. The port it listens on, or why not (said once, too).
    pub fn listen(&mut self, lookup: &dyn Fn(&str, u16) -> Vec<SocketAddr>) -> Result<u16, String> {
        if let Some((port, _, _)) = &self.listener {
            return Ok(*port);
        }
        let Some(port) = self.listen else {
            let why = "It did not listen: the project names no port to listen on ([network] listen).".to_string();
            self.refuse(why.clone());
            return Err(why);
        };
        let mut welcome = HashSet::new();
        for address in self.allowed.clone() {
            if let Ok(found) = self.stands_for(&address, lookup) {
                welcome.extend(found.iter().map(SocketAddr::ip).filter(IpAddr::is_ipv4));
            }
        }
        if welcome.is_empty() {
            let why = format!("It did not listen on port {port}: none of the hosts the project names can be reached.");
            self.refuse(why.clone());
            return Err(why);
        }
        let here = welcome.iter().all(IpAddr::is_loopback);
        let ip = if here { Ipv4Addr::LOCALHOST } else { Ipv4Addr::UNSPECIFIED };
        match UdpSocket::bind((ip, port)).and_then(|socket| socket.set_nonblocking(true).map(|()| socket)) {
            Ok(socket) => {
                self.welcome = welcome;
                self.listener = Some((port, socket, here));
                Ok(port)
            }
            Err(e) => {
                let why = format!("It did not listen on port {port}: {e}.");
                self.refuse(why.clone());
                Err(why)
            }
        }
    }

    /// What a named address stands for, as far as it may be reached: looked up again after a
    /// while, and without the person's own machine or network for a name they were told is out
    /// on the internet. Why not, when nothing it stands for may be reached.
    fn stands_for(&mut self, address: &Address, lookup: &dyn Fn(&str, u16) -> Vec<SocketAddr>) -> Result<Vec<SocketAddr>, String> {
        let now = now();
        let kept = self.looked_up.iter().position(|(was, _, when)| was == address && now.duration_since(*when) < NAMES_KEPT);
        let found = match kept {
            Some(i) => self.looked_up.get(i).map(|(_, found, _)| found.clone()).unwrap_or_default(),
            None => {
                let found = lookup(&address.host, address.port);
                self.looked_up.retain(|(was, _, _)| was != address);
                self.looked_up.push((address.clone(), found.clone(), now));
                found
            }
        };
        if found.is_empty() {
            return Err("the name stands for nothing that can be reached".into());
        }
        // A name the person was told is out on the internet is not let reach their own network.
        let fit: Vec<SocketAddr> = found.into_iter().filter(|at| address.is_local() || !is_local_ip(&at.ip())).collect();
        if fit.is_empty() {
            return Err("the name stands for an address on your own machine or network, which the project does not say it reaches".into());
        }
        Ok(fit)
    }

    /// What was refused since it was last asked, each said once.
    pub fn refusals(&mut self) -> Vec<String> {
        std::mem::take(&mut self.to_say)
    }

    fn refuse(&mut self, why: String) {
        if self.said.len() < MAX_SAID && self.said.insert(why.clone()) {
            self.to_say.push(why);
        }
    }

    /// Sends what the loader handed over, and takes in what has come since the last exchange.
    /// `lookup` says what a name stands for ([`system_lookup`], but for tests).
    pub fn exchange(&mut self, outgoing: Vec<Packet>, lookup: &dyn Fn(&str, u16) -> Vec<SocketAddr>) -> Vec<Packet> {
        for (to, data) in outgoing {
            if let Some(at) = self.where_to(&to, lookup) {
                self.send(at, &to, &data);
            }
        }
        self.take_in()
    }

    /// Where a packet for `to` goes, or `None` (and why, to be said) when it may go nowhere.
    fn where_to(&mut self, to: &str, lookup: &dyn Fn(&str, u16) -> Vec<SocketAddr>) -> Option<SocketAddr> {
        let shown: String = to.chars().take(80).collect();
        if to.len() > MAX_ADDRESS {
            self.refuse(format!("A packet to {shown}... was not sent: that is not an address."));
            return None;
        }
        // An answer to whoever wrote to it.
        if let Ok(at) = to.parse::<SocketAddr>()
            && self.heard.contains(&at)
        {
            return Some(at);
        }
        let named = Address::parse(to).ok().filter(|address| self.allowed.contains(address));
        let Some(address) = named else {
            self.refuse(format!("A packet to {shown} was not sent: the project does not name that address, and nothing there wrote to it."));
            return None;
        };
        match self.stands_for(&address, lookup) {
            Ok(found) => found.iter().find(|at| at.is_ipv4()).or_else(|| found.first()).copied(),
            Err(why) => {
                self.refuse(format!("A packet to {shown} was not sent: {why}."));
                None
            }
        }
    }

    fn send(&mut self, at: SocketAddr, to: &str, data: &[u8]) {
        if data.len() > MAX_PACKET {
            self.refuse(format!("A packet to {to} was not sent: it is {} bytes long, and the most is {MAX_PACKET}.", data.len()));
            return;
        }
        let now = now();
        if now.duration_since(self.second.0) >= Duration::from_secs(1) {
            self.second = (now, 0, 0);
        }
        if self.second.1 >= MAX_A_SECOND || self.second.2.saturating_add(data.len()) > MAX_BYTES_A_SECOND {
            self.refuse(format!(
                "Packets were dropped: a program may send {MAX_A_SECOND} packets, or {} KB, in a second.",
                MAX_BYTES_A_SECOND / 1024
            ));
            return;
        }
        // While it listens, it sends from that port: whoever it hosts wrote there, and its
        // answers are to come from there.
        let from_listener = self.listener.as_ref().is_some_and(|(_, _, here)| at.is_ipv4() && (!here || at.ip().is_loopback()));
        if from_listener {
            if let Some((_, socket, _)) = &self.listener {
                match socket.send_to(data, at) {
                    Ok(_) => {
                        self.second.1 += 1;
                        self.second.2 = self.second.2.saturating_add(data.len());
                        self.sent_to.insert(at.ip());
                    }
                    Err(e) => self.refuse(format!("A packet to {to} was not sent: {e}.")),
                }
            }
            return;
        }
        let kind = Bound::of(&at);
        if !self.sockets.iter().any(|(was, _)| *was == kind) {
            match kind.bind() {
                Ok(socket) => self.sockets.push((kind, socket)),
                Err(e) => {
                    self.refuse(format!("A packet to {to} was not sent: GridCraft could not open the network ({e})."));
                    return;
                }
            }
        }
        let Some((_, socket)) = self.sockets.iter().find(|(was, _)| *was == kind) else { return };
        match socket.send_to(data, at) {
            Ok(_) => {
                self.second.1 += 1;
                self.second.2 = self.second.2.saturating_add(data.len());
                self.sent_to.insert(at.ip());
            }
            Err(e) => self.refuse(format!("A packet to {to} was not sent: {e}.")),
        }
    }

    /// What has come from the hosts it sent to, and while it listens from the hosts it names,
    /// oldest first; what came from anywhere else is dropped.
    fn take_in(&mut self) -> Vec<Packet> {
        let mut found = Vec::new();
        let mut buffer = vec![0u8; MAX_PACKET.max(65_536)];
        let listening = self.listener.iter().map(|(_, socket, _)| socket);
        for socket in listening.chain(self.sockets.iter().map(|(_, socket)| socket)) {
            while found.len() < MAX_A_REPLY {
                match socket.recv_from(&mut buffer) {
                    Ok((length, from)) => {
                        let known = self.sent_to.contains(&from.ip()) || self.welcome.contains(&from.ip());
                        if !known || length > MAX_PACKET {
                            continue;
                        }
                        self.heard.insert(from);
                        found.push((from.to_string(), buffer.get(..length).unwrap_or_default().to_vec()));
                    }
                    // Nothing more has come; or a packet sent before was turned away there,
                    // which a program learns of as a game does, by no answer.
                    Err(e) if e.kind() == ErrorKind::WouldBlock => break,
                    Err(e) if e.kind() == ErrorKind::ConnectionReset => continue,
                    Err(_) => break,
                }
            }
        }
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(texts: &[&str]) -> Vec<Address> {
        texts.iter().map(|text| Address::parse(text).unwrap()).collect()
    }

    /// A socket on this machine that answers each packet it gets with the packet, turned round.
    fn echo() -> (UdpSocket, u16) {
        let socket = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let port = socket.local_addr().unwrap().port();
        (socket, port)
    }

    fn answer_one(socket: &UdpSocket) -> SocketAddr {
        let mut buffer = [0u8; 8192];
        let (length, from) = socket.recv_from(&mut buffer).unwrap();
        let mut turned = buffer[..length].to_vec();
        turned.reverse();
        socket.send_to(&turned, from).unwrap();
        from
    }

    /// Exchanges until something comes back, or a few seconds have gone.
    fn collect(carrier: &mut Carrier, lookup: &dyn Fn(&str, u16) -> Vec<SocketAddr>) -> Vec<Packet> {
        let deadline = now() + Duration::from_secs(5);
        loop {
            let found = carrier.exchange(Vec::new(), lookup);
            if !found.is_empty() || now() > deadline {
                return found;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    #[test]
    fn packets_travel_between_the_loader_and_the_bridge_as_written() {
        let sent = vec![("example.org:26000".to_string(), b"hello".to_vec()), ("127.0.0.1:9".to_string(), Vec::new())];
        let bytes = to_bytes(&sent);
        assert_eq!(&bytes[..2], &[0, 17]);
        assert_eq!(packets(&bytes).unwrap(), sent);
        assert_eq!(packets(&[]).unwrap(), Vec::<Packet>::new());
        // What ends too soon, or isn't text, is not packets.
        assert!(packets(&bytes[..bytes.len() - 1]).is_err());
        assert!(packets(&[0, 1, 0xff, 0, 0]).is_err());
        assert!(packets(&[0, 9, b'a']).is_err());
        let many: Vec<Packet> = (0..=MAX_A_REQUEST).map(|_| ("a:1".to_string(), vec![1])).collect();
        assert!(packets(&to_bytes(&many)).is_err(), "no more than so many at once");
    }

    #[test]
    fn the_persons_own_machine_and_network_are_told_by_their_numbers() {
        for local in [
            "127.0.0.1",
            "10.1.2.3",
            "172.16.0.9",
            "192.168.1.20",
            "169.254.3.4",
            "0.0.0.0",
            "255.255.255.255",
            "::1",
            "fd00::5",
            "fe80::1",
            "::ffff:192.168.0.1",
        ] {
            assert!(is_local_ip(&local.parse().unwrap()), "{local}");
        }
        for out in ["93.184.216.34", "8.8.8.8", "172.32.0.1", "2001:4860:4860::8888", "::ffff:8.8.8.8"] {
            assert!(!is_local_ip(&out.parse().unwrap()), "{out}");
        }
    }

    #[test]
    fn a_packet_goes_where_the_project_names_and_its_answer_comes_back() {
        let (far, port) = echo();
        let mut carrier = Carrier::new(named(&[&format!("127.0.0.1:{port}")]), None);
        let found = carrier.exchange(vec![(format!("127.0.0.1:{port}"), b"ping".to_vec())], &system_lookup);
        let from = answer_one(&far);
        assert!(from.ip().is_loopback(), "sent from this machine to this machine");
        let mut back = found;
        if back.is_empty() {
            back = collect(&mut carrier, &system_lookup);
        }
        assert_eq!(back, vec![(format!("127.0.0.1:{port}"), b"gnip".to_vec())]);
        // It may answer whoever wrote to it, by the number they wrote from.
        carrier.exchange(vec![(format!("127.0.0.1:{port}"), b"again".to_vec())], &system_lookup);
        answer_one(&far);
        assert_eq!(collect(&mut carrier, &system_lookup), vec![(format!("127.0.0.1:{port}"), b"niaga".to_vec())]);
        assert!(carrier.refusals().is_empty());
    }

    #[test]
    fn nothing_goes_anywhere_else_and_nothing_comes_in_from_anywhere_else() {
        let (far, port) = echo();
        let (other, other_port) = echo();
        let mut carrier = Carrier::new(named(&[&format!("127.0.0.1:{port}")]), None);
        // Not named: not sent, and said once.
        carrier
            .exchange(vec![(format!("127.0.0.1:{other_port}"), b"no".to_vec()), (format!("127.0.0.1:{other_port}"), b"no".to_vec())], &system_lookup);
        let said = carrier.refusals();
        assert_eq!(said.len(), 1, "{said:?}");
        assert!(said[0].contains("does not name that address"), "{said:?}");
        other.set_read_timeout(Some(Duration::from_millis(200))).unwrap();
        assert!(other.recv_from(&mut [0u8; 16]).is_err(), "nothing arrived there");
        // A host it did not send to writes to it (another of this machine's own numbers): dropped.
        carrier.exchange(vec![(format!("127.0.0.1:{port}"), b"ping".to_vec())], &system_lookup);
        let ours = answer_one(&far);
        let stranger = UdpSocket::bind((Ipv4Addr::new(127, 0, 0, 2), 0)).unwrap();
        stranger.send_to(b"let me in", ours).unwrap();
        let back = collect(&mut carrier, &system_lookup);
        std::thread::sleep(Duration::from_millis(100));
        let later = carrier.exchange(Vec::new(), &system_lookup);
        let all: Vec<Packet> = back.into_iter().chain(later).collect();
        assert_eq!(all, vec![(format!("127.0.0.1:{port}"), b"gnip".to_vec())], "only the answer of the host it sent to");
        // The host it sent to may write from another port (a game's server does), and be answered there.
        let (same_host, same_host_port) = echo();
        same_host.send_to(b"new port", ours).unwrap();
        assert_eq!(collect(&mut carrier, &system_lookup), vec![(format!("127.0.0.1:{same_host_port}"), b"new port".to_vec())]);
        carrier.exchange(vec![(format!("127.0.0.1:{same_host_port}"), b"ok".to_vec())], &system_lookup);
        answer_one(&same_host);
        assert_eq!(collect(&mut carrier, &system_lookup), vec![(format!("127.0.0.1:{same_host_port}"), b"ko".to_vec())]);
        // Not an address at all, or too long a packet, or too much: not sent.
        carrier.exchange(vec![("nowhere".to_string(), b"x".to_vec())], &system_lookup);
        carrier.exchange(vec![(format!("127.0.0.1:{port}"), vec![0u8; MAX_PACKET + 1])], &system_lookup);
        let said = carrier.refusals();
        assert!(said.iter().any(|why| why.contains("does not name")) && said.iter().any(|why| why.contains("the most is 4096")), "{said:?}");
    }

    #[test]
    fn a_name_that_turns_out_to_stand_for_the_persons_own_network_is_not_sent_to() {
        let (far, port) = echo();
        // The project names a host out on the internet, which stands, today, for this machine.
        let pointed_home = move |_: &str, _: u16| vec![SocketAddr::from((Ipv4Addr::LOCALHOST, port))];
        let mut carrier = Carrier::new(named(&[&format!("game.example.org:{port}")]), None);
        carrier.exchange(vec![(format!("game.example.org:{port}"), b"ping".to_vec())], &pointed_home);
        let said = carrier.refusals();
        assert!(said.iter().any(|why| why.contains("on your own machine or network")), "{said:?}");
        far.set_read_timeout(Some(Duration::from_millis(200))).unwrap();
        assert!(far.recv_from(&mut [0u8; 16]).is_err(), "nothing arrived");
        // A name the project says is on the person's network may stand for it.
        let mut home = Carrier::new(named(&[&format!("box.lan:{port}")]), None);
        home.exchange(vec![(format!("box.lan:{port}"), b"ping".to_vec())], &pointed_home);
        far.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        answer_one(&far);
        assert_eq!(collect(&mut home, &pointed_home), vec![(format!("127.0.0.1:{port}"), b"gnip".to_vec())]);
        // A name that stands for nothing: not sent, and said.
        let mut lost = Carrier::new(named(&["gone.example.org:1"]), None);
        lost.exchange(vec![("gone.example.org:1".to_string(), b"x".to_vec())], &|_: &str, _: u16| Vec::new());
        assert!(lost.refusals().iter().any(|why| why.contains("stands for nothing")));
    }

    #[test]
    fn so_much_is_sent_in_a_second_and_no_more() {
        let (far, port) = echo();
        let mut carrier = Carrier::new(named(&[&format!("127.0.0.1:{port}")]), None);
        let many: Vec<Packet> = (0..MAX_A_REQUEST).map(|_| (format!("127.0.0.1:{port}"), vec![7u8; 8])).collect();
        carrier.exchange(many.clone(), &system_lookup);
        carrier.exchange(many, &system_lookup);
        assert_eq!(carrier.second.1, MAX_A_SECOND, "the rest of the second's were dropped");
        assert!(carrier.refusals().iter().any(|why| why.contains("in a second")));
        drop(far);
    }

    /// A port on this machine that nobody is listening on now.
    fn free_port() -> u16 {
        UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap().local_addr().unwrap().port()
    }

    #[test]
    fn a_program_that_hosts_takes_in_from_the_hosts_it_names_on_the_port_it_names() {
        let port = free_port();
        // Without a port named, it does not listen, and says why.
        let mut client = Carrier::new(named(&["127.0.0.1:26000"]), None);
        assert!(client.listen(&system_lookup).unwrap_err().contains("names no port"));
        // With one: on this machine alone, as the only host named is this machine.
        let mut host = Carrier::new(named(&[&format!("127.0.0.1:{port}")]), Some(port));
        assert_eq!(host.listen(&system_lookup), Ok(port));
        assert_eq!(host.listen(&system_lookup), Ok(port), "once is enough");
        // A player writes to it before it has written to them, from a port of their own.
        let player = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        player.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let player_at = player.local_addr().unwrap();
        player.send_to(b"connect me", (Ipv4Addr::LOCALHOST, port)).unwrap();
        let came = collect(&mut host, &system_lookup);
        assert_eq!(came, vec![(player_at.to_string(), b"connect me".to_vec())]);
        // Its answer goes back from the port it listens on, which is where the player wrote.
        host.exchange(vec![(player_at.to_string(), b"welcome".to_vec())], &system_lookup);
        let mut buffer = [0u8; 64];
        let (length, from) = player.recv_from(&mut buffer).unwrap();
        assert_eq!((&buffer[..length], from.port()), (b"welcome".as_slice(), port));
        // A host it does not name is not let in (another of this machine's own numbers).
        let stranger = UdpSocket::bind((Ipv4Addr::new(127, 0, 0, 2), 0)).unwrap();
        stranger.send_to(b"let me in", (Ipv4Addr::LOCALHOST, port)).unwrap();
        std::thread::sleep(Duration::from_millis(100));
        assert!(host.exchange(Vec::new(), &system_lookup).is_empty());
        // The port is taken: a second carrier can't listen there, and says so.
        let mut second = Carrier::new(named(&[&format!("127.0.0.1:{port}")]), Some(port));
        assert!(second.listen(&system_lookup).unwrap_err().contains(&format!("did not listen on port {port}")));
        assert!(second.refusals().iter().any(|why| why.contains("did not listen")));
    }

    #[test]
    fn the_header_declares_what_the_loader_answers() {
        assert!(HEADER.is_ascii() && !HEADER.contains('\r'));
        for call in ["net_status", "net_send", "net_receive", "net_listen"] {
            assert!(HEADER.contains(&format!("GRIDCRAFT_NET({call}) int {call}(")), "{call}");
            assert!(crate::c_program::LOADER.contains(&format!("      {call}(")), "{call} is not answered by the loader");
        }
        assert!(HEADER.contains(&format!("#define NET_PACKET {MAX_PACKET}")));
        assert!(crate::c_program::LOADER.contains(&format!("NET_PACKET = {MAX_PACKET}")));
    }
}
