//! Panels: a trusted module's page, opened in the user's browser, and the calls it makes.
//!
//! The page is served by the bridge (`gridcraft_aniefork_dev::bridge`), which answers what it can
//! itself (files, Python) and queues the calls that need the workbook. Those are carried out
//! here, on the UI thread, once a frame ([`poll`]).
//!
//! A module is a program calling GridCraft, so it gets what any program does: every command of
//! the engine, by id, and commands never open dialogs for it. What it may run depends on what
//! the user granted it:
//!
//! - a command that only reports needs `read`; any other needs `write`;
//! - a command that reads or writes a file (the `file.` commands, and any command given a
//!   `path`) needs `files`, and even then GridCraft's own settings folder, where the modules and
//!   the user's decisions are kept, is out of reach.
//!
//! A batch is several commands as **one undo step**: all of them, or, if one fails, none.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use gridcraft_aniefork_dev::Permission;
use gridcraft_aniefork_dev::bridge::{self, Bridge, Call, CommandDoc, Kind, Request};
use gridcraft_engine::{CommandSpec, HistoryEntry};
use serde_json::Value as Json;

use super::modules;
use crate::SheetApp;

/// The most calls carried out in one frame, so that a busy panel can't hold up the window.
const CALLS_A_FRAME: usize = 16;
/// As many undo steps as the engine keeps.
const UNDO_LIMIT: usize = 100;

fn bridge_id() -> egui::Id {
    egui::Id::new("aniefork.panels.bridge")
}

fn kind(spec: &CommandSpec) -> Kind {
    if spec.journal || spec.undoable { Kind::Changes } else { Kind::Reads }
}

/// The engine's commands, as the typings served to a panel describe them.
fn command_docs() -> Vec<CommandDoc> {
    let doc = |spec: &CommandSpec| CommandDoc {
        id: spec.id.to_string(),
        label: spec.label.trim_end_matches('…').to_string(),
        params: spec.params.to_string(),
        needs: bridge::needs(spec.id, kind(spec), &Json::Null),
    };
    gridcraft_engine::command_specs().iter().map(doc).collect()
}

/// This window's bridge, if a panel was ever opened from it.
fn running(ctx: &egui::Context) -> Option<Bridge> {
    ctx.data(|d| d.get_temp::<Bridge>(bridge_id()))
}

/// This window's bridge, made when first wanted. It serves nothing until a panel is opened.
fn the_bridge(ctx: &egui::Context) -> Bridge {
    if let Some(found) = running(ctx) {
        return found;
    }
    let window = ctx.clone();
    let made = Bridge::new(move || window.request_repaint());
    made.set_commands(command_docs());
    ctx.data_mut(|d| d.insert_temp(bridge_id(), made.clone()));
    made
}

/// Opens the panel of the module `id` in the user's browser. Refused unless the user trusts the
/// module as it is on disk now.
pub fn open(app: &SheetApp, ctx: &egui::Context, id: &str) -> Result<String, String> {
    let at = modules::places().ok_or(modules::UNAVAILABLE)?;
    let Some(open_url) = &app.services.open_url else { return Err("This copy of GridCraft can't open a browser.".into()) };
    let address = the_bridge(ctx).serve(&at, id)?;
    // The address holds the module's token. It goes to the browser, and is shown nowhere else.
    open_url(&address);
    Ok("The panel is opening in your browser.".into())
}

/// Stops serving the module `id` (its trust was taken back).
pub fn stop(ctx: &egui::Context, id: &str) {
    if let Some(bridge) = running(ctx) {
        bridge.stop(id);
    }
}

/// The modules whose panels are being served.
pub fn serving(ctx: &egui::Context) -> Vec<String> {
    running(ctx).map(|bridge| bridge.serving()).unwrap_or_default()
}

/// Carries out the calls panels made since the last frame. Called once a frame.
pub fn poll(app: &mut SheetApp, ctx: &egui::Context) {
    let Some(bridge) = running(ctx) else { return };
    let calls = bridge.take_calls(CALLS_A_FRAME);
    if calls.is_empty() {
        return;
    }
    for call in calls {
        let result = carry_out(app, &call);
        call.answer(result);
    }
    // What was done should show, and more calls may be waiting.
    ctx.request_repaint();
}

fn carry_out(app: &mut SheetApp, call: &Call) -> Result<Json, String> {
    match &call.request {
        Request::Run { command, params } => {
            allowed(call, command, params)?;
            headless(app, command, params)
        }
        Request::Batch { label, commands } => batch(app, call, label, commands),
        Request::Selection => {
            if !call.granted.contains(&Permission::Read) {
                return Err(not_granted(call, Permission::Read, "the selection"));
            }
            modules::selection_data(app)
        }
    }
}

fn not_granted(call: &Call, needed: Permission, what: &str) -> String {
    format!("{} wasn't granted `{}`, which {what} needs.", call.name, needed.code())
}

/// Whether the module may make this call; if so, what the call needs.
fn allowed(call: &Call, command: &str, params: &Json) -> Result<Permission, String> {
    let Some(spec) = gridcraft_engine::find_command(command) else { return Err(format!("{command} is not a command.")) };
    let needed = bridge::needs(spec.id, kind(spec), params);
    if !call.granted.contains(&needed) {
        return Err(not_granted(call, needed, command));
    }
    if needed == Permission::Files {
        outside_settings(params)?;
    }
    Ok(needed)
}

/// A path as the file system names it. What doesn't exist yet is put back after the nearest
/// folder that does. `None` when that can't be told.
fn resolved(path: &Path) -> Option<PathBuf> {
    let mut existing = path;
    let mut rest = Vec::new();
    loop {
        if let Ok(real) = std::fs::canonicalize(existing) {
            return Some(rest.iter().rev().fold(real, |whole, part| whole.join(part)));
        }
        rest.push(existing.file_name()?);
        existing = existing.parent()?;
    }
}

/// GridCraft's own settings folder holds the modules and the user's decisions about them. No
/// module reads or writes there, whatever it was granted.
fn outside_settings(params: &Json) -> Result<(), String> {
    let Some(path) = params.get("path").and_then(Json::as_str) else { return Ok(()) };
    let Some(settings) = modules::places().and_then(|at| at.trust.parent().map(Path::to_path_buf)) else { return Ok(()) };
    // A settings folder that isn't there holds nothing to keep from anyone.
    let Ok(settings) = std::fs::canonicalize(settings) else { return Ok(()) };
    match resolved(Path::new(path)) {
        Some(target) if !target.starts_with(&settings) => Ok(()),
        _ => Err("Modules can't read or write GridCraft's own settings folder.".into()),
    }
}

/// Runs a command for a caller that is a program: nothing it asks of the user interface (a
/// dialog, a message, a link to open) happens, and a command that would have asked for a file
/// says what it is missing instead.
fn headless(app: &mut SheetApp, command: &str, params: &Json) -> Result<Json, String> {
    app.session.execute_headless(command, params.clone()).map_err(|e| crate::clean_error(&e.to_string()))
}

/// Several commands as one undo step. Every step is checked before the first is run; if one
/// fails, the workbook, the selection and the undo history are put back as they were.
fn batch(app: &mut SheetApp, call: &Call, label: &str, commands: &[(String, Json)]) -> Result<Json, String> {
    for (command, params) in commands {
        if allowed(call, command, params)? == Permission::Files {
            return Err(format!("{command} works with files, which can't be undone: it can't be part of one undo step. Run it on its own."));
        }
    }
    let Some(doc) = app.session.active() else { return Err("There is no workbook open.".into()) };
    let (uid, before, selection, undo, redo) = (doc.uid, doc.wb.clone(), doc.selection.clone(), doc.undo.clone(), doc.redo.clone());
    let mut results = Vec::with_capacity(commands.len());
    let mut failure = None;
    for (index, (command, params)) in commands.iter().enumerate() {
        match headless(app, command, params) {
            Ok(result) => results.push(result),
            Err(why) => {
                failure = Some(format!("Step {} ({command}) failed: {why}", index + 1));
                break;
            }
        }
        if app.session.active().map(|d| d.uid) != Some(uid) {
            failure = Some(format!("Step {} ({command}) left the workbook the batch began in.", index + 1));
            break;
        }
    }
    // Back to the workbook the batch began in, if a step went to another.
    if let Some(index) = app.session.documents().iter().position(|d| d.uid == uid) {
        app.session.set_active(index);
    }
    let Some(doc) = app.session.active_mut().filter(|d| d.uid == uid) else {
        return Err(failure.unwrap_or_else(|| "The workbook the batch began in was closed.".into()));
    };
    if let Some(why) = failure {
        doc.wb = before;
        doc.selection = selection;
        doc.undo = undo;
        doc.redo = redo;
        doc.calc.rebuild(&doc.wb);
        doc.revision += 1;
        return Err(format!("{why} Nothing was changed."));
    }
    if Arc::ptr_eq(&doc.wb, &before) {
        // Nothing changed: the history is as it was, what can be redone included.
        doc.undo = undo;
        doc.redo = redo;
    } else {
        let mut history = undo;
        history.push(HistoryEntry { label: label.to_string(), wb: before, selection });
        if history.len() > UNDO_LIMIT {
            history.remove(0);
        }
        doc.undo = history;
        doc.redo.clear();
    }
    Ok(Json::Array(results))
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::io::{Read, Write};
    use std::rc::Rc;

    use egui_kittest::kittest::Queryable as _;
    use gridcraft_aniefork_dev::python::PythonHost;
    use gridcraft_aniefork_dev::{TrustStore, module};
    use serde_json::json;

    use super::*;
    use crate::aniefork::modules::tests::{Sandbox, app_with, click, drive, harness, yes};

    type Harness = egui_kittest::Harness<'static, SheetApp>;

    /// Installs and trusts a module that asks for `permissions`, the way a user who said yes
    /// would have. Its panel is one page; its script hands back what it was given.
    fn install(s: &Sandbox, id: &str, permissions: &[&str]) {
        let dir = s.0.join("modules").join(id);
        std::fs::create_dir_all(dir.join("panel")).unwrap();
        std::fs::create_dir_all(dir.join("python")).unwrap();
        let asked: Vec<String> = permissions.iter().map(|p| format!("\"{p}\"")).collect();
        let manifest = format!(
            "id = \"{id}\"\nname = \"Module {id}\"\nversion = \"1.0\"\npermissions = [{}]\n\n[panel]\nentry = \"panel/index.html\"\n\n[python]\nscripts = \"python\"\n",
            asked.join(", ")
        );
        std::fs::write(dir.join("module.toml"), manifest).unwrap();
        std::fs::write(dir.join("panel/index.html"), "<script src=\"spreadsheet.js\"></script>").unwrap();
        std::fs::write(dir.join("python/echo.py"), "def main(data):\n    return data\n").unwrap();
        let found = module::load(&dir);
        assert!(found.is_sound(), "{:?}", found.problem);
        TrustStore::load(&s.0.join("trusted-modules.json")).trust(&found, false).unwrap();
    }

    /// An app whose "browser" is a list of the addresses it was asked to open.
    fn app() -> (SheetApp, Rc<RefCell<Vec<String>>>) {
        let opened = Rc::new(RefCell::new(Vec::new()));
        let seen = opened.clone();
        let mut a = app_with(Some(&yes()));
        a.services.open_url = Some(Box::new(move |address: &str| seen.borrow_mut().push(address.to_string())));
        (a, opened)
    }

    /// Opens the panel of `id` and returns its address, as the browser was given it.
    fn open_panel(h: &mut Harness, opened: &Rc<RefCell<Vec<String>>>, id: &str) -> String {
        let ctx = h.ctx.clone();
        open(h.state(), &ctx, id).unwrap();
        let address = opened.borrow().last().cloned().unwrap();
        assert!(address.starts_with("http://127.0.0.1:") && address.ends_with('/'), "{address}");
        address
    }

    /// Sends what a browser would and returns the status and the body.
    fn exchange(address: &str, request: impl Fn(&str, &str) -> String) -> (u16, String) {
        let rest = address.strip_prefix("http://").unwrap();
        let (host, token) = rest.trim_end_matches('/').split_once('/').unwrap();
        let mut stream = std::net::TcpStream::connect(host).unwrap();
        stream.set_read_timeout(Some(std::time::Duration::from_secs(60))).unwrap();
        stream.write_all(request(host, token).as_bytes()).unwrap();
        let mut reply = Vec::new();
        let _ = stream.read_to_end(&mut reply);
        let text = String::from_utf8_lossy(&reply).into_owned();
        let (head, body) = text.split_once("\r\n\r\n").unwrap_or((text.as_str(), ""));
        (head.split(' ').nth(1).and_then(|s| s.parse().ok()).unwrap_or(0), body.to_string())
    }

    fn get(address: &str, path: &str) -> (u16, String) {
        exchange(address, |host, token| format!("GET /{token}/{path} HTTP/1.1\r\nHost: {host}\r\nSec-Fetch-Site: none\r\n\r\n"))
    }

    /// A call from the panel's page. It is made on another thread, as a browser's is, while the
    /// window goes on drawing frames: the call is carried out in one of them.
    fn call(h: &mut Harness, address: &str, body: Json) -> Json {
        let (address, text) = (address.to_string(), body.to_string());
        let asking = std::thread::spawn(move || {
            exchange(&address, |host, token| {
                format!(
                    "POST /{token}/api HTTP/1.1\r\nHost: {host}\r\nOrigin: http://{host}\r\nSec-Fetch-Site: same-origin\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{text}",
                    text.len()
                )
            })
        });
        for _ in 0..20_000 {
            if asking.is_finished() {
                break;
            }
            h.step();
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        let (status, body) = asking.join().unwrap();
        serde_json::from_str(&body).unwrap_or_else(|_| json!({"status": status, "body": body}))
    }

    fn run(h: &mut Harness, address: &str, command: &str, params: Json) -> Json {
        call(h, address, json!({"call": "run", "command": command, "params": params}))
    }

    fn cell(h: &mut Harness, at: &str) -> Json {
        h.state_mut().session.run("cell.get", json!({"cell": at})).unwrap()["value"].clone()
    }

    fn history(h: &Harness) -> (usize, usize) {
        let doc = h.state().session.active().unwrap();
        (doc.undo.len(), doc.redo.len())
    }

    /// The whole path in the real window: Open Panel, and then the panel's page, as a browser
    /// would load it, reads the selection and changes the workbook.
    #[test]
    fn a_trusted_modules_panel_opens_in_the_browser_and_reaches_the_workbook() {
        let s = Sandbox::new("panel");
        crate::aniefork::modules::create("Clean Data").unwrap();
        let (mut a, opened) = app();
        a.session.run("range.setValues", json!({"range": "A1", "values": [["north", 12], ["south", 7.5]]})).unwrap();
        a.session.run("selection.set", json!({"range": "A1:B2"})).unwrap();
        let fingerprint = crate::aniefork::modules::list()["modules"][0]["fingerprint"].as_str().unwrap().to_string();
        crate::aniefork::modules::trust(&a, "clean-data", &fingerprint, false).unwrap();
        let mut h = harness(a);
        click(&mut h, "Open Panel");
        let address = opened.borrow().last().cloned().unwrap();
        assert_eq!(serving(&h.ctx), ["clean-data"]);

        // The page, the script that gives it its `spreadsheet`, and the types for TypeScript.
        let (status, page) = get(&address, "");
        assert!(status == 200 && page.contains("<script src=\"spreadsheet.js\"></script>") && page.contains("Clean Data"), "{status} {page}");
        assert!(get(&address, "spreadsheet.js").1.contains("window.spreadsheet"));
        let typings = get(&address, "spreadsheet.d.ts").1;
        assert!(
            typings.contains("\"range.setValues\": SpreadsheetParams;") && typings.contains("Needs `write`."),
            "{}",
            &typings[..400.min(typings.len())]
        );
        assert!(typings.contains("\"sheet.read\": SpreadsheetParams;") && typings.contains("\"file.save\": SpreadsheetParams;"));

        let info = call(&mut h, &address, json!({"call": "info"}));
        assert_eq!(info["result"]["granted"], json!(["read", "write", "python"]), "{info}");
        // The selection, as a script is handed it.
        let selection = call(&mut h, &address, json!({"call": "selection"}));
        assert_eq!(selection, json!({"ok": true, "result": {"range": "A1:B2", "sheet": "Sheet1", "rows": [["north", 12.0], ["south", 7.5]]}}));
        // Any command, by its id: writing is one undo step, like a user's own.
        let before = history(&h);
        let wrote = run(&mut h, &address, "range.setValues", json!({"range": "D1", "values": [["total", "=SUM(B1:B2)"]]}));
        assert_eq!(wrote["ok"], json!(true), "{wrote}");
        assert_eq!((cell(&mut h, "D1"), cell(&mut h, "E1")), (json!("total"), json!(19.5)));
        assert_eq!(history(&h), (before.0 + 1, 0));
        let read = run(&mut h, &address, "sheet.read", json!({"range": "D1:E1", "formulas": true}));
        assert_eq!(read["result"]["values"], json!([["total", "=SUM(B1:B2)"]]));

        // What isn't a command of the engine is not a call: nothing of the window's own.
        for id in ["no.such.command", "view.theme", "ui.dialog", "developer.modules", "file.options", ""] {
            let answer = run(&mut h, &address, id, json!({"name": "about", "mode": "dark"}));
            assert_eq!(answer["ok"], json!(false), "{id}: {answer}");
        }
        assert!(!h.state().ui.dark);
        // A command that would have opened a dialog for a user opens none for a module.
        let asked = run(&mut h, &address, "home.formatCells", json!({}));
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some("modules"), "{asked}");
        assert!(h.state().message.is_none());
        // Files weren't granted to the starter.
        let refused = run(&mut h, &address, "file.open", json!({}));
        assert!(refused["error"].as_str().is_some_and(|why| why.contains("wasn't granted `files`")), "{refused}");

        // The address holds the module's token: nothing the control channel can read shows it.
        let token = address.trim_end_matches('/').rsplit('/').next().unwrap().to_string();
        let ctx = h.ctx.clone();
        for method in ["ui.inspect", "developer.modules", "engine.commands"] {
            let seen = match crate::control::handle(h.state_mut(), &ctx, method, &json!({})) {
                crate::control::Outcome::Done(v) => v.to_string(),
                _ => String::new(),
            };
            assert!(!seen.contains(&token) && !seen.contains("127.0.0.1"), "{method} shows the panel's address");
        }
        assert!(h.query_by_label_contains("127.0.0.1").is_none(), "nor is it drawn anywhere");

        // Trust is taken back: the panel is answered nothing more.
        click(&mut h, "Stop trusting");
        assert!(serving(&h.ctx).is_empty());
        let _ = s;
    }

    #[test]
    fn a_module_runs_only_what_it_was_granted() {
        let s = Sandbox::new("granted");
        install(&s, "reader", &["read"]);
        install(&s, "writer", &["write"]);
        install(&s, "filer", &["read", "write", "files"]);
        let (a, opened) = app();
        let mut h = harness(a);
        let reader = open_panel(&mut h, &opened, "reader");
        let writer = open_panel(&mut h, &opened, "writer");
        let filer = open_panel(&mut h, &opened, "filer");
        let refusal = |answer: &Json, permission: &str| {
            answer["ok"] == false && answer["error"].as_str().is_some_and(|why| why.contains(&format!("wasn't granted `{permission}`")))
        };

        // Reading only.
        assert_eq!(run(&mut h, &reader, "sheet.read", json!({"range": "A1"}))["ok"], json!(true));
        assert_eq!(call(&mut h, &reader, json!({"call": "selection"}))["ok"], json!(true));
        for (command, params, needs) in [
            ("cell.set", json!({"cell": "A1", "input": "x"}), "write"),
            ("selection.set", json!({"range": "B2"}), "write"),
            ("home.bold", json!({"range": "A1"}), "write"),
            ("file.properties", json!({"title": "Mine"}), "write"),
            ("file.save", json!({}), "files"),
            ("file.new", json!({}), "files"),
            ("file.close", json!({"force": true}), "files"),
            // Whatever the command, a path is a file.
            ("sheet.read", json!({"path": "C:/secrets.txt"}), "files"),
        ] {
            let answer = run(&mut h, &reader, command, params);
            assert!(refusal(&answer, needs), "{command}: {answer}");
        }
        let answer = call(&mut h, &reader, json!({"call": "batch", "commands": [["cell.set", {"cell": "A1", "input": "x"}]]}));
        assert_eq!(answer["ok"], json!(false), "the steps of a batch are checked like any call: {answer}");
        let answer = call(&mut h, &reader, json!({"call": "runPython", "script": "echo.py", "data": 1}));
        assert!(answer["ok"] == false && answer["error"].as_str().unwrap().contains("permission to run Python"), "{answer}");
        assert_eq!((cell(&mut h, "A1"), h.state().session.documents().len()), (Json::Null, 1), "none of that happened");

        // Writing only: it changes, and is told nothing.
        assert_eq!(run(&mut h, &writer, "cell.set", json!({"cell": "A1", "input": "42"}))["ok"], json!(true));
        assert_eq!(cell(&mut h, "A1"), json!(42.0));
        assert!(refusal(&run(&mut h, &writer, "sheet.read", json!({})), "read"));
        assert!(refusal(&run(&mut h, &writer, "document.inspect", json!({})), "read"));
        assert!(refusal(&call(&mut h, &writer, json!({"call": "selection"})), "read"));

        // Files: anywhere but GridCraft's own settings folder, where the modules and the user's
        // decisions are kept.
        let elsewhere = std::env::temp_dir().join(format!("gridcraft-ui-panels-elsewhere-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&elsewhere);
        std::fs::create_dir_all(&elsewhere).unwrap();
        let saved = elsewhere.join("Saved.xlsx");
        let answer = run(&mut h, &filer, "file.save", json!({"path": saved}));
        assert!(answer["ok"] == true && saved.is_file(), "{answer}");
        let trust_file = s.0.join("trusted-modules.json");
        let decisions = std::fs::read(&trust_file).unwrap();
        let roundabout = s.0.join("modules").join("..").join("trusted-modules.json");
        for path in [
            trust_file.clone(),
            roundabout,
            s.0.join("developer.json"),
            s.0.join("modules/filer/panel/index.html"),
            s.0.join("modules/new-module/module.toml"),
            s.0.join("not/there/yet/book.xlsx"),
        ] {
            for command in ["file.save", "file.open", "file.exportCsv"] {
                let answer = run(&mut h, &filer, command, json!({"path": path}));
                assert!(answer["error"].as_str().is_some_and(|why| why.contains("settings folder")), "{command} {path:?}: {answer}");
            }
        }
        assert_eq!(std::fs::read(&trust_file).unwrap(), decisions, "the user's decisions are untouched");
        assert!(!s.0.join("developer.json").exists() && !s.0.join("not").exists());
        // A file command still can't be one step of a batch: it can't be undone.
        let answer =
            call(&mut h, &filer, json!({"call": "batch", "commands": [["cell.set", {"cell": "C3", "input": "1"}], ["file.save", {"path": saved}]]}));
        assert!(answer["error"].as_str().is_some_and(|why| why.contains("can't be part of one undo step")), "{answer}");
        assert_eq!(cell(&mut h, "C3"), Json::Null, "and nothing of that batch was run");
        let _ = std::fs::remove_dir_all(&elsewhere);
        for id in ["reader", "writer", "filer"] {
            stop(&h.ctx, id);
        }
    }

    #[test]
    fn a_batch_is_one_undo_step_and_all_of_it_or_nothing() {
        let s = Sandbox::new("batch");
        install(&s, "tidy", &["read", "write"]);
        let (mut a, opened) = app();
        a.session.run("cell.set", json!({"cell": "A1", "input": "1"})).unwrap();
        let mut h = harness(a);
        let panel = open_panel(&mut h, &opened, "tidy");
        let before = history(&h);

        let steps = json!([["cell.set", {"cell": "A1", "input": "10"}], {"command": "cell.set", "params": {"cell": "B1", "input": "20"}}, ["home.bold", {"range": "A1:B1"}], ["sheet.read", {"range": "A1:B1"}]]);
        let answer = call(&mut h, &panel, json!({"call": "batch", "commands": steps}));
        assert_eq!(answer["ok"], json!(true), "{answer}");
        assert_eq!(answer["results"].as_array().map(Vec::len), Some(4), "one result for each step");
        assert_eq!(answer["results"][3]["values"], json!([[10.0, 20.0]]), "a later step sees what an earlier one did");
        assert_eq!((cell(&mut h, "A1"), cell(&mut h, "B1")), (json!(10.0), json!(20.0)));
        assert_eq!(history(&h), (before.0 + 1, 0), "three changes, one undo step");
        assert_eq!(h.state().session.active().unwrap().undo.last().map(|e| e.label.as_str()), Some("Module tidy"), "named after the module");
        h.state_mut().run("edit.undo", json!({})).unwrap();
        assert_eq!((cell(&mut h, "A1"), cell(&mut h, "B1")), (json!(1.0), Json::Null), "one undo takes all of it back");
        h.state_mut().run("edit.redo", json!({})).unwrap();
        assert_eq!((cell(&mut h, "A1"), cell(&mut h, "B1")), (json!(10.0), json!(20.0)));
        let answer = call(&mut h, &panel, json!({"call": "batch", "label": "Tidy up", "commands": [["cell.set", {"cell": "C1", "input": "3"}]]}));
        assert_eq!(answer["ok"], json!(true));
        assert_eq!(h.state().session.active().unwrap().undo.last().map(|e| e.label.as_str()), Some("Tidy up"), "or as the panel names it");

        // A step fails part of the way through: the workbook, the selection and the history are
        // as they were, what can be redone included.
        h.state_mut().run("edit.undo", json!({})).unwrap();
        h.state_mut().session.run("selection.set", json!({"range": "A1"})).unwrap();
        let (as_it_was, selected) = (history(&h), h.state().session.active().unwrap().selection.active);
        assert!(as_it_was.1 > 0);
        let steps = json!([["cell.set", {"cell": "D1", "input": "4"}], ["selection.set", {"range": "F9"}], ["range.setValues", {"range": "A1"}], ["cell.set", {"cell": "E1", "input": "6"}]]);
        let answer = call(&mut h, &panel, json!({"call": "batch", "commands": steps}));
        let why = answer["error"].as_str().unwrap_or("");
        assert!(answer["ok"] == false && why.contains("Step 3 (range.setValues) failed") && why.ends_with("Nothing was changed."), "{answer}");
        assert_eq!((cell(&mut h, "D1"), cell(&mut h, "E1")), (Json::Null, Json::Null));
        assert_eq!(history(&h), as_it_was);
        assert_eq!(h.state().session.active().unwrap().selection.active, selected);
        // A batch that isn't allowed, or names no command, runs none of its steps.
        let answer =
            call(&mut h, &panel, json!({"call": "batch", "commands": [["cell.set", {"cell": "D1", "input": "4"}], ["no.such.command", {}]]}));
        assert!(answer["ok"] == false && cell(&mut h, "D1") == Json::Null, "{answer}");
        // One that changes nothing leaves the history alone.
        let answer = call(&mut h, &panel, json!({"call": "batch", "commands": [["sheet.read", {}], ["selection.set", {"range": "A1"}]]}));
        assert_eq!((answer["ok"].clone(), history(&h)), (json!(true), as_it_was), "{answer}");
        assert_eq!(call(&mut h, &panel, json!({"call": "batch", "commands": []})), json!({"ok": true, "results": []}));

        // With the history full, a batch is still one step, and what was before it is kept.
        for n in 0..110 {
            h.state_mut().session.run("cell.set", json!({"cell": "H1", "input": n.to_string()})).unwrap();
        }
        let full = history(&h).0;
        let steps: Vec<Json> = (0..8).map(|n| json!(["cell.set", {"cell": format!("J{}", n + 1), "input": "x"}])).collect();
        assert_eq!(call(&mut h, &panel, json!({"call": "batch", "commands": steps}))["ok"], json!(true));
        assert_eq!(history(&h).0, full);
        h.state_mut().run("edit.undo", json!({})).unwrap();
        assert_eq!((cell(&mut h, "J1"), cell(&mut h, "J8"), cell(&mut h, "H1")), (Json::Null, Json::Null, json!(109.0)));
        h.state_mut().run("edit.undo", json!({})).unwrap();
        assert_eq!(cell(&mut h, "H1"), json!(108.0), "the steps before the batch are still there to undo");
        stop(&h.ctx, "tidy");
    }

    /// What the starter module's panel does when its button is pressed, call by call.
    #[test]
    fn the_starter_panel_cleans_the_selection_through_python_as_one_undo_step() {
        let _s = Sandbox::new("round-trip");
        if let Err(why) = PythonHost::global().interpreter("") {
            eprintln!("skipped: {why}");
            return;
        }
        crate::aniefork::modules::create("Clean Data").unwrap();
        let (mut a, opened) = app();
        a.session.run("range.setValues", json!({"range": "B2", "values": [["  north ", 12], ["south  ", 7.5]]})).unwrap();
        a.session.run("selection.set", json!({"range": "B2:C3"})).unwrap();
        let fingerprint = crate::aniefork::modules::list()["modules"][0]["fingerprint"].as_str().unwrap().to_string();
        crate::aniefork::modules::trust(&a, "clean-data", &fingerprint, false).unwrap();
        let mut h = harness(a);
        let panel = open_panel(&mut h, &opened, "clean-data");
        let before = history(&h);

        // const data = await spreadsheet.selection();
        let data = call(&mut h, &panel, json!({"call": "selection"}))["result"].clone();
        assert_eq!(data["range"], json!("B2:C3"));
        // const result = await spreadsheet.runPython("clean.py", data);
        let ran = call(&mut h, &panel, json!({"call": "runPython", "script": "clean.py", "data": data}));
        assert_eq!(ran["result"], json!({"rows": [["north", 12.0], ["south", 7.5]]}), "{ran}");
        // await spreadsheet.write(data.range, result.rows);
        let wrote = run(&mut h, &panel, "range.setValues", json!({"range": data["range"], "values": ran["result"]["rows"]}));
        assert_eq!(wrote["ok"], json!(true), "{wrote}");
        assert_eq!((cell(&mut h, "B2"), cell(&mut h, "B3"), cell(&mut h, "C3")), (json!("north"), json!("south"), json!(7.5)));
        assert_eq!(history(&h), (before.0 + 1, 0), "one undo step");
        h.state_mut().run("edit.undo", json!({})).unwrap();
        assert_eq!(cell(&mut h, "B2"), json!("  north "), "and Undo puts the selection back as it was");

        // The control channel being busy in the window doesn't stop a panel the person opened.
        drive(&mut h);
        assert_eq!(call(&mut h, &panel, json!({"call": "selection"}))["ok"], json!(true));
        // The module's files change under the open panel: nothing more is carried out for it.
        let dir = crate::aniefork::modules::places().unwrap().modules.join("clean-data");
        std::fs::write(dir.join("python/clean.py"), "def main(data):\n    return 'changed'\n").unwrap();
        for body in [
            json!({"call": "selection"}),
            json!({"call": "runPython", "script": "clean.py"}),
            json!({"call": "run", "command": "cell.set", "params": {"cell": "A1", "input": "x"}}),
        ] {
            let answer = call(&mut h, &panel, body.clone());
            assert!(answer["ok"] == false && answer["error"].as_str().unwrap().contains("changed since you trusted it"), "{body}: {answer}");
        }
        assert_eq!(cell(&mut h, "A1"), Json::Null);
        assert_eq!(get(&panel, "").0, 403);
        PythonHost::global().stop(&crate::aniefork::modules::places().unwrap().modules, "clean-data");
        stop(&h.ctx, "clean-data");
    }
}
