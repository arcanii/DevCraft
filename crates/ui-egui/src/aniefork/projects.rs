//! Developer ▸ Projects: the projects the user has installed, the one place where trust in a project
//! is given or taken back, where a trusted project's panel is opened, and a bench for running its
//! scripts by hand.
//!
//! A project does nothing until the user trusts it. Trusting is a decision about exact contents:
//! the dialog shows what the project asks to be allowed and what it contains, and what gets
//! trusted is the fingerprint the user was shown. If the files changed while the user was
//! reading, the decision is refused and they look again.
//!
//! Giving trust takes two things no agent can supply. There is no command for it, so the control
//! channel and MCP cannot ask (`developer.projects` only reports). And the last step is a question
//! drawn by the operating system, outside the app's own window contents, which the control
//! channel's synthetic clicks cannot answer. A build that can't ask it can't trust projects.
//!
//! A trusted project that was granted `python` can have its scripts run from here on the current
//! selection. The script runs in a Python process of its own, off the UI thread; see
//! `gridcraft_aniefork_dev::python`. One with a panel can have it opened in the browser; see
//! [`super::panels`].
//!
//! The control channel can click and type in this dialog as a person does, and rewrite a dialog's
//! values. So the dialog works from a state of its own that the channel can't reach, and what
//! runs code or decides who may (trusting, running a script, opening a panel, naming the Python)
//! is refused while the channel is putting input into the window; see [`super::presence`].

use std::sync::{Arc, Mutex, PoisonError};

use gridcraft_aniefork_dev::bridge::Places;
use gridcraft_aniefork_dev::kit::{self, Kit};
use gridcraft_aniefork_dev::python::{DEFAULT_TIMEOUT, Output, PythonHost, RunError};
use gridcraft_aniefork_dev::{Project, TrustStore, describe, manifest, project, settings, starter};
use serde_json::{Value as Json, json};

use super::{panels, presence};
use crate::SheetApp;
use crate::dialogs::Dialog;
use crate::theme::Tokens;

/// The dialog's name, as `ui.dialog {name}` takes it.
pub const NAME: &str = "projects";

const FOLLOW: &str = "Keep trusting it as its files change (only for a project you are writing yourself)";
/// The most of a result shown in the dialog (characters).
const SHOWN: usize = 6_000;

/// Where this run keeps projects, the user's decisions and the subsystem's settings. `None` when
/// the run keeps no settings (the web build, a run without preferences): then there are no
/// projects.
#[cfg(not(test))]
pub(super) fn places() -> Option<Places> {
    use gridcraft_aniefork_dev::paths;
    Some(Places { projects: paths::projects_dir()?, trust: paths::trust_file()?, settings: paths::settings_file()? })
}

/// In tests: only the folder a test set up for itself, never the developer's own settings.
#[cfg(test)]
pub(super) fn places() -> Option<Places> {
    tests::sandbox().map(|dir| Places {
        projects: dir.join("projects"),
        trust: dir.join("trusted-projects.json"),
        settings: dir.join("developer.json"),
    })
}

pub(super) const UNAVAILABLE: &str = "Projects are not available in this run.";

/// The user's projects and where each stands: what `developer.projects` reports and the dialog
/// shows. Reading only.
pub fn list() -> Json {
    let Some(at) = places() else { return json!({"available": false, "folder": null, "projects": []}) };
    let store = TrustStore::load(&at.trust);
    let projects: Vec<Json> = project::scan(&at.projects).iter().map(|m| describe(m, &store)).collect();
    json!({"available": true, "folder": at.projects.display().to_string(), "projects": projects})
}

/// What the dialog's values report when it opens. The dialog doesn't work from them: see `State`.
pub fn values() -> Json {
    let python = places().map(|at| settings::load(&at.settings).python).unwrap_or_default();
    json!({"list": list(), "review": null, "follow": false, "newName": "", "python": python, "pythonStatus": "", "run": null})
}

/// The question the operating system puts to the user: what they are about to allow, in full.
pub(super) fn question(found: &Project, follow_changes: bool) -> String {
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

/// The user pressed "Trust this project" while reviewing `id`. `reviewed` is the fingerprint they
/// were shown: only that is trusted, and only after the operating system's own confirmation.
pub(super) fn trust(app: &SheetApp, id: &str, reviewed: &str, follow_changes: bool) -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    if !manifest::valid_id(id) {
        return Err("That is not a project.".into());
    }
    let found = project::load(&at.projects.join(id));
    if found.fingerprint.as_deref() != Some(reviewed) {
        return Err(format!("{} changed while you were looking at it. Review it again before trusting it.", found.name()));
    }
    if !found.is_sound() {
        return Err(format!("{} can't be trusted: {}.", found.name(), found.problem.as_deref().unwrap_or("it could not be read")));
    }
    let Some(confirm) = &app.services.confirm else {
        return Err("This copy of GridCraft can't ask your operating system to confirm, so it can't trust projects.".into());
    };
    if !confirm("Trust this project?", &question(&found, follow_changes)) {
        return Ok(format!("{} was not trusted.", found.name()));
    }
    // What is recorded is the fingerprint that was reviewed, whatever happened on disk meanwhile.
    TrustStore::load(&at.trust).trust(&found, follow_changes)?;
    Ok(format!("{} is trusted.", found.name()))
}

fn stop_trusting(id: &str) -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    TrustStore::load(&at.trust).revoke(id)?;
    PythonHost::global().stop(&at.projects, id);
    Ok("The project is no longer trusted.".into())
}

pub(super) fn create(name: &str) -> Result<String, String> {
    create_project_in(&kit::ANIE, name, None)
}

/// Creates a starter project of `kit`: the code of the workbook named `workbook`, or (without
/// one) a library, for any workbook.
pub(super) fn create_project_in(kit: &Kit, name: &str, workbook: Option<&str>) -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    std::fs::create_dir_all(&at.projects).map_err(|e| format!("The projects folder could not be created: {e}"))?;
    let created = starter::create_in(&at.projects, kit, name, workbook)?;
    Ok(match created.manifest.as_ref().and_then(|manifest| manifest.workbook.as_deref()) {
        Some(workbook) => format!("{} was created, as the code of {workbook}. It is not trusted yet.", created.name()),
        None => format!("{} was created. It is not trusted yet.", created.name()),
    })
}

/// Creates the Svelte showcase project. Like any project it is not trusted until the user says.
pub(super) fn create_showcase() -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    std::fs::create_dir_all(&at.projects).map_err(|e| format!("The projects folder could not be created: {e}"))?;
    let created = gridcraft_aniefork_dev::showcase::create(&at.projects)?;
    Ok(format!("{} was created. It is not trusted yet: review and trust it in Projects.", created.name()))
}

/// Remembers which Python to use and says whether it answers.
fn check_python(python: &str) -> Result<String, String> {
    let at = places().ok_or(UNAVAILABLE)?;
    // Only the Python is set here: what else the settings hold stays as it is.
    let mut kept = settings::load(&at.settings);
    kept.python = python.trim().to_string();
    settings::save(&at.settings, &kept)?;
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
    egui::Id::new("aniefork.projects.job")
}

/// The current selection, as the data a script is handed: `{range, sheet, rows}`.
pub(super) fn selection_data(app: &mut SheetApp) -> Result<Json, String> {
    let range = app.session.active().map(|d| d.selection.current().a1()).ok_or("There is no workbook open.")?;
    let read = app.session.run("sheet.read", json!({"range": range})).map_err(|e| crate::clean_error(&e))?;
    Ok(json!({"range": read["range"], "sheet": read["sheet"], "rows": read["values"]}))
}

/// Starts `script` of project `id` on the selection. The trust check is the host's own, on the
/// thread that runs it. What the script asks of the spreadsheet while it runs is carried out by
/// this window, as a panel's calls are, with what the project was granted.
fn start(app: &mut SheetApp, ctx: &egui::Context, id: &str, script: &str) -> Result<(), String> {
    let at = places().ok_or(UNAVAILABLE)?;
    let data = selection_data(app)?;
    let python = settings::load(&at.settings).python;
    let job = Job { slot: Arc::new(Mutex::new(None)) };
    let calls = super::panels::script_calls(ctx, &at, id);
    let (slot, repaint, id, script) = (job.slot.clone(), ctx.clone(), id.to_string(), script.to_string());
    std::thread::Builder::new()
        .name("gridcraft-project-run".into())
        .spawn(move || {
            let result = PythonHost::global().run_with(&at.projects, &at.trust, &python, &id, &script, &data, DEFAULT_TIMEOUT, Some(&calls));
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
    OpenPanel(String),
    DockPanel(String),
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

/// What a project asks for, one line each; the one that can't be limited stands out.
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

/// The scripts a trusted project may run: the `.py` files of its scripts folder, by the names a
/// panel would use.
fn scripts(m: &Json) -> Vec<String> {
    let may_run = m["status"] == "trusted" && m["granted"].as_array().is_some_and(|g| g.iter().any(|p| p == "python"));
    let Some(folder) = m["python"].as_str().filter(|_| may_run) else { return vec![] };
    let prefix = format!("{folder}/");
    let files = m["files"].as_array().map(|all| all.iter().filter_map(Json::as_str).collect::<Vec<_>>()).unwrap_or_default();
    files.into_iter().filter_map(|f| f.strip_prefix(&prefix)).filter(|f| f.ends_with(".py")).take(12).map(str::to_string).collect()
}

#[allow(clippy::too_many_arguments)]
fn card(ui: &mut egui::Ui, m: &Json, reviewing: bool, busy: bool, can_open: bool, can_dock: bool, follow: &mut bool, action: &mut Option<Action>) {
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
        // What it is for is its own word: it changes nothing about what it may do.
        if let Some(workbook) = m["workbook"].as_str() {
            ui.label(egui::RichText::new(format!("It says it is the code of the workbook {workbook:?}.")).small());
        }
        if let Some(problem) = m["problem"].as_str() {
            ui.label(format!("This project can't be used: {problem}."));
            return;
        }
        if !reviewing {
            ui.horizontal(|ui| match status {
                "trusted" => {
                    if can_open
                        && m["panel"].is_string()
                        && ui.button("Open Panel").on_hover_text("Opens this project's panel in your browser").clicked()
                    {
                        *action = Some(Action::OpenPanel(id.clone()));
                    }
                    if can_dock
                        && m["panel"].is_string()
                        && ui.button("Dock Panel").on_hover_text("Shows this project's panel inside the window, beside the sheet").clicked()
                    {
                        *action = Some(Action::DockPanel(id.clone()));
                    }
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
            .on_hover_text("Trust is given to exactly these files. If any of them changes, the project is untrusted again until you decide again.");
        ui.checkbox(follow, FOLLOW);
        ui.horizontal(|ui| {
            if ui.button("Trust this project").clicked() {
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
    let (project, script) = (run["project"].as_str().unwrap_or(""), run["script"].as_str().unwrap_or(""));
    ui.separator();
    if run["state"] == "running" {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label(format!("Running {script}…"));
            if ui.button("Stop").clicked() {
                *action = Some(Action::StopRun(project.to_string()));
            }
        });
        return;
    }
    let ok = run["ok"] == true;
    ui.label(egui::RichText::new(if ok { format!("{script} returned:") } else { format!("{script} did not finish:") }).strong());
    egui::ScrollArea::vertical().id_salt("project_run").max_height(150.0).show(ui, |ui| {
        let text = egui::RichText::new(run["text"].as_str().unwrap_or("")).monospace();
        ui.add(egui::Label::new(if ok { text } else { text.color(ui.visuals().error_fg_color) }).selectable(true));
        if let Some(printed) = run["printed"].as_str().filter(|p| !p.is_empty()) {
            ui.label(egui::RichText::new(if ok { "It printed:" } else { "Details:" }).small());
            ui.add(egui::Label::new(egui::RichText::new(printed).monospace().small()).selectable(true));
        }
    });
}

/// What the dialog works from, kept between frames.
///
/// It is the dialog's own, out of the dialog's values: anything on the control channel can
/// rewrite those (`ui.dialog.set`), and what the user is shown, which project they are reviewing
/// and which Python they named must not be anyone else's to set. The values only report.
#[derive(Clone, Default)]
struct State {
    /// The last frame the dialog was drawn in: after a gap it was closed, and starts afresh.
    seen: u64,
    list: Json,
    /// The project being reviewed, if one is.
    review: String,
    follow: bool,
    new_name: String,
    python: String,
    python_status: String,
    run: Json,
    /// The dialog is to close: what was asked for is shown behind it.
    close: bool,
}

fn state_id() -> egui::Id {
    egui::Id::new("aniefork.projects.state")
}

/// The value that marks a dialog as already drawn: one without it has just been opened.
const OPENED: &str = "opened";

impl State {
    fn fresh() -> State {
        let python = places().map(|at| settings::load(&at.settings).python).unwrap_or_default();
        State { list: list(), python, ..State::default() }
    }

    /// Writes what the dialog's values report. They are never read back.
    fn report(&self, d: &mut Dialog) {
        let review = if self.review.is_empty() { Json::Null } else { json!(self.review) };
        for (key, value) in [
            ("list", self.list.clone()),
            ("review", review),
            ("follow", json!(self.follow)),
            ("newName", json!(self.new_name)),
            ("python", json!(self.python)),
            ("pythonStatus", json!(self.python_status)),
            ("run", self.run.clone()),
            (OPENED, json!(true)),
        ] {
            d.values.insert(key.into(), value);
        }
    }
}

/// Hook of the fork's dialogs: draws Developer ▸ Projects.
pub fn ui(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut Dialog, open: &mut bool) {
    ui.set_min_width(520.0);
    let ctx = ui.ctx().clone();
    let frame = ctx.cumulative_frame_nr();
    let mut state: State = ctx.data(|data| data.get_temp(state_id())).unwrap_or_default();
    if frame > state.seen.saturating_add(1) || !d.values.contains_key(OPENED) {
        state = State::fresh();
    }
    state.seen = frame;
    // Whether what is clicked and typed now may be the control channel's doing.
    let driven = presence::driven(&ctx);
    let mut action = None;

    // A script that was running may have finished since the last frame.
    if state.run["state"] == "running" {
        let job = ctx.data(|data| data.get_temp::<Job>(job_id()));
        let result = job.and_then(|job| job.slot.lock().unwrap_or_else(PoisonError::into_inner).take());
        match result {
            Some(result) => {
                let mut done = finished(result);
                done["project"] = state.run["project"].clone();
                done["script"] = state.run["script"].clone();
                state.run = done;
            }
            None => ctx.request_repaint_after(std::time::Duration::from_millis(100)),
        }
    }
    let busy = state.run["state"] == "running";
    let listing = state.list.clone();

    ui.label("Projects add panels and code to GridCraft. A project does nothing until you trust it.");
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
        let projects = listing["projects"].as_array().cloned().unwrap_or_default();
        let can_open = app.services.open_url.is_some();
        let can_dock = super::dock::available(&ctx);
        let mut follow = state.follow;
        egui::ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
            if projects.is_empty() {
                ui.label("No projects are installed. Create one below, or copy a project's folder into the folder above.");
            }
            for m in &projects {
                let reviewing = !state.review.is_empty() && m["id"].as_str() == Some(state.review.as_str());
                card(ui, m, reviewing, busy, can_open, can_dock, &mut follow, &mut action);
            }
        });
        // A choice about trust is the person's: a tick the control channel made doesn't count.
        if !driven {
            state.follow = follow;
        }
        if !state.run.is_null() {
            run_area(ui, &state.run, &mut action);
        }
        ui.separator();
        egui::Grid::new("projects_settings").num_columns(3).spacing(egui::vec2(8.0, 6.0)).show(ui, |ui| {
            let label = ui.label("New project:");
            // A grid cell would squeeze a text field to nothing: it is given its size.
            let size = egui::vec2(240.0, ui.spacing().interact_size.y);
            ui.add_sized(size, egui::TextEdit::singleline(&mut state.new_name).hint_text("Its name")).labelled_by(label.id);
            if ui.button("Create").clicked() {
                action = Some(Action::Create(state.new_name.clone()));
            }
            ui.end_row();

            let label = ui.label("Python:");
            // The program named here gets run: only what the person typed is kept.
            let mut python = state.python.clone();
            let field = egui::TextEdit::singleline(&mut python).hint_text("Found automatically");
            let edited = ui
                .add_sized(size, field)
                .labelled_by(label.id)
                .on_hover_text("The Python that runs projects' scripts: a path or a command. Leave empty to let GridCraft find one.")
                .changed();
            if edited && !driven {
                state.python = python;
            }
            if ui.button("Check Python").clicked() {
                action = Some(Action::CheckPython(state.python.clone()));
            }
            ui.end_row();
        });
        if !state.python_status.is_empty() {
            ui.label(egui::RichText::new(&state.python_status).small());
        }
    } else {
        ui.label("Projects are not available in this run, because it keeps no settings.");
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

    let outcome = action.map(|action| act(app, &ctx, &mut state, action, driven));
    if std::mem::take(&mut state.close) {
        *open = false;
    }
    state.report(d);
    ctx.data_mut(|data| data.insert_temp(state_id(), state));
    match outcome {
        None => {}
        Some(Ok(said)) if said.is_empty() => {}
        Some(Ok(said)) => app.toast = Some((said, crate::now_ms())),
        Some(Err(why)) => app.message = Some(("Projects".into(), why)),
    }
}

/// Carries out what the user asked for. Trusting a project, running its code, opening its panel
/// and naming the Python to run are for the person at the screen: asked for while the control
/// channel is putting input into the window, they are refused.
fn act(app: &mut SheetApp, ctx: &egui::Context, state: &mut State, action: Action, driven: bool) -> Result<String, String> {
    let by_hand = matches!(action, Action::Trust { .. } | Action::CheckPython(_) | Action::Run { .. } | Action::OpenPanel(_) | Action::DockPanel(_));
    let outcome = match action {
        _ if by_hand && driven => Err(presence::BY_HAND.to_string()),
        Action::Refresh => Ok(String::new()),
        Action::Review(id) => {
            state.review = id;
            state.follow = false;
            Ok(String::new())
        }
        Action::NotNow => {
            state.review.clear();
            Ok(String::new())
        }
        Action::Trust { id, fingerprint, follow } => {
            state.review.clear();
            trust(app, &id, &fingerprint, follow)
        }
        Action::StopTrusting(id) => {
            panels::stop(ctx, &id);
            stop_trusting(&id)
        }
        Action::Create(name) => create(&name).inspect(|_| state.new_name.clear()),
        Action::CheckPython(python) => {
            state.python_status = check_python(&python).unwrap_or_else(|why| why);
            Ok(String::new())
        }
        Action::OpenFolder => match (places(), &app.services.open_url) {
            (Some(at), Some(open_url)) => {
                std::fs::create_dir_all(&at.projects).map_err(|e| format!("The projects folder could not be created: {e}")).map(|()| {
                    open_url(&at.projects.display().to_string());
                    String::new()
                })
            }
            _ => Err(UNAVAILABLE.into()),
        },
        Action::OpenPanel(id) => panels::open(app, ctx, &id),
        // The dialog makes way: the panel is shown where it would be standing.
        Action::DockPanel(id) => super::dock::dock(app, ctx, &id).inspect(|_| state.close = true),
        Action::Run { id, script } => start(app, ctx, &id, &script).map(|()| {
            state.run = json!({"state": "running", "project": id, "script": script});
            String::new()
        }),
        Action::StopRun(id) => {
            if let Some(at) = places() {
                PythonHost::global().stop(&at.projects, &id);
            }
            Ok(String::new())
        }
    };
    // Whatever happened, show what is on disk now, here and in the Development window.
    state.list = list();
    super::kits::forget();
    outcome
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::Services;
    use egui_kittest::kittest::Queryable;
    use std::cell::RefCell;
    use std::path::PathBuf;
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
            let dir = std::env::temp_dir().join(format!("gridcraft-ui-projects-{}-{tag}-{n}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            SANDBOX.with(|s| *s.borrow_mut() = Some(dir.clone()));
            // An app that starts in this sandbox begins without another test's server.
            crate::aniefork::agents::forget();
            Sandbox(dir)
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            if let Some(at) = places() {
                PythonHost::global().stop(&at.projects, "clean-data");
            }
            crate::aniefork::agents::forget();
            SANDBOX.with(|s| *s.borrow_mut() = None);
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    /// What the operating system's question was asked, and what the "user" answers.
    #[derive(Clone, Default)]
    pub(crate) struct Asked {
        questions: Rc<RefCell<Vec<String>>>,
        answer: Rc<RefCell<bool>>,
    }

    impl Asked {
        /// Every question the operating system was asked to put, in order.
        pub(crate) fn questions(&self) -> Vec<String> {
            self.questions.borrow().clone()
        }

        /// What the "user" answers from now on.
        pub(crate) fn says(&self, yes: bool) {
            *self.answer.borrow_mut() = yes;
        }
    }

    /// An app whose operating-system confirmation is `asked` (or missing, as in a build without one).
    pub(crate) fn app_with(asked: Option<&Asked>) -> SheetApp {
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
    pub(crate) fn yes() -> Asked {
        let asked = Asked::default();
        *asked.answer.borrow_mut() = true;
        asked
    }

    fn status(list: &Json, id: &str) -> Option<String> {
        list["projects"].as_array()?.iter().find(|m| m["id"] == id)?["status"].as_str().map(str::to_string)
    }

    fn fingerprint() -> String {
        list()["projects"][0]["fingerprint"].as_str().unwrap().to_string()
    }

    #[test]
    fn a_run_without_settings_has_no_projects() {
        let a = app_with(Some(&yes()));
        assert_eq!(list(), json!({"available": false, "folder": null, "projects": []}));
        assert!(create("Clean Data").is_err() && trust(&a, "clean-data", "", false).is_err() && stop_trusting("clean-data").is_err());
        assert!(check_python("").is_err());
    }

    #[test]
    fn agents_can_see_projects_but_no_command_gives_trust() {
        let s = Sandbox::new("agents");
        create("Clean Data").unwrap();
        let mut a = app_with(Some(&yes()));
        let seen = a.run("developer.projects", json!({})).unwrap();
        assert_eq!(seen["available"], json!(true));
        assert_eq!(status(&seen, "clean-data").as_deref(), Some("untrusted"));
        assert_eq!(seen["projects"][0]["granted"], json!([]));
        // Nothing among the fork's commands trusts or runs, and unknown ids fall through to upstream.
        assert!(super::super::commands().all(|c| {
            let id = c["id"].as_str().unwrap_or("").to_lowercase();
            !id.contains("trust") && !id.contains("run") && !id.contains("python")
        }));
        for id in ["developer.trust", "developer.trustProject", "developer.projects.trust", "developer.runPython", "developer.run"] {
            assert!(super::super::run_command(&mut a, id, &json!({"id": "clean-data"})).is_none(), "{id}");
        }
        // The dialog has no OK command: confirming it over the control channel does nothing.
        let ctx = egui::Context::default();
        let reply = |o: crate::control::Outcome| match o {
            crate::control::Outcome::Done(v) => v,
            _ => Json::Null,
        };
        reply(crate::control::handle(&mut a, &ctx, "ui.dialog", &json!({"name": "projects"})));
        reply(crate::control::handle(&mut a, &ctx, "ui.dialog.set", &json!({"field": "review", "value": "clean-data"})));
        let confirmed = reply(crate::control::handle(&mut a, &ctx, "ui.dialog.confirm", &json!({})));
        assert_eq!(confirmed["ok"], json!(false));
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        assert!(!s.0.join("trusted-projects.json").exists());
    }

    #[test]
    fn only_a_real_project_id_and_the_reviewed_fingerprint_are_trusted() {
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
        assert!(trust(&a, "missing", &fingerprint, false).is_err(), "no such project");
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        assert!(!s.0.join("trusted-projects.json").exists(), "none of that wrote a decision");
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
        assert!(!s.0.join("trusted-projects.json").exists());
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

    pub(crate) fn harness(app: SheetApp) -> egui_kittest::Harness<'static, SheetApp> {
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
    pub(crate) fn click(h: &mut egui_kittest::Harness<'static, SheetApp>, label: &str) {
        h.run_steps(3);
        h.get_by_label(label).click();
        h.run_steps(3);
    }

    fn shown(h: &egui_kittest::Harness<'static, SheetApp>) -> Option<String> {
        status(&h.state().dialog.as_ref()?.values["list"], "clean-data")
    }

    #[test]
    fn a_project_that_is_one_workbooks_code_says_so_and_is_otherwise_a_project_like_any() {
        let _s = Sandbox::new("workbook-code");
        assert_eq!(
            create_project_in(&kit::ANIE, "Budget Macros", Some("Budget.xlsm")).as_deref(),
            Ok("Budget Macros was created, as the code of Budget.xlsm. It is not trusted yet.")
        );
        assert_eq!(create_project_in(&kit::ANIE, "Tools", None).as_deref(), Ok("Tools was created. It is not trusted yet."));
        // A name that is a path is not a workbook's name: nothing is created.
        assert!(create_project_in(&kit::ANIE, "Elsewhere", Some("../Budget.xlsm")).unwrap_err().contains("can't be that workbook's code"));
        let listed = list();
        let of = |id: &str| listed["projects"].as_array().unwrap().iter().find(|m| m["id"] == id).cloned().unwrap();
        assert_eq!((&of("budget-macros")["kind"], &of("budget-macros")["workbook"]), (&json!("workbook"), &json!("Budget.xlsm")));
        assert_eq!((&of("tools")["kind"], &of("tools")["workbook"]), (&json!("library"), &Json::Null));
        assert_eq!(listed["projects"].as_array().unwrap().len(), 2);
        // What it is for gives it nothing: it is untrusted, and asks for what any starter asks.
        assert_eq!((&of("budget-macros")["status"], &of("budget-macros")["granted"]), (&json!("untrusted"), &json!([])));
        assert_eq!(of("budget-macros")["asks"], of("tools")["asks"]);
        // Projects says what it says it is for, as its own word.
        let h = harness(app_with(None));
        h.get_by_label("It says it is the code of the workbook \"Budget.xlsm\".");
    }

    /// The whole path in the real window: create, review, trust, change, trust again, take back.
    #[test]
    fn the_user_creates_reviews_trusts_and_stops_trusting_a_project() {
        let s = Sandbox::new("flow");
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.clone()).as_deref(), Some(NAME));
        typed(&mut h, |state| state.new_name = "Clean Data".into());
        click(&mut h, "Create");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"), "a new project is not trusted");
        assert!(s.0.join("projects/clean-data/project.toml").exists());
        assert!(h.query_by_label("clean.py").is_none(), "an untrusted project's scripts can't be run");

        click(&mut h, "Review…");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"), "looking is not trusting");
        assert!(asked.questions.borrow().is_empty());
        click(&mut h, "Trust this project");
        assert_eq!(shown(&h).as_deref(), Some("trusted"));
        assert_eq!(asked.questions.borrow().len(), 1, "the operating system asked, once");
        assert!(s.0.join("trusted-projects.json").exists(), "the decision is saved");

        // One of its files changes: it is untrusted again until the user decides again.
        std::fs::write(s.0.join("projects/clean-data/python/clean.py"), "import os\n").unwrap();
        click(&mut h, "Refresh");
        assert_eq!(shown(&h).as_deref(), Some("changed"));
        click(&mut h, "Review…");
        click(&mut h, "Not now");
        assert_eq!(shown(&h).as_deref(), Some("changed"), "declining leaves it untrusted");
        click(&mut h, "Review…");
        click(&mut h, FOLLOW);
        click(&mut h, "Trust this project");
        assert_eq!(shown(&h).as_deref(), Some("trusted"));
        std::fs::write(s.0.join("projects/clean-data/python/clean.py"), "import sys\n").unwrap();
        click(&mut h, "Refresh");
        assert_eq!(shown(&h).as_deref(), Some("trusted"), "the user chose to keep trusting their own project as it changes");

        click(&mut h, "Stop trusting");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"));
        click(&mut h, "Close");
        assert!(h.state().dialog.is_none());
    }

    #[test]
    fn saying_no_to_the_operating_system_leaves_the_project_untrusted() {
        let s = Sandbox::new("no");
        create("Clean Data").unwrap();
        let asked = Asked::default();
        let mut h = harness(app_with(Some(&asked)));
        click(&mut h, "Review…");
        click(&mut h, "Trust this project");
        assert_eq!(asked.questions.borrow().len(), 1);
        assert_eq!(shown(&h).as_deref(), Some("untrusted"));
        assert!(!s.0.join("trusted-projects.json").exists());
        assert!(h.state().message.is_none(), "declining is not an error");
    }

    #[test]
    fn a_project_that_changes_while_being_reviewed_is_not_trusted() {
        let s = Sandbox::new("race");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        click(&mut h, "Review…");
        // The files change after the user was shown them and before they press the button.
        std::fs::write(s.0.join("projects/clean-data/python/clean.py"), "import os\n").unwrap();
        click(&mut h, "Trust this project");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"));
        assert!(h.state().message.as_ref().is_some_and(|(_, text)| text.contains("changed while you were looking")), "{:?}", h.state().message);
        assert!(asked.questions.borrow().is_empty(), "the user is not even asked about files they didn't see");
        assert!(!s.0.join("trusted-projects.json").exists());
    }

    #[test]
    fn a_broken_project_is_listed_with_its_problem_and_cannot_be_reviewed() {
        let s = Sandbox::new("broken");
        std::fs::create_dir_all(s.0.join("projects/half")).unwrap();
        std::fs::write(s.0.join("projects/half/readme.txt"), "no manifest").unwrap();
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

    /// Developer ▸ Projects ▸ a script's name: a script that calls the spreadsheet changes the
    /// sheet from there too, without a panel being open.
    #[test]
    fn a_script_run_from_the_dialog_can_change_the_sheet_itself() {
        let _s = Sandbox::new("run-writes");
        if let Err(why) = PythonHost::global().interpreter("") {
            eprintln!("skipped: {why}");
            return;
        }
        create("Stamp").unwrap();
        let s = sandbox().unwrap();
        std::fs::write(
            s.join("projects/stamp/python/stamp.py"),
            "import spreadsheet\n\ndef main(data):\n    spreadsheet.write('D4', [['stamped ' + data['range']]])\n    return spreadsheet.read('D4')\n",
        )
        .unwrap();
        let mut a = app_with(Some(&yes()));
        a.session.run("selection.set", json!({"range": "A1:B2"})).unwrap();
        trust(&a, "stamp", &fingerprint(), false).unwrap();
        let mut h = harness(a);
        click(&mut h, "stamp.py");
        let run = wait_for_run(&mut h);
        assert_eq!((run["ok"].as_bool(), run["script"].as_str()), (Some(true), Some("stamp.py")), "{run}");
        assert_eq!(serde_json::from_str::<Json>(run["text"].as_str().unwrap()).unwrap(), json!([["stamped A1:B2"]]));
        let sheet = h.state_mut().session.run("sheet.read", json!({"range": "D4"})).unwrap();
        assert_eq!(sheet["values"], json!([["stamped A1:B2"]]), "the script wrote to the sheet itself");
        assert_eq!(h.state().session.active().unwrap().undo.len(), 1, "as one undo step");
        PythonHost::global().stop(&s.join("projects"), "stamp");
    }

    /// The whole path in the real window: a trusted project's script runs on the selection, in a
    /// Python process, and its result comes back to the dialog.
    #[test]
    fn a_trusted_projects_script_runs_on_the_selection() {
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
            (run["ok"].as_bool(), run["script"].as_str(), run["project"].as_str()),
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
        std::fs::write(s.join("projects/clean-data/python/clean.py"), "def main(data):\n    return 1 / 0\n").unwrap();
        std::fs::write(s.join("projects/clean-data/python/forever.py"), "def main(data):\n    while True:\n        pass\n").unwrap();
        click(&mut h, "Refresh");
        assert!(h.query_by_label("clean.py").is_none(), "its files changed: it is untrusted, and nothing of it can be run");
        click(&mut h, "Review…");
        click(&mut h, "Trust this project");
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
        typed(&mut h, |state| state.python = "gridcraft-no-such-python-here".into());
        click(&mut h, "Check Python");
        let said = h.state().dialog.as_ref().unwrap().values["pythonStatus"].as_str().unwrap().to_string();
        assert!(said.contains("did not answer"), "{said}");
        assert_eq!(settings::load(&s.0.join("developer.json")).python, "gridcraft-no-such-python-here");
        // It is what the dialog starts from next time.
        assert_eq!(values()["python"], json!("gridcraft-no-such-python-here"));
    }

    #[test]
    fn open_folder_asks_the_host_to_show_the_projects_folder() {
        let s = Sandbox::new("folder");
        let opened = Rc::new(RefCell::new(Vec::new()));
        let seen = opened.clone();
        let mut a = app_with(Some(&yes()));
        a.services.open_url = Some(Box::new(move |path: &str| seen.borrow_mut().push(path.to_string())));
        let mut h = harness(a);
        click(&mut h, "Open Folder");
        assert_eq!(*opened.borrow(), [s.0.join("projects").display().to_string()]);
        assert!(s.0.join("projects").is_dir(), "the folder is made if it wasn't there");
    }

    /// Sets what the user would have typed into the dialog's own fields.
    fn typed(h: &mut egui_kittest::Harness<'static, SheetApp>, set: impl FnOnce(&mut State)) {
        let mut state: State = h.ctx.data(|d| d.get_temp(state_id())).unwrap();
        set(&mut state);
        h.ctx.data_mut(|d| d.insert_temp(state_id(), state));
    }

    /// Puts input in the window the way the control channel does: from then on it counts as driven.
    pub(crate) fn drive(h: &mut egui_kittest::Harness<'static, SheetApp>) {
        h.state_mut().synthetic.push_back(vec![egui::Event::PointerMoved(egui::pos2(1.0, 1.0))]);
        h.run_steps(2);
        assert!(presence::driven(&h.ctx));
    }

    fn control(h: &mut egui_kittest::Harness<'static, SheetApp>, method: &str, params: Json) -> Json {
        let ctx = h.ctx.clone();
        match crate::control::handle(h.state_mut(), &ctx, method, &params) {
            crate::control::Outcome::Done(v) => v,
            _ => Json::Null,
        }
    }

    /// The control channel can rewrite a dialog's values. This dialog doesn't work from them.
    #[test]
    fn what_the_control_channel_writes_into_the_dialog_changes_nothing_it_works_from() {
        let s = Sandbox::new("values");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        for (field, value) in [
            ("python", json!("gridcraft-planted-python")),
            ("list", json!({"available": true, "folder": "elsewhere", "projects": []})),
            ("review", json!("clean-data")),
            ("follow", json!(true)),
            ("run", json!({"state": "done", "ok": true, "text": "planted", "script": "x", "project": "y"})),
        ] {
            assert_eq!(control(&mut h, "ui.dialog.set", json!({"field": field, "value": value}))["ok"], json!(true));
        }
        h.run_steps(3);
        // The next frame reports what the dialog really holds.
        let values = h.state().dialog.as_ref().unwrap().values.clone();
        assert_eq!(status(&values["list"], "clean-data").as_deref(), Some("untrusted"));
        assert_eq!((&values["python"], &values["review"], &values["follow"], &values["run"]), (&json!(""), &Json::Null, &json!(false), &Json::Null));
        assert!(h.query_by_label("Trust this project").is_none(), "no review was opened for the user");
        assert!(h.query_by_label_contains("planted").is_none());
        // The person checks Python: the one they named (none) is used and remembered, not the planted one.
        click(&mut h, "Check Python");
        assert_eq!(settings::load(&s.0.join("developer.json")).python, "");
        // And reviewing by hand still starts with nothing ticked.
        click(&mut h, "Review…");
        click(&mut h, "Trust this project");
        assert!(asked.questions.borrow()[0].contains("will ask you again"), "{:?}", asked.questions.borrow());
    }

    /// Clicks a button while the window is driven, and expects it to be refused as the person's alone.
    fn refused(h: &mut egui_kittest::Harness<'static, SheetApp>, label: &str) {
        click(h, label);
        let said = h.state_mut().message.take();
        assert_eq!(said.as_ref().map(|(_, text)| text.as_str()), Some(presence::BY_HAND), "{label}");
        h.run_steps(2);
    }

    /// The channel goes quiet: after a moment the window is the person's again.
    pub(crate) fn quiet(h: &mut egui_kittest::Harness<'static, SheetApp>) {
        h.state_mut().synthetic.clear();
        // The frame after the channel's last input still counts as its own.
        h.run_steps(2);
        std::thread::sleep(std::time::Duration::from_millis(1_200));
        h.run_steps(2);
        assert!(!presence::driven(&h.ctx));
    }

    /// The control channel clicks as a person does. Trusting is refused while it is putting input
    /// into the window: the operating system's question isn't even raised.
    #[test]
    fn the_control_channel_cannot_trust_a_project() {
        let _s = Sandbox::new("driven-trust");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        click(&mut h, "Review…");
        drive(&mut h);
        refused(&mut h, "Trust this project");
        assert!(asked.questions.borrow().is_empty());
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        // The review is still open. A tick the channel makes on "keep trusting" doesn't count either.
        click(&mut h, FOLLOW);
        assert_eq!(h.state().dialog.as_ref().unwrap().values["follow"], json!(false));
        // When the channel has been quiet for a moment, the person can.
        quiet(&mut h);
        click(&mut h, "Trust this project");
        assert_eq!(asked.questions.borrow().len(), 1);
        assert!(asked.questions.borrow()[0].contains("will ask you again"), "the channel's tick was not taken");
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("trusted"));
    }

    /// Nor can it run a trusted project's script, open its panel, or name the program to run as Python.
    #[test]
    fn the_control_channel_cannot_run_a_script_open_a_panel_or_name_a_python() {
        let s = Sandbox::new("driven-run");
        create("Clean Data").unwrap();
        let opened = Rc::new(RefCell::new(Vec::<String>::new()));
        let seen = opened.clone();
        let mut a = app_with(Some(&yes()));
        a.services.open_url = Some(Box::new(move |address: &str| seen.borrow_mut().push(address.to_string())));
        trust(&a, "clean-data", &fingerprint(), false).unwrap();
        let mut h = harness(a);
        typed(&mut h, |state| state.python = "gridcraft-no-such-python-here".into());

        drive(&mut h);
        refused(&mut h, "clean.py");
        assert!(h.state().dialog.as_ref().unwrap().values["run"].is_null(), "no script was started");
        refused(&mut h, "Open Panel");
        assert!(opened.borrow().is_empty() && panels::serving(&h.ctx).is_empty(), "no panel was served or opened");
        refused(&mut h, "Check Python");
        assert!(!s.0.join("developer.json").exists(), "the program named was not run or remembered");

        quiet(&mut h);
        click(&mut h, "Open Panel");
        assert_eq!(opened.borrow().len(), 1, "{:?}", h.state().message);
        assert_eq!(panels::serving(&h.ctx), ["clean-data"]);
        // Taking trust back is the channel's to do as much as anyone's: it only ever closes doors.
        drive(&mut h);
        click(&mut h, "Stop trusting");
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        assert!(panels::serving(&h.ctx).is_empty(), "and the panel is no longer served");
    }
}
