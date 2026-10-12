//! The Developer tab: the ribbon's home for people who extend and automate GridCraft.
//!
//! It is hidden until File ▸ Options ▸ "Show the Developer tab" turns it on, as Excel's is. Today
//! it gathers the tools that already exist for that audience (the command palette, the action
//! journal, how agents connect), and the Development subsystem designed in
//! `docs/decisions-aniefork.md` (ADR-0001): the Development window, Modules and the Console.

use egui::{Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::SheetApp;
use crate::icons::Icon;
use crate::theme::Tokens;
use crate::widgets::big_button;

/// The tab's name, as `UiState::ribbon_tab` holds it.
pub const TAB: &str = "Developer";

/// Hook of the ribbon's tab strip: the Developer tab follows upstream's tabs when it is turned on.
pub fn tabs(app: &SheetApp, tabs: &mut Vec<&'static str>) {
    if app.ui.aniefork.developer_tab {
        tabs.push(TAB);
    }
}

/// The line between two groups of buttons, as the ribbon draws it.
fn separator(ui: &mut egui::Ui) {
    let t = Tokens::get(ui.ctx());
    let (r, _) = ui.allocate_exact_size(vec2(13.0, 72.0), Sense::hover());
    ui.painter().line_segment([pos2(r.center().x, r.top() + 6.0), pos2(r.center().x, r.bottom() - 6.0)], Stroke::new(1.0, t.separator));
}

/// Hook of the ribbon's content: the Developer tab's buttons.
pub fn ribbon(app: &mut SheetApp, ui: &mut egui::Ui) {
    if !app.ui.aniefork.developer_tab {
        // The tab was turned off (or a saved layout names it): there is nothing to show here.
        app.ui.ribbon_tab = "Home".into();
        return;
    }
    if big_button(ui, Icon::Script, "Command\nPalette", "Search and run any command", false).clicked() {
        app.open_dialog("commandSearch", json!({}));
    }
    if big_button(ui, Icon::Script, "Action\nJournal", "Every command run in this session (replayable)", false).clicked() {
        app.open_dialog("journal", json!({}));
    }
    if big_button(ui, Icon::Script, "Agent\nControl", "How agents drive GridCraft (MCP / control channel)", false).clicked() {
        app.open_dialog("agents", json!({}));
    }
    separator(ui);
    if big_button(ui, Icon::Settings, "Development", "Your development tools, and how far an agent may help you with them", false).clicked() {
        app.open_dialog(super::development::NAME, json!({}));
    }
    if big_button(ui, Icon::Folder, "Modules", "The modules you have installed, and whether you trust them", false).clicked() {
        app.open_dialog(super::modules::NAME, json!({}));
    }
    if big_button(ui, Icon::Script, "Console", "What your modules print and log, and what failed", false).clicked() {
        app.open_dialog(super::console::NAME, json!({}));
    }
    separator(ui);
    if big_button(ui, Icon::Settings, "Options", "Settings, and whether this tab is shown", false).clicked() {
        // Upstream's command: it opens the Options page, where the fork's settings are too.
        app.run_or_alert("file.options", json!({}));
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
    fn the_tab_is_hidden_until_turned_on_and_the_choice_is_saved() {
        let mut a = app();
        let mut shown = vec!["Home", "View"];
        tabs(&a, &mut shown);
        assert_eq!(shown, ["Home", "View"], "hidden by default");
        a.ui.aniefork.developer_tab = true;
        tabs(&a, &mut shown);
        assert_eq!(shown, ["Home", "View", TAB], "after upstream's tabs");
        // A `ui.json` from before the setting existed leaves it hidden; the choice is saved.
        assert!(!UiState::default().aniefork.developer_tab);
        assert!(!serde_json::from_str::<UiState>(r#"{"dark": true}"#).unwrap().aniefork.developer_tab);
        let saved = serde_json::to_value(&a.ui).unwrap();
        assert_eq!(saved["developerTab"], json!(true));
        assert!(serde_json::from_value::<UiState>(saved).unwrap().aniefork.developer_tab);
    }

    fn harness(app: SheetApp) -> egui_kittest::Harness<'static, SheetApp> {
        let mut h = egui_kittest::Harness::builder().with_size(vec2(1200.0, 800.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        h
    }

    #[test]
    fn the_tab_draws_in_every_language_and_theme() {
        let mut a = app();
        a.ui.aniefork.developer_tab = true;
        a.ui.ribbon_tab = TAB.into();
        let mut h = harness(a);
        for language in crate::i18n::Language::ALL {
            h.state_mut().ui.language = language;
            h.run_steps(2);
            assert_eq!(h.state().ui.ribbon_tab, TAB);
        }
        h.state_mut().ui.dark = true;
        h.run_steps(2);
    }

    #[test]
    fn a_layout_that_names_the_tab_while_it_is_off_falls_back_to_home() {
        let mut a = app();
        a.ui.ribbon_tab = TAB.into();
        let h = harness(a);
        assert_eq!(h.state().ui.ribbon_tab, "Home");
    }
}
