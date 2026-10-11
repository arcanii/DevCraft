//! Developer ▸ Modules: the modules the user has installed, and the one place where trust in a
//! module is given or taken back.
//!
//! A module does nothing until the user trusts it. Trusting is a decision about exact contents:
//! the dialog shows what the module asks to be allowed and what it contains, and what gets
//! trusted is the fingerprint the user was shown. If the files changed while the user was
//! reading, the decision is refused and they look again.
//!
//! There is deliberately no command that gives trust, so the control channel and MCP cannot ask
//! for it; `developer.modules` only reports. (Nothing runs yet in this phase. Before anything
//! does, giving trust also gets a confirmation drawn by the operating system: ADR-0001.)

use std::path::PathBuf;

use gridcraft_aniefork_dev::{TrustStore, describe, manifest, module, starter};
use serde_json::{Value as Json, json};

use crate::SheetApp;
use crate::dialogs::Dialog;
use crate::theme::Tokens;

/// The dialog's name, as `ui.dialog {name}` takes it.
pub const NAME: &str = "modules";

const FOLLOW: &str = "Keep trusting it as its files change (only for a module you are writing yourself)";

/// The modules folder and the trust file of this run. `None` when the run keeps no settings (the
/// web build, a run without preferences): then there are no modules.
#[cfg(not(test))]
fn places() -> Option<(PathBuf, PathBuf)> {
    use gridcraft_aniefork_dev::paths;
    Some((paths::modules_dir()?, paths::trust_file()?))
}

/// In tests: only the folder a test set up for itself, never the developer's own settings.
#[cfg(test)]
fn places() -> Option<(PathBuf, PathBuf)> {
    tests::sandbox().map(|dir| (dir.join("modules"), dir.join("trusted-modules.json")))
}

/// The user's modules and where each stands: what `developer.modules` reports and the dialog
/// shows. Reading only.
pub fn list() -> Json {
    let Some((dir, trust_file)) = places() else { return json!({"available": false, "folder": null, "modules": []}) };
    let store = TrustStore::load(&trust_file);
    let modules: Vec<Json> = module::scan(&dir).iter().map(|m| describe(m, &store)).collect();
    json!({"available": true, "folder": dir.display().to_string(), "modules": modules})
}

/// What the dialog starts from.
pub fn values() -> Json {
    json!({"list": list(), "review": null, "follow": false, "newName": ""})
}

/// The user pressed "Trust this module" while reviewing `id`. `reviewed` is the fingerprint they
/// were shown: only that is trusted.
fn trust(id: &str, reviewed: &str, follow_changes: bool) -> Result<String, String> {
    let (dir, trust_file) = places().ok_or("Modules are not available in this run.")?;
    if !manifest::valid_id(id) {
        return Err("That is not a module.".into());
    }
    let found = module::load(&dir.join(id));
    if found.fingerprint.as_deref() != Some(reviewed) {
        return Err(format!("{} changed while you were looking at it. Review it again before trusting it.", found.name()));
    }
    TrustStore::load(&trust_file).trust(&found, follow_changes)?;
    Ok(format!("{} is trusted.", found.name()))
}

fn stop_trusting(id: &str) -> Result<String, String> {
    let (_, trust_file) = places().ok_or("Modules are not available in this run.")?;
    TrustStore::load(&trust_file).revoke(id)?;
    Ok("The module is no longer trusted.".into())
}

fn create(name: &str) -> Result<String, String> {
    let (dir, _) = places().ok_or("Modules are not available in this run.")?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("The modules folder could not be created: {e}"))?;
    let created = starter::create(&dir, name)?;
    Ok(format!("{} was created. It is not trusted yet.", created.name()))
}

/// What the user asked for in one frame; carried out after the dialog is drawn.
enum Action {
    Refresh,
    Review(String),
    NotNow,
    Trust { id: String, fingerprint: String, follow: bool },
    StopTrusting(String),
    Create(String),
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

fn card(ui: &mut egui::Ui, m: &Json, reviewing: bool, follow: &mut bool, action: &mut Option<Action>) {
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
    });
}

/// Hook of the fork's dialogs: draws Developer ▸ Modules.
pub fn ui(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut Dialog, open: &mut bool) {
    ui.set_min_width(520.0);
    let listing = d.values.get("list").cloned().unwrap_or(Json::Null);
    let mut action = None;
    ui.label("Modules add panels and code to GridCraft. A module does nothing until you trust it.");
    if listing["available"] == true {
        ui.horizontal(|ui| {
            ui.label("Folder:");
            ui.add(egui::Label::new(egui::RichText::new(listing["folder"].as_str().unwrap_or("")).monospace().small()).selectable(true));
        });
        ui.separator();
        let modules = listing["modules"].as_array().cloned().unwrap_or_default();
        let reviewing = d.values.get("review").and_then(Json::as_str).unwrap_or("").to_string();
        let mut follow = d.values.get("follow").and_then(Json::as_bool).unwrap_or(false);
        egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
            if modules.is_empty() {
                ui.label("No modules are installed. Create one below, or copy a module's folder into the folder above.");
            }
            for m in &modules {
                card(ui, m, m["id"].as_str() == Some(reviewing.as_str()), &mut follow, &mut action);
            }
        });
        d.values.insert("follow".into(), json!(follow));
        ui.separator();
        ui.horizontal(|ui| {
            let label = ui.label("New module:");
            let mut name = d.values.get("newName").and_then(Json::as_str).unwrap_or("").to_string();
            if ui.add(egui::TextEdit::singleline(&mut name).desired_width(200.0).hint_text("Its name")).labelled_by(label.id).changed() {
                d.values.insert("newName".into(), json!(name));
            }
            if ui.button("Create").clicked() {
                action = Some(Action::Create(name));
            }
        });
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
            trust(&id, &fingerprint, follow)
        }
        Some(Action::StopTrusting(id)) => stop_trusting(&id),
        Some(Action::Create(name)) => create(&name).inspect(|_| {
            d.values.insert("newName".into(), json!(""));
        }),
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
    use egui_kittest::kittest::Queryable;
    use std::cell::RefCell;

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
            SANDBOX.with(|s| *s.borrow_mut() = None);
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    fn app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        SheetApp::new(session, Default::default())
    }

    fn status(list: &Json, id: &str) -> Option<String> {
        list["modules"].as_array()?.iter().find(|m| m["id"] == id)?["status"].as_str().map(str::to_string)
    }

    #[test]
    fn a_run_without_settings_has_no_modules() {
        assert_eq!(list(), json!({"available": false, "folder": null, "modules": []}));
        assert!(create("Clean Data").is_err() && trust("clean-data", "", false).is_err() && stop_trusting("clean-data").is_err());
    }

    #[test]
    fn agents_can_see_modules_but_no_command_gives_trust() {
        let s = Sandbox::new("agents");
        create("Clean Data").unwrap();
        let mut a = app();
        let seen = a.run("developer.modules", json!({})).unwrap();
        assert_eq!(seen["available"], json!(true));
        assert_eq!(status(&seen, "clean-data").as_deref(), Some("untrusted"));
        assert_eq!(seen["modules"][0]["granted"], json!([]));
        // Nothing among the fork's commands trusts, and unknown ids fall through to upstream.
        assert!(super::super::commands().all(|c| !c["id"].as_str().unwrap_or("").to_lowercase().contains("trust")));
        for id in ["developer.trust", "developer.trustModule", "developer.modules.trust"] {
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
        let fingerprint = list()["modules"][0]["fingerprint"].as_str().unwrap().to_string();
        for id in ["", "..", "../clean-data", "clean-data/..", "Clean-Data", "C:", "clean data"] {
            assert!(trust(id, &fingerprint, false).is_err(), "{id:?}");
        }
        assert!(trust("clean-data", "", false).is_err(), "no fingerprint");
        assert!(trust("clean-data", &"0".repeat(64), false).is_err(), "another fingerprint");
        assert!(trust("missing", &fingerprint, false).is_err(), "no such module");
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("untrusted"));
        assert!(!s.0.join("trusted-modules.json").exists(), "none of that wrote a decision");
        assert!(trust("clean-data", &fingerprint, false).is_ok());
        assert_eq!(status(&list(), "clean-data").as_deref(), Some("trusted"));
    }

    fn harness() -> egui_kittest::Harness<'static, SheetApp> {
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app(),
        );
        h.run_steps(4);
        h.state_mut().open_dialog(NAME, json!({}));
        // A new window takes a few frames to find its size and place; clicks go by position.
        h.run_steps(6);
        h
    }

    fn click(h: &mut egui_kittest::Harness<'static, SheetApp>, label: &str) {
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
        let mut h = harness();
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.clone()).as_deref(), Some(NAME));
        if let Some(d) = h.state_mut().dialog.as_mut() {
            d.values.insert("newName".into(), json!("Clean Data"));
        }
        click(&mut h, "Create");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"), "a new module is not trusted");
        assert!(s.0.join("modules/clean-data/module.toml").exists());

        click(&mut h, "Review…");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"), "looking is not trusting");
        click(&mut h, "Trust this module");
        assert_eq!(shown(&h).as_deref(), Some("trusted"));
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
    fn a_module_that_changes_while_being_reviewed_is_not_trusted() {
        let s = Sandbox::new("race");
        create("Clean Data").unwrap();
        let mut h = harness();
        click(&mut h, "Review…");
        // The files change after the user was shown them and before they press the button.
        std::fs::write(s.0.join("modules/clean-data/python/clean.py"), "import os\n").unwrap();
        click(&mut h, "Trust this module");
        assert_eq!(shown(&h).as_deref(), Some("untrusted"));
        assert!(h.state().message.as_ref().is_some_and(|(_, text)| text.contains("changed while you were looking")), "{:?}", h.state().message);
        assert!(!s.0.join("trusted-modules.json").exists());
    }

    #[test]
    fn a_broken_module_is_listed_with_its_problem_and_cannot_be_reviewed() {
        let s = Sandbox::new("broken");
        std::fs::create_dir_all(s.0.join("modules/half")).unwrap();
        std::fs::write(s.0.join("modules/half/readme.txt"), "no manifest").unwrap();
        let mut h = harness();
        let list = h.state().dialog.as_ref().unwrap().values["list"].clone();
        assert_eq!(status(&list, "half").as_deref(), Some("problem"));
        assert!(h.query_by_label("Review…").is_none(), "there is nothing to trust");
        h.run_steps(1);
    }
}
