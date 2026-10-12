//! Where the Development subsystem keeps things: beside the user's settings.
//!
//! This is the folder the desktop app keeps `ui.json` and `prefs.json` in (the same rule as its
//! `config_dir`, repeated here so that nothing above this crate has to hand the path down). A run
//! without preferences (`GRIDCRAFT_NO_PREFS`, agents' test runs) and the web build have no such
//! folder, and so no projects.

use std::path::{Path, PathBuf};

/// The user's GridCraft settings folder, if this run has one.
pub fn data_dir() -> Option<PathBuf> {
    if cfg!(target_arch = "wasm32") || std::env::var_os("GRIDCRAFT_NO_PREFS").is_some() {
        return None;
    }
    let var = |name: &str| std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from);
    if cfg!(target_os = "macos") {
        var("HOME").map(|home| home.join("Library/Application Support/GridCraft"))
    } else if cfg!(windows) {
        var("APPDATA").map(|appdata| appdata.join("GridCraft"))
    } else {
        var("XDG_CONFIG_HOME").or_else(|| var("HOME").map(|home| home.join(".config"))).map(|config| config.join("gridcraft"))
    }
}

/// A name in the settings folder; or the name it had while a project was called a module,
/// where only that one is there. What a person made before the change stays where it is, and
/// is still found: nothing of theirs is moved for them.
pub fn named(dir: &Path, now: &str, before: &str) -> PathBuf {
    let (current, older) = (dir.join(now), dir.join(before));
    if !current.exists() && older.exists() { older } else { current }
}

/// The folder the user's projects are installed in.
pub fn projects_dir() -> Option<PathBuf> {
    data_dir().map(|dir| named(&dir, "projects", "modules"))
}

/// The file holding the user's decisions to trust projects. It is beside the projects folder, not in
/// it: no project's files are ever written there.
pub fn trust_file() -> Option<PathBuf> {
    data_dir().map(|dir| named(&dir, "trusted-projects.json", "trusted-modules.json"))
}

/// The Development subsystem's own settings (which Python to use).
pub fn settings_file() -> Option<PathBuf> {
    data_dir().map(|dir| dir.join("developer.json"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::tests::TempDir;

    #[test]
    fn what_was_kept_under_the_older_names_is_still_found_and_nothing_is_moved() {
        let t = TempDir::new("paths");
        // A settings folder that has neither: the names of now.
        assert_eq!(named(&t.0, "projects", "modules"), t.0.join("projects"));
        assert_eq!(named(&t.0, "trusted-projects.json", "trusted-modules.json"), t.0.join("trusted-projects.json"));
        // One from before the change: what is there is used where it is.
        t.write("modules/clean-data/module.toml", "id = \"clean-data\"");
        t.write("trusted-modules.json", "{}");
        assert_eq!(named(&t.0, "projects", "modules"), t.0.join("modules"));
        assert_eq!(named(&t.0, "trusted-projects.json", "trusted-modules.json"), t.0.join("trusted-modules.json"));
        assert!(t.0.join("modules/clean-data/module.toml").is_file() && !t.0.join("projects").exists(), "nothing was moved");
        // Once the names of now are there, they are the ones used.
        t.write("projects/tools/project.toml", "id = \"tools\"");
        t.write("trusted-projects.json", "{}");
        assert_eq!(named(&t.0, "projects", "modules"), t.0.join("projects"));
        assert_eq!(named(&t.0, "trusted-projects.json", "trusted-modules.json"), t.0.join("trusted-projects.json"));
    }
}
