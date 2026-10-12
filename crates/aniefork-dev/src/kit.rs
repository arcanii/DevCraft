//! Development kits: the ways of building on GridCraft that are loaded into it.
//!
//! A kit says what a project is made of and how it is run. The one this build has is the
//! **Anie Development Kit**: a project is a folder with a `project.toml`, Python scripts and a
//! web panel (written with Svelte or any other web toolkit). Everything else in this crate is
//! that kit at work: its projects on disk ([`crate::project`]), the trust in them
//! ([`crate::trust`]), their scripts ([`crate::python`]) and panels ([`crate::bridge`]).
//!
//! Kits are listed, not assumed to be one, so that another can stand beside this one. There
//! is no way to load one yet: how a kit that isn't compiled into GridCraft would be loaded is
//! not decided (ADR-0001).
//!
//! **A project** is one of two kinds, by what its `project.toml` says: the code of one
//! workbook, which it names, or a library, which is for any workbook. The kind only says what
//! the project is for: nothing is allowed or run because of it. (Until 2026-10-13 a project
//! was called a module, and its manifest `module.toml`: what was made then is still read.)

use crate::manifest::Manifest;

/// A development kit, as it is listed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kit {
    /// Its name among the kits: lowercase letters, digits and hyphens.
    pub id: &'static str,
    pub name: &'static str,
    /// What its projects are written in.
    pub made_of: &'static [&'static str],
    /// What it is, in a sentence or two.
    pub says: &'static str,
    /// It is compiled into GridCraft.
    pub built_in: bool,
}

/// The kit this build has.
pub const ANIE: Kit = Kit {
    id: "anie",
    name: "Anie Development Kit",
    made_of: &["Python", "Svelte"],
    says: "Projects made of Python scripts and a web panel, which can be a Svelte app or any other page. A project does nothing until you trust it.",
    built_in: true,
};

/// The kits that are loaded, in the order they are listed.
pub fn loaded() -> &'static [Kit] {
    &[ANIE]
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
    use crate::manifest;

    #[test]
    fn the_kit_this_build_has_is_listed_and_names_what_it_is_made_of() {
        let kits = loaded();
        assert_eq!(kits.len(), 1, "one kit, until there is a way to load another");
        assert_eq!((kits[0].id, kits[0].name, kits[0].built_in), ("anie", "Anie Development Kit", true));
        assert_eq!(kits[0].made_of, ["Python", "Svelte"]);
        assert!(manifest::valid_id(kits[0].id) && !kits[0].says.is_empty());
    }

    #[test]
    fn a_project_is_workbook_code_when_it_names_a_workbook_and_a_library_otherwise() {
        let read = |more: &str| manifest::parse(&format!("id = \"x\"\nname = \"X\"\nversion = \"1\"\n{more}"), "x").unwrap();
        assert_eq!(ProjectKind::of(Some(&read("workbook = \"Budget.xlsm\""))), ProjectKind::WorkbookCode);
        assert_eq!(ProjectKind::of(Some(&read(""))), ProjectKind::Library);
        assert_eq!(ProjectKind::of(Some(&read("workbook = \"\""))), ProjectKind::Library, "an empty name names none");
        assert_eq!(ProjectKind::of(None), ProjectKind::Library, "one that can't be read is listed with the libraries");
        for kind in ProjectKind::ALL {
            assert!(!kind.code().is_empty() && !kind.label().is_empty() && !kind.heading().is_empty() && !kind.says().is_empty());
        }
        assert_eq!((ProjectKind::WorkbookCode.code(), ProjectKind::Library.code()), ("workbook", "library"));
    }
}
