//! The bridge: a small HTTP server, on the loopback address only, that serves a trusted project's
//! panel to the user's browser and carries the panel's calls to GridCraft.
//!
//! Each project that is served gets a port of its own, so that the browser keeps the panels of two
//! projects apart (a page's origin is its address and port), and a secret token made for this run,
//! which starts every path. A request is answered only when all of this holds:
//!
//! - it names this bridge by its loopback address (a name someone pointed at this machine is
//!   refused), and the browser doesn't say that a page of another site is asking;
//! - its path starts with the project's token;
//! - **the project is trusted as it is on disk now.** It is read, and the user's decisions with
//!   it, for every request: a project that was changed or is no longer trusted stops being served
//!   at once;
//! - a file is one of the project's own, under its panel folder; a call comes from the project's
//!   own page (the browser names the page that asks) and is JSON.
//!
//! What a panel may call:
//!
//! - `runPython` is answered here, by [`crate::python`], which checks trust again for itself.
//!   What the script asks of the spreadsheet while it runs (`import spreadsheet`) is carried to
//!   the app like a panel's own calls: the project is read from disk again for each, and the
//!   script gets only what the project was granted ([`Bridge::script_calls`]).
//! - `run`, `batch` and `selection` need the workbook, which only the app has. They are put in a
//!   queue as [`Call`]s, with what the project was granted, for the app to carry out on its own
//!   thread ([`Bridge::take_calls`]). The app decides what each command needs ([`needs`]).
//! - `events` hands a panel what happened in GridCraft since it last asked ([`EVENTS`]), waiting a
//!   while for something to happen if nothing has. The app says what happened ([`Bridge::emit`]);
//!   a project that wasn't granted `read` is told nothing.
//! - `log` writes a line to the Developer console ([`crate::console`]).
//!
//! The panel is given its `spreadsheet` object by [`CLIENT`], a script served beside the page, and
//! TypeScript is told its shape by [`typings`]. Both are text made here: the repository holds no
//! JavaScript files.
//!
//! A project that wasn't granted `network` is served with a policy that makes the browser refuse
//! its requests to anywhere but this bridge. In a browser that policy cannot stop a page from
//! sending the user to another address, which can carry data with it: that needs the panel docked
//! in the window, where GridCraft decides where a page may go.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use serde_json::Value as Json;

use crate::manifest::Permission;

/// Where this run keeps projects, the user's decisions and the subsystem's settings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Places {
    pub projects: PathBuf,
    pub trust: PathBuf,
    pub settings: PathBuf,
}

/// The most commands one `batch` may hold.
pub const MAX_BATCH: usize = 1_000;
/// The most calls that may wait for the app at once.
pub const MAX_WAITING: usize = 256;
/// The largest call read (bytes): what a script can be handed.
pub const MAX_BODY_BYTES: usize = crate::python::MAX_REQUEST_BYTES;

/// How long the bridge waits: for a request to arrive, and for the app to answer a call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub read: Duration,
    pub answer: Duration,
    /// How long a panel that asks for events is kept waiting for one before it is told there
    /// were none, and asks again.
    pub hold: Duration,
}

impl Default for Limits {
    fn default() -> Self {
        Limits { read: Duration::from_secs(10), answer: Duration::from_secs(30), hold: Duration::from_secs(20) }
    }
}

/// The events a panel can listen for (`spreadsheet.on`), and what each tells. All of them say
/// something about the open workbooks, so a project needs `read` to be told any.
pub const EVENTS: &[(&str, &str)] = &[
    ("selectionChanged", "{workbook, sheet, range, active}: the selection moved, or another sheet was chosen"),
    ("workbookChanged", "{workbook}: something in the workbook changed (an edit, an undo, a redo)"),
    ("workbookActivated", "{workbook}: another workbook became the one being worked on (it was opened, or switched to)"),
];

/// The most events kept for a project's panels to collect. When more pile up the oldest are
/// dropped, and a panel that hadn't collected them is told it missed some.
pub const MAX_EVENTS: usize = 256;
/// The most panels of one project that can be kept waiting for events at once. More are answered
/// at once and told to ask again in a moment.
pub const MAX_LISTENERS: usize = 8;

/// Something that happened in GridCraft.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    /// One of [`EVENTS`].
    pub name: &'static str,
    /// What it tells: the fields of an object.
    pub detail: Json,
}

/// What a command does, as the app that owns it knows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// It only reports.
    Reads,
    /// Anything else: it changes the workbook, the selection, the view or files.
    Changes,
}

/// What a project must have been granted to make this call.
///
/// A command given a `path` reads or writes a file, whatever else it is; so do the `file.`
/// commands that aren't reports (`file.properties` only changes the workbook).
pub fn needs(id: &str, kind: Kind, params: &Json) -> Permission {
    let names_a_file = params.get("path").is_some_and(|path| !path.is_null());
    let file_command = id.starts_with("file.") && kind == Kind::Changes && id != "file.properties";
    if names_a_file || file_command {
        Permission::Files
    } else if kind == Kind::Reads {
        Permission::Read
    } else {
        Permission::Write
    }
}

/// A command as the typings describe it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandDoc {
    pub id: String,
    pub label: String,
    pub params: String,
    /// What a project needs to run it without a `path`.
    pub needs: Permission,
}

/// What a panel asked of the workbook.
#[derive(Clone, Debug, PartialEq)]
pub enum Request {
    /// One command.
    Run { command: String, params: Json },
    /// Several commands as one undo step: all of them, or none.
    Batch { label: String, commands: Vec<(String, Json)> },
    /// The selection, as a script is handed it: `{range, sheet, rows}`.
    Selection,
}

/// A call waiting for the app: who asks, what they were granted, and what they ask.
pub struct Call {
    /// The project's id.
    pub project: String,
    /// Its name, for what the user sees (the label of an undo step).
    pub name: String,
    /// What the user granted it, as of this call.
    pub granted: Vec<Permission>,
    pub request: Request,
    reply: mpsc::Sender<Result<Json, String>>,
    /// The panel stopped waiting: the call must not be carried out late.
    abandoned: Arc<AtomicBool>,
}

impl Call {
    /// Sends the panel its answer.
    pub fn answer(self, result: Result<Json, String>) {
        let _ = self.reply.send(result);
    }

    fn is_abandoned(&self) -> bool {
        self.abandoned.load(Ordering::SeqCst)
    }
}

/// The script that gives a panel its `spreadsheet` object. Served as `spreadsheet.js` beside the
/// panel's page, it finds the bridge from its own address.
pub const CLIENT: &str = r#"// GridCraft: the `spreadsheet` object of a project's panel. Served by GridCraft; not a file to edit.
(function () {
  "use strict";
  const api = new URL("api", document.currentScript.src).href;

  class SpreadsheetError extends Error {
    constructor(message, trace) {
      super(message);
      this.name = "SpreadsheetError";
      if (trace) this.trace = trace;
    }
  }

  async function call(body) {
    let response;
    try {
      response = await fetch(api, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
        cache: "no-store",
      });
    } catch (error) {
      throw new SpreadsheetError("GridCraft can't be reached. Is it still running?");
    }
    let reply;
    try {
      reply = await response.json();
    } catch (error) {
      throw new SpreadsheetError("GridCraft did not answer (" + response.status + ").");
    }
    if (!reply || reply.ok !== true) throw new SpreadsheetError((reply && reply.error) || "The call failed.", reply && reply.trace);
    return reply;
  }

  const step = (c) => (Array.isArray(c) ? { command: c[0], params: c[1] || {} } : { command: c.command, params: c.params || {} });

  // Events: one request is kept open for as long as anything listens, and asked again when it is answered.
  const handlers = new Map();
  let listening = false;
  const pause = (ms) => new Promise((done) => setTimeout(done, ms));

  async function listen(after) {
    if (listening) return;
    listening = true;
    let wait = 0;
    while (handlers.size) {
      let reply;
      try {
        reply = await call({ call: "events", after });
        wait = 0;
      } catch (error) {
        wait = Math.min(wait ? wait * 2 : 1000, 15000);
        await pause(wait);
        continue;
      }
      after = reply.next;
      for (const event of reply.events) {
        for (const handler of Array.from(handlers.get(event.name) || [])) {
          try {
            handler(event);
          } catch (error) {
            console.error(error);
          }
        }
      }
      if (reply.again) await pause(reply.again * 1000);
    }
    listening = false;
  }

  function off(name, handler) {
    const listeners = handlers.get(name);
    if (!listeners) return;
    listeners.delete(handler);
    if (!listeners.size) handlers.delete(name);
  }

  async function on(name, handler) {
    if (typeof handler !== "function") throw new SpreadsheetError("spreadsheet.on needs a function to call.");
    const reply = await call({ call: "events", names: [name] });
    if (!handlers.has(name)) handlers.set(name, new Set());
    handlers.get(name).add(handler);
    listen(reply.next);
    return () => off(name, handler);
  }

  const words = (value) => {
    if (typeof value === "string") return value;
    try {
      const text = JSON.stringify(value);
      return text === undefined ? String(value) : text;
    } catch (error) {
      return String(value);
    }
  };

  // The workbook as objects: handles that name a sheet or a range and run commands on it.
  const run = async (command, params) => (await call({ call: "run", command, params: params || {} })).result;
  const quoted = (sheet) => (/^[A-Za-z_][A-Za-z0-9_.]*$/.test(sheet) ? sheet : "'" + String(sheet).replace(/'/g, "''") + "'");

  function range(address, sheet) {
    const at = String(address);
    const full = sheet && !at.includes("!") ? quoted(sheet) + "!" + at : at;
    return Object.freeze({
      address: full,
      run: (command, params) => run(command, Object.assign({}, params, { range: full })),
      read: async (options) => (await run("sheet.read", Object.assign({}, options, { range: full }))).values,
      formulas: async () => (await run("sheet.read", { range: full, formulas: true })).values,
      write: (rows) => run("range.setValues", { range: full, values: rows }),
      clear: () => run("edit.clearContents", { range: full }),
      select: () => run("selection.set", { range: full }),
    });
  }

  function sheet(name, used, tables) {
    return Object.freeze({
      name,
      used: used ? range(used, name) : null,
      tables: tables || [],
      range: (address) => range(address, name),
      read: async (options) => (await run("sheet.read", Object.assign({}, options, name ? { sheet: name } : {}))).values,
      activate: async () => {
        if (!name) throw new SpreadsheetError("The active sheet is already the active one: name a sheet to activate it.");
        return run("sheet.activate", { sheet: name });
      },
    });
  }

  async function workbook() {
    const found = await run("document.inspect");
    const sheets = (found.sheets || []).map((s) => sheet(s.name, s.usedRange, s.tables));
    return Object.freeze({
      name: found.title,
      sheets,
      active: sheets[found.activeSheet] || null,
      selection: found.selection,
      names: found.names || [],
      tables: sheets.flatMap((s) => s.tables),
      sheet: (name) => {
        const named = sheets.find((s) => s.name === name);
        if (!named) throw new SpreadsheetError("There is no sheet named " + JSON.stringify(name) + ".");
        return named;
      },
    });
  }

  window.spreadsheet = Object.freeze({
    Error: SpreadsheetError,
    sheet: (name) => sheet(name),
    range,
    workbook,
    on,
    off,
    log: (...values) => call({ call: "log", text: values.map(words).join(" ") }).then(
      () => undefined,
      () => undefined
    ),
    run: async (command, params) => (await call({ call: "run", command, params: params || {} })).result,
    batch: async (commands, options) =>
      (await call({ call: "batch", label: options && options.label, commands: Array.from(commands, step) })).results,
    runPython: async (script, data, options) => {
      const reply = await call({ call: "runPython", script, data: data === undefined ? null : data, timeout: options && options.timeout });
      if (options && typeof options.printed === "function") options.printed(reply.printed || "");
      return reply.result;
    },
    selection: async () => (await call({ call: "selection" })).result,
    read: async (range, options) =>
      (await call({ call: "run", command: "sheet.read", params: Object.assign({}, options, range ? { range } : {}) })).result.values,
    write: async (range, rows, options) =>
      (await call({ call: "run", command: "range.setValues", params: Object.assign({}, options, { range, values: rows }) })).result,
    info: async () => (await call({ call: "info" })).result,
  });
})();
"#;

const TYPINGS: &str = r#"// GridCraft: the types of the `spreadsheet` object a project's panel is given.
// Made from this copy of GridCraft's commands; not a file to edit.

/** What a command takes. Each command's line in `SpreadsheetCommands` says what it reads. */
type SpreadsheetParams = Record<string, unknown>;

/** The selection, as `spreadsheet.selection()` gives it and a script is handed it. */
interface SpreadsheetSelection {
  sheet: string;
  range: string;
  rows: unknown[][];
}

interface SpreadsheetInfo {
  project: { id: string; name: string; version: string };
  /** What the user granted the project: "read", "write", "files", "network", "python". */
  granted: string[];
}

/** Cells, by address. On a sheet that isn't the active one the address names the sheet: "Data!A1:C3". */
interface SpreadsheetRange {
  readonly address: string;
  /** Runs a command on these cells: its `range` is filled in. */
  run<K extends keyof SpreadsheetCommands>(command: K, params?: SpreadsheetParams): Promise<any>;
  /** Their values, row by row. */
  read(options?: { formulas?: boolean; formatted?: boolean }): Promise<unknown[][]>;
  /** What was typed into them: formulas as text. */
  formulas(): Promise<unknown[][]>;
  /** Writes rows of values from the first cell. Text that starts with = is a formula. */
  write(rows: unknown[][]): Promise<any>;
  /** Clears what is in them, keeping their formats. */
  clear(): Promise<any>;
  /** Makes them the selection. */
  select(): Promise<any>;
}

/** A sheet, by name. Without a name it is whichever sheet is the active one. */
interface SpreadsheetSheet {
  readonly name: string | undefined;
  /** The cells in use, for a sheet that came from `workbook()` and has any. */
  readonly used: SpreadsheetRange | null;
  /** Its tables, for a sheet that came from `workbook()`. */
  readonly tables: unknown[];
  range(address: string): SpreadsheetRange;
  /** The values of the cells in use, row by row. */
  read(options?: { formulas?: boolean; formatted?: boolean }): Promise<unknown[][]>;
  /** Makes it the active sheet. */
  activate(): Promise<any>;
}

/** The workbook being worked on, as it was when it was asked for. */
interface SpreadsheetWorkbook {
  readonly name: string;
  readonly sheets: SpreadsheetSheet[];
  readonly active: SpreadsheetSheet | null;
  /** The address of the selection, on the active sheet. */
  readonly selection: string;
  readonly names: unknown[];
  readonly tables: unknown[];
  /** The sheet of that name. Throws when there is none. */
  sheet(name: string): SpreadsheetSheet;
}

/** What a function given to `spreadsheet.on` is handed, for each event. */
interface SpreadsheetEvents {
  /** The selection moved, or another sheet was chosen. */
  selectionChanged: { name: "selectionChanged"; seq: number; workbook: string; sheet: string; range: string; active: string };
  /** Something in the workbook changed: an edit, an undo, a redo. */
  workbookChanged: { name: "workbookChanged"; seq: number; workbook: string };
  /** Another workbook became the one being worked on: it was opened, or switched to. */
  workbookActivated: { name: "workbookActivated"; seq: number; workbook: string };
}

/** One step of a batch: `["home.bold", {range: "A1:C1"}]` or `{command, params}`. */
type SpreadsheetStep =
  | [keyof SpreadsheetCommands, SpreadsheetParams?]
  | { command: keyof SpreadsheetCommands; params?: SpreadsheetParams };

interface Spreadsheet {
  /** What a failed call throws. A failed script's traceback is in `trace`. */
  Error: new (message: string, trace?: string) => Error & { trace?: string };
  /** Runs one command, by its id. It never opens a dialog: what it needs goes in `params`. */
  run<K extends keyof SpreadsheetCommands>(command: K, params?: SpreadsheetParams): Promise<any>;
  /** Runs several commands as one undo step. If one fails, none of them stays. */
  batch(commands: SpreadsheetStep[], options?: { label?: string }): Promise<any[]>;
  /**
   * Runs one of the project's own Python scripts. Its `main(data)` is handed `data`, and what it
   * returns comes back. `timeout` is in seconds; `printed` is handed what the script printed.
   */
  runPython(script: string, data?: unknown, options?: { timeout?: number; printed?: (text: string) => void }): Promise<any>;
  /** The selected cells and their values. */
  selection(): Promise<SpreadsheetSelection>;
  /** The values of a range (the used range when none is named), row by row. */
  read(range?: string, options?: { sheet?: string; formulas?: boolean; formatted?: boolean }): Promise<unknown[][]>;
  /** Writes rows of values from the range's first cell. Text that starts with = is a formula. */
  write(range: string, rows: unknown[][], options?: { sheet?: string }): Promise<any>;
  /** The project, and what the user granted it. */
  info(): Promise<SpreadsheetInfo>;
  /** A sheet, by name; without one, whichever sheet is active. Nothing is asked of GridCraft until it is used. */
  sheet(name?: string): SpreadsheetSheet;
  /** Cells, by address: "A1:C3", or "Data!A1:C3" (or a second argument) for another sheet. */
  range(address: string, sheet?: string): SpreadsheetRange;
  /** The workbook being worked on: its sheets, names and tables, as they are now. */
  workbook(): Promise<SpreadsheetWorkbook>;
  /**
   * Calls `handler` each time the event happens, until the function this resolves to is called.
   * Needs `read`. Events that pile up are thinned: a handler is told where the selection is now,
   * not every place it has been.
   */
  on<K extends keyof SpreadsheetEvents>(name: K, handler: (event: SpreadsheetEvents[K]) => void): Promise<() => void>;
  /** Stops calling `handler` for the event. */
  off<K extends keyof SpreadsheetEvents>(name: K, handler: (event: SpreadsheetEvents[K]) => void): void;
  /** Writes a line to GridCraft's Developer console (Developer ▸ Console). What a script prints goes there too. */
  log(...values: unknown[]): Promise<void>;
}

declare const spreadsheet: Spreadsheet;

/**
 * Every command `spreadsheet.run` takes: its name, what it takes, and what the project must have
 * been granted. A command given a `path` needs `files` as well.
 */
interface SpreadsheetCommands {
"#;

/// The TypeScript declarations for a panel, served as `spreadsheet.d.ts` beside its page.
pub fn typings(commands: &[CommandDoc]) -> String {
    let mut out = String::from(TYPINGS);
    for command in commands {
        let Ok(id) = serde_json::to_string(&command.id) else { continue };
        // One line, which can't end the comment it sits in.
        let doc = format!("{}: {} Needs `{}`.", command.label, command.params, command.needs.code());
        let doc: String = doc.replace("*/", "* /").chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
        out.push_str(&format!("  /** {doc} */\n  {id}: SpreadsheetParams;\n"));
    }
    out.push_str("}\n");
    out
}

#[cfg(not(target_arch = "wasm32"))]
pub use server::Bridge;

#[cfg(target_arch = "wasm32")]
pub use web::Bridge;

/// The web build has no sockets to listen on: nothing is served.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;

    #[derive(Clone, Default)]
    pub struct Bridge;

    impl Bridge {
        pub fn new(_wake: impl Fn() + Send + Sync + 'static) -> Bridge {
            Bridge
        }

        pub fn with_limits(_wake: impl Fn() + Send + Sync + 'static, _limits: Limits) -> Bridge {
            Bridge
        }

        pub fn set_commands(&self, _commands: Vec<CommandDoc>) {}

        pub fn serve(&self, _places: &Places, _id: &str) -> Result<String, String> {
            Err("Panels can't be opened from this copy of GridCraft.".into())
        }

        pub fn stop(&self, _id: &str) {}

        pub fn serving(&self) -> Vec<String> {
            Vec::new()
        }

        pub fn take_calls(&self, _most: usize) -> Vec<Call> {
            Vec::new()
        }

        pub fn emit(&self, _event: &Event) {}

        pub fn listeners(&self, _id: &str) -> usize {
            0
        }

        pub fn script_calls(&self, _places: &Places, _id: &str) -> impl Fn(&Json) -> Result<Json, String> + Send + Sync + use<> {
            |_call: &Json| Err("The spreadsheet can't be called from this copy of GridCraft.".to_string())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod server {
    use std::collections::{HashMap, VecDeque};
    use std::io::Write;
    use std::net::{Ipv4Addr, Shutdown, SocketAddr, TcpListener, TcpStream};
    use std::sync::atomic::AtomicUsize;
    use std::sync::{Condvar, Mutex, PoisonError};
    use std::thread;

    use serde_json::json;

    use super::*;
    use crate::console::{self, Console};
    use crate::http::{self, Incoming, lock, same};
    use crate::python::{self, PythonHost, RunError};
    use crate::trust::{Status, TrustStore};
    use crate::{manifest, project, settings};

    /// The most requests answered at once for one project; more are turned away.
    const MAX_CONNECTIONS: usize = 32;
    /// The most events handed to a panel in one answer.
    const EVENTS_A_REPLY: usize = 64;

    /// What the browser is told when the project wasn't granted `network`: load and call nothing
    /// that isn't this bridge's.
    ///
    /// `'wasm-unsafe-eval'` lets the page run WebAssembly, which a browser otherwise refuses
    /// under a policy like this one. It gives a panel nothing it didn't have: a panel's own
    /// scripts already run, WebAssembly reaches the world only through them, and what it
    /// loads still comes from this bridge alone. Text is still not run as script (there is
    /// no `'unsafe-eval'`). It is what a project written in C needs to run in its panel.
    const CLOSED: &str = "default-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' data: blob:; connect-src 'self'; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'";
    /// With `network`: anything, but still in no other page's frame.
    const OPEN: &str = "default-src * 'unsafe-inline' 'unsafe-eval' data: blob:; object-src 'none'; base-uri 'self'; frame-ancestors 'none'";

    /// What every project's listener shares: the queue to the app.
    struct Shared {
        wake: Box<dyn Fn() + Send + Sync>,
        calls: Mutex<VecDeque<Call>>,
        commands: Mutex<Arc<Vec<CommandDoc>>>,
        limits: Limits,
    }

    /// What happened lately, for a project's panels to collect.
    #[derive(Default)]
    struct Events {
        /// The place of the latest event: 0 before the first.
        last: u64,
        /// The place of the latest event dropped because too many had piled up.
        dropped: u64,
        kept: VecDeque<(u64, Event)>,
    }

    /// One project being served.
    struct Site {
        id: String,
        token: String,
        port: u16,
        places: Places,
        stop: AtomicBool,
        open: AtomicUsize,
        events: Mutex<Events>,
        /// Told when there is an event to collect, or the project stops being served.
        news: Condvar,
        /// How many panels are being kept waiting for events.
        listening: AtomicUsize,
        shared: Arc<Shared>,
    }

    struct Owner {
        shared: Arc<Shared>,
        sites: Mutex<HashMap<String, Arc<Site>>>,
    }

    impl Drop for Owner {
        fn drop(&mut self) {
            for (_, site) in lock(&self.sites).drain() {
                halt(&site);
            }
        }
    }

    /// The bridge of one running app. Cloning it gives another handle to the same bridge; when
    /// the last handle goes, nothing is served any more.
    #[derive(Clone)]
    pub struct Bridge(Arc<Owner>);

    impl Bridge {
        /// A bridge that serves nothing yet. `wake` is called, from another thread, when a call
        /// is waiting for the app.
        pub fn new(wake: impl Fn() + Send + Sync + 'static) -> Bridge {
            Bridge::with_limits(wake, Limits::default())
        }

        pub fn with_limits(wake: impl Fn() + Send + Sync + 'static, limits: Limits) -> Bridge {
            let shared = Shared { wake: Box::new(wake), calls: Mutex::default(), commands: Mutex::default(), limits };
            Bridge(Arc::new(Owner { shared: Arc::new(shared), sites: Mutex::default() }))
        }

        /// The commands the typings list.
        pub fn set_commands(&self, commands: Vec<CommandDoc>) {
            *lock(&self.0.shared.commands) = Arc::new(commands);
        }

        /// Starts serving the project `id` and returns the address of its panel. The address holds
        /// the project's token: it is for the user's browser, and nothing else should be shown it.
        /// Refused unless the user trusts the project as it is on disk now and it has a panel.
        pub fn serve(&self, places: &Places, id: &str) -> Result<String, String> {
            let (found, _) = trusted(places, id)?;
            if found.manifest.as_ref().is_none_or(|m| m.panel.is_none()) {
                return Err(format!("{} has no panel.", found.name()));
            }
            let mut sites = lock(&self.0.sites);
            if let Some(site) = sites.get(id).filter(|site| site.places == *places) {
                return Ok(address(site));
            }
            if let Some(old) = sites.remove(id) {
                halt(&old);
            }
            let failed = |e: std::io::Error| format!("The panel could not be served: {e}");
            let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).map_err(failed)?;
            let port = listener.local_addr().map_err(failed)?.port();
            let site = Arc::new(Site {
                id: id.to_string(),
                token: http::token().map_err(|why| format!("The panel could not be served: {why}."))?,
                port,
                places: places.clone(),
                stop: AtomicBool::new(false),
                open: AtomicUsize::new(0),
                events: Mutex::default(),
                news: Condvar::new(),
                listening: AtomicUsize::new(0),
                shared: self.0.shared.clone(),
            });
            let serving = site.clone();
            thread::Builder::new().name("gridcraft-bridge".into()).spawn(move || listen(&listener, &serving)).map_err(failed)?;
            sites.insert(id.to_string(), site.clone());
            Ok(address(&site))
        }

        /// Stops serving the project `id`.
        pub fn stop(&self, id: &str) {
            if let Some(site) = lock(&self.0.sites).remove(id) {
                halt(&site);
            }
        }

        /// The projects being served.
        pub fn serving(&self) -> Vec<String> {
            let mut ids: Vec<String> = lock(&self.0.sites).keys().cloned().collect();
            ids.sort();
            ids
        }

        /// The calls waiting for the app, oldest first, at most `most` of them. Calls whose panel
        /// stopped waiting are dropped: they must not be carried out late.
        pub fn take_calls(&self, most: usize) -> Vec<Call> {
            let mut waiting = lock(&self.0.shared.calls);
            let mut taken = Vec::new();
            while taken.len() < most {
                let Some(call) = waiting.pop_front() else { break };
                if !call.is_abandoned() {
                    taken.push(call);
                }
            }
            taken
        }

        /// Tells the panels of every project being served that something happened.
        ///
        /// An event of the same name that is still the latest one waiting is replaced: a panel
        /// wants where the selection is, not every place it has been. Whether a project is told
        /// is decided when its panel asks, by what the user has granted it by then.
        pub fn emit(&self, event: &Event) {
            for site in lock(&self.0.sites).values() {
                let mut events = lock(&site.events);
                if events.kept.back().is_some_and(|(_, latest)| latest.name == event.name) {
                    events.kept.pop_back();
                }
                events.last += 1;
                let place = events.last;
                events.kept.push_back((place, event.clone()));
                while events.kept.len() > MAX_EVENTS {
                    if let Some((gone, _)) = events.kept.pop_front() {
                        events.dropped = gone;
                    }
                }
                drop(events);
                site.news.notify_all();
            }
        }

        /// How many panels of the project `id` are being kept waiting for events right now.
        pub fn listeners(&self, id: &str) -> usize {
            lock(&self.0.sites).get(id).map_or(0, |site| site.listening.load(Ordering::SeqCst))
        }

        /// What answers the calls a script of the project `id` makes to the spreadsheet while it
        /// runs ([`crate::python::Calls`]), for a script that was not started by a panel. Each
        /// call is put in the queue for the app, as a panel's is, after the project has been read
        /// from disk again: a script gets what its project is granted at that moment.
        pub fn script_calls(&self, places: &Places, id: &str) -> impl Fn(&Json) -> Result<Json, String> + Send + Sync + use<> {
            let (shared, places, id) = (self.0.shared.clone(), places.clone(), id.to_string());
            move |call: &Json| script_call(&shared, &places, &id, call)
        }
    }

    fn address(site: &Site) -> String {
        format!("http://127.0.0.1:{}/{}/", site.port, site.token)
    }

    fn halt(site: &Site) {
        site.stop.store(true, Ordering::SeqCst);
        // Panels waiting for events are answered now, with none.
        site.news.notify_all();
        // The listener is waiting for a connection: give it one, so that it sees it should stop.
        let _ = TcpStream::connect_timeout(&SocketAddr::from((Ipv4Addr::LOCALHOST, site.port)), Duration::from_secs(1));
    }

    fn listen(listener: &TcpListener, site: &Arc<Site>) {
        for stream in listener.incoming() {
            if site.stop.load(Ordering::SeqCst) {
                break;
            }
            let Ok(stream) = stream else {
                // Something is wrong with the socket: don't spin on it.
                thread::sleep(Duration::from_millis(50));
                continue;
            };
            if site.open.fetch_add(1, Ordering::SeqCst) >= MAX_CONNECTIONS {
                site.open.fetch_sub(1, Ordering::SeqCst);
                continue;
            }
            let serving = site.clone();
            let started = thread::Builder::new().name("gridcraft-bridge-request".into()).spawn(move || {
                answer(stream, &serving);
                serving.open.fetch_sub(1, Ordering::SeqCst);
            });
            if started.is_err() {
                site.open.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }

    /// The project as it is on disk now and what the user granted it, or why it isn't served.
    fn trusted(places: &Places, id: &str) -> Result<(project::Project, Vec<Permission>), String> {
        if !manifest::valid_id(id) {
            return Err("That is not a project.".into());
        }
        let found = project::load(&places.projects.join(id));
        match TrustStore::load(&places.trust).status(&found) {
            Status::Trusted(granted) => Ok((found, granted)),
            Status::Changed => Err(format!("{} has changed since you trusted it. Review it again in Developer > Projects.", found.name())),
            Status::Untrusted => Err(format!("{} is not trusted. Trust it in Developer > Projects first.", found.name())),
            Status::Problem(why) => Err(format!("{} can't be used: {why}.", found.name())),
        }
    }

    struct Reply {
        status: u16,
        reason: &'static str,
        kind: &'static str,
        body: Vec<u8>,
        allow: Option<&'static str>,
        /// The project was granted `network`.
        network: bool,
    }

    impl Reply {
        fn new(status: u16, reason: &'static str, kind: &'static str, body: Vec<u8>) -> Reply {
            Reply { status, reason, kind, body, allow: None, network: false }
        }

        fn plain(status: u16, reason: &'static str, text: &str) -> Reply {
            Reply::new(status, reason, "text/plain; charset=utf-8", text.as_bytes().to_vec())
        }

        fn json(status: u16, reason: &'static str, value: &Json) -> Reply {
            Reply::new(status, reason, "application/json", serde_json::to_vec(value).unwrap_or_default())
        }

        fn missing() -> Reply {
            Reply::plain(404, "Not Found", "There is nothing here.")
        }

        fn bad() -> Reply {
            Reply::plain(400, "Bad Request", "That request makes no sense.")
        }

        fn bytes(&self) -> Vec<u8> {
            let mut head = format!(
                "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nCross-Origin-Resource-Policy: same-origin\r\nContent-Security-Policy: {}\r\n",
                self.status,
                self.reason,
                self.kind,
                self.body.len(),
                if self.network { OPEN } else { CLOSED },
            );
            if let Some(allow) = self.allow {
                head.push_str(&format!("Allow: {allow}\r\n"));
            }
            head.push_str("\r\n");
            let mut out = head.into_bytes();
            out.extend_from_slice(&self.body);
            out
        }
    }

    fn answer(mut stream: TcpStream, site: &Site) {
        let wait = Some(site.shared.limits.read);
        if stream.set_read_timeout(wait).is_err() || stream.set_write_timeout(wait).is_err() {
            return;
        }
        let reply = match http::read(&mut stream, MAX_BODY_BYTES, site.shared.limits.read.saturating_mul(3)) {
            Ok(request) => respond(site, &request),
            Err(Some(refused)) => Reply::plain(refused.status, refused.reason, refused.text),
            // Nothing that makes a request arrived in time: there is nobody to answer.
            Err(None) => return,
        };
        let _ = stream.write_all(&reply.bytes());
        let _ = stream.flush();
        let _ = stream.shutdown(Shutdown::Both);
    }

    fn respond(site: &Site, request: &Incoming) -> Reply {
        // The browser must be talking to this address, not to a name someone pointed here.
        let Ok(Some(host)) = request.header("host") else { return Reply::bad() };
        if host != format!("127.0.0.1:{}", site.port) && host != format!("localhost:{}", site.port) {
            return Reply::plain(403, "Forbidden", "That is not this bridge's address.");
        }
        // A page of another site may load nothing from here. (Browsers that say where a request
        // comes from say "none" for an address the user opened themselves.)
        match request.header("sec-fetch-site") {
            Ok(None | Some("same-origin" | "none")) => {}
            _ => return Reply::plain(403, "Forbidden", "Only the project's own panel is served."),
        }
        let path = request.target.split(['?', '#']).next().unwrap_or("");
        let mut parts = path.splitn(3, '/');
        let (_, token, rest) = (parts.next(), parts.next().unwrap_or(""), parts.next());
        if !same(token, &site.token) {
            return Reply::missing();
        }
        let Some(rest) = rest else { return Reply::missing() };
        let calling = rest == "api";
        let (found, granted) = match trusted(&site.places, &site.id) {
            Ok(checked) => checked,
            Err(why) if calling => return Reply::json(403, "Forbidden", &json!({"ok": false, "error": why})),
            Err(why) => return Reply::plain(403, "Forbidden", &why),
        };
        let mut reply = match (request.method.as_str(), rest) {
            ("POST", "api") => api(site, request, &found, &granted, host),
            (_, "api") => Reply { allow: Some("POST"), ..Reply::plain(405, "Method Not Allowed", "Calls are made with POST.") },
            ("GET", "spreadsheet.js") => Reply::new(200, "OK", "text/javascript; charset=utf-8", CLIENT.as_bytes().to_vec()),
            ("GET", "spreadsheet.d.ts") => {
                let commands = lock(&site.shared.commands).clone();
                Reply::new(200, "OK", "text/plain; charset=utf-8", typings(&commands).into_bytes())
            }
            ("GET", rest) => file(&found, rest),
            _ => Reply { allow: Some("GET"), ..Reply::plain(405, "Method Not Allowed", "Pages are read with GET.") },
        };
        reply.network = granted.contains(&Permission::Network);
        reply
    }

    /// Undoes the `%XX` of a path. `None` when it isn't a path a browser would send.
    fn decode(text: &str) -> Option<String> {
        let bytes = text.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while let Some(&byte) = bytes.get(i) {
            if byte == b'%' {
                out.push(u8::from_str_radix(text.get(i + 1..i + 3)?, 16).ok()?);
                i += 3;
            } else {
                out.push(byte);
                i += 1;
            }
        }
        String::from_utf8(out).ok()
    }

    fn content_type(path: &str) -> &'static str {
        let extension = path.rsplit_once('.').map(|(_, e)| e.to_ascii_lowercase()).unwrap_or_default();
        match extension.as_str() {
            "html" | "htm" => "text/html; charset=utf-8",
            "js" | "mjs" => "text/javascript; charset=utf-8",
            "css" => "text/css; charset=utf-8",
            "json" | "map" => "application/json",
            "txt" | "md" | "ts" => "text/plain; charset=utf-8",
            "svg" => "image/svg+xml",
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "ico" => "image/x-icon",
            "woff" => "font/woff",
            "woff2" => "font/woff2",
            "wasm" => "application/wasm",
            _ => "application/octet-stream",
        }
    }

    /// A file of the project's panel: the page itself for the bare address, otherwise the path
    /// beside it. Only what is in the project's own list of files, which is what the user trusted.
    fn file(found: &project::Project, rest: &str) -> Reply {
        let Some(manifest) = &found.manifest else { return Reply::missing() };
        let Some(entry) = manifest.panel.as_deref() else { return Reply::missing() };
        let (folder, page) = entry.rsplit_once('/').map_or(("", entry), |(folder, page)| (folder, page));
        let wanted = if rest.is_empty() {
            page.to_string()
        } else {
            match decode(rest).and_then(|path| manifest::inner_path(&path).ok()) {
                Some(path) => path,
                None => return Reply::missing(),
            }
        };
        let inner = if folder.is_empty() { wanted } else { format!("{folder}/{wanted}") };
        // A panel at the top of its project doesn't get the manifest or the scripts with it.
        let script = manifest.python.as_deref().is_some_and(|scripts| inner.starts_with(&format!("{scripts}/")));
        if manifest::is_manifest(&inner) || script || !found.files.contains(&inner) {
            return Reply::missing();
        }
        match std::fs::read(found.dir.join(&inner)) {
            Ok(bytes) => Reply::new(200, "OK", content_type(&inner), bytes),
            Err(_) => Reply::missing(),
        }
    }

    fn api(site: &Site, request: &Incoming, found: &project::Project, granted: &[Permission], host: &str) -> Reply {
        let refuse = |status, reason, why: &str| Reply::json(status, reason, &json!({"ok": false, "error": why}));
        // Only the project's own page may call: the browser names the page that asks.
        if request.header("origin") != Ok(Some(format!("http://{host}").as_str())) {
            return refuse(403, "Forbidden", "Only this project's own panel may call GridCraft.");
        }
        let json_sent = request.header("content-type").ok().flatten().and_then(|kind| kind.split(';').next());
        if !json_sent.is_some_and(|kind| kind.trim().eq_ignore_ascii_case("application/json")) {
            return refuse(415, "Unsupported Media Type", "Calls are sent as JSON.");
        }
        let Ok(body) = serde_json::from_slice::<Json>(&request.body) else {
            return refuse(400, "Bad Request", "That call is not JSON.");
        };
        let answer = match call(site, found, granted, &body) {
            Ok(Json::Object(mut fields)) => {
                fields.insert("ok".into(), json!(true));
                Json::Object(fields)
            }
            Ok(other) => json!({"ok": true, "result": other}),
            Err(failure) => failure,
        };
        Reply::json(200, "OK", &answer)
    }

    fn fail(why: impl Into<String>) -> Json {
        json!({"ok": false, "error": why.into()})
    }

    /// A command and its parameters, out of a call: `{command, params?}`, or the pair
    /// `[command, params?]` that a batch may list instead.
    fn step(from: &Json) -> Result<(String, Json), Json> {
        let (name, params) = match from {
            Json::Array(pair) if pair.len() <= 2 => (pair.first(), pair.get(1)),
            _ => (from.get("command"), from.get("params")),
        };
        let command = name.and_then(Json::as_str).filter(|c| !c.is_empty() && c.len() <= 100);
        let Some(command) = command else { return Err(fail("A command needs a `command`: its name.")) };
        let params = match params {
            None | Some(Json::Null) => json!({}),
            Some(params @ Json::Object(_)) => params.clone(),
            Some(_) => return Err(fail(format!("The `params` of {command} must be an object."))),
        };
        Ok((command.to_string(), params))
    }

    /// Carries out one call. `Ok` holds the fields of the answer; `Err` the whole answer.
    fn call(site: &Site, found: &project::Project, granted: &[Permission], body: &Json) -> Result<Json, Json> {
        let version = found.manifest.as_ref().map(|m| m.version.as_str()).unwrap_or("");
        match body.get("call").and_then(Json::as_str) {
            Some("info") => Ok(json!({"result": {
                "project": {"id": found.id, "name": found.name(), "version": version},
                "granted": granted.iter().map(|p| p.code()).collect::<Vec<_>>(),
            }})),
            Some("runPython") => run_python(site, body),
            Some("events") => events(site, found, granted, body),
            Some("log") => {
                // A line for the Developer console. The console bounds what it keeps.
                let text = body.get("text").and_then(Json::as_str).unwrap_or("");
                Console::global().push(&site.id, "panel", console::Kind::Log, text);
                Ok(json!({}))
            }
            _ => match workbook_request(found.name(), body) {
                Some(asked) => {
                    let (request, key) = asked?;
                    ask(&site.shared, &site.id, found.name(), granted, request).map(|result| json!({key: result}))
                }
                None => Err(fail("That is not a call GridCraft knows.")),
            },
        }
    }

    /// What a call asks of the workbook, and the name its answer goes under: `None` when it is
    /// not one of the calls that need the workbook. A panel's calls and a script's are read here.
    fn workbook_request(name: &str, body: &Json) -> Option<Result<(Request, &'static str), Json>> {
        match body.get("call").and_then(Json::as_str) {
            Some("selection") => Some(Ok((Request::Selection, "result"))),
            Some("run") => Some(step(body).map(|(command, params)| (Request::Run { command, params }, "result"))),
            Some("batch") => Some(batch_request(name, body).map(|request| (request, "results"))),
            _ => None,
        }
    }

    fn batch_request(name: &str, body: &Json) -> Result<Request, Json> {
        let Some(steps) = body.get("commands").and_then(Json::as_array) else { return Err(fail("A batch needs `commands`: a list.")) };
        if steps.len() > MAX_BATCH {
            return Err(fail(format!("A batch can hold at most {MAX_BATCH} commands.")));
        }
        let commands = steps.iter().map(step).collect::<Result<Vec<_>, _>>()?;
        // The label of the undo step: the project's name unless a plain one was given.
        let given = body.get("label").and_then(Json::as_str).unwrap_or("");
        let label: String = given.chars().filter(|c| !c.is_control()).take(60).collect();
        let label = if label.trim().is_empty() { name.to_string() } else { label.trim().to_string() };
        Ok(Request::Batch { label, commands })
    }

    /// Answers what a project's script asks of the spreadsheet while it runs. The project is read
    /// from disk, and the user's decisions with it, for every call, as for a panel's request: a
    /// script whose project changed or lost its trust while it ran is answered nothing more.
    fn script_call(shared: &Shared, places: &Places, id: &str, body: &Json) -> Result<Json, String> {
        let said = |answer: Json| answer["error"].as_str().unwrap_or("The call failed.").to_string();
        let (found, granted) = trusted(places, id)?;
        match workbook_request(found.name(), body) {
            Some(asked) => {
                let (request, key) = asked.map_err(said)?;
                ask(shared, id, found.name(), &granted, request).map(|result| json!({key: result})).map_err(said)
            }
            None => Err("That is not something a script can ask of the spreadsheet.".into()),
        }
    }

    /// Hands a panel what happened since the event it last saw, keeping it waiting a while for
    /// something to happen if nothing has. Asked without `after`, it says where the events have
    /// got to, so that a panel begins from now.
    fn events(site: &Site, found: &project::Project, granted: &[Permission], body: &Json) -> Result<Json, Json> {
        // Every event says something about the open workbooks.
        if !granted.contains(&Permission::Read) {
            let why = format!("{} wasn't granted `read`, which events need.", found.name());
            Console::global().push(&site.id, "panel", console::Kind::Refused, &why);
            return Err(fail(why));
        }
        if let Some(names) = body.get("names").filter(|names| !names.is_null()) {
            let known = |name: &Json| name.as_str().is_some_and(|name| EVENTS.iter().any(|(event, _)| *event == name));
            if !names.as_array().is_some_and(|names| names.iter().all(known)) {
                let all: Vec<&str> = EVENTS.iter().map(|(event, _)| *event).collect();
                return Err(fail(format!("That is not an event GridCraft has. The events are: {}.", all.join(", "))));
            }
        }
        let after = match body.get("after") {
            None | Some(Json::Null) => return Ok(json!({"events": [], "next": lock(&site.events).last})),
            Some(place) => place.as_u64().ok_or_else(|| fail("`after` is the `next` of the answer before."))?,
        };
        if site.listening.fetch_add(1, Ordering::SeqCst) >= MAX_LISTENERS {
            site.listening.fetch_sub(1, Ordering::SeqCst);
            return Ok(json!({"events": [], "next": after, "again": 2}));
        }
        let collected = collect(site, after);
        site.listening.fetch_sub(1, Ordering::SeqCst);
        Ok(collected)
    }

    /// Waits for an event after the place `after`, and answers with those there are.
    fn collect(site: &Site, after: u64) -> Json {
        let events = lock(&site.events);
        // A place this bridge never gave out (the panel outlived a bridge before this one):
        // the panel begins again from now.
        if after > events.last {
            return json!({"events": [], "next": events.last, "missed": true});
        }
        let nothing_yet = |events: &mut Events| events.last <= after && !site.stop.load(Ordering::SeqCst);
        let (events, _) = site.news.wait_timeout_while(events, site.shared.limits.hold, nothing_yet).unwrap_or_else(PoisonError::into_inner);
        let fresh: Vec<Json> = events
            .kept
            .iter()
            .filter(|(place, _)| *place > after)
            .take(EVENTS_A_REPLY)
            .map(|(place, event)| {
                let mut told = event.detail.as_object().cloned().unwrap_or_default();
                told.insert("name".into(), json!(event.name));
                told.insert("seq".into(), json!(place));
                Json::Object(told)
            })
            .collect();
        let next = fresh.last().and_then(|event| event["seq"].as_u64()).unwrap_or(after);
        let mut answer = json!({"events": fresh, "next": next});
        if events.dropped > after {
            answer["missed"] = json!(true);
        }
        answer
    }

    /// Runs one of the project's scripts. The Python host checks for itself that the project is
    /// trusted and was granted `python`, and that the script is its own.
    fn run_python(site: &Site, body: &Json) -> Result<Json, Json> {
        let Some(script) = body.get("script").and_then(Json::as_str) else { return Err(fail("runPython needs the script's name.")) };
        let data = body.get("data").cloned().unwrap_or(Json::Null);
        let longest = python::MAX_TIMEOUT.as_secs_f64();
        let asked = body.get("timeout").and_then(Json::as_f64).filter(|seconds| seconds.is_finite() && *seconds >= 0.001);
        let timeout = asked.map_or(python::DEFAULT_TIMEOUT, |seconds| Duration::from_secs_f64(seconds.min(longest)));
        let interpreter = settings::load(&site.places.settings).python;
        let calls = |call: &Json| script_call(&site.shared, &site.places, &site.id, call);
        let at = &site.places;
        match PythonHost::global().run_with(&at.projects, &at.trust, &interpreter, &site.id, script, &data, timeout, Some(&calls)) {
            Ok(output) => Ok(json!({"result": output.result, "printed": output.printed})),
            Err(RunError::Script { error, trace }) => Err(json!({"ok": false, "error": error, "trace": trace})),
            Err(other) => Err(fail(other.to_string())),
        }
    }

    /// Puts a call of the project `id` (named `name`) in the queue for the app and waits for its
    /// answer.
    fn ask(shared: &Shared, id: &str, name: &str, granted: &[Permission], request: Request) -> Result<Json, Json> {
        let (reply, answer) = mpsc::channel();
        let abandoned = Arc::new(AtomicBool::new(false));
        {
            let mut waiting = lock(&shared.calls);
            if waiting.len() >= MAX_WAITING {
                return Err(fail("GridCraft has too many calls waiting. Try again in a moment."));
            }
            waiting.push_back(Call {
                project: id.to_string(),
                name: name.to_string(),
                granted: granted.to_vec(),
                request,
                reply,
                abandoned: abandoned.clone(),
            });
        }
        (shared.wake)();
        match answer.recv_timeout(shared.limits.answer) {
            Ok(result) => result.map_err(fail),
            Err(_) => {
                abandoned.store(true, Ordering::SeqCst);
                Err(fail("GridCraft did not answer in time. It may be waiting for you to answer a dialog."))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_call_needs_what_it_touches() {
        use Kind::{Changes, Reads};
        let none = serde_json::json!({});
        for (id, kind, params, permission) in [
            ("sheet.read", Reads, none.clone(), Permission::Read),
            ("document.inspect", Reads, none.clone(), Permission::Read),
            ("file.printPreview", Reads, none.clone(), Permission::Read),
            ("cell.set", Changes, none.clone(), Permission::Write),
            ("selection.set", Changes, none.clone(), Permission::Write),
            ("file.properties", Changes, none.clone(), Permission::Write),
            ("file.save", Changes, none.clone(), Permission::Files),
            ("file.new", Changes, none.clone(), Permission::Files),
            ("file.close", Changes, none.clone(), Permission::Files),
            // Whatever the command, a path is a file.
            ("insert.picture", Changes, serde_json::json!({"path": "C:/x.png"}), Permission::Files),
            ("data.fromTextCsv", Changes, serde_json::json!({"path": "secrets.txt"}), Permission::Files),
            ("sheet.read", Reads, serde_json::json!({"path": 3}), Permission::Files),
            ("cell.set", Changes, serde_json::json!({"path": null}), Permission::Write),
        ] {
            assert_eq!(needs(id, kind, &params), permission, "{id} {params}");
        }
        assert_eq!(needs("cell.set", Changes, &Json::Null), Permission::Write, "parameters that aren't an object");
    }

    #[test]
    fn the_typings_list_every_command_on_a_line_of_its_own() {
        let commands = vec![
            CommandDoc { id: "cell.set".into(), label: "Enter Cell".into(), params: "{cell?, input}".into(), needs: Permission::Write },
            CommandDoc { id: "odd\"id".into(), label: "Ends */ a comment\nand a line".into(), params: String::new(), needs: Permission::Read },
        ];
        let text = typings(&commands);
        assert!(text.contains("declare const spreadsheet: Spreadsheet;"));
        assert!(text.contains("  /** Enter Cell: {cell?, input} Needs `write`. */\n  \"cell.set\": SpreadsheetParams;\n"), "{text}");
        assert!(text.contains("  /** Ends * / a comment and a line:  Needs `read`. */\n  \"odd\\\"id\": SpreadsheetParams;\n"), "{text}");
        assert!(text.ends_with("}\n"));
        assert!(typings(&[]).ends_with("interface SpreadsheetCommands {\n}\n"));
    }

    #[test]
    fn the_client_finds_the_bridge_from_its_own_address_and_holds_no_secret() {
        assert!(CLIENT.contains("new URL(\"api\", document.currentScript.src)"));
        for call in ["run:", "batch:", "runPython:", "selection:", "read:", "write:", "info:"] {
            assert!(CLIENT.contains(call), "{call}");
        }
        assert!(!CLIENT.contains("127.0.0.1") && !CLIENT.contains("token"));
        // The workbook as objects: the handles, and their types.
        for part in ["sheet: (name) => sheet(name),", "range,", "workbook,", "\"edit.clearContents\"", "\"document.inspect\""] {
            assert!(CLIENT.contains(part), "{part}");
        }
        let declared = typings(&[]);
        for part in [
            "interface SpreadsheetRange {",
            "interface SpreadsheetSheet {",
            "interface SpreadsheetWorkbook {",
            "sheet(name?: string): SpreadsheetSheet;",
            "range(address: string, sheet?: string): SpreadsheetRange;",
            "workbook(): Promise<SpreadsheetWorkbook>;",
        ] {
            assert!(declared.contains(part), "{part}");
        }
        // Events and the console: the client's calls, and every event typed.
        for call in ["call: \"events\"", "call: \"log\"", "on,", "off,"] {
            assert!(CLIENT.contains(call), "{call}");
        }
        let declared = typings(&[]);
        for (event, tells) in EVENTS {
            assert!(declared.contains(&format!("  {event}: {{ name: \"{event}\"; seq: number;")), "{event} is typed");
            assert!(tells.starts_with('{'), "{event} says what it tells");
        }
        for method in ["on<K extends keyof SpreadsheetEvents>", "off<K extends keyof SpreadsheetEvents>", "log(...values: unknown[]): Promise<void>;"]
        {
            assert!(declared.contains(method), "{method}");
        }
    }
}

/// The bridge over real connections, as a browser would use it.
#[cfg(test)]
mod served {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::sync::atomic::AtomicUsize;
    use std::thread;

    use serde_json::json;

    use super::*;
    use crate::project;
    use crate::project::tests::TempDir;
    use crate::python::PythonHost;
    use crate::trust::TrustStore;

    const PAGE: &str = "<!doctype html><title>Clean Data</title><script src=\"spreadsheet.js\"></script>";

    fn manifest(permissions: &str, entry: &str) -> String {
        format!(
            "id = \"clean-data\"\nname = \"Clean Data\"\nversion = \"0.1.0\"\npermissions = [{permissions}]\n\n[panel]\nentry = \"{entry}\"\n\n[python]\nscripts = \"python\"\n"
        )
    }

    /// A trusted project being served, and what a test needs to reach it.
    struct Served {
        t: TempDir,
        places: Places,
        bridge: Bridge,
        port: u16,
        token: String,
        woken: Arc<AtomicUsize>,
    }

    impl Served {
        fn new(tag: &str, permissions: &str, limits: Limits) -> Served {
            Served::with_entry(tag, permissions, "panel/index.html", limits)
        }

        fn with_entry(tag: &str, permissions: &str, entry: &str, limits: Limits) -> Served {
            let t = TempDir::new(tag);
            t.write("projects/clean-data/project.toml", &manifest(permissions, entry));
            t.write(&format!("projects/clean-data/{entry}"), PAGE);
            t.write("projects/clean-data/panel/app.js", "console.log('panel');");
            t.write("projects/clean-data/python/clean.py", "def main(data):\n    print('cleaning')\n    return {'got': data}\n");
            t.write("projects/clean-data/python/broken.py", "def main(data):\n    raise ValueError('no good')\n");
            let places = Places { projects: t.0.join("projects"), trust: t.0.join("trusted-projects.json"), settings: t.0.join("developer.json") };
            let woken = Arc::new(AtomicUsize::new(0));
            let counted = woken.clone();
            let bridge = Bridge::with_limits(
                move || {
                    counted.fetch_add(1, Ordering::SeqCst);
                },
                limits,
            );
            let mut served = Served { t, places, bridge, port: 0, token: String::new(), woken };
            served.trust(false);
            let address = served.bridge.serve(&served.places, "clean-data").unwrap();
            let rest = address.strip_prefix("http://127.0.0.1:").unwrap();
            let (port, token) = rest.split_once('/').unwrap();
            served.port = port.parse().unwrap();
            served.token = token.trim_end_matches('/').to_string();
            assert_eq!(served.token.len(), 64, "{address}");
            served
        }

        fn trust(&self, follow_changes: bool) {
            let found = project::load(&self.places.projects.join("clean-data"));
            assert!(found.is_sound(), "{:?}", found.problem);
            TrustStore::load(&self.places.trust).trust(&found, follow_changes).unwrap();
        }

        /// Sends a request as written and returns the status, the headers and the body.
        fn send(&self, request: &str) -> (u16, String, String) {
            let mut stream = TcpStream::connect(("127.0.0.1", self.port)).unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(60))).unwrap();
            stream.write_all(request.as_bytes()).unwrap();
            let mut reply = Vec::new();
            let _ = stream.read_to_end(&mut reply);
            let text = String::from_utf8_lossy(&reply).into_owned();
            let (head, body) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
            let status = head.split(' ').nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            (status, head.to_string(), body.to_string())
        }

        fn host(&self) -> String {
            format!("127.0.0.1:{}", self.port)
        }

        /// What a browser sends to read `path` (after the token).
        fn get(&self, path: &str) -> (u16, String, String) {
            self.send(&format!("GET /{}/{path} HTTP/1.1\r\nHost: {}\r\nSec-Fetch-Site: none\r\n\r\n", self.token, self.host()))
        }

        fn post(&self, body: &str) -> String {
            format!(
                "POST /{}/api HTTP/1.1\r\nHost: {host}\r\nOrigin: http://{host}\r\nSec-Fetch-Site: same-origin\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
                self.token,
                body.len(),
                host = self.host()
            )
        }

        /// What the panel's own page sends to make a call, and the answer as JSON.
        fn call(&self, body: Json) -> Json {
            let (status, _, answer) = self.send(&self.post(&body.to_string()));
            assert_eq!(status, 200, "{answer}");
            serde_json::from_str(&answer).unwrap()
        }

        /// Acts as the app: waits for one call and answers it with what `answer` makes of it.
        fn app_answers(&self, answer: impl FnOnce(&Call) -> Result<Json, String>) {
            for _ in 0..2_000 {
                if let Some(call) = self.bridge.take_calls(1).pop() {
                    let result = answer(&call);
                    call.answer(result);
                    return;
                }
                thread::sleep(Duration::from_millis(2));
            }
            panic!("no call arrived");
        }
    }

    fn quick() -> Limits {
        Limits { read: Duration::from_millis(300), answer: Duration::from_millis(300), hold: Duration::from_millis(300) }
    }

    #[test]
    fn a_trusted_project_is_served_its_page_the_script_and_the_typings() {
        let s = Served::new("page", "\"read\", \"python\"", Limits::default());
        let (status, head, body) = s.get("");
        assert_eq!((status, body.as_str()), (200, PAGE));
        for header in [
            "Content-Type: text/html; charset=utf-8",
            "Cache-Control: no-store",
            "X-Content-Type-Options: nosniff",
            "Referrer-Policy: no-referrer",
            "Cross-Origin-Resource-Policy: same-origin",
            "Connection: close",
            // Not granted `network`: the browser is told to reach nothing but this bridge.
            "connect-src 'self'",
            "frame-ancestors 'none'",
        ] {
            assert!(head.contains(header), "{header} is missing from\n{head}");
        }
        // It may run WebAssembly (a project written in C runs so, in its panel), and still
        // may not run text as script or load from anywhere else.
        assert!(head.contains("default-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' data: blob:;"), "{head}");
        assert!(!head.contains("'unsafe-eval'") && !head.contains("default-src *"), "{head}");
        assert_eq!(s.get("index.html").2, PAGE);
        let (status, head, body) = s.get("app.js?v=2#top");
        assert_eq!((status, body.as_str()), (200, "console.log('panel');"));
        assert!(head.contains("Content-Type: text/javascript"));
        let (status, head, body) = s.get("spreadsheet.js");
        assert_eq!((status, body.as_str()), (200, CLIENT));
        assert!(head.contains("Content-Type: text/javascript"));
        s.bridge.set_commands(vec![CommandDoc {
            id: "cell.set".into(),
            label: "Enter Cell".into(),
            params: "{input}".into(),
            needs: Permission::Write,
        }]);
        let (status, _, body) = s.get("spreadsheet.d.ts");
        assert!(status == 200 && body.contains("\"cell.set\": SpreadsheetParams;"), "{body}");
        // Asking again gives the same address: the panel that is open stays valid.
        assert_eq!(s.bridge.serve(&s.places, "clean-data").unwrap(), format!("http://127.0.0.1:{}/{}/", s.port, s.token));
        assert_eq!(s.bridge.serving(), ["clean-data"]);
        assert_eq!(s.woken.load(Ordering::SeqCst), 0, "reading pages doesn't concern the app");
    }

    #[test]
    fn a_project_granted_network_is_served_with_an_open_policy() {
        let s = Served::new("network", "\"network\"", Limits::default());
        let (status, head, _) = s.get("");
        assert_eq!(status, 200);
        assert!(head.contains("default-src *") && !head.contains("connect-src 'self'"), "{head}");
        assert!(head.contains("frame-ancestors 'none'"));
    }

    #[test]
    fn nothing_is_served_without_the_token_the_address_or_the_users_trust() {
        let s = Served::new("closed", "\"read\"", Limits::default());
        let host = s.host();
        let wrong = "0".repeat(64);
        for (request, status) in [
            (format!("GET /{wrong}/ HTTP/1.1\r\nHost: {host}\r\n\r\n"), 404),
            (format!("GET / HTTP/1.1\r\nHost: {host}\r\n\r\n"), 404),
            (format!("GET /{} HTTP/1.1\r\nHost: {host}\r\n\r\n", s.token), 404),
            (format!("GET /{}x/ HTTP/1.1\r\nHost: {host}\r\n\r\n", s.token), 404),
            // A name pointed at this machine is not this bridge's address.
            (format!("GET /{}/ HTTP/1.1\r\nHost: evil.example:{}\r\n\r\n", s.token, s.port), 403),
            (format!("GET /{}/ HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n", s.token), 403),
            (format!("GET /{}/ HTTP/1.1\r\n\r\n", s.token), 400),
            (format!("GET /{}/ HTTP/1.1\r\nHost: {host}\r\nHost: {host}\r\n\r\n", s.token), 400),
            // A page of another site may load nothing from here.
            (format!("GET /{}/app.js HTTP/1.1\r\nHost: {host}\r\nSec-Fetch-Site: cross-site\r\n\r\n", s.token), 403),
            (format!("GET /{}/app.js HTTP/1.1\r\nHost: {host}\r\nSec-Fetch-Site: same-site\r\n\r\n", s.token), 403),
            (format!("DELETE /{}/ HTTP/1.1\r\nHost: {host}\r\n\r\n", s.token), 405),
            ("nonsense\r\n\r\n".to_string(), 400),
            (format!("GET /{}/ HTTP/2\r\nHost: {host}\r\n\r\n", s.token), 400),
            (format!("GET /{}/ HTTP/1.1\r\nHost: {host}\r\nTransfer-Encoding: chunked\r\n\r\n", s.token), 400),
            (format!("GET /{}/ HTTP/1.1\r\nHost: {host}\r\nX-Long: {}\r\n\r\n", s.token, "x".repeat(20_000)), 431),
        ] {
            let (got, _, body) = s.send(&request);
            assert_eq!(got, status, "{}\n{body}", request.chars().take(120).collect::<String>());
            assert!(!body.contains(PAGE));
        }
        assert_eq!(s.get("").0, 200, "the right request is still answered");

        // A file changes: what was trusted is no longer what is on disk.
        s.t.write("projects/clean-data/panel/app.js", "console.log('changed');");
        let (status, _, body) = s.get("");
        assert!(status == 403 && body.contains("changed since you trusted it"), "{status} {body}");
        assert_eq!(s.get("app.js").0, 403);
        let answer = s.send(&s.post("{\"call\": \"info\"}"));
        assert!(answer.0 == 403 && answer.2.contains("\"ok\":false"), "{answer:?}");
        // Trusted again as it is now, to follow its changes: served through further changes.
        s.trust(true);
        s.t.write("projects/clean-data/panel/app.js", "console.log('changed again');");
        assert_eq!(s.get("app.js").2, "console.log('changed again');");
        // Trust is taken back: at once, nothing.
        TrustStore::load(&s.places.trust).revoke("clean-data").unwrap();
        let (status, _, body) = s.get("");
        assert!(status == 403 && body.contains("is not trusted"), "{status} {body}");
        assert!(s.bridge.serve(&s.places, "clean-data").is_err(), "and it can't be served anew");
        assert!(s.bridge.serve(&s.places, "no-such-project").is_err());
        assert!(s.bridge.serve(&s.places, "../clean-data").is_err());
    }

    #[test]
    fn only_the_panels_own_files_are_served() {
        let s = Served::new("files", "\"read\"", Limits::default());
        for path in [
            "../project.toml",
            "%2e%2e/project.toml",
            "..%2fproject.toml",
            "..%2Fpython/clean.py",
            "%2e%2e%2fpython%2fclean.py",
            "..%5cproject.toml",
            "/project.toml",
            "//project.toml",
            "C:/Windows/win.ini",
            "C%3a/Windows/win.ini",
            "app.js%00.png",
            "app.js/",
            "app.js.",
            "%zz",
            "%",
            "%e9",
            "no-such-file.js",
            "project.toml",
            "python/clean.py",
            "api",
        ] {
            let (status, _, body) = s.get(path);
            assert!(status == 404 || status == 405, "{path} gave {status}");
            assert!(!body.contains("permissions") && !body.contains("def main"), "{path} leaked {body}");
        }
        // A panel kept at the top of its project still doesn't get the manifest or the scripts.
        let top = Served::with_entry("top", "\"read\"", "index.html", Limits::default());
        assert_eq!(top.get("").2, PAGE);
        assert_eq!(top.get("panel/app.js").0, 200);
        for path in ["project.toml", "python/clean.py", "python/broken.py"] {
            assert_eq!(top.get(path).0, 404, "{path}");
        }
        // The project becomes a repository after it was trusted. What git keeps there is not
        // the project's: it takes no trust away, and none of it is served, by any spelling.
        top.t.write("projects/clean-data/.git/config", "[remote \"origin\"]\n\turl = https://user:secret-token@example.invalid/x.git\n");
        top.t.write("projects/clean-data/.git/page.html", "<script>alert(1)</script>");
        assert_eq!(top.get("").2, PAGE, "it is still the project that was trusted");
        for path in [".git/config", ".git/page.html", "%2egit/config", ".GIT/config", "git~1/config", "panel/../.git/config", ".git%2fconfig"] {
            let (status, _, body) = top.get(path);
            assert_eq!(status, 404, "{path}");
            assert!(!body.contains("secret-token") && !body.contains("alert"), "{path} leaked {body}");
        }
        // A project with no panel is not served at all.
        let t = TempDir::new("no-panel");
        t.write("projects/plain/project.toml", "id = \"plain\"\nname = \"Plain\"\nversion = \"1\"\npermissions = []\n");
        let places = Places { projects: t.0.join("projects"), trust: t.0.join("trust.json"), settings: t.0.join("developer.json") };
        TrustStore::load(&places.trust).trust(&project::load(&places.projects.join("plain")), false).unwrap();
        assert_eq!(Bridge::new(|| {}).serve(&places, "plain"), Err("Plain has no panel.".to_string()));
    }

    #[test]
    fn calls_come_only_from_the_panels_own_page_and_as_json() {
        let s = Served::new("calls", "\"read\"", Limits::default());
        let (host, token) = (s.host(), s.token.clone());
        let body = "{\"call\": \"info\"}";
        let with = |headers: &str, body: &str| {
            format!("POST /{token}/api HTTP/1.1\r\nHost: {host}\r\n{headers}Content-Length: {}\r\n\r\n{body}", body.len())
        };
        let json = "Content-Type: application/json\r\n";
        for (request, status) in [
            // No page named, or another one: not the project's panel.
            (with(json, body), 403),
            (with(&format!("{json}Origin: https://evil.example\r\n"), body), 403),
            (with(&format!("{json}Origin: http://127.0.0.1:1\r\n"), body), 403),
            (with(&format!("{json}Origin: null\r\n"), body), 403),
            (with(&format!("{json}Origin: http://{host}\r\nOrigin: http://{host}\r\n"), body), 403),
            (with(&format!("{json}Origin: http://{host}\r\nSec-Fetch-Site: cross-site\r\n"), body), 403),
            // What a page of another site can send without asking: a form's kinds of body.
            (with(&format!("Content-Type: text/plain\r\nOrigin: http://{host}\r\n"), body), 415),
            (with(&format!("Origin: http://{host}\r\n"), body), 415),
            (with(&format!("{json}Origin: http://{host}\r\n"), "not json"), 400),
            (format!("GET /{token}/api HTTP/1.1\r\nHost: {host}\r\n\r\n"), 405),
            (format!("POST /{token}/ HTTP/1.1\r\nHost: {host}\r\nOrigin: http://{host}\r\n{json}Content-Length: 0\r\n\r\n"), 405),
            (
                format!(
                    "POST /{token}/api HTTP/1.1\r\nHost: {host}\r\nOrigin: http://{host}\r\n{json}Content-Length: {}\r\n\r\n",
                    MAX_BODY_BYTES + 1
                ),
                413,
            ),
            (format!("POST /{token}/api HTTP/1.1\r\nHost: {host}\r\nOrigin: http://{host}\r\n{json}Content-Length: many\r\n\r\n"), 400),
            (
                format!(
                    "POST /{token}/api HTTP/1.1\r\nHost: {host}\r\nOrigin: http://{host}\r\n{json}Content-Length: 2\r\nContent-Length: 2\r\n\r\n{{}}"
                ),
                400,
            ),
        ] {
            let (got, _, answer) = s.send(&request);
            assert_eq!(got, status, "{}\n{answer}", request.chars().take(200).collect::<String>());
        }
        assert_eq!(s.bridge.take_calls(10).len(), 0, "none of that reached the app");
        // The panel's own page, with JSON, is answered. `localhost` names the same bridge.
        let info = s.call(json!({"call": "info"}));
        assert_eq!(
            info,
            json!({"ok": true, "result": {"project": {"id": "clean-data", "name": "Clean Data", "version": "0.1.0"}, "granted": ["read"]}})
        );
        let local = format!("localhost:{}", s.port);
        let (status, _, _) = s.send(&format!(
            "POST /{token}/api HTTP/1.1\r\nHost: {local}\r\nOrigin: http://{local}\r\ncontent-type: Application/JSON; charset=utf-8\r\nContent-Length: {}\r\n\r\n{body}",
            body.len()
        ));
        assert_eq!(status, 200);
        // Calls GridCraft doesn't know, or can't make sense of, are refused without reaching the app.
        for bad in [
            json!({"call": "trust"}),
            json!({"call": 7}),
            json!([]),
            json!({"call": "run"}),
            json!({"call": "run", "command": ""}),
            json!({"call": "run", "command": "x".repeat(101)}),
            json!({"call": "run", "command": "cell.set", "params": [1]}),
            json!({"call": "batch"}),
            json!({"call": "batch", "commands": "cell.set"}),
            json!({"call": "batch", "commands": [{"params": {}}]}),
            json!({"call": "batch", "commands": vec![json!({"command": "cell.set"}); MAX_BATCH + 1]}),
            json!({"call": "runPython"}),
        ] {
            let answer = s.call(bad.clone());
            assert!(answer["ok"] == false && answer["error"].is_string(), "{bad} gave {answer}");
        }
        assert_eq!(s.bridge.take_calls(10).len(), 0);
    }

    #[test]
    fn a_call_for_the_workbook_waits_for_the_app_and_says_what_was_granted() {
        let s = Arc::new(Served::new("ask", "\"read\", \"write\"", Limits::default()));
        let panel = |body: Json| {
            let s = s.clone();
            thread::spawn(move || s.call(body))
        };
        let asking = panel(json!({"call": "run", "command": "cell.set", "params": {"cell": "A1", "input": "7"}}));
        s.app_answers(|call| {
            assert_eq!((call.project.as_str(), call.name.as_str()), ("clean-data", "Clean Data"));
            assert_eq!(call.granted, [Permission::Read, Permission::Write]);
            assert_eq!(call.request, Request::Run { command: "cell.set".into(), params: json!({"cell": "A1", "input": "7"}) });
            Ok(json!({"cell": "A1"}))
        });
        assert_eq!(asking.join().unwrap(), json!({"ok": true, "result": {"cell": "A1"}}));
        assert!(s.woken.load(Ordering::SeqCst) >= 1, "the app was woken for it");

        // What the app refuses comes back as the reason.
        let asking = panel(json!({"call": "run", "command": "file.save"}));
        s.app_answers(|call| {
            assert_eq!(call.request, Request::Run { command: "file.save".into(), params: json!({}) });
            Err("This project wasn't granted `files`.".into())
        });
        assert_eq!(asking.join().unwrap(), json!({"ok": false, "error": "This project wasn't granted `files`."}));

        // A batch arrives whole, under the project's name unless the panel names it.
        let steps = json!([{"command": "cell.set", "params": {"input": "1"}}, {"command": "home.bold"}]);
        let asking = panel(json!({"call": "batch", "commands": steps}));
        s.app_answers(|call| {
            let expected = vec![("cell.set".to_string(), json!({"input": "1"})), ("home.bold".to_string(), json!({}))];
            assert_eq!(call.request, Request::Batch { label: "Clean Data".into(), commands: expected });
            Ok(json!([1, 2]))
        });
        assert_eq!(asking.join().unwrap(), json!({"ok": true, "results": [1, 2]}));
        let asking = panel(json!({"call": "batch", "label": "  Tidy\u{7} up  ", "commands": []}));
        s.app_answers(|call| {
            assert_eq!(call.request, Request::Batch { label: "Tidy up".into(), commands: vec![] });
            Ok(json!([]))
        });
        assert_eq!(asking.join().unwrap()["ok"], true);

        let asking = panel(json!({"call": "selection"}));
        s.app_answers(|call| {
            assert_eq!(call.request, Request::Selection);
            Ok(json!({"range": "A1", "sheet": "Sheet1", "rows": [[1]]}))
        });
        assert_eq!(asking.join().unwrap()["result"]["range"], "A1");
    }

    #[test]
    fn a_call_the_app_never_answers_is_given_up_and_never_carried_out_late() {
        let s = Served::new("late", "\"write\"", quick());
        let answer = s.call(json!({"call": "run", "command": "cell.set"}));
        assert!(answer["ok"] == false && answer["error"].as_str().unwrap().contains("did not answer in time"), "{answer}");
        assert_eq!(s.woken.load(Ordering::SeqCst), 1);
        assert!(s.bridge.take_calls(10).is_empty(), "the app is never handed a call nobody waits for");
    }

    #[test]
    fn a_connection_that_says_nothing_is_dropped() {
        let s = Served::new("silent", "\"read\"", quick());
        let mut silent = TcpStream::connect(("127.0.0.1", s.port)).unwrap();
        silent.set_read_timeout(Some(Duration::from_secs(30))).unwrap();
        let mut half = TcpStream::connect(("127.0.0.1", s.port)).unwrap();
        half.set_read_timeout(Some(Duration::from_secs(30))).unwrap();
        half.write_all(b"GET / HTTP/1.1\r\nHost: ").unwrap();
        let mut got = Vec::new();
        assert_eq!(silent.read_to_end(&mut got).unwrap_or(0), 0, "it is closed without an answer");
        assert_eq!(half.read_to_end(&mut got).unwrap_or(0), 0);
        assert_eq!(s.get("").0, 200, "and the bridge goes on serving");
    }

    #[test]
    fn a_script_runs_for_a_project_granted_python_and_for_no_other() {
        // Refused before any process exists: this needs no Python on the machine.
        let without = Served::new("no-python", "\"read\"", Limits::default());
        let answer = without.call(json!({"call": "runPython", "script": "clean.py", "data": 1}));
        assert!(answer["ok"] == false && answer["error"].as_str().unwrap().contains("permission to run Python"), "{answer}");

        let s = Served::new("python", "\"python\"", Limits::default());
        for script in ["../project.toml", "nothing.py", "", "clean.py/../clean.py"] {
            let answer = s.call(json!({"call": "runPython", "script": script}));
            assert_eq!(answer["ok"], false, "{script} gave {answer}");
        }
        if PythonHost::global().interpreter("").is_err() {
            eprintln!("skipped the rest: there is no Python on this machine");
            return;
        }
        let answer = s.call(json!({"call": "runPython", "script": "clean.py", "data": {"rows": [[" a "]]}, "timeout": 30}));
        assert_eq!(answer["result"], json!({"got": {"rows": [[" a "]]}}), "{answer}");
        assert_eq!((answer["ok"].clone(), answer["printed"].as_str().map(str::trim)), (json!(true), Some("cleaning")));
        // No data is `None` to the script; a timeout that makes no sense is the usual one.
        for timeout in [json!(null), json!(-1), json!("soon"), json!(1e300)] {
            let answer = s.call(json!({"call": "runPython", "script": "clean.py", "timeout": timeout}));
            assert_eq!(answer["result"], json!({"got": null}), "{answer}");
        }
        let answer = s.call(json!({"call": "runPython", "script": "broken.py"}));
        assert!(answer["ok"] == false && answer["error"].as_str().unwrap().contains("no good"), "{answer}");
        assert!(answer["trace"].as_str().unwrap().contains("ValueError"), "{answer}");
        PythonHost::global().stop(&s.places.projects, "clean-data");
        assert!(s.bridge.take_calls(10).is_empty(), "scripts never concern the app");
    }

    #[test]
    fn stopping_closes_the_door() {
        let s = Served::new("stop", "\"read\"", quick());
        assert_eq!(s.get("").0, 200);
        s.bridge.stop("clean-data");
        s.bridge.stop("clean-data");
        assert!(s.bridge.serving().is_empty());
        let unanswered = |port: u16| {
            for _ in 0..200 {
                let Ok(mut stream) = TcpStream::connect(("127.0.0.1", port)) else { return true };
                let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
                let _ = stream.write_all(b"GET / HTTP/1.1\r\nHost: x\r\n\r\n");
                let mut reply = Vec::new();
                if stream.read_to_end(&mut reply).unwrap_or(0) == 0 {
                    return true;
                }
                thread::sleep(Duration::from_millis(10));
            }
            false
        };
        assert!(unanswered(s.port), "nothing answers on the old port");
        // Served again, it is at a new address with a new token.
        let again = s.bridge.serve(&s.places, "clean-data").unwrap();
        assert!(!again.contains(&s.token));
        let port: u16 = again.strip_prefix("http://127.0.0.1:").and_then(|rest| rest.split('/').next()).unwrap().parse().unwrap();
        // The bridge going away closes every door.
        let Served { bridge, t, .. } = s;
        drop(bridge);
        assert!(unanswered(port));
        drop(t);
    }

    fn selection(range: &str) -> Event {
        Event { name: "selectionChanged", detail: json!({"workbook": "Book1", "sheet": "Sheet1", "range": range, "active": range}) }
    }

    #[allow(clippy::disallowed_methods)]
    fn now() -> std::time::Instant {
        std::time::Instant::now()
    }

    /// Waits until this many of the project's panels are being kept waiting for events.
    fn waiting_panels(s: &Served, count: usize) {
        for _ in 0..2_000 {
            if s.bridge.listeners("clean-data") == count {
                return;
            }
            thread::sleep(Duration::from_millis(2));
        }
        panic!("{} panels are waiting, not {count}", s.bridge.listeners("clean-data"));
    }

    fn changed() -> Event {
        Event { name: "workbookChanged", detail: json!({"workbook": "Book1"}) }
    }

    /// The Developer console's entries that this project's panel made with this text in them.
    fn console_has(kind: crate::console::Kind, text: &str) -> bool {
        crate::console::Console::global().entries().iter().any(|e| e.project == "clean-data" && e.kind == kind && e.text.contains(text))
    }

    #[test]
    fn a_panel_is_told_what_happened_since_it_last_asked() {
        let s = Served::new("events", "\"read\"", quick());
        // Asked without a place, the bridge says where the events have got to: the panel begins from now.
        s.bridge.emit(&selection("A1"));
        let begin = s.call(json!({"call": "events"}));
        assert_eq!((&begin["ok"], &begin["events"], &begin["next"]), (&json!(true), &json!([]), &json!(1)), "{begin}");
        // What happens after that is handed over, with its name and its place.
        s.bridge.emit(&changed());
        s.bridge.emit(&selection("B2"));
        let told = s.call(json!({"call": "events", "after": 1}));
        assert_eq!(
            told["events"],
            json!([
                {"name": "workbookChanged", "seq": 2, "workbook": "Book1"},
                {"name": "selectionChanged", "seq": 3, "workbook": "Book1", "sheet": "Sheet1", "range": "B2", "active": "B2"},
            ]),
            "{told}"
        );
        assert_eq!(told["next"], json!(3));
        assert!(told.get("missed").is_none() && told.get("again").is_none(), "{told}");
        // Nothing new: the panel is kept waiting a while, then told so, and asks again from the same place.
        let asked = now();
        let quiet = s.call(json!({"call": "events", "after": 3}));
        assert_eq!((&quiet["events"], &quiet["next"]), (&json!([]), &json!(3)), "{quiet}");
        assert!(asked.elapsed() >= Duration::from_millis(250), "it waited: {:?}", asked.elapsed());
        // Events of one kind that pile up are thinned to the latest; another kind between them keeps them apart.
        for range in ["C1", "C2", "C3"] {
            s.bridge.emit(&selection(range));
        }
        s.bridge.emit(&changed());
        s.bridge.emit(&selection("D9"));
        let thinned = s.call(json!({"call": "events", "after": 3}));
        let seen: Vec<(String, String)> = thinned["events"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| (e["name"].as_str().unwrap().to_string(), e["range"].as_str().unwrap_or("").to_string()))
            .collect();
        assert_eq!(
            seen,
            [
                ("selectionChanged".to_string(), "C3".to_string()),
                ("workbookChanged".to_string(), String::new()),
                ("selectionChanged".to_string(), "D9".to_string())
            ]
        );
        assert!(thinned.get("missed").is_none(), "thinning is not missing: {thinned}");
        // A place this bridge never gave out: the panel begins again from now, and is told it missed some.
        let lost = s.call(json!({"call": "events", "after": 5_000}));
        assert_eq!((&lost["events"], &lost["next"], &lost["missed"]), (&json!([]), &thinned["next"], &json!(true)), "{lost}");
        // What makes no sense is refused, saying what the events are.
        let unknown = s.call(json!({"call": "events", "names": ["selectionChanged", "somethingElse"]}));
        assert!(unknown["ok"] == false && unknown["error"].as_str().unwrap().contains("workbookActivated"), "{unknown}");
        assert_eq!(s.call(json!({"call": "events", "names": ["workbookChanged"]}))["ok"], true);
        for after in [json!("soon"), json!(-1), json!(1.5), json!([1])] {
            let answer = s.call(json!({"call": "events", "after": after}));
            assert!(answer["ok"] == false && answer["error"].as_str().unwrap().contains("after"), "{after}: {answer}");
        }
        assert!(s.bridge.take_calls(10).is_empty(), "events never wait for the app");
    }

    #[test]
    fn a_panel_that_waits_is_answered_as_soon_as_something_happens() {
        let s = Arc::new(Served::new("events-wake", "\"read\"", Limits { hold: Duration::from_secs(20), ..quick() }));
        let waiting = s.clone();
        let asked = now();
        let panel = thread::spawn(move || waiting.call(json!({"call": "events", "after": 0})));
        waiting_panels(&s, 1);
        s.bridge.emit(&selection("B2"));
        let told = panel.join().unwrap();
        assert!(asked.elapsed() < Duration::from_secs(10), "it did not wait out the whole hold: {:?}", asked.elapsed());
        assert_eq!(told["events"][0]["range"], json!("B2"), "{told}");
        // And when its project stops being served, it is answered with nothing rather than left hanging.
        let waiting = s.clone();
        let asked = now();
        let panel = thread::spawn(move || {
            let (status, _, body) = waiting.send(&waiting.post(&json!({"call": "events", "after": 1}).to_string()));
            (status, body)
        });
        waiting_panels(&s, 1);
        s.bridge.stop("clean-data");
        let (status, body) = panel.join().unwrap();
        assert!(asked.elapsed() < Duration::from_secs(10), "{:?}", asked.elapsed());
        assert_eq!(status, 200, "{body}");
        assert_eq!(serde_json::from_str::<Json>(&body).unwrap()["events"], json!([]));
    }

    #[test]
    fn a_project_that_was_not_granted_read_is_told_nothing() {
        let s = Served::new("events-unread", "\"python\"", quick());
        s.bridge.emit(&selection("B2"));
        for call in [json!({"call": "events"}), json!({"call": "events", "after": 0}), json!({"call": "events", "names": ["selectionChanged"]})] {
            let answer = s.call(call.clone());
            assert!(answer["ok"] == false && answer["error"].as_str().unwrap().contains("`read`"), "{call}: {answer}");
            assert!(answer.get("events").is_none(), "{answer}");
        }
        assert!(console_has(crate::console::Kind::Refused, "which events need"), "the refusal is in the Developer console");
        // Granted `read` later (the user reviewed it again), the same panel is told from then on.
        s.t.write("projects/clean-data/project.toml", &manifest("\"read\", \"python\"", "panel/index.html"));
        assert_eq!(s.send(&s.post(&json!({"call": "events"}).to_string())).0, 403, "changed files are not trusted yet");
        s.trust(false);
        assert_eq!(s.call(json!({"call": "events", "after": 0}))["events"][0]["range"], json!("B2"));
    }

    #[test]
    fn events_that_pile_up_too_far_are_dropped_oldest_first_and_the_panel_is_told() {
        let s = Served::new("events-pile", "\"read\"", quick());
        // Kinds that alternate are never thinned.
        for n in 0..(MAX_EVENTS + 10) / 2 {
            s.bridge.emit(&selection(&format!("A{}", n + 1)));
            s.bridge.emit(&changed());
        }
        let first = s.call(json!({"call": "events", "after": 0}));
        let events = first["events"].as_array().unwrap();
        assert_eq!(first["missed"], json!(true), "ten were dropped before it asked");
        assert_eq!(events.len(), 64, "an answer holds only so many");
        assert_eq!(events[0]["seq"], json!(11), "the oldest ten are gone");
        // It collects the rest in order, and is not told again that it missed any.
        let mut next = first["next"].as_u64().unwrap();
        let mut collected = events.len();
        loop {
            let more = s.call(json!({"call": "events", "after": next}));
            let events = more["events"].as_array().unwrap();
            if events.is_empty() {
                break;
            }
            assert!(more.get("missed").is_none(), "{more}");
            assert_eq!(events[0]["seq"].as_u64(), Some(next + 1));
            collected += events.len();
            next = more["next"].as_u64().unwrap();
        }
        assert_eq!(collected, MAX_EVENTS);
        assert_eq!(next, (MAX_EVENTS + 10) as u64);
    }

    #[test]
    fn only_so_many_panels_are_kept_waiting() {
        let s = Arc::new(Served::new("events-many", "\"read\"", Limits { hold: Duration::from_secs(20), ..quick() }));
        let panels: Vec<_> = (0..MAX_LISTENERS)
            .map(|_| {
                let waiting = s.clone();
                thread::spawn(move || waiting.call(json!({"call": "events", "after": 0})))
            })
            .collect();
        waiting_panels(&s, MAX_LISTENERS);
        // One more is answered at once, and told when to ask again.
        let asked = now();
        let answer = s.call(json!({"call": "events", "after": 0}));
        let took = asked.elapsed();
        assert_eq!((&answer["ok"], &answer["events"], &answer["next"], &answer["again"]), (&json!(true), &json!([]), &json!(0), &json!(2)));
        assert!(took < Duration::from_secs(5), "{took:?}");
        // Calls that aren't waits still get through.
        assert_eq!(s.call(json!({"call": "info"}))["ok"], true);
        // Something happens: every waiting panel is told.
        s.bridge.emit(&selection("B2"));
        for panel in panels {
            assert_eq!(panel.join().unwrap()["events"][0]["range"], json!("B2"));
        }
    }

    #[test]
    fn a_panel_can_write_a_line_to_the_developer_console() {
        let s = Served::new("log", "\"read\"", quick());
        let answer = s.call(json!({"call": "log", "text": "the panel says h\u{e9}llo 4711"}));
        assert_eq!(answer, json!({"ok": true}));
        assert!(console_has(crate::console::Kind::Log, "the panel says h\u{e9}llo 4711"));
        let entry = crate::console::Console::global().entries().into_iter().find(|e| e.text.contains("4711")).unwrap();
        assert_eq!((entry.project.as_str(), entry.source.as_str()), ("clean-data", "panel"));
        // Nothing to say, or something that isn't text, writes nothing and fails nothing.
        for text in [json!(""), json!(null), json!(12), json!({"a": 1})] {
            assert_eq!(s.call(json!({"call": "log", "text": text}))["ok"], true);
        }
        assert!(!console_has(crate::console::Kind::Log, "\"a\""));
        // A project that isn't trusted any more can't write there either.
        s.t.write("projects/clean-data/panel/app.js", "console.log('changed');");
        let (status, _, body) = s.send(&s.post(&json!({"call": "log", "text": "from a changed project 4712"}).to_string()));
        assert_eq!(status, 403, "{body}");
        assert!(!console_has(crate::console::Kind::Log, "4712"));
        assert!(s.bridge.take_calls(10).is_empty(), "the console never waits for the app");
    }

    #[test]
    fn what_a_script_prints_and_why_it_fails_go_to_the_developer_console() {
        use crate::console::Kind;
        // A run that is refused is recorded, with no Python needed.
        let without = Served::new("console-refused", "\"read\"", Limits::default());
        without.call(json!({"call": "runPython", "script": "clean.py", "data": 1}));
        assert!(console_has(Kind::Refused, "permission to run Python"));
        if PythonHost::global().interpreter("").is_err() {
            eprintln!("skipped the rest: there is no Python on this machine");
            return;
        }
        let s = Served::new("console-python", "\"python\"", Limits::default());
        s.t.write("projects/clean-data/python/talk.py", "def main(data):\n    print('talking about', data)\n    return 1\n");
        s.trust(false);
        assert_eq!(s.call(json!({"call": "runPython", "script": "talk.py", "data": "console-test-8841", "timeout": 30}))["result"], json!(1));
        let entries = crate::console::Console::global().entries();
        let printed = entries.iter().find(|e| e.kind == Kind::Printed && e.text.contains("console-test-8841")).expect("what it printed");
        assert_eq!((printed.project.as_str(), printed.source.as_str()), ("clean-data", "talk.py"));
        // A script that fails leaves its error and its traceback.
        s.call(json!({"call": "runPython", "script": "broken.py"}));
        let entries = crate::console::Console::global().entries();
        let failed = entries.iter().rev().find(|e| e.kind == Kind::Error && e.source == "broken.py").expect("why it failed");
        assert!(failed.text.contains("no good") && failed.text.contains("ValueError"), "{}", failed.text);
        PythonHost::global().stop(&s.places.projects, "clean-data");
    }

    #[test]
    fn what_a_script_asks_of_the_spreadsheet_waits_for_the_app_like_a_panels_call() {
        let s = Arc::new(Served::new("script-calls", "\"read\", \"write\", \"python\"", Limits::default()));
        if PythonHost::global().interpreter("").is_err() {
            eprintln!("skipped: there is no Python on this machine");
            return;
        }
        s.t.write(
            "projects/clean-data/python/writer.py",
            "import spreadsheet\n\ndef main(data):\n    first = spreadsheet.run('cell.set', {'cell': 'B2', 'input': data})\n    try:\n        spreadsheet.selection()\n    except spreadsheet.Error as error:\n        return [first, str(error)]\n    return [first, 'answered']\n",
        );
        s.trust(false);
        // The app: it answers the first call, and before that changes one of the project's files,
        // as an update or tampering would.
        let app = s.clone();
        let acting = thread::spawn(move || {
            let mut seen = Vec::new();
            app.app_answers(|call| {
                seen.push((call.project.clone(), call.name.clone(), call.granted.clone(), call.request.clone()));
                app.t.write("projects/clean-data/panel/app.js", "console.log('changed while a script ran');");
                Ok(json!({"cells": 1}))
            });
            seen
        });
        let answer = s.call(json!({"call": "runPython", "script": "writer.py", "data": "from python", "timeout": 60}));
        let seen = acting.join().unwrap();
        assert_eq!(seen.len(), 1);
        let (project, name, granted, request) = &seen[0];
        assert_eq!((project.as_str(), name.as_str()), ("clean-data", "Clean Data"));
        assert_eq!(granted, &[Permission::Read, Permission::Write, Permission::Python], "the call carries what the project was granted");
        assert_eq!(request, &Request::Run { command: "cell.set".into(), params: json!({"cell": "B2", "input": "from python"}) });
        // The second call found the project changed: it was refused, and never reached the app.
        assert_eq!(answer["ok"], true, "{answer}");
        assert_eq!(answer["result"][0], json!({"cells": 1}));
        assert!(answer["result"][1].as_str().unwrap().contains("has changed since you trusted it"), "{answer}");
        assert!(s.bridge.take_calls(10).is_empty());
        PythonHost::global().stop(&s.places.projects, "clean-data");
    }

    #[test]
    fn a_script_started_outside_a_panel_is_answered_through_the_same_queue() {
        let s = Arc::new(Served::new("script-calls-outside", "\"read\"", quick()));
        // What the window uses for Developer ▸ Projects ▸ Run: no panel is involved.
        let calls = s.bridge.script_calls(&s.places, "clean-data");
        let app = s.clone();
        let acting = thread::spawn(move || {
            let mut granted = Vec::new();
            app.app_answers(|call| {
                granted = call.granted.clone();
                Ok(json!({"sheet": "Sheet1", "range": "A1", "rows": [[1]]}))
            });
            granted
        });
        let answer = calls(&json!({"call": "selection", "calling": 4, "n": 1})).unwrap();
        assert_eq!(answer, json!({"result": {"sheet": "Sheet1", "range": "A1", "rows": [[1]]}}));
        assert_eq!(acting.join().unwrap(), [Permission::Read]);
        // Only what needs the workbook: a script can't reach the bridge's other calls this way.
        for call in [json!({"call": "runPython", "script": "clean.py"}), json!({"call": "events"}), json!({"call": "info"}), json!({}), json!(null)] {
            let refused = calls(&call).unwrap_err();
            assert!(refused.contains("not something a script can ask"), "{call}: {refused}");
        }
        assert!(calls(&json!({"call": "run"})).unwrap_err().contains("needs a `command`"));
        assert!(calls(&json!({"call": "batch", "commands": 3})).unwrap_err().contains("a list"));
        // The app never answers: the script is told, and the call is not carried out late.
        assert!(calls(&json!({"call": "selection"})).unwrap_err().contains("did not answer in time"));
        assert!(s.bridge.take_calls(10).is_empty());
        // A project that is not trusted, or is not a project at all, is answered nothing.
        s.t.write("projects/clean-data/panel/app.js", "console.log('changed');");
        assert!(calls(&json!({"call": "selection"})).unwrap_err().contains("has changed since you trusted it"));
        let stranger = s.bridge.script_calls(&s.places, "../elsewhere");
        assert!(stranger(&json!({"call": "selection"})).is_err());
        assert!(s.bridge.take_calls(10).is_empty());
    }

    #[test]
    fn tokens_are_long_and_never_the_same() {
        let a = Served::new("token-a", "\"read\"", Limits::default());
        let b = Served::new("token-b", "\"read\"", Limits::default());
        assert!(a.token != b.token && a.port != b.port);
        assert!(a.token.chars().all(|c| c.is_ascii_hexdigit()));
        // One project's token opens nothing at another's door.
        assert_eq!(b.send(&format!("GET /{}/ HTTP/1.1\r\nHost: {}\r\n\r\n", a.token, b.host())).0, 404);
    }
}
