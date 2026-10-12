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
//! would take out and put in), what it means, and then yes, yes and don't ask again (for that
//! kind of thing in that module, until GridCraft closes), or no with a word for the agent on
//! what to do differently. When GridCraft runs with the control channel a yes is confirmed by
//! the operating system's own question before anything is done; without it, nothing but the
//! person can click here, and the yes on the card is enough. A yes can't be pressed in the
//! card's first moment on screen, so that a
//! click meant for what was there before answers nothing. **What is on the card is what is
//! answered**: only the person's own click puts a request's card on screen or takes it off
//! (the window opens on a card by itself only when a single request waits, so there is no
//! other it could have been), the card and the operating system's question name the request
//! by its number and by a mark of what it holds, and a request too long to show whole can't
//! be said yes to.
//!
//! **A plan is answered on one card**: its purpose, then every step whole, each as it will find
//! things once the steps before it are done (a file written twice is compared with what the
//! earlier step wrote), and what in it trusts a module or runs code. A yes approves it and does
//! nothing else; the window then shows the plan with where each step stands, and a way to stop
//! it.
//!
//! **The window has a list down its side and one part beside it** ([`super::kits`] says what
//! each holds). At the top, the whole at a glance. Under it the **MCP server**, which is the
//! Agents part described above: it stands at the top because it is not any kit's. Then the
//! **development kits**: the one this build has is the Anie Development Kit (Python and
//! Svelte), and under it its **projects**, by kind: the code of one workbook, and libraries.
//! The kit's part has the way to Modules and the Developer console, the Python that runs
//! scripts, a new project, and a showcase: a project whose panel is a Svelte app, which the
//! window creates and, once the person has trusted it, docks beside the sheet in the system's
//! web view ([`super::dock`]).
//!
//! **What the window says is not worded here.** `card.rs` says what a request's card holds,
//! as blocks, and `kits.rs` the rest; this window draws them. Where the shell can show a web
//! page in a window of its own, the same is drawn there instead (`devpage.rs`), and what is
//! clicked there comes through [`act`] like a click here.
//!
//! The control channel can click and type in this window as a person does, and rewrite a
//! dialog's values. So the window works from a state of its own that the channel can't reach,
//! and everything that decides something (the level, the port, the token, the setup, an answer
//! to a request) is refused while the channel is putting input into the window; see
//! [`super::presence`].

use std::sync::Arc;

use gridcraft_aniefork_dev::agents::{self as dev, Action, Asked, Kind, Level, Sight};
use gridcraft_aniefork_dev::bridge::Places;
use gridcraft_aniefork_dev::{manifest, settings, showcase, starter};
use serde_json::{Value as Json, json};

use super::card::{self, Block, NO, NO_OF_THREE, NOT_NOW, REASON, YES, YES_PLAN};
use super::diff::{Line, Row};
use super::kits::{self, Pane};
use super::{agents, modules, presence};
use crate::SheetApp;
use crate::dialogs::Dialog;
use crate::theme::Tokens;

/// The dialog's name, as `ui.dialog {name}` takes it.
pub const NAME: &str = "development";

const COPIED: &str = "The setup was copied. It holds the token, which is a secret: paste it only into your agent's own settings.";
/// How long a request's card is on screen before its yes can be pressed (milliseconds): a click
/// meant for what was there a moment ago must not answer it.
pub(super) const SETTLE_MS: f64 = if cfg!(test) { 60.0 } else { 900.0 };

/// What the person asked for in one frame; carried out after the window is drawn.
pub(super) enum Act {
    Level(Level),
    Port(String),
    CopySetup {
        as_json: bool,
    },
    NewToken,
    Look(u64),
    NotNow,
    Allow(u64),
    /// Yes, and not to ask again for this kind of thing in this module until GridCraft closes.
    AllowNotAgain(u64),
    Refuse(u64),
    /// Ask again for this kind of thing in this module.
    AskAgain(String, Kind),
    /// Stop a plan that was approved.
    StopPlan(u64),
    /// Show the whole of the file that would be written, or only what changes.
    Whole(bool),
    /// Show another part of the window.
    Go(Pane),
    Open(&'static str),
    OpenFolder,
    /// Open the folder of one project.
    OpenProjectFolder(String),
    /// Create a starter project: the code of the workbook that is open, or a library.
    CreateProject {
        name: String,
        for_workbook: bool,
    },
    /// Create the Svelte showcase module (untrusted, like any new module).
    CreateShowcase,
    /// Show the showcase's panel: docked in the window, or in the browser.
    ShowShowcase {
        docked: bool,
    },
}

/// What the window works from, kept between frames. It is the window's own, out of the dialog's
/// values, which anything on the control channel can rewrite: the values only report.
#[derive(Clone, Default)]
pub(super) struct State {
    /// The last frame the window was drawn in: after a gap it was closed, and starts afresh.
    seen: u64,
    pub(super) port: String,
    /// The request the person is looking at.
    pub(super) looking: Option<u64>,
    /// When its card came on screen (`now_ms`).
    pub(super) shown_at: f64,
    /// What the person would tell the agent with a no.
    pub(super) reason: String,
    /// Show the whole of the file that would be written, not only what changes.
    pub(super) whole: bool,
    pub(super) said: String,
    /// The plan being looked at, followed through: for which request, when, and what was found.
    followed: Option<(u64, f64, Arc<Result<Vec<Sight>, String>>)>,
    /// The part of the window that is shown.
    pub(super) pane: Pane,
    /// The name being typed for a new project, and whether it is to be the open workbook's code.
    new_name: String,
    new_for: bool,
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
    pub(super) fn fresh() -> State {
        let kept = modules::places().map(|at| settings::load(&at.settings).agents.port).unwrap_or_default();
        let mut state = State { port: kept.to_string(), ..State::default() };
        let waiting = agents::snapshot().waiting;
        let only = match waiting.as_slice() {
            [only] => Some(only.id),
            _ => None,
        };
        // While something waits for an answer, the window opens where the answers are given.
        if !waiting.is_empty() {
            state.pane = Pane::Server;
        }
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

    /// A plan's steps followed through, for its card: worked out afresh once a second, not
    /// once a frame.
    pub(super) fn follow(&mut self, at: &Places, id: u64, steps: &[Action], stood: &[(String, Option<String>)]) -> Arc<Result<Vec<Sight>, String>> {
        let now = crate::now_ms();
        if let Some((of, at_ms, kept)) = &self.followed
            && *of == id
            && (0.0..1000.0).contains(&(now - at_ms))
        {
            return kept.clone();
        }
        // Whether it can be approved as things are: its modules as they stood when it was
        // proposed, and its steps fitting one another.
        let fresh = Arc::new(dev::plan_fits(at, steps, stood));
        self.followed = Some((id, now, fresh.clone()));
        fresh
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
            ("part", json!(self.pane.code())),
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
    let (shown, cut) = card::shown_text(text);
    let mut job = egui::text::LayoutJob::default();
    for (index, line) in shown.lines().enumerate() {
        if numbered {
            job.append(&format!("{:>4}  ", index + 1), 0.0, egui::TextFormat { font_id: font.clone(), color: t.text_dim, ..Default::default() });
        }
        job.append(&format!("{}\n", card::visible(line)), 0.0, egui::TextFormat { font_id: font.clone(), color: t.text, ..Default::default() });
    }
    block(ui, salt, job);
    if let Some(cut) = cut {
        let warn = ui.visuals().warn_fg_color;
        ui.label(egui::RichText::new(cut).color(warn).strong());
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
    let (rows, cut) = card::shown_rows(compared);
    let mut job = egui::text::LayoutJob::default();
    for row in &rows {
        match row {
            Row::Skipped(count) => {
                job.append(&format!("       ⋯ {count} line{} the same\n", if *count == 1 { "" } else { "s" }), 0.0, plain(t.text_dim))
            }
            Row::Line(Line::Same(_, number, text)) => {
                job.append(&format!("{number:>4}   "), 0.0, plain(t.text_dim));
                job.append(&format!("{}\n", card::visible(text)), 0.0, plain(t.text));
            }
            Row::Line(Line::Gone(number, text)) => {
                job.append(&format!("{number:>4} - "), 0.0, plain(t.danger));
                job.append(&format!("{}\n", card::visible(text)), 0.0, marked(t.danger));
            }
            Row::Line(Line::New(number, text)) => {
                job.append(&format!("{number:>4} + "), 0.0, plain(t.accent));
                job.append(&format!("{}\n", card::visible(text)), 0.0, marked(t.accent));
            }
        }
    }
    block(ui, salt, job);
    if let Some(cut) = cut {
        let warn = ui.visuals().warn_fg_color;
        ui.label(egui::RichText::new(cut).color(warn).strong());
    }
}

/// Draws what a card says, block by block (`card.rs` says it). `salt` tells its blocks from
/// those of another part of the card.
fn draw(ui: &mut egui::Ui, salt: &str, blocks: &[Block<'_>], action: &mut Option<Act>) {
    let warn = ui.visuals().warn_fg_color;
    for (index, block) in blocks.iter().enumerate() {
        match block {
            Block::Note(text) => {
                ui.label(text.as_str());
            }
            Block::Warn(text) => {
                ui.label(egui::RichText::new(text.as_str()).color(warn).strong());
            }
            Block::Small(text) => {
                ui.label(small(text.as_str()));
            }
            Block::Strong(text) => {
                ui.label(egui::RichText::new(text.as_str()).strong());
            }
            Block::Bullet { text, warn: weighs } => {
                let line = egui::RichText::new(format!("•  {text}"));
                ui.label(if *weighs { line.color(warn).strong() } else { line });
            }
            Block::Toggle { text, button } => {
                ui.horizontal(|ui| {
                    ui.label(small(text.as_str()));
                    if let Some((label, whole)) = button
                        && ui.small_button(*label).clicked()
                    {
                        *action = Some(Act::Whole(*whole));
                    }
                });
            }
            Block::Text { text, numbered } => text_block(ui, &format!("{salt}_text"), text, *numbered),
            Block::Diff(compared) => diff_block(ui, &format!("{salt}_diff"), compared),
            Block::Step { heading, target, blocks } => {
                ui.add_space(4.0);
                ui.separator();
                ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new(heading.as_str()).strong());
                    ui.label(egui::RichText::new(target.as_str()).monospace());
                });
                draw(ui, &format!("{salt}_step{index}"), blocks, action);
            }
        }
    }
}

/// One request that waits for an answer, in the list: what kind of thing, and what about.
fn waiting_card(ui: &mut egui::Ui, at: &Places, asked: &Asked, action: &mut Option<Act>) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(card::kind(&asked.action, card::writes_new(at, &asked.action))).strong());
            ui.label(egui::RichText::new(card::target(&asked.action)).monospace());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Look…").on_hover_text("See everything this request would do, and answer it").clicked() {
                    *action = Some(Act::Look(asked.id));
                }
                ui.label(small(card::named(asked)));
            });
        });
    });
}

/// One request, in full, for the person to answer: what it is, the thing itself, what it
/// means, and yes or no. It takes the place of the rest of the window, so that what is being
/// decided is all there is to look at.
fn review(ui: &mut egui::Ui, at: &Places, asked: &Asked, state: &mut State, driven: bool, confirms: bool, action: &mut Option<Act>) {
    let sight = card::sight_now(at, &asked.action);
    let new = card::is_new(&sight);
    // A plan's steps, followed through: how each will find what it is about.
    let followed = match &asked.action {
        Action::Plan { steps, stood, .. } => Some(state.follow(at, asked.id, steps, stood)),
        _ => None,
    };
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(card::kind(&asked.action, new)).strong().size(15.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(small(card::named(asked)));
        });
    });
    let whole = state.whole;
    egui::Frame::group(ui.style()).fill(ui.visuals().extreme_bg_color).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.label(egui::RichText::new(card::target(&asked.action)).monospace().strong());
        ui.add_space(2.0);
        match (&asked.action, &followed) {
            (Action::Plan { purpose, steps, minutes, .. }, Some(sights)) => {
                // Every step is on the card, however many: the card scrolls, and its answer stays in reach.
                let room = (ui.ctx().content_rect().height() - 440.0).max(160.0);
                egui::ScrollArea::vertical().id_salt("plan_steps").max_height(room).auto_shrink([false, true]).show(ui, |ui| {
                    draw(ui, "request", &card::plan(at, purpose, steps, *minutes, sights, whole), action);
                });
            }
            _ => draw(ui, "request", &card::content(at, &asked.action, &sight, false, whole), action),
        }
    });
    draw(ui, "means", &card::means(at, &asked.action), action);
    ui.add_space(8.0);
    ui.label(egui::RichText::new(card::question(asked, new)).strong());
    ui.add_space(2.0);
    // A card that has only just come on screen can't be said yes to yet.
    let waited = crate::now_ms() - state.shown_at;
    let ready = waited >= SETTLE_MS;
    if !ready {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(100));
    }
    // Nor can one that is too long to have been read whole.
    let cut = card::too_long(&asked.action);
    if cut {
        let warn = ui.visuals().warn_fg_color;
        ui.label(egui::RichText::new(card::TOO_LONG).color(warn).strong());
    }
    // Nor a plan that doesn't fit things as they are.
    let unfit = followed.as_ref().is_some_and(|sights| sights.is_err());
    let yes = if followed.is_some() { YES_PLAN } else { YES };
    if ui.add_enabled(ready && !cut && !unfit, egui::Button::new(yes)).clicked() {
        *action = Some(Act::Allow(asked.id));
    }
    // And don't ask again: for what can go unasked, in a module whose files are looked at
    // before they run.
    let three = agents::can_go_unasked(asked);
    if three {
        let again = egui::Button::new(card::yes_and(&asked.action)).wrap_mode(egui::TextWrapMode::Wrap);
        if ui.add_enabled(ready && !cut, again).clicked() {
            *action = Some(Act::AllowNotAgain(asked.id));
        }
    }
    ui.horizontal(|ui| {
        if ui.button(if three { NO_OF_THREE } else { NO }).clicked() {
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
    ui.label(small(card::asker(asked, confirms)));
}

/// The Agents part: the level, how to connect, who connected, and what waits for an answer.
fn agents_part(ui: &mut egui::Ui, state: &mut State, driven: bool, action: &mut Option<Act>) {
    let t = Tokens::get(ui.ctx());
    let now = agents::snapshot();
    if !now.available {
        ui.label(card::NO_AGENTS);
        return;
    }
    ui.label(card::AGENTS);
    ui.add_space(2.0);
    for level in Level::ALL {
        if ui.radio(now.level == level, level.label()).on_hover_text(level.meaning()).clicked() && level != now.level {
            *action = Some(Act::Level(level));
        }
    }
    ui.label(small(now.level.meaning()));
    ui.add_space(4.0);
    // Where it stands.
    let (kind, status) = card::serving(&now);
    let status = egui::RichText::new(status);
    ui.label(match kind {
        "ok" => status.color(t.accent).strong(),
        "error" => status.color(ui.visuals().error_fg_color).strong(),
        _ => status.small(),
    });
    if let Some(clients) = card::clients(&now) {
        ui.label(small(clients));
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
                waiting_card(ui, &at, asked, action);
            }
        }
    }
    // The plan that was approved, and the last one that ended.
    if !now.plans.is_empty() {
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Plans").strong());
        for plan in now.plans.iter().take(2) {
            plan_card(ui, plan, action);
        }
    }
    // What the person said not to ask about again.
    if !now.unasked.is_empty() {
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Not asked again until GridCraft closes").strong());
        for (module, kind) in &now.unasked {
            ui.horizontal(|ui| {
                ui.label(agents::covers(module, *kind));
                if ui.small_button("Ask again").clicked() {
                    *action = Some(Act::AskAgain(module.clone(), *kind));
                }
            });
        }
    }
    // What was asked lately.
    if !now.recent.is_empty() {
        ui.add_space(4.0);
        ui.label(egui::RichText::new("Lately").strong());
        for line in card::lately(&now) {
            ui.label(small(line));
        }
        ui.label(small(card::RECORD));
    }
}

/// A plan the person approved: where each of its steps stands, and the way to stop it.
fn plan_card(ui: &mut egui::Ui, plan: &dev::Plan, action: &mut Option<Act>) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("Plan #{}", plan.id)).strong());
            ui.label(format!("{:?}", plan.title));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if plan.ended.is_none()
                    && ui.button("Stop Plan").on_hover_text("None of its steps is carried out after this. What was done stays done.").clicked()
                {
                    *action = Some(Act::StopPlan(plan.id));
                }
                ui.label(small(card::plan_stands(plan)));
            });
        });
        for (index, step) in plan.steps.iter().enumerate() {
            ui.label(small(format!("{}. {}  ·  {}", index + 1, step.action.title(), card::step_stands(&step.state))));
        }
    });
}

/// The room between a list and what is shown beside it, in which a line is drawn.
pub(super) const BETWEEN: f32 = 6.0;

/// The line between a list (`side`) and what is shown beside it, as tall as the two (`whole`).
/// It is painted: a separator there takes all the height there is, and the window would grow
/// to give it.
pub(super) fn between(ui: &egui::Ui, side: egui::Rect, whole: egui::Rect) {
    let x = side.right() + ui.spacing().item_spacing.x + BETWEEN / 2.0;
    ui.painter().vline(x, whole.y_range(), ui.visuals().widgets.noninteractive.bg_stroke);
}

/// The words under `key` of what a part says.
fn said<'a>(told: &'a Json, key: &str) -> &'a str {
    told.get(key).and_then(Json::as_str).unwrap_or("")
}

/// A small mark of how something stands: filled when it stands in some way, a ring when not.
fn dot(ui: &mut egui::Ui, mark: &str) {
    let t = Tokens::get(ui.ctx());
    let colour = match mark {
        "trusted" | "ok" => Some(t.accent),
        "changed" => Some(ui.visuals().warn_fg_color),
        "problem" | "error" => Some(ui.visuals().error_fg_color),
        _ => None,
    };
    let (rect, _) = ui.allocate_exact_size(egui::vec2(8.0, ui.spacing().interact_size.y), egui::Sense::hover());
    match colour {
        Some(colour) => ui.painter().circle_filled(rect.center(), 3.5, colour),
        None => ui.painter().circle_stroke(rect.center(), 3.0, egui::Stroke::new(1.0, t.text_dim)),
    };
}

/// How a project stands, in its own colour.
fn standing(ui: &egui::Ui, status: &str, label: &str) -> egui::RichText {
    let t = Tokens::get(ui.ctx());
    match status {
        "trusted" => egui::RichText::new(label).color(t.accent).strong(),
        "changed" => egui::RichText::new(label).color(ui.visuals().warn_fg_color).strong(),
        "problem" => egui::RichText::new(label).color(ui.visuals().error_fg_color).strong(),
        _ => egui::RichText::new(label).color(t.text_dim),
    }
}

/// The list down the window's side: every part that can be shown (`kits::nav` lists them).
fn nav_part(ui: &mut egui::Ui, rows: &Json, action: &mut Option<Act>) {
    let t = Tokens::get(ui.ctx());
    for row in rows.as_array().map(Vec::as_slice).unwrap_or_default() {
        let label = said(row, "label");
        let indent = row["depth"].as_f64().unwrap_or(0.0) as f32 * 9.0;
        match row["t"].as_str() {
            Some("heading") => {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    ui.label(egui::RichText::new(label).small().strong().color(t.text_dim));
                });
            }
            Some("item") => {
                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    if let Some(mark) = row["mark"].as_str() {
                        dot(ui, mark);
                    }
                    let text = if row["depth"] == 0 { egui::RichText::new(label).strong() } else { egui::RichText::new(label) };
                    let shown = ui.add(egui::Button::selectable(row["selected"] == true, text).truncate());
                    let shown = match said(row, "hint") {
                        "" => shown,
                        hint => shown.on_hover_text(hint),
                    };
                    if shown.clicked()
                        && let Some(to) = row["go"].as_str().and_then(Pane::parse)
                    {
                        *action = Some(Act::Go(to));
                    }
                    // How many requests wait there.
                    if let Some(badge) = row["badge"].as_str() {
                        ui.label(egui::RichText::new(badge).small().strong().color(t.accent));
                    }
                });
            }
            Some("more") => {
                ui.horizontal(|ui| {
                    ui.add_space(indent);
                    ui.label(small(label));
                });
            }
            _ => {}
        }
    }
}

/// The top of the window: the MCP server, the kits and the open workbook, each at a glance
/// with the way to its own part.
fn overview_part(ui: &mut egui::Ui, told: &Json, action: &mut Option<Act>) {
    let t = Tokens::get(ui.ctx());
    let warn = ui.visuals().warn_fg_color;
    ui.label(said(told, "intro"));
    ui.add_space(4.0);
    // The MCP server.
    let server = &told["server"];
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(kits::SERVER).strong().size(14.0));
            ui.label(small(said(server, "level")));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Open MCP Server").on_hover_text("How far agents may go, how they connect, and what they ask for").clicked() {
                    *action = Some(Act::Go(Pane::Server));
                }
            });
        });
        if server["available"] != true {
            ui.label(said(server, "unavailable"));
            return;
        }
        let status = egui::RichText::new(said(&server["status"], "text"));
        ui.label(match said(&server["status"], "kind") {
            "ok" => status.color(t.accent).strong(),
            "error" => status.color(ui.visuals().error_fg_color).strong(),
            _ => status.small(),
        });
        if let Some(waiting) = server["waiting"].as_str() {
            ui.label(egui::RichText::new(waiting).color(warn).strong());
        }
        if let Some(plan) = server["plan"].as_str() {
            ui.label(small(plan));
        }
    });
    // The kits.
    heading(ui, said(told, "kitsTitle"));
    for kit in told["kits"].as_array().map(Vec::as_slice).unwrap_or_default() {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(said(kit, "name")).strong().size(14.0));
                let made_of: Vec<&str> = kit["madeOf"].as_array().map(|all| all.iter().filter_map(Json::as_str).collect()).unwrap_or_default();
                ui.label(small(made_of.join(" · ")));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Open Kit").on_hover_text("Its projects, and what it needs to run them").clicked()
                        && let Some(to) = kit["go"].as_str().and_then(Pane::parse)
                    {
                        *action = Some(Act::Go(to));
                    }
                    ui.label(small(said(kit, "standing")));
                });
            });
            ui.label(said(kit, "says"));
            if let Some(projects) = kit["projects"].as_str() {
                ui.label(small(projects));
            }
        });
    }
    ui.label(small(said(told, "kitsNote")));
    // The workbook that is open, and its macros.
    heading(ui, "Open workbook");
    let workbook = &told["workbook"];
    ui.horizontal_wrapped(|ui| {
        if let Some(title) = workbook["title"].as_str() {
            ui.label(egui::RichText::new(title).strong());
        }
        let says = egui::RichText::new(said(workbook, "says"));
        ui.label(if workbook["problem"] == true { says.color(warn).strong() } else { says });
    });
    if let Some(rebuilt) = workbook["rebuilt"].as_str() {
        ui.label(small(rebuilt));
    }
    ui.horizontal(|ui| {
        if workbook["macros"] == true
            && ui.button("Macros…").on_hover_text("Read the macros this workbook carries. GridCraft does not run them.").clicked()
        {
            *action = Some(Act::Open(super::macros::NAME));
        }
        if ui.button("Console…").on_hover_text("What your projects print and log, what failed, and what agents asked for").clicked() {
            *action = Some(Act::Open(super::console::NAME));
        }
        ui.label(small(said(told, "console")));
    });
}

/// The Anie Development Kit: what it needs to run, its projects by kind, a new project, and
/// the showcase.
fn kit_part(ui: &mut egui::Ui, told: &Json, state: &mut State, action: &mut Option<Act>) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(said(told, "title")).strong().size(15.0));
        let made_of: Vec<&str> = told["madeOf"].as_array().map(|all| all.iter().filter_map(Json::as_str).collect()).unwrap_or_default();
        ui.label(small(made_of.join(" · ")));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| ui.label(small(said(told, "standing"))));
    });
    ui.label(said(told, "says"));
    if told["available"] != true {
        ui.label(said(told, "unavailable"));
        return;
    }
    ui.add_space(2.0);
    ui.horizontal(|ui| {
        if ui.button("Modules…").on_hover_text("Review, trust and run your projects; name the Python that runs their scripts").clicked() {
            *action = Some(Act::Open(modules::NAME));
        }
        ui.label(said(told, "projectsSays"));
        ui.add_space(12.0);
        if ui.button("Console…").on_hover_text("What your projects print and log, what failed, and what agents asked for").clicked() {
            *action = Some(Act::Open(super::console::NAME));
        }
        ui.label(said(told, "console"));
    });
    ui.horizontal(|ui| {
        ui.label(small(said(told, "python")));
        let folder = said(told, "folder");
        ui.label(small("Projects folder:"));
        ui.allocate_ui(egui::vec2((ui.available_width() - 110.0).clamp(80.0, 260.0), ui.spacing().interact_size.y), |ui| {
            ui.add(egui::Label::new(egui::RichText::new(folder).monospace().small()).truncate()).on_hover_text(folder);
        });
        if told["canOpenFolder"] == true && ui.button("Open Folder").clicked() {
            *action = Some(Act::OpenFolder);
        }
    });
    // Its projects, by what they are for.
    heading(ui, said(told, "projectsTitle"));
    ui.label(small(said(told, "projectsNote")));
    for group in told["groups"].as_array().map(Vec::as_slice).unwrap_or_default() {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(said(group, "label")).strong());
            ui.label(small(said(group, "says")));
        });
        let projects = group["projects"].as_array().map(Vec::as_slice).unwrap_or_default();
        if projects.is_empty() {
            ui.label(small(said(group, "empty")));
        }
        for project in projects {
            ui.horizontal(|ui| {
                dot(ui, said(project, "status"));
                match project["go"].as_str().and_then(Pane::parse) {
                    Some(to) => {
                        if ui.link(said(project, "name")).on_hover_text("What it is for, what it asks, and what is in it").clicked() {
                            *action = Some(Act::Go(to));
                        }
                    }
                    None => {
                        ui.label(said(project, "name"));
                    }
                }
                if let Some(workbook) = project["workbook"].as_str() {
                    ui.label(small(if project["open"] == true { format!("for {workbook}, which is open") } else { format!("for {workbook}") }));
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(standing(ui, said(project, "status"), said(project, "statusLabel")).small());
                });
            });
            if !said(project, "says").is_empty() {
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label(small(said(project, "says")));
                });
            }
        }
    }
    // A new one.
    ui.add_space(6.0);
    ui.horizontal(|ui| {
        let label = ui.label("New project:");
        let size = egui::vec2(220.0, ui.spacing().interact_size.y);
        ui.add_sized(size, egui::TextEdit::singleline(&mut state.new_name).hint_text("Its name")).labelled_by(label.id);
        if ui.button("Create Project").on_hover_text("Creates a starter project. It does nothing until you trust it.").clicked() {
            *action = Some(Act::CreateProject { name: state.new_name.clone(), for_workbook: state.new_for });
        }
    });
    match told["newFor"].as_str() {
        Some(workbook) => {
            ui.checkbox(&mut state.new_for, format!("It is the code of {workbook:?}, the workbook that is open"));
        }
        None => state.new_for = false,
    }
    // The showcase: a panel written with a web toolkit, shown inside the window.
    let showcase = &told["showcase"];
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Showcase").strong());
        ui.label(said(showcase, "text"));
        match said(showcase, "stands") {
            "none" => {
                if ui.button("Create Showcase").on_hover_text("Creates the Svelte Showcase project. It does nothing until you trust it.").clicked() {
                    *action = Some(Act::CreateShowcase);
                }
            }
            "trusted" => {
                if showcase["canDock"] == true
                    && ui.button("Dock Showcase").on_hover_text("Shows its panel inside the window, beside the sheet").clicked()
                {
                    *action = Some(Act::ShowShowcase { docked: true });
                }
                if showcase["canBrowse"] == true && ui.button("Open in Browser").clicked() {
                    *action = Some(Act::ShowShowcase { docked: false });
                }
            }
            _ => {}
        }
    });
}

/// One project: what it is for, where it stands, what it asks to be allowed, what is in it,
/// and what it says it rebuilds of its workbook's macros.
fn project_part(ui: &mut egui::Ui, told: &Json, action: &mut Option<Act>) {
    let warn = ui.visuals().warn_fg_color;
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(said(told, "title")).strong().size(15.0));
        ui.label(small(said(told, "version")));
        if !said(told, "author").is_empty() {
            ui.label(small(format!("by {}", said(told, "author"))));
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(standing(ui, said(told, "status"), said(told, "statusLabel")));
        });
    });
    ui.label(small(format!("{}  ·  {}", said(told, "kit"), said(told, "kindLabel"))));
    if !said(told, "description").is_empty() {
        ui.label(said(told, "description"));
    }
    ui.label(said(told, "forSays"));
    ui.label(small(said(told, "statusSays")));
    if let Some(problem) = told["problem"].as_str() {
        ui.label(egui::RichText::new(format!("This project can't be used: {problem}.")).color(ui.visuals().error_fg_color).strong());
    }
    // What it asks to be allowed.
    ui.add_space(4.0);
    ui.label(egui::RichText::new(said(told, "asksTitle")).strong());
    let asks = told["asks"].as_array().map(Vec::as_slice).unwrap_or_default();
    if asks.is_empty() {
        ui.label(format!("•  {}", said(told, "asksNothing")));
    }
    for ask in asks {
        let line = egui::RichText::new(format!("•  {}", said(ask, "text")));
        ui.label(if ask["warn"] == true { line.color(warn).strong() } else { line });
    }
    // What is in it.
    ui.add_space(4.0);
    egui::Grid::new("project_parts").num_columns(2).spacing(egui::vec2(10.0, 4.0)).show(ui, |ui| {
        for part in told["parts"].as_array().map(Vec::as_slice).unwrap_or_default() {
            ui.label(small(said(part, "label")));
            ui.add(egui::Label::new(egui::RichText::new(said(part, "value")).monospace().small()).wrap());
            ui.end_row();
        }
    });
    // What it says it rebuilds of the workbook's macros.
    let rebuilt = &told["rebuilt"];
    if !rebuilt.is_null() {
        heading(ui, said(rebuilt, "title"));
        let says = egui::RichText::new(said(rebuilt, "says"));
        ui.label(if rebuilt["problem"] == true { says.color(warn).strong() } else { says.small() });
        for row in rebuilt["rows"].as_array().map(Vec::as_slice).unwrap_or_default() {
            ui.horizontal_wrapped(|ui| {
                ui.label(egui::RichText::new(said(row, "from")).monospace());
                if let Some(script) = row["script"].as_str() {
                    ui.label("→");
                    ui.label(egui::RichText::new(script).monospace());
                }
                ui.label(small(said(row, "stateLabel")).strong());
                if !said(row, "note").is_empty() {
                    ui.label(small(said(row, "note")));
                }
                if row["scriptMissing"] == true {
                    ui.label(egui::RichText::new(said(rebuilt, "missing")).small().color(warn).strong());
                }
            });
        }
    }
    ui.add_space(6.0);
    ui.label(small(said(told, "manage")));
    ui.horizontal(|ui| {
        if ui.button("Modules…").on_hover_text("Review, trust and run your projects").clicked() {
            *action = Some(Act::Open(modules::NAME));
        }
        if told["canOpenFolder"] == true && ui.button("Open Project Folder").clicked() {
            *action = Some(Act::OpenProjectFolder(said(told, "id").to_string()));
        }
    });
}

/// Hook of the fork's dialogs: draws Developer ▸ Development.
pub fn ui(app: &mut SheetApp, ui: &mut egui::Ui, d: &mut Dialog, open: &mut bool) {
    ui.set_min_width(820.0);
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
        (Some(asked), Some(at)) => review(ui, &at, &asked, &mut state, driven, agents::confirms(app), &mut action),
        _ => {
            state.looking = None;
            let now = agents::snapshot();
            // A project that is no longer there gives way to its kit.
            if let Pane::Project(id) = &state.pane
                && kits::project_pane(app, id).is_none()
            {
                state.pane = Pane::Kit;
            }
            let rows = kits::nav(app, &state.pane, &now);
            // The server's part is drawn from how things stand; the others from what `kits` says.
            let told = if state.pane == Pane::Server { Json::Null } else { kits::pane(app, &ctx, &state.pane, &now) };
            // On a small screen both sides scroll, and Close stays in reach.
            let room = (ctx.content_rect().height() - 220.0).max(240.0);
            let row = ui.horizontal_top(|ui| {
                let side = ui.vertical(|ui| {
                    ui.set_width(200.0);
                    egui::ScrollArea::vertical().id_salt("development_parts").max_height(room).show(ui, |ui| nav_part(ui, &rows, &mut action));
                });
                ui.add_space(BETWEEN);
                ui.vertical(|ui| {
                    ui.set_min_width(580.0);
                    egui::ScrollArea::vertical().id_salt("development").max_height(room).show(ui, |ui| match told["kind"].as_str() {
                        Some("overview") => overview_part(ui, &told, &mut action),
                        Some("kit") => kit_part(ui, &told, &mut state, &mut action),
                        Some("project") => project_part(ui, &told, &mut action),
                        _ => {
                            ui.label(egui::RichText::new(kits::SERVER).strong().size(15.0));
                            agents_part(ui, &mut state, driven, &mut action);
                        }
                    });
                });
                side.response.rect
            });
            between(ui, row.inner, row.response.rect);
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
    // While a request waits, or a plan's steps can be carried out, the window keeps up.
    let now = agents::snapshot();
    if !now.waiting.is_empty() || now.plans.iter().any(|plan| plan.ended.is_none()) {
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
pub(super) fn act(app: &mut SheetApp, ctx: &egui::Context, state: &mut State, action: Act, driven: bool, open: &mut bool) -> Result<String, String> {
    let outcome = carry_out(app, ctx, state, action, driven, open);
    // Whatever was done, what the window shows of the projects is read from disk again.
    kits::forget();
    outcome
}

fn carry_out(app: &mut SheetApp, ctx: &egui::Context, state: &mut State, action: Act, driven: bool, open: &mut bool) -> Result<String, String> {
    // Going to another part of the window, or to another of the fork's windows, is anyone's;
    // so is a new project, which is created untrusted. Everything else is the person's: what
    // is set, what is answered, and which request is under their eyes.
    // So is tightening things: being asked again, stopping a plan.
    let anyones = matches!(
        action,
        Act::Go(_)
            | Act::Open(_)
            | Act::OpenFolder
            | Act::OpenProjectFolder(_)
            | Act::AskAgain(..)
            | Act::StopPlan(_)
            | Act::CreateShowcase
            | Act::CreateProject { .. }
    );
    if !anyones && driven {
        return Err(presence::BY_HAND.to_string());
    }
    match action {
        Act::Go(pane) => {
            state.pane = pane;
            Ok(String::new())
        }
        Act::CreateProject { name, for_workbook } => {
            // The workbook it is the code of is the one that is open: nobody names another.
            let workbook = if for_workbook { app.session.active().map(|doc| doc.title.clone()) } else { None };
            let created = modules::create_project(&name, workbook.as_deref())?;
            state.new_name.clear();
            state.new_for = false;
            // The window turns to what was made.
            let id = starter::id_for(&name);
            if manifest::valid_id(&id) {
                state.pane = Pane::Project(id);
            }
            Ok(created)
        }
        Act::OpenProjectFolder(id) => match (modules::places(), &app.services.open_url) {
            (Some(at), Some(open_url)) if manifest::valid_id(&id) && at.modules.join(&id).is_dir() => {
                open_url(&at.modules.join(&id).display().to_string());
                Ok(String::new())
            }
            (Some(_), Some(_)) => Err("That project's folder is no longer there.".into()),
            _ => Err(modules::UNAVAILABLE.into()),
        },
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
        Act::Allow(id) => allow(app, ctx, state, id, agents::Said::Yes),
        Act::AllowNotAgain(id) => allow(app, ctx, state, id, agents::Said::YesAndNotAgain),
        Act::Refuse(id) => {
            let outcome = agents::answer(app, ctx, id, agents::Said::No(state.reason.clone()));
            state.look_at(agents::snapshot().waiting.first().map(|asked| asked.id));
            outcome
        }
        Act::AskAgain(module, kind) => {
            agents::ask_again(&module, kind);
            Ok(format!("You will be asked again for {}.", agents::covers(&module, kind)))
        }
        Act::StopPlan(id) => agents::stop_plan(id, !driven),
        Act::Open(name) => {
            app.open_dialog(name, json!({}));
            *open = false;
            Ok(String::new())
        }
        Act::CreateShowcase => modules::create_showcase(),
        // Showing a panel is opening it: the person's alone to do (checked above).
        Act::ShowShowcase { docked: true } => {
            let said = super::dock::dock(app, ctx, showcase::ID)?;
            // The window makes way: the panel is shown where it would be standing.
            *open = false;
            app.toast = Some((said, crate::now_ms()));
            Ok(String::new())
        }
        Act::ShowShowcase { docked: false } => super::panels::open(app, ctx, showcase::ID),
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

/// The person said yes on the card of the request `id`.
fn allow(app: &mut SheetApp, ctx: &egui::Context, state: &mut State, id: u64, said: agents::Said) -> Result<String, String> {
    // The card's first moment on screen: a click then was meant for something else. And only
    // the request whose card is on screen is answered.
    if crate::now_ms() - state.shown_at < SETTLE_MS || state.looking != Some(id) {
        return Ok(String::new());
    }
    let outcome = agents::answer(app, ctx, id, said);
    // On to the request that has waited longest, when another waits.
    state.look_at(agents::snapshot().waiting.first().map(|asked| asked.id));
    outcome
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
    use crate::aniefork::devpage;
    use crate::aniefork::modules::tests::{Asked as Answers, Sandbox, app_with, drive, quiet, sandbox, yes};
    use crate::aniefork::modules::{create, list, trust};

    type Harness = egui_kittest::Harness<'static, SheetApp>;

    /// An app in a window, as it is when GridCraft has just started with the control channel
    /// open (`--control`): what clicks in the window may then be a program's doing.
    fn harness(mut app: SheetApp) -> Harness {
        let (_nobody, channel) = std::sync::mpsc::channel();
        app.control_rx = Some(channel);
        harness_alone(app)
    }

    /// The same without the control channel: nothing but the person can click in the window.
    fn harness_alone(app: SheetApp) -> Harness {
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

    /// Opens the window and turns to the MCP server's part, where agents are seen to. (While a
    /// single request waits, the window opens on that request's card, with no list beside it.)
    fn open(h: &mut Harness) {
        open_top(h);
        go(h, kits::SERVER);
    }

    /// Opens the window as it opens by itself: on its top, unless something waits for an answer.
    fn open_top(h: &mut Harness) {
        h.state_mut().open_dialog(NAME, json!({}));
        // A new window takes a few frames to find its size and place; clicks go by position.
        h.run_steps(6);
    }

    /// Turns to another part of the window, by its name in the list down the side.
    fn go(h: &mut Harness, part: &str) {
        h.run_steps(2);
        if h.query_by_role_and_label(Role::Button, part).is_some() {
            h.get_by_role_and_label(Role::Button, part).click();
        }
        h.run_steps(4);
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

    /// The person says no on the card, whichever number its no has.
    fn say_no(h: &mut Harness) {
        h.run_steps(3);
        let label = if h.query_by_label(NO_OF_THREE).is_some() { NO_OF_THREE } else { NO };
        h.get_by_label(label).click();
        h.run_steps(4);
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
        say_no(&mut h);
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
        say_no(&mut h);
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
    fn without_the_control_channel_a_yes_on_the_card_is_enough() {
        let s = Sandbox::new("agents-one-click");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness_alone(app_with(Some(&asked)));
        assert!(!agents::confirms(h.state()));
        open(&mut h);
        // Letting agents go further is still confirmed by the operating system.
        choose(&mut h, Level::Ask);
        assert_eq!(asked.questions().len(), 1);
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/extra.py", "text": "x = 1\n"}));
        look(&mut h);
        h.get_by_label_contains("The agent gave no name.");
        assert!(h.query_by_label_contains("confirmed by your operating system").is_none(), "the card promises no second question");
        // One click, and it is done: nothing but the person can click here.
        say_yes(&mut h);
        assert_eq!(asked.questions().len(), 1, "no second question was put");
        assert_eq!(std::fs::read_to_string(s.0.join("modules/clean-data/python/extra.py")).unwrap(), "x = 1\n");
        assert!(stands(id).1.contains("was carried out"));
        let id = asks(&mut h, "delete_module_file", json!({"module": "clean-data", "path": "python/extra.py"}));
        look(&mut h);
        say_yes(&mut h);
        assert!(stands(id).1.contains("was carried out") && !s.0.join("modules/clean-data/python/extra.py").exists());
        // A copy of GridCraft that can't put the operating system's question answers requests
        // all the same, when nothing else can click in its window.
        drop(h);
        agents::forget();
        let mut h = harness_alone(app_with(None));
        let id = asks(&mut h, "create_module", json!({"name": "Another"}));
        open(&mut h);
        say_yes(&mut h);
        assert!(stands(id).1.contains("Another was created as the module another"), "{}", stands(id).1);
        assert_eq!(asked.questions().len(), 1);
    }

    #[test]
    fn a_yes_can_say_not_to_ask_again_until_gridcraft_closes() {
        let s = Sandbox::new("agents-not-again");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        trust(h.state(), "clean-data", list()["modules"][0]["fingerprint"].as_str().unwrap(), false).unwrap();
        open(&mut h);
        choose(&mut h, Level::Ask);
        let again = "2. Yes, and don't ask again for changes to clean-data's files until GridCraft closes";
        let first = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/a.py", "text": "a = 1\n"}));
        look(&mut h);
        h.get_by_label(NO_OF_THREE);
        std::thread::sleep(Duration::from_millis(SETTLE_MS as u64 + 40));
        click(&mut h, again);
        let question = asked.questions().pop().unwrap();
        assert!(question.contains("And GridCraft will not ask you again for changes to clean-data's files until it closes."), "{question}");
        assert!(stands(first).1.contains("was carried out"));
        assert_eq!(asked.questions().len(), 3, "trusting, the level, and this yes");
        // The next change to that module's files is not asked about: it is done, and on record.
        let second = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/b.py", "text": "b = 2\n"}));
        assert!(stands(second).1.contains("python/b.py was created in clean-data"), "{}", stands(second).1);
        assert_eq!(std::fs::read_to_string(s.0.join("modules/clean-data/python/b.py")).unwrap(), "b = 2\n");
        assert_eq!(asked.questions().len(), 3);
        assert!(h.state().notice.is_none() && agents::snapshot().waiting.is_empty());
        let said =
            agents::snapshot().recent.iter().any(|line| line.request == second && line.detail == "without asking: the person said not to ask again");
        assert!(said, "{:?}", agents::snapshot().recent);
        // Nobody looked at that change: the module's trust went with it.
        assert_eq!(list()["modules"][0]["status"], "untrusted");
        // It covers that kind of thing in that module, and nothing else: trusting it asks.
        let third = asks(&mut h, "trust_module", json!({"module": "clean-data"}));
        assert!(stands(third).1.contains("is waiting for the person"));
        look(&mut h);
        h.get_by_label(NO);
        assert!(h.query_by_label_contains("don't ask again").is_none(), "trusting is asked each time");
        say_no(&mut h);
        // The window says what is not asked about, and the report does.
        h.get_by_label("Not asked again until GridCraft closes");
        h.get_by_label("changes to clean-data's files");
        let report = h.state_mut().run("developer.agents", json!({})).unwrap();
        assert_eq!(report["notAskedAgain"], json!([{"module": "clean-data", "what": "changing its files"}]));
        // The person can be asked again.
        click(&mut h, "Ask again");
        assert!(agents::snapshot().unasked.is_empty());
        std::thread::sleep(Duration::from_millis(3_200));
        let fourth = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/c.py", "text": "c = 3\n"}));
        assert!(stands(fourth).1.contains("is waiting for the person"));
        // And letting agents go less far ends it too.
        look(&mut h);
        std::thread::sleep(Duration::from_millis(SETTLE_MS as u64 + 40));
        click(&mut h, again);
        assert_eq!(agents::snapshot().unasked.len(), 1);
        choose(&mut h, Level::Read);
        assert!(agents::snapshot().unasked.is_empty());
        choose(&mut h, Level::Ask);
        let fifth = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/d.py", "text": "d = 4\n"}));
        assert!(stands(fifth).1.contains("is waiting for the person"));
        // GridCraft closing ends it: a server that starts knows of none.
        drop(h);
        agents::forget();
        let _h = harness(app_with(Some(&asked)));
        assert!(agents::snapshot().unasked.is_empty());
    }

    #[test]
    fn dont_ask_again_is_not_offered_where_what_is_written_runs_unlooked_at() {
        let _s = Sandbox::new("agents-not-again-follows");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        // The person keeps trusting this module as its files change.
        trust(h.state(), "clean-data", list()["modules"][0]["fingerprint"].as_str().unwrap(), true).unwrap();
        open(&mut h);
        choose(&mut h, Level::Ask);
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/a.py", "text": "a = 1\n"}));
        look(&mut h);
        h.get_by_label_contains("what is written runs without your looking at it again");
        h.get_by_label(NO);
        assert!(h.query_by_label_contains("don't ask again").is_none());
        // Nor can it be had by going round the card.
        let ctx = h.ctx.clone();
        let refused = agents::answer(h.state_mut(), &ctx, id, agents::Said::YesAndNotAgain);
        assert!(refused.unwrap_err().contains("asked about each time"));
        assert!(agents::snapshot().unasked.is_empty() && agents::snapshot().waiting.len() == 1);
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
        let long = format!("{}# the end\n", "x = 1\n".repeat(card::SHOWN / 6 + 1));
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/long.py", "text": long}));
        look(&mut h);
        h.get_by_label_contains("so it can't be allowed here");
        h.get_by_label_contains("Only the first 200000 characters are shown");
        say_yes(&mut h);
        assert_eq!(asked.questions().len(), 1, "the operating system's question isn't raised");
        assert!(stands(id).1.contains("is waiting for the person") && !s.0.join("modules/clean-data/python/long.py").exists());
        say_no(&mut h);
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
        say_no(&mut h);
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
        for label in [YES, NO_OF_THREE, NOT_NOW, "Show whole file"] {
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
        say_no(&mut h);
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
        say_no(&mut h);
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

    /// The person says yes to a plan's card, once it has been on screen a moment.
    fn approve_plan(h: &mut Harness) {
        std::thread::sleep(Duration::from_millis(SETTLE_MS as u64 + 40));
        click(h, YES_PLAN);
    }

    /// An agent asks for a step of a plan, and the app draws frames until it is settled: (is an
    /// error, what was said).
    fn step(h: &mut Harness, plan: u64, number: u64) -> (bool, String) {
        let (failed, text) = agent("run_plan_step", json!({"plan": plan, "step": number}));
        let Some((_, rest)) = text.split_once("(request #") else { return (failed, text) };
        let id = rest.chars().take_while(char::is_ascii_digit).collect::<String>().parse().unwrap();
        h.run_steps(3);
        settled(h, id)
    }

    #[test]
    fn a_plan_is_approved_on_one_card_and_its_steps_need_no_further_answer() {
        let s = Sandbox::new("agents-plan");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        h.state_mut().dialog = None;
        h.run_steps(2);
        let script = s.0.join("modules/clean-data/python/clean.py");
        let before = std::fs::read_to_string(&script).unwrap();
        let first = "def main(data):\n    return 1\n";
        let second = "def main(data):\n    print('doubling')\n    return {'twice': data['n'] * 2}\n";
        let plan = asks(
            &mut h,
            "propose_plan",
            json!({
                "title": "Double it", "purpose": "Make clean.py double a number, and try it.",
                "steps": [
                    {"tool": "write_module_file", "arguments": {"module": "clean-data", "path": "python/clean.py", "text": first}},
                    {"tool": "write_module_file", "arguments": {"module": "clean-data", "path": "python/clean.py", "text": second}},
                    {"tool": "trust_module", "arguments": {"module": "clean-data"}},
                    {"tool": "run_script", "arguments": {"module": "clean-data", "script": "clean.py", "data": {"n": 21}}},
                ],
            }),
        );
        assert!(h.state().notice.as_ref().is_some_and(|notice| notice.text.contains("1 waiting")), "it is a request like any other");
        // One card, with what it is for and every step whole.
        open(&mut h);
        look(&mut h);
        h.get_by_label("Plan");
        h.get_by_label("\"Double it\"  ·  4 steps");
        h.get_by_label("What the agent says it is for: \"Make clean.py double a number, and try it.\"");
        h.get_by_label_contains("Its files that this plan doesn't write are trusted as they are now.");
        h.get_by_label_contains("Nothing is done now. Each step is done later, when the agent asks for it");
        for heading in ["Step 1 of 4  ·  Edit file", "Step 2 of 4  ·  Edit file", "Step 3 of 4  ·  Trust module", "Step 4 of 4  ·  Run script"] {
            h.get_by_label(heading);
        }
        // Each step is shown as it will find things: the second edit takes out what the first put in.
        assert_eq!(h.query_all_by_label_contains("return 1").count(), 2, "put in by step 1, taken out by step 2");
        assert_eq!(h.query_all_by_label_contains("return {'twice': data['n'] * 2}").count(), 1);
        h.get_by_label_contains("As the steps before this one leave it, it will contain 3 files: module.toml, panel/index.html, python/clean.py.");
        h.get_by_label_contains("Run its Python scripts. This cannot be limited");
        // What in it trusts a module or runs code is said by itself.
        h.get_by_label("Step 3 trusts clean-data: from then on its code can run, with what it asks to be allowed.");
        h.get_by_label("Step 4 runs \"clean.py\" of clean-data: a script can do anything a program you run can do.");
        h.get_by_label("Do you want to approve this plan, as the agent proposes it?");
        assert!(h.query_by_label(YES).is_none() && h.query_by_label_contains("don't ask again").is_none(), "a plan is approved or not");
        let questions = asked.questions().len();
        approve_plan(&mut h);
        // The operating system's question names the plan, its steps and what weighs most.
        let question = asked.questions().pop().unwrap();
        assert_eq!(asked.questions().len(), questions + 1);
        let tag = format!("This is request #{plan}, holding what its card marks ");
        for part in [
            "An agent asks you to approve a plan, \"Double it\". It says it is for: \"Make clean.py double a number, and try it.\"",
            "  1. Write \"python/clean.py\" in clean-data",
            "  3. Trust clean-data",
            "  4. Run \"clean.py\" of clean-data",
            "Nothing is done now.",
            "It trusts a module and runs its code",
            tag.as_str(),
            "Approve the plan?",
        ] {
            assert!(question.contains(part), "{part}: {question}");
        }
        // Approving it did nothing but approve it.
        assert_eq!(std::fs::read_to_string(&script).unwrap(), before);
        let (failed, said) = stands(plan);
        assert!(
            !failed
                && said.contains(&format!("Plan #{plan} is approved: 4 step(s), to be carried out within 60 minute(s). Nothing has been done yet.")),
            "{said}"
        );
        // The window shows the plan, and the way to stop it.
        h.get_by_label_contains("The plan is approved: its 4 steps can be carried out for the next 60 minutes");
        h.get_by_label(format!("Plan #{plan}").as_str());
        h.get_by_label("Stop Plan");
        h.get_by_label_contains("0 of 4 done");
        let report = h.state_mut().run("developer.agents", json!({})).unwrap();
        assert_eq!((&report["plans"][0]["plan"], &report["plans"][0]["ended"]), (&json!(plan), &Json::Null));
        assert_eq!(report["plans"][0]["steps"][2]["what"], "Trust clean-data");
        // The steps are carried out as the agent asks for them, and nobody is asked again.
        let (failed, said) = step(&mut h, plan, 1);
        assert!(!failed && said.contains("python/clean.py was written over in clean-data") && said.contains("The next is step 2 of 4"), "{said}");
        assert_eq!(std::fs::read_to_string(&script).unwrap(), first);
        let (failed, said) = step(&mut h, plan, 2);
        assert!(!failed, "{said}");
        assert_eq!(std::fs::read_to_string(&script).unwrap(), second);
        assert_eq!(list()["modules"][0]["status"], "untrusted");
        let (failed, said) = step(&mut h, plan, 3);
        assert!(!failed && said.contains("clean-data is trusted as it is now."), "{said}");
        assert_eq!(list()["modules"][0]["status"], "trusted");
        assert_eq!(list()["modules"][0]["followsChanges"], false);
        assert_eq!(asked.questions().len(), questions + 1, "the plan was the one thing asked");
        assert!(agents::snapshot().waiting.is_empty() && h.state().notice.is_none());
        h.get_by_label_contains("3 of 4 done");
        if let Err(why) = PythonHost::global().interpreter("") {
            eprintln!("skipped the run: {why}");
            return;
        }
        let (failed, said) = step(&mut h, plan, 4);
        assert!(
            !failed
                && said.contains("clean.py of clean-data returned.")
                && said.contains(&format!("Plan #{plan} has ended: every step was carried out.")),
            "{said}"
        );
        let (_, rest) = said.split_once("<untrusted-data-").unwrap();
        let data: Json = serde_json::from_str(rest.split_once(">\n").unwrap().1.rsplit_once("\n</untrusted-data-").unwrap().0).unwrap();
        assert_eq!(data["result"], json!({"twice": 42}));
        assert_eq!(data["printed"].as_str().map(str::trim), Some("doubling"));
        PythonHost::global().stop(&s.0.join("modules"), "clean-data");
        // It is over: the window says so, and nothing more of it can be asked for.
        h.run_steps(3);
        h.get_by_label_contains("4 of 4 done  ·  ended: every step was carried out");
        assert!(h.query_by_label("Stop Plan").is_none());
        assert!(step(&mut h, plan, 4).1.contains("has ended"));
        assert_eq!(outcomes(&s), ["asked", "allowed", "done", "allowed", "done", "allowed", "done", "allowed", "done", "allowed", "done", "done"]);
    }

    #[test]
    fn a_plans_step_waits_while_its_module_is_not_as_the_plan_left_it_and_the_person_can_stop_the_plan() {
        let s = Sandbox::new("agents-plan-fits");
        create("Clean Data").unwrap();
        let asked = yes();
        // Without the control channel: a yes on the card is enough for a plan too.
        let mut h = harness_alone(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        h.state_mut().dialog = None;
        h.run_steps(2);
        let script = s.0.join("modules/clean-data/python/clean.py");
        let notes = s.0.join("modules/clean-data/notes.txt");
        let before = std::fs::read_to_string(&script).unwrap();
        let write = |text: &str| json!({"tool": "write_module_file", "arguments": {"module": "clean-data", "path": "notes.txt", "text": text}});
        let plan = asks(&mut h, "propose_plan", json!({"title": "Notes", "purpose": "Keep notes.", "steps": [write("one\n"), write("two\n")]}));
        open(&mut h);
        look(&mut h);
        h.get_by_label("Step 1 of 2  ·  Create file");
        h.get_by_label("Step 2 of 2  ·  Edit file");
        let questions = asked.questions().len();
        approve_plan(&mut h);
        assert_eq!(asked.questions().len(), questions, "nothing but the person can click here");
        assert_eq!(agents::snapshot().plans[0].ended, None);
        // Something else changes the module: the plan was approved for the files as they stood.
        std::fs::write(&script, "def main(data):\n    return 'changed by hand'\n").unwrap();
        let (failed, said) = step(&mut h, plan, 1);
        assert!(failed && said.contains("clean-data's files are not as the plan's steps left them"), "{said}");
        assert!(said.contains("That is not held against the plan"), "{said}");
        assert!(!notes.exists(), "the step was not carried out");
        let now = agents::snapshot();
        assert_eq!((now.plans[0].ended.clone(), now.plans[0].done()), (None, 0), "and the plan goes on");
        // Put right, the step is carried out; and the next one finds what it left.
        std::fs::write(&script, &before).unwrap();
        let (failed, said) = step(&mut h, plan, 1);
        assert!(!failed && said.contains("notes.txt was created in clean-data"), "{said}");
        assert_eq!(std::fs::read_to_string(&notes).unwrap(), "one\n");
        // The person stops the plan: what was done stays done, and nothing more is.
        h.run_steps(3);
        click(&mut h, "Stop Plan");
        h.get_by_label(format!("Plan #{plan} was stopped. What was done stays done; what was not will not be.").as_str());
        h.get_by_label_contains("1 of 2 done  ·  ended: the person stopped it");
        let (failed, said) = step(&mut h, plan, 2);
        assert!(failed && said.contains(&format!("Plan #{plan} has ended: the person stopped it.")), "{said}");
        assert_eq!(std::fs::read_to_string(&notes).unwrap(), "one\n");
        assert_eq!(outcomes(&s), ["asked", "allowed", "done", "allowed", "not ready", "allowed", "done", "stopped"]);
    }

    #[test]
    fn a_plan_that_no_longer_fits_cannot_be_approved_and_lowering_the_level_ends_the_one_that_was() {
        let s = Sandbox::new("agents-plan-unfit");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness_alone(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Ask);
        h.state_mut().dialog = None;
        h.run_steps(2);
        let script = s.0.join("modules/clean-data/python/clean.py");
        let delete = json!({"tool": "delete_module_file", "arguments": {"module": "clean-data", "path": "python/clean.py"}});
        let plan = asks(&mut h, "propose_plan", json!({"title": "Tidy", "purpose": "Take the script out.", "steps": [delete]}));
        // The module changes before the person answers: it is no longer the one the plan was
        // proposed for. (So it is with trust asked for by itself.)
        let before = std::fs::read_to_string(&script).unwrap();
        std::fs::write(&script, "def main(data):\n    return 'not what was proposed for'\n").unwrap();
        open(&mut h);
        look(&mut h);
        h.get_by_label_contains(
            "The plan doesn't fit things as they are now, so it can't be approved. clean-data has changed since the agent proposed this plan",
        );
        approve_plan(&mut h);
        assert!(stands(plan).1.contains("is waiting for the person"), "the yes is not there to press");
        assert!(agents::snapshot().plans.is_empty());
        // Nor is it approved if a yes got through all the same.
        let ctx = h.ctx.clone();
        let refused = agents::answer(h.state_mut(), &ctx, plan, agents::Said::Yes).unwrap_err();
        assert!(refused.contains("The plan no longer fits things as they are."), "{refused}");
        let (failed, said) = stands(plan);
        assert!(failed && said.contains("The plan no longer fits things as they are.") && said.contains("Propose it again"), "{said}");
        assert!(said.contains("clean-data has changed since the agent proposed this plan"), "{said}");
        assert!(agents::snapshot().plans.is_empty());
        std::fs::write(&script, &before).unwrap();
        // Nor one whose steps no longer fit: here the person takes back the trust its run needs.
        h.run_steps(3);
        trust(h.state(), "clean-data", list()["modules"][0]["fingerprint"].as_str().unwrap(), false).unwrap();
        let run = json!({"tool": "run_script", "arguments": {"module": "clean-data", "script": "clean.py"}});
        let plan = asks(&mut h, "propose_plan", json!({"title": "Run", "purpose": "Try it.", "steps": [run]}));
        gridcraft_aniefork_dev::TrustStore::load(&s.0.join("trusted-modules.json")).revoke("clean-data").unwrap();
        look(&mut h);
        h.get_by_label_contains("so it can't be approved. Step 1: clean-data will not be trusted when \"clean.py\" is to run");
        let refused = agents::answer(h.state_mut(), &ctx, plan, agents::Said::Yes).unwrap_err();
        assert!(refused.contains("Step 1: clean-data will not be trusted"), "{refused}");
        // A plan that was approved ends when the person lets agents go less far.
        h.run_steps(3);
        let write = json!({"tool": "write_module_file", "arguments": {"module": "clean-data", "path": "notes.txt", "text": "n\n"}});
        let plan = asks(&mut h, "propose_plan", json!({"title": "Notes", "purpose": "Keep notes.", "steps": [write]}));
        look(&mut h);
        approve_plan(&mut h);
        assert_eq!(agents::snapshot().plans[0].ended, None);
        choose(&mut h, Level::Read);
        assert_eq!(agents::snapshot().plans[0].ended.as_deref(), Some("agent access was lowered"));
        let (failed, said) = step(&mut h, plan, 1);
        assert!(failed && !s.0.join("modules/clean-data/notes.txt").exists(), "{said}");
        let report = h.state_mut().run("developer.agents", json!({})).unwrap();
        assert_eq!(report["plans"][0]["ended"], "agent access was lowered");
    }

    /// What the page is told to show now, as the shell would be handed it.
    fn told(h: &mut Harness) -> Json {
        let ctx = h.ctx.clone();
        // It is worked out again when something was clicked there, or after a moment.
        std::thread::sleep(Duration::from_millis(320));
        let text = devpage::state(h.state_mut(), &ctx).expect("the page is wanted, and a moment has passed");
        let token = std::fs::read_to_string(sandbox().unwrap().join("agent-token")).unwrap_or_else(|_| "no token yet".into());
        assert!(!text.contains(&token), "never the token");
        serde_json::from_str(&text).unwrap()
    }

    /// The person clicks something in the page.
    fn page(h: &mut Harness, message: Json) -> bool {
        let ctx = h.ctx.clone();
        let again = devpage::handle(h.state_mut(), &ctx, &message.to_string());
        h.run_steps(2);
        again
    }

    #[test]
    fn where_the_shell_can_show_it_the_window_is_a_page_and_a_click_there_goes_the_same_way() {
        let s = Sandbox::new("devpage");
        create("Clean Data").unwrap();
        let asked = yes();
        let mut h = harness_alone(app_with(Some(&asked)));
        let ctx = h.ctx.clone();
        // Without a shell that can show a page, the window is GridCraft's own.
        assert!(!devpage::takes_over(&ctx) && devpage::state(h.state_mut(), &ctx).is_none());
        open(&mut h);
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(NAME));
        assert!(!page(&mut h, json!({"act": "level", "level": "read"})), "and a message from no page does nothing");
        assert_eq!(agents::snapshot().level, Level::Off);
        h.state_mut().dialog = None;
        h.run_steps(2);
        // With one, asking for the window opens the page instead.
        devpage::host_is_here(&ctx);
        h.state_mut().open_dialog(NAME, json!({}));
        h.run_steps(3);
        assert!(h.state().dialog.is_none() && devpage::wanted(&ctx));
        assert!(devpage::take_raise(&ctx) && !devpage::take_raise(&ctx), "it comes to the front, once");
        // It opens on its top: the whole at a glance, and the list of its parts down the side.
        let first = told(&mut h);
        assert_eq!((&first["title"], &first["pane"]["kind"], &first["looking"]), (&json!("Development"), &json!("overview"), &Json::Null));
        assert_eq!((&first["pane"]["intro"], &first["pane"]["server"]["level"]), (&json!(card::INTRO), &json!("Off")));
        assert_eq!(first["pane"]["server"]["status"], json!({"kind": "off", "text": "Not serving: agent access is off."}));
        assert_eq!(
            (&first["pane"]["kits"][0]["name"], &first["pane"]["kits"][0]["projects"]),
            (&json!("Anie Development Kit"), &json!("1 project, 0 trusted"))
        );
        let listed: Vec<&str> = first["nav"].as_array().unwrap().iter().map(|row| row["label"].as_str().unwrap()).collect();
        assert_eq!(listed, ["Development", "MCP Server", "Development Kits", "Anie Development Kit", "Libraries", "Clean Data"]);
        assert_eq!((&first["nav"][0]["selected"], &first["nav"][1]["go"]), (&json!(true), &json!("server")));
        assert!(first["theme"]["window"].as_str().unwrap().starts_with('#'));
        assert!(devpage::state(h.state_mut(), &ctx).is_none(), "it is not told again at once");
        // The person turns to the MCP server's part.
        page(&mut h, json!({"act": "go", "to": "server"}));
        let server = told(&mut h);
        assert_eq!(
            (&server["pane"]["kind"], &server["pane"]["title"], &server["pane"]["level"]),
            (&json!("server"), &json!("MCP Server"), &json!("off"))
        );
        assert_eq!(server["pane"]["levels"].as_array().unwrap().len(), 5);
        assert_eq!((&server["nav"][0]["selected"], &server["nav"][1]["selected"]), (&json!(false), &json!(true)));
        // A click there goes the way a click in GridCraft's own window goes: raising the level
        // ends in the operating system's question.
        page(&mut h, json!({"act": "level", "level": "ask"}));
        assert_eq!((asked.questions().len(), agents::snapshot().level, kept_level(&s)), (1, Level::Ask, Level::Ask));
        let now = told(&mut h);
        assert_eq!(
            (&now["pane"]["level"], &now["pane"]["serving"], &now["said"]),
            (&json!("ask"), &json!(true), &json!("Agent access: Ask me each time."))
        );
        assert_eq!(
            (&now["nav"][1]["hint"], &now["nav"][1]["mark"], &now["nav"][1]["badge"]),
            (&json!("Ask me each time"), &json!("ok"), &Json::Null)
        );
        // The setup goes to the clipboard by GridCraft: the page is never handed the token.
        page(&mut h, json!({"act": "copySetup", "json": false}));
        assert!(told(&mut h)["said"].as_str().unwrap().starts_with("The setup was copied."));
        assert!(page(&mut h, json!({"act": "ready"})), "a page that has just loaded is told everything again");
        // An agent asks. The window is open: it never turns to a card by itself.
        let text = "def main(data):\n    return 'from the agent'\n";
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/extra.py", "text": text}));
        let now = told(&mut h);
        assert_eq!(now["looking"], Json::Null);
        assert_eq!(now["nav"][1]["badge"], "1", "the list says that something waits there");
        let waiting = &now["pane"]["waiting"][0];
        assert_eq!((&waiting["id"], &waiting["kind"], &waiting["target"]), (&json!(id), &json!("Create file"), &json!("clean-data/python/extra.py")));
        // The person looks: the card, in the words GridCraft's own window uses.
        page(&mut h, json!({"act": "look", "id": id}));
        let shown = told(&mut h)["looking"].clone();
        assert_eq!((&shown["id"], &shown["kind"], &shown["target"]), (&json!(id), &json!("Create file"), &json!("clean-data/python/extra.py")));
        assert_eq!(shown["question"], "Do you want to let the agent create \"python/extra.py\"?");
        assert_eq!(shown["content"][0], json!({"t": "small", "text": "The file is new:"}));
        assert_eq!(shown["content"][1]["lines"], json!(["def main(data):", "    return 'from the agent'"]));
        assert_eq!((&shown["yes"], &shown["no"], &shown["blocked"]), (&json!(YES), &json!(NO_OF_THREE), &json!(false)));
        assert!(shown["yesAnd"].as_str().unwrap().starts_with("2. Yes, and don't ask again for changes to clean-data's files"));
        assert_eq!(shown["asker"], "The agent gave no name.");
        // A yes for another card than the one on screen answers nothing.
        let file = s.0.join("modules/clean-data/python/extra.py");
        std::thread::sleep(Duration::from_millis(SETTLE_MS as u64 + 40));
        page(&mut h, json!({"act": "allow", "id": id + 1}));
        assert!(!file.exists() && stands(id).1.contains("is waiting for the person"));
        // Nor one in the card's first moment on screen: GridCraft keeps the time, not the page.
        page(&mut h, json!({"act": "notNow"}));
        page(&mut h, json!({"act": "look", "id": id}));
        page(&mut h, json!({"act": "allow", "id": id}));
        assert!(!file.exists(), "too soon");
        std::thread::sleep(Duration::from_millis(SETTLE_MS as u64 + 40));
        page(&mut h, json!({"act": "allow", "id": id}));
        assert_eq!(std::fs::read_to_string(&file).unwrap(), text);
        assert_eq!(asked.questions().len(), 1, "without the control channel a yes on the card is enough, here as there");
        let now = told(&mut h);
        assert!(now["looking"].is_null() && now["said"].as_str().unwrap().contains("python/extra.py was created in clean-data"), "{now}");
        // What is too long to show is not said yes to, whatever the page sends; a no carries
        // the person's word.
        let long = "x = 1\n".repeat(card::SHOWN / 6 + 1);
        let id = asks(&mut h, "write_module_file", json!({"module": "clean-data", "path": "python/long.py", "text": long}));
        page(&mut h, json!({"act": "look", "id": id}));
        let shown = told(&mut h)["looking"].clone();
        assert_eq!((&shown["blocked"], &shown["blockedWhy"]), (&json!(true), &json!(card::TOO_LONG)));
        std::thread::sleep(Duration::from_millis(SETTLE_MS as u64 + 40));
        page(&mut h, json!({"act": "allow", "id": id}));
        assert_eq!(told(&mut h)["error"], card::TOO_LONG);
        assert!(!s.0.join("modules/clean-data/python/long.py").exists());
        page(&mut h, json!({"act": "refuse", "id": id, "reason": "smaller pieces, please"}));
        let (failed, said) = stands(id);
        assert!(failed && said.contains("They tell you: \"smaller pieces, please\""), "{said}");
        // What is not one of the page's messages is nothing.
        let before = agents::snapshot().level;
        let wordy = json!({"act": "refuse", "id": 1, "reason": "x".repeat(5000)}).to_string();
        for bad in [
            "",
            "{",
            "[]",
            "{\"act\":\"format_disk\"}",
            "{\"act\":\"allow\"}",
            "{\"act\":\"level\",\"level\":\"root\"}",
            "{\"act\":\"open\",\"name\":\"development\"}",
            "{\"act\":\"go\",\"to\":\"elsewhere\"}",
            "{\"act\":\"go\",\"to\":\"project:../clean-data\"}",
            "{\"act\":\"openProjectFolder\"}",
            "{\"act\":\"createProject\"}",
            wordy.as_str(),
        ] {
            assert!(!devpage::handle(h.state_mut(), &ctx, bad), "{bad:.40}");
        }
        assert_eq!(agents::snapshot().level, before);
        assert!(h.state().dialog.is_none());
        // Its other windows open in GridCraft's own.
        page(&mut h, json!({"act": "open", "name": "modules"}));
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(modules::NAME));
        h.state_mut().dialog = None;
        page(&mut h, json!({"act": "open", "name": "macros"}));
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(crate::aniefork::macros::NAME));
        h.state_mut().dialog = None;
        // The kit's part, with its projects by what they are for.
        page(&mut h, json!({"act": "go", "to": "kit"}));
        let kit = told(&mut h)["pane"].clone();
        assert_eq!((&kit["kind"], &kit["title"], &kit["standing"]), (&json!("kit"), &json!("Anie Development Kit"), &json!("Built in  ·  loaded")));
        assert_eq!((&kit["groups"][0]["label"], &kit["groups"][0]["projects"]), (&json!("Workbook code"), &json!([])));
        assert_eq!(kit["groups"][1]["projects"][0]["name"], "Clean Data");
        // A new project made there is the code of the workbook that is open: GridCraft says
        // which workbook that is, whatever the page sends with it.
        let title = h.state().session.active().unwrap().title.clone();
        page(&mut h, json!({"act": "createProject", "name": "Budget Macros", "forWorkbook": true, "workbook": "Elsewhere.xlsm"}));
        let now = told(&mut h);
        assert_eq!(now["said"], json!(format!("Budget Macros was created, as the code of {title}. It is not trusted yet.")));
        let made = &now["pane"];
        assert_eq!(
            (&made["kind"], &made["id"], &made["kindLabel"], &made["statusLabel"]),
            (&json!("project"), &json!("budget-macros"), &json!("Workbook code"), &json!("Not trusted"))
        );
        assert_eq!(made["forSays"], json!(format!("It says it is the code of the workbook {title:?}, which is open now.")));
        assert!(now["nav"].as_array().unwrap().iter().any(|row| row["go"] == "project:budget-macros" && row["selected"] == true));
        assert_eq!(list()["modules"].as_array().unwrap().iter().find(|m| m["id"] == "budget-macros").unwrap()["status"], "untrusted");
        assert_eq!(asked.questions().len(), 1, "creating it asked nothing and trusted nothing");
        // A project that is gone gives way to its kit.
        std::fs::remove_dir_all(s.0.join("modules/budget-macros")).unwrap();
        assert_eq!(told(&mut h)["pane"]["kind"], "kit");
        // The person can have GridCraft's own window instead, until GridCraft closes.
        page(&mut h, json!({"act": "builtin"}));
        assert!(!devpage::wanted(&ctx) && !devpage::takes_over(&ctx));
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(NAME));
        h.run_steps(3);
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(NAME), "and it stays");
    }

    #[test]
    fn at_read_only_an_agent_reads_the_macros_of_the_workbook_that_is_open() {
        use crate::aniefork::macros::tests::{TOTALS, carry, sample};

        let _s = Sandbox::new("agents-macros");
        let asked = yes();
        let mut h = harness(app_with(Some(&asked)));
        open(&mut h);
        choose(&mut h, Level::Read);
        h.state_mut().dialog = None;
        h.run_steps(2);
        assert!(agent("list_macros", json!({})).1.contains("carries no macros"));
        // The workbook that is open carries a project: the server is told, and an agent reads it.
        let project = sample();
        carry(h.state_mut(), project.clone());
        h.run_steps(3);
        let (failed, text) = agent("list_macros", json!({}));
        assert!(!failed && text.contains("GridCraft does not run it") && text.contains("\"name\": \"Totals\""), "{text}");
        let (failed, text) = agent("read_macro", json!({"module": "Totals"}));
        assert!(!failed && text.contains("Lines 1 to 8 of 8"), "{text}");
        let (_, rest) = text.split_once("<untrusted-data-").unwrap();
        assert_eq!(rest.split_once(">\n").unwrap().1.rsplit_once("\n</untrusted-data-").unwrap().0, TOTALS.lines().collect::<Vec<_>>().join("\n"));
        // Reading asks the person nothing, and changes nothing.
        assert_eq!(asked.questions().len(), 1, "only the level was asked about");
        assert!(agents::snapshot().waiting.is_empty() && h.state().notice.is_none());
        assert_eq!(h.state().session.active().unwrap().wb.vba_project.as_deref(), Some(project.as_slice()));
        // The workbook no longer carries one: nor is it told of any more.
        let doc = h.state_mut().session.active_mut().unwrap();
        std::sync::Arc::make_mut(&mut doc.wb).vba_project = None;
        h.run_steps(3);
        assert!(agent("list_macros", json!({})).1.contains("carries no macros"));
    }

    #[test]
    fn the_window_opens_on_its_top_and_each_part_has_its_place() {
        let s = Sandbox::new("agents-tools");
        create("Clean Data").unwrap();
        let mut h = harness(app_with(Some(&yes())));
        // The top: the MCP server, the kits and the open workbook, each at a glance.
        open_top(&mut h);
        h.get_by_label(card::INTRO);
        h.get_by_label("Not serving: agent access is off.");
        h.get_by_label("1 project, 0 trusted");
        h.get_by_label(kits::KITS_NOTE);
        h.get_by_label("It carries no macros.");
        assert!(h.query_by_role_and_label(Role::RadioButton, "Full control").is_none(), "the levels are in the server's own part");
        // The MCP server's part: it stands at the top of the list, not under a kit.
        // The window keeps its size and place from one frame to the next.
        let place = h.get_by_label("Close").rect();
        h.run_steps(3);
        assert_eq!(h.get_by_label("Close").rect(), place, "the window does not grow as it is drawn");
        click(&mut h, "Open MCP Server");
        h.get_by_role_and_label(Role::RadioButton, "Full control");
        assert!(h.query_by_label(card::INTRO).is_none());
        // The kit's part: what it needs to run, the way to its other windows, its projects by kind.
        go(&mut h, "Anie Development Kit");
        h.get_by_label("Python · Svelte");
        h.get_by_label("1 project, 0 trusted");
        h.get_by_label_contains("Python: found automatically.");
        h.get_by_label(kits::PROJECTS_NOTE);
        h.get_by_label("None yet.");
        click(&mut h, "Modules…");
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(modules::NAME));
        open_top(&mut h);
        go(&mut h, "Anie Development Kit");
        click(&mut h, "Console…");
        assert_eq!(h.state().dialog.as_ref().map(|d| d.name.as_str()), Some(crate::aniefork::console::NAME));
        // A project has a part of its own: what it is for, where it stands, what it asks.
        open_top(&mut h);
        go(&mut h, "Clean Data");
        h.get_by_label("Anie Development Kit  ·  Library");
        h.get_by_label("It is for any workbook.");
        h.get_by_label("It asks to be allowed to:");
        h.get_by_label("Not trusted");
        // A new project, made in the kit's part as the code of the workbook that is open.
        go(&mut h, "Anie Development Kit");
        h.get_by_role_and_label(Role::TextInput, "New project:").focus();
        h.run_steps(2);
        h.get_by_role_and_label(Role::TextInput, "New project:").type_text("Budget Macros");
        h.run_steps(2);
        h.get_by_label_contains("It is the code of").click();
        h.run_steps(2);
        click(&mut h, "Create Project");
        let title = h.state().session.active().unwrap().title.clone();
        let made = gridcraft_aniefork_dev::module::load(&s.0.join("modules/budget-macros"));
        assert_eq!(made.manifest.unwrap().workbook, Some(title.clone()));
        // The window turns to what was made, which is not trusted.
        h.get_by_label("Anie Development Kit  ·  Workbook code");
        h.get_by_label(&format!("It says it is the code of the workbook {title:?}, which is open now."));
        h.get_by_label(&format!("Budget Macros was created, as the code of {title}. It is not trusted yet."));
        assert_eq!(h.state().dialog.as_ref().and_then(|d| d.values.get("part")), Some(&json!("project:budget-macros")));
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
    fn the_showcase_is_created_trusted_by_the_person_and_then_docked() {
        use crate::aniefork::dock;
        let s = Sandbox::new("agents-showcase");
        let asked = yes();
        let mut app = app_with(Some(&asked));
        let opened = std::rc::Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
        let seen = opened.clone();
        app.services.open_url = Some(Box::new(move |address: &str| seen.borrow_mut().push(address.to_string())));
        let mut h = harness(app);
        open_top(&mut h);
        go(&mut h, "Anie Development Kit");
        // Created like any module: on disk, and not trusted.
        click(&mut h, "Create Showcase");
        assert!(s.0.join("modules/svelte-showcase/panel/app.js").is_file() && s.0.join("modules/svelte-showcase/source/src/App.svelte").is_file());
        let listed = list();
        let made = listed["modules"].as_array().unwrap().iter().find(|m| m["id"] == showcase::ID).unwrap().clone();
        assert_eq!((made["status"].as_str(), made["name"].as_str()), (Some("untrusted"), Some("Svelte Showcase")));
        assert_eq!(made["asks"].as_array().unwrap().iter().map(|a| a["permission"].as_str().unwrap()).collect::<Vec<_>>(), ["read", "write"]);
        h.get_by_label_contains("Svelte Showcase was created. It is not trusted yet");
        h.get_by_label_contains("review and trust it in Modules, then come back");
        assert!(h.query_by_label("Dock Showcase").is_none() && h.query_by_label("Create Showcase").is_none());
        assert!(asked.questions().is_empty(), "creating it asked nothing and trusted nothing");
        // The person trusts it, as they would any module: the operating system asks.
        trust(h.state(), showcase::ID, made["fingerprint"].as_str().unwrap(), false).unwrap();
        assert_eq!(asked.questions().len(), 1);
        h.run_steps(3);
        h.get_by_label("Svelte Showcase is ready.");
        // A copy of GridCraft without a web view offers the browser, and only that.
        assert!(h.query_by_label("Dock Showcase").is_none());
        click(&mut h, "Open in Browser");
        assert_eq!(opened.borrow().len(), 1);
        // With one, the panel is docked beside the sheet, and the window makes way for it.
        let ctx = h.ctx.clone();
        dock::host_is_here(&ctx);
        h.run_steps(3);
        // Not by the control channel: showing a panel is the person's to do.
        drive(&mut h);
        click(&mut h, "Dock Showcase");
        assert_eq!(message(&mut h).as_deref(), Some(presence::BY_HAND));
        assert!(dock::docked(&ctx).is_none());
        quiet(&mut h);
        click(&mut h, "Dock Showcase");
        assert!(h.state().dialog.is_none(), "{:?}", h.state().message);
        let docked = dock::docked(&ctx).unwrap();
        assert_eq!((docked.module.as_str(), docked.name.as_str(), docked.network), (showcase::ID, "Svelte Showcase", false));
        h.run_steps(3);
        assert!(dock::wanted(&ctx).is_some_and(|wanted| wanted.rect.is_some()));
        assert_eq!(docked.address, opened.borrow()[0], "the same panel the browser was given, on the same bridge");
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
