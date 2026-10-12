//! Agents: the app's side of the Development server (`gridcraft_aniefork_dev::agents`), the
//! second MCP server, for agents that help with development work.
//!
//! The server listens, checks who is asking, answers what only reads, and keeps the queue of
//! what agents ask to have done. This module does the rest, on the UI thread, once a frame
//! ([`poll`]):
//!
//! - it brings the server in line with the settings when the app starts;
//! - it takes the requests that came in and decides what becomes of each, from the level as it
//!   is at that moment: carried out at once, kept for the person to answer, or refused;
//! - it hands back what finished scripts gave.
//!
//! **How far agents may go is set by the person at the screen and by nothing else.** There is
//! no command for the level, the port or the token (`developer.agents` only reports), and the
//! Development window, where they are set, refuses while the control channel is putting input
//! into the window. Raising the level ends in a question drawn by the operating system, which
//! the control channel's clicks cannot answer. So does a yes to a request, when GridCraft runs
//! with the control channel: without it nothing but the person can click in the window, and the
//! yes on the card is enough ([`confirms`]).
//!
//! With a yes the person may say not to be asked again, for changes to that module's files or
//! for running its code, until GridCraft closes. The server keeps that; lowering the level
//! ends it; and it is never offered for a module kept trusted as its files change.
//!
//! The token is a secret: it is kept in a file of its own, handed to the clipboard when the
//! person asks for the setup, and neither drawn in the window nor reported by anything. The
//! control channel can't open that file, or any other in GridCraft's own settings folder
//! (`super::control_refuses`).
//!
//! **A change nobody looked at ends the trust in a module.** When the level lets an agent change
//! a module's files without asking, the change takes the module's trust back, whatever the
//! files hold afterwards ([`perform`]); and the crate refuses a change while one of the
//! module's scripts is being checked or run. So what runs is what the person trusted.

use std::cell::RefCell;
use std::sync::{Arc, Mutex, PoisonError};

use gridcraft_aniefork_dev::agents::{self as dev, Action, Asked, Client, Done, Level, Policy, Record, Server};
use gridcraft_aniefork_dev::bridge::Places;
use gridcraft_aniefork_dev::python::{Output, PythonHost, RunError};
use gridcraft_aniefork_dev::trust::Status;
use gridcraft_aniefork_dev::{TrustStore, module, settings, starter};
use serde_json::{Value as Json, json};

use super::{modules, panels};
use crate::{Notice, NoticeKind, SheetApp};

/// What the bar under the ribbon says while a request waits for the person.
const ASKING: &str = "An agent is asking for your answer in Development";
/// What it says once agents were given full control.
const FULL: &str = "Agents have full control of your development tools until GridCraft closes.";
/// Why a request the person said no to was refused, as the agent is told.
const SAID_NO: &str = "The person at the screen said no.";
/// The most of what the person tells the agent with a no (characters).
const MAX_REASON: usize = 400;
/// The most scripts that run for agents at once.
const MAX_RUNNING: usize = 4;
pub(super) const NO_CONFIRM: &str = "This copy of GridCraft can't ask your operating system to confirm, so this can't be done here.";
const OFF: &str = "Agent access is off.";

/// A script an agent asked for, running off the UI thread.
#[derive(Clone)]
struct Job {
    request: u64,
    module: String,
    script: String,
    slot: Arc<Mutex<Option<Result<Output, RunError>>>>,
}

/// This window's Development server, and how it stands.
#[derive(Clone, Default)]
struct Running {
    /// The settings have been read, and the server brought in line with them.
    settled: bool,
    /// How far agents may go now. Full control is here and never in the settings.
    level: Level,
    port: u16,
    server: Option<Server>,
    /// Why nothing is served although something should be.
    problem: String,
    jobs: Vec<Job>,
}

thread_local! {
    /// The Development server of the app on this thread. It is kept here, and not in the
    /// window's memory with the fork's other state, because a command that reports it
    /// (`developer.agents`) has no window to ask.
    static RUNNING: RefCell<Running> = RefCell::new(Running::default());
}

fn running() -> Running {
    RUNNING.with(|kept| kept.borrow().clone())
}

fn keep(now: Running) {
    // The one before is let go outside the borrow: a server that goes writes its record.
    let before = RUNNING.with(|kept| std::mem::replace(&mut *kept.borrow_mut(), now));
    drop(before);
}

/// In tests: forgets this thread's server, as when another app starts.
#[cfg(test)]
pub(crate) fn forget() {
    keep(Running::default());
}

/// The port to listen on. Tests take any free one, whatever the settings say: they must never
/// meet the developer's own GridCraft.
fn port_to_use(kept: u16) -> u16 {
    if cfg!(test) { 0 } else { kept }
}

/// The token: what is kept, or, with `make`, a new one when none is.
fn token(at: &Places, make: bool) -> Result<String, String> {
    let file = dev::token_file(at);
    let small = std::fs::metadata(&file).is_ok_and(|found| found.is_file() && found.len() <= 1024);
    let kept = small
        .then(|| std::fs::read_to_string(&file).ok())
        .flatten()
        .map(|text| text.trim().to_string())
        .filter(|token| token.len() == 64 && token.chars().all(|c| c.is_ascii_hexdigit()));
    match kept {
        Some(token) => Ok(token),
        None if make => new_token(at),
        None => Err("There is no token yet: turn agent access on first.".into()),
    }
}

/// Makes a new token and keeps it. Whoever held the one before is locked out.
fn new_token(at: &Places) -> Result<String, String> {
    let token = dev::new_token()?;
    let file = dev::token_file(at);
    let failed = |e: std::io::Error| format!("The token could not be kept: {e}");
    if let Some(dir) = file.parent() {
        std::fs::create_dir_all(dir).map_err(failed)?;
    }
    // It is for this user alone, from the moment it is made. (On Windows the settings folder
    // already is.)
    let _ = std::fs::remove_file(&file);
    let mut made = std::fs::OpenOptions::new();
    made.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        made.mode(0o600);
    }
    let mut kept = made.open(&file).map_err(failed)?;
    std::io::Write::write_all(&mut kept, token.as_bytes()).map_err(failed)?;
    Ok(token)
}

/// Lets the server that is running go, and starts one for the level and port as they are now.
fn start(ctx: &egui::Context, at: &Places, now: &mut Running) {
    // The one before is let go first: it frees its port, and refuses what was waiting. What
    // its scripts give when they finish is nobody's any more: the next server numbers its
    // requests from one again.
    now.server = None;
    now.jobs.clear();
    now.problem.clear();
    if now.level == Level::Off {
        return;
    }
    let window = ctx.clone();
    let made = token(at, true).and_then(|token| Server::start(move || window.request_repaint(), at, now.port, &token, now.level));
    match made {
        Ok(server) => {
            server.set_commands(panels::command_docs());
            now.server = Some(server);
        }
        Err(why) => now.problem = why,
    }
}

/// Hook of the fork's work of every frame: the Development server's requests.
pub fn poll(app: &mut SheetApp, ctx: &egui::Context) {
    let mut now = running();
    if !now.settled {
        now.settled = true;
        if let Some(at) = modules::places() {
            let kept = settings::load(&at.settings).agents;
            now.level = kept.level;
            now.port = port_to_use(kept.port);
            start(ctx, &at, &mut now);
        }
        keep(now.clone());
    }
    let Some(server) = now.server.clone() else { return };
    let mut changed = false;
    // Scripts that finished since the last frame.
    let before = now.jobs.len();
    now.jobs.retain(|job| match job.slot.lock().unwrap_or_else(PoisonError::into_inner).take() {
        Some(result) => {
            server.finish(job.request, script_result(job, result));
            false
        }
        None => true,
    });
    changed |= now.jobs.len() != before;
    // What came in: carried out, kept for the person, or refused, by the level as it is now.
    let mut asking = false;
    for asked in server.take_new() {
        match server.decides(&asked.action) {
            Policy::Allowed => {
                let _ = carry_out(app, ctx, &mut now, &server, asked.id, false);
                changed = true;
            }
            Policy::Asks => asking = true,
            Policy::Refused => server.refuse(asked.id, "Agent access was lowered.", false),
        }
    }
    if changed {
        keep(now);
    }
    notice(app, &server, asking);
}

/// Tells the person that a request is waiting for them, in the bar under the ribbon, and takes
/// that down when none is.
fn notice(app: &mut SheetApp, server: &Server, new: bool) {
    let ours = app.notice.as_ref().is_some_and(|notice| notice.text.starts_with(ASKING));
    if !new && !ours {
        return;
    }
    let waiting = server.waiting().len();
    let looking = app.dialog.as_ref().is_some_and(|d| d.name == super::development::NAME);
    if waiting == 0 || looking {
        if ours {
            app.notice = None;
        }
        return;
    }
    let text = format!("{ASKING} ({waiting} waiting).");
    if ours || app.notice.is_none() {
        app.notice = Some(Notice { text, kind: NoticeKind::Info, action: Some(("Review…".into(), super::development::NAME.into())) });
    } else if new {
        // The bar is saying something else: this is said beside it.
        app.toast = Some((format!("{text} See Developer ▸ Development."), crate::now_ms()));
    }
}

/// What a finished script gave, for the agent that asked for it.
fn script_result(job: &Job, result: Result<Output, RunError>) -> Result<Done, String> {
    match result {
        Ok(out) => Ok(Done {
            said: format!("{} of {} returned.", job.script, job.module),
            data: Some(json!({"result": out.result, "printed": out.printed})),
        }),
        Err(RunError::Script { error, trace }) => Err(format!("{error}\n{}", trace.trim_end())),
        Err(other) => Err(other.to_string()),
    }
}

/// Where a module stands now, in words for the agent.
fn standing(at: &Places, id: &str) -> String {
    let found = module::load(&at.modules.join(id));
    match TrustStore::load(&at.trust).status(&found) {
        Status::Trusted(_) => format!("{id} is trusted as it is now."),
        Status::Untrusted => format!("{id} is not trusted: nothing in it runs until it is."),
        Status::Changed => format!("{id} is no longer the module that was trusted: nothing in it runs until it is trusted again as it is now."),
        Status::Problem(_) => format!("{id} can't be used as it is now: list_modules says what is wrong with it."),
    }
}

/// Starts a script an agent asked for. The trust check is the host's own, on the thread that
/// runs it; what the script asks of the spreadsheet is carried out by this window, as a panel's
/// calls are, with what the module was granted.
fn run(app: &mut SheetApp, ctx: &egui::Context, at: &Places, request: u64, action: &Action) -> Result<Job, String> {
    let Action::RunScript { module, script, data, on_selection, timeout } = action else { return Err("That is not a script to run.".into()) };
    let data = if *on_selection { modules::selection_data(app)? } else { data.clone() };
    let python = settings::load(&at.settings).python;
    let job = Job { request, module: module.clone(), script: script.clone(), slot: Arc::new(Mutex::new(None)) };
    let calls = panels::script_calls(ctx, at, module);
    let (slot, repaint, at, module, script, timeout) = (job.slot.clone(), ctx.clone(), at.clone(), module.clone(), script.clone(), *timeout);
    std::thread::Builder::new()
        .name("gridcraft-agent-run".into())
        .spawn(move || {
            let result = PythonHost::global().run_with(&at.modules, &at.trust, &python, &module, &script, &data, timeout, Some(&calls));
            *slot.lock().unwrap_or_else(PoisonError::into_inner) = Some(result);
            repaint.request_repaint();
        })
        .map_err(|e| format!("The script could not be started: {e}"))?;
    Ok(job)
}

/// Takes a module's trust back, and stops what of it is running or being served.
fn take_trust_back(ctx: &egui::Context, at: &Places, id: &str) -> Result<(), String> {
    panels::stop(ctx, id);
    TrustStore::load(&at.trust).revoke(id)?;
    PythonHost::global().stop(&at.modules, id);
    Ok(())
}

/// Does what an agent asked for, apart from running a script. `unasked`: nobody was shown it
/// (the level let it through), and the level is not full control.
fn perform(app: &mut SheetApp, ctx: &egui::Context, at: &Places, action: &Action, unasked: bool) -> Result<Done, String> {
    let said = |said: String| Ok(Done { said, data: None });
    // A change to a module's files that nobody looked at ends the trust in that module, whatever
    // the files hold afterwards. Trust is in contents the person chose to trust: putting the old
    // contents back must not bring it back, or a file could be swapped for the length of one
    // run and nobody the wiser.
    let untrusts = unasked && matches!(action, Action::WriteFile { .. } | Action::DeleteFile { .. });
    if untrusts {
        take_trust_back(ctx, at, action.module())?;
    }
    let after = |id: &str| if untrusts { format!("{id} is not trusted: nothing in it runs until it is.") } else { standing(at, id) };
    match action {
        Action::CreateModule { name } => {
            std::fs::create_dir_all(&at.modules).map_err(|e| format!("The modules folder could not be created: {e}"))?;
            let made = starter::create(&at.modules, name)?;
            said(format!(
                "{} was created as the module {}, with {} files. It is not trusted: nothing in it runs until it is.",
                made.name(),
                made.id,
                made.files.len()
            ))
        }
        Action::WriteFile { module, path, text } => {
            let written = dev::write_file(at, module, path, text)?;
            let how = if written.created { "created" } else { "written over" };
            said(format!("{path} was {how} in {module} ({} bytes). {}", written.bytes, after(module)))
        }
        Action::DeleteFile { module, path } => {
            dev::delete_file(at, module, path)?;
            said(format!("{path} was deleted from {module}. {}", after(module)))
        }
        Action::OpenPanel { module } => panels::open(app, ctx, module).and_then(said),
        Action::Trust { module, fingerprint } => {
            let found = module::load(&at.modules.join(module));
            // Trust goes to the contents that were asked about, and to no others.
            if found.fingerprint.as_deref() != Some(fingerprint.as_str()) {
                return Err(format!("{module} changed after this was asked: it was not trusted. Ask again for the module as it is now."));
            }
            // Trusted as it is now: never "as it changes", which only the person chooses.
            TrustStore::load(&at.trust).trust(&found, false)?;
            said(format!("{module} is trusted as it is now."))
        }
        Action::StopTrusting { module } => {
            take_trust_back(ctx, at, module)?;
            said(format!("{module} is no longer trusted."))
        }
        Action::RunScript { .. } => Err("That is a script to run.".into()),
    }
}

/// Carries out the request `id`: it was let through by the level, or the person said yes.
/// Returns what to tell the person.
fn carry_out(app: &mut SheetApp, ctx: &egui::Context, now: &mut Running, server: &Server, id: u64, asked_first: bool) -> Result<String, String> {
    let Some(asked) = server.begin(id, asked_first) else { return Err("That request is no longer waiting: it lapsed, or was answered.".into()) };
    let Some(at) = modules::places() else {
        server.finish(id, Err(modules::UNAVAILABLE.into()));
        return Err(modules::UNAVAILABLE.into());
    };
    if matches!(asked.action, Action::RunScript { .. }) {
        if now.jobs.len() >= MAX_RUNNING {
            let why = format!("{MAX_RUNNING} scripts are running for agents already. Wait for one to finish.");
            server.finish(id, Err(why.clone()));
            return Err(why);
        }
        return match run(app, ctx, &at, id, &asked.action) {
            Ok(job) => {
                now.jobs.push(job);
                Ok("The script is running.".into())
            }
            Err(why) => {
                server.finish(id, Err(why.clone()));
                Err(why)
            }
        };
    }
    let unasked = !asked_first && server.level() != Level::Full;
    let result = perform(app, ctx, &at, &asked.action, unasked);
    server.finish(id, result.clone());
    result.map(|done| done.said)
}

/// The question the operating system puts to the person before a request is carried out: who
/// asks, for what, and what it means.
fn question(at: &Places, asked: &Asked) -> String {
    let who = if asked.client == "an agent" { "An agent".to_string() } else { format!("An agent that calls itself \"{}\"", asked.client) };
    let follows = |module: &str| TrustStore::load(&at.trust).follows_changes(module);
    let changes = |module: &str| {
        if follows(module) {
            format!("You chose to keep trusting {module} as its files change: what is written runs without your looking at it again.")
        } else {
            format!("{module} stops being trusted when its files change: nothing in it runs until you review it again.")
        }
    };
    let means = match &asked.action {
        Action::CreateModule { .. } => "The new module is not trusted: it does nothing until you trust it.".to_string(),
        Action::WriteFile { module, .. } | Action::DeleteFile { module, .. } => changes(module),
        Action::RunScript { .. } => "A script can do anything a program you run can do.".to_string(),
        Action::OpenPanel { .. } => "Its panel can do what you granted the module.".to_string(),
        Action::Trust { module, .. } => {
            let found = module::load(&at.modules.join(module));
            return format!("{who} asks you to trust a module (request #{}).\n\n{}", asked.id, modules::question(&found, false));
        }
        Action::StopTrusting { .. } => "It does nothing after that until you trust it again.".to_string(),
    };
    format!("{who} asks GridCraft to:\n\n{}\n\n{means}\n\n{}Allow it, this once?", asked.action.title(), tag(asked))
}

/// What ties the operating system's question to the card the person read: the request's
/// number, and the mark of what it holds.
fn tag(asked: &Asked) -> String {
    match dev::mark(&asked.action).as_str() {
        "" => format!("This is request #{}.\n\n", asked.id),
        mark => format!("This is request #{}, holding what its card marks {mark}.\n\n", asked.id),
    }
}

/// Whether a yes to a request is confirmed by the operating system's own question. It is when
/// GridCraft runs with the control channel, which can click in the window as a person does.
/// Without the channel nothing but the person can click there, and the yes on the card is
/// theirs.
pub(super) fn confirms(app: &SheetApp) -> bool {
    app.control_rx.is_some()
}

/// What the person answered a request with.
pub(super) enum Said {
    /// Yes, this once.
    Yes,
    /// Yes, and not to ask again for this kind of thing in this module until GridCraft closes.
    YesAndNotAgain,
    /// No, with a word (or none) on what the agent should do differently.
    No(String),
}

/// Whether "and don't ask again" can be said to this request: it is of a kind that can go
/// unasked, and its module is not one kept trusted as its files change.
pub(super) fn can_go_unasked(asked: &Asked) -> bool {
    let unreviewed = modules::places().is_some_and(|at| dev::runs_unreviewed(&at, &asked.action));
    dev::can_go_unasked(asked.action.kind()) && !asked.action.module().is_empty() && !unreviewed
}

/// What "don't ask again" would cover for this request, in words that follow "for".
pub(super) fn covers(module: &str, kind: dev::Kind) -> String {
    match kind {
        dev::Kind::Edit => format!("changes to {module}'s files"),
        _ => format!("running {module}'s scripts or opening its panel"),
    }
}

/// The person answered the request `id` in the Development window. A yes is confirmed by the
/// operating system's own question before anything is done, when the control channel is open
/// ([`confirms`]). A no may carry a word from the person on what the agent should do
/// differently, which the agent is told.
pub(super) fn answer(app: &mut SheetApp, ctx: &egui::Context, id: u64, said: Said) -> Result<String, String> {
    let mut now = running();
    let server = now.server.clone().ok_or(OFF)?;
    let gone = "That request is no longer waiting: it lapsed, or was answered.";
    let asked = server.waiting().into_iter().find(|asked| asked.id == id).ok_or(gone)?;
    let not_again = matches!(said, Said::YesAndNotAgain);
    if let Said::No(reason) = &said {
        let reason: String = reason.chars().filter(|c| !c.is_control()).take(MAX_REASON).collect::<String>().trim().to_string();
        if reason.is_empty() {
            server.refuse(id, SAID_NO, true);
            return Ok("The request was refused.".into());
        }
        server.refuse(id, &format!("{SAID_NO} They tell you: \"{reason}\""), true);
        return Ok("The request was refused, and the agent was told why.".into());
    }
    let at = modules::places().ok_or(modules::UNAVAILABLE)?;
    if let Action::Trust { module, fingerprint } = &asked.action
        && module::load(&at.modules.join(module)).fingerprint.as_deref() != Some(fingerprint.as_str())
    {
        server.refuse(id, "The module's files changed after this was asked. Ask again for the module as it is now.", false);
        return Err(format!("{module} changed after the agent asked. The request was dropped: the agent has to ask again."));
    }
    if not_again && !can_go_unasked(&asked) {
        return Err("That is asked about each time: it can't be allowed for the rest of the session.".into());
    }
    if confirms(app) {
        let Some(confirm) = &app.services.confirm else { return Err(NO_CONFIRM.into()) };
        let mut text = question(&at, &asked);
        if not_again {
            text.push_str(&format!(
                "\n\nAnd GridCraft will not ask you again for {} until it closes.",
                covers(asked.action.module(), asked.action.kind())
            ));
        }
        if !confirm("Allow this request from an agent?", &text) {
            server.refuse(id, SAID_NO, true);
            return Ok("The request was refused.".into());
        }
    }
    if not_again {
        server.allow_unasked(asked.action.module(), asked.action.kind());
    }
    let done = carry_out(app, ctx, &mut now, &server, id, true);
    keep(now);
    done
}

/// The person wants to be asked again about this kind of thing in this module.
pub(super) fn ask_again(module: &str, kind: dev::Kind) {
    if let Some(server) = &running().server {
        server.ask_again(module, kind);
    }
}

/// How the Development server stands, for the Development window.
pub(super) struct Snapshot {
    /// This run keeps settings, so it can have a server at all.
    pub available: bool,
    pub level: Level,
    pub port: u16,
    pub serving: bool,
    pub problem: String,
    pub clients: Vec<Client>,
    pub waiting: Vec<Asked>,
    pub recent: Vec<Record>,
    /// What the person said not to ask about again, until GridCraft closes.
    pub unasked: Vec<(String, dev::Kind)>,
}

pub(super) fn snapshot() -> Snapshot {
    let now = running();
    let server = now.server.as_ref();
    Snapshot {
        available: modules::places().is_some(),
        level: now.level,
        port: server.map_or(now.port, Server::port),
        serving: server.is_some(),
        problem: now.problem.clone(),
        clients: server.map(Server::clients).unwrap_or_default(),
        waiting: server.map(Server::waiting).unwrap_or_default(),
        recent: server.map(Server::recent).unwrap_or_default(),
        unasked: server.map(Server::unasked).unwrap_or_default(),
    }
}

/// Sets how far agents may go. The person chose it, in the Development window: whoever calls
/// this has made sure of that.
pub(super) fn set_level(app: &mut SheetApp, ctx: &egui::Context, level: Level) -> Result<String, String> {
    let at = modules::places().ok_or(modules::UNAVAILABLE)?;
    let mut now = running();
    now.settled = true;
    // Full control is for this run: the settings keep what was chosen before it.
    if level != Level::Full {
        let mut kept = settings::load(&at.settings);
        kept.agents.level = level;
        settings::save(&at.settings, &kept)?;
    }
    now.level = level;
    match (&now.server, level) {
        (_, Level::Off) => {
            now.server = None;
            now.problem.clear();
        }
        (Some(server), _) => {
            server.set_level(level);
            // What was waiting for an answer and is no longer allowed is refused now.
            for asked in server.waiting() {
                if server.decides(&asked.action) == Policy::Refused {
                    server.refuse(asked.id, "Agent access was lowered.", false);
                }
            }
        }
        (None, _) => {
            if now.port == 0 {
                now.port = port_to_use(settings::load(&at.settings).agents.port);
            }
            start(ctx, &at, &mut now);
        }
    }
    let problem = now.problem.clone();
    keep(now);
    let ours = app.notice.as_ref().is_some_and(|notice| notice.text == FULL);
    if level == Level::Full && (ours || app.notice.is_none()) {
        app.notice =
            Some(Notice { text: FULL.into(), kind: NoticeKind::Warning, action: Some(("Development…".into(), super::development::NAME.into())) });
    } else if ours {
        app.notice = None;
    }
    if !problem.is_empty() {
        return Err(problem);
    }
    Ok(format!("Agent access: {}.", level.label()))
}

/// Sets the port the server listens on, and moves a running server to it.
pub(super) fn set_port(ctx: &egui::Context, port: u16) -> Result<String, String> {
    let at = modules::places().ok_or(modules::UNAVAILABLE)?;
    if port < 1024 {
        return Err("Choose a port from 1024 to 65535.".into());
    }
    let mut kept = settings::load(&at.settings);
    kept.agents.port = port;
    settings::save(&at.settings, &kept)?;
    let mut now = running();
    now.settled = true;
    now.port = port_to_use(port);
    if now.level != Level::Off {
        start(ctx, &at, &mut now);
    }
    let problem = now.problem.clone();
    keep(now);
    if !problem.is_empty() {
        return Err(problem);
    }
    Ok(format!("The Development server listens on port {port}. Agents set up for another port need the setup again."))
}

/// Makes a new token: an agent that holds the old one can no longer connect.
pub(super) fn renew_token(ctx: &egui::Context) -> Result<String, String> {
    let at = modules::places().ok_or(modules::UNAVAILABLE)?;
    new_token(&at)?;
    let mut now = running();
    if now.level != Level::Off {
        start(ctx, &at, &mut now);
    }
    let problem = now.problem.clone();
    keep(now);
    if !problem.is_empty() {
        return Err(problem);
    }
    Ok("There is a new token. Agents set up with the old one can no longer connect: give them the setup again.".into())
}

/// What an agent needs to connect, as text for the clipboard: a command for Claude Code, or
/// the entry of an MCP settings file. **It holds the token.** It is handed to the person, by
/// their own click, and to nothing else.
pub(super) fn setup(as_json: bool) -> Result<String, String> {
    let at = modules::places().ok_or(modules::UNAVAILABLE)?;
    let now = running();
    let Some(server) = &now.server else { return Err(if now.problem.is_empty() { OFF.to_string() } else { now.problem.clone() }) };
    let token = token(&at, false)?;
    let address = format!("http://127.0.0.1:{}/mcp", server.port());
    if as_json {
        let entry = json!({"mcpServers": {"gridcraft-development": {"type": "http", "url": address, "headers": {"Authorization": format!("Bearer {token}")}}}});
        return serde_json::to_string_pretty(&entry).map_err(|e| e.to_string());
    }
    Ok(format!("claude mcp add --transport http gridcraft-development {address} --header \"Authorization: Bearer {token}\""))
}

/// What `developer.agents` reports. Reading only, and never the token.
pub fn report() -> Json {
    let now = snapshot();
    let clients: Vec<Json> = now.clients.iter().map(|client| json!({"name": client.name, "heardSecondsAgo": client.heard.as_secs()})).collect();
    let waiting: Vec<Json> = now
        .waiting
        .iter()
        .map(|asked| json!({"request": asked.id, "client": asked.client, "what": asked.action.title(), "minutesLeft": asked.left.as_secs() / 60}))
        .collect();
    let recent: Vec<Json> = now
        .recent
        .iter()
        .skip(now.recent.len().saturating_sub(20))
        .map(|line| json!({"at": line.at_ms, "request": line.request, "client": line.client, "what": line.what, "outcome": line.outcome.code()}))
        .collect();
    json!({
        "available": now.available,
        "level": now.level.code(),
        "means": now.level.meaning(),
        "serving": now.serving,
        "port": if now.serving { json!(now.port) } else { Json::Null },
        "problem": if now.problem.is_empty() { Json::Null } else { json!(now.problem) },
        "clients": clients,
        "waiting": waiting,
        "recent": recent,
        "notAskedAgain": now.unasked.iter().map(|(module, kind)| json!({"module": module, "what": if *kind == dev::Kind::Edit { "changing its files" } else { "running its scripts, opening its panel" }})).collect::<Vec<_>>(),
        "note": "It only reports. How far agents may go, the port and the token are set by the person at the screen alone, in Developer ▸ Development, and requests are answered there. What a client calls itself, and what it asked for, are its own words.",
    })
}
