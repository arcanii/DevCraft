//! Front-end code private to the aniefork fork of GridCraft.
//!
//! Everything the fork adds to the UI lives under this module, so merges from upstream
//! (`storytold/gridcraft`) only ever meet the short hooks that call into it. See
//! `AGENTS-aniefork.md` at the repository root.

pub mod developer;
pub mod modules;
pub mod options;
pub mod selection;

use serde_json::{Value as Json, json};

use crate::SheetApp;
use crate::i18n::Language;

/// The fork's UI-only commands: id, label and params doc. Like upstream's own (`view.theme`…),
/// they act on the window rather than the workbook, so they live in the UI crate.
const COMMANDS: &[(&str, &str, &str)] = &[
    ("view.animations", "Animations", "{on?: bool} (toggles without it) → {on}"),
    (
        "file.options",
        "Options",
        "{theme?: \"system\"|\"light\"|\"dark\", language?: \"en\"…, userName?, animations?: bool, developerTab?: bool} changes the settings named and returns all of them; with no params it only reports them",
    ),
    (
        "developer.modules",
        "Modules",
        "→ {available, folder, modules: [{id, name, version, status: \"trusted\"|\"untrusted\"|\"changed\"|\"problem\", asks, granted, files, fingerprint…}]}. It only reports: trust is given by the user alone, in Developer ▸ Modules",
    ),
];

/// Hook of `SheetApp::run_ui_command`: runs `id` when it is one of the fork's commands.
///
/// - `view.animations {on?: bool}` turns motion in the UI on or off (today: the selection
///   outline's glide); without `on` it toggles. Returns `{on}`. Saved with the UI preferences.
/// - `file.options {…}` is the settings dialog's command: see [`options`].
/// - `developer.modules` reports the user's modules and whether each is trusted: see [`modules`].
///   There is no command that gives trust.
pub fn run_command(app: &mut SheetApp, id: &str, p: &Json) -> Option<Result<Json, String>> {
    match id {
        "view.animations" => {
            app.ui.animations = p.get("on").and_then(Json::as_bool).unwrap_or(!app.ui.animations);
            Some(Ok(json!({"on": app.ui.animations})))
        }
        "file.options" => Some(options::run(app, p)),
        "developer.modules" => Some(Ok(modules::list())),
        _ => None,
    }
}

/// Hook of the control channel's `engine.commands`: the fork's commands, listed like upstream's
/// UI commands.
pub fn commands() -> impl Iterator<Item = Json> {
    COMMANDS.iter().map(|(id, label, params)| json!({"id": id, "label": label, "params": params, "enabled": true, "ui": true}))
}

const ANIMATIONS: &str = "Animations";
const ANIMATIONS_HINT: &str = "Glide the selection outline to the cell you move to instead of jumping there";

/// The fork's interface strings in the languages upstream translates. They are kept here so that
/// upstream's catalogs stay untouched; any other language shows the English text, as upstream's
/// untranslated labels do.
fn tr(lang: Language, text: &'static str) -> &'static str {
    match (text, lang) {
        (ANIMATIONS, Language::Ja) => "アニメーション",
        (ANIMATIONS, Language::Zh) => "动画",
        (ANIMATIONS, Language::Ko) => "애니메이션",
        (ANIMATIONS, Language::Ru) => "Анимация",
        (ANIMATIONS, Language::PtBr) => "Animações",
        (ANIMATIONS_HINT, Language::Ja) => "選択範囲の枠を、移動先のセルまで滑らかに動かします（瞬時には切り替えません）",
        (ANIMATIONS_HINT, Language::Zh) => "选择框平滑滑动到新的单元格，而不是直接跳过去",
        (ANIMATIONS_HINT, Language::Ko) => "선택 테두리가 바로 건너뛰지 않고 이동한 셀까지 부드럽게 미끄러집니다",
        (ANIMATIONS_HINT, Language::Ru) => "Рамка выделения плавно скользит к новой ячейке, а не перескакивает",
        (ANIMATIONS_HINT, Language::PtBr) => "Desliza o contorno da seleção até a nova célula em vez de saltar",
        _ => text,
    }
}

/// Hook of `Dialog::open`: for a name that is one of the fork's dialogs, its title, the values it
/// starts from, and the command OK runs with those values.
pub fn dialog(app: &SheetApp, name: &str, _params: &Json) -> Option<(&'static str, Json, Option<&'static str>)> {
    match name {
        options::NAME => Some(("Options", options::values(app), Some("file.options"))),
        // No command: nothing an agent can confirm gives a module trust.
        modules::NAME => Some(("Modules", modules::values(), None)),
        _ => None,
    }
}

/// Hook of `dialogs::show`: whether the fork draws the dialog of this name.
pub fn draws_dialog(name: &str) -> bool {
    matches!(name, options::NAME | modules::NAME)
}

/// Hook of `dialogs::show`: draws one of the fork's dialogs.
pub fn dialog_ui(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut crate::dialogs::Dialog, confirm: &mut bool, open: &mut bool) {
    match d.name.as_str() {
        options::NAME => options::ui(app, ui, d, confirm, open),
        modules::NAME => modules::ui(app, ui, d, open),
        _ => {}
    }
}

/// Hook of the ribbon's File menu: the fork's items, after upstream's.
pub fn file_menu(app: &mut SheetApp, ui: &mut egui::Ui) {
    ui.separator();
    if ui.button("Options…").clicked() {
        app.open_dialog(options::NAME, json!({}));
        ui.close();
    }
}

/// Hook of the ribbon's View tab: the fork's settings, under upstream's Display theme picker
/// (settings live in the View tab).
pub fn view_options(app: &mut SheetApp, ui: &mut egui::Ui) {
    let lang = app.ui.language;
    let mut on = app.ui.animations;
    if ui.checkbox(&mut on, tr(lang, ANIMATIONS)).on_hover_text(tr(lang, ANIMATIONS_HINT)).changed() {
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
        assert!(run_command(&mut a, "view.theme", &json!({})).is_none());
        assert!(run_command(&mut a, "", &json!({})).is_none());
    }

    #[test]
    fn agents_find_and_run_the_command_over_the_control_channel() {
        let mut a = app();
        let ctx = egui::Context::default();
        let reply = |o: crate::control::Outcome| match o {
            crate::control::Outcome::Done(v) => v,
            _ => Json::Null,
        };
        let listed = reply(crate::control::handle(&mut a, &ctx, "engine.commands", &json!({})));
        let entry = listed["result"].as_array().and_then(|all| all.iter().find(|c| c["id"] == "view.animations")).cloned().unwrap();
        assert_eq!((&entry["label"], &entry["enabled"], &entry["ui"]), (&json!("Animations"), &json!(true), &json!(true)));
        assert!(entry["params"].as_str().is_some_and(|doc| doc.contains("on")), "{entry}");
        // Upstream's search of the listing finds it too.
        let found = reply(crate::control::handle(&mut a, &ctx, "engine.commands", &json!({"search": "animations"})));
        assert!(found["result"].as_array().is_some_and(|all| all.iter().any(|c| c["id"] == "view.animations")), "{found}");
        let ran = reply(crate::control::handle(&mut a, &ctx, "engine.execute", &json!({"command": "view.animations", "params": {"on": false}})));
        assert_eq!(ran, json!({"ok": true, "result": {"on": false}}));
        // `ui.inspect` reports it with the other UI preferences.
        let seen = reply(crate::control::handle(&mut a, &ctx, "ui.inspect", &json!({})));
        assert_eq!(seen["result"]["ui"]["animations"], json!(false));
    }

    #[test]
    fn the_setting_is_worded_in_the_languages_upstream_translates() {
        for text in [ANIMATIONS, ANIMATIONS_HINT] {
            assert_eq!(tr(Language::En, text), text);
            for lang in [Language::Ja, Language::Zh, Language::Ko, Language::Ru, Language::PtBr] {
                let translated = tr(lang, text);
                assert!(!translated.is_empty() && translated != text, "{lang:?}: {text}");
            }
            // Every language shows something: its own words, or the English ones.
            for lang in Language::ALL {
                assert!(!tr(lang, text).is_empty());
            }
        }
        assert_eq!(tr(Language::Ja, "Sheet1!A1"), "Sheet1!A1", "text that isn't the fork's is left alone");
    }
}
