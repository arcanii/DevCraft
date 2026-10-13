//! Development kits: the ways of building on GridCraft that are loaded into it.
//!
//! A kit says what a project is made of and how it is run. This build has two:
//!
//! - the **Anie Development Kit**: a project is a folder with a `project.toml`, Python scripts
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
//! nothing is the Anie kit's, as every project was before there were two. A kit's projects may
//! hold only what the kit knows how to serve and run ([`check`]): a C project has no scripts
//! and can't ask for `python` or `network`, so nothing of the Anie kit that can't be limited
//! can be made to run for it.
//!
//! Both kits are compiled into GridCraft. There is no way to load one yet: how a kit that isn't
//! compiled in would be loaded is not decided (ADR-0001).
//!
//! **A project** is one of two kinds, by what its `project.toml` says: the code of one
//! workbook, which it names, or a library, which is for any workbook. The kind only says what
//! the project is for: nothing is allowed or run because of it. (Until 2026-10-13 a project
//! was called a module, and its manifest `module.toml`: what was made then is still read.)

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
    /// What its projects may ask to be allowed.
    pub permissions: &'static [Permission],
    /// Its projects may have a panel (`[panel]` in the manifest).
    pub panels: bool,
    /// Its projects may have Python scripts (`[python]` in the manifest).
    pub scripts: bool,
    /// Its projects are built, from a recipe (`[c]` in the manifest).
    pub builds: bool,
}

/// The kit there was before there were two: Python scripts and a web panel.
pub const ANIE: Kit = Kit {
    id: manifest::DEFAULT_KIT,
    name: "Anie Development Kit",
    made_of: &["Python", "Svelte"],
    says: "Projects made of Python scripts and a web panel, which can be a Svelte app or any other page. A project does nothing until you trust it.",
    project_is: "A project is a folder with a project.toml, Python scripts and a web panel.",
    not_yet: "",
    built_in: true,
    permissions: &Permission::ALL,
    panels: true,
    scripts: true,
    builds: false,
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
    // What a project's page can ask of the spreadsheet for the program it runs. The program
    // is WebAssembly: it reaches nothing but what that page hands it, so running it needs no
    // leave of its own, as `python` does.
    permissions: &[Permission::Read, Permission::Write, Permission::Files],
    panels: true,
    scripts: false,
    builds: true,
};

/// The kits that are loaded, in the order they are listed.
pub fn loaded() -> &'static [Kit] {
    &[ANIE, C]
}

/// The loaded kit with this id.
pub fn named(id: &str) -> Option<&'static Kit> {
    loaded().iter().find(|kit| kit.id == id)
}

/// The kit a project is listed under: the one its manifest names. A project that can't be
/// placed (its manifest can't be read, or names a kit this build doesn't have) is listed under
/// the first kit, with what is wrong with it.
pub fn of(manifest: Option<&Manifest>) -> &'static Kit {
    manifest.and_then(|manifest| named(&manifest.kit)).unwrap_or(&ANIE)
}

/// Whether a manifest is one a project of its kit can have: the kit is one this build has, and
/// the project holds and asks for nothing the kit doesn't know. A project that names a kit and
/// then asks for what another kit runs is refused, not guessed at.
pub fn check(manifest: &Manifest) -> Result<&'static Kit, String> {
    let Some(kit) = named(&manifest.kit) else {
        return Err(format!("it is a project of the kit {:?}, which this GridCraft doesn't have", manifest.kit));
    };
    if let Some(asked) = manifest.permissions.iter().find(|asked| !kit.permissions.contains(asked)) {
        return Err(format!("it asks for {:?}, which a project of the {} can't be given", asked.code(), kit.name));
    }
    if manifest.panel.is_some() && !kit.panels {
        return Err(format!("it has a panel, which a project of the {} can't have", kit.name));
    }
    if manifest.python.is_some() && !kit.scripts {
        return Err(format!("it has Python scripts, which a project of the {} can't have", kit.name));
    }
    if manifest.c.is_some() && !kit.builds {
        return Err(format!("it has a C recipe, which a project of the {} can't have", kit.name));
    }
    Ok(kit)
}

/// What a project is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectKind {
    /// The code of one workbook, which its `project.toml` names.
    WorkbookCode,
    /// For any workbook.
    Library,
}

impl ProjectKind {
    pub const ALL: [ProjectKind; 2] = [ProjectKind::WorkbookCode, ProjectKind::Library];

    /// What a project with this manifest is. One without a manifest that could be read is
    /// listed with the libraries.
    pub fn of(manifest: Option<&Manifest>) -> ProjectKind {
        match manifest.and_then(|manifest| manifest.workbook.as_ref()) {
            Some(_) => ProjectKind::WorkbookCode,
            None => ProjectKind::Library,
        }
    }

    /// The word reports use.
    pub fn code(self) -> &'static str {
        match self {
            ProjectKind::WorkbookCode => "workbook",
            ProjectKind::Library => "library",
        }
    }

    /// The words people are shown: one project of the kind, and the heading over several.
    pub fn label(self) -> &'static str {
        match self {
            ProjectKind::WorkbookCode => "Workbook code",
            ProjectKind::Library => "Library",
        }
    }

    pub fn heading(self) -> &'static str {
        match self {
            ProjectKind::WorkbookCode => "Workbook code",
            ProjectKind::Library => "Libraries",
        }
    }

    /// What the kind is, in a sentence.
    pub fn says(self) -> &'static str {
        match self {
            ProjectKind::WorkbookCode => "Projects that are the code of one workbook, as its macros were.",
            ProjectKind::Library => "Projects for any workbook.",
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
        assert_eq!(kits.iter().map(|kit| kit.id).collect::<Vec<_>>(), ["anie", "c"], "two kits, and no way to load another yet");
        assert_eq!((kits[0].name, kits[0].made_of, kits[0].built_in), ("Anie Development Kit", ["Python", "Svelte"].as_slice(), true));
        assert_eq!((kits[1].name, kits[1].made_of, kits[1].built_in), ("C Development Kit", ["C"].as_slice(), true));
        for kit in kits {
            assert!(manifest::valid_id(kit.id) && !kit.says.is_empty() && !kit.project_is.is_empty(), "{}", kit.id);
            assert_eq!(named(kit.id), Some(kit));
        }
        assert_eq!(ANIE.id, manifest::DEFAULT_KIT, "a project that names no kit is the first kit's");
        // Both are whole: neither has something to say it can't do yet.
        assert!(ANIE.not_yet.is_empty() && C.not_yet.is_empty());
        assert!(C.says.contains("WebAssembly") && C.says.contains("panel") && C.project_is.contains("panel"));
        assert_eq!((C.builds, ANIE.builds), (true, false), "the second kit builds its projects; the first has nothing to build");
        assert_eq!((C.panels, C.scripts), (true, false), "what it builds runs in the project's panel; it has no scripts");
        assert_eq!((named("rust"), named(""), named("C")), (None, None, None));
    }

    #[test]
    fn a_project_is_of_the_kit_it_names_and_of_the_first_when_it_names_none() {
        assert_eq!(of(Some(&read(""))), &ANIE);
        assert_eq!(of(Some(&read("kit = \"anie\""))), &ANIE);
        assert_eq!(of(Some(&read("kit = \"c\""))), &C);
        // One that can't be placed is listed under the first kit: `check` says what is wrong.
        assert_eq!((of(Some(&read("kit = \"rust\""))), of(None)), (&ANIE, &ANIE));
        assert_eq!(check(&read("")), Ok(&ANIE));
        assert_eq!(check(&read("kit = \"c\"")), Ok(&C));
        assert_eq!(check(&read("kit = \"rust\"")).unwrap_err(), "it is a project of the kit \"rust\", which this GridCraft doesn't have");
    }

    #[test]
    fn a_kits_project_holds_only_what_the_kit_can_serve_and_run() {
        // Everything there was before is still an Anie project's to have.
        let all = "permissions = [\"read\", \"write\", \"files\", \"network\", \"python\"]\n[panel]\nentry = \"panel/index.html\"\n[python]\nscripts = \"python\"";
        assert_eq!(check(&read(all)), Ok(&ANIE));
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
        assert_eq!(recipe, "it has a C recipe, which a project of the Anie Development Kit can't have");
        // No kit's projects may be given something that is not a permission at all.
        for kit in loaded() {
            assert!(kit.permissions.iter().all(|allowed| Permission::ALL.contains(allowed)));
        }
        assert!(!C.permissions.iter().any(|allowed| allowed.is_unlimited()), "nothing a C project can be given is without limit");
    }

    #[test]
    fn a_project_is_workbook_code_when_it_names_a_workbook_and_a_library_otherwise() {
        assert_eq!(ProjectKind::of(Some(&read("workbook = \"Budget.xlsm\""))), ProjectKind::WorkbookCode);
        assert_eq!(ProjectKind::of(Some(&read(""))), ProjectKind::Library);
        assert_eq!(ProjectKind::of(Some(&read("workbook = \"\""))), ProjectKind::Library, "an empty name names none");
        assert_eq!(ProjectKind::of(None), ProjectKind::Library, "one that can't be read is listed with the libraries");
        // A project of any kit is one or the other.
        assert_eq!(ProjectKind::of(Some(&read("kit = \"c\"\nworkbook = \"Doom.xlsx\""))), ProjectKind::WorkbookCode);
        for kind in ProjectKind::ALL {
            assert!(!kind.code().is_empty() && !kind.label().is_empty() && !kind.heading().is_empty() && !kind.says().is_empty());
        }
        assert_eq!((ProjectKind::WorkbookCode.code(), ProjectKind::Library.code()), ("workbook", "library"));
    }
}
