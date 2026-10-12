//! Developer ▸ Development: the Development management window. The one place where the
//! development tools are gathered, and where the person sets how far an agent may help with
//! them.
//!
//! **Agents.** GridCraft's Development server (see [`super::agents`]) is off until it is turned
//! on here. The person chooses a level; raising it ends in a question drawn by the operating
//! system. The window hands out what an agent needs to connect (to the clipboard: the token is
//! never drawn or reported), shows who connected, and holds the requests that wait for an
//! answer.
//!
//! **A request is answered on a card**, in the manner of a coding agent's permission prompt:
//! what it is, the thing itself in a block of its own (for a file that is there, the lines it
//! would take out and put in), what it means, and then yes, or no with a word for the agent
//! on what to do differently. A yes is confirmed by the operating system's own question before
//! anything is done, and can't be pressed in the card's first moment on screen, so that a
//! click meant for what was there before answers nothing. **What is on the card is what is
//! answered**: only the person's own click puts a request's card on screen or takes it off
//! (the window opens on a card by itself only when a single request waits, so there is no
//! other it could have been), the card and the operating system's question name the request
//! by its number and by a mark of what it holds, and a request too long to show whole can't
//! be said yes to.
//!
//! **Tools.** The modules, the Developer console and the Python that runs scripts, each with
//! where it stands and the way to its own window.
//!
//! The control channel can click and type in this window as a person does, and rewrite a
//! dialog's values. So the window works from a state of its own that the channel can't reach,
//! and everything that decides something (the level, the port, the token, the setup, an answer
//! to a request) is refused while the channel is putting input into the window; see
//! [`super::presence`].

use gridcraft_aniefork_dev::agents::{Action, Asked, Level};
use gridcraft_aniefork_dev::bridge::Places;
use gridcraft_aniefork_dev::trust::TrustStore;
use gridcraft_aniefork_dev::{module, settings, starter};
use serde_json::{Value as Json, json};

use super::diff::{self, Line, Row};
use super::{agents, modules, presence};
use crate::SheetApp;
use crate::dialogs::Dialog;
use crate::theme::Tokens;

/// The dialog's name, as `ui.dialog {name}` takes it.
pub const NAME: &str = "development";

const INTRO: &str = "Your development tools, and how far an agent may help you with them.";
const AGENTS: &str =
    "An agent (Claude Code, for one) can connect to GridCraft's Development server to help you build modules. You set how far it may go.";
const COPIED: &str = "The setup was copied. It holds the token, which is a secret: paste it only into your agent's own settings.";
/// The most of a file or of a script's data shown in a request (characters). What is cut says so.
const SHOWN: usize = 200_000;
/// The most lines of a comparison drawn. What is cut says so.
const ROWS: usize = 4_000;
/// How many unchanged lines are shown before and after each change to a file.
const AROUND: usize = 3;
/// How long a request's card is on screen before its yes can be pressed (milliseconds): a click
/// meant for what was there a moment ago must not answer it.
const SETTLE_MS: f64 = if cfg!(test) { 60.0 } else { 900.0 };

const YES: &str = "1. Yes, allow it once";
const NO: &str = "2. No";
const REASON: &str = "…and tell the agent what to do differently";
const NOT_NOW: &str = "Not now";

/// What the person asked for in one frame; carried out after the window is drawn.
enum Act {
    Level(Level),
    Port(String),
    CopySetup {
        as_json: bool,
    },
    NewToken,
    Look(u64),
    NotNow,
    Allow(u64),
    Refuse(u64),
    /// Show the whole of the file that would be written, or only what changes.
    Whole(bool),
    Open(&'static str),
    OpenFolder,
}

/// What the window works from, kept between frames. It is the window's own, out of the dialog's
/// values, which anything on the control channel can rewrite: the values only report.
#[derive(Clone, Default)]
struct State {
    /// The last frame the window was drawn in: after a gap it was closed, and starts afresh.
    seen: u64,
    port: String,
    /// The request the person is looking at.
    looking: Option<u64>,
    /// When its card came on screen (`now_ms`).
    shown_at: f64,
    /// What the person would tell the agent with a no.
    reason: String,
    /// Show the whole of the file that would be written, not only what changes.
    whole: bool,
    said: String,
}

fn state_id() -> egui::Id {
    egui::Id::new("aniefork.development.state")
}

/// The value that marks a dialog as already drawn: one without it has just been opened.
const OPENED: &str = "opened";

/// What the dialog's values report when it opens. The window doesn't work from them.
pub fn values() -> Json {
    let now = agents::snapshot();
    json!({"level": now.level.code(), "serving": now.serving, "waiting": now.waiting.len(), "looking": null, "said": ""})
}

impl State {
    /// The window as it opens: on the request that waits, when exactly one does. With several
    /// it opens on the list, for the person to choose: opening the window (which anything on
    /// the control channel can do) never decides which request is under their eyes.
    fn fresh() -> State {
        let kept = modules::places().map(|at| settings::load(&at.settings).agents.port).unwrap_or_default();
        let mut state = State { port: kept.to_string(), ..State::default() };
        let only = match agents::snapshot().waiting.as_slice() {
            [only] => Some(only.id),
            _ => None,
        };
        state.look_at(only);
        state
    }

    /// Turns to the request `id` (or away from them all): its card starts afresh.
    fn look_at(&mut self, id: Option<u64>) {
        self.looking = id;
        self.shown_at = crate::now_ms();
        self.reason.clear();
        self.whole = false;
    }

    /// Writes what the dialog's values report. They are never read back, and never hold the token.
    fn report(&self, d: &mut Dialog) {
        let now = agents::snapshot();
        let waiting: Vec<Json> = now.waiting.iter().map(|asked| json!({"request": asked.id, "what": asked.action.title()})).collect();
        for (key, value) in [
            ("level", json!(now.level.code())),
            ("serving", json!(now.serving)),
            ("port", if now.serving { json!(now.port) } else { Json::Null }),
            ("problem", json!(now.problem)),
            ("waiting", json!(waiting)),
            ("looking", json!(self.looking)),
            ("said", json!(self.said)),
            (OPENED, json!(true)),
        ] {
            d.values.insert(key.into(), value);
        }
    }
}

fn heading(ui: &mut egui::Ui, text: &str) {
    ui.add_space(6.0);
    ui.label(egui::RichText::new(text).strong().size(14.0));
    ui.separator();
}

fn small(text: impl Into<String>) -> egui::RichText {
    egui::RichText::new(text).small()
}

/// A block for the thing itself: a file's lines, a script's data. It scrolls when it is long.
fn block(ui: &mut egui::Ui, salt: &str, job: egui::text::LayoutJob) {
    egui::ScrollArea::both().id_salt(salt).max_height(230.0).auto_shrink([false, true]).show(ui, |ui| {
        ui.add(egui::Label::new(job).selectable(true).extend());
    });
}

fn mono(ui: &egui::Ui) -> egui::FontId {
    let mut font = egui::TextStyle::Monospace.resolve(ui.style());
    font.size = (font.size * 0.92).max(9.0);
    font
}

/// Text in a block, whole, with its lines numbered. What is too long to draw is cut, and says
/// how much is missing.
fn text_block(ui: &mut egui::Ui, salt: &str, text: &str, numbered: bool) {
    let t = Tokens::get(ui.ctx());
    let font = mono(ui);
    let count = text.chars().count();
    let shown: String = if count > SHOWN { text.chars().take(SHOWN).collect() } else { text.to_string() };
    let mut job = egui::text::LayoutJob::default();
    for (index, line) in shown.lines().enumerate() {
        if numbered {
            job.append(&format!("{:>4}  ", index + 1), 0.0, egui::TextFormat { font_id: font.clone(), color: t.text_dim, ..Default::default() });
        }
        job.append(&format!("{line}\n"), 0.0, egui::TextFormat { font_id: font.clone(), color: t.text, ..Default::default() });
    }
    block(ui, salt, job);
    if count > SHOWN {
        let warn = ui.visuals().warn_fg_color;
        ui.label(egui::RichText::new(format!("Only the first {SHOWN} characters are shown: {} more are not.", count - SHOWN)).color(warn).strong());
    }
}

/// What writing `new` over `old` would do: the lines taken out and put in, with a little of
/// what stays around them.
fn diff_block(ui: &mut egui::Ui, salt: &str, compared: &[Line]) {
    let t = Tokens::get(ui.ctx());
    let font = mono(ui);
    let tint = |colour: egui::Color32| egui::Color32::from_rgba_unmultiplied(colour.r(), colour.g(), colour.b(), 46);
    let plain = |colour: egui::Color32| egui::TextFormat { font_id: font.clone(), color: colour, ..Default::default() };
    let marked = |colour: egui::Color32| egui::TextFormat { font_id: font.clone(), color: t.text, background: tint(colour), ..Default::default() };
    let rows = diff::shown(compared, AROUND);
    let mut job = egui::text::LayoutJob::default();
    for row in rows.iter().take(ROWS) {
        match row {
            Row::Skipped(count) => {
                job.append(&format!("       ⋯ {count} line{} the same\n", if *count == 1 { "" } else { "s" }), 0.0, plain(t.text_dim))
            }
            Row::Line(Line::Same(_, number, text)) => {
                job.append(&format!("{number:>4}   "), 0.0, plain(t.text_dim));
                job.append(&format!("{text}\n"), 0.0, plain(t.text));
            }
            Row::Line(Line::Gone(number, text)) => {
                job.append(&format!("{number:>4} - "), 0.0, plain(t.danger));
                job.append(&format!("{text}\n"), 0.0, marked(t.danger));
            }
            Row::Line(Line::New(number, text)) => {
                job.append(&format!("{number:>4} + "), 0.0, plain(t.accent));
                job.append(&format!("{text}\n"), 0.0, marked(t.accent));
            }
        }
    }
    block(ui, salt, job);
    if rows.len() > ROWS {
        let warn = ui.visuals().warn_fg_color;
        ui.label(
            egui::RichText::new(format!("Only the first {ROWS} lines of the changes are shown. Show the whole file to read the rest."))
                .color(warn)
                .strong(),
        );
    }
}

/// What the agent's client calls itself, for a sentence: its own word, or "the agent".
fn agent(asked: &Asked) -> String {
    if asked.client == "an agent" { "the agent".into() } else { asked.client.clone() }
}

/// Whether the module has a plain file at `path` now.
fn there(at: &Places, id: &str, path: &str) -> bool {
    at.modules.join(id).join(path).is_file()
}

/// What kind of thing a request is: the card's heading.
fn kind(at: &Places, action: &Action) -> &'static str {
    match action {
        Action::CreateModule { .. } => "Create module",
        Action::WriteFile { module, path, .. } if there(at, module, path) => "Edit file",
        Action::WriteFile { .. } => "Create file",
        Action::DeleteFile { .. } => "Delete file",
        Action::RunScript { .. } => "Run script",
        Action::OpenPanel { .. } => "Open panel",
        Action::Trust { .. } => "Trust module",
        Action::StopTrusting { .. } => "Stop trusting module",
    }
}

/// What a request is about: the first line of the card's block.
fn target(action: &Action) -> String {
    match action {
        Action::CreateModule { name } => format!("{}/   ({name:?})", starter::slug(name)),
        Action::WriteFile { module, path, .. } | Action::DeleteFile { module, path } => format!("{module}/{path}"),
        Action::RunScript { module, script, .. } => format!("{module}: {script}"),
        Action::OpenPanel { module } | Action::Trust { module, .. } | Action::StopTrusting { module } => module.clone(),
    }
}

/// The question the card puts. What the agent named freely is inside quotes.
fn question(at: &Places, asked: &Asked) -> String {
    let agent = agent(asked);
    match &asked.action {
        Action::CreateModule { .. } => format!("Do you want to let {agent} create this module?"),
        Action::WriteFile { module, path, .. } if there(at, module, path) => format!("Do you want to let {agent} make this edit to {path:?}?"),
        Action::WriteFile { path, .. } => format!("Do you want to let {agent} create {path:?}?"),
        Action::DeleteFile { path, .. } => format!("Do you want to let {agent} delete {path:?}?"),
        Action::RunScript { script, .. } => format!("Do you want to let {agent} run {script:?}?"),
        Action::OpenPanel { module } => format!("Do you want to let {agent} open the panel of {module} in your browser?"),
        Action::Trust { module, .. } => format!("Do you want to trust {module}, as {agent} asks?"),
        Action::StopTrusting { module } => format!("Do you want to stop trusting {module}, as {agent} asks?"),
    }
}

/// What a change to a module's files means for its trust.
fn change_means(ui: &mut egui::Ui, at: &Places, id: &str) {
    if TrustStore::load(&at.trust).follows_changes(id) {
        let warn = ui.visuals().warn_fg_color;
        let text = format!("You chose to keep trusting {id} as its files change: what is written runs without your looking at it again.");
        ui.label(egui::RichText::new(text).color(warn).strong());
    } else {
        ui.label(format!("{id} stops being trusted when its files change: nothing in it runs until you review it again."));
    }
}

/// The thing itself, in the card's block: everything a request would do.
fn content(ui: &mut egui::Ui, at: &Places, asked: &Asked, whole: bool, action: &mut Option<Act>) {
    let warn = ui.visuals().warn_fg_color;
    match &asked.action {
        Action::CreateModule { .. } => {
            ui.label("A starter module: a module.toml, a panel and a Python script.");
        }
        Action::WriteFile { module, path, text } => {
            if !there(at, module, path) {
                ui.label(small("The file is new:"));
                text_block(ui, "request_text", text, true);
                return;
            }
            // The file that is there, when it can be read as text and compared.
            let old = gridcraft_aniefork_dev::agents::read_file(at, module, path).ok();
            let compared = old.as_deref().and_then(|old| diff::lines(old, text));
            match (&compared, whole) {
                (Some(compared), false) => {
                    let (gone, new) = diff::counts(compared);
                    ui.horizontal(|ui| {
                        let same = gone == 0 && new == 0;
                        ui.label(small(if same {
                            "It changes the file that is there: no line differs but for line breaks or what ends it.".to_string()
                        } else {
                            format!("It changes the file that is there: {new} line{} in, {gone} out.", if new == 1 { "" } else { "s" })
                        }));
                        if ui.small_button("Show whole file").clicked() {
                            *action = Some(Act::Whole(true));
                        }
                    });
                    diff_block(ui, "request_diff", compared);
                }
                _ => {
                    ui.horizontal(|ui| {
                        ui.label(small(if compared.is_some() {
                            "It replaces the file that is there with this:"
                        } else {
                            "It replaces the file that is there, which can't be compared line by line, with this:"
                        }));
                        if compared.is_some() && ui.small_button("Show changes").clicked() {
                            *action = Some(Act::Whole(false));
                        }
                    });
                    text_block(ui, "request_text", text, true);
                }
            }
        }
        Action::DeleteFile { .. } => {
            ui.label("The file is deleted.");
        }
        Action::RunScript { data, on_selection, timeout, .. } => {
            if *on_selection {
                ui.label("main(data) is handed the current selection.");
            } else {
                ui.label(small("main(data) is handed this:"));
                text_block(ui, "request_data", &serde_json::to_string_pretty(data).unwrap_or_default(), false);
            }
            ui.label(small(format!("It may run for up to {} seconds.", timeout.as_secs().max(1))));
        }
        Action::OpenPanel { .. } => {
            ui.label("Its panel opens in your browser.");
        }
        Action::Trust { module, fingerprint } => {
            let found = module::load(&at.modules.join(module));
            if found.fingerprint.as_deref() != Some(fingerprint.as_str()) {
                ui.label(
                    egui::RichText::new("The module changed after the agent asked. It has to ask again: this request can't be allowed.")
                        .color(warn)
                        .strong(),
                );
                return;
            }
            ui.label(egui::RichText::new("It would be allowed to:").strong());
            let asks = found.manifest.as_ref().map(|m| m.permissions.clone()).unwrap_or_default();
            if asks.is_empty() {
                ui.label("•  Show its panel, and nothing more");
            }
            for permission in asks {
                let line = egui::RichText::new(format!("•  {}", permission.meaning()));
                ui.label(if permission.is_unlimited() { line.color(warn).strong() } else { line });
            }
            let shown: Vec<&str> = found.files.iter().map(String::as_str).take(8).collect();
            let more = found.files.len().saturating_sub(shown.len());
            ui.label(small(format!(
                "It contains {} file{}: {}{}. Fingerprint {}…",
                found.files.len(),
                if found.files.len() == 1 { "" } else { "s" },
                shown.join(", "),
                if more > 0 { format!(" and {more} more") } else { String::new() },
                fingerprint.chars().take(16).collect::<String>()
            )));
        }
        Action::StopTrusting { .. } => {
            ui.label("Its trust is taken back.");
        }
    }
}

/// What a yes would mean, under the card's block.
fn means(ui: &mut egui::Ui, at: &Places, action: &Action) {
    let warn = ui.visuals().warn_fg_color;
    match action {
        Action::CreateModule { .. } => {
            ui.label("It is created untrusted: it does nothing until you trust it.");
        }
        Action::WriteFile { module, .. } | Action::DeleteFile { module, .. } => change_means(ui, at, module),
        Action::RunScript { .. } => {
            ui.label("It may change the workbook, with what its module was granted.");
            ui.label(egui::RichText::new("A script can do anything a program you run can do.").color(warn).strong());
        }
        Action::OpenPanel { .. } => {
            ui.label("Its panel can do what you granted the module.");
        }
        Action::Trust { .. } => {
            ui.label("Trust is given to exactly these files. To read them first, say no, and review the module in Modules.");
        }
        Action::StopTrusting { .. } => {
            ui.label("It does nothing after that until you trust it again.");
        }
    }
}

/// What names a request, on its card and in the list: its number, the mark of what it holds
/// (the same mark the operating system's question gives), and how long it has.
fn named(asked: &Asked) -> String {
    let mark = match gridcraft_aniefork_dev::agents::mark(&asked.action).as_str() {
        "" => String::new(),
        mark => format!("  ·  marked {mark}"),
    };
    format!("request #{}{mark}  ·  lapses in {} min", asked.id, asked.left.as_secs() / 60 + 1)
}

/// Whether a request holds more than a card can show whole. Such a request can't be said yes
/// to: nobody has read the whole of it.
fn too_long(action: &Action) -> bool {
    match action {
        Action::WriteFile { text, .. } => text.chars().count() > SHOWN,
        Action::RunScript { data, on_selection: false, .. } => serde_json::to_string_pretty(data).unwrap_or_default().chars().count() > SHOWN,
        _ => false,
    }
}

/// One request that waits for an answer, in the list: what kind of thing, and what about.
fn card(ui: &mut egui::Ui, at: &Places, asked: &Asked, action: &mut Option<Act>) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(kind(at, &asked.action)).strong());
            ui.label(egui::RichText::new(target(&asked.action)).monospace());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Look…").on_hover_text("See everything this request would do, and answer it").clicked() {
                    *action = Some(Act::Look(asked.id));
                }
                ui.label(small(named(asked)));
            });
        });
    });
}

/// One request, in full, for the person to answer: what it is, the thing itself, what it
/// means, and yes or no. It takes the place of the rest of the window, so that what is being
/// decided is all there is to look at.
fn review(ui: &mut egui::Ui, at: &Places, asked: &Asked, state: &mut State, driven: bool, action: &mut Option<Act>) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(kind(at, &asked.action)).strong().size(15.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(small(named(asked)));
        });
    });
    egui::Frame::group(ui.style()).fill(ui.visuals().extreme_bg_color).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.label(egui::RichText::new(target(&asked.action)).monospace().strong());
        ui.add_space(2.0);
        content(ui, at, asked, state.whole, action);
    });
    means(ui, at, &asked.action);
    ui.add_space(8.0);
    ui.label(egui::RichText::new(question(at, asked)).strong());
    ui.add_space(2.0);
    // A card that has only just come on screen can't be said yes to yet.
    let waited = crate::now_ms() - state.shown_at;
    let ready = waited >= SETTLE_MS;
    if !ready {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(100));
    }
    // Nor can one that is too long to have been read whole.
    let cut = too_long(&asked.action);
    if cut {
        let warn = ui.visuals().warn_fg_color;
        let text = "This is too long to show whole, so it can't be allowed here. Say no, and have the agent do it in smaller pieces.";
        ui.label(egui::RichText::new(text).color(warn).strong());
    }
    if ui.add_enabled(ready && !cut, egui::Button::new(YES)).clicked() {
        *action = Some(Act::Allow(asked.id));
    }
    ui.horizontal(|ui| {
        if ui.button(NO).clicked() {
            *action = Some(Act::Refuse(asked.id));
        }
        // The word for the agent is the person's: only what they typed is kept.
        let mut reason = state.reason.clone();
        let edited = ui.add(egui::TextEdit::singleline(&mut reason).hint_text(REASON).desired_width(f32::INFINITY)).changed();
        if edited && !driven {
            state.reason = reason;
        }
    });
    if ui.button(NOT_NOW).on_hover_text("Leave it waiting, and go back").clicked() {
        *action = Some(Act::NotNow);
    }
    ui.add_space(2.0);
    let named = if asked.client == "an agent" {
        "The agent gave no name.".to_string()
    } else {
        format!("Asked by an agent that calls itself \"{}\": the name is its own word.", asked.client)
    };
    ui.label(small(format!("{named} A yes is confirmed by your operating system.")));
}

/// The Agents part: the level, how to connect, who connected, and what waits for an answer.
fn agents_part(ui: &mut egui::Ui, state: &mut State, driven: bool, action: &mut Option<Act>) {
    let t = Tokens::get(ui.ctx());
    let now = agents::snapshot();
    if !now.available {
        ui.label("Agents can't connect in this run, because it keeps no settings.");
        return;
    }
    ui.label(AGENTS);
    ui.add_space(2.0);
    for level in Level::ALL {
        if ui.radio(now.level == level, level.label()).on_hover_text(level.meaning()).clicked() && level != now.level {
            *action = Some(Act::Level(level));
        }
    }
    ui.label(small(now.level.meaning()));
    ui.add_space(4.0);
    // Where it stands.
    if now.serving {
        ui.label(egui::RichText::new(format!("Serving on 127.0.0.1, port {}.", now.port)).color(t.accent).strong());
        match now.clients.first() {
            Some(client) => {
                let more = now.clients.len() - 1;
                let others = if more > 0 { format!(", and {more} more") } else { String::new() };
                ui.label(small(format!(
                    "Last heard from: {} ({} s ago){others}. A client's name is what it chose to say.",
                    client.name,
                    client.heard.as_secs()
                )));
            }
            None => {
                ui.label(small("No agent has connected yet."));
            }
        }
    } else if !now.problem.is_empty() {
        ui.label(egui::RichText::new(format!("Not serving: {}", now.problem)).color(ui.visuals().error_fg_color).strong());
    } else {
        ui.label(small("Not serving: agent access is off."));
    }
    ui.horizontal(|ui| {
        let label = ui.label("Port:");
        // The port is the person's to choose: only what they typed is kept.
        let mut port = state.port.clone();
        let edited =
            ui.add_sized(egui::vec2(70.0, ui.spacing().interact_size.y), egui::TextEdit::singleline(&mut port)).labelled_by(label.id).changed();
        if edited && !driven {
            state.port = port;
        }
        if ui.button("Set Port").clicked() {
            *action = Some(Act::Port(state.port.clone()));
        }
    });
    if now.serving {
        ui.horizontal(|ui| {
            if ui
                .button("Copy Setup for Claude Code")
                .on_hover_text("Copies the command that adds this server to Claude Code. It holds the token.")
                .clicked()
            {
                *action = Some(Act::CopySetup { as_json: false });
            }
            if ui.button("Copy Setup as JSON").on_hover_text("Copies an MCP settings entry, for other agents. It holds the token.").clicked() {
                *action = Some(Act::CopySetup { as_json: true });
            }
            if ui.button("New Token").on_hover_text("Makes a new token: an agent that holds the old one can no longer connect").clicked() {
                *action = Some(Act::NewToken);
            }
        });
    }
    // What waits for an answer.
    if !now.waiting.is_empty() {
        ui.add_space(4.0);
        ui.label(egui::RichText::new(format!("Waiting for your answer ({})", now.waiting.len())).strong());
        if let Some(at) = modules::places() {
            for asked in &now.waiting {
                card(ui, &at, asked, action);
            }
        }
    }
    // What was asked lately.
    if !now.recent.is_empty() {
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Lately").strong());
        let at_ms = crate::now_ms();
        // Each request once, as it stands now: the newest first.
        let mut told = Vec::new();
        for line in now.recent.iter().rev() {
            if line.request != 0 && told.contains(&line.request) {
                continue;
            }
            told.push(line.request);
            let when = super::console::ago(line.at_ms, at_ms);
            ui.label(small(format!("{when}  ·  {}: {}  ·  {}", line.client, line.what, line.outcome.code())));
            if told.len() >= 4 {
                break;
            }
        }
        ui.label(small("The whole record is in the Developer console, and in agent-log.jsonl beside your settings."));
    }
}

/// The Tools part: each development tool, where it stands, and the way to its own window.
fn tools_part(app: &SheetApp, ui: &mut egui::Ui, action: &mut Option<Act>) {
    let listing = modules::list();
    if listing["available"] != true {
        ui.label("Modules are not available in this run, because it keeps no settings.");
        return;
    }
    let all = listing["modules"].as_array().cloned().unwrap_or_default();
    let trusted = all.iter().filter(|m| m["status"] == "trusted").count();
    let console = super::console::report(&json!({"last": 1})).ok().and_then(|report| report["kept"].as_u64()).unwrap_or(0);
    let python = modules::places().map(|at| settings::load(&at.settings).python).unwrap_or_default();
    ui.horizontal(|ui| {
        if ui.button("Modules…").on_hover_text("Create, review, trust and run modules; name the Python that runs their scripts").clicked() {
            *action = Some(Act::Open(modules::NAME));
        }
        ui.label(format!("{} installed, {trusted} trusted", all.len()));
        ui.add_space(12.0);
        if ui.button("Console…").on_hover_text("What your modules print and log, what failed, and what agents asked for").clicked() {
            *action = Some(Act::Open(super::console::NAME));
        }
        ui.label(format!("{console} entr{}", if console == 1 { "y" } else { "ies" }));
    });
    ui.horizontal(|ui| {
        ui.label(small(format!("Python: {}.", if python.is_empty() { "found automatically" } else { python.as_str() })));
        let folder = listing["folder"].as_str().unwrap_or("");
        ui.label(small("Modules folder:"));
        ui.allocate_ui(egui::vec2((ui.available_width() - 110.0).clamp(80.0, 260.0), ui.spacing().interact_size.y), |ui| {
            ui.add(egui::Label::new(egui::RichText::new(folder).monospace().small()).truncate()).on_hover_text(folder);
        });
        if app.services.open_url.is_some() && ui.button("Open Folder").clicked() {
            *action = Some(Act::OpenFolder);
        }
    });
}

/// Hook of the fork's dialogs: draws Developer ▸ Development.
pub fn ui(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut Dialog, open: &mut bool) {
    ui.set_min_width(560.0);
    let ctx = ui.ctx().clone();
    let frame = ctx.cumulative_frame_nr();
    let mut state: State = ctx.data(|data| data.get_temp(state_id())).unwrap_or_default();
    if frame > state.seen.saturating_add(1) || !d.values.contains_key(OPENED) {
        state = State::fresh();
    }
    state.seen = frame;
    // Whether what is clicked and typed now may be the control channel's doing.
    let driven = presence::driven(&ctx);
    let mut action = None;

    // A request being looked at takes the place of the rest, while it still waits.
    let looked_at = state.looking.and_then(|id| agents::snapshot().waiting.into_iter().find(|asked| asked.id == id));
    match (looked_at, modules::places()) {
        (Some(asked), Some(at)) => review(ui, &at, &asked, &mut state, driven, &mut action),
        _ => {
            state.looking = None;
            ui.label(INTRO);
            // On a small screen the rest scrolls, and Close stays in reach.
            let room = (ctx.content_rect().height() - 220.0).max(240.0);
            egui::ScrollArea::vertical().id_salt("development").max_height(room).show(ui, |ui| {
                heading(ui, "Agents");
                agents_part(ui, &mut state, driven, &mut action);
                heading(ui, "Tools");
                tools_part(app, ui, &mut action);
            });
        }
    }
    if !state.said.is_empty() {
        ui.add_space(4.0);
        ui.label(small(state.said.clone()));
    }
    ui.add_space(8.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if ui.button("Close").clicked() {
            *open = false;
        }
    });
    // While a request waits, or a script an agent asked for runs, the window keeps up with it.
    if !agents::snapshot().waiting.is_empty() {
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }

    let outcome = action.map(|action| act(app, &ctx, &mut state, action, driven, open));
    match outcome {
        None => {}
        Some(Ok(said)) => state.said = said,
        Some(Err(why)) => {
            state.said.clear();
            app.message = Some(("Development".into(), why));
        }
    }
    state.report(d);
    ctx.data_mut(|data| data.insert_temp(state_id(), state));
}

/// Carries out what the person asked for. Everything that decides something is theirs alone:
/// asked for while the control channel is putting input into the window, it is refused.
fn act(app: &mut SheetApp, ctx: &egui::Context, state: &mut State, action: Act, driven: bool, open: &mut bool) -> Result<String, String> {
    // Going to another of the fork's windows is anyone's. Everything else is the person's: what
    // is set, what is answered, and which request is under their eyes.
    let anyones = matches!(action, Act::Open(_) | Act::OpenFolder);
    if !anyones && driven {
        return Err(presence::BY_HAND.to_string());
    }
    match action {
        Act::Level(level) => {
            let from = agents::snapshot().level;
            // Letting agents go further is confirmed by the operating system's own question.
            if level > from {
                let Some(confirm) = &app.services.confirm else { return Err(agents::NO_CONFIRM.into()) };
                let text = format!(
                    "Set agent access to \"{}\"?\n\n{}\n\nAn agent connects to GridCraft on this computer, with a token that only you hand out.",
                    level.label(),
                    level.meaning()
                );
                if !confirm("Change agent access?", &text) {
                    return Ok("Agent access was not changed.".into());
                }
            }
            state.looking = None;
            agents::set_level(app, ctx, level)
        }
        Act::Port(text) => {
            let port = text.trim().parse::<u16>().map_err(|_| "Choose a port from 1024 to 65535.".to_string())?;
            agents::set_port(ctx, port)
        }
        Act::CopySetup { as_json } => {
            let setup = agents::setup(as_json)?;
            ctx.copy_text(setup);
            Ok(COPIED.into())
        }
        Act::NewToken => agents::renew_token(ctx),
        Act::Look(id) => {
            state.look_at(Some(id));
            Ok(String::new())
        }
        Act::NotNow => {
            state.look_at(None);
            Ok(String::new())
        }
        Act::Whole(whole) => {
            state.whole = whole;
            Ok(String::new())
        }
        Act::Allow(id) => {
            // The card's first moment on screen: a click then was meant for something else. And
            // only the request whose card is on screen is answered.
            if crate::now_ms() - state.shown_at < SETTLE_MS || state.looking != Some(id) {
                return Ok(String::new());
            }
            let outcome = agents::answer(app, ctx, id, true, "");
            // On to the request that has waited longest, when another waits.
            state.look_at(agents::snapshot().waiting.first().map(|asked| asked.id));
            outcome
        }
        Act::Refuse(id) => {
            let outcome = agents::answer(app, ctx, id, false, &state.reason);
            state.look_at(agents::snapshot().waiting.first().map(|asked| asked.id));
            outcome
        }
        Act::Open(name) => {
            app.open_dialog(name, json!({}));
            *open = false;
            Ok(String::new())
        }
        Act::OpenFolder => match (modules::places(), &app.services.open_url) {
            (Some(at), Some(open_url)) => {
                std::fs::create_dir_all(&at.modules).map_err(|e| format!("The modules folder could not be created: {e}"))?;
                open_url(&at.modules.display().to_string());
                Ok(String::new())
            }
            _ => Err(modules::UNAVAILABLE.into()),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::time::Duration;

    use egui::accesskit::Role;
    use egui_kittest::kittest::Queryable;
    use gridcraft_aniefork_dev::python::PythonHost;

    use super::*;
    use crate::aniefork::modules::tests::{Asked as Answers, Sandbox, app_with, drive, quiet, sandbox, yes};
    use crate::aniefork::modules::{create, list, trust};

    type Harness = egui_kittest::Harness<'static, SheetApp>;

    /// An app in a window, as it is when GridCraft has just started.
    fn harness(app: SheetApp) -> Harness {
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(1200.0, 900.0)).build_ui_state(
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

    fn open(h: &mut Harness) {
        h.state_mut().open_dialog(NAME, json!({}));
        // A new window takes a few frames to find its size and place; clicks go by position.
        h.run_steps(6);
    }

    fn click(h: &mut Harness, label: &str) {
        h.run_steps(3);
        h.get_by_label(label).click();
        h.run_steps(4);
    }

    /// Turns to the request that waits, when the window isn't on its card already.
    fn look(h: &mut Harness) {
        h.run_steps(3);
        if h.query_by_label("Look…").is_some() {
            h.get_by_label("Look…").click();
        }
        h.run_steps(4);
    }

    /// The person says yes on the card, once it has been on screen a moment.
    fn say_yes(h: &mut Harness) {
        std::thread::sleep(Duration::from_millis(SETTLE_MS as u64 + 40));
        click(h, YES);
    }

    /// The person chooses a level in the window.
    fn choose(h: &mut Harness, level: Level) {
        h.run_steps(3);
        h.get_by_role_and_label(Role::RadioButton, level.label()).click();
        h.run_steps(4);
    }

    /// The message the app put up, taken down again.
    fn message(h: &mut Harness) -> Option<String> {
        let said = h.state_mut().message.take().map(|(_, text)| text);
        h.run_steps(2);
        said
    }

    fn token() -> String {
        std::fs::read_to_string(sandbox().unwrap().join("agent-token")).unwrap()
    }

    /// Sends one message to the Development server with `token`: the status, and the answer.
    fn post(token: &str, body: &Json) -> (u16, Json) {
        let now = agents::snapshot();
        assert!(now.serving, "nothing is served: {}", now.problem);
        let body = body.to_string();
        let mut stream = TcpStream::connect(("127.0.0.1", now.port)).unwrap();
        stream.set_read_timeout(Some(Duration::from_secs(30))).unwrap();
        let request = format!(
            "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1:{}\r\nAuthorization: Bearer {token}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{body}",
            now.port,
            body.len()
        );
        stream.write_all(request.as_bytes()).unwrap();
        let mut reply = Vec::new();
        let _ = stream.read_to_end(&mut reply);
        let reply = String::from_utf8_lossy(&reply).to_string();
        let (head, body) = reply.split_once("\r\n\r\n").unwrap_or((&reply, ""));
        let status = head.split(' ').nth(1).and_then(|code| code.parse().ok()).unwrap_or(0);
        (status, serde_json::from_str(body).unwrap_or(Json::Null))
    }

    /// A tool call, as an agent's client holding the token makes it. It doesn't wait for the
    /// outcome: the app has to draw a frame to see the request. Gives (is an error, what was said).
    fn agent(tool: &str, mut arguments: Json) -> (bool, String) {
        arguments["wait_seconds"] = json!(0);
        let message = json!({"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": tool, "arguments": arguments}});
        let (status, reply) = post(&token(), &message);
        assert_eq!(status, 200, "{reply}");
        let result = &reply["result"];
        (result["isError"] == true, result["content"][0]["text"].as_str().unwrap_or_else(|| panic!("{reply}")).to_string())
    }

    /// The number of the request an answer names.
    fn number(text: &str) -> u64 {
        let rest = text.split_once("Request #").unwrap_or_else(|| panic!("no request in {text:?}")).1;
        rest.chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap()
    }

    /// How the request `id` stands, as the agent is told.
    fn stands(id: u64) -> (bool, String) {
        agent("request_status", json!({"request": id}))
    }

    /// An agent asks for something, and the app draws a few frames: (the request, what was said).
    fn asks(h: &mut Harness, tool: &str, arguments: Json) -> u64 {
        let (failed, text) = agent(tool, arguments);
        assert!(!failed, "{text}");
        let id = number(&text);
        h.run_steps(3);
        id
    }

    /// Draws frames until the request `id` is settled.
    fn settled(h: &mut Harness, id: u64) -> (bool, String) {
        for _ in 0..600 {
            h.run_steps(2);
            let (failed, text) = stands(id);
            if !text.contains("is being carried out") && !text.contains("is about to be carried out") {
                return (failed, text);
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        panic!("request #{id} was never settled");
    }

    fn outcomes(s: &Sandbox) -> Vec<String> {
        let text = std::fs::read_to_string(s.0.join("agent-log.jsonl")).unwrap_or_default();
        text.lines().map(|line| serde_json::from_str::<Json>(line).unwrap()["outcome"].as_str().unwrap().to_string()).collect()
    }

    fn kept_level(s: &Sandbox) -> Level {
        settings::load(&s.0.join("developer.json")).agents.level
    }

    const SCRIPT: &str = "def main(data):\n    print('tidying')\n    return {'rows': [[cell.strip() for cell in row] for row in data['rows']]}\n";

    #[test]
    fn agent_access_is_off_until_the_person_turns_it_on() {
        let s = Sandbox::new("agents-off");
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        let report = h.state_mut().run("developer.agents", json!({})).unwrap();
        assert_eq!((&report["level"], &report["serving"], &report["port"]), (&json!("off"), &json!(false), &Json::Null));
        assert!(!s.0.join("agent-token").exists(), "no token is made before one is wanted");
        open(&mut h);
        h.get_by_label("Not serving: agent access is off.");
        assert!(h.query_by_label("Copy Setup for Claude Code").is_none(), "there is nothing to connect to");
        // Turning it on is the person's choice, and the operating system asks them to confirm it.
        choose(&mut h, Level::Read);
        let questions = asked.questions();
        assert_eq!(questions.len(), 1);
        assert!(questions[0].contains("\"Read only\"") && questions[0].contains(Level::Read.meaning()), "{questions:?}");
        let now = agents::snapshot();
        assert!(now.serving && now.level == Level::Read && now.port != 0, "{}", now.problem);
        assert_eq!(kept_level(&s), Level::Read, "the choice is kept");
        h.get_by_label(format!("Serving on 127.0.0.1, port {}.", now.port).as_str());
        // The token is a secret: kept in a file, and neither reported nor drawn.
        let token = token();
        assert!(token.len() == 64 && token.chars().all(|c| c.is_ascii_hexdigit()));
        let report = h.state_mut().run("developer.agents", json!({})).unwrap();
        assert_eq!((&report["level"], &report["serving"], &report["port"]), (&json!("read"), &json!(true), &json!(now.port)));
        let values = serde_json::to_string(&h.state().dialog.as_ref().unwrap().values).unwrap();
        assert!(!values.contains(&token) && !report.to_string().contains(&token));
        assert!(!std::fs::read_to_string(s.0.join("developer.json")).unwrap().contains(&token));
        assert!(h.query_by_label_contains(&token[..12]).is_none(), "it is nowhere in the window");
        // An agent that holds it is answered, and one that doesn't is not.
        let (failed, text) = agent("list_modules", json!({}));
        assert!(!failed && text.starts_with("0 module(s)."), "{text}");
        assert_eq!(post(&"0".repeat(64), &json!({"jsonrpc": "2.0", "id": 1, "method": "tools/list"})).0, 401);
        h.run_steps(3);
        assert!(agents::snapshot().clients.is_empty(), "a client is known once it has said who it is");
        // Turning it off needs no question, and closes the door.
        choose(&mut h, Level::Off);
        assert_eq!(asked.questions().len(), 1);
        assert!(!agents::snapshot().serving);
        assert_eq!(kept_level(&s), Level::Off);
        let address = std::net::SocketAddr::from(([127, 0, 0, 1], now.port));
        assert!(TcpStream::connect_timeout(&address, Duration::from_millis(500)).is_err(), "nothing listens any more");
    }

    #[test]
    fn letting_agents_go_further_needs_the_operating_systems_yes() {
        let s = Sandbox::new("agents-raise");
        let asked = Answers::default();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        // The person says no to the operating system's question: nothing changes.
        choose(&mut h, Level::Ask);
        assert_eq!(asked.questions().len(), 1);
        assert_eq!(agents::snapshot().level, Level::Off);
        assert!(!agents::snapshot().serving && !s.0.join("agent-token").exists());
        h.get_by_label("Agent access was not changed.");
        asked.says(true);
        choose(&mut h, Level::Ask);
        assert_eq!((agents::snapshot().level, asked.questions().len()), (Level::Ask, 2));
        // Each step further asks again; a step back asks nothing.
        choose(&mut h, Level::Build);
        assert_eq!((agents::snapshot().level, asked.questions().len()), (Level::Build, 3));
        assert!(asked.questions()[2].contains("\"Build, and ask before running\""));
        choose(&mut h, Level::Read);
        assert_eq!((agents::snapshot().level, asked.questions().len()), (Level::Read, 3));
        assert_eq!(kept_level(&s), Level::Read);
        // A copy of GridCraft that can't put the operating system's question can't raise it.
        drop(h);
        agents::forget();
        let mut h = harness(app_with(None));
        assert_eq!(agents::snapshot().level, Level::Read, "what was chosen is how it starts");
        open(&mut h);
        choose(&mut h, Level::Ask);
        assert_eq!(message(&mut h).as_deref(), Some(agents::NO_CONFIRM));
        assert_eq!((agents::snapshot().level, kept_level(&s)), (Level::Read, Level::Read));
        // It can still lower it.
        choose(&mut h, Level::Off);
        assert_eq!(agents::snapshot().level, Level::Off);
    }

    #[test]
    fn the_server_starts_with_the_app_when_access_was_left_on() {
        let s = Sandbox::new("agents-start");
        std::fs::write(s.0.join("developer.json"), r#"{"python": "", "agents": {"level": "build", "port": 7980}}"#).unwrap();
        let h = harness(app_with(None));
        let now = agents::snapshot();
        assert!(now.serving && now.level == Level::Build, "{}", now.problem);
        let first = token();
        // The same token serves the next time: an agent set up once stays set up.
        drop(h);
        agents::forget();
        let h = harness(app_with(None));
        assert!(agents::snapshot().serving);
        assert_eq!(token(), first);
        // A settings file that says full control is not believed: that is never kept.
        drop(h);
        agents::forget();
        std::fs::write(s.0.join("developer.json"), r#"{"agents": {"level": "full"}}"#).unwrap();
        let _h = harness(app_with(None));
        assert_eq!(agents::snapshot().level, Level::Ask);
        // A token file that was emptied or damaged is replaced, not used.
        agents::forget();
        std::fs::write(s.0.join("agent-token"), "short").unwrap();
        let _h = harness(app_with(None));
        assert!(agents::snapshot().serving);
        assert_eq!(token().len(), 64);
        assert_ne!(token(), first);
    }

    #[test]
    fn a_request_is_looked_at_and_carried_out_on_the_persons_yes() {
        let s = Sandbox::new("agents-yes");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        trust(h.state(), "clean-data", list()["modules"][0]["fingerprint"].as_str().unwrap(), false).unwrap();
        open(&mut h);
        choose(&mut h, Level::Ask);
        h.state_mut().dialog = None;
        h.run_steps(2);
        let text = "def main(data):\n    return 'from the agent'\n";
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/extra.py", "text": text}));
        // Nothing was done. The person is told, in the bar under the ribbon.
        let file = s.0.join("modules/clean-data/python/extra.py");
        assert!(!file.exists());
        let notice = h.state().notice.clone().unwrap();
        assert_eq!(notice.text, "An agent is asking for your answer in Development (1 waiting).");
        assert_eq!(notice.action, Some(("Review…".to_string(), NAME.to_string())));
        assert_eq!(h.state_mut().run("developer.agents", json!({})).unwrap()["waiting"][0]["request"], id);
        // They look at it: all of it is there to read.
        click(&mut h, "Review…");
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(NAME));
        assert!(h.state().notice.is_none(), "the bar has said its piece");
        let title = format!("Write \"python/extra.py\" in clean-data ({} bytes)", text.len());
        h.get_by_label("Create file");
        h.get_by_label("clean-data/python/extra.py");
        h.get_by_label("The file is new:");
        h.get_by_label("Do you want to let the agent create \"python/extra.py\"?");
        h.get_by_label_contains("The agent gave no name.");
        assert!(h.query_by_label("Look…").is_none(), "opened from the bar, the window is on the request at once");
        h.get_by_label_contains("return 'from the agent'");
        h.get_by_label("clean-data stops being trusted when its files change: nothing in it runs until you review it again.");
        assert_eq!(asked.questions().len(), 2, "trusting, and the level: looking asks nothing");
        // And say yes, to the operating system's own question.
        say_yes(&mut h);
        let questions = asked.questions();
        assert_eq!(questions.len(), 3);
        for part in ["An agent asks GridCraft to:", title.as_str(), "stops being trusted", "Allow it, this once?"] {
            assert!(questions[2].contains(part), "{part}: {}", questions[2]);
        }
        assert_eq!(std::fs::read_to_string(&file).unwrap(), text);
        let (failed, said) = stands(id);
        assert!(!failed && said.contains("was carried out. python/extra.py was created in clean-data"), "{said}");
        assert!(said.contains("clean-data is no longer the module that was trusted"), "{said}");
        assert_eq!(list()["modules"][0]["status"], "changed", "what the agent wrote runs nowhere until the person looks again");
        assert_eq!(outcomes(&s), ["asked", "allowed", "done"]);
        assert!(agents::snapshot().waiting.is_empty());
        // The window is back to the whole of it, and says what happened.
        h.get_by_label_contains("python/extra.py was created in clean-data");
        h.get_by_role_and_label(Role::RadioButton, "Ask me each time");
    }

    #[test]
    fn refuse_is_a_no_and_the_agent_must_wait_before_asking_again() {
        let s = Sandbox::new("agents-no");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let id = asks(&mut h, "delete_module_file", json!({"module": "clean-data", "path": "python/clean.py"}));
        look(&mut h);
        click(&mut h, NO);
        assert_eq!(asked.questions().len(), 1, "a no needs no confirming");
        assert!(s.0.join("modules/clean-data/python/clean.py").exists());
        let (failed, said) = stands(id);
        assert!(failed && said.contains("was refused: The person at the screen said no."), "{said}");
        h.get_by_label("The request was refused.");
        // Whatever it asks next has to wait.
        let (failed, said) = agent("create_module", json!({"name": "Another"}));
        assert!(failed && said.contains("The person refused the last request. Wait"), "{said}");
        assert!(agents::snapshot().waiting.is_empty() && !s.0.join("modules/another").exists());
        assert_eq!(outcomes(&s), ["asked", "refused", "not allowed"]);
    }

    #[test]
    fn a_no_can_tell_the_agent_what_to_do_differently() {
        let s = Sandbox::new("agents-no-why");
        create("Clean Data").unwrap();
        let mut h = harness(app_with(Some(&yes())));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let id = asks(&mut h, "delete_module_file", json!({"module": "clean-data", "path": "python/clean.py"}));
        look(&mut h);
        h.get_by_label("Delete file");
        h.get_by_label("Do you want to let the agent delete \"python/clean.py\"?");
        // What the control channel types into the card is not the person's word.
        drive(&mut h);
        let mut state: State = h.ctx.data(|d| d.get_temp(state_id())).unwrap();
        assert_eq!(state.reason, "");
        quiet(&mut h);
        state = h.ctx.data(|d| d.get_temp(state_id())).unwrap();
        state.reason = "  Keep it:\trename it to tidy.py instead.  ".into();
        h.ctx.data_mut(|d| d.insert_temp(state_id(), state));
        click(&mut h, NO);
        let (failed, said) = stands(id);
        let told = "was refused: The person at the screen said no. They tell you: \"Keep it:rename it to tidy.py instead.\"";
        assert!(failed && said.contains(told), "{said}");
        h.get_by_label("The request was refused, and the agent was told why.");
        assert!(s.0.join("modules/clean-data/python/clean.py").exists());
        // The word is on the record too.
        let log = std::fs::read_to_string(s.0.join("agent-log.jsonl")).unwrap();
        assert!(log.lines().last().unwrap().contains("rename it to tidy.py instead"));
    }

    #[test]
    fn an_edit_is_shown_as_the_lines_it_takes_out_and_puts_in() {
        let s = Sandbox::new("agents-diff");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let file = s.0.join("modules/clean-data/python/clean.py");
        let old = std::fs::read_to_string(&file).unwrap();
        let line = old.lines().find(|line| line.contains("print(")).unwrap();
        let new = old.replacen(line, "    print('all done')", 1);
        assert_ne!(new, old);
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/clean.py", "text": new}));
        look(&mut h);
        h.get_by_label("Edit file");
        h.get_by_label("It changes the file that is there: 1 line in, 1 out.");
        h.get_by_label("Do you want to let the agent make this edit to \"python/clean.py\"?");
        // The block holds the change, with a little around it, and not the whole file.
        let first = old.lines().next().unwrap();
        h.get_by_label_contains("+     print('all done')");
        h.get_by_label_contains(format!("- {line}").as_str());
        h.get_by_label_contains("the same");
        assert!(h.query_by_label_contains(first).is_none(), "what doesn't change, far from the change, isn't shown");
        // The whole of what would be written is there to read too.
        click(&mut h, "Show whole file");
        h.get_by_label("It replaces the file that is there with this:");
        h.get_by_label_contains("print('all done')");
        h.get_by_label_contains(first);
        assert!(h.query_by_label_contains(line).is_none(), "the line that would go is not in what would be written");
        click(&mut h, "Show changes");
        h.get_by_label("It changes the file that is there: 1 line in, 1 out.");
        say_yes(&mut h);
        assert_eq!(std::fs::read_to_string(&file).unwrap(), new);
        assert!(stands(id).1.contains("python/clean.py was written over in clean-data"));
    }

    #[test]
    fn the_card_under_the_persons_eyes_is_the_one_they_chose_and_names_what_it_holds() {
        let s = Sandbox::new("agents-which");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        h.state_mut().dialog = None;
        h.run_steps(2);
        // Two requests that read alike: the same file, the same length, another text.
        let benign = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/clean.py", "text": "x = 'kind'\n"}));
        let other = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/clean.py", "text": "x = 'evil'\n"}));
        let waiting = agents::snapshot().waiting;
        assert_eq!(waiting[0].action.title(), waiting[1].action.title(), "they are told apart by nothing in their titles");
        let marks: Vec<String> = waiting.iter().map(|asked| gridcraft_aniefork_dev::agents::mark(&asked.action)).collect();
        assert!(marks[0].len() == 8 && marks[0] != marks[1]);
        // With several waiting the window opens on the list: opening it chooses nothing.
        open(&mut h);
        assert!(h.query_by_label(YES).is_none());
        assert_eq!(h.get_all_by_label("Look…").count(), 2);
        h.get_by_label(format!("request #{benign}  ·  marked {}  ·  lapses in 20 min", marks[0]).as_str());
        h.get_by_label(format!("request #{other}  ·  marked {}  ·  lapses in 20 min", marks[1]).as_str());
        // Closing the window and opening it again (which the control channel can do) leaves the
        // choice with the person.
        let ctx = h.ctx.clone();
        let _ = crate::control::handle(h.state_mut(), &ctx, "ui.dialog.cancel", &json!({}));
        let _ = crate::control::handle(h.state_mut(), &ctx, "ui.dialog", &json!({"name": NAME}));
        h.run_steps(6);
        assert!(h.query_by_label(YES).is_none(), "no card came on screen by itself");
        // The person chooses the second, reads it, and says yes: the question the operating
        // system puts is about that one, by its number and by the mark on its card.
        h.get_all_by_label("Look…").nth(1).unwrap().click();
        h.run_steps(4);
        h.get_by_label(format!("request #{other}  ·  marked {}  ·  lapses in 20 min", marks[1]).as_str());
        h.get_by_label_contains("x = 'evil'");
        say_yes(&mut h);
        let question = asked.questions().pop().unwrap();
        assert!(question.contains(&format!("This is request #{other}, holding what its card marks {}.", marks[1])), "{question}");
        assert_eq!(std::fs::read_to_string(s.0.join("modules/clean-data/python/clean.py")).unwrap(), "x = 'evil'\n");
        // The one that still waits is the next card: the person's own answer brought it.
        h.get_by_label(format!("request #{benign}  ·  marked {}  ·  lapses in 20 min", marks[0]).as_str());
        h.get_by_label_contains("x = 'kind'");
    }

    #[test]
    fn a_request_too_long_to_show_whole_cannot_be_said_yes_to() {
        let s = Sandbox::new("agents-long");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let long = format!("{}# the end\n", "x = 1\n".repeat(SHOWN / 6 + 1));
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/long.py", "text": long}));
        look(&mut h);
        h.get_by_label_contains("so it can't be allowed here");
        h.get_by_label_contains("Only the first 200000 characters are shown");
        say_yes(&mut h);
        assert_eq!(asked.questions().len(), 1, "the operating system's question isn't raised");
        assert!(stands(id).1.contains("is waiting for the person") && !s.0.join("modules/clean-data/python/long.py").exists());
        click(&mut h, NO);
        assert!(stands(id).1.contains("was refused"));
    }

    #[test]
    fn the_control_channel_reaches_nothing_in_the_settings_folder() {
        let s = Sandbox::new("agents-reach");
        create("Clean Data").unwrap();
        let mut h = harness(app_with(Some(&yes())));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let token = token();
        let ctx = h.ctx.clone();
        let sent = |h: &mut Harness, method: &str, params: Json| match crate::control::handle(h.state_mut(), &ctx, method, &params) {
            crate::control::Outcome::Done(reply) => reply,
            _ => Json::Null,
        };
        let inside = |name: &str| s.0.join(name).display().to_string();
        let script = s.0.join("modules/clean-data/python/clean.py");
        let before = std::fs::read_to_string(&script).unwrap();
        // Reading: the token, the decisions, a module's files.
        for name in ["agent-token", "trusted-modules.json", "developer.json", "modules/clean-data/python/clean.py"] {
            for (method, params) in [
                ("app.open", json!({"path": inside(name)})),
                ("engine.execute", json!({"command": "file.open", "params": {"path": inside(name)}})),
                ("command", json!({"id": "file.open", "params": {"path": inside(name)}})),
            ] {
                let reply = sent(&mut h, method, params);
                assert_eq!(reply["ok"], false, "{method} {name}: {reply}");
                assert!(reply["error"].as_str().unwrap().contains("can't be read or written through the control channel"), "{reply}");
            }
        }
        let sheet = sent(&mut h, "engine.execute", json!({"command": "sheet.read", "params": {}}));
        assert!(!sheet.to_string().contains(&token), "the token is in no cell");
        // Writing: over the token, the decisions, the settings, a module's script; by any spelling.
        let _ = sent(&mut h, "engine.execute", json!({"command": "cell.set", "params": {"cell": "A1", "input": "0".repeat(64)}}));
        let spelled = format!("{}/modules/../agent-token", s.0.display());
        for path in [
            inside("agent-token"),
            inside("trusted-modules.json"),
            inside("developer.json"),
            script.display().to_string(),
            inside("new.csv"),
            spelled,
        ] {
            for (method, params) in [
                ("app.save", json!({"path": path})),
                ("engine.execute", json!({"command": "file.saveAs", "params": {"path": path}})),
                ("ui.screenshot", json!({"path": path})),
            ] {
                let reply = sent(&mut h, method, params);
                assert_eq!(reply["ok"], false, "{method} {path}: {reply}");
            }
        }
        assert_eq!(self::token(), token, "the token is as it was");
        assert_eq!(std::fs::read_to_string(&script).unwrap(), before);
        assert!(!s.0.join("new.csv").exists());
        assert_eq!(agents::snapshot().level, Level::Ask);
        // Anywhere else is the channel's to use as before.
        let outside = std::env::temp_dir().join(format!("gridcraft-reach-{}.csv", std::process::id()));
        let reply = sent(&mut h, "app.save", json!({"path": outside.display().to_string()}));
        assert_eq!(reply["ok"], true, "{reply}");
        let _ = std::fs::remove_file(&outside);
    }

    #[test]
    fn a_card_cannot_be_said_yes_to_in_its_first_moment_on_screen() {
        let s = Sandbox::new("agents-settle");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let id = asks(&mut h, "delete_module_file", json!({"module": "clean-data", "path": "python/clean.py"}));
        look(&mut h);
        // The card has only just come on screen (here: for a long while yet).
        let mut state: State = h.ctx.data(|d| d.get_temp(state_id())).unwrap();
        state.shown_at = crate::now_ms() + 60_000.0;
        h.ctx.data_mut(|d| d.insert_temp(state_id(), state));
        click(&mut h, YES);
        assert_eq!(asked.questions().len(), 1, "the operating system's question isn't raised");
        assert!(stands(id).1.contains("is waiting for the person") && s.0.join("modules/clean-data/python/clean.py").exists());
        // No is never held back.
        click(&mut h, NO);
        assert!(stands(id).1.contains("was refused"));
    }

    #[test]
    fn a_no_to_the_operating_systems_question_is_a_no() {
        let s = Sandbox::new("agents-os-no");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/clean.py", "text": "changed"}));
        look(&mut h);
        h.get_by_label("Edit file");
        h.get_by_label_contains("It changes the file that is there:");
        // Not now leaves it waiting.
        click(&mut h, "Not now");
        assert_eq!(agents::snapshot().waiting.len(), 1);
        look(&mut h);
        asked.says(false);
        say_yes(&mut h);
        assert_eq!(asked.questions().len(), 2, "the operating system asked");
        assert!(stands(id).1.contains("was refused: The person at the screen said no."));
        assert_ne!(std::fs::read_to_string(s.0.join("modules/clean-data/python/clean.py")).unwrap(), "changed");
        // A copy of GridCraft that can't put that question carries nothing out for an agent.
        drop(h);
        agents::forget();
        let mut h = harness(app_with(None));
        std::thread::sleep(Duration::from_millis(50));
        let id = asks(&mut h, "stop_trusting_module", json!({"module": "clean-data"}));
        open(&mut h);
        look(&mut h);
        say_yes(&mut h);
        assert_eq!(message(&mut h).as_deref(), Some(agents::NO_CONFIRM));
        assert!(stands(id).1.contains("is waiting for the person"), "it still waits, for a refusal");
    }

    #[test]
    fn the_control_channel_cannot_set_the_level_hand_out_the_setup_or_answer_a_request() {
        let s = Sandbox::new("agents-driven");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        // While the channel is putting input into the window, the level is not its to set.
        drive(&mut h);
        choose(&mut h, Level::Full);
        assert_eq!(message(&mut h).as_deref(), Some(presence::BY_HAND));
        assert!(asked.questions().is_empty(), "the operating system's question isn't even raised");
        assert_eq!((agents::snapshot().level, agents::snapshot().serving), (Level::Off, false));
        assert!(!s.0.join("agent-token").exists());
        quiet(&mut h);
        choose(&mut h, Level::Ask);
        assert_eq!(agents::snapshot().level, Level::Ask);
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/clean.py", "text": "changed"}));
        let (token_before, port) = (token(), agents::snapshot().port);
        drive(&mut h);
        // Which request is under the person's eyes is theirs to choose: the channel can't turn
        // the window to one.
        click(&mut h, "Look…");
        assert_eq!(message(&mut h).as_deref(), Some(presence::BY_HAND));
        assert!(h.query_by_label(YES).is_none(), "no card came on screen");
        quiet(&mut h);
        look(&mut h);
        h.get_by_label_contains("It changes the file that is there:");
        std::thread::sleep(Duration::from_millis(SETTLE_MS as u64 + 40));
        // Nor can it answer the one they are looking at, or take its card away.
        drive(&mut h);
        for label in [YES, NO, NOT_NOW, "Show whole file"] {
            click(&mut h, label);
            assert_eq!(message(&mut h).as_deref(), Some(presence::BY_HAND), "{label}");
            assert_eq!(agents::snapshot().waiting.len(), 1, "{label}: it still waits");
            h.get_by_label_contains("It changes the file that is there:");
        }
        assert_eq!(asked.questions().len(), 1, "only the level was ever asked about");
        assert_ne!(std::fs::read_to_string(s.0.join("modules/clean-data/python/clean.py")).unwrap(), "changed");
        // Nor is the setup, which holds the token, the token itself, or the port.
        quiet(&mut h);
        click(&mut h, "Not now");
        drive(&mut h);
        for label in ["Copy Setup for Claude Code", "Copy Setup as JSON", "New Token", "Set Port"] {
            h.run_steps(3);
            h.get_by_label(label).click();
            let copied = (0..5).any(|_| {
                h.step();
                h.output().platform_output.commands.iter().any(|command| matches!(command, egui::OutputCommand::CopyText(_)))
            });
            assert!(!copied, "{label}: nothing reached the clipboard");
            assert_eq!(message(&mut h).as_deref(), Some(presence::BY_HAND), "{label}");
        }
        assert_eq!((token(), agents::snapshot().port), (token_before.clone(), port));
        // What the channel writes into the dialog's values changes nothing the window works from.
        let ctx = h.ctx.clone();
        for (field, value) in [("level", json!("full")), ("looking", json!(id)), ("serving", json!(false))] {
            let _ = crate::control::handle(h.state_mut(), &ctx, "ui.dialog.set", &json!({"field": field, "value": value}));
        }
        let _ = crate::control::handle(h.state_mut(), &ctx, "ui.dialog.confirm", &json!({}));
        h.run_steps(3);
        assert_eq!((agents::snapshot().level, agents::snapshot().waiting.len()), (Level::Ask, 1));
        assert_eq!(h.state().dialog.as_ref().unwrap().values["level"], "ask", "the values only report");
        // And no command sets any of it, whatever it is handed.
        for params in [json!({"level": "full"}), json!({"port": 1}), json!({"allow": id}), json!({"token": "x"})] {
            assert_eq!(h.state_mut().run("developer.agents", params).unwrap()["level"], "ask");
        }
        let about_agents = |c: &Json| ["agent", "token", "development"].iter().any(|word| c["id"].as_str().unwrap().to_lowercase().contains(word));
        let ids: Vec<String> = crate::aniefork::commands().filter(about_agents).map(|c| c["id"].as_str().unwrap().to_string()).collect();
        assert_eq!(ids, ["developer.agents"], "the one command there is only reports");
        assert_eq!((agents::snapshot().level, agents::snapshot().waiting.len(), token()), (Level::Ask, 1, token_before));
        // The person, once the channel is quiet, answers as before.
        quiet(&mut h);
        look(&mut h);
        click(&mut h, NO);
        assert!(stands(id).1.contains("was refused"));
    }

    #[test]
    fn the_setup_is_handed_to_the_person_and_a_new_token_locks_the_old_one_out() {
        let _s = Sandbox::new("agents-setup");
        let mut h = harness(app_with(Some(&yes())));
        open(&mut h);
        choose(&mut h, Level::Read);
        let (first, port) = (token(), agents::snapshot().port);
        let copied = |h: &mut Harness, label: &str| {
            h.run_steps(3);
            h.get_by_label(label).click();
            (0..5)
                .find_map(|_| {
                    h.step();
                    h.output().platform_output.commands.iter().find_map(|command| match command {
                        egui::OutputCommand::CopyText(text) => Some(text.clone()),
                        _ => None,
                    })
                })
                .unwrap_or_else(|| panic!("{label} copied nothing"))
        };
        let command = copied(&mut h, "Copy Setup for Claude Code");
        assert_eq!(
            command,
            format!("claude mcp add --transport http gridcraft-development http://127.0.0.1:{port}/mcp --header \"Authorization: Bearer {first}\"")
        );
        h.run_steps(2);
        h.get_by_label(COPIED);
        let entry: Json = serde_json::from_str(&copied(&mut h, "Copy Setup as JSON")).unwrap();
        let server = &entry["mcpServers"]["gridcraft-development"];
        assert_eq!((server["type"].as_str(), server["url"].as_str()), (Some("http"), Some(format!("http://127.0.0.1:{port}/mcp").as_str())));
        assert_eq!(server["headers"]["Authorization"], format!("Bearer {first}"));
        // What was copied connects.
        let hello = json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "clientInfo": {"name": "claude-code", "version": "2.1.0"}}});
        assert_eq!(post(&first, &hello).0, 200);
        h.run_steps(3);
        h.get_by_label_contains("Last heard from: claude-code 2.1.0");
        assert_eq!(h.state_mut().run("developer.agents", json!({})).unwrap()["clients"][0]["name"], "claude-code 2.1.0");
        // A new token: whoever holds the old one is locked out.
        click(&mut h, "New Token");
        let second = token();
        assert_ne!(second, first);
        assert!(agents::snapshot().serving, "{}", agents::snapshot().problem);
        assert_eq!(post(&first, &hello).0, 401);
        assert_eq!(post(&second, &hello).0, 200);
        // The port is the person's to choose; one that can't be is refused, and nothing moves.
        let mut state: State = h.ctx.data(|d| d.get_temp(state_id())).unwrap();
        state.port = "80".into();
        h.ctx.data_mut(|d| d.insert_temp(state_id(), state));
        click(&mut h, "Set Port");
        assert_eq!(message(&mut h).as_deref(), Some("Choose a port from 1024 to 65535."));
        assert!(agents::snapshot().serving);
    }

    #[test]
    fn with_build_an_agent_writes_without_asking_and_running_still_asks() {
        let s = Sandbox::new("agents-build");
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Build);
        h.state_mut().dialog = None;
        h.run_steps(2);
        // It creates a module and writes its script: nobody is asked.
        let id = asks(&mut h, "create_module", json!({"name": "Tidy"}));
        let (failed, said) = stands(id);
        assert!(!failed && said.contains("Tidy was created as the module tidy, with 3 files. It is not trusted"), "{said}");
        let id = asks(&mut h, "write_module_file", json!({"module": "tidy", "path": "python/tidy.py", "text": SCRIPT}));
        let (failed, said) = stands(id);
        assert!(!failed && said.contains("python/tidy.py was created in tidy") && said.contains("tidy is not trusted"), "{said}");
        assert_eq!(std::fs::read_to_string(s.0.join("modules/tidy/python/tidy.py")).unwrap(), SCRIPT);
        assert_eq!(asked.questions().len(), 1, "only the level was asked about");
        assert!(h.state().notice.is_none() && agents::snapshot().waiting.is_empty());
        // What it wrote runs nowhere: the module isn't trusted.
        let (failed, said) = agent("run_script", json!({"module": "tidy", "script": "tidy.py"}));
        assert!(failed && said.contains("tidy is not trusted"), "{said}");
        // Trusting it is the person's to allow, and they see what it would be allowed.
        let id = asks(&mut h, "trust_module", json!({"module": "tidy"}));
        assert!(h.state().notice.as_ref().is_some_and(|notice| notice.text.contains("1 waiting")));
        open(&mut h);
        look(&mut h);
        h.get_by_label("It would be allowed to:");
        h.get_by_label_contains("Run its Python scripts. This cannot be limited");
        say_yes(&mut h);
        let question = asked.questions().pop().unwrap();
        for part in [
            "An agent asks you to trust a module (request #",
            "Trust \"Tidy\"",
            "Run its Python scripts",
            "If any of them changes, GridCraft will ask you again.",
        ] {
            assert!(question.contains(part), "{part}: {question}");
        }
        assert!(stands(id).1.contains("was carried out. tidy is trusted as it is now."));
        assert_eq!(list()["modules"][0]["status"], "trusted");
        assert_eq!(list()["modules"][0]["followsChanges"], false, "never \"as it changes\": that is the person's alone to choose");
        // Running it asks too, and what the script gave comes back as data.
        if let Err(why) = PythonHost::global().interpreter("") {
            eprintln!("skipped the run: {why}");
            return;
        }
        let id = asks(&mut h, "run_script", json!({"module": "tidy", "script": "tidy.py", "data": {"rows": [["  a ", "b  "]]}}));
        look(&mut h);
        h.get_by_label("main(data) is handed this:");
        h.get_by_label("A script can do anything a program you run can do.");
        say_yes(&mut h);
        assert!(asked.questions().pop().unwrap().contains("Run \"tidy.py\" of tidy"));
        let (failed, said) = settled(&mut h, id);
        assert!(!failed && said.contains("was carried out. tidy.py of tidy returned."), "{said}");
        let (_, rest) = said.split_once("<untrusted-data-").unwrap();
        let data: Json = serde_json::from_str(rest.split_once(">\n").unwrap().1.rsplit_once("\n</untrusted-data-").unwrap().0).unwrap();
        assert_eq!(data["result"], json!({"rows": [["a", "b"]]}));
        assert_eq!(data["printed"].as_str().map(str::trim), Some("tidying"));
        PythonHost::global().stop(&s.0.join("modules"), "tidy");
        // A module the person keeps trusting as it changes: writing to it is running code, and asks.
        trust(h.state(), "tidy", list()["modules"][0]["fingerprint"].as_str().unwrap(), true).unwrap();
        let id =
            asks(&mut h, "write_module_file", json!({"module": "tidy", "path": "python/tidy.py", "text": "def main(data):\n    return 'swapped'\n"}));
        assert!(stands(id).1.contains("is waiting for the person"));
        assert_eq!(std::fs::read_to_string(s.0.join("modules/tidy/python/tidy.py")).unwrap(), SCRIPT);
        look(&mut h);
        h.get_by_label("You chose to keep trusting tidy as its files change: what is written runs without your looking at it again.");
    }

    #[test]
    fn at_build_a_change_nobody_looked_at_ends_the_trust_in_the_module_whatever_it_holds_after() {
        let s = Sandbox::new("agents-swap");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        trust(h.state(), "clean-data", list()["modules"][0]["fingerprint"].as_str().unwrap(), false).unwrap();
        open(&mut h);
        choose(&mut h, Level::Build);
        let file = s.0.join("modules/clean-data/python/clean.py");
        let original = std::fs::read_to_string(&file).unwrap();
        let fingerprint = list()["modules"][0]["fingerprint"].clone();
        // The agent swaps the script, and puts the old one back: the files are as they were
        // trusted, to the byte.
        let id = asks(
            &mut h,
            "write_module_file",
            json!({"module": "clean-data", "path": "python/clean.py", "text": "def main(data):\n    return 'swapped'\n"}),
        );
        assert!(stands(id).1.contains("clean-data is not trusted"), "{}", stands(id).1);
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/clean.py", "text": original}));
        assert!(stands(id).1.contains("was carried out"));
        assert_eq!(list()["modules"][0]["fingerprint"], fingerprint);
        // The trust is gone all the same: it was in what the person looked at, and nobody
        // looked at what was there in between.
        assert_eq!(list()["modules"][0]["status"], "untrusted");
        let (failed, said) = agent("run_script", json!({"module": "clean-data", "script": "clean.py"}));
        assert!(failed && said.contains("is not trusted"), "{said}");
        assert_eq!(asked.questions().len(), 2, "trusting, and the level: the changes asked nothing");
        // A module the person keeps trusting as it changes: taking its trust back would end
        // that choice and leave its files free to change, so it asks, as changing them does.
        trust(h.state(), "clean-data", list()["modules"][0]["fingerprint"].as_str().unwrap(), true).unwrap();
        let id = asks(&mut h, "stop_trusting_module", json!({"module": "clean-data"}));
        assert!(stands(id).1.contains("is waiting for the person"));
        assert_eq!(list()["modules"][0]["status"], "trusted");
        look(&mut h);
        h.get_by_label("Stop trusting module");
        click(&mut h, NO);
        assert_eq!(list()["modules"][0]["followsChanges"], true);
    }

    #[test]
    fn with_full_control_nothing_asks_until_gridcraft_closes() {
        let s = Sandbox::new("agents-full");
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        choose(&mut h, Level::Full);
        let questions = asked.questions();
        assert_eq!(questions.len(), 2);
        assert!(
            questions[1].contains("\"Full control\"") && questions[1].contains("the same as letting it run any program as you"),
            "{}",
            questions[1]
        );
        assert_eq!(agents::snapshot().level, Level::Full);
        assert_eq!(kept_level(&s), Level::Ask, "full control is never kept");
        assert_eq!(h.state_mut().run("developer.agents", json!({})).unwrap()["level"], "full");
        // The window says so for as long as it lasts.
        let notice = h.state().notice.clone().unwrap();
        assert_eq!(
            (notice.text.as_str(), notice.kind),
            ("Agents have full control of your development tools until GridCraft closes.", crate::NoticeKind::Warning)
        );
        h.state_mut().dialog = None;
        h.run_steps(2);
        // It builds a module, trusts it and runs it: nobody is asked.
        for (tool, arguments, part) in [
            ("create_module", json!({"name": "Tidy"}), "Tidy was created as the module tidy"),
            ("write_module_file", json!({"module": "tidy", "path": "python/tidy.py", "text": SCRIPT}), "python/tidy.py was created in tidy"),
            ("trust_module", json!({"module": "tidy"}), "tidy is trusted as it is now."),
        ] {
            let id = asks(&mut h, tool, arguments);
            let (failed, said) = stands(id);
            assert!(!failed && said.contains(part), "{tool}: {said}");
        }
        assert_eq!(list()["modules"][0]["status"], "trusted");
        if PythonHost::global().interpreter("").is_ok() {
            let id = asks(&mut h, "run_script", json!({"module": "tidy", "script": "tidy.py", "data": {"rows": [[" x "]]}}));
            let (failed, said) = settled(&mut h, id);
            assert!(!failed && said.contains("tidy.py of tidy returned.") && said.contains("\"x\""), "{said}");
            PythonHost::global().stop(&s.0.join("modules"), "tidy");
        }
        assert_eq!(asked.questions().len(), 2, "nothing more was asked");
        assert!(agents::snapshot().waiting.is_empty());
        assert!(outcomes(&s).iter().all(|outcome| outcome == "allowed" || outcome == "done"), "{:?}", outcomes(&s));
        // Each thing it did is on the record.
        let recent = agents::snapshot().recent;
        assert!(recent.iter().any(|line| line.what == "Trust tidy" && line.detail == "without asking, by the level set"));
        // GridCraft closes and starts again: agents are back to what was chosen before.
        drop(h);
        agents::forget();
        let _h = harness(app_with(Some(&asked)));
        assert_eq!(agents::snapshot().level, Level::Ask);
    }

    #[test]
    fn trust_asked_for_a_module_that_then_changed_is_not_given() {
        let s = Sandbox::new("agents-trust-changed");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let id = asks(&mut h, "trust_module", json!({"module": "clean-data"}));
        // A file changes after the agent asked, before the person answers.
        std::fs::write(s.0.join("modules/clean-data/python/clean.py"), "def main(data):\n    return 'swapped'\n").unwrap();
        look(&mut h);
        h.get_by_label("The module changed after the agent asked. It has to ask again: this request can't be allowed.");
        say_yes(&mut h);
        assert!(message(&mut h).unwrap().contains("clean-data changed after the agent asked"));
        assert_eq!(asked.questions().len(), 1, "the person is not asked about files they didn't see");
        assert_eq!(list()["modules"][0]["status"], "untrusted");
        let (failed, said) = stands(id);
        assert!(failed && said.contains("was refused: The module's files changed after this was asked."), "{said}");
        // That was nobody's no: the agent may ask again at once, for the module as it is now.
        let again = asks(&mut h, "trust_module", json!({"module": "clean-data"}));
        assert_ne!(again, id);
        look(&mut h);
        say_yes(&mut h);
        assert_eq!(list()["modules"][0]["status"], "trusted");
    }

    #[test]
    fn lowering_the_level_refuses_what_was_waiting() {
        let _s = Sandbox::new("agents-lowered");
        create("Clean Data").unwrap();
        let mut h = harness(app_with(Some(&yes())));
        open(&mut h);
        choose(&mut h, Level::Ask);
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/clean.py", "text": "changed"}));
        choose(&mut h, Level::Read);
        let (failed, said) = stands(id);
        assert!(failed && said.contains("was refused: Agent access was lowered."), "{said}");
        assert!(agents::snapshot().waiting.is_empty());
        // That counts against nobody, and at this level there is nothing to ask for anyway.
        let (failed, said) = agent("write_module_file", json!({"module": "clean-data", "path": "python/clean.py", "text": "changed"}));
        assert!(failed && said.starts_with("Not allowed:") && said.contains("\"Read only\""), "{said}");
        // Turning agents off while something waits refuses it too.
        choose(&mut h, Level::Ask);
        let id = asks(&mut h, "stop_trusting_module", json!({"module": "clean-data"}));
        assert_eq!(agents::snapshot().waiting[0].id, id);
        choose(&mut h, Level::Off);
        assert!(!agents::snapshot().serving);
    }

    #[test]
    fn the_window_gathers_the_tools() {
        let _s = Sandbox::new("agents-tools");
        create("Clean Data").unwrap();
        let mut h = harness(app_with(Some(&yes())));
        open(&mut h);
        h.get_by_label(INTRO);
        h.get_by_label("1 installed, 0 trusted");
        h.get_by_label_contains("Python: found automatically.");
        click(&mut h, "Modules…");
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(modules::NAME));
        open(&mut h);
        click(&mut h, "Console…");
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(crate::aniefork::console::NAME));
        // The Developer tab's button, and the bar under the ribbon, open it by this name. (The
        // ribbon's buttons carry no names a test can click them by.)
        assert!(crate::aniefork::draws_dialog(NAME));
        h.state_mut().dialog = None;
        h.state_mut().ui.aniefork.developer_tab = true;
        h.state_mut().ui.ribbon_tab = crate::aniefork::developer::TAB.into();
        h.run_steps(3);
        assert_eq!(h.state().ui.ribbon_tab, crate::aniefork::developer::TAB);
    }

    #[test]
    fn a_run_without_settings_has_no_development_server() {
        agents::forget();
        let mut h = harness(app_with(Some(&yes())));
        assert_eq!(h.state_mut().run("developer.agents", json!({})).unwrap()["available"], false);
        open(&mut h);
        h.get_by_label("Agents can't connect in this run, because it keeps no settings.");
        assert!(h.query_by_role_and_label(Role::RadioButton, "Full control").is_none());
        assert!(!agents::snapshot().serving);
    }
}
