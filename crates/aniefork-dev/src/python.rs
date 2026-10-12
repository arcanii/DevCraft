//! Running a trusted module's Python scripts, in a process of their own.
//!
//! `run` is the only way in, and it decides for itself whether the module may run anything: it
//! reads the module and the user's decisions fresh from disk on every call. A module that isn't
//! trusted with `python`, has changed since it was trusted, or names a script that isn't one of
//! its own files, is refused before any process exists.
//!
//! A script is a file in the module's scripts folder with a `main(data)` function. GridCraft
//! starts the user's own Python with a small runner, which stays alive between calls (so a
//! module's imports, pandas for instance, are paid for once) and speaks one line of JSON per
//! request and per reply. Each module gets its own process, started again whenever the module's
//! files change.
//!
//! The process is what keeps GridCraft safe from the script, not the other way round: a script
//! that crashes, hangs or floods its output is stopped and reported, and never takes the app with
//! it. It does **not** limit what the script can do to the user's computer. That is what trust
//! is for.

use std::fmt;
use std::time::Duration;

use serde_json::Value as Json;

/// The time a script gets when the caller doesn't say.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(60);
/// The longest time a caller can give a script.
pub const MAX_TIMEOUT: Duration = Duration::from_secs(600);
/// The most data sent to a script in one call (bytes of JSON).
pub const MAX_REQUEST_BYTES: usize = 32 * 1024 * 1024;
/// The longest reply read from a script (bytes of JSON).
pub const MAX_REPLY_BYTES: usize = 64 * 1024 * 1024;
/// How much of what a script prints is kept for the caller (the latest bytes).
pub const PRINTED_KEPT: usize = 64 * 1024;

/// A Python that answered when asked for its version.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interpreter {
    pub program: String,
    pub args: Vec<String>,
    /// For instance `3.12.4`.
    pub version: String,
}

impl Interpreter {
    /// "Python 3.12.4 (py -3)".
    pub fn describe(&self) -> String {
        let mut command = self.program.clone();
        for a in &self.args {
            command.push(' ');
            command.push_str(a);
        }
        format!("Python {} ({command})", self.version)
    }
}

/// Why a script produced no result.
#[derive(Clone, Debug, PartialEq)]
pub enum RunError {
    /// The module may not run this script. Nothing was started.
    Refused(String),
    /// There is no Python to run it with.
    NoPython(String),
    /// The script raised an error, or returned something JSON can't hold.
    Script { error: String, trace: String },
    /// It ran longer than it was given and was stopped.
    TimedOut(Duration),
    /// The Python process ended, was stopped, or stopped making sense.
    Failed(String),
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunError::Refused(why) | RunError::NoPython(why) | RunError::Failed(why) => f.write_str(why),
            RunError::Script { error, .. } => write!(f, "The script failed: {error}"),
            RunError::TimedOut(limit) => write!(f, "The script ran longer than {} seconds and was stopped.", limit.as_secs()),
        }
    }
}

/// What a script gave back.
#[derive(Clone, Debug, PartialEq)]
pub struct Output {
    /// What `main` returned.
    pub result: Json,
    /// What the script printed while it ran (the latest [`PRINTED_KEPT`] bytes).
    pub printed: String,
}

/// What answers a script's calls to the spreadsheet while it runs: it is handed the call (`{call:
/// "run", command, params}` and the like, as a panel sends them) and gives the fields of the
/// answer, or why the call failed. It is called on the thread that runs the script.
pub type Calls<'a> = &'a (dyn Fn(&Json) -> Result<Json, String> + Sync);

#[cfg(not(target_arch = "wasm32"))]
pub use host::PythonHost;

#[cfg(target_arch = "wasm32")]
pub use web::PythonHost;

/// The web build has no processes: every run reports that there is no Python.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use std::path::Path;

    #[derive(Default)]
    pub struct PythonHost;

    impl PythonHost {
        pub fn new() -> Self {
            PythonHost
        }

        pub fn global() -> &'static PythonHost {
            static HOST: PythonHost = PythonHost;
            &HOST
        }

        pub fn interpreter(&self, _configured: &str) -> Result<Interpreter, String> {
            Err("Python is not available in the web build.".into())
        }

        #[allow(clippy::too_many_arguments)]
        pub fn run(
            &self,
            _modules: &Path,
            _trust: &Path,
            _python: &str,
            _id: &str,
            _script: &str,
            _data: &Json,
            _timeout: Duration,
        ) -> Result<Output, RunError> {
            Err(RunError::NoPython("Python is not available in the web build.".into()))
        }

        #[allow(clippy::too_many_arguments)]
        pub fn run_with(
            &self,
            _modules: &Path,
            _trust: &Path,
            _python: &str,
            _id: &str,
            _script: &str,
            _data: &Json,
            _timeout: Duration,
            _calls: Option<Calls<'_>>,
        ) -> Result<Output, RunError> {
            Err(RunError::NoPython("Python is not available in the web build.".into()))
        }

        pub fn stop(&self, _modules: &Path, _id: &str) {}

        pub fn look_again(&self) {}

        pub fn running(&self) -> usize {
            0
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod host {
    use super::*;
    use std::collections::HashMap;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::path::{Path, PathBuf};
    use std::process::{Child, ChildStdin, Command, Stdio};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::mpsc::{Receiver, RecvTimeoutError, channel};
    use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
    use std::time::Instant;

    use serde_json::json;

    use crate::console::{self, Console};
    use crate::manifest::{self, Permission};
    use crate::module;
    use crate::trust::{Status, TrustStore};

    /// How long a new Python gets to start and say it is ready.
    const START_TIMEOUT: Duration = Duration::from_secs(30);
    /// How long a Python gets to tell its version.
    const PROBE_TIMEOUT: Duration = Duration::from_secs(10);

    /// The program the worker runs. It reads one JSON request per line and writes one JSON reply
    /// per line; whatever a script prints goes to standard error, so it can't garble a reply. Just
    /// before each reply it marks on standard error where that call's printing ends.
    ///
    /// It also gives scripts a module to import, `spreadsheet`, of the same shape as the object a
    /// panel has. A call through it is a line written where replies go, marked `calling`, and
    /// the script waits for the line that answers it. `sheet`, `range` and `workbook` are the
    /// workbook as objects: handles that name a sheet or a range and run commands on it.
    const RUNNER: &str = r#"
import sys, os, json, traceback, importlib.util, threading, types

_state = {"id": None, "calls": 0, "requests": None, "replies": None}
_turn = threading.Lock()

class SpreadsheetError(Exception):
    """What a call to GridCraft raises when it fails."""

def _ask(body):
    with _turn:
        if _state["id"] is None:
            raise SpreadsheetError("The spreadsheet can only be called while main() is running.")
        _state["calls"] += 1
        number = _state["calls"]
        try:
            text = json.dumps(dict(body, calling=_state["id"], n=number), allow_nan=False)
        except (TypeError, ValueError) as error:
            raise SpreadsheetError("That can't be sent to GridCraft: " + str(error))
        _state["replies"].write(text.encode("utf-8") + b"\n")
        _state["replies"].flush()
        line = _state["requests"].readline()
        if not line:
            raise SpreadsheetError("GridCraft stopped answering.")
        answer = json.loads(line.decode("utf-8"))
        if answer.get("answer") != number:
            raise SpreadsheetError("GridCraft's answer was out of turn.")
        if not answer.get("ok"):
            raise SpreadsheetError(answer.get("error") or "The call failed.")
        return answer

def _run(command, params=None):
    return _ask({"call": "run", "command": command, "params": dict(params or {})}).get("result")

def _batch(commands, label=None):
    steps = [list(step) if isinstance(step, (list, tuple)) else step for step in commands]
    return _ask({"call": "batch", "commands": steps, "label": label}).get("results")

def _selection():
    return _ask({"call": "selection"}).get("result")

def _quoted(sheet):
    plain = sheet.replace("_", "a").replace(".", "a")
    if plain.isascii() and plain.isalnum() and not (sheet[:1].isdigit() or sheet[:1] == "."):
        return sheet
    return "'" + sheet.replace("'", "''") + "'"

class Range:
    """Cells, by address. On a sheet that isn't the active one the address names the sheet."""
    def __init__(self, address, sheet=None):
        address = str(address)
        self.address = _quoted(sheet) + "!" + address if sheet and "!" not in address else address
    def __repr__(self):
        return "Range(" + repr(self.address) + ")"
    def run(self, command, params=None):
        return _run(command, dict(params or {}, range=self.address))
    def read(self, formulas=False, formatted=False):
        return _run("sheet.read", {"range": self.address, "formulas": bool(formulas), "formatted": bool(formatted)})["values"]
    def formulas(self):
        return self.read(formulas=True)
    def write(self, rows):
        return _run("range.setValues", {"range": self.address, "values": rows})
    def clear(self):
        return _run("edit.clearContents", {"range": self.address})
    def select(self):
        return _run("selection.set", {"range": self.address})

class Sheet:
    """A sheet, by name. Without a name it is whichever sheet is the active one."""
    def __init__(self, name=None, used=None, tables=()):
        self.name = name
        self.used = Range(used, name) if used else None
        self.tables = list(tables)
    def __repr__(self):
        return "Sheet(" + repr(self.name) + ")"
    def range(self, address):
        return Range(address, self.name)
    def read(self, formulas=False, formatted=False):
        params = {"formulas": bool(formulas), "formatted": bool(formatted)}
        if self.name:
            params["sheet"] = self.name
        return _run("sheet.read", params)["values"]
    def activate(self):
        if not self.name:
            raise SpreadsheetError("The active sheet is already the active one: name a sheet to activate it.")
        return _run("sheet.activate", {"sheet": self.name})

class Workbook:
    """The workbook being worked on, as it was when it was asked for."""
    def __init__(self, found):
        self.name = found.get("title")
        self.sheets = [Sheet(s.get("name"), s.get("usedRange"), s.get("tables") or ()) for s in found.get("sheets") or []]
        active = found.get("activeSheet")
        self.active = self.sheets[active] if isinstance(active, int) and 0 <= active < len(self.sheets) else None
        self.selection = found.get("selection")
        self.names = list(found.get("names") or [])
        self.tables = [table for sheet in self.sheets for table in sheet.tables]
    def __repr__(self):
        return "Workbook(" + repr(self.name) + ")"
    def sheet(self, name):
        for sheet in self.sheets:
            if sheet.name == name:
                return sheet
        raise SpreadsheetError("There is no sheet named " + repr(name) + ".")

def _read(range=None, sheet=None, formulas=False, formatted=False):
    if range is not None:
        return Range(range, sheet).read(formulas, formatted)
    return Sheet(sheet).read(formulas, formatted)

def _install():
    module = types.ModuleType("spreadsheet", "GridCraft: what a module's script can ask of the spreadsheet. Made by GridCraft; not a file.")
    module.Error = SpreadsheetError
    module.run, module.batch, module.selection, module.read = _run, _batch, _selection, _read
    module.write = lambda range, rows, sheet=None: Range(range, sheet).write(rows)
    module.sheet = lambda name=None: Sheet(name)
    module.range = lambda address, sheet=None: Range(address, sheet)
    module.workbook = lambda: Workbook(_run("document.inspect"))
    module.Range, module.Sheet, module.Workbook = Range, Sheet, Workbook
    sys.modules["spreadsheet"] = module

def _load(folder, name):
    path = os.path.join(folder, *name.split("/"))
    spec = importlib.util.spec_from_file_location("gridcraft_script_" + "".join(c if c.isalnum() else "_" for c in name), path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def _serve():
    folder = sys.argv[1]
    sys.path.insert(0, folder)
    requests, replies, printed = sys.stdin.buffer, sys.stdout.buffer, sys.stderr.buffer
    sys.stdout = sys.stderr
    _state["requests"], _state["replies"] = requests, replies
    _install()
    loaded = {}
    replies.write(b'{"ready": true}\n')
    replies.flush()
    for line in requests:
        request = None
        try:
            request = json.loads(line.decode("utf-8"))
            name = request["script"]
            if name not in loaded:
                loaded[name] = _load(folder, name)
            main = getattr(loaded[name], "main", None)
            if not callable(main):
                raise TypeError(name + " has no main(data) function")
            _state["id"], _state["calls"] = request["id"], 0
            text = json.dumps({"id": request["id"], "ok": True, "result": main(request.get("data"))}, allow_nan=False)
        except BaseException as error:
            text = json.dumps({
                "id": request.get("id") if isinstance(request, dict) else None,
                "ok": False,
                "error": "".join(traceback.format_exception_only(type(error), error)).strip(),
                "trace": traceback.format_exc(),
            })
        _state["id"] = None
        sys.stderr.flush()
        printed.write(b"\x1e" + str(request.get("id") if isinstance(request, dict) else "").encode() + b"\x1e\n")
        printed.flush()
        replies.write(text.encode("utf-8") + b"\n")
        replies.flush()

_serve()
"#;

    /// The clock, for time limits. This module isn't built for the web, where there is none.
    #[allow(clippy::disallowed_methods)]
    fn now() -> Instant {
        Instant::now()
    }

    /// Writes a run to the Developer console: what the script printed, or why it gave no result.
    fn record(module: &str, script: &str, result: &Result<Output, RunError>) {
        let console = Console::global();
        match result {
            Ok(output) => console.push(module, script, console::Kind::Printed, &output.printed),
            Err(RunError::Script { error, trace }) => {
                let text = if trace.trim().is_empty() { error.clone() } else { format!("{error}\n{trace}") };
                console.push(module, script, console::Kind::Error, &text);
            }
            Err(RunError::Refused(why)) => console.push(module, script, console::Kind::Refused, why),
            Err(other) => console.push(module, script, console::Kind::Error, &other.to_string()),
        }
    }

    fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
        m.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// A command that never opens a console window of its own (Windows), with nothing inherited
    /// on its standard streams.
    fn command(program: &str, args: &[String]) -> Command {
        let mut cmd = Command::new(program);
        cmd.args(args).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        cmd
    }

    /// Asks one candidate for its version. `None` unless it is a Python 3.8 or later that answers
    /// in time (the Windows Store placeholder, for one, doesn't).
    fn probe(program: &str, args: &[&str]) -> Option<Interpreter> {
        let args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
        let mut child = command(program, &args).arg("-c").arg("import sys; sys.stdout.write('%d.%d.%d' % sys.version_info[:3])").spawn().ok()?;
        drop(child.stdin.take());
        let deadline = now() + PROBE_TIMEOUT;
        loop {
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) if now() < deadline => std::thread::sleep(Duration::from_millis(15)),
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return None;
                }
            }
        }
        let mut version = String::new();
        child.stdout.take()?.take(64).read_to_string(&mut version).ok()?;
        let mut parts = version.split('.').map(|p| p.parse::<u32>().ok());
        let (major, minor) = (parts.next()??, parts.next()??);
        (major == 3 && minor >= 8).then_some(Interpreter { program: program.to_string(), args, version })
    }

    /// Finds the Python to use: the user's choice when there is one, or the first that answers.
    fn find(configured: &str) -> Result<Interpreter, String> {
        let configured = configured.trim();
        if !configured.is_empty() {
            return probe(configured, &[]).ok_or_else(|| format!("{configured} did not answer as a Python 3.8 or later."));
        }
        let candidates: &[(&str, &[&str])] =
            if cfg!(windows) { &[("py", &["-3"]), ("python", &[]), ("python3", &[])] } else { &[("python3", &[]), ("python", &[])] };
        candidates
            .iter()
            .find_map(|(program, args)| probe(program, args))
            .ok_or_else(|| "No Python was found. Install Python 3.8 or later, or say where it is in Developer ▸ Modules.".to_string())
    }

    enum Line {
        Text(Vec<u8>),
        TooLong,
        Closed,
    }

    /// One module's Python process.
    struct Worker {
        fingerprint: String,
        interpreter: Interpreter,
        child: Mutex<Child>,
        stdin: Mutex<ChildStdin>,
        lines: Mutex<Receiver<Line>>,
        printed: Arc<Mutex<Vec<u8>>>,
        /// One call at a time.
        turn: Mutex<()>,
        next_id: AtomicU64,
    }

    impl Worker {
        fn start(interpreter: &Interpreter, scripts: &Path, fingerprint: &str) -> Result<Worker, RunError> {
            let failed = |what: String| RunError::Failed(format!("Python could not be started: {what}"));
            let mut cmd = command(&interpreter.program, &interpreter.args);
            // `-B`: Python keeps no compiled copy of what it imports. It would keep it inside the
            // module, whose files would then no longer be the ones the user trusted.
            cmd.arg("-u").arg("-B").arg("-c").arg(RUNNER).arg(scripts).current_dir(scripts);
            cmd.env("PYTHONIOENCODING", "utf-8").env("PYTHONUNBUFFERED", "1").env("PYTHONDONTWRITEBYTECODE", "1");
            let mut child = cmd.spawn().map_err(|e| failed(e.to_string()))?;
            let (Some(stdin), Some(stdout), Some(mut stderr)) = (child.stdin.take(), child.stdout.take(), child.stderr.take()) else {
                let _ = child.kill();
                return Err(failed("its streams could not be opened".into()));
            };
            let (send, lines) = channel();
            let reader = std::thread::Builder::new().name("gridcraft-python-replies".into()).spawn(move || {
                let mut stdout = BufReader::new(stdout);
                loop {
                    let mut line = Vec::new();
                    let read = (&mut stdout).take(MAX_REPLY_BYTES as u64 + 1).read_until(b'\n', &mut line);
                    let next = match read {
                        Ok(0) | Err(_) => Line::Closed,
                        Ok(_) if line.last() != Some(&b'\n') && line.len() > MAX_REPLY_BYTES => Line::TooLong,
                        Ok(_) => Line::Text(line),
                    };
                    let done = !matches!(next, Line::Text(_));
                    if send.send(next).is_err() || done {
                        break;
                    }
                }
            });
            let printed = Arc::new(Mutex::new(Vec::new()));
            let kept = printed.clone();
            let printer = std::thread::Builder::new().name("gridcraft-python-printed".into()).spawn(move || {
                let mut chunk = [0u8; 4096];
                while let Ok(n) = stderr.read(&mut chunk) {
                    if n == 0 {
                        break;
                    }
                    let mut kept = lock(&kept);
                    kept.extend_from_slice(chunk.get(..n).unwrap_or(&[]));
                    let extra = kept.len().saturating_sub(PRINTED_KEPT);
                    kept.drain(..extra);
                }
            });
            if let Err(e) = reader.and(printer) {
                let _ = child.kill();
                let _ = child.wait();
                return Err(failed(e.to_string()));
            }
            let worker = Worker {
                fingerprint: fingerprint.to_string(),
                interpreter: interpreter.clone(),
                child: Mutex::new(child),
                stdin: Mutex::new(stdin),
                lines: Mutex::new(lines),
                printed,
                turn: Mutex::new(()),
                next_id: AtomicU64::new(1),
            };
            // The runner says it is ready before anything is asked of it.
            match lock(&worker.lines).recv_timeout(START_TIMEOUT) {
                Ok(Line::Text(line)) if serde_json::from_slice::<Json>(&line).is_ok_and(|v| v["ready"] == true) => {}
                _ => {
                    let said = worker.take_printed();
                    worker.kill();
                    return Err(failed(if said.trim().is_empty() { "it did not start in time".into() } else { said.trim().to_string() }));
                }
            }
            Ok(worker)
        }

        fn kill(&self) {
            let mut child = lock(&self.child);
            let _ = child.kill();
            let _ = child.wait();
        }

        fn is_running(&self) -> bool {
            matches!(lock(&self.child).try_wait(), Ok(None))
        }

        fn take_printed(&self) -> String {
            String::from_utf8_lossy(&std::mem::take(&mut *lock(&self.printed))).into_owned()
        }

        /// What the call `id` printed: everything up to the runner's end mark for it. The mark is
        /// written before the reply but arrives on another stream, so it is waited for briefly.
        fn printed_by(&self, id: u64) -> String {
            let mark = format!("\u{1e}{id}\u{1e}\n").into_bytes();
            let deadline = now() + Duration::from_secs(2);
            loop {
                {
                    let mut kept = lock(&self.printed);
                    if let Some(at) = kept.windows(mark.len()).position(|w| w == mark.as_slice()) {
                        let printed = String::from_utf8_lossy(kept.get(..at).unwrap_or(&[])).into_owned();
                        kept.drain(..at + mark.len());
                        return printed;
                    }
                }
                if now() >= deadline {
                    return self.take_printed();
                }
                std::thread::sleep(Duration::from_millis(2));
            }
        }

        /// Hands the script the answer to what it asked of the spreadsheet.
        fn answer(&self, number: &Json, answer: Result<Json, String>) -> std::io::Result<()> {
            let told = match answer {
                Ok(Json::Object(mut fields)) => {
                    fields.insert("answer".into(), number.clone());
                    fields.insert("ok".into(), json!(true));
                    Json::Object(fields)
                }
                Ok(other) => json!({"answer": number, "ok": true, "result": other}),
                Err(why) => json!({"answer": number, "ok": false, "error": why}),
            };
            let mut line = serde_json::to_vec(&told).unwrap_or_default();
            if line.is_empty() || line.len() > MAX_REQUEST_BYTES {
                let too_large = json!({"answer": number, "ok": false, "error": "The answer was too large to hand to the script."});
                line = serde_json::to_vec(&too_large).unwrap_or_default();
            }
            line.push(b'\n');
            let mut stdin = lock(&self.stdin);
            stdin.write_all(&line).and_then(|()| stdin.flush())
        }

        /// One request and its reply, with whatever the script asks of the spreadsheet in between
        /// answered by `calls`. On anything but a reply the process is no longer usable.
        fn call(&self, script: &str, data: &Json, timeout: Duration, calls: Option<Calls<'_>>) -> Result<Output, RunError> {
            let _turn = lock(&self.turn);
            let id = self.next_id.fetch_add(1, Ordering::Relaxed);
            let mut request = serde_json::to_vec(&json!({"id": id, "script": script, "data": data}))
                .map_err(|e| RunError::Refused(format!("The data can't be sent to the script: {e}")))?;
            if request.len() > MAX_REQUEST_BYTES {
                return Err(RunError::Refused(format!(
                    "That is more data than a script can be sent at once ({} MB).",
                    MAX_REQUEST_BYTES / (1024 * 1024)
                )));
            }
            request.push(b'\n');
            self.take_printed();
            let gone = || RunError::Failed("The Python process ended before it answered.".into());
            {
                let mut stdin = lock(&self.stdin);
                stdin.write_all(&request).and_then(|()| stdin.flush()).map_err(|_| gone())?;
            }
            let deadline = now() + timeout;
            let lines = lock(&self.lines);
            loop {
                let left = deadline.saturating_duration_since(now());
                let line = match lines.recv_timeout(left) {
                    Ok(Line::Text(line)) => line,
                    Ok(Line::TooLong) => {
                        return Err(RunError::Failed(format!("The script's reply was larger than {} MB.", MAX_REPLY_BYTES / (1024 * 1024))));
                    }
                    Ok(Line::Closed) | Err(RecvTimeoutError::Disconnected) => return Err(gone()),
                    Err(RecvTimeoutError::Timeout) => return Err(RunError::TimedOut(timeout)),
                };
                // Anything that isn't this call's reply (a stray write to standard output) is skipped.
                let Ok(reply) = serde_json::from_slice::<Json>(&line) else { continue };
                // The script asks something of the spreadsheet: it is answered, and the wait for
                // its reply goes on, against the same time limit.
                if reply["calling"].as_u64() == Some(id) {
                    let answer = match calls {
                        Some(calls) => calls(&reply),
                        None => Err("The spreadsheet can't be called from this run.".into()),
                    };
                    self.answer(&reply["n"], answer).map_err(|_| gone())?;
                    continue;
                }
                if reply["id"].as_u64() != Some(id) {
                    continue;
                }
                let printed = self.printed_by(id);
                return if reply["ok"] == true {
                    Ok(Output { result: reply["result"].clone(), printed })
                } else {
                    let trace = format!(
                        "{}{}",
                        reply["trace"].as_str().unwrap_or(""),
                        if printed.is_empty() { String::new() } else { format!("\n{printed}") }
                    );
                    Err(RunError::Script { error: reply["error"].as_str().unwrap_or("unknown error").to_string(), trace })
                };
            }
        }
    }

    impl Drop for Worker {
        fn drop(&mut self) {
            self.kill();
        }
    }

    /// The Python processes of the modules that have run something, and the Pythons found.
    #[derive(Default)]
    pub struct PythonHost {
        workers: Mutex<HashMap<PathBuf, Arc<Worker>>>,
        found: Mutex<HashMap<String, Result<Interpreter, String>>>,
        /// How many times each module was told to stop: a process still starting when its module
        /// is stopped must not go on to run.
        stops: Mutex<HashMap<PathBuf, u64>>,
    }

    impl PythonHost {
        pub fn new() -> Self {
            PythonHost::default()
        }

        /// The app's one host.
        pub fn global() -> &'static PythonHost {
            static HOST: std::sync::OnceLock<PythonHost> = std::sync::OnceLock::new();
            HOST.get_or_init(PythonHost::new)
        }

        /// The Python that `configured` names (empty: the first found). Looked for once per name.
        pub fn interpreter(&self, configured: &str) -> Result<Interpreter, String> {
            let key = configured.trim().to_string();
            if let Some(known) = lock(&self.found).get(&key) {
                return known.clone();
            }
            let result = find(&key);
            lock(&self.found).insert(key, result.clone());
            result
        }

        /// Forgets what was found, so the next call looks again (the user installed a Python).
        pub fn look_again(&self) {
            lock(&self.found).clear();
        }

        /// How many modules have a Python process right now.
        pub fn running(&self) -> usize {
            lock(&self.workers).values().filter(|w| w.is_running()).count()
        }

        /// Runs `script` of the module `id` with `data`, and returns what its `main` returned.
        ///
        /// Refused, with nothing started, unless the user trusts the module as it is on disk now
        /// and granted it `python`, and `script` is one of the files in its scripts folder.
        #[allow(clippy::too_many_arguments)]
        pub fn run(
            &self,
            modules: &Path,
            trust: &Path,
            python: &str,
            id: &str,
            script: &str,
            data: &Json,
            timeout: Duration,
        ) -> Result<Output, RunError> {
            self.run_with(modules, trust, python, id, script, data, timeout, None)
        }

        /// As [`PythonHost::run`], for a script that may call the spreadsheet while it runs:
        /// `calls` answers what it asks (`import spreadsheet` in the script). Without `calls` a
        /// script that asks is told it can't.
        #[allow(clippy::too_many_arguments)]
        pub fn run_with(
            &self,
            modules: &Path,
            trust: &Path,
            python: &str,
            id: &str,
            script: &str,
            data: &Json,
            timeout: Duration,
            calls: Option<Calls<'_>>,
        ) -> Result<Output, RunError> {
            let result = self.run_checked(modules, trust, python, id, script, data, timeout, calls);
            record(id, script, &result);
            result
        }

        #[allow(clippy::too_many_arguments)]
        fn run_checked(
            &self,
            modules: &Path,
            trust: &Path,
            python: &str,
            id: &str,
            script: &str,
            data: &Json,
            timeout: Duration,
            calls: Option<Calls<'_>>,
        ) -> Result<Output, RunError> {
            let refused = |why: String| Err(RunError::Refused(why));
            if !manifest::valid_id(id) {
                return refused("That is not a module.".into());
            }
            let dir = modules.join(id);
            // The module's files stay as they are checked here until the script has finished:
            // nothing changes them through GridCraft meanwhile (see `module::in_use`).
            let in_use = module::in_use(&dir);
            let _as_checked = in_use.read().unwrap_or_else(std::sync::PoisonError::into_inner);
            let found = module::load(&dir);
            let name = found.name().to_string();
            match TrustStore::load(trust).status(&found) {
                Status::Trusted(granted) if granted.contains(&Permission::Python) => {}
                Status::Trusted(_) => return refused(format!("{name} was not given permission to run Python.")),
                Status::Untrusted => return refused(format!("{name} is not trusted. Review it in Developer ▸ Modules.")),
                Status::Changed => return refused(format!("{name} has changed since you trusted it. Review it again in Developer ▸ Modules.")),
                Status::Problem(why) => return refused(format!("{name} can't be used: {why}.")),
            }
            let (Some(manifest), Some(fingerprint)) = (&found.manifest, &found.fingerprint) else {
                return refused(format!("{name} could not be read."));
            };
            let Some(folder) = &manifest.python else { return refused(format!("{name} has no scripts.")) };
            let is_script = manifest::inner_path(script).is_ok() && script.ends_with(".py") && found.files.contains(&format!("{folder}/{script}"));
            if !is_script {
                return refused(format!("{script:?} is not one of {name}'s scripts."));
            }
            let interpreter = self.interpreter(python).map_err(RunError::NoPython)?;
            let timeout = timeout.clamp(Duration::from_secs(1), MAX_TIMEOUT);

            // The module's process: a new one if it has none, or if its files or the Python changed.
            // Starting one takes a moment, and is done without holding up the other modules.
            let stops = |host: &PythonHost| lock(&host.stops).get(&dir).copied().unwrap_or(0);
            let stops_before = stops(self);
            let current =
                lock(&self.workers).get(&dir).filter(|w| w.fingerprint == *fingerprint && w.interpreter == interpreter && w.is_running()).cloned();
            let worker = match current {
                Some(worker) => worker,
                None => {
                    let worker = Arc::new(Worker::start(&interpreter, &dir.join(folder), fingerprint)?);
                    lock(&self.workers).insert(dir.clone(), worker.clone());
                    worker
                }
            };
            if stops(self) != stops_before {
                // The module was stopped while its process was starting.
                self.stop(modules, id);
                return Err(RunError::Failed("The script was stopped.".into()));
            }
            let outcome = worker.call(script, data, timeout, calls);
            if matches!(outcome, Err(RunError::TimedOut(_) | RunError::Failed(_))) {
                // It is hung, dead or confused: the next call gets a fresh one.
                worker.kill();
                let mut workers = lock(&self.workers);
                if workers.get(&dir).is_some_and(|w| Arc::ptr_eq(w, &worker)) {
                    workers.remove(&dir);
                }
            }
            outcome
        }

        /// Stops the module's Python process, whatever it is doing. A call in progress fails.
        pub fn stop(&self, modules: &Path, id: &str) {
            let dir = modules.join(id);
            *lock(&self.stops).entry(dir.clone()).or_insert(0) += 1;
            if let Some(worker) = lock(&self.workers).remove(&dir) {
                worker.kill();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::module::tests::TempDir;

        /// The Python on this machine, or `None` (with a note) when there is none: the tests that
        /// need one are then skipped, and the refusals are still tested.
        fn python(host: &PythonHost) -> bool {
            let found = host.interpreter("");
            if let Err(why) = &found {
                eprintln!("skipped: {why}");
            }
            found.is_ok()
        }

        struct Fixture {
            t: TempDir,
            host: PythonHost,
        }

        impl Fixture {
            /// A module `m` with these scripts, asking for `permissions`, and trusted unless said otherwise.
            fn new(tag: &str, permissions: &str, scripts: &[(&str, &str)], trusted: bool) -> Fixture {
                let t = TempDir::new(tag);
                t.write(
                    "modules/m/module.toml",
                    &format!("id = \"m\"\nname = \"M\"\nversion = \"1\"\npermissions = [{permissions}]\n\n[python]\nscripts = \"python\"\n"),
                );
                for (name, source) in scripts {
                    t.write(&format!("modules/m/python/{name}"), source);
                }
                let f = Fixture { t, host: PythonHost::new() };
                if trusted {
                    f.trust(false);
                }
                f
            }

            fn trust(&self, follow_changes: bool) {
                TrustStore::load(&self.trust_file()).trust(&module::load(&self.t.0.join("modules/m")), follow_changes).unwrap();
            }

            fn trust_file(&self) -> PathBuf {
                self.t.0.join("trusted-modules.json")
            }

            fn run_for(&self, script: &str, data: Json, seconds: u64) -> Result<Output, RunError> {
                self.host.run(&self.t.0.join("modules"), &self.trust_file(), "", "m", script, &data, Duration::from_secs(seconds))
            }

            fn run(&self, script: &str, data: Json) -> Result<Output, RunError> {
                self.run_for(script, data, 30)
            }
        }

        const ECHO: &str = "def main(data):\n    return data\n";

        #[test]
        fn nothing_runs_for_a_module_that_may_not_run_python() {
            let refused = |f: &Fixture, script: &str| match f.run(script, json!(1)) {
                Err(RunError::Refused(why)) => why,
                other => panic!("expected a refusal, got {other:?}"),
            };
            // Not trusted.
            let f = Fixture::new("untrusted", "\"read\", \"python\"", &[("echo.py", ECHO)], false);
            assert!(refused(&f, "echo.py").contains("not trusted"));
            // Trusted, then a file changed.
            f.trust(false);
            f.t.write("modules/m/python/echo.py", "def main(data):\n    import os\n");
            assert!(refused(&f, "echo.py").contains("changed since you trusted it"));
            // Trusted, but it never asked for (so was never granted) python.
            let f = Fixture::new("no-python", "\"read\", \"write\"", &[("echo.py", ECHO)], true);
            assert!(refused(&f, "echo.py").contains("permission to run Python"));
            // Trusted with python: only its own scripts, by name, from its scripts folder.
            let f = Fixture::new("names", "\"python\"", &[("echo.py", ECHO), ("notes.txt", "x")], true);
            f.t.write("outside.py", ECHO);
            for script in [
                "missing.py",
                "../../outside.py",
                "..\\..\\outside.py",
                "/etc/passwd",
                "notes.txt",
                "echo",
                "",
                "../module.toml",
                "echo.py/../echo.py",
            ] {
                assert!(refused(&f, script).contains("is not one of"), "{script:?}");
            }
            // A module id that isn't one, and a module that isn't there.
            for id in ["", "..", "../m", "M"] {
                let r = f.host.run(&f.t.0.join("modules"), &f.trust_file(), "", id, "echo.py", &json!(1), DEFAULT_TIMEOUT);
                assert!(matches!(r, Err(RunError::Refused(_))), "{id:?}: {r:?}");
            }
            assert!(matches!(
                f.host.run(&f.t.0.join("modules"), &f.trust_file(), "", "gone", "echo.py", &json!(1), DEFAULT_TIMEOUT),
                Err(RunError::Refused(_))
            ));
            assert_eq!(f.host.running(), 0, "no refusal started a process");
        }

        #[test]
        fn a_python_that_is_not_one_is_reported() {
            let host = PythonHost::new();
            let nowhere = host.interpreter("gridcraft-no-such-python-here");
            assert!(nowhere.is_err_and(|why| why.contains("did not answer")));
            let f = Fixture::new("no-interp", "\"python\"", &[("echo.py", ECHO)], true);
            let r = f.host.run(&f.t.0.join("modules"), &f.trust_file(), "gridcraft-no-such-python-here", "m", "echo.py", &json!(1), DEFAULT_TIMEOUT);
            assert!(matches!(r, Err(RunError::NoPython(_))), "{r:?}");
            assert_eq!(f.host.running(), 0);
        }

        #[test]
        fn a_trusted_script_runs_on_data_and_its_process_is_kept() {
            let f = Fixture::new(
                "runs",
                "\"python\"",
                &[
                    (
                        "clean.py",
                        "def main(data):\n    return {\"rows\": [[c.strip() if isinstance(c, str) else c for c in row] for row in data[\"rows\"]]}\n",
                    ),
                    ("count.py", "calls = 0\n\ndef main(data):\n    global calls\n    calls += 1\n    return calls\n"),
                    ("sub/deep.py", "import helper\n\ndef main(data):\n    return helper.twice(data)\n"),
                    ("helper.py", "def twice(x):\n    return [x, x]\n"),
                ],
                true,
            );
            if !python(&f.host) {
                return;
            }
            let cleaned = f.run("clean.py", json!({"rows": [["  a ", 1, null], ["日本語 ", true, 2.5]]})).unwrap();
            assert_eq!(
                cleaned.result,
                json!({"rows": [["a", 1, null], ["日本語", true, 2.5]]}),
                "data goes in and comes back as JSON, text as UTF-8"
            );
            // The same process answers the next call: the script's own state is still there.
            assert_eq!(f.run("count.py", Json::Null).unwrap().result, json!(1));
            assert_eq!(f.run("count.py", Json::Null).unwrap().result, json!(2));
            assert_eq!(f.host.running(), 1);
            // A script in a subfolder, importing a sibling from the scripts folder.
            assert_eq!(f.run("sub/deep.py", json!("x")).unwrap().result, json!(["x", "x"]));
            // Running wrote nothing into the module: it is still the one that was trusted.
            let found = module::load(&f.t.0.join("modules/m"));
            assert!(found.files.iter().all(|file| !file.contains("__pycache__") && !file.ends_with(".pyc")), "{:?}", found.files);
            assert!(TrustStore::load(&f.trust_file()).status(&found).is_trusted());
        }

        #[test]
        fn what_a_script_prints_is_kept_apart_from_its_reply() {
            let f = Fixture::new(
                "prints",
                "\"python\"",
                &[(
                    "loud.py",
                    "import sys, os\n\ndef main(data):\n    print(\"to stdout\")\n    print(\"to stderr\", file=sys.stderr)\n    os.write(1, b\"raw bytes on fd 1\\n\")\n    return \"done\"\n",
                )],
                true,
            );
            if !python(&f.host) {
                return;
            }
            let out = f.run("loud.py", Json::Null).unwrap();
            assert_eq!(out.result, json!("done"), "printing doesn't garble the reply");
            assert!(out.printed.contains("to stdout") && out.printed.contains("to stderr"), "{:?}", out.printed);
        }

        #[test]
        fn a_script_that_fails_is_reported_and_the_next_call_still_works() {
            let f = Fixture::new(
                "fails",
                "\"python\"",
                &[
                    ("bad.py", "def main(data):\n    return 1 / 0\n"),
                    ("set.py", "def main(data):\n    return {1, 2}\n"),
                    ("nan.py", "def main(data):\n    return float(\"nan\")\n"),
                    ("nomain.py", "x = 1\n"),
                    ("syntax.py", "def main(data:\n"),
                    ("exit.py", "import sys\n\ndef main(data):\n    sys.exit(3)\n"),
                    ("echo.py", ECHO),
                ],
                true,
            );
            if !python(&f.host) {
                return;
            }
            let error = |script: &str| match f.run(script, Json::Null) {
                Err(RunError::Script { error, trace }) => (error, trace),
                other => panic!("{script}: expected a script error, got {other:?}"),
            };
            let (said, trace) = error("bad.py");
            assert!(said.contains("ZeroDivisionError") && trace.contains("bad.py"), "{said} / {trace}");
            assert!(error("set.py").0.contains("not JSON serializable"));
            assert!(error("nan.py").0.contains("Out of range float"), "NaN is not JSON");
            assert!(error("nomain.py").0.contains("no main(data) function"));
            assert!(error("syntax.py").0.contains("SyntaxError"));
            assert!(error("exit.py").0.contains("SystemExit"), "leaving the interpreter is an error, not the end of the process");
            assert_eq!(f.run("echo.py", json!([1])).unwrap().result, json!([1]));
            assert_eq!(f.host.running(), 1, "none of those cost the process");
        }

        #[test]
        fn a_script_that_hangs_or_dies_is_stopped_and_the_next_call_gets_a_new_process() {
            let f = Fixture::new(
                "hangs",
                "\"python\"",
                &[
                    ("forever.py", "def main(data):\n    while True:\n        pass\n"),
                    ("dies.py", "import os\n\ndef main(data):\n    os._exit(3)\n"),
                    ("echo.py", ECHO),
                ],
                true,
            );
            if !python(&f.host) {
                return;
            }
            assert_eq!(f.run_for("forever.py", Json::Null, 1), Err(RunError::TimedOut(Duration::from_secs(1))));
            assert_eq!(f.host.running(), 0, "the hung process was stopped");
            assert_eq!(f.run("echo.py", json!("again")).unwrap().result, json!("again"));
            assert!(matches!(f.run("dies.py", Json::Null), Err(RunError::Failed(_))));
            assert_eq!(f.run("echo.py", json!("and again")).unwrap().result, json!("and again"));
            // Stopping a module's process from outside fails the call in progress.
            std::thread::scope(|s| {
                let call = s.spawn(|| f.run_for("forever.py", Json::Null, 60));
                while f.host.running() == 0 {
                    std::thread::sleep(Duration::from_millis(10));
                }
                std::thread::sleep(Duration::from_millis(300));
                f.host.stop(&f.t.0.join("modules"), "m");
                assert!(matches!(call.join().unwrap(), Err(RunError::Failed(_))));
            });
            assert_eq!(f.host.running(), 0);
        }

        #[test]
        fn changed_scripts_run_only_when_the_user_keeps_trusting_them() {
            let f = Fixture::new("follow", "\"python\"", &[("v.py", "def main(data):\n    return 1\n")], false);
            f.trust(true);
            if !python(&f.host) {
                return;
            }
            assert_eq!(f.run("v.py", Json::Null).unwrap().result, json!(1));
            // The developer edits their own module: the new code runs, in a new process.
            f.t.write("modules/m/python/v.py", "def main(data):\n    return 2\n");
            assert_eq!(f.run("v.py", Json::Null).unwrap().result, json!(2));
            assert_eq!(f.host.running(), 1);
            // Taking trust back stops it from running at all.
            TrustStore::load(&f.trust_file()).revoke("m").unwrap();
            assert!(matches!(f.run("v.py", Json::Null), Err(RunError::Refused(_))));
        }

        /// What scripts asked of the spreadsheet, and a stand-in for the window that answers them.
        #[derive(Default)]
        struct Asked(Mutex<Vec<Json>>);

        impl Asked {
            /// The calls so far, without the marks the runner adds to each.
            fn calls(&self) -> Vec<Json> {
                let strip = |call: &Json| {
                    let mut call = call.clone();
                    if let Some(fields) = call.as_object_mut() {
                        fields.remove("calling");
                        fields.remove("n");
                    }
                    call
                };
                lock(&self.0).iter().map(strip).collect()
            }

            fn answer(&self, call: &Json) -> Result<Json, String> {
                lock(&self.0).push(call.clone());
                let command = call["command"].as_str().unwrap_or("");
                match (call["call"].as_str(), command) {
                    (Some("selection"), _) => Ok(json!({"result": {"sheet": "Sheet1", "range": "A1:B1", "rows": [[1, 2]]}})),
                    (Some("batch"), _) => Ok(json!({"results": [{"on": true}, {"on": true}]})),
                    (Some("run"), "sheet.read") => Ok(json!({"result": {"range": call["params"]["range"], "values": [[1, "two"], [null, true]]}})),
                    (Some("run"), "document.inspect") => Ok(json!({"result": {
                        "title": "Book1",
                        "activeSheet": 1,
                        "selection": "B2:C3",
                        "names": [{"name": "Rate", "refersTo": "=Sheet1!$A$1"}],
                        "sheets": [
                            {"name": "Sheet1", "usedRange": "A1:B3", "tables": [{"name": "Table1", "range": "A1:B3"}]},
                            {"name": "My Data", "usedRange": null, "tables": []},
                        ],
                    }})),
                    (Some("run"), "cell.refuse") => Err("M wasn't granted `write`, which cell.refuse needs.".into()),
                    (Some("run"), _) => Ok(json!({"result": {"cells": 1}})),
                    _ => Err("That is not something a script can ask of the spreadsheet.".into()),
                }
            }
        }

        impl Fixture {
            fn run_asking(&self, script: &str, data: Json, asked: &Asked) -> Result<Output, RunError> {
                let calls = |call: &Json| asked.answer(call);
                self.host.run_with(&self.t.0.join("modules"), &self.trust_file(), "", "m", script, &data, Duration::from_secs(30), Some(&calls))
            }
        }

        #[test]
        fn a_script_calls_the_spreadsheet_while_it_runs() {
            let script = "import spreadsheet\n\ndef main(data):\n    chosen = spreadsheet.selection()\n    spreadsheet.run('cell.set', {'cell': 'A1', 'input': data})\n    done = spreadsheet.batch([['home.bold', {'range': 'A1'}], {'command': 'home.italic', 'params': {'range': 'A1'}}], label='Tidy')\n    rows = spreadsheet.read('A1:B2')\n    everything = spreadsheet.read(sheet='Data', formulas=True)\n    wrote = spreadsheet.write('C1', [[1, 2]])\n    spreadsheet.run('view.zoom')\n    return {'chosen': chosen, 'done': done, 'rows': rows, 'everything': everything, 'wrote': wrote}\n";
            let f = Fixture::new("calls", "\"python\"", &[("ask.py", script)], true);
            if !python(&f.host) {
                return;
            }
            let asked = Asked::default();
            let output = f.run_asking("ask.py", json!("hello"), &asked).unwrap();
            assert_eq!(
                output.result,
                json!({
                    "chosen": {"sheet": "Sheet1", "range": "A1:B1", "rows": [[1, 2]]},
                    "done": [{"on": true}, {"on": true}],
                    "rows": [[1, "two"], [null, true]],
                    "everything": [[1, "two"], [null, true]],
                    "wrote": {"cells": 1},
                })
            );
            // Each call reached the window as a panel's would, in order.
            assert_eq!(
                asked.calls(),
                [
                    json!({"call": "selection"}),
                    json!({"call": "run", "command": "cell.set", "params": {"cell": "A1", "input": "hello"}}),
                    json!({"call": "batch", "label": "Tidy", "commands": [["home.bold", {"range": "A1"}], {"command": "home.italic", "params": {"range": "A1"}}]}),
                    json!({"call": "run", "command": "sheet.read", "params": {"range": "A1:B2", "formulas": false, "formatted": false}}),
                    json!({"call": "run", "command": "sheet.read", "params": {"sheet": "Data", "formulas": true, "formatted": false}}),
                    json!({"call": "run", "command": "range.setValues", "params": {"range": "C1", "values": [[1, 2]]}}),
                    json!({"call": "run", "command": "view.zoom", "params": {}}),
                ]
            );
            // The calls of one run are numbered from one, and the next run's start again.
            let numbers: Vec<Json> = lock(&asked.0).iter().map(|call| call["n"].clone()).collect();
            assert_eq!(numbers, (1..=7).map(|n| json!(n)).collect::<Vec<_>>());
            f.run_asking("ask.py", json!("again"), &asked).unwrap();
            assert_eq!(lock(&asked.0).get(7).map(|call| call["n"].clone()), Some(json!(1)));
            // A script that asks a great deal is answered each time.
            f.t.write(
                "modules/m/python/many.py",
                "import spreadsheet\n\ndef main(data):\n    return sum(spreadsheet.run('cell.get', {'cell': 'A1'})['cells'] for _ in range(300))\n",
            );
            f.trust(false);
            assert_eq!(f.run_asking("many.py", Json::Null, &Asked::default()).unwrap().result, json!(300));
        }

        #[test]
        fn a_script_works_with_the_workbook_as_objects() {
            let script = "import spreadsheet\n\ndef main(data):\n    book = spreadsheet.workbook()\n    sheet = spreadsheet.sheet('My Data')\n    cells = sheet.range('B2:C3')\n    cells.write([[1, 2], [3, 4]])\n    cells.run('home.bold', {'on': True})\n    values = cells.read()\n    formulas = cells.formulas()\n    cells.clear()\n    cells.select()\n    sheet.read(formatted=True)\n    sheet.activate()\n    spreadsheet.sheet().read()\n    spreadsheet.range('A1').read()\n    spreadsheet.range('Totals!A1:A9').read()\n    spreadsheet.sheet(\"It's\").range('A1').read()\n    book.sheet('Sheet1').used.read()\n    return {\n        'name': book.name,\n        'sheets': [s.name for s in book.sheets],\n        'active': book.active.name,\n        'used': [s.used.address if s.used else None for s in book.sheets],\n        'tables': book.tables,\n        'names': book.names,\n        'selection': book.selection,\n        'address': cells.address,\n        'shown': [repr(cells), repr(sheet), repr(book)],\n        'values': values,\n        'formulas': formulas,\n    }\n";
            let f = Fixture::new("objects", "\"python\"", &[("objects.py", script)], true);
            if !python(&f.host) {
                return;
            }
            let asked = Asked::default();
            let output = f.run_asking("objects.py", Json::Null, &asked).unwrap();
            assert_eq!(
                output.result,
                json!({
                    "name": "Book1",
                    "sheets": ["Sheet1", "My Data"],
                    "active": "My Data",
                    "used": ["Sheet1!A1:B3", null],
                    "tables": [{"name": "Table1", "range": "A1:B3"}],
                    "names": [{"name": "Rate", "refersTo": "=Sheet1!$A$1"}],
                    "selection": "B2:C3",
                    "address": "'My Data'!B2:C3",
                    "shown": ["Range(\"'My Data'!B2:C3\")", "Sheet('My Data')", "Workbook('Book1')"],
                    "values": [[1, "two"], [null, true]],
                    "formulas": [[1, "two"], [null, true]],
                })
            );
            // Every object is a command with the sheet or the range filled in.
            let at = "'My Data'!B2:C3";
            let run = |command: &str, params: Json| json!({"call": "run", "command": command, "params": params});
            assert_eq!(
                asked.calls(),
                [
                    run("document.inspect", json!({})),
                    run("range.setValues", json!({"range": at, "values": [[1, 2], [3, 4]]})),
                    run("home.bold", json!({"on": true, "range": at})),
                    run("sheet.read", json!({"range": at, "formulas": false, "formatted": false})),
                    run("sheet.read", json!({"range": at, "formulas": true, "formatted": false})),
                    run("edit.clearContents", json!({"range": at})),
                    run("selection.set", json!({"range": at})),
                    run("sheet.read", json!({"sheet": "My Data", "formulas": false, "formatted": true})),
                    run("sheet.activate", json!({"sheet": "My Data"})),
                    run("sheet.read", json!({"formulas": false, "formatted": false})),
                    run("sheet.read", json!({"range": "A1", "formulas": false, "formatted": false})),
                    run("sheet.read", json!({"range": "Totals!A1:A9", "formulas": false, "formatted": false})),
                    run("sheet.read", json!({"range": "'It''s'!A1", "formulas": false, "formatted": false})),
                    run("sheet.read", json!({"range": "Sheet1!A1:B3", "formulas": false, "formatted": false})),
                ]
            );
        }

        #[test]
        fn a_call_that_fails_is_an_error_the_script_can_catch() {
            let scripts = [
                (
                    "caught.py",
                    "import spreadsheet\n\ndef main(data):\n    try:\n        spreadsheet.run('cell.refuse', {})\n    except spreadsheet.Error as error:\n        return str(error)\n",
                ),
                ("uncaught.py", "import spreadsheet\n\ndef main(data):\n    spreadsheet.run('cell.refuse', {})\n"),
                (
                    "nan.py",
                    "import spreadsheet\n\ndef main(data):\n    try:\n        spreadsheet.write('A1', [[float('nan')]])\n    except spreadsheet.Error as error:\n        return str(error)\n",
                ),
                (
                    "early.py",
                    "import spreadsheet\n\nspreadsheet.run('cell.set', {'cell': 'A1', 'input': 'at import'})\n\ndef main(data):\n    return 1\n",
                ),
                (
                    "nowhere.py",
                    "import spreadsheet\n\ndef main(data):\n    try:\n        spreadsheet.workbook().sheet('Nope')\n    except spreadsheet.Error as error:\n        first = str(error)\n    try:\n        spreadsheet.sheet().activate()\n    except spreadsheet.Error as error:\n        return [first, str(error)]\n",
                ),
                ("plain.py", "import spreadsheet\n\ndef main(data):\n    return spreadsheet.selection()\n"),
            ];
            let f = Fixture::new("call-errors", "\"python\"", &scripts, true);
            if !python(&f.host) {
                return;
            }
            let asked = Asked::default();
            assert_eq!(f.run_asking("caught.py", Json::Null, &asked).unwrap().result, json!("M wasn't granted `write`, which cell.refuse needs."));
            match f.run_asking("uncaught.py", Json::Null, &asked) {
                Err(RunError::Script { error, trace }) => {
                    assert!(error.contains("SpreadsheetError") && error.contains("wasn't granted `write`"), "{error}");
                    assert!(trace.contains("uncaught.py"), "{trace}");
                }
                other => panic!("{other:?}"),
            }
            // What JSON can't hold never leaves the script.
            let before = asked.calls().len();
            let said = f.run_asking("nan.py", Json::Null, &asked).unwrap().result;
            assert!(said.as_str().unwrap().contains("can't be sent to GridCraft"), "{said}");
            assert_eq!(asked.calls().len(), before);
            // The spreadsheet is called from main(), not while a script is being loaded.
            match f.run_asking("early.py", Json::Null, &asked) {
                Err(RunError::Script { error, .. }) => assert!(error.contains("while main() is running"), "{error}"),
                other => panic!("{other:?}"),
            }
            assert_eq!(asked.calls().len(), before, "nothing was asked");
            let said = f.run_asking("nowhere.py", Json::Null, &asked).unwrap().result;
            assert!(said[0].as_str().unwrap().contains("no sheet named 'Nope'") && said[1].as_str().unwrap().contains("name a sheet"), "{said}");
            // A run that has nothing to answer its calls says so, and the script goes on.
            match f.run("plain.py", Json::Null) {
                Err(RunError::Script { error, .. }) => assert!(error.contains("can't be called from this run"), "{error}"),
                other => panic!("{other:?}"),
            }
            assert_eq!(f.run("echo.py", json!(5)).err().map(|e| matches!(e, RunError::Refused(_))), Some(true), "echo.py is not one of its scripts");
            assert_eq!(
                f.run_asking("caught.py", Json::Null, &asked).unwrap().result,
                json!("M wasn't granted `write`, which cell.refuse needs."),
                "the process is still usable"
            );
        }
    }
}
