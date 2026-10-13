//! The Development window as a page, in a window of its own (ADR-0001, phase 2d).
//!
//! The UI says what the window shows and takes what is clicked in it
//! (`gridcraft_ui_egui::aniefork::devpage`). This file has the window: a second window of the
//! app, with the system's web view in it (WebView2, through `wry`), showing a page written in
//! Svelte that is compiled into the app ([`super::page`]).
//!
//! The window has a thread of its own, with its own message loop (`tao`): the app's main loop
//! belongs to its first window. The two talk through channels. Once a frame the UI thread
//! takes what the page said, and tells it what to show when that has changed.
//!
//! **What the UI relies on, kept here.** A message from the page is taken as the click of the
//! person at the screen. So:
//!
//! - the page is served from the app's own files, by name, and nothing else is served;
//! - its address ends in `.invalid`, which no network resolves: were the page ever not served
//!   from here, nothing would load in its place;
//! - every answer carries a policy that lets the page run its own script and use its own
//!   stylesheet, and nothing else: no script written into the page, nothing from anywhere;
//! - the page can go to no other address, open no window and download nothing;
//! - a message is taken only when it comes from the page's own address;
//! - the developer tools, the browser's own keys and its menu are off.

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::time::Duration;

use gridcraft_ui_egui::SheetApp;
use gridcraft_ui_egui::aniefork::devpage;
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy};
use tao::platform::run_return::EventLoopExtRunReturn;
use tao::platform::windows::{EventLoopBuilderExtWindows, WindowBuilderExtWindows};
use tao::window::WindowBuilder;
use wry::WebViewBuilderExtWindows;
use wry::http::{Request, Response, header};

use super::page;

/// The scheme the page is served under. The web view shows it as `https://gridcraft.<host>`.
const SCHEME: &str = "gridcraft";
/// The page's address, as it is asked for and as the web view names it.
const START: &str = "gridcraft://development.invalid/index.html";
const OWN: [&str; 2] = ["https://gridcraft.development.invalid/", "gridcraft://development.invalid/"];
/// What the page may do: run its own script, use its own stylesheet. Nothing else.
const POLICY: &str =
    "default-src 'none'; script-src 'self'; style-src 'self'; img-src 'self' data:; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";
const TITLE: &str = "Development - GridCraft";

/// What the UI thread tells the window's thread.
enum Tell {
    /// What the page is to show, as JSON.
    Show(String),
    /// Come to the front.
    Raise,
    Close,
}

/// What the window's thread tells the UI thread.
enum Told {
    /// A message from the page.
    Said(String),
    /// The window was closed.
    Closed,
    /// The window, or its web view, could not be made.
    Failed(String),
}

/// The window that is open.
struct Open {
    to: EventLoopProxy<Tell>,
    from: Receiver<Told>,
    /// What the page was last told to show.
    shown: String,
}

thread_local! {
    /// The Development window of the app on this thread.
    static OPEN: RefCell<Option<Open>> = const { RefCell::new(None) };
    /// The first frame of the app on this thread is still to come.
    static AT_START: Cell<bool> = const { Cell::new(true) };
}

/// Whether an address is the page's own.
fn is_own(address: &str) -> bool {
    OWN.iter().any(|own| address.starts_with(own))
}

/// The kind of a file of the page, by its name.
fn kind(name: &str) -> &'static str {
    match name.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        _ => "text/plain; charset=utf-8",
    }
}

/// Answers a request of the page: one of its own files, by name, or nothing.
fn serve(request: &Request<Vec<u8>>) -> Response<Cow<'static, [u8]>> {
    let name = request.uri().path().trim_start_matches('/');
    let found = (request.method() == "GET").then(|| page::FILES.iter().find(|(file, _)| *file == name)).flatten();
    let (status, kind, body): (u16, &str, &'static [u8]) = match found {
        Some((file, text)) => (200, kind(file), text.as_bytes()),
        None => (404, "text/plain; charset=utf-8", b"Not found"),
    };
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, kind)
        .header("Content-Security-Policy", POLICY)
        .header("X-Content-Type-Options", "nosniff")
        .header(header::CACHE_CONTROL, "no-store")
        .body(Cow::Borrowed(body))
        .unwrap_or_else(|_| Response::new(Cow::Borrowed(&[])))
}

/// The script that hands the page what to show. The JSON goes in as a string for the page
/// to read, so that nothing in it is ever taken as script.
fn script(state: &str) -> String {
    let quoted = serde_json::to_string(state).unwrap_or_else(|_| "\"null\"".into());
    // Two characters JSON allows in a string that older script rules did not.
    let quoted = quoted.replace('\u{2028}', "\\u2028").replace('\u{2029}', "\\u2029");
    format!("window.gridcraftState&&window.gridcraftState(JSON.parse({quoted}))")
}

/// The window's thread: makes the window and its web view, and runs their loop until the
/// window is closed.
fn run(said: Sender<Told>, ready: Sender<EventLoopProxy<Tell>>, wake: egui::Context, folder: Option<PathBuf>) {
    let tell = |told: Told| {
        let _ = said.send(told);
        wake.request_repaint();
    };
    let mut event_loop = EventLoopBuilder::<Tell>::with_user_event().with_any_thread(true).build();
    if ready.send(event_loop.create_proxy()).is_err() {
        return;
    }
    // A class of its own: the app's first window is made by another library, which names its
    // class as this one would by default.
    let window = WindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(LogicalSize::new(1040.0, 820.0))
        .with_min_inner_size(LogicalSize::new(640.0, 420.0))
        .with_window_classname("GridCraft Development Window")
        .build(&event_loop);
    let window = match window {
        Ok(window) => window,
        Err(why) => return tell(Told::Failed(why.to_string())),
    };
    let mut context = wry::WebContext::new(folder);
    let (from_page, wake_page) = (said.clone(), wake.clone());
    let view = wry::WebViewBuilder::new_with_web_context(&mut context)
        .with_custom_protocol(SCHEME.into(), |_, request| serve(&request))
        .with_https_scheme(true)
        .with_url(START)
        // Only the page served from here speaks for the person.
        .with_ipc_handler(move |request: Request<String>| {
            if is_own(&request.uri().to_string()) {
                let _ = from_page.send(Told::Said(request.into_body()));
                wake_page.request_repaint();
            }
        })
        .with_navigation_handler(|to| is_own(&to))
        .with_new_window_req_handler(|_, _| wry::NewWindowResponse::Deny)
        .with_download_started_handler(|_, _| false)
        .with_devtools(false)
        .with_browser_accelerator_keys(false)
        .with_default_context_menus(false)
        .with_hotkeys_zoom(false)
        .build(&window);
    let view = match view {
        Ok(view) => view,
        Err(why) => return tell(Told::Failed(why.to_string())),
    };
    event_loop.run_return(move |event, _, flow| {
        *flow = ControlFlow::Wait;
        match event {
            Event::UserEvent(Tell::Show(state)) => {
                let _ = view.evaluate_script(&script(&state));
            }
            Event::UserEvent(Tell::Raise) => {
                window.set_minimized(false);
                window.set_visible(true);
                window.set_focus();
            }
            Event::UserEvent(Tell::Close) | Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *flow = ControlFlow::Exit,
            _ => {}
        }
    });
    tell(Told::Closed);
}

/// Opens the window, on a thread of its own.
fn start(ctx: &egui::Context) -> Result<Open, String> {
    let (said, from) = mpsc::channel();
    let (ready, made) = mpsc::channel();
    let (wake, folder) = (ctx.clone(), devpage::web_view_folder());
    std::thread::Builder::new()
        .name("gridcraft-development-window".into())
        .spawn(move || run(said, ready, wake, folder))
        .map_err(|e| format!("its thread could not be started: {e}"))?;
    // The thread says when its loop is there to be told things: a moment. The window and the
    // web view are made after that, and say so themselves if they can't be.
    let to = made.recv_timeout(Duration::from_secs(5)).map_err(|_| "its thread did not start".to_string())?;
    Ok(Open { to, from, shown: String::new() })
}

/// Hook of the shell's `ui`, after the app has drawn: the Development window, when it is
/// shown as a page.
pub fn keep_up(app: &mut SheetApp, ctx: &egui::Context) {
    devpage::host_is_here(ctx);
    // A development kit's package is chosen in the operating system's own file dialog.
    if !devpage::has_kit_picker() {
        devpage::set_kit_picker(Box::new(|| {
            rfd::FileDialog::new()
                .set_title("Load a development kit")
                .add_filter("GridCraft development kit", &["gridkit"])
                .pick_file()
                .and_then(|chosen| chosen.to_str().map(str::to_string))
        }));
    }
    // Whoever starts GridCraft can have the window open as it starts (`GRIDCRAFT_DEVELOPMENT`).
    // Opening it decides nothing.
    if AT_START.with(|first| first.replace(false)) && std::env::var_os("GRIDCRAFT_DEVELOPMENT").is_some_and(|set| !set.is_empty()) {
        devpage::open(ctx);
    }
    OPEN.with(|open| {
        let mut open = open.borrow_mut();
        // What the page said, and whether its window is still there.
        let mut gone = false;
        if let Some(window) = open.as_mut() {
            loop {
                match window.from.try_recv() {
                    Ok(Told::Said(message)) => {
                        if devpage::handle(app, ctx, &message) {
                            window.shown.clear();
                        }
                    }
                    Ok(Told::Failed(why)) => {
                        devpage::failed(app, ctx, &why);
                        gone = true;
                        break;
                    }
                    Ok(Told::Closed) | Err(TryRecvError::Disconnected) => {
                        devpage::closed(ctx);
                        gone = true;
                        break;
                    }
                    Err(TryRecvError::Empty) => break,
                }
            }
        }
        if gone {
            *open = None;
        }
        if !devpage::wanted(ctx) {
            // Not wanted any more: the window goes.
            if let Some(window) = open.take() {
                let _ = window.to.send_event(Tell::Close);
            }
            return;
        }
        if open.is_none() {
            match start(ctx) {
                Ok(window) => *open = Some(window),
                Err(why) => return devpage::failed(app, ctx, &why),
            }
        }
        let Some(window) = open.as_mut() else { return };
        if devpage::take_raise(ctx) {
            let _ = window.to.send_event(Tell::Raise);
        }
        // What it shows, when that has changed.
        if let Some(state) = devpage::state(app, ctx)
            && state != window.shown
        {
            let _ = window.to.send_event(Tell::Show(state.clone()));
            window.shown = state;
        }
        // The window keeps up with what happens while nobody touches the app's first window.
        ctx.request_repaint_after(Duration::from_millis(350));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asked(method: &str, address: &str) -> Response<Cow<'static, [u8]>> {
        serve(&Request::builder().method(method).uri(address).body(Vec::new()).unwrap())
    }

    #[test]
    fn the_page_is_served_from_the_apps_own_files_and_nothing_else_is() {
        for (name, kind) in
            [("index.html", "text/html; charset=utf-8"), ("app.js", "text/javascript; charset=utf-8"), ("app.css", "text/css; charset=utf-8")]
        {
            let answer = asked("GET", &format!("gridcraft://development.invalid/{name}"));
            assert_eq!(answer.status(), 200, "{name}");
            assert_eq!(answer.headers()[header::CONTENT_TYPE], kind);
            // Every answer carries the policy: the page's own script and stylesheet, nothing else.
            let policy = answer.headers()["Content-Security-Policy"].to_str().unwrap();
            for part in ["default-src 'none'", "script-src 'self'", "style-src 'self'", "base-uri 'none'", "form-action 'none'"] {
                assert!(policy.contains(part), "{part}");
            }
            assert!(!policy.contains("unsafe") && !policy.contains('*') && !policy.contains("http"), "{policy}");
            assert_eq!(answer.headers()["X-Content-Type-Options"], "nosniff");
            assert!(!answer.body().is_empty());
        }
        for other in ["", "index.htm", "../page.rs", "src/main.js", "README.md", "app.js/x", "%2e%2e/app.js"] {
            let answer = asked("GET", &format!("gridcraft://development.invalid/{other}"));
            assert_eq!(answer.status(), 404, "{other}");
            assert!(answer.headers().contains_key("Content-Security-Policy"));
        }
        assert_eq!(asked("POST", "gridcraft://development.invalid/index.html").status(), 404);
    }

    #[test]
    fn only_the_pages_own_address_is_its_own() {
        for own in
            ["https://gridcraft.development.invalid/index.html", "https://gridcraft.development.invalid/", "gridcraft://development.invalid/app.js"]
        {
            assert!(is_own(own), "{own}");
        }
        for other in [
            "https://gridcraft.development.invalid.example.com/",
            "https://gridcraft.development.invalid",
            "http://gridcraft.development.invalid/",
            "https://example.com/?https://gridcraft.development.invalid/",
            "https://development.invalid/",
            "about:blank",
            "file:///C:/Users/",
            "javascript:alert(1)",
            "data:text/html,<script>window.ipc.postMessage('{}')</script>",
        ] {
            assert!(!is_own(other), "{other}");
        }
        assert!(is_own(START));
    }

    #[test]
    fn the_page_has_no_script_of_its_own_writing_and_asks_for_nothing_from_outside() {
        let file = |name: &str| page::FILES.iter().find(|(file, _)| *file == name).unwrap().1;
        let html = file("index.html");
        assert_eq!(html.matches("<script").count(), 1);
        assert!(html.contains("<script src=\"app.js\"></script>") && html.contains("<link rel=\"stylesheet\" href=\"app.css\""));
        assert!(!html.contains("http") && !html.contains("style=") && !html.contains("onload") && !html.contains("onclick"));
        let js = file("app.js");
        for never in ["eval(", "new Function(", "document.write", "insertAdjacentHTML", "XMLHttpRequest", "fetch(", "WebSocket", "import("] {
            assert!(!js.contains(never), "{never}");
        }
        // What the page says to GridCraft, it says through the one door.
        assert!(js.contains("window.ipc") && js.contains("gridcraftState"));
        assert!(!file("app.css").contains("url("), "the stylesheet loads nothing");
    }

    #[test]
    fn the_page_has_a_colour_for_every_badge_gridcraft_names() {
        // GridCraft says which badge something carries by a code; the page's stylesheet holds
        // the colours. A code without a rule there would be drawn as nothing in particular.
        let css = page::FILES.iter().find(|(file, _)| *file == "app.css").unwrap().1;
        let script = page::FILES.iter().find(|(file, _)| *file == "app.js").unwrap().1;
        let codes = devpage::badge_codes();
        assert!(!codes.is_empty());
        for code in codes {
            assert!(css.contains(&format!(".b-{code}{{")), "the stylesheet has no badge {code:?}");
            assert!(script.contains(&format!("b-{code}")), "the script knows no badge {code:?}");
        }
        assert!(css.contains(".b-plain") || css.contains(".tech{"), "a badge with no colour of its own is still a badge");
    }

    #[test]
    fn the_marks_in_the_badges_are_paths_and_every_badge_with_a_colour_has_one() {
        // The marks are other people's, from Simple Icons (the fork's exception:
        // ATTRIBUTION-aniefork.md). What is kept of each is one path of drawing commands,
        // which the page looks up by a code of its own.
        let source = page::SOURCE.iter().find(|(file, _)| *file == "src/logos.js").unwrap().1;
        let script = page::FILES.iter().find(|(file, _)| *file == "app.js").unwrap().1;
        let codes = devpage::badge_codes();
        let mut found = Vec::new();
        for line in source.lines() {
            let Some((code, rest)) = line.strip_prefix("  ").and_then(|entry| entry.split_once(": \"")) else { continue };
            if !code.chars().all(|c| c.is_ascii_lowercase()) {
                continue;
            }
            let path = rest.strip_suffix("\",").unwrap();
            assert!(codes.contains(&code), "{code}");
            assert!(!path.is_empty() && path.len() < 4000, "{code}");
            assert!(path.chars().all(|c| "MmLlHhVvCcSsQqTtAaZz0123456789eE .,-".contains(c)), "{code}: not drawing commands alone");
            assert!(script.contains(path), "the built page does not hold the mark of {code:?}: build it again");
            found.push(code);
        }
        assert_eq!(found, codes, "a mark for every badge that has a colour, and no other");
        // Whose they are and where the shapes are from is said with the page, and in the fork's list.
        let notices = page::SOURCE.iter().find(|(file, _)| *file == "THIRD-PARTY.md").unwrap().1;
        let listed = include_str!("../../../../ATTRIBUTION-aniefork.md");
        for said in ["Simple Icons 16.35.0", "CC0 1.0", "Python Software Foundation", "not to be offered upstream"] {
            assert!(notices.contains(said) && listed.contains(said), "{said}");
        }
    }

    #[test]
    fn what_the_page_is_told_goes_in_as_a_string() {
        let state = "{\"said\":\"</script><script>alert(1)//\\\"\u{2028}\"}";
        let handed = script(state);
        assert!(handed.starts_with("window.gridcraftState&&window.gridcraftState(JSON.parse(\"") && handed.ends_with("\"))"));
        assert!(!handed.contains('\u{2028}') && handed.contains("\\u2028"));
        // Inside the call there is one string, and it reads back as what was handed over.
        let inner = handed.strip_prefix("window.gridcraftState&&window.gridcraftState(JSON.parse(").unwrap().strip_suffix("))").unwrap();
        assert_eq!(serde_json::from_str::<String>(inner).unwrap(), state);
    }

    // --- the two tools for whoever changes the page ---------------------------------------

    /// The file `page.rs`, written from the page's files and its project.
    fn rendered(files: &[(String, String)], source: &[(String, String)]) -> String {
        let raw = |text: &str| {
            let mut hashes = 1;
            while text.contains(&format!("\"{}", "#".repeat(hashes))) {
                hashes += 1;
            }
            format!("r{0}\"{text}\"{0}", "#".repeat(hashes))
        };
        let whole = include_str!("page.rs").replace("\r\n", "\n");
        let head = whole.split("\n/// The page as the shell serves it").next().unwrap().trim_end().to_string();
        let mut out = vec![head, String::new()];
        out.push("/// The page as the shell serves it: each file's name, and its text.".into());
        out.push("#[rustfmt::skip]".into());
        out.push("pub(crate) const FILES: &[(&str, &str)] = &[".into());
        out.extend(files.iter().map(|(name, text)| format!("    ({name:?}, {}),", raw(text))));
        out.push("];".into());
        out.push(String::new());
        out.push("/// The project the page is built from: each file's path, and its text. It is for whoever".into());
        out.push("/// changes the page (its README.md says how); the app itself doesn't hold it.".into());
        out.push("#[cfg(test)]".into());
        out.push("#[rustfmt::skip]".into());
        out.push("pub(crate) const SOURCE: &[(&str, &str)] = &[".into());
        out.extend(source.iter().map(|(name, text)| format!("    ({name:?}, {}),", raw(text))));
        out.push("];".into());
        out.join("\n") + "\n"
    }

    fn owned(files: &[(&str, &str)]) -> Vec<(String, String)> {
        files.iter().map(|(name, text)| (name.to_string(), text.replace("\r\n", "\n"))).collect()
    }

    /// Where the project is written out, and taken back in from.
    fn project() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/development-page")
    }

    #[test]
    fn the_generated_file_is_what_its_tool_writes() {
        assert_eq!(rendered(&owned(page::FILES), &owned(page::SOURCE)), include_str!("page.rs").replace("\r\n", "\n"));
        // And the built script is of this project: it sends what the source sends.
        let script = page::FILES.iter().find(|(file, _)| *file == "app.js").unwrap().1;
        let source: String = page::SOURCE.iter().filter(|(file, _)| file.starts_with("src/")).map(|(_, text)| *text).collect();
        let mut acts = 0;
        for call in source.split("send(\"").skip(1) {
            let act = call.split('"').next().unwrap();
            // However the build quotes it.
            let sent = ['"', '\'', '`'].iter().any(|quote| script.contains(&format!("{quote}{act}{quote}")));
            assert!(sent, "the built page never sends {act:?}: build it again");
            acts += 1;
        }
        assert!(acts > 15, "{acts}");
    }

    /// Writes the page's project to `target/development-page`, to be changed and built there.
    #[test]
    #[ignore = "a tool: it writes the page's project out"]
    fn write_the_pages_source() {
        let dir = project();
        for (name, text) in page::SOURCE {
            let file = dir.join(name);
            std::fs::create_dir_all(file.parent().unwrap()).unwrap();
            std::fs::write(file, text.replace("\r\n", "\n")).unwrap();
        }
        println!("the page's project is in {}", dir.display());
    }

    /// Takes the project in `target/development-page` and what `npm run build` made of it,
    /// and writes `page.rs` from them.
    #[test]
    #[ignore = "a tool: it rewrites page.rs from the built project"]
    fn take_the_built_page() {
        let dir = project();
        let read = |name: &str| std::fs::read_to_string(dir.join(name)).unwrap_or_else(|e| panic!("{name}: {e}")).replace("\r\n", "\n");
        let files: Vec<(String, String)> = page::FILES
            .iter()
            .map(|(name, _)| (name.to_string(), read(&if *name == "index.html" { name.to_string() } else { format!("dist/{name}") })))
            .collect();
        // The project's files as they were, and whatever was added under src/.
        let mut names: Vec<String> = page::SOURCE.iter().map(|(name, _)| name.to_string()).filter(|name| dir.join(name).is_file()).collect();
        let mut added: Vec<String> = std::fs::read_dir(dir.join("src"))
            .unwrap()
            .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
            .map(|name| format!("src/{name}"))
            .filter(|name| !names.contains(name) && dir.join(name).is_file())
            .collect();
        added.sort();
        names.extend(added);
        let source: Vec<(String, String)> = names.iter().map(|name| (name.clone(), read(name))).collect();
        let file = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/aniefork/page.rs");
        std::fs::write(&file, rendered(&files, &source)).unwrap();
        println!("wrote {}", file.display());
    }
}
