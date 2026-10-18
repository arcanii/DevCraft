//! The Svelte showcase: a project whose panel is a Svelte app, to show what a panel written with
//! a web toolkit looks like in GridCraft, docked in the window or in the browser.
//!
//! It is a project like any other: created untrusted, in the user's projects folder, and it does
//! nothing until the user trusts it. It asks for `read` and `write` and nothing more, runs no
//! Python, and loads nothing from the network (the Svelte runtime is inside its script).
//!
//! Its files are in `showcase_files`, which is generated: the panel as GridCraft serves it
//! (`panel/`), and the Svelte source that panel is built from (`source/`), so that whoever
//! makes the project has what they need to change it. The fork ships them as text inside Rust
//! source, as it does the starter project's: the repository holds no JavaScript file.
//!
//! **Every kit's showcases** are listed here too ([`ALL`]): this one, PythonCraft's, and the C
//! kit's two, the turning triangle ([`crate::triangle`]) and the Mandelbrot set
//! ([`crate::mandelbrot`]). A kit's part offers its own, by id ([`named`], [`of_kit`]).

use std::fs;
use std::path::Path;

use crate::project::{self, Project};
use crate::showcase_files::FILES;

/// The showcase project's id, and its folder's name.
pub const ID: &str = "svelte-showcase";

/// A showcase: a project made from files compiled into GridCraft, to show what a kit's projects
/// can be. Each is created untrusted, like any project.
#[derive(Clone, Copy, Debug)]
pub struct Showcase {
    /// The project's id, and its folder's name.
    pub id: &'static str,
    pub name: &'static str,
    /// The kit whose part offers it, by the kit's id.
    pub kit: &'static str,
    /// What it is, as its kit's part says before it is created.
    pub says: &'static str,
    /// Its files: where each goes in the project, and its text.
    pub files: &'static [(&'static str, &'static str)],
}

/// The Svelte showcase, as PythonCraft's part offers it.
pub const SVELTE: Showcase = Showcase {
    id: ID,
    name: "Svelte Showcase",
    kit: crate::kit::PYTHONCRAFT.id,
    says: "A project whose panel is a Svelte app, docked beside the sheet.",
    files: FILES,
};

/// Every showcase, in the order each kit's part offers its own.
pub static ALL: [Showcase; 3] = [SVELTE, crate::triangle::SHOWCASE, crate::mandelbrot::SHOWCASE];

/// The showcase of that id.
pub fn named(id: &str) -> Option<&'static Showcase> {
    ALL.iter().find(|one| one.id == id)
}

/// The showcases a kit's part offers, by the kit's id.
pub fn of_kit(kit: &str) -> impl Iterator<Item = &'static Showcase> + '_ {
    ALL.iter().filter(move |one| one.kit == kit)
}

impl Showcase {
    /// Creates the showcase project in `projects_dir` and returns it as read back from disk. It
    /// never touches a folder that already exists.
    pub fn create(&self, projects_dir: &Path) -> Result<Project, String> {
        let dir = projects_dir.join(self.id);
        if dir.exists() {
            return Err(format!("There is already a project folder named {}.", self.id));
        }
        let failed = |e: std::io::Error| format!("The showcase could not be created: {e}");
        for (path, text) in self.files {
            let file = path.split('/').fold(dir.clone(), |at, part| at.join(part));
            if let Some(folder) = file.parent() {
                fs::create_dir_all(folder).map_err(failed)?;
            }
            fs::write(&file, text).map_err(failed)?;
        }
        let made = project::load(&dir);
        match &made.problem {
            Some(problem) => Err(format!("The showcase was created, but can't be used: {problem}.")),
            None => Ok(made),
        }
    }
}

/// Creates the showcase project in `projects_dir` and returns it as read back from disk. It
/// never touches a folder that already exists.
pub fn create(projects_dir: &Path) -> Result<Project, String> {
    SVELTE.create(projects_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Permission;
    use crate::project::tests::TempDir;
    use crate::trust::{Status, TrustStore};

    #[test]
    fn the_showcase_is_a_sound_project_that_asks_for_little_and_starts_untrusted() {
        let t = TempDir::new("showcase");
        let made = create(&t.0).unwrap();
        assert!(made.is_sound(), "{:?}", made.problem);
        assert_eq!((made.id.as_str(), made.name()), (ID, "Svelte Showcase"));
        let manifest = made.manifest.as_ref().unwrap();
        assert_eq!(manifest.permissions, [Permission::Read, Permission::Write], "no Python, no network, no files");
        assert_eq!((manifest.panel.as_deref(), manifest.python.as_deref()), (Some("panel/index.html"), None));
        // Every file is there, and nothing else.
        let mut listed: Vec<&str> = FILES.iter().map(|(path, _)| *path).collect();
        listed.sort_unstable();
        assert_eq!(made.files, listed);
        // Like any project it does nothing until the user trusts it.
        assert_eq!(TrustStore::load(&t.0.join("trusted-projects.json")).status(&made), Status::Untrusted);
        // A second one is not made over the first.
        std::fs::write(t.0.join(ID).join("panel/app.css"), "/* mine */").unwrap();
        assert!(create(&t.0).unwrap_err().contains("already a project folder"));
        assert_eq!(std::fs::read_to_string(t.0.join(ID).join("panel/app.css")).unwrap(), "/* mine */");
    }

    #[test]
    fn the_panel_is_built_and_loads_nothing_from_anywhere_else() {
        let file = |path: &str| FILES.iter().find(|(name, _)| *name == path).map(|(_, text)| *text).unwrap_or_else(|| panic!("no {path}"));
        // The page: GridCraft's script, then the app's, and no address but its own files.
        let page = file("panel/index.html");
        let (first, second) = (page.find("src=\"spreadsheet.js\"").unwrap(), page.find("src=\"app.js\"").unwrap());
        assert!(first < second, "the spreadsheet object is there before the app starts");
        assert!(page.contains("href=\"app.css\"") && !page.contains("http"), "{page}");
        // The script is the built one: one long line, with the app in it.
        let script = file("panel/app.js");
        assert!(script.len() > 20_000 && script.lines().count() < 20, "{} bytes", script.len());
        assert!(script.contains("Svelte in GridCraft") && script.contains("selectionChanged"));
        assert!(
            !script.starts_with("import")
                && !script.contains(
                    "
import "
                ),
            "it is one script, not a module to resolve"
        );
        // It asks nothing of the network: no request is made from the app's own code.
        for call in ["fetch(", "XMLHttpRequest", "WebSocket(", "sendBeacon", "eval("] {
            assert!(!script.contains(call), "{call}");
        }
        assert!(!file("panel/app.css").contains("url("));
        // The source it was built from goes with it, and so do the notices of what it holds.
        let source = file("source/src/App.svelte");
        assert!(source.contains("Svelte in GridCraft") && source.contains("$derived") && source.contains("window.spreadsheet"));
        let package = file("source/package.json");
        assert!(package.contains("\"svelte\": \"") && package.contains("\"vite\": \""));
        let notices = file("source/THIRD-PARTY.md");
        for library in ["svelte", "clsx", "esm-env"] {
            assert!(notices.contains(&format!("## {library} ")), "{library}");
        }
        assert!(notices.matches("Permission is hereby granted").count() >= 3);
        // Every path is one a project may hold.
        for (path, _) in FILES {
            assert_eq!(crate::manifest::inner_path(path).as_deref(), Ok(*path));
        }
    }

    #[test]
    fn each_kit_offers_its_own_showcases_each_a_sound_project_of_that_kit() {
        let t = TempDir::new("showcases");
        let ids = |kit: &str| of_kit(kit).map(|one| one.id).collect::<Vec<_>>();
        assert_eq!(ids(crate::kit::PYTHONCRAFT.id), [ID]);
        assert_eq!(ids(crate::kit::C.id), [crate::triangle::ID, crate::mandelbrot::ID]);
        assert_eq!(ids("no-such-kit"), Vec::<&str>::new());
        for one in &ALL {
            assert_eq!(named(one.id).map(|found| found.name), Some(one.name));
            // Made, it is a project of the kit that offers it, by its own id and name.
            let made = one.create(&t.0).unwrap();
            assert!(made.is_sound(), "{}: {:?}", one.id, made.problem);
            assert_eq!((made.id.as_str(), made.name()), (one.id, one.name));
            assert_eq!(crate::manifest::kit_id(&made.manifest.as_ref().unwrap().kit), one.kit);
            assert!(!one.says.is_empty() && one.says.ends_with('.'), "{}", one.says);
        }
        assert!(named("blink").is_none() && named("").is_none());
    }
}
