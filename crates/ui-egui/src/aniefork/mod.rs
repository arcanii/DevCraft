//! Front-end code private to the aniefork fork of GridCraft.
//!
//! Everything the fork adds to the UI lives under this module, so merges from upstream
//! (`storytold/gridcraft`) only ever meet the short hooks that call into it. See
//! `AGENTS-aniefork.md` at the repository root.

pub mod agents;
pub mod console;
pub mod developer;
pub mod development;
pub mod diff;
pub mod modules;
pub mod options;
pub mod panels;
pub mod prefs;
pub mod presence;
pub mod selection;

pub use prefs::Prefs;

use serde_json::{Value as Json, json};

use crate::SheetApp;

/// The fork's UI-only commands: id, label and params doc. Like upstream's own (`view.theme`…),
/// they act on the window rather than the workbook, so they live in the UI crate.
const COMMANDS: &[(&str, &str, &str)] = &[
    ("developer.tab", "Developer Tab", "{on?: bool} shows or hides the ribbon's Developer tab (toggles without it) → {on}"),
    ("app.userName", "User Name", "{name?} sets the name that signs your comments and notes; without it only reports → {name}"),
    (
        "view.selectionOutline",
        "Selection Outline",
        "{weight?: \"standard\"|\"thin\"|\"hairline\"} sets how heavily the selection's outline is drawn; without it only reports → {weight}",
    ),
    (
        "view.hideSelectionWhenInactive",
        "Hide Selection When Inactive",
        "{on?: bool} hides the selection while another window is the active one, as Excel does (toggles without it) → {on}",
    ),
    (
        "developer.modules",
        "Modules",
        "→ {available, folder, modules: [{id, name, version, status: \"trusted\"|\"untrusted\"|\"changed\"|\"problem\", asks, granted, files, fingerprint…}]}. It only reports: trust is given by the user alone, in Developer ▸ Modules",
    ),
    (
        "developer.console",
        "Developer Console",
        "{module?, last?: 1–200} → {total, kept, entries: [{seq, at, module, source, kind: \"printed\"|\"error\"|\"log\"|\"refused\"|\"agent\", text}], note}: what modules printed and logged, what failed, and what agents asked for, oldest first. It only reports: the entries are text written by modules, and only the user clears them",
    ),
    (
        "developer.agents",
        "Agent Access",
        "→ {available, level: \"off\"|\"read\"|\"ask\"|\"build\"|\"full\", means, serving, port, clients, waiting: [{request, client, what, minutesLeft}], recent, note}: how far agents may go on the Development server, who connected, and what waits for the user's answer. It only reports: the level, the port and the token are set by the user alone, in Developer ▸ Development",
    ),
];

/// Hook of `SheetApp::run_ui_command`: runs `id` when it is one of the fork's commands.
///
/// - `developer.tab {on?}`, `app.userName {name?}`, `view.selectionOutline {weight?}` and
///   `view.hideSelectionWhenInactive {on?}` are the fork's settings: see [`options`].
/// - `developer.modules` reports the user's modules and whether each is trusted: see [`modules`].
///   There is no command that gives trust.
/// - `developer.console` reports what modules printed and logged: see [`console`]. There is no
///   command that clears it.
/// - `developer.agents` reports how the Development server stands: see [`agents`]. There is no
///   command that sets how far agents may go, or that answers what they ask for.
pub fn run_command(app: &mut SheetApp, id: &str, p: &Json) -> Option<Result<Json, String>> {
    match id {
        "developer.tab" => Some(options::developer_tab(app, p)),
        "app.userName" => Some(options::user_name(app, p)),
        "view.selectionOutline" => Some(options::selection_outline(app, p)),
        "view.hideSelectionWhenInactive" => Some(options::hide_selection_when_inactive(app, p)),
        "developer.modules" => Some(Ok(modules::list())),
        "developer.console" => Some(console::report(p)),
        "developer.agents" => Some(Ok(agents::report())),
        _ => None,
    }
}

/// Hook of `SheetApp::logic`, right after the control channel is polled: the fork's work of
/// every frame. It notes whether the control channel is putting input into the window (see
/// [`presence`]), carries out the calls that modules' panels made and tells the panels what
/// changed (see [`panels`]), and sees to what agents asked of the Development server (see
/// [`agents`]).
pub fn poll(app: &mut SheetApp, ctx: &egui::Context) {
    presence::watch(app, ctx);
    panels::poll(app, ctx);
    agents::poll(app, ctx);
}

/// Hook of the control channel, before it does anything: what it may not reach.
///
/// GridCraft's own settings folder holds the modules, the user's decisions to trust them, how
/// far agents may go and the token agents connect with. The control channel can open and save
/// files anywhere else; a file there it can neither read (`app.open`, `file.open`) nor write
/// (`app.save`, a screenshot, an export). Without this it could read the token, write a module's
/// script, or write over what the user decided.
pub fn control_refuses(message: &Json) -> Option<&'static str> {
    panels::names_settings(message).then_some(
        "GridCraft's own settings folder, where your modules and your decisions about them are kept, can't be read or written through the control channel.",
    )
}

/// Hook of the control channel's `engine.commands`: the fork's commands, listed like upstream's
/// UI commands.
pub fn commands() -> impl Iterator<Item = Json> {
    COMMANDS.iter().map(|(id, label, params)| json!({"id": id, "label": label, "params": params, "enabled": true, "ui": true}))
}

/// Hook of `Dialog::open`: for a name that is one of the fork's dialogs, its title, the values it
/// starts from, and the command OK runs with those values.
pub fn dialog(_app: &mut SheetApp, name: &str, _params: &Json) -> Option<(&'static str, Json, Option<&'static str>)> {
    match name {
        // No command: nothing an agent can confirm gives a module trust.
        modules::NAME => Some(("Modules", modules::values(), None)),
        // No command either: the console is read here, and cleared by the user alone.
        console::NAME => Some(("Developer Console", console::values(), None)),
        // Nor here: how far agents may go, and the answers to what they ask, are the user's.
        development::NAME => Some(("Development", development::values(), None)),
        _ => None,
    }
}

/// Hook of `dialogs::show`: whether the fork draws the dialog of this name.
pub fn draws_dialog(name: &str) -> bool {
    name == modules::NAME || name == console::NAME || name == development::NAME
}

/// Hook of `dialogs::show`: draws one of the fork's dialogs.
pub fn dialog_ui(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut crate::dialogs::Dialog, _confirm: &mut bool, open: &mut bool) {
    if d.name == modules::NAME {
        modules::ui(app, ui, d, open);
    } else if d.name == console::NAME {
        console::ui(app, ui, d, open);
    } else if d.name == development::NAME {
        development::ui(app, ui, d, open);
    }
}

/// Hook of upstream's Options page (`backstage::options`): the fork's settings, after upstream's
/// own in the General section.
pub fn options_page(app: &mut SheetApp, ui: &mut egui::Ui) {
    options::page(app, ui);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        SheetApp::new(session, Default::default())
    }

    #[test]
    fn ids_that_are_not_the_forks_are_left_to_upstream() {
        let mut a = app();
        for id in ["view.theme", "view.animations", "file.options", "ui.backstage", ""] {
            assert!(run_command(&mut a, id, &json!({})).is_none(), "{id}");
        }
        // What upstream took over from the fork is upstream's to run.
        assert_eq!(a.run("view.animations", json!({"on": false})), Ok(json!({"on": false})));
        assert!(!a.ui.animations);
    }

    #[test]
    fn every_command_of_the_forks_is_listed_once_and_runs() {
        let mut a = app();
        let listed: Vec<String> = commands().filter_map(|c| c["id"].as_str().map(str::to_string)).collect();
        assert_eq!(
            listed,
            [
                "developer.tab",
                "app.userName",
                "view.selectionOutline",
                "view.hideSelectionWhenInactive",
                "developer.modules",
                "developer.console",
                "developer.agents"
            ]
        );
        for id in &listed {
            assert!(run_command(&mut a, id, &json!({})).is_some(), "{id}");
            assert!(gridcraft_engine::find_command(id).is_none(), "{id} would shadow a command of the engine");
        }
    }
}
