//! The Development subsystem's own settings, in `developer.json` beside the user's settings (not
//! in upstream's `ui.json` or `prefs.json`, so those files keep upstream's shape).

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// The largest settings file read (bytes).
const MAX_BYTES: u64 = 64 * 1024;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    /// The Python that runs modules' scripts: a path or a command. Empty: GridCraft finds one.
    pub python: String,
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
    let text = serde_json::to_string_pretty(settings).map_err(|e| format!("the developer settings could not be saved: {e}"))?;
    fs::write(path, text).map_err(failed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::tests::TempDir;

    #[test]
    fn settings_are_kept_and_a_damaged_file_gives_the_defaults() {
        let t = TempDir::new("settings");
        let file = t.0.join("deep/developer.json");
        assert_eq!(load(&file), Settings::default());
        save(&file, &Settings { python: "C:\\Python\\python.exe".into() }).unwrap();
        assert_eq!(load(&file).python, "C:\\Python\\python.exe");
        for damaged in ["", "not json", "[]", "{\"python\": 3}"] {
            fs::write(&file, damaged).unwrap();
            assert_eq!(load(&file), Settings::default(), "{damaged:?}");
        }
        fs::write(&file, "{\"future\": true}").unwrap();
        assert_eq!(load(&file), Settings::default(), "keys it doesn't know are ignored");
    }
}
