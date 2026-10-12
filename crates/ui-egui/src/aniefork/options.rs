//! The fork's settings, on upstream's Options page (File ▸ Options), and the commands behind them.
//!
//! Upstream's page holds the display theme, the interface language, the formula bar, AutoSave and
//! the calculation mode. The fork adds the two settings of its own:
//!
//! - **User name**: the author of the comments and notes the user adds. `app.userName {name?}`
//!   sets it; without a name it only reports.
//! - **Show the Developer tab**: `developer.tab {on?}` shows or hides the ribbon's Developer tab,
//!   and toggles it without `on`.
//!
//! A value that makes no sense is an error and changes nothing. Neither command opens anything.
//!
//! (The fork had a settings dialog of its own, and a `file.options` command, before upstream had
//! an Options page. Upstream's `file.options` now opens that page, and the dialog is gone.)

use serde_json::{Value as Json, json};

use super::developer;
use crate::SheetApp;

/// The longest user name kept (characters).
const MAX_USER_NAME: usize = 100;

/// `developer.tab {on?: bool}`: shows or hides the ribbon's Developer tab. Returns `{on}`.
pub fn developer_tab(app: &mut SheetApp, p: &Json) -> Result<Json, String> {
    let on = match p.get("on") {
        None | Some(Json::Null) => !app.ui.developer_tab,
        Some(Json::Bool(on)) => *on,
        Some(other) => return Err(format!("`on` must be true or false, not {other}")),
    };
    app.ui.developer_tab = on;
    if !on && app.ui.ribbon_tab == developer::TAB {
        app.ui.ribbon_tab = "Home".into();
    }
    Ok(json!({"on": on}))
}

/// `app.userName {name?}`: sets the name that signs comments and notes. Returns `{name}`.
pub fn user_name(app: &mut SheetApp, p: &Json) -> Result<Json, String> {
    match p.get("name") {
        None | Some(Json::Null) => {}
        Some(Json::String(name)) => {
            let name = name.trim();
            if name.is_empty() {
                return Err("the user name can't be empty".into());
            }
            if name.chars().count() > MAX_USER_NAME {
                return Err(format!("the user name can't be longer than {MAX_USER_NAME} characters"));
            }
            app.session.prefs.user_name = name.to_string();
        }
        Some(other) => return Err(format!("`name` must be text, not {other}")),
    }
    Ok(json!({"name": app.session.prefs.user_name}))
}

fn draft_id() -> egui::Id {
    egui::Id::new("aniefork.options.userName")
}

/// Hook of upstream's Options page: the fork's settings, at the end of its General section. Like
/// upstream's own there, they apply at once.
pub fn page(app: &mut SheetApp, ui: &mut egui::Ui) {
    ui.add_space(6.0);
    let escaping = ui.input(|i| i.key_pressed(egui::Key::Escape));
    let mut entered = None;
    ui.horizontal(|ui| {
        let label = ui.label("User name");
        // What is being typed is kept until the field is left; then it becomes the name.
        let current = app.session.prefs.user_name.clone();
        let mut text = ui.data(|d| d.get_temp::<String>(draft_id())).unwrap_or_else(|| current.clone());
        let field = ui.add(egui::TextEdit::singleline(&mut text).desired_width(200.0)).labelled_by(label.id);
        if field.changed() {
            ui.data_mut(|d| d.insert_temp(draft_id(), text.clone()));
        }
        if field.lost_focus() {
            ui.data_mut(|d| d.remove::<String>(draft_id()));
            if text != current && !escaping {
                entered = Some(text);
            }
        }
    });
    ui.label(egui::RichText::new("The user name is the author of the comments and notes you add.").small());
    if let Some(name) = entered {
        app.run_or_alert("app.userName", json!({"name": name}));
    }
    ui.add_space(6.0);
    let mut shown = app.ui.developer_tab;
    if ui.checkbox(&mut shown, "Show the Developer tab").on_hover_text("Tools for people who extend and automate GridCraft").changed() {
        app.run_or_alert("developer.tab", json!({"on": shown}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control::{self, Outcome};
    use egui_kittest::kittest::Queryable;

    fn app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        SheetApp::new(session, Default::default())
    }

    #[test]
    fn the_developer_tab_is_shown_hidden_and_toggled_by_its_command() {
        let mut a = app();
        assert_eq!(a.run("developer.tab", json!({"on": true})), Ok(json!({"on": true})));
        assert!(a.ui.developer_tab);
        assert_eq!(a.run("developer.tab", json!({"on": true})), Ok(json!({"on": true})), "setting it twice changes nothing");
        // No parameters: it toggles, and opens nothing.
        assert_eq!(a.run("developer.tab", json!({})), Ok(json!({"on": false})));
        assert_eq!(a.run("developer.tab", Json::Null), Ok(json!({"on": true})));
        assert!(a.dialog.is_none() && a.message.is_none() && a.backstage.is_none());
        for bad in [json!({"on": "yes"}), json!({"on": 1}), json!({"on": []})] {
            assert!(a.run("developer.tab", bad.clone()).is_err(), "{bad}");
            assert!(a.ui.developer_tab, "{bad} changed something");
        }
        // The saved preferences carry it.
        assert_eq!(serde_json::to_value(&a.ui).unwrap()["developerTab"], json!(true));
        // Hiding the tab while on it goes back to Home.
        a.ui.ribbon_tab = developer::TAB.into();
        a.run("developer.tab", json!({"on": false})).unwrap();
        assert_eq!(a.ui.ribbon_tab, "Home");
    }

    #[test]
    fn the_user_name_is_set_and_reported_by_its_command() {
        let mut a = app();
        let before = a.session.prefs.user_name.clone();
        assert_eq!(a.run("app.userName", json!({})), Ok(json!({"name": before})), "without a name it only reports");
        assert_eq!(a.run("app.userName", json!({"name": "  Ada Lovelace  "})), Ok(json!({"name": "Ada Lovelace"})), "names are trimmed");
        assert_eq!(a.session.prefs.user_name, "Ada Lovelace");
        for bad in [
            json!({"name": ""}),
            json!({"name": "   "}),
            json!({"name": "x".repeat(MAX_USER_NAME + 1)}),
            json!({"name": 7}),
            json!({"name": ["Ada"]}),
        ] {
            assert!(a.run("app.userName", bad.clone()).is_err(), "{bad}");
            assert_eq!(a.session.prefs.user_name, "Ada Lovelace", "{bad} changed something");
        }
        assert!(a.run("app.userName", json!({"name": "x".repeat(MAX_USER_NAME)})).is_ok(), "the longest name allowed");
        assert!(a.dialog.is_none() && a.message.is_none() && a.backstage.is_none());
    }

    #[test]
    fn agents_find_and_run_the_commands_and_upstreams_file_options_is_left_alone() {
        let mut a = app();
        let ctx = egui::Context::default();
        let reply = |o: Outcome| match o {
            Outcome::Done(v) => v,
            _ => Json::Null,
        };
        for (id, label) in [("developer.tab", "Developer Tab"), ("app.userName", "User Name")] {
            let listed = reply(control::handle(&mut a, &ctx, "engine.commands", &json!({"search": id})));
            let entry = listed["result"].as_array().and_then(|all| all.iter().find(|c| c["id"] == id)).cloned().unwrap_or_default();
            assert_eq!((&entry["label"], &entry["ui"]), (&json!(label), &json!(true)), "{listed}");
        }
        let ran = reply(control::handle(&mut a, &ctx, "engine.execute", &json!({"command": "developer.tab", "params": {"on": true}})));
        assert_eq!(ran, json!({"ok": true, "result": {"on": true}}));
        assert_eq!(reply(control::handle(&mut a, &ctx, "ui.inspect", &json!({})))["result"]["ui"]["developerTab"], json!(true));
        // `file.options` is upstream's: it opens the Options page, where the fork's settings are.
        assert!(super::super::run_command(&mut a, "file.options", &json!({})).is_none());
        a.run("file.options", json!({})).unwrap();
        assert_eq!(a.backstage.as_ref().map(|b| b.page), Some(crate::backstage::Page::Options));
    }

    /// The whole path in the real window: File, Options, tick the box, type a name.
    #[test]
    fn the_options_page_shows_the_forks_settings_and_they_apply_at_once() {
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app(),
        );
        h.run_steps(4);
        h.state_mut().open_dialog("options", json!({}));
        h.run_steps(4);
        assert!(!h.state().ui.developer_tab);
        h.get_by_label("Show the Developer tab").click();
        h.run_steps(3);
        assert!(h.state().ui.developer_tab);
        h.get_by_label("User name").focus();
        h.run_steps(2);
        h.get_by_label("User name").type_text(" Jr");
        h.run_steps(2);
        let typed = h.state().session.prefs.user_name.clone();
        h.key_press(egui::Key::Enter);
        h.run_steps(3);
        let named = h.state().session.prefs.user_name.clone();
        assert!(named != typed && named.contains("Jr"), "the name is set when the field is left: {typed:?} then {named:?}");
        assert!(h.state().backstage.is_some(), "the page stays");
    }
}
