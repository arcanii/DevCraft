//! The fork's part of the desktop shell: the system web view (ADR-0001 in
//! `docs/decisions-aniefork.md`, phase 2d). It shows two things: a project's panel docked in
//! the window (this file), and the Development window as a page in a window of its own
//! ([`window`]).
//!
//! The UI says which panel is docked and where its place is
//! (`gridcraft_ui_egui::aniefork::dock`); this file has the window, and so the web view. Once a
//! frame it puts the view where the UI wants it, hides it when something of the window's own
//! is drawn there, and lets it go when the panel is closed.
//!
//! The page is the one a browser would be given, from the same bridge with the same checks.
//! What the window adds is a say over where the page may go, which a browser doesn't give: a
//! project that wasn't granted `network` is kept to its own address, opens no other window and
//! downloads nothing.
//!
//! Windows only so far (WebView2, through `wry`). Elsewhere there is no host: the UI is never
//! told there is one, and panels open in the browser as before.

#[cfg(target_os = "windows")]
mod icon;
#[cfg(target_os = "windows")]
mod page;
#[cfg(target_os = "windows")]
mod window;

/// Hook of the shell's `ui`, after the app has drawn: the docked panel's web view, and the
/// Development window when it is shown as a page.
#[cfg(not(target_os = "windows"))]
pub fn frame(_app: &mut gridcraft_ui_egui::SheetApp, _ctx: &egui::Context, _window: &eframe::Frame) {}

/// Hook of the shell's `ui`, after the app has drawn: the docked panel's web view, and the
/// Development window when it is shown as a page.
#[cfg(target_os = "windows")]
pub fn frame(app: &mut gridcraft_ui_egui::SheetApp, ctx: &egui::Context, window: &eframe::Frame) {
    host::frame(app, ctx, window);
    self::window::keep_up(app, ctx);
}

#[cfg(target_os = "windows")]
mod host {
    use std::cell::RefCell;

    use gridcraft_ui_egui::SheetApp;
    use gridcraft_ui_egui::aniefork::dock::{self, Docked};
    use wry::dpi::{PhysicalPosition, PhysicalSize};

    /// The view on screen, and what it was made for.
    struct Shown {
        address: String,
        view: wry::WebView,
        bounds: [i32; 4],
        visible: bool,
    }

    struct Host {
        /// Where the web view keeps what it keeps (its cache, its storage): a folder of its own.
        context: wry::WebContext,
        shown: Option<Shown>,
        /// A project to dock as the app starts (`GRIDCRAFT_DOCK`), for whoever launches it.
        at_start: Option<String>,
    }

    thread_local! {
        /// The window's web view host. It belongs to the thread that draws the window.
        static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
    }

    /// Hook of the shell's `ui`, after the app has drawn: the docked panel's web view.
    pub fn frame(app: &mut SheetApp, ctx: &egui::Context, window: &eframe::Frame) {
        HOST.with(|host| {
            let mut host = host.borrow_mut();
            let host = host.get_or_insert_with(|| {
                dock::host_is_here(ctx);
                let at_start = std::env::var("GRIDCRAFT_DOCK").ok().filter(|id| !id.is_empty());
                Host { context: wry::WebContext::new(dock::web_view_folder()), shown: None, at_start }
            });
            if let Some(id) = host.at_start.take()
                && let Err(why) = dock::dock(app, ctx, &id)
            {
                log::warn!("GRIDCRAFT_DOCK={id}: {why}");
            }
            host.keep_up(app, ctx, window);
        });
    }

    /// The view's place in the window's own pixels: left, top, width, height.
    fn bounds(docked: &Docked, pixels_per_point: f32) -> Option<[i32; 4]> {
        let place = docked.rect?;
        let px = |points: f32| (points * pixels_per_point).round() as i32;
        let (left, top) = (px(place.left()), px(place.top()));
        let (width, height) = (px(place.right()) - left, px(place.bottom()) - top);
        (width > 0 && height > 0).then_some([left, top, width, height])
    }

    fn rect([left, top, width, height]: [i32; 4]) -> wry::Rect {
        wry::Rect { position: PhysicalPosition::new(left, top).into(), size: PhysicalSize::new(width.max(1) as u32, height.max(1) as u32).into() }
    }

    /// The start of every address on the panel's own bridge: `http://127.0.0.1:<port>/`.
    fn own_addresses(address: &str) -> String {
        let after_scheme = address.find("://").map_or(0, |at| at + 3);
        let end = address.get(after_scheme..).and_then(|rest| rest.find('/')).map_or(address.len(), |at| after_scheme + at);
        format!("{}/", address.get(..end).unwrap_or(address))
    }

    impl Host {
        fn keep_up(&mut self, app: &mut SheetApp, ctx: &egui::Context, window: &eframe::Frame) {
            let Some(docked) = dock::docked(ctx) else {
                // The panel was closed: the view goes with it.
                self.shown = None;
                return;
            };
            let place = dock::wanted(ctx).and_then(|wanted| bounds(&wanted, ctx.pixels_per_point()));
            // Another panel than the one shown: its view is made anew, with its own limits.
            if self.shown.as_ref().is_some_and(|shown| shown.address != docked.address) {
                self.shown = None;
            }
            let Some(place) = place else {
                // Covered, or its place not yet known: hidden, not gone.
                if let Some(shown) = self.shown.as_mut().filter(|shown| shown.visible) {
                    let _ = shown.view.set_visible(false);
                    shown.visible = false;
                }
                return;
            };
            match &mut self.shown {
                Some(shown) => {
                    if shown.bounds != place {
                        let _ = shown.view.set_bounds(rect(place));
                        shown.bounds = place;
                    }
                    if !shown.visible {
                        let _ = shown.view.set_visible(true);
                        shown.visible = true;
                    }
                }
                None => match self.make(&docked, place, window) {
                    Ok(view) => self.shown = Some(Shown { address: docked.address.clone(), view, bounds: place, visible: true }),
                    Err(why) => {
                        dock::undock(ctx);
                        app.message = Some((
                            "Development".into(),
                            format!("{} could not be shown in the window ({why}). Open its panel in your browser instead.", docked.name),
                        ));
                    }
                },
            }
        }

        fn make(&mut self, docked: &Docked, place: [i32; 4], window: &eframe::Frame) -> Result<wry::WebView, String> {
            let (own, anywhere) = (own_addresses(&docked.address), docked.network);
            wry::WebViewBuilder::new_with_web_context(&mut self.context)
                .with_url(&docked.address)
                .with_bounds(rect(place))
                .with_focused(false)
                .with_devtools(false)
                // Where the page may go is the window's to say. Without `network`: its own
                // bridge and nowhere else.
                .with_navigation_handler(move |to| anywhere || to.starts_with(&own) || to == "about:blank")
                .with_new_window_req_handler(|_, _| wry::NewWindowResponse::Deny)
                .with_download_started_handler(|_, _| false)
                .build_as_child(window)
                .map_err(|e| e.to_string())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn a_page_without_network_is_kept_to_its_own_bridge() {
            let own = own_addresses("http://127.0.0.1:50123/0123abcd/");
            assert_eq!(own, "http://127.0.0.1:50123/");
            for inside in ["http://127.0.0.1:50123/0123abcd/", "http://127.0.0.1:50123/0123abcd/app.js", "http://127.0.0.1:50123/"] {
                assert!(inside.starts_with(&own), "{inside}");
            }
            // Not a name that only begins like it, another port, another scheme, or anywhere else.
            for outside in [
                "http://127.0.0.1:50123.evil.example/",
                "http://127.0.0.1:5012/",
                "http://127.0.0.1:501234/",
                "https://127.0.0.1:50123/",
                "http://evil.example/?http://127.0.0.1:50123/",
                "file:///C:/Users/",
            ] {
                assert!(!outside.starts_with(&own), "{outside}");
            }
            assert_eq!(own_addresses("http://127.0.0.1:50123"), "http://127.0.0.1:50123/");
        }

        #[test]
        fn the_views_place_is_the_panes_in_the_windows_own_pixels() {
            let pane = |rect| Docked {
                project: "m".into(),
                name: "M".into(),
                address: "http://127.0.0.1:1/t/".into(),
                network: false,
                rect,
                covered: false,
            };
            let place = egui::Rect::from_min_max(egui::pos2(800.0, 120.5), egui::pos2(1200.0, 700.0));
            assert_eq!(bounds(&pane(Some(place)), 1.0), Some([800, 121, 400, 579]));
            assert_eq!(bounds(&pane(Some(place)), 1.5), Some([1200, 181, 600, 869]));
            assert_eq!(bounds(&pane(None), 1.5), None, "a pane not yet drawn has no place");
            let empty = egui::Rect::from_min_max(egui::pos2(800.0, 120.0), egui::pos2(800.0, 700.0));
            assert_eq!(bounds(&pane(Some(empty)), 1.0), None, "nor has one with no room");
        }
    }
}
