//! What the Development window is made of, and what it says of the kits, their projects and
//! the open workbook's macros.
//!
//! The window has a list down its side and one part shown beside it:
//!
//! ```text
//! Development                    the whole, at a glance
//!   MCP Server                   agents: how far they may go, and what they ask
//!   Development Kits
//!     Anie Development Kit       Python and Svelte
//!       Workbook code            projects that are the code of one workbook
//!       Libraries                projects for any workbook
//!     C Development Kit          C: built as WebAssembly, and run in the project's panel
//!       …                        its projects, by the same two kinds
//! ```
//!
//! A **kit** is a way of building on GridCraft (`gridcraft_aniefork_dev::kit`); a **project**
//! is a folder with a `project.toml`, which says which kit it is of. Each kit has a part of
//! its own, made the same way from what the kit says of itself. The MCP server is not a
//! kit's: it stands beside them, at the top.
//!
//! Like a request's card (`card.rs`), what is said here is said once, as JSON, and drawn by
//! both windows: GridCraft's own (`development.rs`) and the page (`devpage.rs`). The page is
//! handed it as it is, and draws every string as text.
//!
//! Nothing here decides anything. A project's kind, the workbook it names and what it says it
//! rebuilds of that workbook's macros are the project's own words: they are shown as such.

use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::Arc;

use gridcraft_aniefork_dev::bridge::Places;
use gridcraft_aniefork_dev::kit::{self, Kit, ProjectKind};
use gridcraft_aniefork_dev::{Project, TrustStore, c, describe, highlight, manifest, project, rebuilt, settings, showcase, vba};
use serde_json::{Value as Json, json};

use super::{agents, card, macros, projects};
use crate::SheetApp;

/// The window's name for its top, and for the two things under it.
pub(super) const TOP: &str = "Development";
pub(super) const SERVER: &str = "MCP Server";
pub(super) const KITS: &str = "Development Kits";
/// Said under the kits: they are the ones compiled in, and there is no way to another yet.
pub(super) const KITS_NOTE: &str = "These kits are built into GridCraft: there is no way to load another yet.";

/// Said where a kit's projects are listed: what a project of it is, and that it does nothing
/// by being there.
pub(super) fn projects_note(kit: &Kit) -> String {
    format!("{} It does nothing until you trust it.", kit.project_is)
}
pub(super) const NO_WORKBOOK: &str = "No workbook is open.";
/// The most projects of one kind listed down the side. The kit's own part lists them all.
const LISTED: usize = 40;
/// The most of a project's files named.
const NAMED: usize = 12;
/// The most of a project's files offered for reading.
const OFFERED: usize = 240;
/// Said where a project's files are offered for reading.
pub(super) const FILES_NOTE: &str = "Choose a file to read it. It is shown as text: nothing in it is run.";
/// How long what was read from disk is shown before it is read again (milliseconds).
const KEEP_MS: f64 = if cfg!(test) { 0.0 } else { 1000.0 };

/// Which part of the window is shown.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) enum Pane {
    #[default]
    Overview,
    /// The MCP server: agents.
    Server,
    /// A development kit, by its id: one of those that are loaded.
    Kit(&'static str),
    /// A project, by id.
    Project(String),
}

impl Pane {
    /// A kit's part.
    pub(super) fn kit(kit: &Kit) -> Pane {
        Pane::Kit(kit.id)
    }

    /// The part a window falls back to when what it showed is gone: the first kit's.
    pub(super) fn first_kit() -> Pane {
        Pane::kit(&kit::ANIE)
    }

    /// The word a window uses to ask for this part.
    pub(super) fn code(&self) -> String {
        match self {
            Pane::Overview => "overview".into(),
            Pane::Server => "server".into(),
            Pane::Kit(id) => format!("kit:{id}"),
            Pane::Project(id) => format!("project:{id}"),
        }
    }

    pub(super) fn parse(code: &str) -> Option<Pane> {
        match code {
            "overview" => Some(Pane::Overview),
            "server" => Some(Pane::Server),
            _ => match code.strip_prefix("kit:") {
                // Only a kit that is loaded has a part.
                Some(id) => kit::named(id).map(Pane::kit),
                None => code.strip_prefix("project:").filter(|id| manifest::valid_id(id)).map(|id| Pane::Project(id.to_string())),
            },
        }
    }
}

/// The projects on disk, as they were last read.
#[derive(Clone)]
pub(super) struct Found {
    pub on_disk: Arc<Vec<Project>>,
    /// Each of them described, in the order of `projects`.
    pub projects: Arc<Vec<Json>>,
    pub folder: String,
    /// Another number each time they are read again.
    stamp: u64,
}

thread_local! {
    /// What was last read, where, and when: the windows draw many times a second, and reading
    /// the projects reads every file of every one of them.
    static KEPT: RefCell<Option<(f64, PathBuf, Found)>> = const { RefCell::new(None) };
    /// The open workbook's macros with what the projects say of them: for which workbook and
    /// which reading of the projects.
    static COVERED: RefCell<Option<((macros::Key, u64), Arc<Coverage>)>> = const { RefCell::new(None) };
    /// The file that is being read, as it is shown: of which project, as which fingerprint,
    /// and which file. Reading a file into lines is not done again for every frame.
    static VIEWED: RefCell<Option<((String, Option<String>, String), Arc<Json>)>> = const { RefCell::new(None) };
}

/// The projects, read from disk at most once a moment. `None` in a run that keeps no settings.
pub(super) fn found() -> Option<Found> {
    let at = projects::places()?;
    let now = crate::now_ms();
    KEPT.with(|kept| {
        let mut kept = kept.borrow_mut();
        if let Some((read_at, of, found)) = &*kept
            && *of == at.projects
            && (0.0..KEEP_MS).contains(&(now - read_at))
        {
            return Some(found.clone());
        }
        let store = TrustStore::load(&at.trust);
        let all = project::scan(&at.projects);
        let projects: Vec<Json> = all.iter().map(|project| describe(project, &store)).collect();
        let stamp = kept.as_ref().map_or(0, |(_, _, before)| before.stamp).wrapping_add(1);
        let found = Found { on_disk: Arc::new(all), projects: Arc::new(projects), folder: at.projects.display().to_string(), stamp };
        *kept = Some((now, at.projects.clone(), found.clone()));
        Some(found)
    })
}

/// Something was done that may have changed them: they are read again the next time.
pub(super) fn forget() {
    KEPT.with(|kept| {
        if let Some((read_at, _, _)) = &mut *kept.borrow_mut() {
            *read_at = f64::NEG_INFINITY;
        }
    });
}

fn plural(count: usize) -> &'static str {
    if count == 1 { "" } else { "s" }
}

fn text<'a>(of: &'a Json, key: &str) -> &'a str {
    of.get(key).and_then(Json::as_str).unwrap_or("")
}

/// A project's standing in a word or two, as Projects says it.
pub(super) fn status_label(code: &str) -> &'static str {
    match code {
        "trusted" => "Trusted",
        "changed" => "Changed since you trusted it",
        "problem" => "Can't be used",
        _ => "Not trusted",
    }
}

/// What a project's standing means for it.
fn status_says(code: &str) -> &'static str {
    match code {
        "trusted" => "You trust it as it is now: what it holds can run, with what you allowed it.",
        "changed" => "Its files are not the ones you trusted. It does nothing until you look again.",
        "problem" => "It can't be read as a project, so it can't be trusted or run.",
        _ => "It does nothing until you trust it.",
    }
}

fn kind_of(project: &Json) -> ProjectKind {
    if project["kind"] == ProjectKind::WorkbookCode.code() { ProjectKind::WorkbookCode } else { ProjectKind::Library }
}

/// The kit a described project is listed under.
fn kit_of(project: &Json) -> &'static Kit {
    kit::named(text(project, "kit")).unwrap_or(&kit::ANIE)
}

/// The projects of one kit, as they were described.
fn of_kit<'a>(found: &'a Found, kit: &'a Kit) -> impl Iterator<Item = &'a Json> {
    found.projects.iter().filter(move |project| kit_of(project).id == kit.id)
}

/// The name of the workbook that is open.
fn open_workbook(app: &SheetApp) -> Option<String> {
    app.session.active().map(|doc| doc.title.clone())
}

/// What a project says it rebuilds of its workbook's macros, in a line. `None` when it says
/// nothing.
fn rebuilt_says(note: &Json) -> Option<String> {
    if note.is_null() {
        return None;
    }
    if let Some(problem) = note["problem"].as_str() {
        return Some(format!("Its note of what it rebuilds can't be used: {problem}."));
    }
    let count = |key: &str| note[key].as_u64().unwrap_or(0);
    Some(format!("Of the workbook's macros it says: {} rebuilt, {} in part, {} left as they are.", count("rebuilt"), count("partly"), count("left")))
}

/// The open workbook's macros, with what the projects that are its code say became of each
/// procedure.
pub(super) struct Coverage {
    /// The workbook's title.
    pub title: String,
    /// The VBA project as `vba::summary` gives it, each procedure with `rebuilt` beside it.
    /// An error when the project could not be read.
    pub summary: Result<Json, String>,
    /// How many procedures there are, and of how many a project says it is rebuilt, whole or
    /// in part.
    pub procedures: usize,
    pub spoken_for: usize,
}

/// What the open workbook's macros are and what was rebuilt of them. `None` when the workbook
/// that is open carries none.
pub(super) fn coverage(app: &SheetApp) -> Option<Arc<Coverage>> {
    let read = macros::read(app)?;
    let found = found();
    let key = (read.key, found.as_ref().map_or(0, |found| found.stamp));
    COVERED.with(|covered| {
        let mut covered = covered.borrow_mut();
        if let Some((of, kept)) = &*covered
            && *of == key
            && kept.title == read.title
        {
            return Some(kept.clone());
        }
        let fresh = match read.project.as_ref() {
            Err(why) => Coverage { title: read.title.clone(), summary: Err(why.clone()), procedures: 0, spoken_for: 0 },
            Ok(project) => {
                let mut summary = vba::summary(project);
                let claims = found.as_ref().map(|found| rebuilt::claims(&found.on_disk, &read.title)).unwrap_or_default();
                let (procedures, spoken_for) = rebuilt::mark(&mut summary, &claims);
                Coverage { title: read.title.clone(), summary: Ok(summary), procedures, spoken_for }
            }
        };
        let fresh = Arc::new(fresh);
        *covered = Some((key, fresh.clone()));
        Some(fresh)
    })
}

/// How many of a workbook's procedures are rebuilt, in a sentence.
pub(super) fn covered_says(procedures: usize, spoken_for: usize) -> String {
    match (procedures, spoken_for) {
        (0, _) => "It holds no procedures.".into(),
        (_, 0) => "None of them is rebuilt as a project yet.".into(),
        (all, some) if some >= all => "All of them are rebuilt, whole or in part, as your projects say.".into(),
        (all, some) => format!("{some} of the {all} {} rebuilt, whole or in part, as your projects say.", if some == 1 { "is" } else { "are" }),
    }
}

// --- the list down the side ------------------------------------------------------------------

fn item(pane: &Pane, go: Pane, label: &str, hint: String, depth: u8) -> Json {
    json!({"t": "item", "go": go.code(), "label": label, "hint": hint, "depth": depth, "selected": *pane == go, "mark": null, "badge": null})
}

/// The list down the window's side: every part that can be shown, and which one is.
pub(super) fn nav(app: &SheetApp, pane: &Pane, now: &agents::Snapshot) -> Json {
    let mut rows = vec![item(pane, Pane::Overview, TOP, String::new(), 0)];
    // The MCP server stands at the top, beside the kits: it is not a kit's.
    let mut server = item(pane, Pane::Server, SERVER, if now.available { now.level.label().to_string() } else { "Not available".into() }, 1);
    server["mark"] = json!(card::serving(now).0);
    if !now.waiting.is_empty() {
        server["badge"] = json!(now.waiting.len().to_string());
    }
    rows.push(server);
    rows.push(json!({"t": "heading", "label": KITS, "depth": 1}));
    let found = found();
    let open = open_workbook(app);
    for kit in kit::loaded() {
        rows.push(item(pane, Pane::kit(kit), kit.name, kit.made_of.join(" · "), 1));
        let Some(found) = &found else { continue };
        for kind in ProjectKind::ALL {
            let of_kind: Vec<&Json> = of_kit(found, kit).filter(|project| kind_of(project) == kind).collect();
            if of_kind.is_empty() {
                continue;
            }
            rows.push(json!({"t": "heading", "label": kind.heading(), "depth": 2}));
            for project in of_kind.iter().take(LISTED) {
                let id = text(project, "id");
                // A project whose folder's name is no id can't be one: it is listed in the kit's part.
                let Some(go) = Pane::parse(&format!("project:{id}")) else { continue };
                let workbook = text(project, "workbook");
                let hint = match (workbook.is_empty(), open.as_deref().is_some_and(|open| manifest::same_workbook(open, workbook))) {
                    (true, _) => String::new(),
                    (false, true) => format!("{workbook}  ·  open"),
                    (false, false) => workbook.to_string(),
                };
                let mut row = item(pane, go, text(project, "name"), hint, 3);
                row["mark"] = project["status"].clone();
                rows.push(row);
            }
            if of_kind.len() > LISTED {
                rows.push(json!({"t": "more", "label": format!("and {} more", of_kind.len() - LISTED), "go": Pane::kit(kit).code(), "depth": 3}));
            }
        }
    }
    Json::Array(rows)
}

// --- the parts ----------------------------------------------------------------------------------

/// The MCP server in a few lines, for the top of the window.
fn server_card(now: &agents::Snapshot) -> Json {
    if !now.available {
        return json!({"available": false, "unavailable": card::NO_AGENTS, "go": Pane::Server.code(), "name": SERVER});
    }
    let (kind, status) = card::serving(now);
    let waiting = match now.waiting.len() {
        0 => Json::Null,
        1 => json!("1 request waits for your answer."),
        count => json!(format!("{count} requests wait for your answer.")),
    };
    let plan = now.plans.iter().find(|plan| plan.ended.is_none()).map(|plan| format!("Plan #{}: {}", plan.id, card::plan_stands(plan)));
    json!({
        "available": true,
        "go": Pane::Server.code(),
        "name": SERVER,
        "says": card::AGENTS,
        "level": now.level.label(),
        "status": {"kind": kind, "text": status},
        "clients": card::clients(now),
        "waiting": waiting,
        "plan": plan,
    })
}

/// How many projects a kit has, and how many of them are trusted.
fn projects_says(found: &Found, kit: &Kit) -> String {
    let trusted = of_kit(found, kit).filter(|project| project["status"] == "trusted").count();
    match of_kit(found, kit).count() {
        0 => "No projects yet".into(),
        count => format!("{count} project{}, {trusted} trusted", plural(count)),
    }
}

/// Where a kit stands: all of them are compiled in so far.
fn kit_stands(kit: &Kit) -> &'static str {
    if kit.built_in { "Built in  ·  loaded" } else { "Loaded" }
}

/// What a kit can't do yet, or nothing for one that is whole.
fn not_yet(kit: &Kit) -> Json {
    if kit.not_yet.is_empty() { Json::Null } else { json!(kit.not_yet) }
}

fn kit_card(kit: &Kit, found: Option<&Found>) -> Json {
    json!({
        "go": Pane::kit(kit).code(),
        "name": kit.name,
        "madeOf": kit.made_of,
        "says": kit.says,
        "standing": kit_stands(kit),
        "notYet": not_yet(kit),
        "projects": found.map(|found| projects_says(found, kit)),
    })
}

/// The open workbook, and the macros it carries, for the top of the window.
fn workbook_card(app: &SheetApp) -> Json {
    let Some(title) = open_workbook(app) else { return json!({"title": null, "says": NO_WORKBOOK, "macros": false}) };
    let Some(covered) = coverage(app) else {
        return json!({"title": title, "says": "It carries no macros.", "macros": false});
    };
    match &covered.summary {
        Err(why) => {
            json!({"title": title, "says": format!("It carries macros (VBA), which could not be read: {why}"), "macros": true, "problem": true})
        }
        Ok(summary) => {
            let modules = summary["modules"].as_array().map_or(0, Vec::len);
            json!({
                "title": title,
                "macros": true,
                "says": format!(
                    "It carries macros (VBA): {modules} module{}, {} procedure{}. GridCraft does not run them.",
                    plural(modules),
                    covered.procedures,
                    plural(covered.procedures)
                ),
                "rebuilt": covered_says(covered.procedures, covered.spoken_for),
            })
        }
    }
}

fn console_says() -> String {
    let kept = super::console::report(&json!({"last": 1})).ok().and_then(|report| report["kept"].as_u64()).unwrap_or(0);
    format!("{kept} entr{}", if kept == 1 { "y" } else { "ies" })
}

/// The top of the window: everything at a glance, each with the way to its own part.
pub(super) fn overview(app: &SheetApp, now: &agents::Snapshot) -> Json {
    let found = found();
    json!({
        "kind": "overview",
        "title": TOP,
        "intro": card::INTRO,
        "server": server_card(now),
        "kitsTitle": KITS,
        "kits": kit::loaded().iter().map(|kit| kit_card(kit, found.as_ref())).collect::<Vec<_>>(),
        "kitsNote": KITS_NOTE,
        "workbook": workbook_card(app),
        "console": console_says(),
    })
}

/// What is said where no compiler is named: which one is looked for, and when.
pub(super) const COMPILER_FOUND: &str = "zig, looked for when a project is built";
/// Said beside the field the compiler is named in.
pub(super) const COMPILER_HINT: &str = "A path or a command. Empty: zig, wherever programs are found.";
/// Said of a project that has a recipe and is not trusted as it is.
pub(super) const BUILD_NEEDS_TRUST: &str = "It is built once you trust it as it is: building runs your compiler over its files.";

/// Said of a project that has a panel and is not trusted as it is.
pub(super) const PANEL_NEEDS_TRUST: &str = "Its panel is shown once you trust it as it is.";
/// Said of the panel of a project that is built, by how its build stands.
pub(super) const PANEL_RUNS: &str = "Its panel runs the program built from the files it holds now.";
pub(super) const PANEL_UNBUILT: &str = "Its panel runs the program built from it, and there is none for the files it holds now: build it first.";
/// Said of any other project's panel.
pub(super) const PANEL_SHOWN: &str = "Its page, shown beside the sheet or in your browser.";

/// A project's panel: what it shows, and whether the person can show it now, docked in the
/// window or in their browser. `None` for a project that has no panel. Showing a panel is the
/// person's alone to do: this only says whether the buttons are there.
fn panel_says(app: &SheetApp, ctx: &egui::Context, project: &Json, build: Option<&Json>) -> Option<Json> {
    let entry = project["panel"].as_str()?;
    let status = text(project, "status");
    let trusted = status == "trusted";
    let says = match build.map(|build| text(build, "state")) {
        Some("built") => PANEL_RUNS,
        Some(_) => PANEL_UNBUILT,
        None => PANEL_SHOWN,
    };
    Some(json!({
        "title": "Panel",
        "entry": entry,
        "says": says,
        // A panel is shown for a project the person trusts as it is, where there is a way to.
        "canDock": trusted && super::dock::available(ctx),
        "canBrowse": trusted && app.services.open_url.is_some(),
        "docked": super::dock::docked(ctx).is_some_and(|docked| docked.project == text(project, "id")),
        // One that can't be used says so above: it has no trust to be given.
        "needsTrust": if trusted || status == "problem" { Json::Null } else { json!(PANEL_NEEDS_TRUST) },
    }))
}

/// A size in words: whole kilobytes, and never none.
fn kilobytes(bytes: u64) -> String {
    format!("{} KB", bytes.div_ceil(1024).max(1))
}

/// A time in words, from milliseconds: tenths of a second when it is short.
fn seconds(millis: u64) -> String {
    if millis < 10_000 { format!("{}.{} s", millis / 1000, (millis % 1000) / 100) } else { format!("{} s", millis / 1000) }
}

/// A project's build: what its recipe builds, how it stands, and whether the person can build
/// it now. `None` for a project that has no recipe.
fn build_says(at: &Places, on_disk: &Project, status: &str) -> Option<Json> {
    let recipe = on_disk.manifest.as_ref()?.c.as_ref()?;
    let builds = match recipe.sources_among(&on_disk.files) {
        Ok(sources) => {
            let shown: Vec<&str> = sources.iter().map(String::as_str).take(NAMED).collect();
            let more = sources.len().saturating_sub(shown.len());
            format!(
                "It builds {} C file{} into one WebAssembly program: {}{}.",
                sources.len(),
                plural(sources.len()),
                shown.join(", "),
                if more > 0 { format!(", and {more} more") } else { String::new() }
            )
        }
        Err(why) => format!("It can't be built: {why}."),
    };
    let standing = c::standing(&at.settings, &on_disk.id, on_disk.fingerprint.as_deref());
    let by_standing = || match &standing {
        c::Standing::Built { bytes } => ("built", format!("Built from the files it holds now: {}, {}.", c::OUTPUT, kilobytes(*bytes))),
        c::Standing::Stale => ("stale", "What was built is from other files than it holds now. Build it again.".to_string()),
        c::Standing::NotBuilt => ("none", "Not built yet.".to_string()),
    };
    let (state, says, output) = match c::Builder::global().progress(&at.projects, &on_disk.id) {
        Some(c::Progress::Building { millis }) => ("building", format!("Building… {} so far.", seconds(millis)), None),
        Some(c::Progress::Done(Err(why))) => {
            let (first, rest) = why.split_once('\n').map_or((why.as_str(), ""), |(first, rest)| (first, rest));
            ("failed", first.to_string(), (!rest.trim().is_empty()).then(|| rest.to_string()))
        }
        Some(c::Progress::Done(Ok(built))) if matches!(standing, c::Standing::Built { .. }) => (
            "built",
            format!(
                "Built: {}, {}, from {} C file{}, in {}, by {}.",
                c::OUTPUT,
                kilobytes(built.bytes),
                built.sources,
                plural(built.sources),
                seconds(built.millis),
                built.compiler
            ),
            None,
        ),
        _ => {
            let (state, says) = by_standing();
            (state, says, None)
        }
    };
    let trusted = status == "trusted";
    Some(json!({
        "title": "Build",
        "builds": builds,
        "state": state,
        "says": says,
        "output": output,
        // Building is for a project the person trusts as it is, and one build at a time.
        "can": trusted && state != "building",
        "needsTrust": if trusted { Json::Null } else { json!(BUILD_NEEDS_TRUST) },
    }))
}

/// One project in the kit's list.
fn project_row(project: &Json, open: Option<&str>) -> Json {
    let id = text(project, "id");
    let workbook = text(project, "workbook");
    let is_open = !workbook.is_empty() && open.is_some_and(|open| manifest::same_workbook(open, workbook));
    json!({
        "go": Pane::parse(&format!("project:{id}")).map(|pane| pane.code()),
        "id": id,
        "name": text(project, "name"),
        "status": project["status"],
        "statusLabel": status_label(text(project, "status")),
        "workbook": if workbook.is_empty() { Json::Null } else { json!(workbook) },
        "open": is_open,
        "says": match (rebuilt_says(&project["rebuilt"]), text(project, "description")) {
            (Some(rebuilt), _) => rebuilt,
            (None, description) => description.to_string(),
        },
    })
}

/// A development kit: what it is, where it stands, what it can't do yet, and its projects by
/// kind. What only one kit has (the Python that runs scripts, the showcase) is said for that
/// kit and left out for the others.
pub(super) fn kit_pane(app: &SheetApp, ctx: &egui::Context, kit: &Kit) -> Json {
    let head = json!({
        "kind": "kit",
        "id": kit.id,
        "title": kit.name,
        "above": [TOP, KITS],
        "madeOf": kit.made_of,
        "standing": kit_stands(kit),
        "says": kit.says,
        "notYet": not_yet(kit),
    });
    let (Some(found), Some(at)) = (found(), projects::places()) else {
        let mut told = head;
        told["available"] = json!(false);
        told["unavailable"] = json!(card::NO_PROJECTS);
        return told;
    };
    let open = open_workbook(app);
    let groups: Vec<Json> = ProjectKind::ALL
        .iter()
        .map(|kind| {
            let rows: Vec<Json> =
                of_kit(&found, kit).filter(|project| kind_of(project) == *kind).map(|project| project_row(project, open.as_deref())).collect();
            json!({"label": kind.heading(), "says": kind.says(), "projects": rows, "empty": "None yet."})
        })
        .collect();
    let mut told = head;
    for (key, value) in [
        ("available", json!(true)),
        ("projectsTitle", json!("Projects")),
        ("projectsSays", json!(projects_says(&found, kit))),
        ("projectsNote", json!(projects_note(kit))),
        ("groups", json!(groups)),
        // A new project can be the code of the workbook that is open, or a library.
        ("newFor", json!(open)),
        ("folder", json!(found.folder)),
        ("canOpenFolder", json!(app.services.open_url.is_some())),
        ("console", json!(console_says())),
    ] {
        told[key] = value;
    }
    // The Python that runs a project's scripts: for a kit whose projects have scripts.
    if kit.scripts {
        let python = settings::load(&at.settings).python;
        told["python"] = json!(format!("Python: {}.", if python.is_empty() { "found automatically" } else { python.as_str() }));
    }
    // The compiler that builds: for a kit whose projects are built. The person names it here.
    if kit.builds {
        let named = settings::load(&at.settings).compiler;
        told["compiler"] = json!({
            "title": "C compiler",
            "says": if named.is_empty() { COMPILER_FOUND } else { named.as_str() },
            "named": named,
            "hint": COMPILER_HINT,
        });
    }
    // The showcase is a project of the first kit.
    if kit.id == kit::ANIE.id {
        let standing = found.projects.iter().find(|project| project["id"] == showcase::ID).map(|project| project["status"] == "trusted");
        told["showcase"] = json!({
            "stands": match standing { None => "none", Some(true) => "trusted", Some(false) => "untrusted" },
            "text": match standing {
                None => "A project whose panel is a Svelte app, docked beside the sheet.",
                Some(true) => "Svelte Showcase is ready.",
                Some(false) => "Svelte Showcase is not trusted yet: review and trust it in Projects, then come back.",
            },
            "canDock": super::dock::available(ctx),
            "canBrowse": app.services.open_url.is_some(),
        });
    }
    told
}

/// One project: what it is for, where it stands, what it asks, what is in it, and what it says
/// it rebuilds of its workbook's macros. `None` when there is no such project any more.
pub(super) fn project_pane(app: &SheetApp, ctx: &egui::Context, id: &str) -> Option<Json> {
    let found = found()?;
    let project = found.projects.iter().find(|project| project["id"] == id)?;
    let open = open_workbook(app);
    let kind = kind_of(project);
    let kit = kit_of(project);
    // What of it is built, and how that stands: for a project with a recipe.
    let build = match (projects::places(), found.on_disk.iter().find(|on_disk| on_disk.id == id)) {
        (Some(at), Some(on_disk)) if kit.builds => build_says(&at, on_disk, text(project, "status")),
        _ => None,
    };
    // Its panel, and the way to show it: a C project's is where the program built from it runs.
    let panel = panel_says(app, ctx, project, build.as_ref());
    let workbook = text(project, "workbook");
    let status = text(project, "status");
    let for_says = match (kind, open.as_deref().is_some_and(|open| manifest::same_workbook(open, workbook))) {
        (ProjectKind::Library, _) => "It is for any workbook.".to_string(),
        (ProjectKind::WorkbookCode, true) => format!("It says it is the code of the workbook {workbook:?}, which is open now."),
        (ProjectKind::WorkbookCode, false) => format!("It says it is the code of the workbook {workbook:?}, which is not the one that is open."),
    };
    let asks: Vec<Json> = project["asks"]
        .as_array()
        .map(|all| all.iter().map(|ask| json!({"text": ask["means"], "warn": ask["unlimited"]})).collect())
        .unwrap_or_default();
    let files: Vec<&str> = project["files"].as_array().map(|all| all.iter().filter_map(Json::as_str).collect()).unwrap_or_default();
    let scripts: Vec<&str> = match project["python"].as_str() {
        Some(folder) => files.iter().filter_map(|file| file.strip_prefix(folder)?.strip_prefix('/')).filter(|file| file.ends_with(".py")).collect(),
        None => vec![],
    };
    let named = |all: &[&str]| {
        let shown: Vec<&str> = all.iter().copied().take(NAMED).collect();
        let more = all.len().saturating_sub(shown.len());
        format!("{}{}", shown.join(", "), if more > 0 { format!(", and {more} more") } else { String::new() })
    };
    let mut parts = Vec::new();
    if let Some(panel) = project["panel"].as_str() {
        parts.push(json!({"label": "Panel", "value": panel}));
    }
    if !scripts.is_empty() {
        parts.push(json!({"label": format!("Script{}", plural(scripts.len())), "value": named(&scripts)}));
    }
    parts.push(json!({"label": format!("{} file{}", files.len(), plural(files.len())), "value": named(&files)}));
    // What it says it rebuilds of the workbook's macros: its own word.
    let note = &project["rebuilt"];
    let rebuilt = if note.is_null() {
        Json::Null
    } else {
        let rows: Vec<Json> = note["procedures"]
            .as_array()
            .map(|all| {
                all.iter()
                    .map(|entry| {
                        let state = rebuilt::State::parse(text(entry, "state"));
                        json!({
                            "from": format!("{}.{}", text(entry, "module"), text(entry, "name")),
                            "state": entry["state"],
                            "stateLabel": state.map_or("", rebuilt::State::label),
                            "script": entry["script"],
                            "scriptMissing": entry["scriptMissing"],
                            "note": entry["note"],
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();
        json!({
            "title": "Rebuilt from macros",
            "says": rebuilt_says(note),
            "problem": note["problem"].is_string(),
            "rows": rows,
            "missing": "The script it names is not in the project.",
        })
    };
    Some(json!({
        "kind": "project",
        "id": id,
        "above": [TOP, KITS, kit.name],
        "title": text(project, "name"),
        "version": text(project, "version"),
        "author": text(project, "author"),
        "description": text(project, "description"),
        "kit": kit.name,
        "kitGo": Pane::kit(kit).code(),
        // What its kit can't do yet is said with the project, where it would be looked for.
        "notYet": not_yet(kit),
        "kindLabel": kind.label(),
        "forSays": for_says,
        "status": status,
        "statusLabel": status_label(status),
        "statusSays": status_says(status),
        "problem": project["problem"],
        "asksTitle": if status == "trusted" { "You allowed it to:" } else { "It asks to be allowed to:" },
        "asks": asks,
        "asksNothing": match (panel.is_some(), build.is_some()) {
            (true, true) => "Show its panel and run its program there, and nothing more.",
            (true, false) => "Show its panel, and nothing more.",
            (false, _) => "Nothing.",
        },
        "parts": parts,
        // Its files, to choose one to read. Which is being read is the window's own to say.
        "files": {
            "title": "Files",
            "says": FILES_NOTE,
            "list": files.iter().take(OFFERED).collect::<Vec<_>>(),
            "more": (files.len() > OFFERED).then(|| format!("and {} more", files.len() - OFFERED)),
        },
        "build": build,
        "panel": panel,
        "rebuilt": rebuilt,
        "manage": "Reviewing, trusting and running it is done in Projects.",
        "canOpenFolder": app.services.open_url.is_some(),
    }))
}

/// One of a project's files as it is shown to be read: its lines, each in pieces that say
/// what they are, by the languages the project's kit describes. `None` when there is no such
/// project any more. A file that can't be shown (it is no file of the project, is not text,
/// is too long) says why in place of its lines.
///
/// The file is read as text and drawn as text. Nothing in it is followed or run, and only a
/// file in the project's own list is read: nothing of what git keeps there, and nothing
/// outside it.
pub(super) fn file_view(id: &str, path: &str) -> Option<Json> {
    let found = found()?;
    let on_disk = found.on_disk.iter().find(|on_disk| on_disk.id == id)?;
    let key = (id.to_string(), on_disk.fingerprint.clone(), path.to_string());
    if let Some(kept) = VIEWED.with(|viewed| viewed.borrow().as_ref().filter(|(of, _)| *of == key).map(|(_, kept)| kept.clone())) {
        return Some((*kept).clone());
    }
    let told = match on_disk.read_text(path, highlight::MAX_BYTES) {
        Err(why) => json!({"path": path, "says": path, "problem": why, "lines": [], "more": null}),
        Ok(text) => {
            let shown = highlight::show(kit::of(on_disk.manifest.as_ref()), path, &text);
            let lines: Vec<Vec<Json>> =
                shown.lines.iter().map(|line| line.iter().map(|(piece, kind)| json!([piece, kind.map(highlight::Kind::code)])).collect()).collect();
            let says = match shown.language {
                Some(language) => format!("{path}  ·  {language}  ·  {} line{}", shown.of_lines, plural(shown.of_lines)),
                None => format!("{path}  ·  {} line{}", shown.of_lines, plural(shown.of_lines)),
            };
            json!({
                "path": path,
                "language": shown.language,
                "says": says,
                "problem": null,
                "lines": lines,
                "more": shown.cut().then(|| format!("Only the first {} of its {} lines are shown.", shown.lines.len(), shown.of_lines)),
            })
        }
    };
    VIEWED.with(|viewed| *viewed.borrow_mut() = Some((key, Arc::new(told.clone()))));
    Some(told)
}

/// Puts the file that is being read into a project's part: `viewing` is the project and the
/// file. It is left out of any other part, and of a project that is not the one read from.
pub(super) fn with_view(told: &mut Json, viewing: Option<&(String, String)>) {
    let Some((id, path)) = viewing else { return };
    if told["kind"] == "project"
        && told["id"] == id.as_str()
        && let Some(view) = file_view(id, path)
    {
        told["view"] = view;
    }
}

/// The part of the window `pane` names, as it is now. A project that is no longer there
/// gives its kit's part.
pub(super) fn pane(app: &SheetApp, ctx: &egui::Context, pane: &Pane, now: &agents::Snapshot) -> Json {
    match pane {
        Pane::Overview => overview(app, now),
        // The server's part is said by whoever draws it: what waits there is decided on.
        Pane::Server => json!({"kind": "server", "title": SERVER, "above": [TOP]}),
        Pane::Kit(id) => kit_pane(app, ctx, kit::named(id).unwrap_or(&kit::ANIE)),
        Pane::Project(id) => project_pane(app, ctx, id).unwrap_or_else(|| kit_pane(app, ctx, &kit::ANIE)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aniefork::macros::tests::{carry, sample};
    use crate::aniefork::projects::tests::{Sandbox, app_with};

    fn write(s: &Sandbox, path: &str, text: &str) {
        let file = s.0.join("projects").join(path);
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(file, text).unwrap();
    }

    /// A project that says it is the code of `workbook`, with a note on the sample's macros.
    fn budget(s: &Sandbox, workbook: &str) {
        write(
            s,
            "budget/project.toml",
            &format!(
                "id = \"budget\"\nname = \"Budget Macros\"\nversion = \"1\"\npermissions = [\"read\", \"python\"]\nworkbook = \"{workbook}\"\n\n[python]\nscripts = \"python\"\n"
            ),
        );
        write(s, "budget/python/add_up.py", "def main(data):\n    return 1\n");
        write(
            s,
            "budget/rebuilt.toml",
            "[[procedure]]\nmodule = \"Totals\"\nname = \"AddUp\"\nstate = \"rebuilt\"\nscript = \"add_up.py\"\nnote = \"Adds A1:A10 up.\"\n\n[[procedure]]\nmodule = \"ThisWorkbook\"\nname = \"Workbook_Open\"\nstate = \"left\"\nnote = \"Nothing runs by itself here.\"\n",
        );
    }

    #[test]
    fn the_parts_of_the_window_have_names_a_window_can_ask_by() {
        for pane in [Pane::Overview, Pane::Server, Pane::kit(&kit::ANIE), Pane::kit(&kit::C), Pane::Project("clean-data".into())] {
            assert_eq!(Pane::parse(&pane.code()), Some(pane.clone()), "{pane:?}");
        }
        assert_eq!((Pane::kit(&kit::ANIE).code(), Pane::kit(&kit::C).code()), ("kit:anie".to_string(), "kit:c".to_string()));
        assert_eq!(Pane::first_kit(), Pane::kit(&kit::ANIE));
        // Only a kit that is loaded has a part.
        for bad in ["", "tools", "project:", "project:../x", "project:Clean Data", "kit", "kit:", "kit:other", "kit:C", "kit:anie/x"] {
            assert_eq!(Pane::parse(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn the_list_has_the_server_at_the_top_and_the_projects_under_their_kit_by_kind() {
        let s = Sandbox::new("kits-nav");
        projects::create("Clean Data").unwrap();
        let mut a = app_with(None);
        let title = a.session.active().unwrap().title.clone();
        budget(&s, &title);
        // Two projects of the C kit: a library, and one that names a workbook.
        projects::create_project_in(&kit::C, "Blink", None).unwrap();
        write(&s, "doom/project.toml", "id = \"doom\"\nname = \"Doom\"\nversion = \"1\"\nkit = \"c\"\nworkbook = \"Doom.xlsx\"\n");
        write(&s, "doom/src/main.c", "int main(void) { return 0; }\n");
        forget();
        let rows = nav(&a, &Pane::kit(&kit::ANIE), &agents::snapshot());
        let listed: Vec<(String, u64)> =
            rows.as_array().unwrap().iter().map(|row| (text(row, "label").to_string(), row["depth"].as_u64().unwrap())).collect();
        let expected = [
            ("Development", 0),
            ("MCP Server", 1),
            ("Development Kits", 1),
            ("Anie Development Kit", 1),
            ("Workbook code", 2),
            ("Budget Macros", 3),
            ("Libraries", 2),
            ("Clean Data", 3),
            // Each kit has its own projects under it, by the same two kinds.
            ("C Development Kit", 1),
            ("Workbook code", 2),
            ("Doom", 3),
            ("Libraries", 2),
            ("Blink", 3),
        ];
        assert_eq!(listed, expected.map(|(label, depth)| (label.to_string(), depth)));
        let row = |label: &str| rows.as_array().unwrap().iter().find(|row| row["label"] == label).unwrap().clone();
        assert_eq!((&row("Anie Development Kit")["selected"], &row("Development")["selected"]), (&json!(true), &json!(false)));
        assert_eq!(row("Anie Development Kit")["hint"], "Python · Svelte");
        assert_eq!(
            (&row("C Development Kit")["go"], &row("C Development Kit")["hint"], &row("C Development Kit")["selected"]),
            (&json!("kit:c"), &json!("C"), &json!(false))
        );
        assert_eq!(
            (&row("Doom")["go"], &row("Doom")["hint"], &row("Blink")["mark"]),
            (&json!("project:doom"), &json!("Doom.xlsx"), &json!("untrusted"))
        );
        assert_eq!(
            (&row("MCP Server")["go"], &row("MCP Server")["hint"], &row("MCP Server")["mark"]),
            (&json!("server"), &json!("Off"), &json!("off"))
        );
        let code = row("Budget Macros");
        assert_eq!((&code["go"], &code["mark"]), (&json!("project:budget"), &json!("untrusted")));
        assert_eq!(code["hint"], json!(format!("{title}  ·  open")), "the workbook it names is the one that is open");
        assert_eq!(row("Clean Data")["hint"], "");
        // Another workbook's code says whose it is, and no more.
        budget(&s, "Elsewhere.xlsm");
        forget();
        let rows = nav(&a, &Pane::Overview, &agents::snapshot());
        assert_eq!(rows.as_array().unwrap().iter().find(|row| row["label"] == "Budget Macros").unwrap()["hint"], "Elsewhere.xlsm");
        // The workbook that is open carries macros: the top says so, and what was rebuilt of them.
        carry(&mut a, sample());
        let top = overview(&a, &agents::snapshot());
        assert_eq!(top["workbook"]["says"], "It carries macros (VBA): 2 modules, 2 procedures. GridCraft does not run them.");
        assert_eq!(top["workbook"]["rebuilt"], "None of them is rebuilt as a project yet.", "the project is another workbook's");
        budget(&s, &title.to_uppercase());
        forget();
        let top = overview(&a, &agents::snapshot());
        assert_eq!(top["workbook"]["rebuilt"], "1 of the 2 is rebuilt, whole or in part, as your projects say.");
        assert_eq!(covered_says(5, 2), "2 of the 5 are rebuilt, whole or in part, as your projects say.");
        assert_eq!(covered_says(2, 2), "All of them are rebuilt, whole or in part, as your projects say.");
        assert_eq!(covered_says(0, 0), "It holds no procedures.");
        assert_eq!(
            (&top["server"]["level"], &top["kits"][0]["name"], &top["kits"][0]["projects"]),
            (&json!("Off"), &json!("Anie Development Kit"), &json!("2 projects, 0 trusted"))
        );
        // Each kit counts its own projects, and neither says there is something it can't do yet.
        assert_eq!(
            (&top["kits"][1]["name"], &top["kits"][1]["projects"], &top["kits"][1]["go"]),
            (&json!("C Development Kit"), &json!("2 projects, 0 trusted"), &json!("kit:c"))
        );
        assert_eq!((&top["kits"][0]["notYet"], &top["kits"][1]["notYet"]), (&Json::Null, &Json::Null));
        assert_eq!(top["kits"][1]["says"], kit::C.says);
        assert_eq!(top["kits"].as_array().map(Vec::len), Some(2));
        assert_eq!(top["kitsNote"], KITS_NOTE);
    }

    #[test]
    fn the_kit_lists_its_projects_by_kind_and_a_project_says_what_it_is_for() {
        let s = Sandbox::new("kits-panes");
        projects::create("Clean Data").unwrap();
        let a = app_with(None);
        let ctx = egui::Context::default();
        let title = a.session.active().unwrap().title.clone();
        budget(&s, &title);
        let told = kit_pane(&a, &ctx, &kit::ANIE);
        assert_eq!(
            (&told["title"], &told["madeOf"], &told["projectsSays"]),
            (&json!("Anie Development Kit"), &json!(["Python", "Svelte"]), &json!("2 projects, 0 trusted"))
        );
        assert_eq!((&told["id"], &told["notYet"]), (&json!("anie"), &Json::Null));
        assert_eq!(
            told["projectsNote"],
            "A project is a folder with a project.toml, Python scripts and a web panel. It does nothing until you trust it."
        );
        assert!(told["python"].is_string() && told["showcase"].is_object(), "what the first kit has of its own");
        assert_eq!(told["newFor"], json!(title));
        let groups = told["groups"].as_array().unwrap();
        assert_eq!((&groups[0]["label"], &groups[1]["label"]), (&json!("Workbook code"), &json!("Libraries")));
        let code = &groups[0]["projects"][0];
        assert_eq!(
            (&code["name"], &code["go"], &code["open"], &code["statusLabel"]),
            (&json!("Budget Macros"), &json!("project:budget"), &json!(true), &json!("Not trusted"))
        );
        assert_eq!(code["says"], "Of the workbook's macros it says: 1 rebuilt, 0 in part, 1 left as they are.");
        assert_eq!(groups[1]["projects"][0]["name"], "Clean Data");
        // One project, in full.
        let one = project_pane(&a, &ctx, "budget").unwrap();
        assert_eq!(
            (&one["title"], &one["kindLabel"], &one["statusLabel"]),
            (&json!("Budget Macros"), &json!("Workbook code"), &json!("Not trusted"))
        );
        assert_eq!(one["forSays"], json!(format!("It says it is the code of the workbook {title:?}, which is open now.")));
        assert_eq!(one["asksTitle"], "It asks to be allowed to:");
        assert_eq!(one["asks"][1], json!({"text": manifest::Permission::Python.meaning(), "warn": true}));
        assert_eq!(one["parts"][0], json!({"label": "Script", "value": "add_up.py"}));
        let rows = one["rebuilt"]["rows"].as_array().unwrap();
        assert_eq!(
            rows[0],
            json!({"from": "Totals.AddUp", "state": "rebuilt", "stateLabel": "Rebuilt", "script": "add_up.py", "scriptMissing": false, "note": "Adds A1:A10 up."})
        );
        assert_eq!(
            (&rows[1]["from"], &rows[1]["stateLabel"], &rows[1]["script"]),
            (&json!("ThisWorkbook.Workbook_Open"), &json!("Left as it is"), &Json::Null)
        );
        let library = project_pane(&a, &ctx, "clean-data").unwrap();
        assert_eq!(
            (&library["kindLabel"], &library["forSays"], &library["rebuilt"]),
            (&json!("Library"), &json!("It is for any workbook."), &Json::Null)
        );
        assert_eq!((&library["kit"], &library["kitGo"], &library["notYet"]), (&json!("Anie Development Kit"), &json!("kit:anie"), &Json::Null));
        assert_eq!(library["asksNothing"], "Show its panel, and nothing more.");
        // A project that is gone gives the first kit's part.
        assert!(project_pane(&a, &ctx, "nowhere").is_none());
        assert_eq!(pane(&a, &ctx, &Pane::Project("nowhere".into()), &agents::snapshot())["kind"], "kit");
        assert_eq!(pane(&a, &ctx, &Pane::Project("budget".into()), &agents::snapshot())["kind"], "project");
    }

    #[test]
    fn the_c_kit_has_a_part_of_its_own_with_its_own_projects() {
        let s = Sandbox::new("kits-c");
        projects::create("Clean Data").unwrap();
        let a = app_with(None);
        let ctx = egui::Context::default();
        // With no project of its own yet, it says what it is. It builds and runs: there is
        // nothing it says it can't do yet.
        let empty = kit_pane(&a, &ctx, &kit::C);
        assert_eq!(
            (&empty["kind"], &empty["id"], &empty["title"], &empty["madeOf"], &empty["standing"]),
            (&json!("kit"), &json!("c"), &json!("C Development Kit"), &json!(["C"]), &json!("Built in  ·  loaded"))
        );
        assert_eq!((&empty["notYet"], &empty["projectsSays"]), (&Json::Null, &json!("No projects yet")));
        assert_eq!(
            empty["projectsNote"],
            "A project is a folder with a project.toml, C sources and a web panel that runs the program built from them. It does nothing until you trust it."
        );
        assert_eq!(
            (&empty["groups"][0]["projects"], &empty["groups"][1]["projects"]),
            (&json!([]), &json!([])),
            "the first kit's are not listed here"
        );
        // What only the first kit has is not said of this one.
        assert_eq!((&empty["python"], &empty["showcase"]), (&Json::Null, &Json::Null));
        // It builds: the compiler is named here, or looked for. The first kit has none to name.
        assert_eq!(empty["compiler"], json!({"title": "C compiler", "says": COMPILER_FOUND, "named": "", "hint": COMPILER_HINT}));
        assert_eq!(kit_pane(&a, &ctx, &kit::ANIE)["compiler"], Json::Null);
        std::fs::write(s.0.join("developer.json"), r#"{"compiler": "C:\\tools\\zig\\zig.exe"}"#).unwrap();
        let named = kit_pane(&a, &ctx, &kit::C);
        assert_eq!((&named["compiler"]["says"], &named["compiler"]["named"]), (&json!("C:\\tools\\zig\\zig.exe"), &json!("C:\\tools\\zig\\zig.exe")));
        std::fs::remove_file(s.0.join("developer.json")).unwrap();
        // A project made in it is its own, and not the first kit's.
        assert_eq!(projects::create_project_in(&kit::C, "Blink", None).unwrap(), "Blink was created. It is not trusted yet.");
        forget();
        let told = kit_pane(&a, &ctx, &kit::C);
        assert_eq!((&told["projectsSays"], &told["groups"][1]["projects"][0]["name"]), (&json!("1 project, 0 trusted"), &json!("Blink")));
        assert_eq!(told["groups"][1]["projects"][0]["go"], "project:blink");
        let first = kit_pane(&a, &ctx, &kit::ANIE);
        assert_eq!((&first["projectsSays"], &first["groups"][1]["projects"].as_array().map(Vec::len)), (&json!("1 project, 0 trusted"), &Some(1)));
        assert_eq!(pane(&a, &ctx, &Pane::kit(&kit::C), &agents::snapshot())["title"], "C Development Kit");
        // The project says which kit it is of.
        let one = project_pane(&a, &ctx, "blink").unwrap();
        assert_eq!(
            (&one["title"], &one["kit"], &one["kitGo"], &one["kindLabel"], &one["statusLabel"]),
            (&json!("Blink"), &json!("C Development Kit"), &json!("kit:c"), &json!("Library"), &json!("Not trusted"))
        );
        assert_eq!(one["above"], json!(["Development", "Development Kits", "C Development Kit"]));
        assert_eq!(
            (&one["notYet"], &one["asks"], &one["asksNothing"]),
            (&Json::Null, &json!([]), &json!("Show its panel and run its program there, and nothing more."))
        );
        assert_eq!(
            one["parts"],
            json!([{"label": "Panel", "value": "panel/index.html"}, {"label": "3 files", "value": "panel/index.html, project.toml, src/main.c"}]),
            "a panel and no scripts"
        );
        // Its panel is where the program built from it runs. It is not trusted, and nothing
        // was built from it: there is no way to show the panel, and it says what is missing.
        assert_eq!(
            one["panel"],
            json!({
                "title": "Panel",
                "entry": "panel/index.html",
                "says": PANEL_UNBUILT,
                "canDock": false,
                "canBrowse": false,
                "docked": false,
                "needsTrust": PANEL_NEEDS_TRUST,
            })
        );
        // Its recipe, and how its build stands: not built, and not to be until it is trusted.
        assert_eq!(
            one["build"],
            json!({
                "title": "Build",
                "builds": "It builds 1 C file into one WebAssembly program: src/main.c.",
                "state": "none",
                "says": "Not built yet.",
                "output": null,
                "can": false,
                "needsTrust": BUILD_NEEDS_TRUST,
            })
        );
        // A program kept under its fingerprint is its own; one kept under another is not.
        let print = found().unwrap().on_disk.iter().find(|on_disk| on_disk.id == "blink").unwrap().fingerprint.clone().unwrap();
        let kept = s.0.join("built/blink").join("0".repeat(64));
        std::fs::create_dir_all(&kept).unwrap();
        std::fs::write(kept.join("program.wasm"), b"\0asm....").unwrap();
        assert_eq!(project_pane(&a, &ctx, "blink").unwrap()["build"]["state"], "stale");
        assert_eq!(
            project_pane(&a, &ctx, "blink").unwrap()["build"]["says"],
            "What was built is from other files than it holds now. Build it again."
        );
        assert_eq!(project_pane(&a, &ctx, "blink").unwrap()["panel"]["says"], PANEL_UNBUILT, "a program built from other files is not its panel's");
        let own = s.0.join("built/blink").join(&print);
        std::fs::create_dir_all(&own).unwrap();
        std::fs::write(own.join("program.wasm"), vec![0u8; 3000]).unwrap();
        let built = project_pane(&a, &ctx, "blink").unwrap()["build"].clone();
        assert_eq!((&built["state"], &built["says"]), (&json!("built"), &json!("Built from the files it holds now: program.wasm, 3 KB.")));
        assert_eq!(project_pane(&a, &ctx, "blink").unwrap()["panel"]["says"], PANEL_RUNS);
        // A project without a recipe, and one of the first kit, have nothing to build: the
        // first kit's panel is a page, and no program's.
        let first = project_pane(&a, &ctx, "clean-data").unwrap();
        assert_eq!(first["build"], Json::Null);
        assert_eq!((&first["panel"]["says"], &first["panel"]["entry"]), (&json!(PANEL_SHOWN), &json!("panel/index.html")));
        assert_eq!((kilobytes(1), kilobytes(1024), kilobytes(1025), kilobytes(0)), ("1 KB".into(), "1 KB".into(), "2 KB".into(), "1 KB".into()));
        assert_eq!((seconds(950), seconds(3_240), seconds(25_900)), ("0.9 s".into(), "3.2 s".into(), "25 s".into()));
        // One that asks for what the first kit runs can't be used, and says why.
        write(&s, "sly/project.toml", "id = \"sly\"\nname = \"Sly\"\nversion = \"1\"\nkit = \"c\"\npermissions = [\"python\"]\n");
        // One of a kit this GridCraft doesn't have is listed under the first kit, as a problem.
        write(&s, "rusty/project.toml", "id = \"rusty\"\nname = \"Rusty\"\nversion = \"1\"\nkit = \"rust\"\n");
        forget();
        let sly = project_pane(&a, &ctx, "sly").unwrap();
        assert_eq!((&sly["kit"], &sly["statusLabel"]), (&json!("C Development Kit"), &json!("Can't be used")));
        assert_eq!(sly["problem"], "it asks for \"python\", which a project of the C Development Kit can't be given");
        let rusty = project_pane(&a, &ctx, "rusty").unwrap();
        assert_eq!((&rusty["kit"], &rusty["statusLabel"]), (&json!("Anie Development Kit"), &json!("Can't be used")));
        assert_eq!(rusty["problem"], "it is a project of the kit \"rust\", which this GridCraft doesn't have");
    }

    #[test]
    fn a_projects_panel_can_be_shown_once_it_is_trusted_and_where_there_is_a_way_to() {
        use crate::aniefork::dock;
        let s = Sandbox::new("kits-panel");
        projects::create("Clean Data").unwrap();
        let asked = crate::aniefork::projects::tests::yes();
        let mut a = app_with(Some(&asked));
        let ctx = egui::Context::default();
        let panel = |a: &SheetApp| {
            forget();
            project_pane(a, &ctx, "clean-data").unwrap()["panel"].clone()
        };
        let ways = |a: &SheetApp| {
            let told = panel(a);
            (told["canDock"] == true, told["canBrowse"] == true, told["docked"] == true, told["needsTrust"].clone())
        };
        // This copy of GridCraft has a web view and a browser to hand a panel to. A project
        // that is not trusted still has no way to be shown.
        dock::host_is_here(&ctx);
        a.services.open_url = Some(Box::new(|_: &str| {}));
        assert_eq!(ways(&a), (false, false, false, json!(PANEL_NEEDS_TRUST)));
        let print = found().unwrap().on_disk[0].fingerprint.clone().unwrap();
        projects::trust(&a, "clean-data", &print, false).unwrap();
        assert_eq!(ways(&a), (true, true, false, Json::Null));
        // Each way is offered only where it is there.
        a.services.open_url = None;
        assert_eq!(ways(&a), (true, false, false, Json::Null));
        let bare = egui::Context::default();
        forget();
        let told = project_pane(&a, &bare, "clean-data").unwrap()["panel"].clone();
        assert_eq!((&told["canDock"], &told["canBrowse"]), (&json!(false), &json!(false)), "no web view, no browser: nothing is offered");
        // Docked, its part says so.
        dock::dock(&a, &ctx, "clean-data").unwrap();
        assert_eq!(ways(&a), (true, false, true, Json::Null));
        // Its files change: it is not the project that was trusted, and there is no way again.
        write(&s, "clean-data/panel/extra.js", "console.log('new');");
        assert_eq!(ways(&a), (false, false, true, json!(PANEL_NEEDS_TRUST)));
        // A project without a panel has none to show, and asks for nothing by being there.
        write(&s, "plain/project.toml", "id = \"plain\"\nname = \"Plain\"\nversion = \"1\"\n");
        forget();
        let plain = project_pane(&a, &ctx, "plain").unwrap();
        assert_eq!((&plain["panel"], &plain["asksNothing"]), (&Json::Null, &json!("Nothing.")));
        // One that can't be used says why above, and is not told to be trusted.
        write(
            &s,
            "broken/project.toml",
            "id = \"broken\"\nname = \"Broken\"\nversion = \"1\"\nkit = \"c\"\npermissions = [\"python\"]\n\n[panel]\nentry = \"panel/index.html\"\n",
        );
        write(&s, "broken/panel/index.html", "<h1>x</h1>");
        forget();
        let broken = project_pane(&a, &ctx, "broken").unwrap();
        assert_eq!((&broken["status"], &broken["panel"]["canDock"], &broken["panel"]["needsTrust"]), (&json!("problem"), &json!(false), &Json::Null));
    }

    #[test]
    fn a_projects_files_are_offered_and_one_is_shown_as_its_kit_reads_it() {
        let s = Sandbox::new("kits-files");
        projects::create_project_in(&kit::C, "Blink", None).unwrap();
        projects::create("Clean Data").unwrap();
        let a = app_with(None);
        let ctx = egui::Context::default();
        let reading = ("blink".to_string(), "src/main.c".to_string());
        let one = project_pane(&a, &ctx, "blink").unwrap();
        assert_eq!(
            one["files"],
            json!({"title": "Files", "says": FILES_NOTE, "list": ["panel/index.html", "project.toml", "src/main.c"], "more": null})
        );
        assert_eq!(one["view"], Json::Null, "none is read until one is chosen");
        // The C source, read as the C kit describes C: in lines, each piece saying what it is.
        let view = file_view("blink", "src/main.c").unwrap();
        assert_eq!(
            (&view["path"], &view["language"], &view["problem"], &view["more"]),
            (&json!("src/main.c"), &json!("C"), &Json::Null, &Json::Null)
        );
        let lines = view["lines"].as_array().unwrap();
        assert_eq!(view["says"], json!(format!("src/main.c  \u{b7}  C  \u{b7}  {} lines", lines.len())));
        let pieces: Vec<&Json> = lines.iter().flat_map(|line| line.as_array().unwrap()).collect();
        for piece in [json!(["/*", "comment"]), json!(["int", "keyword"]), json!(["\"Hello from blink.\"", "string"]), json!(["0", "number"])] {
            assert!(pieces.contains(&&piece), "{piece}");
        }
        assert_eq!(lines[0], json!([["/*", "comment"]]));
        // Its page is read as markup and its manifest as keys; a first-kit project's script as
        // Python, which the C kit does not describe.
        assert_eq!(file_view("blink", "panel/index.html").unwrap()["language"], "HTML");
        assert_eq!(file_view("blink", "project.toml").unwrap()["language"], "TOML");
        assert_eq!(file_view("clean-data", "python/clean.py").unwrap()["language"], "Python");
        write(&s, "blink/notes.py", "import os\n");
        assert_eq!(file_view("blink", "notes.py").unwrap()["language"], Json::Null);
        assert_eq!(file_view("blink", "notes.py").unwrap()["lines"], json!([[["import os", null]]]));
        // What is no file of the project is not read: why is said in place of its lines.
        write(&s, "blink/.git/config", "token = secret-token");
        std::fs::write(s.0.join("outside.txt"), "outside").unwrap();
        for path in [".git/config", "../outside.txt", "../../outside.txt", "nowhere.c", ""] {
            let refused = file_view("blink", path).unwrap();
            assert!(refused["problem"].as_str().is_some_and(|why| why.ends_with("is not a file of this project.")), "{path}: {refused}");
            assert_eq!(refused["lines"], json!([]));
            assert!(!refused.to_string().contains("secret-token") && !refused.to_string().contains("outside\""));
        }
        assert!(file_view("nowhere", "src/main.c").is_none());
        assert!(!project_pane(&a, &ctx, "blink").unwrap()["files"].to_string().contains(".git"), "what git keeps is not offered");
        // It is put into the part of the project it is a file of, and into no other part.
        let mut told = project_pane(&a, &ctx, "blink").unwrap();
        with_view(&mut told, Some(&reading));
        assert_eq!(told["view"]["path"], "src/main.c");
        let mut other = project_pane(&a, &ctx, "clean-data").unwrap();
        with_view(&mut other, Some(&reading));
        let mut of_kit = kit_pane(&a, &ctx, &kit::C);
        with_view(&mut of_kit, Some(&reading));
        let mut none = project_pane(&a, &ctx, "blink").unwrap();
        with_view(&mut none, None);
        assert_eq!((&other["view"], &of_kit["view"], &none["view"]), (&Json::Null, &Json::Null, &Json::Null));
        // The file changes: what is shown is the file as it is now.
        write(&s, "blink/src/main.c", "int two;\n");
        assert_eq!(file_view("blink", "src/main.c").unwrap()["lines"], json!([[["int", "keyword"], [" two;", null]]]));
        // What is not text, and what is too long, say so.
        std::fs::write(s.0.join("projects/blink/panel/data.bin"), [0u8, 159, 146, 150]).unwrap();
        write(&s, "blink/long.md", &"x".repeat(highlight::MAX_BYTES as usize + 1));
        assert_eq!(file_view("blink", "panel/data.bin").unwrap()["problem"], "panel/data.bin is not text.");
        assert!(file_view("blink", "long.md").unwrap()["problem"].as_str().unwrap().contains("too long to show"));
        // A long file is shown in part, and says so.
        write(&s, "blink/many.c", &"int x;\n".repeat(highlight::MAX_LINES + 5));
        let many = file_view("blink", "many.c").unwrap();
        assert_eq!(many["lines"].as_array().map(Vec::len), Some(highlight::MAX_LINES));
        assert_eq!(many["more"], json!(format!("Only the first {} of its {} lines are shown.", highlight::MAX_LINES, highlight::MAX_LINES + 5)));
        // A project with very many files offers so many, and says there are more.
        for n in 0..(OFFERED + 3) {
            write(&s, &format!("blink/doom/f{n:04}.h"), "");
        }
        let full = project_pane(&a, &ctx, "blink").unwrap();
        assert_eq!(full["files"]["list"].as_array().map(Vec::len), Some(OFFERED));
        assert!(full["files"]["more"].as_str().is_some_and(|more| more.starts_with("and ") && more.ends_with(" more")));
    }

    #[test]
    fn a_run_without_settings_has_a_kit_and_no_projects() {
        agents::forget();
        let a = app_with(None);
        let ctx = egui::Context::default();
        assert!(found().is_none());
        for kit in kit::loaded() {
            let told = kit_pane(&a, &ctx, kit);
            assert_eq!((&told["title"], &told["available"], &told["unavailable"]), (&json!(kit.name), &json!(false), &json!(card::NO_PROJECTS)));
        }
        let rows = nav(&a, &Pane::Overview, &agents::snapshot());
        assert_eq!(rows.as_array().unwrap().len(), 5, "the top, the server, the heading and the two kits");
        assert_eq!(overview(&a, &agents::snapshot())["kits"][0]["projects"], Json::Null);
    }
}
