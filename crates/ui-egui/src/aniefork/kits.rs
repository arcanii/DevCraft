//! What the Development window is made of, and what it says of the kits, their projects and
//! the open workbook's macros.
//!
//! The window has a list down its side and one part shown beside it:
//!
//! ```text
//! Development                    the whole, at a glance
//!   MCP Server                   agents: how far they may go, and what they ask
//!   Development Kits
//!     Anie Development Kit       Python and Svelte: the kit this build has
//!       Workbook code            projects that are the code of one workbook
//!       Libraries                projects for any workbook
//! ```
//!
//! A **kit** is a way of building on GridCraft (`gridcraft_aniefork_dev::kit`); a **project**
//! is a folder of the kit's, with a `project.toml`. The MCP server is not a kit's: it stands
//! beside them, at the top.
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

use gridcraft_aniefork_dev::kit::{self, ProjectKind};
use gridcraft_aniefork_dev::{Project, TrustStore, describe, manifest, project, rebuilt, settings, showcase, vba};
use serde_json::{Value as Json, json};

use super::{agents, card, macros, projects};
use crate::SheetApp;

/// The window's name for its top, and for the two things under it.
pub(super) const TOP: &str = "Development";
pub(super) const SERVER: &str = "MCP Server";
pub(super) const KITS: &str = "Development Kits";
/// Said under the kits: there is one, and no way to another yet.
pub(super) const KITS_NOTE: &str = "Only the kit that is built into GridCraft can be used so far: there is no way to load another yet.";
/// Said where projects are listed: what a project is, in the words the rest of GridCraft uses.
pub(super) const PROJECTS_NOTE: &str =
    "A project is a folder with a project.toml, Python scripts and a web panel. It does nothing until you trust it.";
pub(super) const NO_WORKBOOK: &str = "No workbook is open.";
/// The most projects of one kind listed down the side. The kit's own part lists them all.
const LISTED: usize = 40;
/// The most of a project's files named.
const NAMED: usize = 12;
/// How long what was read from disk is shown before it is read again (milliseconds).
const KEEP_MS: f64 = if cfg!(test) { 0.0 } else { 1000.0 };

/// Which part of the window is shown.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) enum Pane {
    #[default]
    Overview,
    /// The MCP server: agents.
    Server,
    /// The Anie Development Kit.
    Kit,
    /// One of its projects, by id.
    Project(String),
}

impl Pane {
    /// The word a window uses to ask for this part.
    pub(super) fn code(&self) -> String {
        match self {
            Pane::Overview => "overview".into(),
            Pane::Server => "server".into(),
            Pane::Kit => "kit".into(),
            Pane::Project(id) => format!("project:{id}"),
        }
    }

    pub(super) fn parse(code: &str) -> Option<Pane> {
        match code {
            "overview" => Some(Pane::Overview),
            "server" => Some(Pane::Server),
            "kit" => Some(Pane::Kit),
            _ => code.strip_prefix("project:").filter(|id| manifest::valid_id(id)).map(|id| Pane::Project(id.to_string())),
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
        rows.push(item(pane, Pane::Kit, kit.name, kit.made_of.join(" · "), 1));
        let Some(found) = &found else { continue };
        for kind in ProjectKind::ALL {
            let of_kind: Vec<&Json> = found.projects.iter().filter(|project| kind_of(project) == kind).collect();
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
                rows.push(json!({"t": "more", "label": format!("and {} more", of_kind.len() - LISTED), "go": Pane::Kit.code(), "depth": 3}));
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

/// How many projects there are, and how many of them are trusted.
fn projects_says(found: &Found) -> String {
    let trusted = found.projects.iter().filter(|project| project["status"] == "trusted").count();
    match found.projects.len() {
        0 => "No projects yet".into(),
        count => format!("{count} project{}, {trusted} trusted", plural(count)),
    }
}

fn kit_card(kit: &kit::Kit, found: Option<&Found>) -> Json {
    json!({
        "go": Pane::Kit.code(),
        "name": kit.name,
        "madeOf": kit.made_of,
        "says": kit.says,
        "standing": if kit.built_in { "Built in  ·  loaded" } else { "Loaded" },
        "projects": found.map(projects_says),
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

/// The Anie Development Kit: what it is, where it stands, and its projects by kind.
pub(super) fn kit_pane(app: &SheetApp, ctx: &egui::Context) -> Json {
    let kit = kit::ANIE;
    let head =
        json!({"kind": "kit", "title": kit.name, "above": [TOP, KITS], "madeOf": kit.made_of, "standing": "Built in  ·  loaded", "says": kit.says});
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
                found.projects.iter().filter(|project| kind_of(project) == *kind).map(|project| project_row(project, open.as_deref())).collect();
            json!({"label": kind.heading(), "says": kind.says(), "projects": rows, "empty": "None yet."})
        })
        .collect();
    let python = settings::load(&at.settings).python;
    let standing = found.projects.iter().find(|project| project["id"] == showcase::ID).map(|project| project["status"] == "trusted");
    let mut told = head;
    for (key, value) in [
        ("available", json!(true)),
        ("projectsTitle", json!("Projects")),
        ("projectsSays", json!(projects_says(&found))),
        ("projectsNote", json!(PROJECTS_NOTE)),
        ("groups", json!(groups)),
        // A new project can be the code of the workbook that is open, or a library.
        ("newFor", json!(open)),
        ("python", json!(format!("Python: {}.", if python.is_empty() { "found automatically" } else { python.as_str() }))),
        ("folder", json!(found.folder)),
        ("canOpenFolder", json!(app.services.open_url.is_some())),
        ("console", json!(console_says())),
        (
            "showcase",
            json!({
                "stands": match standing { None => "none", Some(true) => "trusted", Some(false) => "untrusted" },
                "text": match standing {
                    None => "A project whose panel is a Svelte app, docked beside the sheet.",
                    Some(true) => "Svelte Showcase is ready.",
                    Some(false) => "Svelte Showcase is not trusted yet: review and trust it in Projects, then come back.",
                },
                "canDock": super::dock::available(ctx),
                "canBrowse": app.services.open_url.is_some(),
            }),
        ),
    ] {
        told[key] = value;
    }
    told
}

/// One project: what it is for, where it stands, what it asks, what is in it, and what it says
/// it rebuilds of its workbook's macros. `None` when there is no such project any more.
pub(super) fn project_pane(app: &SheetApp, id: &str) -> Option<Json> {
    let found = found()?;
    let project = found.projects.iter().find(|project| project["id"] == id)?;
    let open = open_workbook(app);
    let kind = kind_of(project);
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
        "above": [TOP, KITS, kit::ANIE.name],
        "title": text(project, "name"),
        "version": text(project, "version"),
        "author": text(project, "author"),
        "description": text(project, "description"),
        "kit": kit::ANIE.name,
        "kindLabel": kind.label(),
        "forSays": for_says,
        "status": status,
        "statusLabel": status_label(status),
        "statusSays": status_says(status),
        "problem": project["problem"],
        "asksTitle": if status == "trusted" { "You allowed it to:" } else { "It asks to be allowed to:" },
        "asks": asks,
        "asksNothing": "Show its panel, and nothing more.",
        "parts": parts,
        "rebuilt": rebuilt,
        "manage": "Reviewing, trusting and running it is done in Projects.",
        "canOpenFolder": app.services.open_url.is_some(),
    }))
}

/// The part of the window `pane` names, as it is now. A project that is no longer there
/// gives its kit's part.
pub(super) fn pane(app: &SheetApp, ctx: &egui::Context, pane: &Pane, now: &agents::Snapshot) -> Json {
    match pane {
        Pane::Overview => overview(app, now),
        // The server's part is said by whoever draws it: what waits there is decided on.
        Pane::Server => json!({"kind": "server", "title": SERVER, "above": [TOP]}),
        Pane::Kit => kit_pane(app, ctx),
        Pane::Project(id) => project_pane(app, id).unwrap_or_else(|| kit_pane(app, ctx)),
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
        for pane in [Pane::Overview, Pane::Server, Pane::Kit, Pane::Project("clean-data".into())] {
            assert_eq!(Pane::parse(&pane.code()), Some(pane.clone()), "{pane:?}");
        }
        for bad in ["", "tools", "project:", "project:../x", "project:Clean Data", "kit:other"] {
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
        let rows = nav(&a, &Pane::Kit, &agents::snapshot());
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
        ];
        assert_eq!(listed, expected.map(|(label, depth)| (label.to_string(), depth)));
        let row = |label: &str| rows.as_array().unwrap().iter().find(|row| row["label"] == label).unwrap().clone();
        assert_eq!((&row("Anie Development Kit")["selected"], &row("Development")["selected"]), (&json!(true), &json!(false)));
        assert_eq!(row("Anie Development Kit")["hint"], "Python · Svelte");
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
        let told = kit_pane(&a, &ctx);
        assert_eq!(
            (&told["title"], &told["madeOf"], &told["projectsSays"]),
            (&json!("Anie Development Kit"), &json!(["Python", "Svelte"]), &json!("2 projects, 0 trusted"))
        );
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
        let one = project_pane(&a, "budget").unwrap();
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
        let library = project_pane(&a, "clean-data").unwrap();
        assert_eq!(
            (&library["kindLabel"], &library["forSays"], &library["rebuilt"]),
            (&json!("Library"), &json!("It is for any workbook."), &Json::Null)
        );
        // A project that is gone gives its kit's part.
        assert!(project_pane(&a, "nowhere").is_none());
        assert_eq!(pane(&a, &ctx, &Pane::Project("nowhere".into()), &agents::snapshot())["kind"], "kit");
        assert_eq!(pane(&a, &ctx, &Pane::Project("budget".into()), &agents::snapshot())["kind"], "project");
    }

    #[test]
    fn a_run_without_settings_has_a_kit_and_no_projects() {
        agents::forget();
        let a = app_with(None);
        let ctx = egui::Context::default();
        assert!(found().is_none());
        let told = kit_pane(&a, &ctx);
        assert_eq!(
            (&told["title"], &told["available"], &told["unavailable"]),
            (&json!("Anie Development Kit"), &json!(false), &json!(card::NO_PROJECTS))
        );
        let rows = nav(&a, &Pane::Overview, &agents::snapshot());
        assert_eq!(rows.as_array().unwrap().len(), 4, "the top, the server, the heading and the kit");
        assert_eq!(overview(&a, &agents::snapshot())["kits"][0]["projects"], Json::Null);
    }
}
