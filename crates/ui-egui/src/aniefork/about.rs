//! What the fork says of itself: in Help ▸ About, and in the Help menu.
//!
//! Upstream's brand licence (`docs/brand/LICENSE-brand.txt`) asks of a published, modified
//! version of GridCraft that it carry none of the ArtCraft marks and not present itself as an
//! ArtCraft product; it may say, in plain text, that it is based on GridCraft by the ArtCraft
//! team. This fork is such a version (the owner, 2026-10-14: "we need to do it anyways from
//! their rules. We reference upstream of course."). So here it says what it is and where it
//! comes from, names upstream in plain text with the address of its repository, and sends
//! nobody to upstream's own site or community for a program that is not upstream's.

/// What the program is, said under its name and version.
pub const SAYS: &str = "A clean-room, open-source, Rust-native spreadsheet.";
/// That this is a fork, and what it is for.
pub const FORK: &str = "This is DevCraft, a fork of GridCraft with development kits for the spreadsheet.";
/// Where it comes from, in plain text.
pub const BASED_ON: &str = "Based on GridCraft by the ArtCraft team.";
/// The fork's own page, and upstream's repository: what is shown, and the address.
pub const FORK_LINK: (&str, &str) = ("github.com/arcanii/DevCraft", "https://github.com/arcanii/DevCraft");
pub const UPSTREAM_LINK: (&str, &str) = ("github.com/storytold/gridcraft", "https://github.com/storytold/gridcraft");

/// The Help menu's two links on macOS (`native_menu.rs`), each a label and what choosing it
/// does, as that menu writes its items.
pub const HELP_MENU: [(&str, &str); 2] =
    [("DevCraft on GitHub", "url:https://github.com/arcanii/DevCraft"), ("GridCraft on GitHub", "url:https://github.com/storytold/gridcraft")];

/// Which DevCraft this is: its own version, numbered apart from GridCraft's above it.
pub fn devcraft() -> String {
    format!("DevCraft {}", gridcraft_aniefork_dev::DEVCRAFT_VERSION)
}

/// Hook of the About window (`dialogs::about`): what stands under the name and the version.
pub fn credits(ui: &mut egui::Ui) {
    ui.label(devcraft());
    ui.label(SAYS);
    ui.label(FORK);
    ui.hyperlink_to(FORK_LINK.0, FORK_LINK.1);
    ui.add_space(4.0);
    ui.label(BASED_ON);
    ui.hyperlink_to(UPSTREAM_LINK.0, UPSTREAM_LINK.1);
}

#[cfg(test)]
mod tests {
    use egui_kittest::kittest::Queryable as _;
    use serde_json::json;

    use super::*;
    use crate::SheetApp;

    #[test]
    fn the_fork_names_upstream_in_plain_text_and_carries_none_of_its_marks() {
        // Upstream is named once, as the licence allows: whose work the fork is based on.
        assert_eq!(BASED_ON, "Based on GridCraft by the ArtCraft team.");
        for said in [SAYS, FORK, FORK_LINK.0, FORK_LINK.1, UPSTREAM_LINK.0, UPSTREAM_LINK.1] {
            assert!(!said.to_lowercase().contains("artcraft"), "{said}");
        }
        // Every address is a repository's, the fork's or upstream's: not upstream's site or community.
        for (label, does) in HELP_MENU {
            assert!(!label.to_lowercase().contains("artcraft") && does.starts_with("url:https://github.com/"), "{label}: {does}");
        }
        assert!(FORK_LINK.1.starts_with("https://github.com/arcanii/") && UPSTREAM_LINK.1.starts_with("https://github.com/storytold/"));
        // The menu's items are written as that menu writes them: a label, what it does, and no shortcut.
        let items: Vec<(&'static str, &'static str, Option<&'static str>)> = HELP_MENU.iter().map(|(label, does)| (*label, *does, None)).collect();
        assert_eq!(items.len(), 2);
    }

    #[test]
    fn the_about_window_says_what_the_fork_is_and_where_it_comes_from() {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            SheetApp::new(session, Default::default()),
        );
        h.run_steps(4);
        h.state_mut().open_dialog("about", json!({}));
        h.run_steps(4);
        for said in [SAYS, FORK, BASED_ON, FORK_LINK.0, UPSTREAM_LINK.0] {
            assert!(h.query_by_label(said).is_some(), "{said}");
        }
        // DevCraft has a version of its own, which is not GridCraft's.
        assert_eq!(devcraft(), "DevCraft 0.1.0");
        assert!(h.query_by_label(&devcraft()).is_some() && gridcraft_aniefork_dev::DEVCRAFT_VERSION != env!("CARGO_PKG_VERSION"));
        // What upstream's own build says there is not said by the fork's.
        for gone in
            ["A clean-room, open-source, Rust-native spreadsheet. Part of ArtCraft.", "getartcraft.com/apps/gridcraft", "Join the ArtCraft Discord"]
        {
            assert!(h.query_by_label(gone).is_none(), "{gone}");
        }
    }
}
