//! What the Development window says, apart from how it is drawn.
//!
//! Two windows show the same things: GridCraft's own (`development.rs`, drawn with the app's
//! toolkit) and, where the shell can show one, the Development window as a page
//! (`devpage.rs`). What a request's card says is security: the person decides on it. So
//! neither window words it for itself. Both are given it from here, as [`Block`]s, and only
//! draw what they are given.

use std::borrow::Cow;

use gridcraft_aniefork_dev::agents::{self as dev, Action, Asked, Before, Sight, StepState};
use gridcraft_aniefork_dev::bridge::Places;
use gridcraft_aniefork_dev::trust::TrustStore;
use gridcraft_aniefork_dev::{project, starter};
use serde_json::{Value as Json, json};

use super::agents;
use super::diff::{self, Line, Row};

/// The most of a file or of a script's data shown in a request (characters). What is cut says so.
pub(super) const SHOWN: usize = 200_000;
/// The most lines of a comparison shown. What is cut says so.
pub(super) const ROWS: usize = 4_000;
/// How many unchanged lines are shown before and after each change to a file.
pub(super) const AROUND: usize = 3;

pub(super) const YES: &str = "1. Yes, allow it once";
/// Yes, to a plan.
pub(super) const YES_PLAN: &str = "1. Yes, approve the plan";
/// No, when the card has two choices, and when it has the third between them.
pub(super) const NO: &str = "2. No";
pub(super) const NO_OF_THREE: &str = "3. No";
pub(super) const REASON: &str = "…and tell the agent what to do differently";
pub(super) const NOT_NOW: &str = "Not now";
/// Said on a card that holds more than it can show whole.
pub(super) const TOO_LONG: &str = "This is too long to show whole, so it can't be allowed here. Say no, and have the agent do it in smaller pieces.";
pub(super) const INTRO: &str = "Your development kits and their projects, and how far an agent may help you with them.";
pub(super) const AGENTS: &str =
    "An agent (Claude Code, for one) can connect to GridCraft's Development server, which speaks MCP, to help you build. You set how far it may go.";
pub(super) const NO_AGENTS: &str = "Agents can't connect in this run, because it keeps no settings.";
pub(super) const NO_PROJECTS: &str = "Projects are not available in this run, because it keeps no settings.";
pub(super) const RECORD: &str = "The whole record is in the Developer console, and in agent-log.jsonl beside your settings.";

/// One thing a card says or shows.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Block<'a> {
    /// A line of plain words.
    Note(String),
    /// Words that weigh: drawn as a warning.
    Warn(String),
    /// A line in small type.
    Small(String),
    Strong(String),
    /// One of a list. `warn`: it weighs.
    Bullet {
        text: String,
        warn: bool,
    },
    /// A small line about a file, and the way to see the file whole (`true`) or as what
    /// changes in it (`false`).
    Toggle {
        text: String,
        button: Option<(&'static str, bool)>,
    },
    /// Text in a block of its own: a file's lines, numbered, or a script's data.
    Text {
        text: Cow<'a, str>,
        numbered: bool,
    },
    /// What a change to a file takes out and puts in.
    Diff(Vec<Line<'a>>),
    /// One step of a plan: its heading, what it is about, and what it holds.
    Step {
        heading: String,
        target: String,
        blocks: Vec<Block<'a>>,
    },
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

/// Whether a character of a file or of data is drawn as itself. A tab is; what a text toolkit
/// draws as nothing is not.
fn drawn(c: char) -> bool {
    c == '\t' || dev::shows(c)
}

/// A line as it is drawn, with every character that would not show by itself written out as
/// its number. What runs is what is read: a carriage return in the middle of a line is a new
/// line to Python and nothing to the eye, and a character that turns the order of text can
/// make code read as a comment.
pub(super) fn visible(line: &str) -> Cow<'_, str> {
    if line.chars().all(drawn) {
        return Cow::Borrowed(line);
    }
    let mut out = String::with_capacity(line.len() + 16);
    for c in line.chars() {
        if drawn(c) {
            out.push(c);
        } else {
            out.push_str(&format!("⟨U+{:04X}⟩", c as u32));
        }
    }
    Cow::Owned(out)
}

/// How many characters of `text` would not show by themselves: line breaks apart (a carriage
/// return before a line feed is one).
fn hidden(text: &str) -> usize {
    let mut chars = text.chars().peekable();
    let mut count = 0;
    while let Some(c) = chars.next() {
        let line_break = c == '\n' || (c == '\r' && chars.peek() == Some(&'\n'));
        if !line_break && !drawn(c) {
            count += 1;
        }
    }
    count
}

/// Said of text that holds characters which don't show.
fn hidden_note(count: usize) -> Block<'static> {
    Block::Warn(format!(
        "It holds {count} character{} that would not show by {}: they are written out as ⟨U+…⟩. Text like that can read differently from how it runs.",
        plural(count),
        if count == 1 { "itself" } else { "themselves" }
    ))
}

/// What the agent's client calls itself, for a sentence: its own word, or "the agent".
pub(super) fn agent(asked: &Asked) -> String {
    if asked.client == "an agent" { "the agent".into() } else { asked.client.clone() }
}

/// Whether a request writes a file that is not there now. (The list of requests asks only
/// this of the disk; a card reads the file.)
pub(super) fn writes_new(at: &Places, action: &Action) -> bool {
    match action {
        Action::WriteFile { project, path, .. } => !at.projects.join(project).join(path).is_file(),
        _ => false,
    }
}

/// How a request finds what it is about, as things are now. A plan's steps are followed
/// through instead ([`dev::sights`]).
pub(super) fn sight_now(at: &Places, action: &Action) -> Sight {
    match action {
        Action::WriteFile { project, path, .. } | Action::DeleteFile { project, path } => {
            Sight { before: Some(dev::file_now(at, project, path)), ..Sight::default() }
        }
        _ => Sight::default(),
    }
}

/// Whether the file a request writes is not there when it is written.
pub(super) fn is_new(sight: &Sight) -> bool {
    sight.before == Some(Before::Nothing)
}

/// What kind of thing a request is: the card's heading. `new`: the file it writes is not there.
pub(super) fn kind(action: &Action, new: bool) -> &'static str {
    match action {
        Action::CreateProject { .. } => "Create project",
        Action::WriteFile { .. } if new => "Create file",
        Action::WriteFile { .. } => "Edit file",
        Action::DeleteFile { .. } => "Delete file",
        Action::RunScript { .. } => "Run script",
        Action::OpenPanel { .. } => "Open panel",
        Action::Trust { .. } => "Trust project",
        Action::StopTrusting { .. } => "Stop trusting project",
        Action::Plan { .. } => "Plan",
    }
}

/// What a request is about: the first line of the card's block.
pub(super) fn target(action: &Action) -> String {
    match action {
        Action::CreateProject { name } => format!("{}/   ({name:?})", starter::id_for(name)),
        Action::WriteFile { project, path, .. } | Action::DeleteFile { project, path } => format!("{project}/{path}"),
        Action::RunScript { project, script, .. } => format!("{project}: {script}"),
        Action::OpenPanel { project } | Action::Trust { project, .. } | Action::StopTrusting { project } => project.clone(),
        Action::Plan { title, steps, .. } => format!("{title:?}  ·  {} step{}", steps.len(), plural(steps.len())),
    }
}

/// The question the card puts. What the agent named freely is inside quotes. `new`: the file
/// it writes is not there.
pub(super) fn question(asked: &Asked, new: bool) -> String {
    let agent = agent(asked);
    match &asked.action {
        Action::CreateProject { .. } => format!("Do you want to let {agent} create this project?"),
        Action::WriteFile { path, .. } if new => format!("Do you want to let {agent} create {path:?}?"),
        Action::WriteFile { path, .. } => format!("Do you want to let {agent} make this edit to {path:?}?"),
        Action::DeleteFile { path, .. } => format!("Do you want to let {agent} delete {path:?}?"),
        Action::RunScript { script, .. } => format!("Do you want to let {agent} run {script:?}?"),
        Action::OpenPanel { project } => format!("Do you want to let {agent} open the panel of {project} in your browser?"),
        Action::Trust { project, .. } => format!("Do you want to trust {project}, as {agent} asks?"),
        Action::StopTrusting { project } => format!("Do you want to stop trusting {project}, as {agent} asks?"),
        Action::Plan { .. } => format!("Do you want to approve this plan, as {agent} proposes it?"),
    }
}

/// What names a request, on its card and in the list: its number, the mark of what it holds
/// (the same mark the operating system's question gives), and how long it has.
pub(super) fn named(asked: &Asked) -> String {
    let mark = match mark(asked).as_str() {
        "" => String::new(),
        mark => format!("  ·  marked {mark}"),
    };
    format!("request #{}{mark}  ·  lapses in {} min", asked.id, asked.left.as_secs() / 60 + 1)
}

/// The mark of what a request holds, worked out once: a plan's is of all it holds, and a card
/// is drawn many times a second.
fn mark(asked: &Asked) -> String {
    thread_local! {
        static MARKS: std::cell::RefCell<Vec<((u64, Option<u64>), String)>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    // A request's number begins again with each server: when it was asked tells them apart.
    let key = (asked.id, asked.at_ms);
    MARKS.with(|marks| {
        let mut marks = marks.borrow_mut();
        if let Some((_, mark)) = marks.iter().find(|(known, _)| *known == key) {
            return mark.clone();
        }
        let mark = dev::mark(&asked.action);
        if marks.len() >= 16 {
            marks.remove(0);
        }
        marks.push((key, mark.clone()));
        mark
    })
}

/// Whether a request holds more than a card can show whole. Such a request can't be said yes
/// to: nobody has read the whole of it.
pub(super) fn too_long(action: &Action) -> bool {
    match action {
        Action::WriteFile { text, .. } => text.chars().count() > SHOWN,
        Action::RunScript { data, on_selection: false, .. } => serde_json::to_string_pretty(data).unwrap_or_default().chars().count() > SHOWN,
        Action::Plan { steps, .. } => steps.iter().any(too_long),
        _ => false,
    }
}

/// The card's second choice, when it has three.
pub(super) fn yes_and(action: &Action) -> String {
    format!("2. Yes, and don't ask again for {} until GridCraft closes", agents::covers(action.project(), action.kind()))
}

/// The card's last line: who asks, and whether a yes is confirmed elsewhere.
pub(super) fn asker(asked: &Asked, confirms: bool) -> String {
    let named = if asked.client == "an agent" {
        "The agent gave no name.".to_string()
    } else {
        format!("Asked by an agent that calls itself \"{}\": the name is its own word.", asked.client)
    };
    format!("{named}{}", if confirms { " A yes is confirmed by your operating system." } else { "" })
}

/// What a change to a project's files means for its trust.
fn change_means(at: &Places, id: &str) -> Block<'static> {
    if TrustStore::load(&at.trust).follows_changes(id) {
        Block::Warn(format!("You chose to keep trusting {id} as its files change: what is written runs without your looking at it again."))
    } else {
        Block::Note(format!("{id} stops being trusted when its files change: nothing in it runs until you review it again."))
    }
}

/// What a project would be allowed, one line each. `None` asked: it asks for nothing.
fn allowed(asks: &[gridcraft_aniefork_dev::manifest::Permission]) -> Vec<Block<'static>> {
    if asks.is_empty() {
        return vec![Block::Bullet { text: "Show its panel, and nothing more".into(), warn: false }];
    }
    asks.iter().map(|permission| Block::Bullet { text: permission.meaning().to_string(), warn: permission.is_unlimited() }).collect()
}

/// The files a project holds, the first few by name.
fn listed(files: &[String]) -> String {
    let shown: Vec<&str> = files.iter().map(String::as_str).take(8).collect();
    let more = files.len().saturating_sub(shown.len());
    format!("{} file{}: {}{}", files.len(), plural(files.len()), shown.join(", "), if more > 0 { format!(" and {more} more") } else { String::new() })
}

/// The thing itself, in the card's block: everything a request, or one step of a plan, would
/// do. `sight`: how it finds what it is about. `of_plan`: it is a step of a plan, which finds
/// things as the steps before it leave them. `whole`: a file is shown whole, not as what
/// changes in it.
pub(super) fn content<'a>(at: &Places, step: &'a Action, sight: &'a Sight, of_plan: bool, whole: bool) -> Vec<Block<'a>> {
    match step {
        Action::CreateProject { .. } => vec![Block::Note("A starter project: a project.toml, a panel and a Python script.".into())],
        Action::WriteFile { text, .. } => {
            let mut blocks = Vec::new();
            // What would not show is said first, and written out where it stands.
            let unseen = hidden(text);
            if unseen > 0 {
                blocks.push(hidden_note(unseen));
            }
            // The file that is there, when it can be read as text and compared.
            let old = match &sight.before {
                Some(Before::Text(old)) => Some(old.as_str()),
                Some(Before::Unreadable) => None,
                Some(Before::Nothing) | None => {
                    blocks.extend([Block::Small("The file is new:".into()), Block::Text { text: Cow::Borrowed(text), numbered: true }]);
                    return blocks;
                }
            };
            // Its changes are shown when all of them can be: a card is never said yes to on
            // part of what it would do.
            let compared = old.and_then(|old| diff::lines(old, text)).filter(|compared| diff::shown(compared, AROUND).len() <= ROWS);
            match (compared, whole) {
                (Some(compared), false) => {
                    let (gone, new) = diff::counts(&compared);
                    let said = if gone == 0 && new == 0 {
                        "It changes the file that is there: no line differs but for line breaks or what ends it.".to_string()
                    } else {
                        format!("It changes the file that is there: {new} line{} in, {gone} out.", plural(new))
                    };
                    blocks.extend([Block::Toggle { text: said, button: Some(("Show whole file", true)) }, Block::Diff(compared)]);
                }
                (compared, _) => {
                    let said = if compared.is_some() {
                        "It replaces the file that is there with this:"
                    } else {
                        "It replaces the file that is there, which can't be shown as what changes line by line, with this:"
                    };
                    let button = compared.is_some().then_some(("Show changes", false));
                    blocks.extend([Block::Toggle { text: said.into(), button }, Block::Text { text: Cow::Borrowed(text), numbered: true }]);
                }
            }
            blocks
        }
        Action::DeleteFile { .. } => vec![Block::Note("The file is deleted.".into())],
        Action::RunScript { data, on_selection, timeout, .. } => {
            let mut blocks = if *on_selection {
                vec![Block::Note("main(data) is handed the current selection.".into())]
            } else {
                let data = serde_json::to_string_pretty(data).unwrap_or_default();
                vec![Block::Small("main(data) is handed this:".into()), Block::Text { text: Cow::Owned(data), numbered: false }]
            };
            blocks.push(Block::Small(format!("It may run for up to {} seconds.", timeout.as_secs().max(1))));
            blocks
        }
        Action::OpenPanel { .. } => vec![Block::Note("Its panel opens in your browser.".into())],
        // In a plan: the project as the plan's steps leave it.
        Action::Trust { .. } if of_plan => {
            let mut blocks = vec![Block::Strong("It would be allowed to:".into())];
            match &sight.asks {
                Some(Ok(asks)) => blocks.extend(allowed(asks)),
                Some(Err(why)) => blocks.push(Block::Warn(format!("Its project.toml will not do ({why}): this step would fail."))),
                None => {}
            }
            blocks.push(Block::Small(format!("As the steps before this one leave it, it will contain {}.", listed(&sight.files))));
            // Trust is in all of the project, not only in what the plan writes.
            blocks.push(Block::Small(
                "Its files that this plan doesn't write are trusted as they are now. To read them first, say no, and review the project in Projects."
                    .into(),
            ));
            blocks
        }
        Action::Trust { project, fingerprint } => {
            let found = project::load(&at.projects.join(project));
            if found.fingerprint.as_deref() != Some(fingerprint.as_str()) {
                return vec![Block::Warn("The project changed after the agent asked. It has to ask again: this request can't be allowed.".into())];
            }
            let asks = found.manifest.as_ref().map(|m| m.permissions.clone()).unwrap_or_default();
            let mut blocks = vec![Block::Strong("It would be allowed to:".into())];
            blocks.extend(allowed(&asks));
            blocks.push(Block::Small(format!(
                "It contains {}. Fingerprint {}…",
                listed(&found.files),
                fingerprint.chars().take(16).collect::<String>()
            )));
            blocks
        }
        Action::StopTrusting { .. } => vec![Block::Note("Its trust is taken back.".into())],
        // A plan's card lists its steps one by one: `plan`.
        Action::Plan { .. } => vec![],
    }
}

/// A plan, in the card's block: what it is for, then every step whole, each as it will find
/// things once the steps before it are done.
pub(super) fn plan<'a>(
    at: &Places,
    purpose: &str,
    steps: &'a [Action],
    minutes: u64,
    sights: &'a Result<Vec<Sight>, String>,
    whole: bool,
) -> Vec<Block<'a>> {
    let mut blocks = vec![
        // The agent's own words, shown as its words.
        Block::Note(format!("What the agent says it is for: {purpose:?}")),
        Block::Small(format!(
            "Nothing is done now. Each step is done later, when the agent asks for it: in order, without asking you again, within {minutes} minute{} of your yes. You can stop the plan in this window.",
            if minutes == 1 { "" } else { "s" }
        )),
    ];
    match sights {
        Err(why) => blocks.push(Block::Warn(format!("The plan doesn't fit things as they are now, so it can't be approved. {why}"))),
        Ok(sights) => {
            for (index, (step, sight)) in steps.iter().zip(sights).enumerate() {
                blocks.push(Block::Step {
                    heading: format!("Step {} of {}  ·  {}", index + 1, steps.len(), kind(step, is_new(sight))),
                    target: target(step),
                    blocks: content(at, step, sight, true, whole),
                });
            }
        }
    }
    blocks
}

/// What a yes would mean, under the card's block.
pub(super) fn means(at: &Places, action: &Action) -> Vec<Block<'static>> {
    match action {
        Action::CreateProject { .. } => vec![Block::Note("It is created untrusted: it does nothing until you trust it.".into())],
        Action::WriteFile { project, .. } | Action::DeleteFile { project, .. } => vec![change_means(at, project)],
        Action::RunScript { .. } => vec![
            Block::Note("It may change the workbook, with what its project was granted.".into()),
            Block::Warn("A script can do anything a program you run can do.".into()),
        ],
        Action::OpenPanel { .. } => vec![Block::Note("Its panel can do what you granted the project.".into())],
        Action::Trust { .. } => {
            vec![Block::Note("Trust is given to exactly these files. To read them first, say no, and review the project in Projects.".into())]
        }
        Action::StopTrusting { .. } => vec![Block::Note("It does nothing after that until you trust it again.".into())],
        Action::Plan { steps, .. } => {
            let mut blocks = Vec::new();
            // What in it trusts a project or runs code is said by itself.
            for (index, step) in steps.iter().enumerate() {
                match step {
                    Action::Trust { project, .. } => blocks.push(Block::Warn(format!(
                        "Step {} trusts {project}: from then on its code can run, with what it asks to be allowed.",
                        index + 1
                    ))),
                    Action::RunScript { project, script, .. } => blocks.push(Block::Warn(format!(
                        "Step {} runs {script:?} of {project}: a script can do anything a program you run can do.",
                        index + 1
                    ))),
                    _ => {}
                }
            }
            let store = TrustStore::load(&at.trust);
            let mut told: Vec<&str> = Vec::new();
            for step in steps.iter().filter(|step| matches!(step, Action::WriteFile { .. } | Action::DeleteFile { .. })) {
                let id = step.project();
                if store.follows_changes(id) && !told.contains(&id) {
                    told.push(id);
                    blocks.push(Block::Warn(format!(
                        "You chose to keep trusting {id} as its files change: what this plan writes there runs without your looking at it again."
                    )));
                }
            }
            blocks.push(Block::Note(
                "The agent may stop after any step, and a step that fails ends the plan: it should be safe to leave at each one.".into(),
            ));
            blocks
        }
    }
}

/// The text a block shows of `text`, and what to say of what is left out.
pub(super) fn shown_text(text: &str) -> (String, Option<String>) {
    let count = text.chars().count();
    if count <= SHOWN {
        return (text.to_string(), None);
    }
    (text.chars().take(SHOWN).collect(), Some(format!("Only the first {SHOWN} characters are shown: {} more are not.", count - SHOWN)))
}

/// The rows a comparison shows, and what to say of what is left out.
pub(super) fn shown_rows<'a>(compared: &[Line<'a>]) -> (Vec<Row<'a>>, Option<String>) {
    let mut rows = diff::shown(compared, AROUND);
    if rows.len() <= ROWS {
        return (rows, None);
    }
    rows.truncate(ROWS);
    (rows, Some(format!("Only the first {ROWS} lines of the changes are shown. Show the whole file to read the rest.")))
}

/// The blocks as the page is given them. Text only: the page draws every string as text.
pub(super) fn json(blocks: &[Block<'_>]) -> Json {
    let one = |block: &Block<'_>| match block {
        Block::Note(text) => json!({"t": "note", "text": text}),
        Block::Warn(text) => json!({"t": "warn", "text": text}),
        Block::Small(text) => json!({"t": "small", "text": text}),
        Block::Strong(text) => json!({"t": "strong", "text": text}),
        Block::Bullet { text, warn } => json!({"t": "bullet", "text": text, "warn": warn}),
        Block::Toggle { text, button } => {
            json!({"t": "toggle", "text": text, "button": button.map(|(label, _)| label), "whole": button.is_some_and(|(_, whole)| whole)})
        }
        Block::Text { text, numbered } => {
            let (shown, cut) = shown_text(text);
            json!({"t": "text", "lines": shown.lines().map(visible).collect::<Vec<_>>(), "numbered": numbered, "cut": cut})
        }
        Block::Diff(compared) => {
            let (rows, cut) = shown_rows(compared);
            let rows: Vec<Json> = rows
                .iter()
                .map(|row| match row {
                    Row::Skipped(count) => json!({"k": "skip", "text": format!("{count} line{} the same", plural(*count))}),
                    Row::Line(Line::Same(_, number, text)) => json!({"k": "same", "n": number, "text": visible(text)}),
                    Row::Line(Line::Gone(number, text)) => json!({"k": "gone", "n": number, "text": visible(text)}),
                    Row::Line(Line::New(number, text)) => json!({"k": "new", "n": number, "text": visible(text)}),
                })
                .collect();
            json!({"t": "diff", "rows": rows, "cut": cut})
        }
        Block::Step { heading, target, blocks } => json!({"t": "step", "heading": heading, "target": target, "blocks": json(blocks)}),
    };
    Json::Array(blocks.iter().map(one).collect())
}

// --- the rest of the window ------------------------------------------------------------------

/// Whether the server is serving, in a line: how it is to be drawn (`ok`, `error`, `off`), and
/// the words.
pub(super) fn serving(now: &agents::Snapshot) -> (&'static str, String) {
    if now.serving {
        ("ok", format!("Serving on 127.0.0.1, port {}.", now.port))
    } else if !now.problem.is_empty() {
        ("error", format!("Not serving: {}", now.problem))
    } else {
        ("off", "Not serving: agent access is off.".into())
    }
}

/// Who connected, in a line. `None` when nothing is served.
pub(super) fn clients(now: &agents::Snapshot) -> Option<String> {
    if !now.serving {
        return None;
    }
    Some(match now.clients.first() {
        Some(client) => {
            let more = now.clients.len() - 1;
            let others = if more > 0 { format!(", and {more} more") } else { String::new() };
            format!("Last heard from: {} ({} s ago){others}. A client's name is what it chose to say.", client.name, client.heard.as_secs())
        }
        None => "No agent has connected yet.".into(),
    })
}

/// Where a plan stands, in a few words: how much of it is done, and how long it has or why it ended.
pub(super) fn plan_stands(plan: &dev::Plan) -> String {
    let done = format!("{} of {} done", plan.done(), plan.steps.len());
    match &plan.ended {
        None => format!("{done}  ·  {} min left", plan.left.as_secs() / 60 + 1),
        Some(why) => format!("{done}  ·  ended: {why}"),
    }
}

/// Where a step of a plan stands, in a word or two.
pub(super) fn step_stands(state: &StepState) -> &'static str {
    match state {
        StepState::Waiting => "waiting",
        StepState::Running(_) => "being carried out",
        StepState::Passed(_) => "done",
        StepState::Failed(_) => "failed",
    }
}

/// What was asked lately: each request once, as it stands now, the newest first.
pub(super) fn lately(now: &agents::Snapshot) -> Vec<String> {
    let at_ms = crate::now_ms();
    let mut told = Vec::new();
    let mut lines = Vec::new();
    for line in now.recent.iter().rev() {
        if line.request != 0 && told.contains(&line.request) {
            continue;
        }
        told.push(line.request);
        let when = super::console::ago(line.at_ms, at_ms);
        lines.push(format!("{when}  ·  {}: {}  ·  {}", line.client, line.what, line.outcome.code()));
        if lines.len() >= 4 {
            break;
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn what_is_too_long_to_show_says_how_much_is_left_out() {
        assert_eq!(shown_text("short"), ("short".to_string(), None));
        let long = "x".repeat(SHOWN + 7);
        let (shown, cut) = shown_text(&long);
        assert_eq!(shown.chars().count(), SHOWN);
        assert_eq!(cut.as_deref(), Some("Only the first 200000 characters are shown: 7 more are not."));
        let new: String = (0..ROWS + 50).map(|n| format!("new {n}\n")).collect();
        let compared = diff::lines("old\n", &new).unwrap();
        let (rows, cut) = shown_rows(&compared);
        assert_eq!(rows.len(), ROWS);
        assert!(cut.unwrap().starts_with("Only the first 4000 lines of the changes are shown."));
    }

    #[test]
    fn a_change_too_long_to_show_as_its_changes_is_shown_whole() {
        let at = Places { projects: "nowhere".into(), trust: "nowhere/trust.json".into(), settings: "nowhere/developer.json".into() };
        let write = |text: &str| Action::WriteFile { project: "m".into(), path: "a.py".into(), text: text.to_string() };
        let there = Sight { before: Some(Before::Text("old\n".into())), ..Sight::default() };
        // A change that can be shown line by line is, with the way to the whole file.
        let small = write("new\n");
        let blocks = content(&at, &small, &there, false, false);
        assert!(matches!(&blocks[..], [Block::Toggle { button: Some(("Show whole file", true)), .. }, Block::Diff(_)]), "{blocks:?}");
        // One with more changed lines than a card shows is the whole file, with no way to less.
        let long: String = (0..ROWS + 50).map(|n| format!("new {n}\n")).collect();
        let large = write(&long);
        for whole in [false, true] {
            let blocks = content(&at, &large, &there, false, whole);
            let [Block::Toggle { text, button: None }, Block::Text { numbered: true, .. }] = &blocks[..] else { panic!("{blocks:?}") };
            assert!(text.contains("can't be shown as what changes line by line"), "{text}");
        }
    }

    #[test]
    fn what_would_not_show_is_written_out_and_said() {
        assert_eq!(visible("x = 1\t# plain, with a tab"), "x = 1\t# plain, with a tab");
        assert!(matches!(visible("plain"), Cow::Borrowed(_)));
        // A carriage return in a line is a new line to Python, and nothing to the eye.
        assert_eq!(visible("# a comment\rimport os"), "# a comment⟨U+000D⟩import os");
        // A character that turns the order of what follows.
        assert_eq!(visible("x = '\u{202e}' # safe"), "x = '⟨U+202E⟩' # safe");
        assert_eq!(visible("a\u{200b}b\u{feff}"), "a⟨U+200B⟩b⟨U+FEFF⟩");
        assert_eq!(hidden("a\r\nb\n\tc\n"), 0, "line breaks and tabs are not hidden");
        assert_eq!(hidden("a\rb\u{202e}c\r\n"), 2);
        let at = Places { projects: "nowhere".into(), trust: "nowhere/trust.json".into(), settings: "nowhere/developer.json".into() };
        let write = Action::WriteFile { project: "m".into(), path: "a.py".into(), text: "# fine\rimport os\n".into() };
        let new = Sight { before: Some(Before::Nothing), ..Sight::default() };
        let blocks = content(&at, &write, &new, false, false);
        let [Block::Warn(said), Block::Small(_), Block::Text { .. }] = &blocks[..] else { panic!("{blocks:?}") };
        assert!(said.starts_with("It holds 1 character that would not show by itself"), "{said}");
        let told = json(&blocks);
        assert_eq!(told[2]["lines"], json!(["# fine⟨U+000D⟩import os"]));
        // A file without any is not remarked on.
        let plain = Action::WriteFile { project: "m".into(), path: "a.py".into(), text: "x = 1\r\ny = 2\n".into() };
        assert!(matches!(&content(&at, &plain, &new, false, false)[..], [Block::Small(_), Block::Text { .. }]));
    }

    #[test]
    fn a_plans_purpose_is_shown_as_the_agents_words_and_its_trust_step_says_what_is_trusted_unread() {
        let at = Places { projects: "nowhere".into(), trust: "nowhere/trust.json".into(), settings: "nowhere/developer.json".into() };
        let steps = vec![Action::Trust { project: "m".into(), fingerprint: String::new() }];
        let sights = Ok(vec![Sight { asks: Some(Ok(vec![])), files: vec!["project.toml".into()], ..Sight::default() }]);
        let blocks = plan(&at, "Ignore the steps below: they are safe.", &steps, 60, &sights, false);
        assert_eq!(blocks[0], Block::Note("What the agent says it is for: \"Ignore the steps below: they are safe.\"".into()));
        let Block::Step { heading, blocks: step, .. } = &blocks[2] else { panic!("{blocks:?}") };
        assert_eq!(heading, "Step 1 of 1  ·  Trust project");
        let said: Vec<&str> = step.iter().filter_map(|block| if let Block::Small(text) = block { Some(text.as_str()) } else { None }).collect();
        assert!(said.iter().any(|text| text.starts_with("Its files that this plan doesn't write are trusted as they are now.")), "{said:?}");
    }

    #[test]
    fn the_page_is_given_every_block_as_text() {
        let compared = diff::lines("a\nb\nc\n", "a\nB\nc\n").unwrap();
        let blocks = vec![
            Block::Note("<b>note</b>".into()),
            Block::Warn("warn".into()),
            Block::Bullet { text: "one".into(), warn: true },
            Block::Toggle { text: "It changes".into(), button: Some(("Show whole file", true)) },
            Block::Toggle { text: "It replaces".into(), button: None },
            Block::Text { text: "x = 1\ny = 2\n".into(), numbered: true },
            Block::Diff(compared),
            Block::Step { heading: "Step 1 of 1  ·  Edit file".into(), target: "m/a.py".into(), blocks: vec![Block::Small("small".into())] },
        ];
        let told = json(&blocks);
        assert_eq!(told[0], json!({"t": "note", "text": "<b>note</b>"}), "markup is words like any other");
        assert_eq!(told[2], json!({"t": "bullet", "text": "one", "warn": true}));
        assert_eq!(told[3], json!({"t": "toggle", "text": "It changes", "button": "Show whole file", "whole": true}));
        assert_eq!(told[4], json!({"t": "toggle", "text": "It replaces", "button": null, "whole": false}));
        assert_eq!(told[5], json!({"t": "text", "lines": ["x = 1", "y = 2"], "numbered": true, "cut": null}));
        let rows = told[6]["rows"].as_array().unwrap();
        assert_eq!(rows.iter().map(|row| row["k"].as_str().unwrap()).collect::<Vec<_>>(), ["same", "gone", "new", "same"]);
        assert_eq!((&rows[1]["n"], &rows[1]["text"], &rows[2]["text"]), (&json!(2), &json!("b"), &json!("B")));
        assert_eq!(told[7]["blocks"][0], json!({"t": "small", "text": "small"}));
    }
}
