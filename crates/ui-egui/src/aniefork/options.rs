//! File ▸ Options: the settings dialog, and `file.options`, the command behind it.
//!
//! Upstream keeps its few settings on the View tab and lists the Options dialog as missing. This
//! one gathers the settings that work today: the display theme and interface language (upstream's
//! own, applied through upstream's commands), the user name (the author of comments and notes),
//! Animations, and whether the ribbon shows the Developer tab.
//!
//! The dialog edits a copy; nothing changes until OK, which runs `file.options` with the copy.
//! Run by an agent, the command never opens the dialog: without parameters it reports the
//! settings, with parameters it changes the ones named. A value that makes no sense is an error
//! and changes nothing.

use egui::vec2;
use serde_json::{Value as Json, json};

use super::{ANIMATIONS, ANIMATIONS_HINT, developer, tr};
use crate::SheetApp;
use crate::dialogs::Dialog;
use crate::i18n::Language;

/// The dialog's name, as `ui.dialog {name}` takes it.
pub const NAME: &str = "options";

/// The display themes `view.theme` takes, with upstream's labels for them.
const THEMES: [(&str, &str); 3] = [("system", "System"), ("light", "Light"), ("dark", "Dark")];
/// The longest user name kept (characters).
const MAX_USER_NAME: usize = 100;

/// The settings as `file.options` reports them, and as the dialog starts from.
pub fn values(app: &SheetApp) -> Json {
    json!({
        "theme": app.ui.theme_mode(),
        "language": app.ui.language.code(),
        "userName": app.session.prefs.user_name,
        "animations": app.ui.animations,
        "developerTab": app.ui.developer_tab,
    })
}

/// A text parameter: absent (or null) is `None`; anything but text is an error.
fn text<'a>(p: &'a Json, key: &str) -> Result<Option<&'a str>, String> {
    match p.get(key) {
        None | Some(Json::Null) => Ok(None),
        Some(Json::String(s)) => Ok(Some(s)),
        Some(other) => Err(format!("`{key}` must be text, not {other}")),
    }
}

/// An on/off parameter: absent (or null) is `None`; anything but true or false is an error.
fn flag(p: &Json, key: &str) -> Result<Option<bool>, String> {
    match p.get(key) {
        None | Some(Json::Null) => Ok(None),
        Some(Json::Bool(b)) => Ok(Some(*b)),
        Some(other) => Err(format!("`{key}` must be true or false, not {other}")),
    }
}

/// `file.options`: changes the settings named in `p` and returns all of them.
pub fn run(app: &mut SheetApp, p: &Json) -> Result<Json, String> {
    // Check everything before changing anything, so a bad value changes nothing.
    let theme = text(p, "theme")?;
    if let Some(mode) = theme
        && !THEMES.iter().any(|(value, _)| *value == mode)
    {
        return Err(format!("unknown theme {mode:?} (use \"system\", \"light\" or \"dark\")"));
    }
    let language = match text(p, "language")? {
        Some(code) => Some(Language::parse(code).ok_or_else(|| format!("unknown language {code:?}"))?),
        None => None,
    };
    let user_name = match text(p, "userName")?.map(str::trim) {
        Some("") => return Err("the user name can't be empty".into()),
        Some(name) if name.chars().count() > MAX_USER_NAME => return Err(format!("the user name can't be longer than {MAX_USER_NAME} characters")),
        other => other,
    };
    let animations = flag(p, "animations")?;
    let developer_tab = flag(p, "developerTab")?;

    // The theme and the language are upstream's settings: change them the way upstream does.
    if let Some(mode) = theme {
        app.run("view.theme", json!({"mode": mode}))?;
    }
    if let Some(language) = language {
        app.run("app.language.set", json!({"language": language.code()}))?;
    }
    if let Some(name) = user_name {
        app.session.prefs.user_name = name.to_string();
    }
    if let Some(on) = animations {
        app.ui.animations = on;
    }
    if let Some(on) = developer_tab {
        app.ui.developer_tab = on;
        if !on && app.ui.ribbon_tab == developer::TAB {
            app.ui.ribbon_tab = "Home".into();
        }
    }
    Ok(values(app))
}

fn section(ui: &mut egui::Ui, title: &str) {
    ui.add_space(6.0);
    ui.label(egui::RichText::new(title).strong());
    ui.add_space(2.0);
}

/// Hook of `dialogs::show`: draws the Options dialog. It edits the dialog's own values; OK runs
/// `file.options` with them.
pub fn ui(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut Dialog, confirm: &mut bool, open: &mut bool) {
    let lang = app.ui.language;
    ui.set_min_width(400.0);
    let value = |d: &Dialog, key: &str| d.values.get(key).and_then(Json::as_str).unwrap_or("").to_string();
    let is_on = |d: &Dialog, key: &str| d.values.get(key).and_then(Json::as_bool).unwrap_or(false);

    section(ui, "General");
    egui::Grid::new("options_general").num_columns(2).spacing(vec2(14.0, 8.0)).show(ui, |ui| {
        ui.label(lang.tr("Display theme"));
        let theme = value(d, "theme");
        let shown = THEMES.iter().find(|(v, _)| *v == theme).map_or("Light", |(_, label)| *label);
        egui::ComboBox::from_id_salt("options_theme").width(180.0).selected_text(lang.tr(shown)).show_ui(ui, |ui| {
            for (v, label) in THEMES {
                if ui.selectable_label(theme == v, lang.tr(label)).clicked() {
                    d.values.insert("theme".into(), json!(v));
                }
            }
        });
        ui.end_row();

        ui.label(lang.tr("Interface language"));
        let code = value(d, "language");
        let current = Language::parse(&code).unwrap_or_default();
        egui::ComboBox::from_id_salt("options_language").width(180.0).selected_text(current.name()).show_ui(ui, |ui| {
            for l in Language::ALL {
                if ui.selectable_label(l == current, l.name()).clicked() {
                    d.values.insert("language".into(), json!(l.code()));
                }
            }
        });
        ui.end_row();

        let label = ui.label("User name");
        let mut name = value(d, "userName");
        if ui.add(egui::TextEdit::singleline(&mut name).desired_width(180.0)).labelled_by(label.id).changed() {
            d.values.insert("userName".into(), json!(name));
        }
        ui.end_row();
    });
    ui.label(egui::RichText::new("The user name is the author of the comments and notes you add.").small());

    section(ui, "Feedback");
    let mut animations = is_on(d, "animations");
    if ui.checkbox(&mut animations, tr(lang, ANIMATIONS)).on_hover_text(tr(lang, ANIMATIONS_HINT)).changed() {
        d.values.insert("animations".into(), json!(animations));
    }

    section(ui, "Ribbon");
    let mut developer_tab = is_on(d, "developerTab");
    if ui.checkbox(&mut developer_tab, "Show the Developer tab").on_hover_text("Tools for people who extend and automate GridCraft").changed() {
        d.values.insert("developerTab".into(), json!(developer_tab));
    }

    // OK and Cancel, as upstream's dialogs have them: Enter is OK.
    ui.add_space(10.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let accent = crate::theme::Tokens::get(ui.ctx()).accent;
        let ok = ui.add(egui::Button::new(egui::RichText::new("OK").color(egui::Color32::WHITE)).fill(accent).min_size(vec2(64.0, 0.0)));
        if ok.clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            *confirm = true;
        }
        if ui.add(egui::Button::new("Cancel").min_size(vec2(64.0, 0.0))).clicked() {
            *open = false;
        }
    });
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

    fn reply(outcome: Outcome) -> Json {
        match outcome {
            Outcome::Done(v) => v,
            _ => Json::Null,
        }
    }

    #[test]
    fn without_parameters_it_reports_the_settings_and_opens_nothing() {
        let mut a = app();
        let defaults = json!({"theme": "light", "language": "en", "userName": "GridCraft User", "animations": true, "developerTab": false});
        assert_eq!(a.run("file.options", json!({})), Ok(defaults.clone()));
        assert_eq!(a.run("file.options", Json::Null), Ok(defaults.clone()), "no parameters at all");
        assert_eq!(a.run("file.options", json!({"theme": null})), Ok(defaults), "a null is an absent parameter");
        assert!(a.dialog.is_none() && a.message.is_none(), "run by an agent it never opens the dialog");
    }

    #[test]
    fn it_changes_the_settings_it_is_given_and_leaves_the_rest() {
        let mut a = app();
        let r = a.run("file.options", json!({"theme": "dark", "userName": "  Ada Lovelace  "})).unwrap();
        assert_eq!((r["theme"].as_str(), r["userName"].as_str()), (Some("dark"), Some("Ada Lovelace")), "names are trimmed");
        assert!(a.ui.dark && !a.ui.system_theme);
        assert_eq!(a.session.prefs.user_name, "Ada Lovelace");
        assert!(a.ui.animations && !a.ui.developer_tab, "what wasn't named is untouched");

        let r = a.run("file.options", json!({"theme": "system", "language": "ja", "animations": false, "developerTab": true})).unwrap();
        assert_eq!(r, json!({"theme": "system", "language": "ja", "userName": "Ada Lovelace", "animations": false, "developerTab": true}));
        assert!(a.ui.system_theme && a.ui.language == Language::Ja && !a.ui.animations && a.ui.developer_tab);
        // The saved preferences carry them.
        let saved = serde_json::to_value(&a.ui).unwrap();
        assert_eq!((&saved["developerTab"], &saved["animations"], &saved["language"]), (&json!(true), &json!(false), &json!("ja")));
    }

    #[test]
    fn a_value_that_makes_no_sense_is_an_error_and_changes_nothing() {
        let mut a = app();
        let before = values(&a);
        for bad in [
            json!({"theme": "sepia"}),
            json!({"theme": 3}),
            json!({"language": "xx"}),
            json!({"language": ["ja"]}),
            json!({"userName": ""}),
            json!({"userName": "   "}),
            json!({"userName": "x".repeat(MAX_USER_NAME + 1)}),
            json!({"userName": 7}),
            json!({"animations": "no"}),
            json!({"developerTab": 1}),
            // One bad value among good ones: none of them is applied.
            json!({"theme": "dark", "language": "ja", "animations": false, "developerTab": "yes"}),
        ] {
            assert!(a.run("file.options", bad.clone()).is_err(), "{bad}");
            assert_eq!(values(&a), before, "{bad} changed something");
        }
        assert!(a.run("file.options", json!({"userName": "x".repeat(MAX_USER_NAME)})).is_ok(), "the longest name allowed");
        assert!(a.run("file.options", json!({"unknownSetting": 1})).is_ok(), "settings it doesn't know are ignored");
    }

    #[test]
    fn hiding_the_developer_tab_while_on_it_goes_back_to_home() {
        let mut a = app();
        a.run("file.options", json!({"developerTab": true})).unwrap();
        a.ui.ribbon_tab = developer::TAB.into();
        a.run("file.options", json!({"developerTab": false})).unwrap();
        assert_eq!(a.ui.ribbon_tab, "Home");
    }

    #[test]
    fn agents_open_fill_in_and_confirm_the_dialog() {
        let mut a = app();
        let ctx = egui::Context::default();
        let listed = reply(control::handle(&mut a, &ctx, "engine.commands", &json!({"search": "file.options"})));
        assert!(listed["result"].as_array().is_some_and(|all| all.iter().any(|c| c["id"] == "file.options" && c["ui"] == true)), "{listed}");
        let opened = reply(control::handle(&mut a, &ctx, "ui.dialog", &json!({"name": "options"})));
        assert_eq!(opened["result"]["dialog"], json!("options"));
        assert_eq!(a.dialog.as_ref().map(|d| Json::Object(d.values.clone())), Some(values(&a)), "it starts from the settings");
        reply(control::handle(&mut a, &ctx, "ui.dialog.set", &json!({"field": "developerTab", "value": true})));
        reply(control::handle(&mut a, &ctx, "ui.dialog.set", &json!({"field": "userName", "value": "Grace"})));
        assert!(!a.ui.developer_tab, "nothing changes before OK");
        let done = reply(control::handle(&mut a, &ctx, "ui.dialog.confirm", &json!({})));
        assert_eq!(done["ok"], json!(true), "{done}");
        assert!(a.dialog.is_none() && a.ui.developer_tab && a.session.prefs.user_name == "Grace");
        // Cancel discards; a bad value keeps the dialog open with the error.
        reply(control::handle(&mut a, &ctx, "ui.dialog", &json!({"name": "options"})));
        reply(control::handle(&mut a, &ctx, "ui.dialog.set", &json!({"field": "animations", "value": false})));
        reply(control::handle(&mut a, &ctx, "ui.dialog.cancel", &json!({})));
        assert!(a.dialog.is_none() && a.ui.animations);
        reply(control::handle(&mut a, &ctx, "ui.dialog", &json!({"name": "options"})));
        reply(control::handle(&mut a, &ctx, "ui.dialog.set", &json!({"field": "userName", "value": ""})));
        let refused = reply(control::handle(&mut a, &ctx, "ui.dialog.confirm", &json!({})));
        assert_eq!(refused["ok"], json!(false));
        assert!(a.dialog.is_some() && a.session.prefs.user_name == "Grace");
    }

    /// The whole path in the real window: File ▸ Options…, tick a box, OK.
    #[test]
    fn the_file_menu_opens_the_dialog_and_ok_applies_it() {
        let mut h = egui_kittest::Harness::builder().with_size(vec2(1200.0, 800.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app(),
        );
        h.run_steps(4);
        h.get_by_label("File").click();
        h.run_steps(3);
        h.get_by_label("Options…").click();
        h.run_steps(3);
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.clone()).as_deref(), Some(NAME));
        h.get_by_label("Show the Developer tab").click();
        h.get_by_label("Animations").click();
        h.run_steps(2);
        assert!(!h.state().ui.developer_tab && h.state().ui.animations, "the dialog edits a copy");
        h.get_by_label("OK").click();
        h.run_steps(3);
        assert!(h.state().dialog.is_none(), "OK closes it");
        assert!(h.state().ui.developer_tab && !h.state().ui.animations, "and applies the copy");
        // It opens again showing the new settings, and Cancel leaves them alone.
        h.state_mut().open_dialog(NAME, json!({}));
        h.run_steps(2);
        h.get_by_label("Show the Developer tab").click();
        h.run_steps(2);
        h.get_by_label("Cancel").click();
        h.run_steps(3);
        assert!(h.state().dialog.is_none() && h.state().ui.developer_tab);
    }
}
