//! Front-end code private to the aniefork fork of GridCraft.
//!
//! Everything the fork adds to the UI lives under this module, so merges from upstream
//! (`storytold/gridcraft`) only ever meet the short hooks that call into it. See
//! `AGENTS-aniefork.md` at the repository root.

pub mod selection;

use serde_json::{Value as Json, json};

use crate::SheetApp;

/// The fork's UI-only commands: id and label. Like upstream's own (`view.darkMode`…), they act on
/// the window rather than the workbook, so they live in the UI crate.
const COMMANDS: &[(&str, &str)] = &[("view.animations", "Animations")];

/// Hook of `SheetApp::run_ui_command`: runs `id` when it is one of the fork's commands.
///
/// - `view.animations {on?: bool}` turns motion in the UI on or off (today: the selection
///   outline's glide); without `on` it toggles. Returns `{on}`. Saved with the UI preferences.
pub fn run_command(app: &mut SheetApp, id: &str, p: &Json) -> Option<Result<Json, String>> {
    match id {
        "view.animations" => {
            app.ui.animations = p.get("on").and_then(Json::as_bool).unwrap_or(!app.ui.animations);
            Some(Ok(json!({"on": app.ui.animations})))
        }
        _ => None,
    }
}

/// Hook of the control channel's `engine.commands`: the fork's commands, listed like upstream's
/// UI commands.
pub fn commands() -> impl Iterator<Item = Json> {
    COMMANDS.iter().map(|(id, label)| json!({"id": id, "label": label, "enabled": true, "ui": true}))
}

/// Hook of the ribbon's View tab: the fork's settings, under Dark Mode (settings live in the
/// View tab).
pub fn view_options(app: &mut SheetApp, ui: &mut egui::Ui) {
    let mut on = app.ui.animations;
    if ui.checkbox(&mut on, "Animations").on_hover_text("Glide the selection outline to the cell you move to instead of jumping there").changed() {
        app.run_or_alert("view.animations", json!({"on": on}));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UiState;

    fn app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        SheetApp::new(session, Default::default())
    }

    #[test]
    fn animations_are_on_until_turned_off_and_the_choice_is_saved() {
        assert!(UiState::default().animations);
        // A `ui.json` written before the setting existed (or by upstream) leaves them on.
        let old: UiState = serde_json::from_str(r#"{"dark": true, "ribbonTab": "View"}"#).unwrap();
        assert!(old.animations && old.dark);
        let off: UiState = serde_json::from_value(json!({"animations": false})).unwrap();
        assert!(!off.animations);
        let saved = serde_json::to_value(&off).unwrap();
        assert_eq!(saved["animations"], json!(false));
        assert!(!serde_json::from_value::<UiState>(saved).unwrap().animations);
    }

    #[test]
    fn the_command_sets_and_toggles_animations() {
        let mut a = app();
        assert_eq!(a.run("view.animations", json!({"on": false})), Ok(json!({"on": false})));
        assert!(!a.ui.animations);
        assert_eq!(a.run("view.animations", json!({"on": false})), Ok(json!({"on": false})), "setting it twice changes nothing");
        // No parameters: it toggles, and never opens a dialog.
        assert_eq!(a.run("view.animations", json!({})), Ok(json!({"on": true})));
        assert_eq!(a.run("view.animations", json!({})), Ok(json!({"on": false})));
        assert!(a.dialog.is_none() && a.message.is_none());
        // Parameters that make no sense toggle too, rather than failing or panicking.
        assert_eq!(a.run("view.animations", json!({"on": "yes"})), Ok(json!({"on": true})));
        assert_eq!(a.run("view.animations", Json::Null), Ok(json!({"on": false})));
        // Other ids are left to upstream.
        assert!(run_command(&mut a, "view.darkMode", &json!({})).is_none());
        assert!(run_command(&mut a, "", &json!({})).is_none());
    }

    #[test]
    fn agents_find_and_run_the_command_over_the_control_channel() {
        let mut a = app();
        let ctx = egui::Context::default();
        let reply = |o: crate::control::Outcome| match o {
            crate::control::Outcome::Done(v) => v,
            crate::control::Outcome::Screenshot { .. } => Json::Null,
        };
        let listed = reply(crate::control::handle(&mut a, &ctx, "engine.commands", &json!({})));
        let entry = listed["result"].as_array().and_then(|all| all.iter().find(|c| c["id"] == "view.animations")).cloned();
        assert_eq!(entry, Some(json!({"id": "view.animations", "label": "Animations", "enabled": true, "ui": true})));
        let ran = reply(crate::control::handle(&mut a, &ctx, "engine.execute", &json!({"command": "view.animations", "params": {"on": false}})));
        assert_eq!(ran, json!({"ok": true, "result": {"on": false}}));
        // `ui.inspect` reports it with the other UI preferences.
        let seen = reply(crate::control::handle(&mut a, &ctx, "ui.inspect", &json!({})));
        assert_eq!(seen["result"]["ui"]["animations"], json!(false));
    }
}
