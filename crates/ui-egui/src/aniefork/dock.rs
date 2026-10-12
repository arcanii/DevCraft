//! A project's panel docked in the window: the same page a browser would be given, shown in
//! the system's web view beside the grid (ADR-0001, phase 2d).
//!
//! The page is still served by the bridge, with every check it makes for a browser: nothing
//! about what a panel may do changes when it is docked. What changes is where it is shown, and
//! who decides where the page may go: a browser lets a page send the user anywhere, and the
//! window's web view does not (see `Docked::network`).
//!
//! This module holds what the UI knows: which panel is docked, and where in the window its
//! place is. The web view itself belongs to the desktop shell, which has the window: once a
//! frame it asks [`wanted`] and puts the view there. A shell without a web view (the web
//! build, a system that has none) never says [`host_is_here`], and docking falls back to
//! saying so; the panel can always be opened in the browser instead.
//!
//! Docking a panel is opening it: the person's alone to do, as [`super::panels::open`] is.

use egui::{Rect, vec2};

use super::{panels, projects};
use crate::SheetApp;
use crate::theme::Tokens;

/// A panel docked in the window, as the shell's web view needs it.
#[derive(Clone, Debug, PartialEq)]
pub struct Docked {
    /// The project's id.
    pub project: String,
    /// Its name, for the pane's heading.
    pub name: String,
    /// The panel's address on the bridge. It holds the project's token: it goes to the web view,
    /// and is shown nowhere.
    pub address: String,
    /// The project was granted `network`. Without it the web view lets the page go nowhere but
    /// its own address, opens no other window and downloads nothing.
    pub network: bool,
    /// Where the view belongs, in the window's points. `None` until the pane has been drawn.
    pub rect: Option<Rect>,
    /// Something of the window's own is drawn over the pane's place (a dialog, a menu, the File
    /// view): the web view, which would hide it, is to be hidden instead.
    pub covered: bool,
}

fn id() -> egui::Id {
    egui::Id::new("aniefork.dock")
}

fn host_id() -> egui::Id {
    egui::Id::new("aniefork.dock.host")
}

/// The shell says it has a web view to show docked panels in. Called once, when it starts.
pub fn host_is_here(ctx: &egui::Context) {
    ctx.data_mut(|d| d.insert_temp(host_id(), true));
}

/// Where the shell's web view keeps what it keeps (its cache, its storage): a folder of its own
/// beside the settings, which the control channel can't reach. `None` in a run without settings.
pub fn web_view_folder() -> Option<std::path::PathBuf> {
    projects::places().map(|at| at.settings.with_file_name("webview"))
}

/// Whether this copy of GridCraft can show a panel inside its window.
pub fn available(ctx: &egui::Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(host_id())).unwrap_or(false)
}

/// The panel docked in the window, if one is.
pub fn docked(ctx: &egui::Context) -> Option<Docked> {
    ctx.data(|d| d.get_temp::<Docked>(id()))
}

/// What the shell's web view is to show, and where: the docked panel, once its place is known
/// and while nothing of the window's own covers it.
pub fn wanted(ctx: &egui::Context) -> Option<Docked> {
    docked(ctx).filter(|docked| docked.rect.is_some() && !docked.covered)
}

/// Docks the panel of the project `id` in the window. Refused unless the user trusts the project
/// as it is on disk now, and unless this copy of GridCraft has a web view to show it in.
pub fn dock(_app: &SheetApp, ctx: &egui::Context, id: &str) -> Result<String, String> {
    if !available(ctx) {
        return Err("This copy of GridCraft can't show a panel inside its window. Open it in your browser instead.".into());
    }
    let at = projects::places().ok_or(projects::UNAVAILABLE)?;
    let address = panels::serve(ctx, &at, id)?;
    let listed = projects::list();
    let found = listed["projects"].as_array().and_then(|all| all.iter().find(|m| m["id"] == id));
    let name = found.and_then(|m| m["name"].as_str()).unwrap_or(id).to_string();
    let network = found.and_then(|m| m["granted"].as_array()).is_some_and(|granted| granted.iter().any(|p| p == "network"));
    let docked = Docked { project: id.to_string(), name: name.clone(), address, network, rect: None, covered: false };
    ctx.data_mut(|d| d.insert_temp(self::id(), docked));
    ctx.request_repaint();
    Ok(format!("{name} is docked in the window."))
}

/// Takes the docked panel out of the window.
pub fn undock(ctx: &egui::Context) {
    ctx.data_mut(|d| d.remove::<Docked>(id()));
    ctx.request_repaint();
}

/// Part of the fork's work of every frame, before anything is drawn: whether something of the
/// window's own (a dialog, a message, a menu, the File view) is drawn where the docked pane is.
/// The web view would hide it, so the view is hidden instead. This is told here and not where
/// the pane is drawn, because the File view takes the whole window and draws no pane at all.
pub fn watch(app: &SheetApp, ctx: &egui::Context) {
    let Some(mut now) = docked(ctx) else { return };
    let covered = app.dialog.is_some() || app.message.is_some() || app.backstage.is_some() || egui::Popup::is_any_open(ctx);
    if now.covered != covered {
        now.covered = covered;
        ctx.data_mut(|d| d.insert_temp(id(), now));
    }
}

/// Hook of the window's layout, before the grid takes what is left: the pane of the docked
/// panel, on the right. It draws the pane's heading and keeps its body clear for the web view.
pub fn show(app: &mut SheetApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let Some(mut now) = docked(&ctx) else { return };
    // A project that is no longer served (its trust was taken back) has no panel to show.
    if !panels::serving(&ctx).contains(&now.project) {
        undock(&ctx);
        return;
    }
    let t = Tokens::get(&ctx);
    let mut close = false;
    let mut in_browser = false;
    let mut place = None;
    egui::Panel::right("aniefork.dock").default_size(400.0).size_range(260.0..=900.0).resizable(true).frame(egui::Frame::NONE.fill(t.ribbon)).show(
        ui,
        |ui| {
            egui::Frame::NONE.inner_margin(egui::Margin::symmetric(10, 6)).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&now.name).font(crate::theme::ui_bold(14.0)));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if crate::widgets::icon_button(ui, crate::icons::Icon::Close, t.text_dim, "Close the panel", vec2(22.0, 22.0)).clicked() {
                            close = true;
                        }
                        if app.services.open_url.is_some()
                            && ui.small_button("Open in Browser").on_hover_text("Open this panel in your browser instead").clicked()
                        {
                            in_browser = true;
                        }
                    });
                });
            });
            // The rest is the web view's. Until it has drawn (and where there is none), say so.
            let body = ui.available_rect_before_wrap();
            ui.painter().rect_filled(body, 0.0, t.ribbon);
            ui.painter().text(body.center(), egui::Align2::CENTER_CENTER, "The panel is shown here.", crate::theme::ui_font(12.5), t.text_dim);
            ui.allocate_rect(body, egui::Sense::hover());
            place = Some(body);
        },
    );
    if close {
        undock(&ctx);
        return;
    }
    if in_browser {
        // Opening a panel is the person's alone to do: the same rule as in Projects.
        let said = if super::presence::driven(&ctx) { Err(super::presence::BY_HAND.to_string()) } else { panels::open(app, &ctx, &now.project) };
        match said {
            Ok(said) => app.toast = Some((said, crate::now_ms())),
            Err(why) => app.message = Some(("Development".into(), why)),
        }
    }
    // Where the web view belongs. (Whether it is covered is [`watch`]'s to say.)
    if now.rect != place {
        let covered = docked(&ctx).is_some_and(|latest| latest.covered);
        now.rect = place;
        now.covered = covered;
        ctx.data_mut(|d| d.insert_temp(id(), now));
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use serde_json::json;

    use super::*;
    use crate::aniefork::projects::tests::{Sandbox, app_with, drive, yes};
    use crate::aniefork::projects::{create, list, trust};

    type Harness = egui_kittest::Harness<'static, SheetApp>;

    fn harness(app: SheetApp) -> Harness {
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_ui_state(
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

    /// An app with a trusted starter project, and what its `open_url` was handed.
    fn trusted() -> (SheetApp, Rc<RefCell<Vec<String>>>) {
        create("Clean Data").unwrap();
        let opened = Rc::new(RefCell::new(Vec::<String>::new()));
        let seen = opened.clone();
        let mut app = app_with(Some(&yes()));
        app.services.open_url = Some(Box::new(move |address: &str| seen.borrow_mut().push(address.to_string())));
        trust(&app, "clean-data", list()["projects"][0]["fingerprint"].as_str().unwrap(), false).unwrap();
        (app, opened)
    }

    #[test]
    fn a_panel_docks_only_where_there_is_a_web_view_and_only_for_a_trusted_project() {
        let _s = Sandbox::new("dock-where");
        let (app, opened) = trusted();
        let mut h = harness(app);
        let ctx = h.ctx.clone();
        // A shell without a web view: docking says so, and nothing is served.
        assert!(!available(&ctx));
        let refused = dock(h.state(), &ctx, "clean-data").unwrap_err();
        assert!(refused.contains("can't show a panel inside its window"), "{refused}");
        assert!(docked(&ctx).is_none() && panels::serving(&ctx).is_empty());
        // With one: the trusted project's panel is served, and has a place in the window.
        host_is_here(&ctx);
        assert_eq!(dock(h.state(), &ctx, "clean-data").unwrap(), "Clean Data is docked in the window.");
        assert_eq!(panels::serving(&ctx), ["clean-data"]);
        assert!(wanted(&ctx).is_none(), "its place isn't known until the pane has been drawn");
        h.run_steps(3);
        let now = wanted(&ctx).unwrap();
        assert_eq!((now.project.as_str(), now.name.as_str(), now.network), ("clean-data", "Clean Data", false));
        assert!(now.address.starts_with("http://127.0.0.1:") && now.address.ends_with('/'));
        let place = now.rect.unwrap();
        let window = ctx.content_rect();
        assert!(place.width() > 250.0 && place.height() > 300.0, "{place:?}");
        assert!(place.right() <= window.right() + 0.5 && place.left() > window.center().x, "on the right: {place:?}");
        // The address holds the project's token: it is for the web view, and shown nowhere.
        use egui_kittest::kittest::Queryable;
        h.get_by_label("Clean Data");
        assert!(h.query_by_label_contains("127.0.0.1").is_none());
        assert!(opened.borrow().is_empty(), "nothing was sent to the browser");
        // A project that isn't trusted, or isn't there, is not docked in its place.
        create("Other").unwrap();
        assert!(dock(h.state(), &ctx, "other").unwrap_err().contains("not trusted"));
        assert!(dock(h.state(), &ctx, "nowhere").is_err());
        assert_eq!(docked(&ctx).unwrap().project, "clean-data");
    }

    #[test]
    fn the_web_view_gives_way_to_what_the_window_draws_over_it_and_goes_with_the_projects_trust() {
        let _s = Sandbox::new("dock-covered");
        let (app, _) = trusted();
        let mut h = harness(app);
        let ctx = h.ctx.clone();
        host_is_here(&ctx);
        dock(h.state(), &ctx, "clean-data").unwrap();
        h.run_steps(3);
        assert!(wanted(&ctx).is_some());
        // A dialog, a message, the File view: the web view would hide them, so it is hidden.
        h.state_mut().open_dialog(projects::NAME, json!({}));
        h.run_steps(3);
        assert!(wanted(&ctx).is_none() && docked(&ctx).is_some_and(|now| now.covered));
        h.state_mut().dialog = None;
        h.run_steps(3);
        assert!(wanted(&ctx).is_some());
        h.state_mut().message = Some(("Something".into(), "to read".into()));
        h.run_steps(3);
        assert!(wanted(&ctx).is_none());
        h.state_mut().message = None;
        h.run_steps(3);
        assert!(wanted(&ctx).is_some());
        // The File view takes the whole window: there is no pane, and no place for the view.
        h.state_mut().run("ui.backstage", json!({})).unwrap();
        h.run_steps(3);
        assert!(wanted(&ctx).is_none());
        h.state_mut().backstage = None;
        h.run_steps(3);
        assert!(wanted(&ctx).is_some());
        // The project's trust is taken back: its panel stops being served, and the pane goes.
        panels::stop(&ctx, "clean-data");
        h.run_steps(3);
        assert!(docked(&ctx).is_none());
    }

    #[test]
    fn the_pane_can_be_closed_and_its_panel_sent_to_the_browser_by_the_person() {
        use egui_kittest::kittest::Queryable;
        let _s = Sandbox::new("dock-pane");
        let (app, opened) = trusted();
        let mut h = harness(app);
        let ctx = h.ctx.clone();
        host_is_here(&ctx);
        dock(h.state(), &ctx, "clean-data").unwrap();
        h.run_steps(3);
        h.get_by_label("Open in Browser").click();
        h.run_steps(3);
        assert_eq!(opened.borrow().len(), 1);
        assert_eq!(opened.borrow()[0], docked(&ctx).unwrap().address, "the same panel, on the same bridge");
        // Not by the control channel, which can't open a panel anywhere.
        drive(&mut h);
        h.get_by_label("Open in Browser").click();
        h.run_steps(3);
        assert_eq!(opened.borrow().len(), 1);
        assert_eq!(h.state_mut().message.take().map(|(_, text)| text).as_deref(), Some(crate::aniefork::presence::BY_HAND));
        // Closing the pane takes the view out of the window; the project is still served.
        undock(&ctx);
        h.run_steps(3);
        assert!(docked(&ctx).is_none() && wanted(&ctx).is_none());
        assert!(h.query_by_label("Open in Browser").is_none());
    }
}
