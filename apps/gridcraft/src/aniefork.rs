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
//! The page runs in the web view's own processes, not GridCraft's. Every two seconds the page
//! is asked a question; when it has not answered for ten, or its web view has gone, the UI is
//! told ([`dock::answering`]), and says so over the panel with a way to load it again.
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
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    use gridcraft_ui_egui::SheetApp;
    use gridcraft_ui_egui::aniefork::dock::{self, Answer, Docked};
    use wry::dpi::{PhysicalPosition, PhysicalSize};

    /// The view on screen, and what it was made for.
    struct Shown {
        address: String,
        /// The reload it was made for: a new one is a new view.
        reloads: u32,
        view: wry::WebView,
        bounds: [i32; 4],
        visible: bool,
        watch: Watch,
    }

    /// How often the page is asked whether it answers, and how long it may take before it is
    /// said not to (milliseconds). A page busy for longer than that is said to be silent too,
    /// and its view stays: it may answer yet.
    const ASK_EVERY: f64 = 2_000.0;
    const SILENT_AFTER: f64 = 10_000.0;
    /// The question, and the one answer that counts: a page that crashed answers nothing, or
    /// nothing like this.
    const QUESTION: &str = "'answering'";
    const ANSWER: &str = "\"answering\"";

    /// Whether the page answers: one question at a time, asked every [`ASK_EVERY`].
    struct Watch {
        /// When the question not yet answered was asked, while one is.
        out: Option<f64>,
        /// When the last question was asked.
        asked: Option<f64>,
        /// Set by the answer, when it comes (on whatever thread the web view calls back on).
        heard: Arc<AtomicBool>,
    }

    impl Watch {
        fn new() -> Watch {
            Watch { out: None, asked: None, heard: Arc::new(AtomicBool::new(false)) }
        }

        /// Whether a question is to be asked now: none is out, and the last was long enough ago.
        fn due(&mut self, now: f64) -> bool {
            if self.heard.swap(false, Ordering::SeqCst) {
                self.out = None;
            }
            self.out.is_none() && self.asked.is_none_or(|asked| now - asked >= ASK_EVERY)
        }

        fn asking(&mut self, now: f64) {
            self.out = Some(now);
            self.asked = Some(now);
        }

        /// Whether the page answers, as far as can be said now.
        fn answer(&self, now: f64) -> Answer {
            if self.out.is_some_and(|since| now - since >= SILENT_AFTER) { Answer::Silent } else { Answer::Yes }
        }
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
            // Another panel than the one shown, or the same loaded again: its view is made anew.
            if self.shown.as_ref().is_some_and(|shown| shown.address != docked.address || shown.reloads != docked.reloads) {
                self.shown = None;
            }
            // Whether its page answers.
            if let Some(shown) = self.shown.as_mut() {
                let now = gridcraft_ui_egui::now_ms();
                if shown.watch.due(now) {
                    let heard = shown.watch.heard.clone();
                    let asked = shown.view.evaluate_script_with_callback(QUESTION, move |said| {
                        if said == ANSWER {
                            heard.store(true, Ordering::SeqCst);
                        }
                    });
                    if asked.is_err() {
                        // What showed it is gone: there is nothing to ask, and nothing to show.
                        log::warn!("the docked panel's web view is gone");
                        self.shown = None;
                        dock::answering(ctx, Answer::Gone);
                        return;
                    }
                    shown.watch.asking(now);
                }
                dock::answering(ctx, shown.watch.answer(now));
                ctx.request_repaint_after(std::time::Duration::from_millis(ASK_EVERY as u64));
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
                    Ok(view) => {
                        self.shown = Some(Shown {
                            address: docked.address.clone(),
                            reloads: docked.reloads,
                            view,
                            bounds: place,
                            visible: true,
                            watch: Watch::new(),
                        })
                    }
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
        fn the_page_is_asked_one_question_at_a_time_and_is_silent_when_it_goes_unanswered() {
            let mut watch = Watch::new();
            // Asked at once, and not again while the question is out.
            assert!(watch.due(1_000.0));
            watch.asking(1_000.0);
            assert!(!watch.due(5_000.0) && watch.answer(5_000.0) == Answer::Yes);
            // Ten seconds without an answer: silent.
            assert_eq!(watch.answer(10_999.0), Answer::Yes);
            assert_eq!(watch.answer(11_000.0), Answer::Silent);
            // It answers, late: it answers again, and is asked again.
            watch.heard.store(true, Ordering::SeqCst);
            assert!(watch.due(12_000.0));
            assert_eq!(watch.answer(12_000.0), Answer::Yes);
            watch.asking(12_000.0);
            watch.heard.store(true, Ordering::SeqCst);
            // Not sooner than two seconds after the last question.
            assert!(!watch.due(13_000.0) && watch.due(14_000.0));
            // Only the one answer counts: the question and the answer are what a page says of a string.
            assert_eq!((QUESTION, ANSWER), ("'answering'", "\"answering\""));
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
                answer: Answer::Yes,
                reloads: 0,
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
