//! The Development subsystem of the aniefork fork of GridCraft: **modules** a user installs to
//! give GridCraft new abilities (web panels, Python). Designed in `docs/decisions-aniefork.md`
//! (ADR-0001).
//!
//! It finds modules, reads their manifests, fingerprints exactly what they contain, keeps the
//! user's decisions to trust them, and runs a trusted module's Python scripts. Whatever runs or
//! serves anything of a module asks first, here, whether the module is trusted and what it was
//! granted.
//!
//! - [`manifest`]: `module.toml`, and the permissions a module can ask for.
//! - [`module`]: modules on disk and their fingerprints.
//! - [`trust`]: the user's decisions, bound to fingerprints.
//! - [`python`]: running a trusted module's scripts in a process of their own.
//! - [`bridge`]: serving a trusted module's panel to the browser, carrying its calls, and telling
//!   it what happens in GridCraft.
//! - [`console`]: the Developer console: what modules printed and logged, and what failed.
//! - [`agents`]: the Development server, a second MCP server for agents that help with
//!   development work, as far as the user lets them go.
//! - [`starter`]: "New Module".
//! - [`paths`], [`settings`]: where modules, decisions and the subsystem's settings are kept.
//!
//! It has no dependency on the UI toolkit or on the rest of GridCraft, so that it can be offered
//! upstream as a piece of its own.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

pub mod agents;
pub mod bridge;
pub mod console;
#[cfg(not(target_arch = "wasm32"))]
mod http;
pub mod manifest;
pub mod module;
pub mod paths;
pub mod python;
pub mod settings;
pub mod sha256;
pub mod starter;
pub mod trust;

use serde_json::{Value as Json, json};

pub use manifest::{Manifest, Permission};
pub use module::Module;
pub use trust::{Status, TrustStore};

/// A module and where it stands, as the Modules dialog shows it and `developer.modules` reports it.
pub fn describe(module: &Module, store: &TrustStore) -> Json {
    let status = store.status(module);
    let manifest = module.manifest.as_ref();
    let asks: Vec<Json> = manifest
        .map(|m| m.permissions.iter().map(|p| json!({"permission": p.code(), "means": p.meaning(), "unlimited": p.is_unlimited()})).collect())
        .unwrap_or_default();
    let granted: Vec<&str> = match &status {
        Status::Trusted(permissions) => permissions.iter().map(|p| p.code()).collect(),
        _ => vec![],
    };
    json!({
        "id": module.id,
        "name": module.name(),
        "version": manifest.map(|m| m.version.as_str()).unwrap_or(""),
        "author": manifest.map(|m| m.author.as_str()).unwrap_or(""),
        "description": manifest.map(|m| m.description.as_str()).unwrap_or(""),
        "status": status.code(),
        "problem": match &status { Status::Problem(why) => Json::String(why.clone()), _ => Json::Null },
        "asks": asks,
        "granted": granted,
        "followsChanges": store.follows_changes(&module.id),
        "panel": manifest.and_then(|m| m.panel.as_deref()),
        "python": manifest.and_then(|m| m.python.as_deref()),
        "files": module.files,
        "fingerprint": module.fingerprint,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::tests::{TempDir, sample};

    #[test]
    fn a_module_is_described_with_what_it_asks_for_and_what_it_was_granted() {
        let t = TempDir::new("describe");
        let m = module::load(&sample(&t));
        let mut store = TrustStore::load(&t.0.join("trusted-modules.json"));
        let before = describe(&m, &store);
        assert_eq!(
            (before["id"].as_str(), before["name"].as_str(), before["status"].as_str()),
            (Some("clean-data"), Some("Clean Data"), Some("untrusted"))
        );
        assert_eq!(before["granted"], json!([]), "nothing is granted to an untrusted module");
        let asks: Vec<&str> = before["asks"].as_array().unwrap().iter().filter_map(|a| a["permission"].as_str()).collect();
        assert_eq!(asks, ["read", "python"]);
        assert_eq!(before["asks"][1]["unlimited"], json!(true), "Python is marked as the one that can't be limited");
        assert!(before["asks"].as_array().unwrap().iter().all(|a| a["means"].as_str().is_some_and(|text| !text.is_empty())));
        assert_eq!(before["files"], json!(["module.toml", "panel/index.html", "python/clean.py"]));
        store.trust(&m, false).unwrap();
        let after = describe(&m, &store);
        assert_eq!((after["status"].as_str(), &after["granted"]), (Some("trusted"), &json!(["read", "python"])));
        // A broken module says why.
        t.write("broken/readme.txt", "x");
        let broken = describe(&module::load(&t.0.join("broken")), &store);
        assert_eq!(broken["status"], json!("problem"));
        assert!(broken["problem"].as_str().is_some_and(|why| why.contains("module.toml")));
    }
}
