//! The Development subsystem's own settings, in `developer.json` beside the user's settings (not
//! in upstream's `ui.json` or `prefs.json`, so those files keep upstream's shape).

use std::fs;
use std::path::Path;

use serde::{Deserialize, Deserializer, Serialize};

use crate::agents::{DEFAULT_PORT, Level};

/// The largest settings file read (bytes).
const MAX_BYTES: u64 = 64 * 1024;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    /// The Python that runs projects' scripts: a path or a command. Empty: GridCraft finds one.
    pub python: String,
    /// The Development server: how far agents may go, and where it listens.
    pub agents: Agents,
}

/// The Development server's settings. They are set by hand, in the Development window.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Agents {
    /// How far agents may go. Full control is never kept: it lasts until GridCraft closes, and
    /// a file that says `full` is read as `ask`.
    #[serde(deserialize_with = "level")]
    pub level: Level,
    /// The loopback port the server listens on.
    pub port: u16,
}

impl Default for Agents {
    fn default() -> Self {
        Agents { level: Level::Off, port: DEFAULT_PORT }
    }
}

/// A level as the file may hold it: a word this version doesn't know is `off`.
fn level<'de, D: Deserializer<'de>>(from: D) -> Result<Level, D::Error> {
    let word = serde_json::Value::deserialize(from)?;
    Ok(word.as_str().and_then(Level::parse).unwrap_or_default().kept())
}

/// Loads the settings. A file that is missing, too large or damaged gives the defaults.
pub fn load(path: &Path) -> Settings {
    let small = fs::metadata(path).is_ok_and(|m| m.is_file() && m.len() <= MAX_BYTES);
    small.then(|| fs::read_to_string(path).ok()).flatten().and_then(|text| serde_json::from_str(&text).ok()).unwrap_or_default()
}

pub fn save(path: &Path, settings: &Settings) -> Result<(), String> {
    let failed = |e: std::io::Error| format!("the developer settings could not be saved: {e}");
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(failed)?;
    }
    let mut kept = settings.clone();
    kept.agents.level = kept.agents.level.kept();
    let text = serde_json::to_string_pretty(&kept).map_err(|e| format!("the developer settings could not be saved: {e}"))?;
    fs::write(path, text).map_err(failed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::tests::TempDir;

    #[test]
    fn settings_are_kept_and_a_damaged_file_gives_the_defaults() {
        let t = TempDir::new("settings");
        let file = t.0.join("deep/developer.json");
        assert_eq!(load(&file), Settings::default());
        save(&file, &Settings { python: "C:\\Python\\python.exe".into(), ..Settings::default() }).unwrap();
        assert_eq!(load(&file).python, "C:\\Python\\python.exe");
        for damaged in ["", "not json", "[]", "{\"python\": 3}"] {
            fs::write(&file, damaged).unwrap();
            assert_eq!(load(&file), Settings::default(), "{damaged:?}");
        }
        fs::write(&file, "{\"future\": true}").unwrap();
        assert_eq!(load(&file), Settings::default(), "keys it doesn't know are ignored");
    }

    #[test]
    fn agent_access_is_off_until_set_and_full_control_is_never_kept() {
        let t = TempDir::new("agent-settings");
        let file = t.0.join("developer.json");
        assert_eq!(load(&file).agents, Agents { level: Level::Off, port: DEFAULT_PORT });
        // A file from before the server existed leaves it off, and keeps what it held.
        fs::write(&file, "{\"python\": \"py\"}").unwrap();
        assert_eq!((load(&file).python.as_str(), load(&file).agents.level), ("py", Level::Off));
        for level in [Level::Read, Level::Ask, Level::Build] {
            save(&file, &Settings { agents: Agents { level, port: 7001 }, ..Settings::default() }).unwrap();
            assert_eq!(load(&file).agents, Agents { level, port: 7001 });
        }
        // Full control lasts until GridCraft closes: it isn't written, and isn't read.
        save(&file, &Settings { agents: Agents { level: Level::Full, port: 7001 }, ..Settings::default() }).unwrap();
        assert_eq!(load(&file).agents.level, Level::Ask);
        assert!(!fs::read_to_string(&file).unwrap().contains("full"));
        fs::write(&file, "{\"agents\": {\"level\": \"full\"}}").unwrap();
        assert_eq!(load(&file).agents.level, Level::Ask, "a file edited to say full");
        // A word this version doesn't know is off, and the rest of the file is still read.
        fs::write(&file, "{\"python\": \"py\", \"agents\": {\"level\": \"everything\"}}").unwrap();
        assert_eq!((load(&file).python.as_str(), load(&file).agents.level), ("py", Level::Off));
    }
}
