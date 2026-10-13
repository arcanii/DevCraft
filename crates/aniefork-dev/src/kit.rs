//! Development kits: the ways of building on GridCraft that are loaded into it.
//!
//! A kit says what a project is made of and how it is run. This build has two:
//!
//! - **PythonCraft**: a project is a folder with a `project.toml`, Python scripts
//!   and a web panel (written with Svelte or any other web toolkit). Most of this crate is that
//!   kit at work: its scripts ([`crate::python`]) and panels ([`crate::bridge`]).
//! - the **C Development Kit**: a project is a folder with a `project.toml`, C sources and a
//!   web panel. It builds a project's C as one WebAssembly program with the person's own
//!   compiler ([`crate::c`]), and the project's panel runs it: the bridge serves the program
//!   beside the page, with the loader that starts it ([`crate::c_program`]). The program can
//!   reach nothing but what its page hands it (the owner's decision, 2026-10-13: WebAssembly
//!   first; ADR-0001 has the phases).
//!
//! What every kit's projects share is not a kit's: a project on disk and its fingerprint
//! ([`crate::project`]), the person's trust in it ([`crate::trust`]), what it asks to be
//! allowed ([`crate::manifest`]), and the Development server ([`crate::agents`]).
//!
//! A project says which kit it is of with `kit = "…"` in its `project.toml`. One that says
//! nothing is PythonCraft's, as every project was before there were two. A kit's projects may
//! hold only what the kit knows how to serve and run ([`check`]): a C project has no scripts
//! and can't ask for `python` or `network`, so nothing of PythonCraft that can't be limited
//! can be made to run for it.
//!
//! **The first kit is named PythonCraft** (the owner, 2026-10-14: "Let's name our first DevKit :
//! PythonCraft - which is the Python / Svelte UI"). Until then it was the Anie Development Kit,
//! with the id `anie`: a `project.toml` or a kit's package that names it so is still read
//! ([`manifest::kit_id`]), so nothing that was made has to be changed, or trusted again.
//!
//! Both kits are compiled into GridCraft. **A further kit is loaded from a package file**
//! ([`crate::kitpack`]; the owner's decision, 2026-10-13): it describes, and runs nothing of
//! its own. Its projects are projects of one of the two built-in kits (its `base`), with that
//! kit's limits; what it brings is its words and a starter. Such a kit is kept in a folder
//! of its own beside the projects folder (`kits/<id>/kit.toml`), and found there: so which
//! kits there are is asked for a projects folder ([`all`], [`named_in`], [`of_in`],
//! [`check_in`]), and [`loaded`] is the two that are built in.
//!
//! **A project** is for one of two things, by what its `project.toml` says: it is the code
//! of one workbook, which it names, or it is for any workbook. That only says what the
//! project is for: nothing is allowed or run because of it. (Until 2026-10-13 a project was
//! called a module, and its manifest `module.toml`: what was made then is still read.)
//!
//! **A project is not called a library.** The word is kept for a project that other projects,
//! or a spreadsheet, can refer to. None can yet: nothing in a `project.toml` says so and
//! nothing is built for it, so no window and no report uses the word (the owner, 2026-10-13:
//! "We interchange Project and Library which is confusing").

use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};

use crate::highlight::{self, Language};
use crate::manifest::{self, Manifest, Permission};

/// A development kit, as it is listed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kit {
    /// Its name among the kits, and the word a `project.toml` names it by: lowercase letters,
    /// digits and hyphens.
    pub id: &'static str,
    pub name: &'static str,
    /// What its projects are written in.
    pub made_of: &'static [&'static str],
    /// What it is, in a sentence or two.
    pub says: &'static str,
    /// What one of its projects is, in a sentence.
    pub project_is: &'static str,
    /// What it can't do yet. Empty for a kit that is whole.
    pub not_yet: &'static str,
    /// It is compiled into GridCraft.
    pub built_in: bool,
    /// The built-in kit its projects are projects of: itself, for a kit that is built in.
    pub base: &'static str,
    /// Its version and who made it, as a kit from a package says them. Empty for the built-in.
    pub version: &'static str,
    pub author: &'static str,
    /// What its projects may ask to be allowed.
    pub permissions: &'static [Permission],
    /// Its projects may have a panel (`[panel]` in the manifest).
    pub panels: bool,
    /// Its projects may have Python scripts (`[python]` in the manifest).
    pub scripts: bool,
    /// Its projects are built, from a recipe (`[c]` in the manifest).
    pub builds: bool,
    /// How the files its projects hold are read for showing: the languages it describes
    /// ([`crate::highlight`]). A file of another kind is shown plain.
    pub languages: &'static [&'static Language],
}

/// PythonCraft, the kit there was before there were two: Python scripts and a web panel.
pub const PYTHONCRAFT: Kit = Kit {
    id: manifest::DEFAULT_KIT,
    name: "PythonCraft",
    made_of: &["Python", "Svelte"],
    says: "Projects made of Python scripts and a web panel, which can be a Svelte app or any other page. A project does nothing until you trust it.",
    project_is: "A project is a folder with a project.toml, Python scripts and a web panel.",
    not_yet: "",
    built_in: true,
    base: manifest::DEFAULT_KIT,
    version: "",
    author: "",
    permissions: &Permission::ALL,
    panels: true,
    scripts: true,
    builds: false,
    languages: &[
        &highlight::PYTHON,
        &highlight::MARKUP,
        &highlight::JAVASCRIPT,
        &highlight::CSS,
        &highlight::TOML,
        &highlight::JSON,
        &highlight::MARKDOWN,
    ],
};

/// Programs written in C: built as WebAssembly, and run in the project's panel.
pub const C: Kit = Kit {
    id: "c",
    name: "C Development Kit",
    made_of: &["C"],
    says: "Projects written in C, built as WebAssembly by a compiler you name and run in the project's panel, where the program can do only what its page hands it. A project does nothing until you trust it.",
    project_is: "A project is a folder with a project.toml, C sources and a web panel that runs the program built from them.",
    // What is left (an agent asking for a build) is not the person's concern.
    not_yet: "",
    built_in: true,
    base: "c",
    version: "",
    author: "",
    // What a project's page can ask of the spreadsheet for the program it runs. The program
    // is WebAssembly: it reaches nothing but what that page hands it, so running it needs no
    // leave of its own, as `python` does.
    permissions: &[Permission::Read, Permission::Write, Permission::Files],
    panels: true,
    scripts: false,
    builds: true,
    // Its own language, and those of the page that runs the program.
    languages: &[
        &highlight::C,
        &highlight::MARKUP,
        &highlight::JAVASCRIPT,
        &highlight::CSS,
        &highlight::TOML,
        &highlight::JSON,
        &highlight::MARKDOWN,
    ],
};

impl Kit {
    /// Its name as it stands in a sentence: "the C Development Kit", but "PythonCraft", which
    /// is a name of its own.
    pub fn called(&self) -> String {
        if self.name.ends_with(" Kit") { format!("the {}", self.name) } else { self.name.to_string() }
    }
}

/// The kits GridCraft is built with, in the order they are listed.
pub fn loaded() -> &'static [Kit] {
    &[PYTHONCRAFT, C]
}

/// The file in which a kit from a package says what it is, the most kits there can be, and
/// the largest such a file may be.
pub const KIT_FILE: &str = "kit.toml";
pub const MAX_KITS: usize = 64;
const MAX_KIT_FILE: u64 = 64 * 1024;

/// What a kit from a package says of itself, in its `kit.toml`. Anything else said there is
/// refused: a word this GridCraft does not know is not passed over.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Described {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub author: String,
    /// What it is, in a sentence or two.
    #[serde(default)]
    pub description: String,
    /// What its projects are written in: names, shown as badges.
    #[serde(default)]
    pub made_of: Vec<String>,
    /// The built-in kit its projects are projects of.
    pub base: String,
    /// What one of its projects is, in a sentence.
    #[serde(default)]
    pub project_is: String,
}

/// Whether some words of a kit's can be shown: nothing in them that is not text, and not too many.
fn words(text: &str, most: usize, what: &str, needed: bool) -> Result<(), String> {
    if text.chars().any(char::is_control) {
        return Err(format!("its {what} holds characters that can't be shown"));
    }
    if text.chars().count() > most {
        return Err(format!("its {what} is longer than {most} characters"));
    }
    if needed && text.trim().is_empty() {
        return Err(format!("it has no {what}"));
    }
    Ok(())
}

/// Reads what a kit says of itself, and checks that it is a kit this GridCraft can list: an id
/// of its own, a name, and a built-in kit it stands on.
pub fn describe(text: &str) -> Result<Described, String> {
    let said: Described = toml::from_str(text).map_err(|e| format!("its {KIT_FILE} can't be read: {}", e.message()))?;
    if !manifest::valid_id(&said.id) {
        return Err(format!("its id, {:?}, is not lowercase letters, digits and hyphens", said.id.chars().take(40).collect::<String>()));
    }
    if named(&said.id).is_some() {
        return Err(format!("its id, {:?}, is the name of a kit GridCraft is built with", said.id));
    }
    let Some(base) = named(&said.base) else {
        return Err(format!("it stands on the kit {:?}, which GridCraft is not built with", said.base.chars().take(40).collect::<String>()));
    };
    debug_assert!(base.built_in);
    words(&said.name, 60, "name", true)?;
    words(&said.version, 30, "version", false)?;
    words(&said.author, 80, "author", false)?;
    words(&said.description, 400, "description", false)?;
    words(&said.project_is, 200, "sentence on what a project is", false)?;
    if said.made_of.len() > 6 {
        return Err("it says it is made of more than six things".into());
    }
    for part in &said.made_of {
        words(part, 24, "list of what it is made of", true)?;
    }
    Ok(said)
}

/// The words of a kit from a package, kept for as long as GridCraft runs: a kit is listed by
/// reference, as the built-in ones are.
fn keep(text: &str) -> &'static str {
    Box::leak(text.trim().to_string().into_boxed_str())
}

/// The kit a description makes: its own words, on the built-in kit it stands on.
fn from_described(said: &Described) -> Option<Kit> {
    let base = named(&said.base)?;
    let made_of: Vec<&'static str> = said.made_of.iter().map(|part| keep(part)).collect();
    Some(Kit {
        id: keep(&said.id),
        name: keep(&said.name),
        made_of: Box::leak(made_of.into_boxed_slice()),
        says: if said.description.trim().is_empty() {
            keep(&format!("A development kit loaded from a package. Its projects run by {}.", base.called()))
        } else {
            keep(&said.description)
        },
        project_is: if said.project_is.trim().is_empty() { base.project_is } else { keep(&said.project_is) },
        not_yet: "",
        built_in: false,
        base: base.id,
        version: keep(&said.version),
        author: keep(&said.author),
        ..*base
    })
}

/// The kits read from packages so far: for which folder, from which text, and the kit. A kit
/// is made once for each text it was read from, and kept.
static PACKAGED: Mutex<Vec<(PathBuf, String, &'static Kit)>> = Mutex::new(Vec::new());

/// Something was loaded or removed: nothing is kept that the folder no longer holds. (The
/// kits are read from their folder each time they are asked for; this is here for whoever
/// changes that.)
pub fn forget() {}

/// The folder kits from packages are kept in, for projects kept in `projects_dir`: beside it.
/// `None` when that is not a projects folder of GridCraft's by its name, so that a folder a
/// test or a tool made for itself is never given kits from beside it.
pub fn folder_beside(projects_dir: &Path) -> Option<PathBuf> {
    let name = projects_dir.file_name()?.to_str()?;
    matches!(name, "projects" | "modules").then(|| projects_dir.with_file_name("kits"))
}

/// The kits loaded from packages that are kept in `kits_dir`, by id. One that can't be read as
/// a kit is left out.
fn packaged(kits_dir: &Path) -> Vec<&'static Kit> {
    let Ok(entries) = std::fs::read_dir(kits_dir) else { return vec![] };
    let mut found: Vec<&'static Kit> = Vec::new();
    for entry in entries.flatten().take(MAX_KITS * 4) {
        if found.len() >= MAX_KITS {
            break;
        }
        let folder = entry.path();
        let Some(id) = folder.file_name().and_then(|name| name.to_str()).map(str::to_string) else { continue };
        if !manifest::valid_id(&id) || named(&id).is_some() {
            continue;
        }
        let file = folder.join(KIT_FILE);
        let Ok(kind) = std::fs::symlink_metadata(&file) else { continue };
        if !kind.is_file() || kind.len() > MAX_KIT_FILE {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&file) else { continue };
        let mut kept = PACKAGED.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some((_, _, kit)) = kept.iter().find(|(dir, of, kit)| dir == kits_dir && *of == text && kit.id == id) {
            found.push(*kit);
            continue;
        }
        // Its folder is named as it names itself: a kit is found by its id.
        let Some(kit) = describe(&text).ok().filter(|said| said.id == id).and_then(|said| from_described(&said)) else { continue };
        let kit: &'static Kit = Box::leak(Box::new(kit));
        kept.push((kits_dir.to_path_buf(), text, kit));
        found.push(kit);
    }
    found.sort_by_key(|kit| kit.id);
    found
}

/// Every kit there is for projects kept in `projects_dir`: the ones GridCraft is built with,
/// then the ones loaded from packages.
pub fn all(projects_dir: &Path) -> Vec<&'static Kit> {
    let mut kits: Vec<&'static Kit> = loaded().iter().collect();
    if let Some(dir) = folder_beside(projects_dir) {
        kits.extend(packaged(&dir));
    }
    kits
}

/// The kit with this id, among those there are for projects kept in `projects_dir`.
pub fn named_in(projects_dir: &Path, id: &str) -> Option<&'static Kit> {
    named(id).or_else(|| all(projects_dir).into_iter().find(|kit| kit.id == id))
}

/// The kit a project kept in `projects_dir` is listed under: the one its manifest names, or
/// the first when it can't be placed.
pub fn of_in(projects_dir: &Path, manifest: Option<&Manifest>) -> &'static Kit {
    manifest.and_then(|manifest| named_in(projects_dir, &manifest.kit)).unwrap_or(&PYTHONCRAFT)
}

/// [`check`], for a project kept in `projects_dir`: its kit can be one loaded from a package.
pub fn check_in(projects_dir: &Path, manifest: &Manifest) -> Result<&'static Kit, String> {
    check_as(named_in(projects_dir, &manifest.kit), manifest)
}

/// The loaded kit with this id. The first is found by the id it had before it was named, too.
pub fn named(id: &str) -> Option<&'static Kit> {
    let id = manifest::kit_id(id);
    loaded().iter().find(|kit| kit.id == id)
}

/// The kit a project is listed under: the one its manifest names. A project that can't be
/// placed (its manifest can't be read, or names a kit this build doesn't have) is listed under
/// the first kit, with what is wrong with it.
pub fn of(manifest: Option<&Manifest>) -> &'static Kit {
    manifest.and_then(|manifest| named(&manifest.kit)).unwrap_or(&PYTHONCRAFT)
}

/// Whether a manifest is one a project of its kit can have: the kit is one this build has, and
/// the project holds and asks for nothing the kit doesn't know. A project that names a kit and
/// then asks for what another kit runs is refused, not guessed at.
pub fn check(manifest: &Manifest) -> Result<&'static Kit, String> {
    check_as(named(&manifest.kit), manifest)
}

fn check_as(kit: Option<&'static Kit>, manifest: &Manifest) -> Result<&'static Kit, String> {
    let Some(kit) = kit else {
        return Err(format!("it is a project of the kit {:?}, which this GridCraft doesn't have: it is not loaded", manifest.kit));
    };
    if let Some(asked) = manifest.permissions.iter().find(|asked| !kit.permissions.contains(asked)) {
        return Err(format!("it asks for {:?}, which a project of {} can't be given", asked.code(), kit.called()));
    }
    if manifest.panel.is_some() && !kit.panels {
        return Err(format!("it has a panel, which a project of {} can't have", kit.called()));
    }
    if manifest.python.is_some() && !kit.scripts {
        return Err(format!("it has Python scripts, which a project of {} can't have", kit.called()));
    }
    if manifest.c.is_some() && !kit.builds {
        return Err(format!("it has a C recipe, which a project of {} can't have", kit.called()));
    }
    Ok(kit)
}

/// What a project is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {
    /// The code of one workbook, which its `project.toml` names.
    WorkbookCode,
    /// For any workbook: it names none.
    General,
}

impl ProjectKind {
    pub const ALL: [ProjectKind; 2] = [ProjectKind::WorkbookCode, ProjectKind::General];

    /// What a project with this manifest is for. One without a manifest that could be read
    /// names no workbook.
    pub fn of(manifest: Option<&Manifest>) -> ProjectKind {
        match manifest.and_then(|manifest| manifest.workbook.as_ref()) {
            Some(_) => ProjectKind::WorkbookCode,
            None => ProjectKind::General,
        }
    }

    /// The word reports use.
    pub fn code(self) -> &'static str {
        match self {
            ProjectKind::WorkbookCode => "workbook",
            ProjectKind::General => "general",
        }
    }

    /// The words people are shown for what a project is for.
    pub fn label(self) -> &'static str {
        match self {
            ProjectKind::WorkbookCode => "Workbook code",
            ProjectKind::General => "For any workbook",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(more: &str) -> Manifest {
        manifest::parse(&format!("id = \"x\"\nname = \"X\"\nversion = \"1\"\n{more}"), "x").unwrap()
    }

    #[test]
    fn the_kits_this_build_has_are_listed_and_say_what_they_are_made_of() {
        let kits = loaded();
        assert_eq!(kits.iter().map(|kit| kit.id).collect::<Vec<_>>(), ["pythoncraft", "c"], "two kits, and no way to load another yet");
        assert_eq!((kits[0].name, kits[0].made_of, kits[0].built_in), ("PythonCraft", ["Python", "Svelte"].as_slice(), true));
        assert_eq!((kits[1].name, kits[1].made_of, kits[1].built_in), ("C Development Kit", ["C"].as_slice(), true));
        for kit in kits {
            assert!(manifest::valid_id(kit.id) && !kit.says.is_empty() && !kit.project_is.is_empty(), "{}", kit.id);
            assert_eq!(named(kit.id), Some(kit));
        }
        assert_eq!(PYTHONCRAFT.id, manifest::DEFAULT_KIT, "a project that names no kit is the first kit's");
        // It is found by the id it had before it was named; and a name stands in a sentence as it reads.
        assert_eq!(named("anie"), Some(&PYTHONCRAFT));
        assert_eq!((PYTHONCRAFT.called(), C.called()), ("PythonCraft".to_string(), "the C Development Kit".to_string()));
        // Both are whole: neither has something to say it can't do yet.
        assert!(PYTHONCRAFT.not_yet.is_empty() && C.not_yet.is_empty());
        assert!(C.says.contains("WebAssembly") && C.says.contains("panel") && C.project_is.contains("panel"));
        assert_eq!((C.builds, PYTHONCRAFT.builds), (true, false), "the second kit builds its projects; the first has nothing to build");
        assert_eq!((C.panels, C.scripts), (true, false), "what it builds runs in the project's panel; it has no scripts");
        assert_eq!((named("rust"), named(""), named("C")), (None, None, None));
    }

    #[test]
    fn a_project_is_of_the_kit_it_names_and_of_the_first_when_it_names_none() {
        assert_eq!(of(Some(&read(""))), &PYTHONCRAFT);
        assert_eq!(of(Some(&read("kit = \"pythoncraft\""))), &PYTHONCRAFT);
        assert_eq!(of(Some(&read("kit = \"anie\""))), &PYTHONCRAFT, "the id it had before it was named");
        assert_eq!(of(Some(&read("kit = \"c\""))), &C);
        // One that can't be placed is listed under the first kit: `check` says what is wrong.
        assert_eq!((of(Some(&read("kit = \"rust\""))), of(None)), (&PYTHONCRAFT, &PYTHONCRAFT));
        assert_eq!(check(&read("")), Ok(&PYTHONCRAFT));
        assert_eq!(check(&read("kit = \"c\"")), Ok(&C));
        assert_eq!(
            check(&read("kit = \"rust\"")).unwrap_err(),
            "it is a project of the kit \"rust\", which this GridCraft doesn't have: it is not loaded"
        );
    }

    #[test]
    fn a_kits_project_holds_only_what_the_kit_can_serve_and_run() {
        // Everything there was before is still a PythonCraft project's to have.
        let all = "permissions = [\"read\", \"write\", \"files\", \"network\", \"python\"]\n[panel]\nentry = \"panel/index.html\"\n[python]\nscripts = \"python\"";
        assert_eq!(check(&read(all)), Ok(&PYTHONCRAFT));
        // A C project may ask to see and change the workbook, and to open and save files.
        assert_eq!(check(&read("kit = \"c\"\npermissions = [\"read\", \"write\", \"files\"]")), Ok(&C));
        // It has no scripts: nothing of the first kit that can't be limited can be made to run for it.
        let python = check(&read("kit = \"c\"\npermissions = [\"python\"]")).unwrap_err();
        assert_eq!(python, "it asks for \"python\", which a project of the C Development Kit can't be given");
        assert!(check(&read("kit = \"c\"\npermissions = [\"read\", \"network\"]")).unwrap_err().contains("\"network\""));
        let scripts = check(&read("kit = \"c\"\n[python]\nscripts = \"python\"")).unwrap_err();
        assert_eq!(scripts, "it has Python scripts, which a project of the C Development Kit can't have");
        // It may have a panel: that is where the program built from it runs.
        assert_eq!(check(&read("kit = \"c\"\n[panel]\nentry = \"panel/index.html\"\n[c]\nsources = [\"src\"]")), Ok(&C));
        assert_eq!(check(&read("kit = \"c\"\n[panel]\nentry = \"panel/index.html\"")), Ok(&C));
        // A recipe is the C kit's: it is nothing the first kit builds.
        assert_eq!(check(&read("kit = \"c\"\n[c]\nsources = [\"src\"]")), Ok(&C));
        let recipe = check(&read("[c]\nsources = [\"src\"]")).unwrap_err();
        assert_eq!(recipe, "it has a C recipe, which a project of PythonCraft can't have");
        // No kit's projects may be given something that is not a permission at all.
        for kit in loaded() {
            assert!(kit.permissions.iter().all(|allowed| Permission::ALL.contains(allowed)));
        }
        assert!(!C.permissions.iter().any(|allowed| allowed.is_unlimited()), "nothing a C project can be given is without limit");
    }

    #[test]
    fn a_project_is_workbook_code_when_it_names_a_workbook_and_for_any_workbook_otherwise() {
        assert_eq!(ProjectKind::of(Some(&read("workbook = \"Budget.xlsm\""))), ProjectKind::WorkbookCode);
        assert_eq!(ProjectKind::of(Some(&read(""))), ProjectKind::General);
        assert_eq!(ProjectKind::of(Some(&read("workbook = \"\""))), ProjectKind::General, "an empty name names none");
        assert_eq!(ProjectKind::of(None), ProjectKind::General, "one that can't be read names none");
        // A project of any kit is one or the other.
        assert_eq!(ProjectKind::of(Some(&read("kit = \"c\"\nworkbook = \"Doom.xlsx\""))), ProjectKind::WorkbookCode);
        for kind in ProjectKind::ALL {
            assert!(!kind.code().is_empty() && !kind.label().is_empty());
        }
        assert_eq!((ProjectKind::WorkbookCode.code(), ProjectKind::General.code()), ("workbook", "general"));
    }

    #[test]
    fn no_project_is_called_a_library() {
        // The word is kept for a project that other projects or a spreadsheet can refer to,
        // which none can yet: nothing people or agents are told of a project's kind uses it.
        for kind in ProjectKind::ALL {
            for said in [kind.code(), kind.label()] {
                assert!(!said.to_lowercase().contains("librar"), "{said}");
            }
        }
        for kit in loaded() {
            for said in [kit.name, kit.says, kit.project_is, kit.not_yet] {
                assert!(!said.to_lowercase().contains("librar"), "{said}");
            }
        }
    }
}
