//! Developer ▸ Modules: the modules the user has installed, the one place where trust in a module
//! is given or taken back, and a bench for running a trusted module's scripts by hand.
//!
//! A module does nothing until the user trusts it. Trusting is a decision about exact contents:
//! the dialog shows what the module asks to be allowed and what it contains, and what gets
//! trusted is the fingerprint the user was shown. If the files changed while the user was
//! reading, the decision is refused and they look again.
//!
//! Giving trust takes two things no agent can supply. There is no command for it, so the control
//! channel and MCP cannot ask (`developer.modules` only reports). And the last step is a question
//! drawn by the operating system, outside the app's own window contents, which the control
//! channel's synthetic clicks cannot answer. A build that can't ask it can't trust modules.
//!
//! A trusted module that was granted `python` can have its scripts run from here on the current
//! selection. The script runs in a Python process of its own, off the UI thread; see
//! `gridcraft_aniefork_dev::python`.

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};

use gridcraft_aniefork_dev::python::{DEFAULT_TIMEOUT, Output, PythonHost, RunError};
use gridcraft_aniefork_dev::{Module, TrustStore, describe, manifest, module, settings, starter};
use serde_json::{Value as Json, json};

use crate::SheetApp;
use crate::dialogs::Dialog;
use crate::theme::Tokens;

/// The dialog's name, as `ui.dialog {name}` takes it.
pub const NAME: &str = "modules";

const FOLLOW: &str = "Keep trusting it as its files change (only for a module you are writing yourself)";
/// The most of a result shown in the dialog (characters).
const SHOWN: usize = 6_000;

/// Where this run keeps modules, the user's decisions and the subsystem's settings.
struct Places {
    modules: PathBuf,
    trust: PathBuf,
    settings: PathBuf,
}

/// `None` when the run keeps no settings (the web build, a run without preferences): then there
/// are no modules.
#[cfg(not(test))]
fn places() -> Option<Places> {
    use gridcraft_aniefork_dev::paths;
    Some(Places { modules: paths::modules_dir()?, trust: paths::trust_file()?, settings: paths::settings_file()? })
}

/// In tests: only the folder a test set up for itself, never the developer's own settings.
#[cfg(test)]
fn places() -> Option<Places> {
    tests::sandbox().map(|dir| Places { modules: dir.join("modules"), trust: dir.join("trusted-modules.json"), settings: dir.join("developer.json") })
}

const UNAVAILABLE: &str = "Modules are not available in this run.";

/// The user's modules and where each stands: what `developer.modules` reports and the dialog
/// shows. Reading only.
pub fn list() -> Json {
    let Some(at) = places() else { return json!({"available": false, "folder": null, "modules": []}) };
    let store = TrustStore::load(&at.trust);
    let modules: Vec<Json> = module::scan(&at.modules).iter().map(|m| describe(m, &store)).collect();
    json!({"available": true, "folder": at.modules.display().to_string(), "modules": modules})
}

/// What the dialog starts from.
pub fn values() -> Json {
    let python = places().map(|at| settings::load(&at.settings).python).unwrap_or_default();
    json!({"list": list(), "review": null, "follow": false, "newName": "", "python": python, "pythonStatus": "", "run": null})
}

/// The question the operating system puts to the user: what they are about to allow, in full.
fn question(found: &Module, follow_changes: bool) -> String {
    let mut text = format!("Trust \"{}\"", found.name());
    if let Some(m) = &found.manifest {
        text.push_str(&format!(" {}?\n\nIt will be allowed to:\n", m.version));
        if m.permissions.is_empty() {
            text.push_str("- Show its panel, and nothing more\n");
        }
        for p in &m.permissions {
            text.push_str(&format!("- {}\n", p.meaning()));
        }
    }
    let files = found.files.len();
    text.push_str(&format!("\nIt has {files} file{}. ", if files == 1 { "" } else { "s" }));
    text.push_str(if follow_changes {
        "You chose to keep trusting it as its files change."
    } else {
        "If any of them changes, GridCraft will ask you again."
    });
    text
}

/// The user pressed "Trust this module" while reviewing `id`. `reviewed` is the fingerprint they
/// were shown: only that is trusted, and only after the operating system's own confirmation.
fn trust(app: &SheetApp, id: &str, reviewed: &str, follow_changes: bool) -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    if !manifest::valid_id(id) {
        return Err("That is not a module.".into());
    }
    let found = module::load(&at.modules.join(id));
    if found.fingerprint.as_deref() != Some(reviewed) {
        return Err(format!("{} changed while you were looking at it. Review it again before trusting it.", found.name()));
    }
    if !found.is_sound() {
        return Err(format!("{} can't be trusted: {}.", found.name(), found.problem.as_deref().unwrap_or("it could not be read")));
    }
    let Some(confirm) = &app.services.confirm else {
        return Err("This copy of GridCraft can't ask your operating system to confirm, so it can't trust modules.".into());
    };
    if !confirm("Trust this module?", &question(&found, follow_changes)) {
        return Ok(format!("{} was not trusted.", found.name()));
    }
    // What is recorded is the fingerprint that was reviewed, whatever happened on disk meanwhile.
    TrustStore::load(&at.trust).trust(&found, follow_changes)?;
    Ok(format!("{} is trusted.", found.name()))
}

fn stop_trusting(id: &str) -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    TrustStore::load(&at.trust).revoke(id)?;
    PythonHost::global().stop(&at.modules, id);
    Ok("The module is no longer trusted.".into())
}

fn create(name: &str) -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    std::fs::create_dir_all(&at.modules).map_err(|e| format!("The modules folder could not be created: {e}"))?;
    let created = starter::create(&at.modules, name)?;
    Ok(format!("{} was created. It is not trusted yet.", created.name()))
}

/// Remembers which Python to use and says whether it answers.
fn check_python(python: &str) -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    settings::save(&at.settings, &settings::Settings { python: python.trim().to_string() })?;
    let host = PythonHost::global();
    host.look_again();
    host.interpreter(python).map(|found| found.describe())
}

/// A script running off the UI thread, and where its result will appear.
#[derive(Clone)]
struct Job {
    slot: Arc<Mutex<Option<Result<Output, RunError>>>>,
}

fn job_id() -> egui::Id {
    egui::Id::new("aniefork.modules.job")
}

/// The current selection, as the data a script is handed: `{range, sheet, rows}`.
fn selection_data(app: &mut SheetApp) -> Result<Json, String> {
    let range = app.session.active().map(|d| d.selection.current().a1()).ok_or("There is no workbook open.")?;
    let read = app.session.run("sheet.read", json!({"range": range})).map_err(|e| crate::clean_error(&e))?;
    Ok(json!({"range": read["range"], "sheet": read["sheet"], "rows": read["values"]}))
}

/// Starts `script` of module `id` on the selection. The trust check is the host's own, on the
/// thread that runs it.
fn start(app: &mut SheetApp, ctx: &egui::Context, id: &str, script: &str) -> Result<(), String> {
    let at = places().ok_or(UNAVAILABLE)?;
    let data = selection_data(app)?;
    let python = settings::load(&at.settings).python;
    let job = Job { slot: Arc::new(Mutex::new(None)) };
    let (slot, repaint, id, script) = (job.slot.clone(), ctx.clone(), id.to_string(), script.to_string());
    std::thread::Builder::new()
        .name("gridcraft-module-run".into())
        .spawn(move || {
            let result = PythonHost::global().run(&at.modules, &at.trust, &python, &id, &script, &data, DEFAULT_TIMEOUT);
            *slot.lock().unwrap_or_else(PoisonError::into_inner) = Some(result);
            repaint.request_repaint();
        })
        .map_err(|e| format!("The script could not be started: {e}"))?;
    ctx.data_mut(|d| d.insert_temp(job_id(), job));
    Ok(())
}

/// The first `SHOWN` characters, saying so when there were more.
fn shortened(text: &str) -> String {
    if text.chars().count() <= SHOWN {
        return text.to_string();
    }
    format!("{}\n… ({} more characters)", text.chars().take(SHOWN).collect::<String>(), text.chars().count() - SHOWN)
}

/// What to show for a finished run.
fn finished(result: Result<Output, RunError>) -> Json {
    match result {
        Ok(out) => json!({
            "state": "done",
            "ok": true,
            "text": shortened(&serde_json::to_string_pretty(&out.result).unwrap_or_default()),
            "printed": shortened(out.printed.trim_end()),
        }),
        Err(RunError::Script { error, trace }) => json!({"state": "done", "ok": false, "text": error, "printed": shortened(trace.trim_end())}),
        Err(other) => json!({"state": "done", "ok": false, "text": other.to_string(), "printed": ""}),
    }
}

/// What the user asked for in one frame; carried out after the dialog is drawn.
enum Action {
    Refresh,
    Review(String),
    NotNow,
    Trust { id: String, fingerprint: String, follow: bool },
    StopTrusting(String),
    Create(String),
    CheckPython(String),
    OpenFolder,
    Run { id: String, script: String },
    StopRun(String),
}

fn status_label(ui: &egui::Ui, m: &Json) -> egui::RichText {
    let t = Tokens::get(ui.ctx());
    match m["status"].as_str() {
        Some("trusted") => egui::RichText::new("Trusted").color(t.accent).strong(),
        Some("changed") => egui::RichText::new("Changed since you trusted it").color(ui.visuals().warn_fg_color).strong(),
        Some("problem") => egui::RichText::new("Can't be used").color(ui.visuals().error_fg_color).strong(),
        _ => egui::RichText::new("Not trusted").color(t.text_dim),
    }
}

/// What a module asks for, one line each; the one that can't be limited stands out.
fn asks(ui: &mut egui::Ui, m: &Json) {
    let asked = m["asks"].as_array().cloned().unwrap_or_default();
    if asked.is_empty() {
        ui.label("It asks for nothing: it can show its panel and no more.");
    }
    for a in asked {
        let line = format!("•  {}", a["means"].as_str().unwrap_or(""));
        if a["unlimited"] == true {
            ui.label(egui::RichText::new(line).color(ui.visuals().warn_fg_color).strong());
        } else {
            ui.label(line);
        }
    }
}

/// The scripts a trusted module may run: the `.py` files of its scripts folder, by the names a
/// panel would use.
fn scripts(m: &Json) -> Vec<String> {
    let may_run = m["status"] == "trusted" && m["granted"].as_array().is_some_and(|g| g.iter().any(|p| p == "python"));
    let Some(folder) = m["python"].as_str().filter(|_| may_run) else { return vec![] };
    let prefix = format!("{folder}/");
    let files = m["files"].as_array().map(|all| all.iter().filter_map(Json::as_str).collect::<Vec<_>>()).unwrap_or_default();
    files.into_iter().filter_map(|f| f.strip_prefix(&prefix)).filter(|f| f.ends_with(".py")).take(12).map(str::to_string).collect()
}

fn card(ui: &mut egui::Ui, m: &Json, reviewing: bool, busy: bool, follow: &mut bool, action: &mut Option<Action>) {
    let id = m["id"].as_str().unwrap_or("").to_string();
    let status = m["status"].as_str().unwrap_or("");
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(m["name"].as_str().unwrap_or(&id)).strong());
            ui.label(egui::RichText::new(m["version"].as_str().unwrap_or("")).small());
            if let Some(author) = m["author"].as_str().filter(|a| !a.is_empty()) {
                ui.label(egui::RichText::new(format!("by {author}")).small());
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.label(status_label(ui, m)));
        });
        if let Some(description) = m["description"].as_str().filter(|text| !text.is_empty()) {
            ui.label(description);
        }
        if let Some(problem) = m["problem"].as_str() {
            ui.label(format!("This module can't be used: {problem}."));
            return;
        }
        if !reviewing {
            ui.horizontal(|ui| match status {
                "trusted" => {
                    if ui.button("Stop trusting").clicked() {
                        *action = Some(Action::StopTrusting(id.clone()));
                    }
                    if m["followsChanges"] == true {
                        ui.label(egui::RichText::new("You chose to keep trusting it as its files change.").small());
                    }
                }
                _ => {
                    if ui.button("Review…").clicked() {
                        *action = Some(Action::Review(id.clone()));
                    }
                    if status == "changed" {
                        ui.label(egui::RichText::new("Its files are not the ones you trusted. It does nothing until you look again.").small());
                    }
                }
            });
            let runnable = scripts(m);
            if !runnable.is_empty() {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Run on the selection:");
                    for script in runnable {
                        if ui.add_enabled(!busy, egui::Button::new(&script)).clicked() {
                            *action = Some(Action::Run { id: id.clone(), script });
                        }
                    }
                });
            }
            return;
        }
        ui.separator();
        ui.label(egui::RichText::new("Before you trust it, this is what it asks to be allowed:").strong());
        asks(ui, m);
        let files: Vec<&str> = m["files"].as_array().map(|all| all.iter().filter_map(Json::as_str).collect()).unwrap_or_default();
        let shown: Vec<&str> = files.iter().copied().take(8).collect();
        let more = files.len().saturating_sub(shown.len());
        ui.label(format!(
            "It contains {} file{}: {}{}",
            files.len(),
            if files.len() == 1 { "" } else { "s" },
            shown.join(", "),
            if more > 0 { format!(" and {more} more") } else { String::new() }
        ));
        let fingerprint = m["fingerprint"].as_str().unwrap_or("").to_string();
        ui.label(egui::RichText::new(format!("Fingerprint {}…", fingerprint.chars().take(16).collect::<String>())).small())
            .on_hover_text("Trust is given to exactly these files. If any of them changes, the module is untrusted again until you decide again.");
        ui.checkbox(follow, FOLLOW);
        ui.horizontal(|ui| {
            if ui.button("Trust this module").clicked() {
                *action = Some(Action::Trust { id: id.clone(), fingerprint, follow: *follow });
            }
            if ui.button("Not now").clicked() {
                *action = Some(Action::NotNow);
            }
        });
        ui.label(egui::RichText::new("Your operating system will ask you to confirm.").small());
    });
}

/// The script that is running, or what the last one gave.
fn run_area(ui: &mut egui::Ui, run: &Json, action: &mut Option<Action>) {
    let (module, script) = (run["module"].as_str().unwrap_or(""), run["script"].as_str().unwrap_or(""));
    ui.separator();
    if run["state"] == "running" {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label(format!("Running {script}…"));
            if ui.button("Stop").clicked() {
                *action = Some(Action::StopRun(module.to_string()));
            }
        });
        return;
    }
    let ok = run["ok"] == true;
    ui.label(egui::RichText::new(if ok { format!("{script} returned:") } else { format!("{script} did not finish:") }).strong());
    egui::ScrollArea::vertical().id_salt("module_run").max_height(150.0).show(ui, |ui| {
        let text = egui::RichText::new(run["text"].as_str().unwrap_or("")).monospace();
        ui.add(egui::Label::new(if ok { text } else { text.color(ui.visuals().error_fg_color) }).selectable(true));
        if let Some(printed) = run["printed"].as_str().filter(|p| !p.is_empty()) {
            ui.label(egui::RichText::new(if ok { "It printed:" } else { "Details:" }).small());
            ui.add(egui::Label::new(egui::RichText::new(printed).monospace().small()).selectable(true));
        }
    });
}

/// Hook of the fork's dialogs: draws Developer ▸ Modules.
pub fn ui(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut Dialog, open: &mut bool) {
    ui.set_min_width(520.0);
    let listing = d.values.get("list").cloned().unwrap_or(Json::Null);
    let mut action = None;

    // A script that was running may have finished since the last frame.
    let mut run = d.values.get("run").cloned().unwrap_or(Json::Null);
    if run["state"] == "running" {
        let job = ui.ctx().data(|data| data.get_temp::<Job>(job_id()));
        let result = job.and_then(|job| job.slot.lock().unwrap_or_else(PoisonError::into_inner).take());
        match result {
            Some(result) => {
                let mut done = finished(result);
                done["module"] = run["module"].clone();
                done["script"] = run["script"].clone();
                run = done;
                d.values.insert("run".into(), run.clone());
            }
            None => ui.ctx().request_repaint_after(std::time::Duration::from_millis(100)),
        }
    }
    let busy = run["state"] == "running";

    ui.label("Modules add panels and code to GridCraft. A module does nothing until you trust it.");
    if listing["available"] == true {
        ui.horizontal(|ui| {
            ui.label("Folder:");
            // A long path is cut short (the whole of it shows on hover), so the button stays in reach.
            let folder = listing["folder"].as_str().unwrap_or("");
            let room = (ui.available_width() - 110.0).clamp(120.0, 380.0);
            ui.allocate_ui(egui::vec2(room, ui.spacing().interact_size.y), |ui| {
                ui.add(egui::Label::new(egui::RichText::new(folder).monospace().small()).truncate()).on_hover_text(folder);
            });
            if app.services.open_url.is_some() && ui.button("Open Folder").clicked() {
                action = Some(Action::OpenFolder);
            }
        });
        ui.separator();
        let modules = listing["modules"].as_array().cloned().unwrap_or_default();
        let reviewing = d.values.get("review").and_then(Json::as_str).unwrap_or("").to_string();
        let mut follow = d.values.get("follow").and_then(Json::as_bool).unwrap_or(false);
        egui::ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
            if modules.is_empty() {
                ui.label("No modules are installed. Create one below, or copy a module's folder into the folder above.");
            }
            for m in &modules {
                card(ui, m, m["id"].as_str() == Some(reviewing.as_str()), busy, &mut follow, &mut action);
            }
        });
        d.values.insert("follow".into(), json!(follow));
        if !run.is_null() {
            run_area(ui, &run, &mut action);
        }
        ui.separator();
        egui::Grid::new("modules_settings").num_columns(3).spacing(egui::vec2(8.0, 6.0)).show(ui, |ui| {
            let label = ui.label("New module:");
            let mut name = d.values.get("newName").and_then(Json::as_str).unwrap_or("").to_string();
            // A grid cell would squeeze a text field to nothing: it is given its size.
            let size = egui::vec2(240.0, ui.spacing().interact_size.y);
            if ui.add_sized(size, egui::TextEdit::singleline(&mut name).hint_text("Its name")).labelled_by(label.id).changed() {
                d.values.insert("newName".into(), json!(name));
            }
            if ui.button("Create").clicked() {
                action = Some(Action::Create(name));
            }
            ui.end_row();

            let label = ui.label("Python:");
            let mut python = d.values.get("python").and_then(Json::as_str).unwrap_or("").to_string();
            let field = egui::TextEdit::singleline(&mut python).hint_text("Found automatically");
            if ui
                .add_sized(size, field)
                .labelled_by(label.id)
                .on_hover_text("The Python that runs modules' scripts: a path or a command. Leave empty to let GridCraft find one.")
                .changed()
            {
                d.values.insert("python".into(), json!(python));
            }
            if ui.button("Check Python").clicked() {
                action = Some(Action::CheckPython(python));
            }
            ui.end_row();
        });
        if let Some(status) = d.values.get("pythonStatus").and_then(Json::as_str).filter(|s| !s.is_empty()) {
            ui.label(egui::RichText::new(status).small());
        }
    } else {
        ui.label("Modules are not available in this run, because it keeps no settings.");
    }
    ui.add_space(8.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if ui.button("Close").clicked() {
            *open = false;
        }
        if listing["available"] == true && ui.button("Refresh").clicked() {
            action = Some(Action::Refresh);
        }
    });

    let outcome = match action {
        None => return,
        Some(Action::Refresh) => Ok(String::new()),
        Some(Action::Review(id)) => {
            d.values.insert("review".into(), json!(id));
            d.values.insert("follow".into(), json!(false));
            Ok(String::new())
        }
        Some(Action::NotNow) => {
            d.values.insert("review".into(), Json::Null);
            Ok(String::new())
        }
        Some(Action::Trust { id, fingerprint, follow }) => {
            d.values.insert("review".into(), Json::Null);
            trust(app, &id, &fingerprint, follow)
        }
        Some(Action::StopTrusting(id)) => stop_trusting(&id),
        Some(Action::Create(name)) => create(&name).inspect(|_| {
            d.values.insert("newName".into(), json!(""));
        }),
        Some(Action::CheckPython(python)) => {
            let status = check_python(&python).unwrap_or_else(|why| why);
            d.values.insert("pythonStatus".into(), json!(status));
            Ok(String::new())
        }
        Some(Action::OpenFolder) => match (places(), &app.services.open_url) {
            (Some(at), Some(open_url)) => {
                std::fs::create_dir_all(&at.modules).map_err(|e| format!("The modules folder could not be created: {e}")).map(|()| {
                    open_url(&at.modules.display().to_string());
                    String::new()
                })
            }
            _ => Err(UNAVAILABLE.into()),
        },
        Some(Action::Run { id, script }) => start(app, ui.ctx(), &id, &script).map(|()| {
            d.values.insert("run".into(), json!({"state": "running", "module": id, "script": script}));
            String::new()
        }),
        Some(Action::StopRun(id)) => {
            if let Some(at) = places() {
                PythonHost::global().stop(&at.modules, &id);
            }
            Ok(String::new())
        }
    };
    // Whatever happened, show what is on disk now.
    d.values.insert("list".into(), list());
    match outcome {
        Ok(said) if said.is_empty() => {}
        Ok(said) => app.toast = Some((said, crate::now_ms())),
        Err(why) => app.message = Some(("Modules".into(), why)),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::Services;
    use egui_kittest::kittest::Queryable;
    use std::cell::RefCell;
    use std::rc::Rc;

    thread_local! {
        static SANDBOX: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
    }

    /// The settings folder the current test set up, if it did.
    pub(crate) fn sandbox() -> Option<PathBuf> {
        SANDBOX.with(|s| s.borrow().clone())
    }

    /// A settings folder of its own for one test (on this thread), removed when the test ends.
    pub(crate) struct Sandbox(pub PathBuf);

    impl Sandbox {
        pub(crate) fn new(tag: &str) -> Sandbox {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("gridcraft-ui-modules-{}-{tag}-{n}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            SANDBOX.with(|s| *s.borrow_mut() = Some(dir.clone()));
            Sandbox(dir)
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            if let Some(at) = places() {
                PythonHost::global().stop(&at.modules, "clean-data");
            }
            SANDBOX.with(|s| *s.borrow_mut() = None);
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// What the operating system's question was asked, and what the "user" answers.
    #[derive(Clone, Default)]
    struct Asked {
        questions: Rc<RefCell<Vec<String>>>,
        answer: Rc<RefCell<bool>>,
    }

    /// An app whose operating-system confirmation is `asked` (or missing, as in a build without one).
    fn app_with(asked: Option<&Asked>) -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        let confirm = asked.cloned().map(|asked| {
            Box::new(move |_title: &str, text: &str| {
                asked.questions.borrow_mut().push(text.to_string());
                *asked.answer.borrow()
            }) as Box<dyn Fn(&str, &str) -> bool>
        });
        SheetApp::new(session, Services { confirm, ..Default::default() })
    }

    /// A user who answers yes.
    fn yes() -> Asked {
        let asked = Asked::default();
        *asked.answer.borrow_mut() = true;
        asked
    }

    fn status(list: &Json, id: &str) -> Option<String> {
        list["modules"].as_array()?.iter().find(|m| m["id"] == id)?["status"].as_str().map(str::to_string)
    }

    fn fingerprint() -> String {
        list()["modules"][0]["fingerprint"].as_str().unwrap().to_string()
    }

    #[test]
    fn a_run_without_settings_has_no_modules() {
        let a = app_with(Some(&yes()));
        assert_eq!(list(), json!({"available": false, "folder": null, "modules": []}));
        assert!(create("Clean Data").is_err() && trust(&a, "clean-data", "", false).is_err() && stop_trusting("clean-data").is_err());
        assert!(check_python("").is_err());
    }

    #[test]
    fn agents_can_see_modules_but_no_command_gives_trust() {
        let s = Sandbox::new("agents");
        create("Clean Data").unwrap();
        let mut a = app_with(Some(&yes()));
        let seen = a.run("developer.modules", json!({})).unwrap();
        assert_eq!(seen["available"], json!(true));
        assert_eq!(status(&seen, "clean-data").as_deref(), Some("untrusted"));
        assert_eq!(seen["modules"][0]["granted"], json!([]));
        // Nothing among the fork's commands trusts or runs, and unknown ids fall through to upstream.
        assert!(super::super::commands().all(|c| {
            let id = c["id"].as_str().unwrap_or("").to_lowercase();
            !id.contains("trust") && !id.contains("run") && !id.contains("python")
        }));
        for id in ["developer.trust", "developer.trustModule", "developer.modules.trust", "developer.runPython", "developer.run"] {
            assert!(super::super::run_command(&mut a, id, &json!({"id": "clean-data"})).is_none(), "{id}");
        }
        // The dialog has no OK command: confirming it over the control channel does nothing.
        let ctx = egui::Context::default();
        let reply = |o: crate::control::Outcome| match o {
            crate::control::Outcome::Done(v) => v,
            _ => Json::Null,
        };
        reply(crate::control::handle(&mut a, &ctx, "ui.dialog", &json!({"name": "modules"})));
        reply(crate::control::handle(&mut a, &ctx, "ui.dialog.set", &json!({"field": "review", "value": "clean-data"})));
        let confirmed = reply(crate::control::handle(&mut a, &ctx, "ui.dialog.confirm", &json!({})));
        assert_eq!(confirmed["ok"], json!(false));
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        assert!(!s.0.join("trusted-modules.json").exists());
    }

    #[test]
    fn only_a_real_module_id_and_the_reviewed_fingerprint_are_trusted() {
        let s = Sandbox::new("ids");
        create("Clean Data").unwrap();
        let asked = yes();
        let a = app_with(Some(&asked));
        let fingerprint = fingerprint();
        for id in ["", "..", "../clean-data", "clean-data/..", "Clean-Data", "C:", "clean data"] {
            assert!(trust(&a, id, &fingerprint, false).is_err(), "{id:?}");
        }
        assert!(trust(&a, "clean-data", "", false).is_err(), "no fingerprint");
        assert!(trust(&a, "clean-data", &"0".repeat(64), false).is_err(), "another fingerprint");
        assert!(trust(&a, "missing", &fingerprint, false).is_err(), "no such module");
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        assert!(!s.0.join("trusted-modules.json").exists(), "none of that wrote a decision");
        assert!(asked.questions.borrow().is_empty(), "nor got as far as asking the user");
        assert!(trust(&a, "clean-data", &fingerprint, false).is_ok());
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("trusted"));
    }

    #[test]
    fn trust_needs_the_operating_systems_own_confirmation() {
        let s = Sandbox::new("confirm");
        create("Clean Data").unwrap();
        let fingerprint = fingerprint();
        // A build that can't ask (no such service) can't trust.
        let refused = trust(&app_with(None), "clean-data", &fingerprint, false);
        assert!(refused.is_err_and(|why| why.contains("can't ask your operating system")));
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        // The user is asked once, told everything they are allowing, and says no.
        let asked = Asked::default();
        let a = app_with(Some(&asked));
        assert_eq!(trust(&a, "clean-data", &fingerprint, false).as_deref(), Ok("Clean Data was not trusted."));
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        assert!(!s.0.join("trusted-modules.json").exists());
        let questions = asked.questions.borrow().clone();
        assert_eq!(questions.len(), 1);
        let q = &questions[0];
        assert!(q.contains("Trust \"Clean Data\" 0.1.0?"), "{q}");
        for means in ["See what is in your open workbooks", "Change your open workbooks", "Run its Python scripts. This cannot be limited"] {
            assert!(q.contains(means), "{q}");
        }
        assert!(q.contains("It has 3 files") && q.contains("will ask you again"), "{q}");
        // Then says yes, keeping trust as files change: the question says that too.
        *asked.answer.borrow_mut() = true;
        assert_eq!(trust(&a, "clean-data", &fingerprint, true).as_deref(), Ok("Clean Data is trusted."));
        assert!(asked.questions.borrow()[1].contains("keep trusting it as its files change"));
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("trusted"));
    }

    fn harness(app: SheetApp) -> egui_kittest::Harness<'static, SheetApp> {
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        h.state_mut().open_dialog(NAME, json!({}));
        // A new window takes a few frames to find its size and place; clicks go by position.
        h.run_steps(6);
        h
    }

    /// Clicks go by position, so the window is given a moment to settle first: it changes size
    /// when a review opens or a result appears.
    fn click(h: &mut egui_kittest::Harness<'static, SheetApp>, label: &str) {
        h.run_steps(3);
        h.get_by_label(label).click();
        h.run_steps(3);
    }

    fn shown(h: &egui_kittest::Harness<'static, SheetApp>) -> Option<String> {
        status(&h.state().dialog.as_ref()?.values["list"], "clean-data")
    }

    /// The whole path in the real window: create, review, trust, change, trust again, take back.
    #[test]
    fn the_user_creates_reviews_trusts_and_stops_trusting_a_module() {
        let s = Sandbox::new("flow");
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.clone()).as_deref(), Some(NAME));
        if let Some(d) = h.state_mut().dialog.as_mut() {
            d.values.insert("newName".into(), json!("Clean Data"));
        }
        click(&mut h, "Create");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"), "a new module is not trusted");
        assert!(s.0.join("modules/clean-data/module.toml").exists());
        assert!(h.query_by_label("clean.py").is_none(), "an untrusted module's scripts can't be run");

        click(&mut h, "Review…");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"), "looking is not trusting");
        assert!(asked.questions.borrow().is_empty());
        click(&mut h, "Trust this module");
        assert_eq!(shown(&h).as_deref(), Some("trusted"));
        assert_eq!(asked.questions.borrow().len(), 1, "the operating system asked, once");
        assert!(s.0.join("trusted-modules.json").exists(), "the decision is saved");

        // One of its files changes: it is untrusted again until the user decides again.
        std::fs::write(s.0.join("modules/clean-data/python/clean.py"), "import os\n").unwrap();
        click(&mut h, "Refresh");
        assert_eq!(shown(&h).as_deref(), Some("changed"));
        click(&mut h, "Review…");
        click(&mut h, "Not now");
        assert_eq!(shown(&h).as_deref(), Some("changed"), "declining leaves it untrusted");
        click(&mut h, "Review…");
        click(&mut h, FOLLOW);
        click(&mut h, "Trust this module");
        assert_eq!(shown(&h).as_deref(), Some("trusted"));
        std::fs::write(s.0.join("modules/clean-data/python/clean.py"), "import sys\n").unwrap();
        click(&mut h, "Refresh");
        assert_eq!(shown(&h).as_deref(), Some("trusted"), "the user chose to keep trusting their own module as it changes");

        click(&mut h, "Stop trusting");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"));
        click(&mut h, "Close");
        assert!(h.state().dialog.is_none());
    }

    #[test]
    fn saying_no_to_the_operating_system_leaves_the_module_untrusted() {
        let s = Sandbox::new("no");
        create("Clean Data").unwrap();
        let asked = Asked::default();
        let mut h = harness(app_with(Some(&asked)));
        click(&mut h, "Review…");
        click(&mut h, "Trust this module");
        assert_eq!(asked.questions.borrow().len(), 1);
        assert_eq!(shown(&h).as_deref(), Some("untrusted"));
        assert!(!s.0.join("trusted-modules.json").exists());
        assert!(h.state().message.is_none(), "declining is not an error");
    }

    #[test]
    fn a_module_that_changes_while_being_reviewed_is_not_trusted() {
        let s = Sandbox::new("race");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        click(&mut h, "Review…");
        // The files change after the user was shown them and before they press the button.
        std::fs::write(s.0.join("modules/clean-data/python/clean.py"), "import os\n").unwrap();
        click(&mut h, "Trust this module");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"));
        assert!(h.state().message.as_ref().is_some_and(|(_, text)| text.contains("changed while you were looking")), "{:?}", h.state().message);
        assert!(asked.questions.borrow().is_empty(), "the user is not even asked about files they didn't see");
        assert!(!s.0.join("trusted-modules.json").exists());
    }

    #[test]
    fn a_broken_module_is_listed_with_its_problem_and_cannot_be_reviewed() {
        let s = Sandbox::new("broken");
        std::fs::create_dir_all(s.0.join("modules/half")).unwrap();
        std::fs::write(s.0.join("modules/half/readme.txt"), "no manifest").unwrap();
        let mut h = harness(app_with(Some(&yes())));
        let list = h.state().dialog.as_ref().unwrap().values["list"].clone();
        assert_eq!(status(&list, "half").as_deref(), Some("problem"));
        assert!(h.query_by_label("Review…").is_none(), "there is nothing to trust");
        h.run_steps(1);
    }

    /// Steps the window until the running script has given its result (or a long time passes).
    fn wait_for_run(h: &mut egui_kittest::Harness<'static, SheetApp>) -> Json {
        for _ in 0..3_000 {
            h.step();
            let run = h.state().dialog.as_ref().map(|d| d.values["run"].clone()).unwrap_or(Json::Null);
            if run["state"] == "done" {
                return run;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        panic!("the script did not finish");
    }

    /// The whole path in the real window: a trusted module's script runs on the selection, in a
    /// Python process, and its result comes back to the dialog.
    #[test]
    fn a_trusted_modules_script_runs_on_the_selection() {
        let _s = Sandbox::new("run");
        if let Err(why) = PythonHost::global().interpreter("") {
            eprintln!("skipped: {why}");
            return;
        }
        create("Clean Data").unwrap();
        let mut a = app_with(Some(&yes()));
        a.session.run("range.setValues", json!({"range": "A1", "values": [["  north ", 12], ["south  ", 7.5]]})).unwrap();
        a.session.run("selection.set", json!({"range": "A1:B2"})).unwrap();
        trust(&a, "clean-data", &fingerprint(), false).unwrap();
        let mut h = harness(a);
        click(&mut h, "clean.py");
        let run = wait_for_run(&mut h);
        assert_eq!(
            (run["ok"].as_bool(), run["script"].as_str(), run["module"].as_str()),
            (Some(true), Some("clean.py"), Some("clean-data")),
            "{run}"
        );
        let result: Json = serde_json::from_str(run["text"].as_str().unwrap()).unwrap();
        assert_eq!(result, json!({"rows": [["north", 12.0], ["south", 7.5]]}), "the starter script trims the text in the selection");
        // The sheet is not changed by running a script here: writing back is the panel's job.
        let sheet = h.state_mut().session.run("sheet.read", json!({"range": "A1:B2"})).unwrap();
        assert_eq!(sheet["values"][0][0], json!("  north "));

        // A script that fails says why, and one that never ends can be stopped.
        let s = sandbox().unwrap();
        std::fs::write(s.join("modules/clean-data/python/clean.py"), "def main(data):\n    return 1 / 0\n").unwrap();
        std::fs::write(s.join("modules/clean-data/python/forever.py"), "def main(data):\n    while True:\n        pass\n").unwrap();
        click(&mut h, "Refresh");
        assert!(h.query_by_label("clean.py").is_none(), "its files changed: it is untrusted, and nothing of it can be run");
        click(&mut h, "Review…");
        click(&mut h, "Trust this module");
        click(&mut h, "clean.py");
        let failed = wait_for_run(&mut h);
        assert_eq!(failed["ok"], json!(false));
        assert!(failed["text"].as_str().unwrap().contains("ZeroDivisionError"), "{failed}");
        click(&mut h, "forever.py");
        assert_eq!(h.state().dialog.as_ref().unwrap().values["run"]["state"], json!("running"));
        std::thread::sleep(std::time::Duration::from_millis(400));
        click(&mut h, "Stop");
        let stopped = wait_for_run(&mut h);
        assert_eq!(stopped["ok"], json!(false), "{stopped}");
    }

    #[test]
    fn the_python_to_use_is_remembered_and_checked() {
        let s = Sandbox::new("python");
        let mut h = harness(app_with(Some(&yes())));
        if let Some(d) = h.state_mut().dialog.as_mut() {
            d.values.insert("python".into(), json!("gridcraft-no-such-python-here"));
        }
        click(&mut h, "Check Python");
        let said = h.state().dialog.as_ref().unwrap().values["pythonStatus"].as_str().unwrap().to_string();
        assert!(said.contains("did not answer"), "{said}");
        assert_eq!(settings::load(&s.0.join("developer.json")).python, "gridcraft-no-such-python-here");
        // It is what the dialog starts from next time.
        assert_eq!(values()["python"], json!("gridcraft-no-such-python-here"));
    }

    #[test]
    fn open_folder_asks_the_host_to_show_the_modules_folder() {
        let s = Sandbox::new("folder");
        let opened = Rc::new(RefCell::new(Vec::new()));
        let seen = opened.clone();
        let mut a = app_with(Some(&yes()));
        a.services.open_url = Some(Box::new(move |path: &str| seen.borrow_mut().push(path.to_string())));
        let mut h = harness(a);
        click(&mut h, "Open Folder");
        assert_eq!(*opened.borrow(), [s.0.join("modules").display().to_string()]);
        assert!(s.0.join("modules").is_dir(), "the folder is made if it wasn't there");
    }
}
