//! Where the Development subsystem keeps things: beside the user's settings.
//!
//! This is the folder the desktop app keeps `ui.json` and `prefs.json` in (the same rule as its
//! `config_dir`, repeated here so that nothing above this crate has to hand the path down). A run
//! without preferences (`GRIDCRAFT_NO_PREFS`, agents' test runs) and the web build have no such
//! folder, and so no modules.

use std::path::PathBuf;

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

/// The folder the user's modules are installed in.
pub fn modules_dir() -> Option<PathBuf> {
    data_dir().map(|dir| dir.join("modules"))
}

/// The file holding the user's decisions to trust modules. It is beside the modules folder, not in
/// it: no module's files are ever written there.
pub fn trust_file() -> Option<PathBuf> {
    data_dir().map(|dir| dir.join("trusted-modules.json"))
}

/// The Development subsystem's own settings (which Python to use).
pub fn settings_file() -> Option<PathBuf> {
    data_dir().map(|dir| dir.join("developer.json"))
}
