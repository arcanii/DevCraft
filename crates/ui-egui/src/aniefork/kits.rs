//! What the Development window is made of, and what it says of the kits, their projects and
//! the open workbook's macros.
//!
//! The window has a list down its side and one part shown beside it:
//!
//! ```text
//! Development                    the whole, at a glance
//!   MCP Server                   agents: how far they may go, and what they ask
//!   Development Kits
//!     PythonCraft                Python and Svelte
//!     C Development Kit          C: built as WebAssembly, and run in the project's panel
//! ```
//!
//! A **kit** is a way of building on GridCraft (`gridcraft_aniefork_dev::kit`); a **project**
//! is a folder with a `project.toml`, which says which kit it is of. Each kit has a part of
//! its own, made the same way from what the kit says of itself, and its projects are listed
//! there: the list down the side stops at the kits (the owner, 2026-10-13). A project has a
//! part too, reached from its kit's. The MCP server is not a kit's: it stands beside them, at
//! the top.
//!
//! A project is called a project. The one that names a workbook says so beside its name; the
//! one that names none is for any workbook, and is not called a library (`kit.rs` says why).
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
pub(super) const KITS_NOTE: &str = "The kits GridCraft is built with can't be removed. A kit loaded from a package describes its projects and runs nothing of its own: they run by a built-in kit.";
/// Said on the way to load a kit, and where there is none.
pub(super) const LOAD_KIT: &str = "Load a development kit from a package file (.gridkit)";
pub(super) const NO_PICKER: &str = "This copy of GridCraft can't show a file dialog, so it can't load a kit.";

/// Said where a kit's projects are listed: what a project of it is, and that it does nothing
/// by being there.
pub(super) fn projects_note(kit: &Kit) -> String {
    format!("{} It does nothing until you trust it.", kit.project_is)
}
pub(super) const NO_WORKBOOK: &str = "No workbook is open.";
/// Said where a kit has no project.
pub(super) const NONE_YET: &str = "None yet. Create one below, or copy a project's folder into the projects folder.";
/// The most of a project's files named.
const NAMED: usize = 12;
/// The most of a project's files offered for reading.
const OFFERED: usize = 240;
/// Said where a project's files are offered for reading.
pub(super) const FILES_NOTE: &str = "Choose a file to read it. It is shown as text: nothing in it is run.";
/// How long what was read from disk is shown before it is read again (milliseconds).
const KEEP_MS: f64 = if cfg!(test) { 0.0 } else { 1000.0 };
/// How long a measure of the space the kits use is kept (milliseconds): taking it walks
/// folders, and what a compiler keeps beside what it built can be many files.
const MEASURE_MS: f64 = if cfg!(test) { 0.0 } else { 15_000.0 };
/// The most files and folders looked at in one measure. Past it the measure says "more than".
const MEASURED: usize = 60_000;
/// Said in the field the kits are filtered by, and when none of them matches.
pub(super) const FILTER_HINT: &str = "Filter kits";
pub(super) const FILTER_NONE: &str = "No development kit matches.";
/// What the space a kit uses is made of.
pub(super) const SPACE_COUNTS: &str = "Its projects' files.";
pub(super) const SPACE_COUNTS_BUILT: &str = "Its projects' files, and what was built from them with what the compiler keeps beside it.";

/// Which part of the window is shown.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) enum Pane {
    #[default]
    Overview,
    /// The MCP server: agents.
    Server,
    /// The Console: what projects printed and logged.
    Console,
    /// The macros the open workbook carries, to read.
    Macros,
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
        Pane::kit(&kit::PYTHONCRAFT)
    }

    /// The word a window uses to ask for this part.
    pub(super) fn code(&self) -> String {
        match self {
            Pane::Overview => "overview".into(),
            Pane::Server => "server".into(),
            Pane::Console => "console".into(),
            Pane::Macros => "macros".into(),
            Pane::Kit(id) => format!("kit:{id}"),
            Pane::Project(id) => format!("project:{id}"),
        }
    }

    pub(super) fn parse(code: &str) -> Option<Pane> {
        match code {
            "overview" => Some(Pane::Overview),
            "server" => Some(Pane::Server),
            "console" => Some(Pane::Console),
            "macros" => Some(Pane::Macros),
            _ => match code.strip_prefix("kit:") {
                // Only a kit that is loaded has a part.
                Some(id) => kit_named(id).map(Pane::kit),
                None => code.strip_prefix("project:").filter(|id| manifest::valid_id(id)).map(|id| Pane::Project(id.to_string())),
            },
        }
    }
}

/// The projects on disk, as they were last read.
#[derive(Clone)]
pub(super) struct Found {
    pub on_disk: Arc<Vec<Project>>,
    /// The kits there are: the ones GridCraft is built with, and those loaded from packages.
    pub kits: Arc<Vec<&'static Kit>>,
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
    /// The space the kits use, as it was last measured: when, and for which reading of the projects.
    static MEASURE: RefCell<Option<(f64, PathBuf, Arc<Space>)>> = const { RefCell::new(None) };
}

/// The space the kits use on disk: for each kit by its id, in bytes.
pub(super) struct Space {
    kits: Vec<(&'static str, u64)>,
    /// The measure stopped before it had looked at everything: the true figure is larger.
    cut: bool,
}

impl Space {
    fn of(&self, kit: &Kit) -> u64 {
        self.kits.iter().find(|(id, _)| *id == kit.id).map_or(0, |(_, bytes)| *bytes)
    }

    fn all(&self) -> u64 {
        self.kits.iter().fold(0, |sum, (_, bytes)| sum.saturating_add(*bytes))
    }
}

/// The size of everything under `folder`, in bytes. Links are not followed and not counted;
/// `left` is how many more files and folders may be looked at, and is counted down.
fn folder_bytes(folder: &std::path::Path, left: &mut usize) -> u64 {
    let mut bytes = 0u64;
    let mut waiting = vec![folder.to_path_buf()];
    while let Some(next) = waiting.pop() {
        let Ok(entries) = std::fs::read_dir(&next) else { continue };
        for entry in entries.flatten() {
            if *left == 0 {
                return bytes;
            }
            *left -= 1;
            let Ok(found) = std::fs::symlink_metadata(entry.path()) else { continue };
            if found.is_dir() {
                waiting.push(entry.path());
            } else if found.is_file() {
                bytes = bytes.saturating_add(found.len());
            }
        }
    }
    bytes
}

/// The space the kits use, measured at most once in a while: for each kit its projects'
/// files, and for a kit that builds also what was built, with what the compiler keeps
/// beside it. `None` in a run that keeps no settings.
pub(super) fn space() -> Option<Arc<Space>> {
    let at = projects::places()?;
    let found = found()?;
    let now = crate::now_ms();
    MEASURE.with(|kept| {
        let mut kept = kept.borrow_mut();
        if let Some((taken_at, of, space)) = &*kept
            && *of == at.projects
            && (0.0..MEASURE_MS).contains(&(now - taken_at))
        {
            return Some(space.clone());
        }
        let mut left = MEASURED;
        let kits = found
            .kits
            .iter()
            .map(|kit| {
                let mut bytes = 0u64;
                for project in found.on_disk.iter().filter(|project| kit::of_in(&at.projects, project.manifest.as_ref()).id == kit.id) {
                    for file in &project.files {
                        if left == 0 {
                            break;
                        }
                        left -= 1;
                        if let Ok(found) = std::fs::symlink_metadata(project.dir.join(file)) {
                            bytes = bytes.saturating_add(found.len());
                        }
                    }
                }
                // What was built is counted once, for the kit that builds: one that stands
                // on it shares the same folder.
                if kit.builds && kit.built_in {
                    bytes = bytes.saturating_add(folder_bytes(&c::built_dir(&at.settings), &mut left));
                }
                // A kit from a package uses its own folder too.
                if !kit.built_in
                    && let Some(kits) = kit::folder_beside(&at.projects)
                {
                    bytes = bytes.saturating_add(folder_bytes(&kits.join(kit.id), &mut left));
                }
                (kit.id, bytes)
            })
            .collect();
        let space = Arc::new(Space { kits, cut: left == 0 });
        *kept = Some((now, at.projects.clone(), space.clone()));
        Some(space)
    })
}

/// A size in words a person reads at a glance.
pub(super) fn bytes_say(bytes: u64) -> String {
    const KB: u64 = 1024;
    match bytes {
        0 => "0 KB".into(),
        b if b < KB * KB => format!("{} KB", b.div_ceil(KB)),
        b if b < KB * KB * KB => format!("{:.1} MB", b as f64 / (KB * KB) as f64),
        b => format!("{:.1} GB", b as f64 / (KB * KB * KB) as f64),
    }
}

/// The space one kit uses, in words.
fn space_says(space: &Space, kit: &Kit) -> String {
    format!("{}{}", if space.cut { "more than " } else { "" }, bytes_say(space.of(kit)))
}

/// How many kits there are and the space they use, in a line.
fn kits_summary(space: Option<&Space>) -> String {
    let count = all_kits().len();
    match space {
        None => format!("{count} kit{}", plural(count)),
        Some(space) => format!("{count} kit{}  ·  {}{}", plural(count), if space.cut { "more than " } else { "" }, bytes_say(space.all())),
    }
}

/// Everything a kit says of itself and of its projects, in small letters: what a search of
/// the kits looks through.
fn kit_search(kit: &Kit, found: Option<&Found>) -> String {
    let mut all = vec![kit.name.to_string(), kit.id.to_string(), kit.made_of.join(" "), kit.says.to_string(), kit.project_is.to_string()];
    if let Some(found) = found {
        for project in of_kit(found, kit) {
            for key in ["name", "id", "description", "workbook"] {
                all.push(text(project, key).to_string());
            }
        }
    }
    all.join("\n").to_lowercase()
}

/// Whether what a kit says holds every word of what was typed to filter by.
pub(super) fn matches(search: &str, typed: &str) -> bool {
    typed.split_whitespace().all(|word| search.contains(&word.to_lowercase()))
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
        let found = Found {
            on_disk: Arc::new(all),
            kits: Arc::new(kit::all(&at.projects)),
            projects: Arc::new(projects),
            folder: at.projects.display().to_string(),
            stamp,
        };
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
    // What they use on disk may have changed with it.
    MEASURE.with(|kept| {
        if let Some((taken_at, _, _)) = &mut *kept.borrow_mut() {
            *taken_at = f64::NEG_INFINITY;
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
    if project["kind"] == ProjectKind::WorkbookCode.code() { ProjectKind::WorkbookCode } else { ProjectKind::General }
}

/// The kit a described project is listed under.
fn kit_of(project: &Json) -> &'static Kit {
    kit_named(text(project, "kit")).unwrap_or(&kit::PYTHONCRAFT)
}

/// The kits there are: the ones GridCraft is built with, and those loaded from packages.
pub(super) fn all_kits() -> Vec<&'static Kit> {
    found().map_or_else(|| kit::loaded().iter().collect(), |found| found.kits.to_vec())
}

/// The kit with this id, among those there are.
pub(super) fn kit_named(id: &str) -> Option<&'static Kit> {
    kit::named(id).or_else(|| all_kits().into_iter().find(|kit| kit.id == id))
}

thread_local! {
    /// The shell's way to let the person choose a kit's package file, when it has one.
    static PICKER: RefCell<Option<Box<dyn Fn() -> Option<String>>>> = const { RefCell::new(None) };
}

/// The shell says how the person chooses a kit's package: the operating system's own file dialog.
pub(super) fn set_picker(picker: Box<dyn Fn() -> Option<String>>) {
    PICKER.with(|kept| *kept.borrow_mut() = Some(picker));
}

pub(super) fn has_picker() -> bool {
    PICKER.with(|kept| kept.borrow().is_some())
}

/// Asks the person to choose a package. `Err` when there is no way to ask; `Ok(None)` when
/// they chose nothing.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn pick_package() -> Result<Option<String>, String> {
    PICKER.with(|kept| kept.borrow().as_ref().map(|pick| pick()).ok_or_else(|| NO_PICKER.to_string()))
}

thread_local! {
    /// The shell's way to let the person choose the compiler's program, when it has one.
    static COMPILER_PICKER: RefCell<Option<Box<dyn Fn() -> Option<String>>>> = const { RefCell::new(None) };
}

/// The shell says how the person chooses the compiler: the operating system's own file dialog.
pub(super) fn set_compiler_picker(picker: Box<dyn Fn() -> Option<String>>) {
    COMPILER_PICKER.with(|kept| *kept.borrow_mut() = Some(picker));
}

pub(super) fn has_compiler_picker() -> bool {
    COMPILER_PICKER.with(|kept| kept.borrow().is_some())
}

/// Asks the person to choose the compiler's program. `Err` when there is no way to ask;
/// `Ok(None)` when they chose nothing.
pub(super) fn pick_compiler() -> Result<Option<String>, String> {
    COMPILER_PICKER
        .with(|kept| kept.borrow().as_ref().map(|pick| pick()))
        .ok_or_else(|| "This copy of GridCraft can't show a file dialog: type where the compiler is.".to_string())
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

/// The list down the window's side: the top, the MCP server and the kits, and which of them
/// is shown. A kit's projects are not listed here: they are in the kit's own part.
pub(super) fn nav(app: &SheetApp, pane: &Pane, now: &agents::Snapshot) -> Json {
    let mut top = item(pane, Pane::Overview, TOP, String::new(), 0);
    // Which of a fixed few pictures stands beside it: the window draws it, not what is said here.
    top["icon"] = json!("tools");
    let mut rows = vec![top];
    // The MCP server stands at the top, beside the kits: it is not a kit's.
    let mut server = item(pane, Pane::Server, SERVER, if now.available { now.level.label().to_string() } else { "Not available".into() }, 1);
    server["mark"] = json!(card::serving(now).0);
    if !now.waiting.is_empty() {
        server["badge"] = json!(now.waiting.len().to_string());
    }
    rows.push(server);
    // The Console and the open workbook's macros are no kit's either: they stand here too.
    rows.push(item(pane, Pane::Console, "Console", console_says(), 1));
    let carries = match (open_workbook(app), coverage(app)) {
        (None, _) => "No workbook is open".to_string(),
        (Some(_), None) => "None in this workbook".to_string(),
        (Some(title), Some(_)) => format!("In {title}"),
    };
    rows.push(item(pane, Pane::Macros, "Macros", carries, 1));
    let found = found();
    let space = space();
    // Over the kits: how many they are and the space they use, and a field to filter them by.
    rows.push(json!({
        "t": "heading",
        "label": KITS,
        "depth": 1,
        "says": kits_summary(space.as_deref()),
        "filter": FILTER_HINT,
        "none": FILTER_NONE,
        // The way to load another kit, where the shell can show a file dialog.
        "add": if has_picker() { json!(LOAD_KIT) } else { Json::Null },
    }));
    // The kit whose project is shown, when one is: its row is the way back to it.
    let within = match (pane, &found) {
        (Pane::Project(id), Some(found)) => found.projects.iter().find(|project| project["id"] == id.as_str()).map(|project| kit_of(project).id),
        _ => None,
    };
    for kit in all_kits() {
        // A kit stands apart from the rest of the list: its own row, with its own picture.
        let mut row = item(pane, Pane::kit(kit), kit.name, kit.made_of.join(" · "), 1);
        row["t"] = json!("kit");
        row["icon"] = json!("shield");
        row["projects"] = json!(found.as_ref().map(|found| projects_count(found, kit)));
        row["within"] = json!(within == Some(kit.id));
        row["search"] = json!(kit_search(kit, found.as_ref()));
        rows.push(row);
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

/// How many projects a kit has, in a word or two: for the list down the side.
fn projects_count(found: &Found, kit: &Kit) -> String {
    match of_kit(found, kit).count() {
        0 => "No projects".into(),
        count => format!("{count} project{}", plural(count)),
    }
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
    if kit.built_in { "Built in  ·  loaded" } else { "Loaded from a package" }
}

/// What a kit can't do yet, or nothing for one that is whole.
fn not_yet(kit: &Kit) -> Json {
    if kit.not_yet.is_empty() { Json::Null } else { json!(kit.not_yet) }
}

/// What a kit is made of, each as a badge: its name, and which of the few badges there are
/// is its own. The windows draw the badge; the name is drawn as text.
fn badges(kit: &Kit) -> Vec<Json> {
    kit.made_of.iter().map(|name| json!({"label": name, "code": super::icons::badge_for(name).code})).collect()
}

fn kit_card(kit: &Kit, found: Option<&Found>, space: Option<&Space>) -> Json {
    json!({
        "search": kit_search(kit, found),
        "space": space.map(|space| space_says(space, kit)),
        "go": Pane::kit(kit).code(),
        "name": kit.name,
        "madeOf": kit.made_of,
        "badges": badges(kit),
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
/// Which DevCraft this is, and which GridCraft it is built on: said at the top of the window.
pub(super) fn versions() -> String {
    format!("DevCraft {}  \u{b7}  GridCraft {}", gridcraft_aniefork_dev::DEVCRAFT_VERSION, env!("CARGO_PKG_VERSION"))
}

pub(super) fn overview(app: &SheetApp, now: &agents::Snapshot) -> Json {
    let found = found();
    let space = space();
    json!({
        "kind": "overview",
        "title": TOP,
        "version": versions(),
        "intro": card::INTRO,
        "server": server_card(now),
        "kitsTitle": KITS,
        "kitsSummary": kits_summary(space.as_deref()),
        "kitsNone": FILTER_NONE,
        "kits": all_kits().into_iter().map(|kit| kit_card(kit, found.as_ref(), space.as_deref())).collect::<Vec<_>>(),
        "kitsNote": KITS_NOTE,
        "workbook": workbook_card(app),
        "console": console_says(),
    })
}

/// What is said where no compiler is named: which one is looked for, and when.
pub(super) const COMPILER_FOUND: &str = "zig, looked for when a project is built";
/// Said beside the field the compiler is named in.
pub(super) const COMPILER_HINT: &str = "The zig program, the folder it is in, or a command. Empty: zig, wherever programs are found.";
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
        // Said beside the name of a project that is one workbook's code. One for any workbook
        // says nothing there: it is what a project is unless it says otherwise.
        "for": match (workbook.is_empty(), is_open) {
            (true, _) => Json::Null,
            (false, true) => json!(format!("Code of {workbook}  ·  open")),
            (false, false) => json!(format!("Code of {workbook}")),
        },
        "says": match (rebuilt_says(&project["rebuilt"]), text(project, "description")) {
            (Some(rebuilt), _) => rebuilt,
            (None, description) => description.to_string(),
        },
    })
}

/// A development kit: what it is, where it stands, what it can't do yet, and its projects.
/// What only one kit has (the Python that runs scripts, the showcase) is said for that kit
/// and left out for the others.
pub(super) fn kit_pane(app: &SheetApp, ctx: &egui::Context, kit: &Kit) -> Json {
    let head = json!({
        "kind": "kit",
        "id": kit.id,
        "title": kit.name,
        "above": [TOP, KITS],
        "aboveGo": [Pane::Overview.code(), Pane::Overview.code()],
        "madeOf": kit.made_of,
        "badges": badges(kit),
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
    // Its projects, as one list: what each is for is said beside its name.
    let rows: Vec<Json> = of_kit(&found, kit).map(|project| project_row(project, open.as_deref())).collect();
    let mut told = head;
    for (key, value) in [
        ("available", json!(true)),
        ("projectsTitle", json!("Projects")),
        ("projectsSays", json!(projects_says(&found, kit))),
        ("projectsNote", json!(projects_note(kit))),
        ("projects", json!(rows)),
        ("projectsEmpty", json!(NONE_YET)),
        // The space it uses, and what that is made of.
        (
            "space",
            json!(space().map(|space| json!({
                "title": "Space used",
                "says": space_says(&space, kit),
                "counts": if kit.builds { SPACE_COUNTS_BUILT } else { SPACE_COUNTS },
            }))),
        ),
        // A new project can be the code of the workbook that is open, or for any workbook.
        ("newFor", json!(open)),
        ("folder", json!(found.folder)),
        ("canOpenFolder", json!(app.services.open_url.is_some())),
        ("console", json!(console_says())),
    ] {
        told[key] = value;
    }
    // A kit from a package: whose it is, what it stands on, and the way to remove it.
    if !kit.built_in {
        let base = kit::named(kit.base).unwrap_or(&kit::PYTHONCRAFT);
        let projects = of_kit(&found, kit).count();
        told["package"] = json!({
            "title": "Package",
            "version": kit.version,
            "author": kit.author,
            "says": format!("Loaded from a package. It runs nothing of its own: its projects run by {}, with that kit's limits.", base.called()),
            "base": base.name,
            "baseGo": Pane::kit(base).code(),
            "removeSays": match projects {
                0 => "Removing it takes its own folder away, and nothing else.".to_string(),
                count => format!("Removing it leaves its {count} project{} where {} and unusable until it is loaded again.", plural(count), if count == 1 { "it is," } else { "they are," }),
            },
        });
    }
    // The Python that runs a project's scripts: for a kit whose projects have scripts.
    if kit.scripts {
        let python = settings::load(&at.settings).python;
        told["python"] = json!(format!("Python: {}.", if python.is_empty() { "found automatically" } else { python.as_str() }));
        // The field it is named in, where the window houses that.
        told["pythonField"] = json!({
            "title": "Python",
            "named": python,
            "hint": "Found automatically",
            "says": "The Python that runs projects' scripts: a path or a command. Leave empty to let GridCraft find one.",
        });
    }
    // The compiler that builds: for a kit whose projects are built. The person names it here.
    if kit.builds {
        let named = settings::load(&at.settings).compiler;
        told["compiler"] = json!({
            "title": "C compiler",
            "says": if named.is_empty() { COMPILER_FOUND } else { named.as_str() },
            "named": named,
            "hint": COMPILER_HINT,
            "canPick": has_compiler_picker(),
        });
    }
    // The showcase is a project of the first kit.
    if kit.id == kit::PYTHONCRAFT.id {
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

/// Said of a network that is turned on or off, while this GridCraft carries nothing yet.
pub(super) const NETWORK_NOT_YET: &str = "This GridCraft keeps your decision and carries nothing yet: sending and receiving come in a later version.";
const NETWORK_IS_ON: &str = "Send and receive over the network, through GridCraft, to the addresses it names. You turned this on";
const NETWORK_READS: &str = "It may see what is in your open workbooks: with the network on, it could send that to these addresses.";
const NETWORK_LOCAL: &str = "One of these is on your own machine or network: it could reach what is behind your router.";
const LOCAL_NOTE: &str = "on your own machine or network";

/// What stands against turning a project's network on, in sentences: it may read the
/// workbook, and an address is the person's own.
fn network_warnings(project: &Json) -> Vec<&'static str> {
    let reads = project["asks"].as_array().is_some_and(|asks| asks.iter().any(|ask| ask["permission"] == "read"));
    let local = project["network"]["addresses"].as_array().is_some_and(|all| all.iter().any(|address| address["local"] == true));
    [(reads, NETWORK_READS), (local, NETWORK_LOCAL)].into_iter().filter_map(|(so, sentence)| so.then_some(sentence)).collect()
}

/// The network a project would reach, as its part says it: the addresses, whether it is on,
/// and what stands against it. `Null` for a project that names no addresses.
fn network_says(project: &Json, status: &str) -> Json {
    let Some(addresses) = project["network"]["addresses"].as_array() else { return Json::Null };
    let on = project["network"]["on"] == true;
    json!({
        "title": "Network",
        "says": "It asks to send and receive over the network, through GridCraft, to these addresses and no others.",
        "addresses": addresses.iter().map(|address| json!({"text": address["text"], "note": if address["local"] == true { LOCAL_NOTE } else { "" }})).collect::<Vec<_>>(),
        "on": on,
        "state": if on { "On, for these addresses." } else { "Off. Trusting it does not turn this on: you do, here." },
        "warnings": network_warnings(project),
        "canTurnOn": status == "trusted" && !on,
        "notYet": NETWORK_NOT_YET,
    })
}

/// What the operating system asks before a project's network is turned on: where to, in full,
/// and what stands against it.
pub(super) fn network_question(project: &Json) -> String {
    let mut asked = format!("Turn the network on for {}?\n\nIt will be able to send and receive, through GridCraft, to:\n", text(project, "name"));
    for address in project["network"]["addresses"].as_array().map(Vec::as_slice).unwrap_or_default() {
        let note = if address["local"] == true { format!("  ({LOCAL_NOTE})") } else { String::new() };
        asked.push_str(&format!("    {}{note}\n", text(address, "text")));
    }
    for sentence in network_warnings(project) {
        asked.push_str(&format!("\n{sentence}\n"));
    }
    asked.push_str("\nIt stays on until you turn it off, trust the project again, or the addresses it names change.");
    asked
}

/// Turns a project's network on, for the addresses it names now, or off. On is the person's:
/// it ends in the operating system's own question, and the project is read again after the
/// answer, so that what is turned on is what was asked about.
pub(super) fn set_network(app: &SheetApp, id: &str, on: bool) -> Result<String, String> {
    let at = projects::places().ok_or(projects::UNAVAILABLE)?;
    if !manifest::valid_id(id) {
        return Err("That is not a project.".into());
    }
    let read = || {
        let found = project::load(&at.projects.join(id));
        let store = TrustStore::load(&at.trust);
        (describe(&found, &store), found, store)
    };
    let (said, found, mut store) = read();
    if !on {
        store.set_network(&found, false)?;
        return Ok(format!("{}'s network is off.", found.name()));
    }
    if said["network"].is_null() {
        return Err(format!("{} asks for no network.", found.name()));
    }
    if said["status"] != "trusted" {
        return Err(format!("{} is not trusted as it is now, so its network stays off.", found.name()));
    }
    let Some(confirm) = &app.services.confirm else {
        return Err("This copy of GridCraft can't ask your operating system to confirm, so the network stays off.".into());
    };
    if !confirm("Turn this project's network on?", &network_question(&said)) {
        return Ok(format!("{}'s network stays off.", found.name()));
    }
    let (after, again, mut store) = read();
    if again.fingerprint != found.fingerprint || after["network"] != said["network"] {
        return Err(format!("{} changed while you were deciding: look at it again.", found.name()));
    }
    store.set_network(&again, true)?;
    Ok(format!("{}'s network is on, for the addresses it names. {NETWORK_NOT_YET}", again.name()))
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
        (ProjectKind::General, _) => "It is for any workbook.".to_string(),
        (ProjectKind::WorkbookCode, true) => format!("It says it is the code of the workbook {workbook:?}, which is open now."),
        (ProjectKind::WorkbookCode, false) => format!("It says it is the code of the workbook {workbook:?}, which is not the one that is open."),
    };
    let asks: Vec<Json> = project["asks"]
        .as_array()
        .map(|all| {
            // A network that was turned on is said to be on: the sentence it is asked with says "off until".
            let on = |ask: &Json| ask["permission"] == "network" && project["network"]["on"] == true;
            all.iter()
                .map(|ask| json!({"text": if on(ask) { json!(NETWORK_IS_ON) } else { ask["means"].clone() }, "warn": ask["unlimited"]}))
                .collect()
        })
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
        "aboveGo": [Pane::Overview.code(), Pane::Overview.code(), Pane::kit(kit).code()],
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
        "network": network_says(project, status),
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
            let shown = highlight::show(
                kit_named(on_disk.manifest.as_ref().map_or("", |manifest| manifest.kit.as_str())).unwrap_or(&kit::PYTHONCRAFT),
                path,
                &text,
            );
            let lines: Vec<Vec<Json>> =
                shown.lines.iter().map(|line| line.iter().map(|(piece, kind)| json!([piece, kind.map(highlight::Kind::code)])).collect()).collect();
            let says = match shown.language {
                Some(language) => format!("{path}  ·  {language}  ·  {} line{}", shown.of_lines, plural(shown.of_lines)),
                None => format!("{path}  ·  {} line{}", shown.of_lines, plural(shown.of_lines)),
            };
            json!({
                "path": path,
                "language": shown.language,
                // The project it was read from, as it is now: whoever shows the file beside a
                // fingerprint the person is deciding on can tell when the two are not the same.
                "fingerprint": on_disk.fingerprint,
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

/// What can be called from the immediate line now: each script of each project the person
/// trusts and that may run its scripts, named as a call names it.
pub(super) fn callable() -> Vec<String> {
    let Some(found) = found() else { return vec![] };
    found
        .projects
        .iter()
        .flat_map(|project| {
            let id = text(project, "id").to_string();
            projects::scripts(project).into_iter().map(move |script| format!("{id}.{}", super::immediate::function_of(&script)))
        })
        .take(60)
        .collect()
}

/// The project a call names, and the script that is the function it names: for a project the
/// person trusts and that may run its scripts. Otherwise why not, in words for the person.
pub(super) fn resolve(call: &super::immediate::Call) -> Result<(String, String), String> {
    let found = found().ok_or(projects::UNAVAILABLE)?;
    let Some(project) = found.projects.iter().find(|project| super::immediate::names(&call.project, text(project, "id"))) else {
        return Err(format!("There is no project named {}.", call.project));
    };
    let (id, name) = (text(project, "id"), text(project, "name"));
    if let Some(kit) = &call.kit
        && !super::immediate::names(kit, text(project, "kit"))
    {
        return Err(format!("{name} is not a project of a kit named {kit}: it is of {}.", kit_of(project).called()));
    }
    if project["status"] != "trusted" {
        return Err(format!("{name} is not trusted as it is now, so nothing of it can be called."));
    }
    let scripts = projects::scripts(project);
    if scripts.is_empty() {
        return Err(format!("{name} has nothing to call: it may not run scripts, or has none."));
    }
    let Some(script) = scripts.iter().find(|script| super::immediate::function_of(script) == call.function || **script == call.function) else {
        let have: Vec<&str> = scripts.iter().map(|script| super::immediate::function_of(script)).collect();
        return Err(format!("{name} has no function named {}. It has: {}.", call.function, have.join(", ")));
    };
    Ok((id.to_string(), script.clone()))
}

/// A project's fingerprint as it is on disk now. `None` when there is no such project, or it
/// can't be read.
pub(super) fn fingerprint_of(id: &str) -> Option<String> {
    found()?.on_disk.iter().find(|on_disk| on_disk.id == id)?.fingerprint.clone()
}

/// Puts into a project's part what is decided there by the window that houses it: trusting
/// it, and running its scripts. `reviewing` is the project and the fingerprint it had when the
/// person began to look at its part; `run` is the script that was run from a part, if one was.
pub(super) fn with_trust(told: &mut Json, app: &SheetApp, reviewing: Option<&(String, String)>, run: &Json) {
    if told["kind"] != "project" {
        return;
    }
    let Some(found) = found() else { return };
    let id = text(told, "id").to_string();
    let Some(project) = found.projects.iter().find(|project| project["id"] == id.as_str()) else { return };
    let pinned = reviewing.filter(|(of, _)| *of == id).map(|(_, print)| print.as_str());
    told["trust"] = projects::trust_says(app, project, pinned);
    told["scripts"] = json!(projects::scripts(project));
    told["run"] = if run["project"] == id.as_str() { run.clone() } else { Json::Null };
}

/// The part of the window `pane` names, as it is now. A project that is no longer there
/// gives its kit's part.
pub(super) fn pane(app: &SheetApp, ctx: &egui::Context, pane: &Pane, now: &agents::Snapshot) -> Json {
    match pane {
        Pane::Overview => overview(app, now),
        // The server's part is said by whoever draws it: what waits there is decided on.
        Pane::Server => json!({"kind": "server", "title": SERVER, "above": [TOP], "aboveGo": [Pane::Overview.code()]}),
        Pane::Console => super::console::pane(),
        // Which module is read is the window's own to say: whoever draws it names it.
        Pane::Macros => macros::pane(app, &mut String::new()),
        Pane::Kit(id) => kit_pane(app, ctx, kit_named(id).unwrap_or(&kit::PYTHONCRAFT)),
        Pane::Project(id) => project_pane(app, ctx, id).unwrap_or_else(|| kit_pane(app, ctx, &kit::PYTHONCRAFT)),
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
        for pane in [
            Pane::Overview,
            Pane::Server,
            Pane::Console,
            Pane::Macros,
            Pane::kit(&kit::PYTHONCRAFT),
            Pane::kit(&kit::C),
            Pane::Project("clean-data".into()),
        ] {
            assert_eq!(Pane::parse(&pane.code()), Some(pane.clone()), "{pane:?}");
        }
        assert_eq!((Pane::kit(&kit::PYTHONCRAFT).code(), Pane::kit(&kit::C).code()), ("kit:pythoncraft".to_string(), "kit:c".to_string()));
        assert_eq!(Pane::first_kit(), Pane::kit(&kit::PYTHONCRAFT));
        // Only a kit that is loaded has a part.
        for bad in ["", "tools", "project:", "project:../x", "project:Clean Data", "kit", "kit:", "kit:other", "kit:C", "kit:anie/x"] {
            assert_eq!(Pane::parse(bad), None, "{bad:?}");
        }
    }

    #[test]
    fn the_list_has_the_server_at_the_top_and_stops_at_the_kits() {
        let s = Sandbox::new("kits-nav");
        projects::create("Clean Data").unwrap();
        let mut a = app_with(None);
        let title = a.session.active().unwrap().title.clone();
        budget(&s, &title);
        // Two projects of the C kit: one for any workbook, and one that names a workbook.
        projects::create_project_in(&kit::C, "Blink", None).unwrap();
        write(&s, "doom/project.toml", "id = \"doom\"\nname = \"Doom\"\nversion = \"1\"\nkit = \"c\"\nworkbook = \"Doom.xlsx\"\n");
        write(&s, "doom/src/main.c", "int main(void) { return 0; }\n");
        forget();
        let rows = nav(&a, &Pane::kit(&kit::PYTHONCRAFT), &agents::snapshot());
        let listed: Vec<(String, String, u64)> = rows
            .as_array()
            .unwrap()
            .iter()
            .map(|row| (text(row, "t").to_string(), text(row, "label").to_string(), row["depth"].as_u64().unwrap()))
            .collect();
        // No project is listed down the side: a kit's are in the kit's own part.
        let expected = [
            ("item", "Development", 0),
            ("item", "MCP Server", 1),
            // The Console and the open workbook's macros are no kit's: they stand with the server.
            ("item", "Console", 1),
            ("item", "Macros", 1),
            ("heading", "Development Kits", 1),
            ("kit", "PythonCraft", 1),
            ("kit", "C Development Kit", 1),
        ];
        assert_eq!(listed, expected.map(|(kind, label, depth)| (kind.to_string(), label.to_string(), depth)));
        let row = |rows: &Json, label: &str| rows.as_array().unwrap().iter().find(|row| row["label"] == label).unwrap().clone();
        assert_eq!((&row(&rows, "PythonCraft")["selected"], &row(&rows, "Development")["selected"]), (&json!(true), &json!(false)));
        // A kit is set apart with its shield, and says what it is made of and how many projects it has.
        let first = row(&rows, "PythonCraft");
        assert_eq!(
            (&first["hint"], &first["icon"], &first["projects"], &first["within"]),
            (&json!("Python · Svelte"), &json!("shield"), &json!("2 projects"), &json!(false))
        );
        let second = row(&rows, "C Development Kit");
        assert_eq!(
            (&second["go"], &second["hint"], &second["selected"], &second["icon"], &second["projects"]),
            (&json!("kit:c"), &json!("C"), &json!(false), &json!("shield"), &json!("2 projects"))
        );
        assert_eq!(row(&rows, "Development")["icon"], "tools");
        assert_eq!(
            (&row(&rows, "MCP Server")["go"], &row(&rows, "MCP Server")["hint"], &row(&rows, "MCP Server")["mark"]),
            (&json!("server"), &json!("Off"), &json!("off"))
        );
        // While a project is shown its kit's row says so: it is the way back to the kit.
        let rows = nav(&a, &Pane::Project("doom".into()), &agents::snapshot());
        assert_eq!(
            (&row(&rows, "C Development Kit")["within"], &row(&rows, "C Development Kit")["selected"], &row(&rows, "PythonCraft")["within"]),
            (&json!(true), &json!(false), &json!(false))
        );
        assert_eq!(rows.as_array().map(Vec::len), Some(7), "the project itself is not a row");
        assert_eq!((&row(&rows, "Console")["go"], &row(&rows, "Macros")["go"]), (&json!("console"), &json!("macros")));
        assert_eq!((&row(&rows, "Console")["hint"], &row(&rows, "Macros")["hint"]), (&json!("0 entries"), &json!("None in this workbook")));
        // The kit's own part says what each of its projects is for, beside its name.
        let ctx = egui::Context::default();
        let of = |told: &Json, name: &str| told["projects"].as_array().unwrap().iter().find(|project| project["name"] == name).unwrap().clone();
        let c_kit = kit_pane(&a, &ctx, &kit::C);
        assert_eq!((&of(&c_kit, "Doom")["for"], &of(&c_kit, "Blink")["for"]), (&json!("Code of Doom.xlsx"), &Json::Null));
        assert_eq!(of(&kit_pane(&a, &ctx, &kit::PYTHONCRAFT), "Budget Macros")["for"], json!(format!("Code of {title}  ·  open")));
        budget(&s, "Elsewhere.xlsm");
        forget();
        assert_eq!(of(&kit_pane(&a, &ctx, &kit::PYTHONCRAFT), "Budget Macros")["for"], "Code of Elsewhere.xlsm");
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
            (&json!("Off"), &json!("PythonCraft"), &json!("2 projects, 0 trusted"))
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
    fn the_kits_say_how_many_they_are_the_space_they_use_and_what_a_search_looks_through() {
        let s = Sandbox::new("kits-space");
        projects::create("Clean Data").unwrap();
        projects::create_project_in(&kit::C, "Blink", None).unwrap();
        forget();
        let a = app_with(None);
        let ctx = egui::Context::default();
        let rows = nav(&a, &Pane::Overview, &agents::snapshot());
        // Over the kits: how many, the space they use, and a field to filter them by.
        let heading = rows.as_array().unwrap().iter().find(|row| row["t"] == "heading").unwrap().clone();
        assert_eq!((&heading["label"], &heading["filter"], &heading["none"]), (&json!(KITS), &json!(FILTER_HINT), &json!(FILTER_NONE)));
        let says = heading["says"].as_str().unwrap();
        assert!(says.starts_with("2 kits  ·  ") && says.ends_with(" KB"), "{says}");
        // What a search looks through: what the kit says of itself, and its projects.
        let kit_row =
            |label: &str| rows.as_array().unwrap().iter().find(|row| row["label"] == label).unwrap()["search"].as_str().unwrap().to_string();
        let (first, second) = (kit_row("PythonCraft"), kit_row("C Development Kit"));
        assert!(matches(&first, "python") && matches(&first, "Clean DATA") && matches(&first, "svelte panel"));
        assert!(!matches(&first, "blink") && !matches(&first, "python webassembly"), "every word typed must be said");
        assert!(matches(&second, "blink") && matches(&second, "webassembly") && !matches(&second, "python"));
        assert!(matches(&first, "") && matches(&second, "   "), "nothing typed shows every kit");
        // The space: each kit's projects' files, and for the kit that builds what was built too.
        let on_disk = |id: &str| folder_bytes(&s.0.join("projects").join(id), &mut usize::MAX.clone());
        let measured = space().unwrap();
        assert!(on_disk("clean-data") > 0 && on_disk("blink") > 0);
        assert_eq!((measured.of(&kit::PYTHONCRAFT), measured.of(&kit::C)), (on_disk("clean-data"), on_disk("blink")));
        std::fs::create_dir_all(s.0.join("built/blink/print")).unwrap();
        std::fs::write(s.0.join("built/blink/print/program.wasm"), vec![0u8; 5000]).unwrap();
        forget();
        let measured = space().unwrap();
        assert_eq!(measured.of(&kit::C), on_disk("blink") + 5000);
        assert_eq!(measured.of(&kit::PYTHONCRAFT), on_disk("clean-data"), "what was built is not the first kit's");
        assert_eq!(measured.all(), measured.of(&kit::PYTHONCRAFT) + measured.of(&kit::C));
        assert!(!measured.cut);
        // In words, and where it is said.
        assert_eq!(
            [bytes_say(0), bytes_say(1), bytes_say(1024), bytes_say(1025), bytes_say(1536 * 1024), bytes_say(3 * 1024 * 1024 * 1024)],
            ["0 KB", "1 KB", "1 KB", "2 KB", "1.5 MB", "3.0 GB"].map(str::to_string)
        );
        let told = kit_pane(&a, &ctx, &kit::C);
        assert_eq!((&told["space"]["title"], &told["space"]["counts"]), (&json!("Space used"), &json!(SPACE_COUNTS_BUILT)));
        assert_eq!(told["space"]["says"], json!(bytes_say(on_disk("blink") + 5000)));
        assert_eq!(kit_pane(&a, &ctx, &kit::PYTHONCRAFT)["space"]["counts"], SPACE_COUNTS);
        let top = overview(&a, &agents::snapshot());
        assert!(top["kitsSummary"].as_str().unwrap().starts_with("2 kits  ·  "), "{}", top["kitsSummary"]);
        assert_eq!((&top["kits"][1]["space"], &top["kitsNone"]), (&told["space"]["says"], &json!(FILTER_NONE)));
        assert!(top["kits"][1]["search"].as_str().unwrap().contains("blink"));
        // A measure that has to stop early stops, and says it did not see everything.
        let mut left = 1;
        folder_bytes(&s.0.join("projects"), &mut left);
        assert_eq!(left, 0);
    }

    #[test]
    fn the_kit_lists_its_projects_and_a_project_says_what_it_is_for() {
        let s = Sandbox::new("kits-panes");
        projects::create("Clean Data").unwrap();
        let a = app_with(None);
        let ctx = egui::Context::default();
        let title = a.session.active().unwrap().title.clone();
        budget(&s, &title);
        let told = kit_pane(&a, &ctx, &kit::PYTHONCRAFT);
        assert_eq!(
            (&told["title"], &told["madeOf"], &told["projectsSays"]),
            (&json!("PythonCraft"), &json!(["Python", "Svelte"]), &json!("2 projects, 0 trusted"))
        );
        assert_eq!((&told["id"], &told["notYet"]), (&json!("pythoncraft"), &Json::Null));
        // What it is made of, each with its badge.
        assert_eq!(told["badges"], json!([{"label": "Python", "code": "python"}, {"label": "Svelte", "code": "svelte"}]));
        assert_eq!(kit_pane(&a, &ctx, &kit::C)["badges"], json!([{"label": "C", "code": "c"}]));
        assert_eq!(overview(&a, &agents::snapshot())["kits"][1]["badges"], json!([{"label": "C", "code": "c"}]));
        assert_eq!(
            told["projectsNote"],
            "A project is a folder with a project.toml, Python scripts and a web panel. It does nothing until you trust it."
        );
        assert!(told["python"].is_string() && told["showcase"].is_object(), "what the first kit has of its own");
        assert_eq!(told["newFor"], json!(title));
        // One list, under one word: Projects. Nothing in the kit's part calls one a library.
        assert_eq!((&told["projectsTitle"], &told["projectsEmpty"], &told["groups"]), (&json!("Projects"), &json!(NONE_YET), &Json::Null));
        assert!(!told.to_string().to_lowercase().contains("librar"), "{told}");
        let listed = told["projects"].as_array().unwrap();
        assert_eq!(listed.len(), 2);
        let code = &listed[0];
        assert_eq!(
            (&code["name"], &code["go"], &code["open"], &code["statusLabel"]),
            (&json!("Budget Macros"), &json!("project:budget"), &json!(true), &json!("Not trusted"))
        );
        assert_eq!(code["says"], "Of the workbook's macros it says: 1 rebuilt, 0 in part, 1 left as they are.");
        assert_eq!((&listed[1]["name"], &listed[1]["for"]), (&json!("Clean Data"), &Json::Null));
        assert_eq!((&told["above"], &told["aboveGo"]), (&json!(["Development", "Development Kits"]), &json!(["overview", "overview"])));
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
        let general = project_pane(&a, &ctx, "clean-data").unwrap();
        assert_eq!(
            (&general["kindLabel"], &general["forSays"], &general["rebuilt"]),
            (&json!("For any workbook"), &json!("It is for any workbook."), &Json::Null)
        );
        assert_eq!((&general["kit"], &general["kitGo"], &general["notYet"]), (&json!("PythonCraft"), &json!("kit:pythoncraft"), &Json::Null));
        assert_eq!(general["asksNothing"], "Show its panel, and nothing more.");
        assert!(!general.to_string().to_lowercase().contains("librar"), "{general}");
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
        assert_eq!((&empty["projects"], &empty["projectsEmpty"]), (&json!([]), &json!(NONE_YET)), "the first kit's are not listed here");
        // What only the first kit has is not said of this one.
        assert_eq!((&empty["python"], &empty["showcase"]), (&Json::Null, &Json::Null));
        // It builds: the compiler is named here, or looked for. The first kit has none to name.
        assert_eq!(empty["compiler"], json!({"title": "C compiler", "says": COMPILER_FOUND, "named": "", "hint": COMPILER_HINT, "canPick": false}));
        assert_eq!(kit_pane(&a, &ctx, &kit::PYTHONCRAFT)["compiler"], Json::Null);
        std::fs::write(s.0.join("developer.json"), r#"{"compiler": "C:\\tools\\zig\\zig.exe"}"#).unwrap();
        let named = kit_pane(&a, &ctx, &kit::C);
        assert_eq!((&named["compiler"]["says"], &named["compiler"]["named"]), (&json!("C:\\tools\\zig\\zig.exe"), &json!("C:\\tools\\zig\\zig.exe")));
        std::fs::remove_file(s.0.join("developer.json")).unwrap();
        // A project made in it is its own, and not the first kit's.
        assert_eq!(projects::create_project_in(&kit::C, "Blink", None).unwrap(), "Blink was created. It is not trusted yet.");
        forget();
        let told = kit_pane(&a, &ctx, &kit::C);
        assert_eq!((&told["projectsSays"], &told["projects"][0]["name"]), (&json!("1 project, 0 trusted"), &json!("Blink")));
        assert_eq!(told["projects"][0]["go"], "project:blink");
        let first = kit_pane(&a, &ctx, &kit::PYTHONCRAFT);
        assert_eq!((&first["projectsSays"], &first["projects"].as_array().map(Vec::len)), (&json!("1 project, 0 trusted"), &Some(1)));
        assert_eq!(pane(&a, &ctx, &Pane::kit(&kit::C), &agents::snapshot())["title"], "C Development Kit");
        // The project says which kit it is of.
        let one = project_pane(&a, &ctx, "blink").unwrap();
        assert_eq!(
            (&one["title"], &one["kit"], &one["kitGo"], &one["kindLabel"], &one["statusLabel"]),
            (&json!("Blink"), &json!("C Development Kit"), &json!("kit:c"), &json!("For any workbook"), &json!("Not trusted"))
        );
        assert_eq!(one["above"], json!(["Development", "Development Kits", "C Development Kit"]));
        assert_eq!(one["aboveGo"], json!(["overview", "overview", "kit:c"]), "each of them is a way back");
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
        assert_eq!((&rusty["kit"], &rusty["statusLabel"]), (&json!("PythonCraft"), &json!("Can't be used")));
        assert_eq!(rusty["problem"], "it is a project of the kit \"rust\", which this GridCraft doesn't have: it is not loaded");
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
        assert_eq!(rows.as_array().unwrap().len(), 7, "the top, the server, the console, the macros, the heading and the two kits");
        assert_eq!(overview(&a, &agents::snapshot())["kits"][0]["projects"], Json::Null);
    }
}
