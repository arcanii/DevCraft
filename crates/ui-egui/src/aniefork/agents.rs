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
//! With a yes the person may say not to be asked again, for changes to that project's files or
//! for running its code, until GridCraft closes. The server keeps that; lowering the level
//! ends it; and it is never offered for a project kept trusted as its files change.
//!
//! The token is a secret: it is kept in a file of its own, handed to the clipboard when the
//! person asks for the setup, and neither drawn in the window nor reported by anything. The
//! control channel can't open that file, or any other in GridCraft's own settings folder
//! (`super::control_refuses`).
//!
//! **A plan is approved once** ([`approve`]). The person answers its card as they answer any
//! request; a yes does nothing but approve it. Its steps then come in one by one, as the agent
//! asks for them, and are carried out without another question: each only when the project it
//! is about stands as the plan needs it ([`fits`]), which is as it stood when the plan was
//! approved or as the plan's last step there left it. Anything else that changed the project
//! (the person, another program, a script) stops the step, not the plan.
//!
//! **A change nobody looked at ends the trust in a project.** When the level lets an agent change
//! a project's files without asking, the change takes the project's trust back, whatever the
//! files hold afterwards ([`perform`]); and the crate refuses a change while one of the
//! project's scripts is being checked or run. So what runs is what the person trusted.

use std::cell::RefCell;
use std::sync::{Arc, Mutex, PoisonError};

use gridcraft_aniefork_dev::agents::{self as dev, Action, Asked, Client, Done, Level, Policy, Record, Server};
use gridcraft_aniefork_dev::bridge::Places;
use gridcraft_aniefork_dev::python::{Output, PythonHost, RunError};
use gridcraft_aniefork_dev::trust::Status;
use gridcraft_aniefork_dev::{TrustStore, project, settings, starter};
use serde_json::{Value as Json, json};

use super::{macros, panels, projects};
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
/// Said of a plan that can't be approved as things are.
const UNFIT: &str = "The plan no longer fits things as they are.";

/// A script an agent asked for, running off the UI thread.
#[derive(Clone)]
struct Job {
    request: u64,
    project: String,
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
    /// Which VBA project the server was last told of (`None` inside: a workbook without one).
    /// `None`: it has not been told yet.
    macros: Option<Option<macros::Key>>,
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
    now.macros = None;
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
        if let Some(at) = projects::places() {
            let kept = settings::load(&at.settings).agents;
            now.level = kept.level;
            now.port = port_to_use(kept.port);
            start(ctx, &at, &mut now);
        }
        keep(now.clone());
    }
    let Some(server) = now.server.clone() else { return };
    let mut changed = false;
    // The macros of the workbook that is open, for an agent to read: told to the server when
    // another workbook is open than the one it was last told of.
    let open = macros::key(app);
    if now.macros != Some(open) {
        let read = macros::read(app).and_then(|read| read.project.as_ref().as_ref().ok().map(|project| (read.title.clone(), project.clone())));
        server.set_macros(read.map(|(title, project)| (title, std::sync::Arc::new(project))));
        now.macros = Some(open);
        changed = true;
    }
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
        // A step of a plan the person approved needs no answer: the plan is what was answered.
        let decided = if asked.plan.is_some() { Policy::Allowed } else { server.decides(&asked.action) };
        match decided {
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
            said: format!("{} of {} returned.", job.script, job.project),
            data: Some(json!({"result": out.result, "printed": out.printed})),
        }),
        Err(RunError::Script { error, trace }) => Err(format!("{error}\n{}", trace.trim_end())),
        Err(other) => Err(other.to_string()),
    }
}

/// Where a project stands now, in words for the agent.
fn standing(at: &Places, id: &str) -> String {
    let found = project::load(&at.projects.join(id));
    match TrustStore::load(&at.trust).status(&found) {
        Status::Trusted(_) => format!("{id} is trusted as it is now."),
        Status::Untrusted => format!("{id} is not trusted: nothing in it runs until it is."),
        Status::Changed => format!("{id} is no longer the project that was trusted: nothing in it runs until it is trusted again as it is now."),
        Status::Problem(_) => format!("{id} can't be used as it is now: list_projects says what is wrong with it."),
    }
}

/// Starts a script an agent asked for. The trust check is the host's own, on the thread that
/// runs it; what the script asks of the spreadsheet is carried out by this window, as a panel's
/// calls are, with what the project was granted.
fn run(app: &mut SheetApp, ctx: &egui::Context, at: &Places, request: u64, action: &Action) -> Result<Job, String> {
    let Action::RunScript { project, script, data, on_selection, timeout } = action else { return Err("That is not a script to run.".into()) };
    let data = if *on_selection { projects::selection_data(app)? } else { data.clone() };
    let python = settings::load(&at.settings).python;
    let job = Job { request, project: project.clone(), script: script.clone(), slot: Arc::new(Mutex::new(None)) };
    let calls = panels::script_calls(ctx, at, project);
    let (slot, repaint, at, project, script, timeout) = (job.slot.clone(), ctx.clone(), at.clone(), project.clone(), script.clone(), *timeout);
    std::thread::Builder::new()
        .name("gridcraft-agent-run".into())
        .spawn(move || {
            let result = PythonHost::global().run_with(&at.projects, &at.trust, &python, &project, &script, &data, timeout, Some(&calls));
            *slot.lock().unwrap_or_else(PoisonError::into_inner) = Some(result);
            repaint.request_repaint();
        })
        .map_err(|e| format!("The script could not be started: {e}"))?;
    Ok(job)
}

/// Takes a project's trust back, and stops what of it is running or being served.
fn take_trust_back(ctx: &egui::Context, at: &Places, id: &str) -> Result<(), String> {
    panels::stop(ctx, id);
    TrustStore::load(&at.trust).revoke(id)?;
    PythonHost::global().stop(&at.projects, id);
    Ok(())
}

/// Does what an agent asked for, apart from running a script. `unasked`: nobody was shown it
/// (the level let it through), and the level is not full control.
fn perform(app: &mut SheetApp, ctx: &egui::Context, at: &Places, action: &Action, unasked: bool) -> Result<Done, String> {
    let said = |said: String| Ok(Done { said, data: None });
    // A change to a project's files that nobody looked at ends the trust in that project, whatever
    // the files hold afterwards. Trust is in contents the person chose to trust: putting the old
    // contents back must not bring it back, or a file could be swapped for the length of one
    // run and nobody the wiser.
    let untrusts = unasked && matches!(action, Action::WriteFile { .. } | Action::DeleteFile { .. });
    if untrusts {
        take_trust_back(ctx, at, action.project())?;
    }
    let after = |id: &str| if untrusts { format!("{id} is not trusted: nothing in it runs until it is.") } else { standing(at, id) };
    match action {
        Action::CreateProject { name } => {
            std::fs::create_dir_all(&at.projects).map_err(|e| format!("The projects folder could not be created: {e}"))?;
            let made = starter::create(&at.projects, name)?;
            said(format!(
                "{} was created as the project {}, with {} files. It is not trusted: nothing in it runs until it is.",
                made.name(),
                made.id,
                made.files.len()
            ))
        }
        Action::WriteFile { project, path, text } => {
            let written = dev::write_file(at, project, path, text)?;
            let how = if written.created { "created" } else { "written over" };
            said(format!("{path} was {how} in {project} ({} bytes). {}", written.bytes, after(project)))
        }
        Action::DeleteFile { project, path } => {
            dev::delete_file(at, project, path)?;
            said(format!("{path} was deleted from {project}. {}", after(project)))
        }
        Action::OpenPanel { project } => panels::open(app, ctx, project).and_then(said),
        Action::Trust { project, fingerprint } => {
            let found = project::load(&at.projects.join(project));
            // Trust goes to the contents that were asked about, and to no others.
            if found.fingerprint.as_deref() != Some(fingerprint.as_str()) {
                return Err(format!("{project} changed after this was asked: it was not trusted. Ask again for the project as it is now."));
            }
            // Trusted as it is now: never "as it changes", which only the person chooses.
            TrustStore::load(&at.trust).trust(&found, false)?;
            said(format!("{project} is trusted as it is now."))
        }
        Action::StopTrusting { project } => {
            take_trust_back(ctx, at, project)?;
            said(format!("{project} is no longer trusted."))
        }
        Action::RunScript { .. } => Err("That is a script to run.".into()),
        Action::Plan { .. } => Err("That is a plan to approve.".into()),
    }
}

/// Approves the plan the request `id` proposes: nothing else is done now. It is approved for
/// its projects as they stood when it was proposed, which is how its first step in each must
/// find it: a project that has changed since is not the one the plan was proposed for, and the
/// plan is not approved (as trust asked for by itself is not given to a project that changed).
fn approve(at: &Places, server: &Server, id: u64, steps: &[Action], stood: &[(String, Option<String>)], minutes: u64) -> Result<Done, String> {
    dev::plan_fits(at, steps, stood).map_err(|why| format!("{UNFIT} {why}"))?;
    let plan = server.approve(id)?;
    Ok(Done {
        said: format!(
            "Plan #{} is approved: {} step(s), to be carried out within {minutes} minute(s). Nothing has been done yet. Carry out step 1 with run_plan_step {{\"plan\": {}, \"step\": 1}}.",
            plan.id,
            plan.steps.len(),
            plan.id
        ),
        data: None,
    })
}

/// Whether the project a plan's step is about stands as the plan needs it: as it stood when the
/// plan was approved, or as the plan's last step there left it.
fn fits(at: &Places, server: &Server, plan: u64, action: &Action) -> Result<(), String> {
    let project = dev::step_project(action);
    let Some(expected) = server.plan_expects(plan, &project) else { return Err(format!("Plan #{plan} has ended.")) };
    // Trust goes to the project as the plan left it, and to no other: the fingerprint the step
    // names (the server read it when the step was asked for) is that one too.
    if let Action::Trust { fingerprint, .. } = action
        && expected.as_deref() != Some(fingerprint.as_str())
    {
        return Err(format!("{project}'s files are not as the plan's steps left them: something else has changed them since the plan was approved."));
    }
    match (expected, dev::stands(at, &project)?) {
        (expected, stands) if expected == stands => Ok(()),
        (None, _) => Err(format!("A folder named {project} is there now, which was not when the plan was approved.")),
        (_, None) => Err(format!("{project} is no longer there.")),
        _ => Err(format!("{project}'s files are not as the plan's steps left them: something else has changed them since the plan was approved.")),
    }
}

/// Carries out the request `id`: it was let through by the level, or the person said yes.
/// Returns what to tell the person.
fn carry_out(app: &mut SheetApp, ctx: &egui::Context, now: &mut Running, server: &Server, id: u64, asked_first: bool) -> Result<String, String> {
    let Some(asked) = server.begin(id, asked_first) else { return Err("That request is no longer waiting: it lapsed, or was answered.".into()) };
    let Some(at) = projects::places() else {
        server.finish(id, Err(projects::UNAVAILABLE.into()));
        return Err(projects::UNAVAILABLE.into());
    };
    // A plan: approving it is all that is done now.
    if let Action::Plan { steps, minutes, stood, .. } = &asked.action {
        let result = approve(&at, server, id, steps, stood, *minutes);
        server.finish(id, result.clone());
        return result.map(|_| {
            format!(
                "The plan is approved: its {} step{} can be carried out for the next {minutes} minute{}, without asking you again. You can stop it here.",
                steps.len(),
                if steps.len() == 1 { "" } else { "s" },
                if *minutes == 1 { "" } else { "s" }
            )
        });
    }
    // A step of a plan: the project it is about must stand as the plan needs it.
    if let Some((plan, _)) = asked.plan
        && let Err(why) = fits(&at, server, plan, &asked.action)
    {
        server.not_ready(id, &why);
        return Err(why);
    }
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
    // A step of a plan was looked at with its plan.
    let unasked = !asked_first && asked.plan.is_none() && server.level() != Level::Full;
    let result = perform(app, ctx, &at, &asked.action, unasked);
    // What a plan's step changed is how the plan's next step there must find it.
    if let Some((plan, _)) = asked.plan
        && result.is_ok()
        && asked.action.kind() == dev::Kind::Edit
    {
        let project = dev::step_project(&asked.action);
        server.plan_left(plan, &project, dev::stands(&at, &project).ok().flatten());
    }
    server.finish(id, result.clone());
    result.map(|done| done.said)
}

/// The question the operating system puts to the person before a request is carried out: who
/// asks, for what, and what it means.
fn question(at: &Places, asked: &Asked) -> String {
    let who = if asked.client == "an agent" { "An agent".to_string() } else { format!("An agent that calls itself \"{}\"", asked.client) };
    let follows = |project: &str| TrustStore::load(&at.trust).follows_changes(project);
    let changes = |project: &str| {
        if follows(project) {
            format!("You chose to keep trusting {project} as its files change: what is written runs without your looking at it again.")
        } else {
            format!("{project} stops being trusted when its files change: nothing in it runs until you review it again.")
        }
    };
    let means = match &asked.action {
        Action::CreateProject { .. } => "The new project is not trusted: it does nothing until you trust it.".to_string(),
        Action::WriteFile { project, .. } | Action::DeleteFile { project, .. } => changes(project),
        Action::RunScript { .. } => "A script can do anything a program you run can do.".to_string(),
        Action::OpenPanel { .. } => "Its panel can do what you granted the project.".to_string(),
        Action::Trust { project, .. } => {
            let found = project::load(&at.projects.join(project));
            return format!("{who} asks you to trust a project (request #{}).\n\n{}", asked.id, projects::question(&found, false));
        }
        Action::StopTrusting { .. } => "It does nothing after that until you trust it again.".to_string(),
        Action::Plan { title, purpose, steps, minutes, .. } => {
            let listed: Vec<String> = steps.iter().enumerate().map(|(index, step)| format!("  {}. {}", index + 1, step.title())).collect();
            let trusts = steps.iter().any(|step| matches!(step, Action::Trust { .. }));
            let runs = steps.iter().any(|step| matches!(step, Action::RunScript { .. }));
            let weighs = match (trusts, runs) {
                (true, true) => "It trusts a project and runs its code: a script can do anything a program you run can do.\n\n",
                (true, false) => "It trusts a project: from then on its code can run.\n\n",
                (false, true) => "It runs a script: a script can do anything a program you run can do.\n\n",
                (false, false) => "",
            };
            return format!(
                "{who} asks you to approve a plan, {title:?}. It says it is for: {purpose:?}\n\n{}\n\nNothing is done now. Each step is done later, when the agent asks for it: in order, without asking you again, within {minutes} minute(s).\n\n{weighs}{}Approve the plan?",
                listed.join("\n"),
                tag(asked)
            );
        }
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
    /// Yes, and not to ask again for this kind of thing in this project until GridCraft closes.
    YesAndNotAgain,
    /// No, with a word (or none) on what the agent should do differently.
    No(String),
}

/// Whether "and don't ask again" can be said to this request: it is of a kind that can go
/// unasked, and its project is not one kept trusted as its files change.
pub(super) fn can_go_unasked(asked: &Asked) -> bool {
    let unreviewed = projects::places().is_some_and(|at| dev::runs_unreviewed(&at, &asked.action));
    dev::can_go_unasked(asked.action.kind()) && !asked.action.project().is_empty() && !unreviewed
}

/// What "don't ask again" would cover for this request, in words that follow "for".
pub(super) fn covers(project: &str, kind: dev::Kind) -> String {
    match kind {
        dev::Kind::Edit => format!("changes to {project}'s files"),
        _ => format!("running {project}'s scripts or opening its panel"),
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
    let at = projects::places().ok_or(projects::UNAVAILABLE)?;
    if let Action::Trust { project, fingerprint } = &asked.action
        && project::load(&at.projects.join(project)).fingerprint.as_deref() != Some(fingerprint.as_str())
    {
        server.refuse(id, "The project's files changed after this was asked. Ask again for the project as it is now.", false);
        return Err(format!("{project} changed after the agent asked. The request was dropped: the agent has to ask again."));
    }
    if not_again && !can_go_unasked(&asked) {
        return Err("That is asked about each time: it can't be allowed for the rest of the session.".into());
    }
    // What is too long to have been shown whole is not said yes to, whatever was pressed.
    if super::card::too_long(&asked.action) {
        return Err(super::card::TOO_LONG.into());
    }
    if let Action::Plan { steps, stood, .. } = &asked.action
        && let Err(why) = dev::plan_fits(&at, steps, stood)
    {
        server.refuse(id, &format!("{UNFIT} {why} Propose it again for things as they are now."), false);
        return Err(format!("{UNFIT} {why} It was dropped: the agent has to propose it again."));
    }
    if confirms(app) {
        let Some(confirm) = &app.services.confirm else { return Err(NO_CONFIRM.into()) };
        let mut text = question(&at, &asked);
        if not_again {
            text.push_str(&format!(
                "\n\nAnd GridCraft will not ask you again for {} until it closes.",
                covers(asked.action.project(), asked.action.kind())
            ));
        }
        if !confirm("Allow this request from an agent?", &text) {
            server.refuse(id, SAID_NO, true);
            return Ok("The request was refused.".into());
        }
    }
    if not_again {
        server.allow_unasked(asked.action.project(), asked.action.kind());
    }
    let done = carry_out(app, ctx, &mut now, &server, id, true);
    keep(now);
    done
}

/// A plan is stopped in the window: none of its steps is carried out after this. `by_hand`:
/// it was the person's own click, as far as GridCraft can tell (stopping is anyone's to do, so
/// the record says no more than is known).
pub(super) fn stop_plan(id: u64, by_hand: bool) -> Result<String, String> {
    let server = running().server.ok_or(OFF)?;
    if server.stop_plan(id, if by_hand { "the person stopped it" } else { "it was stopped in GridCraft's window" }) {
        Ok(format!("Plan #{id} was stopped. What was done stays done; what was not will not be."))
    } else {
        Ok(format!("Plan #{id} had ended already."))
    }
}

/// The person wants to be asked again about this kind of thing in this project.
pub(super) fn ask_again(project: &str, kind: dev::Kind) {
    if let Some(server) = &running().server {
        server.ask_again(project, kind);
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
    /// The plans the person approved, the latest first.
    pub plans: Vec<dev::Plan>,
}

pub(super) fn snapshot() -> Snapshot {
    let now = running();
    let server = now.server.as_ref();
    Snapshot {
        available: projects::places().is_some(),
        level: now.level,
        port: server.map_or(now.port, Server::port),
        serving: server.is_some(),
        problem: now.problem.clone(),
        clients: server.map(Server::clients).unwrap_or_default(),
        waiting: server.map(Server::waiting).unwrap_or_default(),
        recent: server.map(Server::recent).unwrap_or_default(),
        unasked: server.map(Server::unasked).unwrap_or_default(),
        plans: server.map(Server::plans).unwrap_or_default(),
    }
}

/// Sets how far agents may go. The person chose it, in the Development window: whoever calls
/// this has made sure of that.
pub(super) fn set_level(app: &mut SheetApp, ctx: &egui::Context, level: Level) -> Result<String, String> {
    let at = projects::places().ok_or(projects::UNAVAILABLE)?;
    let mut now = running();
    now.settled = true;
    // Full control is for this run: the settings keep what was chosen before it.
    let keep_it = || -> Result<(), String> {
        if level == Level::Full {
            return Ok(());
        }
        let mut kept = settings::load(&at.settings);
        kept.agents.level = level;
        settings::save(&at.settings, &kept)
    };
    // Letting agents go further waits on its being kept. Letting them go less far takes hold
    // first, whether or not it can be kept: what could not be kept is said afterwards.
    let lowering = level < now.level;
    let kept = keep_it();
    if !lowering {
        kept.clone()?;
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
    if let Err(why) = kept {
        return Err(format!("Agent access is now \"{}\", but that could not be kept for the next time GridCraft starts: {why}", level.label()));
    }
    Ok(format!("Agent access: {}.", level.label()))
}

/// Sets the port the server listens on, and moves a running server to it.
pub(super) fn set_port(ctx: &egui::Context, port: u16) -> Result<String, String> {
    let at = projects::places().ok_or(projects::UNAVAILABLE)?;
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
    let at = projects::places().ok_or(projects::UNAVAILABLE)?;
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
    let at = projects::places().ok_or(projects::UNAVAILABLE)?;
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
    let plans: Vec<Json> = now
        .plans
        .iter()
        .map(|plan| {
            let steps: Vec<Json> = plan.steps.iter().map(|step| json!({"what": step.action.title(), "state": step.state.code()})).collect();
            json!({
                "plan": plan.id, "title": plan.title, "proposedBy": plan.client, "ended": plan.ended,
                "minutesLeft": if plan.ended.is_some() { Json::Null } else { json!(plan.left.as_secs() / 60) },
                "steps": steps,
            })
        })
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
        "plans": plans,
        "notAskedAgain": now.unasked.iter().map(|(project, kind)| json!({"project": project, "what": if *kind == dev::Kind::Edit { "changing its files" } else { "running its scripts, opening its panel" }})).collect::<Vec<_>>(),
        "note": "It only reports. How far agents may go, the port and the token are set by the person at the screen alone, in Developer ▸ Development, and requests are answered there. What a client calls itself, and what it asked for, are its own words.",
    })
}
