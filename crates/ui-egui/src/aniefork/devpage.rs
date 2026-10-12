//! The Development window as a page (ADR-0001, phase 2d): for a shell that can show a web
//! page in a window of its own, beside GridCraft's.
//!
//! GridCraft's own Development window (`development.rs`) is a dialog over the sheet. Where
//! the desktop shell has a web view, the same window is shown as a page, written in Svelte,
//! in a window of its own that stays open while the person works. There are two halves, as
//! with a docked panel:
//!
//! - **This module** says what the window shows ([`state`]): the list down its side and the
//!   one part beside it, as JSON, in the words `card.rs` and `kits.rs` give both windows. And
//!   it takes what the person clicked there ([`handle`]),
//!   which goes through the same [`development::act`] as a click in GridCraft's own window.
//! - **The shell** has the window and the web view. It serves the page from files compiled
//!   into the app, hands it the state when that changes, and passes on its messages.
//!
//! **A message from the page is taken as the person's own click.** That holds only because of
//! what the shell keeps to, so it is said here where it is relied on: the page is loaded from
//! the app's own files and from nowhere else, under a policy that lets it run its own script
//! and nothing more; it can't go to another address or open another window; and a message is
//! taken only from that page. The control channel can't reach it: what it clicks and types
//! goes into GridCraft's window, never into the web view. Everything a request holds is drawn
//! by the page as text, never as markup.
//!
//! What stays as it is in either window: the token is never part of what the page is told
//! (the setup goes to the clipboard, by GridCraft); raising the level, and a yes when the
//! control channel is open, end in the operating system's own question; a yes in a card's
//! first moment on screen, or to a card that is not the one on screen, answers nothing
//! (GridCraft keeps the time and the card, not the page).

use gridcraft_aniefork_dev::agents::{Action, Asked, Kind, Level};
use gridcraft_aniefork_dev::bridge::Places;
use serde_json::{Value as Json, json};

use super::development::{self, Act, State};
use super::kits::{self, Pane};
use super::{agents, card, modules};
use crate::SheetApp;
use crate::theme::Tokens;

/// The longest message taken from the page (bytes): a click, and at most a few words.
const MAX_MESSAGE: usize = 4096;
/// The most of what the person tells the agent with a no (characters).
const MAX_REASON: usize = 400;
/// The longest name typed for a new project (characters).
const MAX_NAME: usize = 80;
/// How often what the window shows is worked out again while nothing was clicked (milliseconds).
const EVERY_MS: f64 = 300.0;

/// How the page's window stands.
#[derive(Clone, Default)]
struct Page {
    /// The shell can show the page in a window of its own.
    host: bool,
    /// The person wants the window open.
    wanted: bool,
    /// The window is to be brought to the front.
    raise: bool,
    /// The person chose GridCraft's own window, until GridCraft closes.
    builtin: bool,
    /// The window starts afresh: on the request that waits, when exactly one does.
    fresh: bool,
    /// What it shows has to be worked out again now.
    stale: bool,
    /// When it was last worked out (`now_ms`).
    told_at: f64,
    /// Why the last thing asked for there could not be done.
    error: String,
}

fn id() -> egui::Id {
    egui::Id::new("aniefork.devpage")
}

fn state_id() -> egui::Id {
    egui::Id::new("aniefork.devpage.state")
}

fn page(ctx: &egui::Context) -> Page {
    ctx.data(|data| data.get_temp(id())).unwrap_or_default()
}

fn keep(ctx: &egui::Context, page: Page) {
    ctx.data_mut(|data| data.insert_temp(id(), page));
}

/// The shell says it can show the Development window as a page, in a window of its own.
pub fn host_is_here(ctx: &egui::Context) {
    let mut now = page(ctx);
    if !now.host {
        now.host = true;
        keep(ctx, now);
    }
}

/// Whether the Development window is shown as a page: the shell can, and the person hasn't
/// chosen GridCraft's own window instead.
pub fn takes_over(ctx: &egui::Context) -> bool {
    let now = page(ctx);
    now.host && !now.builtin
}

/// Where the page's web view keeps what it keeps: a folder of its own beside the settings,
/// apart from the one modules' panels use.
pub fn web_view_folder() -> Option<std::path::PathBuf> {
    modules::places().map(|at| at.settings.with_file_name("webview-development"))
}

/// The person asked for the window: it is opened, or brought to the front.
pub fn open(ctx: &egui::Context) {
    let mut now = page(ctx);
    now.wanted = true;
    now.raise = true;
    now.fresh = true;
    now.stale = true;
    now.error.clear();
    keep(ctx, now);
}

/// Whether the person wants the window open.
pub fn wanted(ctx: &egui::Context) -> bool {
    page(ctx).wanted
}

/// Whether the window is to be brought to the front; asked once.
pub fn take_raise(ctx: &egui::Context) -> bool {
    let mut now = page(ctx);
    let raise = std::mem::take(&mut now.raise);
    if raise {
        keep(ctx, now);
    }
    raise
}

/// The window was closed, by its own close box.
pub fn closed(ctx: &egui::Context) {
    let mut now = page(ctx);
    now.wanted = false;
    keep(ctx, now);
}

/// The shell could not show the window: GridCraft's own is used instead, until it closes.
pub fn failed(app: &mut SheetApp, ctx: &egui::Context, why: &str) {
    let mut now = page(ctx);
    now.wanted = false;
    now.builtin = true;
    keep(ctx, now);
    log::warn!("the Development window could not be shown as a page: {why}");
    app.open_dialog(development::NAME, json!({}));
}

/// Hook of the fork's work of every frame: where the Development window is shown as a page,
/// asking for GridCraft's own (from the ribbon, the bar under it, or by name) opens the page.
pub fn route(app: &mut SheetApp, ctx: &egui::Context) {
    if app.dialog.as_ref().is_some_and(|d| d.name == development::NAME) && takes_over(ctx) {
        app.dialog = None;
        open(ctx);
    }
}

fn colour(colour: egui::Color32) -> String {
    format!("#{:02x}{:02x}{:02x}", colour.r(), colour.g(), colour.b())
}

fn tint(colour: egui::Color32) -> String {
    format!("rgba({}, {}, {}, 0.18)", colour.r(), colour.g(), colour.b())
}

/// The app's colours, so that the page looks like the window it belongs to.
fn theme(ctx: &egui::Context) -> Json {
    let t = Tokens::get(ctx);
    let visuals = ctx.global_style().visuals.clone();
    json!({
        "dark": t.dark,
        "window": colour(t.window),
        "text": colour(t.text),
        "dim": colour(t.text_dim),
        "accent": colour(t.accent),
        "onAccent": "#ffffff",
        "danger": colour(t.danger),
        "warn": colour(visuals.warn_fg_color),
        "border": colour(t.separator),
        "input": colour(t.input_bg),
        "block": colour(visuals.extreme_bg_color),
        "hover": colour(t.hover),
        "added": tint(t.accent),
        "removed": tint(t.danger),
    })
}

/// One request, in full, for the person to answer.
fn review(at: &Places, asked: &Asked, state: &mut State, confirms: bool) -> Json {
    let sight = card::sight_now(at, &asked.action);
    let new = card::is_new(&sight);
    // A plan's steps, followed through: how each will find what it is about.
    let followed = match &asked.action {
        Action::Plan { steps, stood, .. } => Some(state.follow(at, asked.id, steps, stood)),
        _ => None,
    };
    let content = match (&asked.action, &followed) {
        (Action::Plan { purpose, steps, minutes, .. }, Some(sights)) => card::plan(at, purpose, steps, *minutes, sights, state.whole),
        _ => card::content(at, &asked.action, &sight, false, state.whole),
    };
    let cut = card::too_long(&asked.action);
    let unfit = followed.as_ref().is_some_and(|sights| sights.is_err());
    let three = agents::can_go_unasked(asked);
    // GridCraft keeps the time: the page only shows it.
    let ready_in = (development::SETTLE_MS - (crate::now_ms() - state.shown_at)).clamp(0.0, development::SETTLE_MS).ceil();
    json!({
        "id": asked.id,
        "kind": card::kind(&asked.action, new),
        "named": card::named(asked),
        "target": card::target(&asked.action),
        "long": followed.is_some(),
        "content": card::json(&content),
        "means": card::json(&card::means(at, &asked.action)),
        "question": card::question(asked, new),
        "readyIn": ready_in,
        "blocked": cut || unfit,
        "blockedWhy": if cut { json!(card::TOO_LONG) } else { Json::Null },
        "yes": if followed.is_some() { card::YES_PLAN } else { card::YES },
        "yesAnd": if three { json!(card::yes_and(&asked.action)) } else { Json::Null },
        "no": if three { card::NO_OF_THREE } else { card::NO },
        "reasonHint": card::REASON,
        "notNow": card::NOT_NOW,
        "asker": card::asker(asked, confirms),
    })
}

/// The MCP server's part: the level, how to connect, who connected, what waits, the plans.
fn server_part(now: &agents::Snapshot, state: &State) -> Json {
    if !now.available {
        return json!({"kind": "server", "title": kits::SERVER, "above": [kits::TOP], "available": false, "unavailable": card::NO_AGENTS});
    }
    let at = modules::places();
    let levels: Vec<Json> =
        Level::ALL.iter().map(|level| json!({"code": level.code(), "label": level.label(), "meaning": level.meaning()})).collect();
    let (kind, status) = card::serving(now);
    let waiting: Vec<Json> = now
        .waiting
        .iter()
        .map(|asked| {
            let new = at.as_ref().is_some_and(|at| card::writes_new(at, &asked.action));
            json!({"id": asked.id, "kind": card::kind(&asked.action, new), "target": card::target(&asked.action), "named": card::named(asked)})
        })
        .collect();
    // The plan that was approved, and the last one that ended.
    let plans: Vec<Json> = now
        .plans
        .iter()
        .take(2)
        .map(|plan| {
            let steps: Vec<Json> = plan
                .steps
                .iter()
                .enumerate()
                .map(|(index, step)| {
                    json!({"what": format!("{}. {}", index + 1, step.action.title()), "state": step.state.code(), "stands": card::step_stands(&step.state)})
                })
                .collect();
            json!({
                "id": plan.id,
                "name": format!("Plan #{}", plan.id),
                "title": format!("{:?}", plan.title),
                "stands": card::plan_stands(plan),
                "live": plan.ended.is_none(),
                "steps": steps,
            })
        })
        .collect();
    let unasked: Vec<Json> = now
        .unasked
        .iter()
        .map(|(module, kind)| json!({"module": module, "kind": if *kind == Kind::Edit { "edit" } else { "run" }, "covers": agents::covers(module, *kind)}))
        .collect();
    json!({
        "kind": "server",
        "title": kits::SERVER,
        "above": [kits::TOP],
        "available": true,
        "intro": card::AGENTS,
        "level": now.level.code(),
        "levels": levels,
        "meaning": now.level.meaning(),
        "status": {"kind": kind, "text": status},
        "clients": card::clients(now),
        "serving": now.serving,
        "portText": state.port,
        "waitingTitle": format!("Waiting for your answer ({})", now.waiting.len()),
        "waiting": waiting,
        "plans": plans,
        "unasked": unasked,
        "lately": card::lately(now),
        "latelyNote": card::RECORD,
    })
}

/// What the window shows, as JSON, when it is time to work it out again: something was
/// clicked there, or a moment has passed. `None` when the window isn't wanted, or it is not
/// time yet. It never holds the token.
pub fn state(app: &mut SheetApp, ctx: &egui::Context) -> Option<String> {
    let mut page = page(ctx);
    if !page.wanted {
        return None;
    }
    let at_ms = crate::now_ms();
    if !page.stale && (0.0..EVERY_MS).contains(&(at_ms - page.told_at)) {
        return None;
    }
    let mut state: State = ctx.data(|data| data.get_temp(state_id())).unwrap_or_default();
    if std::mem::take(&mut page.fresh) {
        state = State::fresh();
    }
    let now = agents::snapshot();
    // A request being looked at takes the place of the rest, while it still waits.
    let looked_at = state.looking.and_then(|id| now.waiting.iter().find(|asked| asked.id == id));
    let looking = match (looked_at, modules::places()) {
        (Some(asked), Some(at)) => review(&at, asked, &mut state, agents::confirms(app)),
        _ => {
            state.looking = None;
            Json::Null
        }
    };
    // A project that is no longer there gives way to its kit.
    if let Pane::Project(id) = &state.pane
        && kits::project_pane(app, id).is_none()
    {
        state.pane = Pane::Kit;
    }
    // The list down the side, and the one part that is shown beside it.
    let part = match &state.pane {
        Pane::Server => server_part(&now, &state),
        other => kits::pane(app, ctx, other, &now),
    };
    let told = json!({
        "theme": theme(ctx),
        "title": kits::TOP,
        "said": state.said,
        "error": page.error,
        "looking": looking,
        "nav": kits::nav(app, &state.pane, &now),
        "pane": part,
    });
    page.stale = false;
    page.told_at = at_ms;
    keep(ctx, page);
    ctx.data_mut(|data| data.insert_temp(state_id(), state));
    Some(told.to_string())
}

/// What the person clicked in the page, as the page says it. Returns whether the page is to
/// be told everything again whether or not it changed (it has just loaded).
///
/// Whoever calls this has made sure the message is from the page GridCraft itself serves: it
/// is taken as the person's own click.
pub fn handle(app: &mut SheetApp, ctx: &egui::Context, message: &str) -> bool {
    if message.len() > MAX_MESSAGE {
        return false;
    }
    let Ok(message) = serde_json::from_str::<Json>(message) else { return false };
    let Some(name) = message.get("act").and_then(Json::as_str) else { return false };
    let mut page = page(ctx);
    if !page.wanted {
        return false;
    }
    let mut state: State = ctx.data(|data| data.get_temp(state_id())).unwrap_or_default();
    let id = || message.get("id").and_then(Json::as_u64);
    let action = match name {
        // The page has loaded: it is told what to show.
        "ready" => {
            page.stale = true;
            keep(ctx, page);
            return true;
        }
        // GridCraft's own window instead, until GridCraft closes.
        "builtin" => {
            page.builtin = true;
            page.wanted = false;
            keep(ctx, page);
            app.open_dialog(development::NAME, json!({}));
            return false;
        }
        "level" => message.get("level").and_then(Json::as_str).and_then(Level::parse).map(Act::Level),
        "port" => message.get("port").and_then(Json::as_str).filter(|text| text.len() <= 8).map(|text| Act::Port(text.to_string())),
        "copySetup" => Some(Act::CopySetup { as_json: message.get("json").and_then(Json::as_bool).unwrap_or(false) }),
        "newToken" => Some(Act::NewToken),
        "look" => id().map(Act::Look),
        "notNow" => Some(Act::NotNow),
        "whole" => message.get("whole").and_then(Json::as_bool).map(Act::Whole),
        "allow" => id().map(Act::Allow),
        "allowNotAgain" => id().map(Act::AllowNotAgain),
        "refuse" => id().map(|id| {
            // The word for the agent is what the person typed in the page.
            state.reason = message.get("reason").and_then(Json::as_str).unwrap_or("").chars().take(MAX_REASON).collect();
            Act::Refuse(id)
        }),
        "askAgain" => {
            let kind = match message.get("kind").and_then(Json::as_str) {
                Some("edit") => Some(Kind::Edit),
                Some("run") => Some(Kind::Run),
                _ => None,
            };
            let module = message.get("module").and_then(Json::as_str).filter(|module| module.len() <= 64);
            kind.zip(module).map(|(kind, module)| Act::AskAgain(module.to_string(), kind))
        }
        "stopPlan" => id().map(Act::StopPlan),
        // Another part of the window.
        "go" => message.get("to").and_then(Json::as_str).filter(|to| to.len() <= 80).and_then(Pane::parse).map(Act::Go),
        "open" => match message.get("name").and_then(Json::as_str) {
            Some("modules") => Some(Act::Open(modules::NAME)),
            Some("console") => Some(Act::Open(super::console::NAME)),
            Some("macros") => Some(Act::Open(super::macros::NAME)),
            _ => None,
        },
        "openFolder" => Some(Act::OpenFolder),
        "openProjectFolder" => message.get("id").and_then(Json::as_str).filter(|id| id.len() <= 64).map(|id| Act::OpenProjectFolder(id.to_string())),
        // A new project: its name is what the person typed, and the workbook it is the code
        // of is the one that is open (GridCraft says which, not the page).
        "createProject" => message.get("name").and_then(Json::as_str).filter(|name| name.chars().count() <= MAX_NAME).map(|name| {
            Act::CreateProject { name: name.to_string(), for_workbook: message.get("forWorkbook").and_then(Json::as_bool).unwrap_or(false) }
        }),
        "createShowcase" => Some(Act::CreateShowcase),
        "showShowcase" => message.get("docked").and_then(Json::as_bool).map(|docked| Act::ShowShowcase { docked }),
        _ => None,
    };
    let Some(action) = action else { return false };
    // It closes nothing here: GridCraft's own window is not the one that is open.
    let mut open = true;
    match development::act(app, ctx, &mut state, action, false, &mut open) {
        Ok(said) => {
            state.said = said;
            page.error.clear();
        }
        Err(why) => {
            state.said.clear();
            page.error = why;
        }
    }
    page.stale = true;
    keep(ctx, page);
    ctx.data_mut(|data| data.insert_temp(state_id(), state));
    false
}
