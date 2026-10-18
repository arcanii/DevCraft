//! The Development subsystem of the aniefork fork of GridCraft: **projects** a user installs to
//! give GridCraft new abilities (web panels, Python). Designed in `docs/decisions-aniefork.md`
//! (ADR-0001).
//!
//! It finds projects, reads their manifests, fingerprints exactly what they contain, keeps the
//! user's decisions to trust them, and runs a trusted project's Python scripts. Whatever runs or
//! serves anything of a project asks first, here, whether the project is trusted and what it was
//! granted.
//!
//! - [`kit`]: the development kits that are loaded (PythonCraft, which is most
//!   of this crate, and the C Development Kit, which builds a project's C and runs it in the
//!   project's panel), and what a project is for: the code of one workbook, or any workbook.
//! - [`manifest`]: `project.toml`, and the permissions a project can ask for.
//! - [`project`]: projects on disk and their fingerprints.
//! - [`trust`]: the user's decisions, bound to fingerprints.
//! - [`python`]: running a trusted project's scripts in a process of their own.
//! - [`c`]: the C kit's recipe, and building a trusted project's C as WebAssembly with the
//!   person's own compiler.
//! - [`c_program`]: the loader a C project's page runs its program with.
//! - [`bridge`]: serving a trusted project's panel to the browser (and, for a C project, the
//!   program built from it and its loader), carrying its calls, and telling it what happens in
//!   GridCraft.
//! - [`console`]: the Developer console: what projects printed and logged, and what failed.
//! - [`agents`]: the Development server, a second MCP server for agents that help with
//!   development work, as far as the user lets them go.
//! - [`highlight`]: how a project's files are read for showing, from what its kit describes.
//! - [`starter`]: "New Project".
//! - [`showcase`]: a project whose panel is a Svelte app, to show a web toolkit at work; and
//!   every kit's showcases, among them the C kit's [`triangle`] and [`mandelbrot`].
//! - [`setup`]: Doom, Quake and GLQuake as projects of the C kit, downloaded, pinned and checked,
//!   on the person's Set Up alone ([`setup_fetch`]: the HTTPS it downloads with).
//! - [`paths`], [`settings`]: where projects, decisions and the subsystem's settings are kept.
//! - [`vba`]: looking inside a workbook's VBA project, to read it. Nothing of it is ever run.
//! - [`rebuilt`]: a project's note of what it rebuilds of a workbook's macros.
//!
//! It has no dependency on the UI toolkit or on the rest of GridCraft, so that it can be offered
//! upstream as a piece of its own.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

pub mod agents;
pub mod bridge;
pub mod c;
pub mod c_program;
pub mod console;
pub mod gl1;
pub mod gles3;
pub mod highlight;
#[cfg(not(target_arch = "wasm32"))]
mod http;
pub mod kit;
#[cfg(not(target_arch = "wasm32"))]
pub mod kitpack;
pub mod mandelbrot;
pub mod manifest;
pub mod net;
pub mod paths;
pub mod project;
pub mod python;
pub mod rebuilt;
pub mod settings;
pub mod setup;
#[cfg(not(target_arch = "wasm32"))]
pub mod setup_fetch;
mod setup_files;
pub mod sha256;
pub mod showcase;
mod showcase_files;
pub mod starter;
pub mod triangle;
pub mod trust;
pub mod vba;

use serde_json::{Value as Json, json};

/// DevCraft's own version: what the fork adds to GridCraft, numbered apart from GridCraft's
/// version, which is upstream's (the owner, 2026-10-14: "a devCraft marketing version different
/// than GridCraft (start at 0.1.0)"). It is changed by hand, when the owner says a version is
/// made; the Development window and Help ▸ About show it.
pub const DEVCRAFT_VERSION: &str = "0.1.0";

pub use manifest::{Manifest, Permission};
pub use project::Project;
pub use trust::{Status, TrustStore};

/// A project and where it stands, as the Projects dialog shows it and `developer.projects` reports it.
/// What `network` means for a project that names the addresses it would reach.
pub const CARRIED_NETWORK: &str = "Send and receive over the network, through GridCraft, to the addresses it names. Off until you turn it on";

pub fn describe(project: &Project, store: &TrustStore) -> Json {
    let status = store.status(project);
    let manifest = project.manifest.as_ref();
    // The addresses it would reach, for a project that names them: its `network` is then
    // GridCraft's to carry, and means another thing than the first kit's.
    let carried = manifest.and_then(|m| m.network.as_ref());
    let means = |p: &Permission| if *p == Permission::Network && carried.is_some() { CARRIED_NETWORK } else { p.meaning() };
    let asks: Vec<Json> = manifest
        .map(|m| m.permissions.iter().map(|p| json!({"permission": p.code(), "means": means(p), "unlimited": p.is_unlimited()})).collect())
        .unwrap_or_default();
    let granted: Vec<&str> = match &status {
        Status::Trusted(permissions) => permissions.iter().map(|p| p.code()).collect(),
        _ => vec![],
    };
    json!({
        "id": project.id,
        "name": project.name(),
        "version": manifest.map(|m| m.version.as_str()).unwrap_or(""),
        "author": manifest.map(|m| m.author.as_str()).unwrap_or(""),
        "description": manifest.map(|m| m.description.as_str()).unwrap_or(""),
        // The kit it is listed under: the one it names, or the first when it can't be placed.
        "kit": kit::of_in(project.dir.parent().unwrap_or(&project.dir), manifest).id,
        // What it is for: the code of the workbook it names, or any workbook.
        "kind": kit::ProjectKind::of(manifest).code(),
        "workbook": manifest.and_then(|m| m.workbook.as_deref()),
        // What it says it rebuilds of a workbook's macros: its own word.
        "rebuilt": rebuilt::describe(project),
        "status": status.code(),
        "problem": match &status { Status::Problem(why) => Json::String(why.clone()), _ => Json::Null },
        "asks": asks,
        // Where it would reach and whether the person turned that on: for a project that names addresses.
        "network": carried.map(|list| json!({
            "addresses": list.iter().map(|address| json!({"text": address.text(), "local": address.is_local()})).collect::<Vec<_>>(),
            "listen": manifest.and_then(|m| m.listen),
            "on": store.network_on(project),
        })),
        // Whether its page may talk to the internet: the first kit's `network`, never a carried one.
        "pageNetwork": granted.contains(&"network") && carried.is_none(),
        "granted": granted,
        "followsChanges": store.follows_changes(&project.id),
        "panel": manifest.and_then(|m| m.panel.as_deref()),
        "python": manifest.and_then(|m| m.python.as_deref()),
        // Whether it has a recipe: something of it is built.
        "builds": manifest.is_some_and(|m| m.c.is_some()),
        "files": project.files,
        "fingerprint": project.fingerprint,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::tests::{TempDir, sample};

    #[test]
    fn a_project_that_names_where_it_would_reach_is_described_with_that_and_whether_it_is_on() {
        let t = TempDir::new("describe-network");
        t.write("net/project.toml", "id = \"net\"\nname = \"Net\"\nversion = \"1\"\nkit = \"c\"\npermissions = [\"read\", \"network\"]\n[panel]\nentry = \"panel/index.html\"\n[c]\nsources = [\"src\"]\n[network]\nallow = [\"example.org:26000\", \"192.168.1.20:26000\"]\n");
        t.write("net/panel/index.html", "<p>net</p>");
        t.write("net/src/main.c", "int main(void) { return 0; }\n");
        let found = project::load(&t.0.join("net"));
        assert!(found.is_sound(), "{:?}", found.problem);
        let mut store = TrustStore::load(&t.0.join("trusted.json"));
        let said = describe(&found, &store);
        // Its `network` is GridCraft's to carry: said so where it asks, and with where to.
        assert_eq!(said["asks"][1], json!({"permission": "network", "means": CARRIED_NETWORK, "unlimited": false}));
        assert_eq!(
            said["network"],
            json!({"addresses": [{"text": "example.org:26000", "local": false}, {"text": "192.168.1.20:26000", "local": true}], "listen": null, "on": false})
        );
        // Trusted: what it asks is granted, and not the network. Turned on: it is, and its page
        // is still not let out.
        store.trust(&found, false).unwrap();
        let trusted = describe(&found, &store);
        assert_eq!((&trusted["granted"], &trusted["network"]["on"], &trusted["pageNetwork"]), (&json!(["read"]), &json!(false), &json!(false)));
        store.set_network(&found, true).unwrap();
        let on = describe(&found, &store);
        assert_eq!((&on["granted"], &on["network"]["on"], &on["pageNetwork"]), (&json!(["read", "network"]), &json!(true), &json!(false)));
        // The first kit's `network` is its page's: no list, and granted with trust.
        t.write("web/project.toml", "id = \"web\"\nname = \"Web\"\nversion = \"1\"\npermissions = [\"network\"]\n");
        let web = project::load(&t.0.join("web"));
        store.trust(&web, false).unwrap();
        let web = describe(&web, &store);
        assert_eq!(
            (&web["network"], &web["pageNetwork"], &web["asks"][0]["means"]),
            (&Json::Null, &json!(true), &json!(Permission::Network.meaning()))
        );
    }

    #[test]
    fn a_project_is_described_with_what_it_asks_for_and_what_it_was_granted() {
        let t = TempDir::new("describe");
        let m = project::load(&sample(&t));
        let mut store = TrustStore::load(&t.0.join("trusted-projects.json"));
        let before = describe(&m, &store);
        assert_eq!(
            (before["id"].as_str(), before["name"].as_str(), before["status"].as_str()),
            (Some("clean-data"), Some("Clean Data"), Some("untrusted"))
        );
        assert_eq!(before["granted"], json!([]), "nothing is granted to an untrusted project");
        let asks: Vec<&str> = before["asks"].as_array().unwrap().iter().filter_map(|a| a["permission"].as_str()).collect();
        assert_eq!(asks, ["read", "python"]);
        assert_eq!(before["asks"][1]["unlimited"], json!(true), "Python is marked as the one that can't be limited");
        assert!(before["asks"].as_array().unwrap().iter().all(|a| a["means"].as_str().is_some_and(|text| !text.is_empty())));
        assert_eq!(before["files"], json!(["panel/index.html", "project.toml", "python/clean.py"]));
        assert_eq!((&before["kind"], &before["workbook"], &before["rebuilt"]), (&json!("general"), &Json::Null, &Json::Null));
        assert_eq!(before["kit"], json!("pythoncraft"), "a project that names no kit is the first kit's");
        // A project of the C kit says so; one of a kit this build lacks is listed under the first, as a problem.
        t.write("blink/project.toml", "id = \"blink\"\nname = \"Blink\"\nversion = \"1\"\nkit = \"c\"\n");
        let blink = describe(&project::load(&t.0.join("blink")), &store);
        assert_eq!(
            (&blink["kit"], &blink["status"], &blink["panel"], &blink["python"]),
            (&json!("c"), &json!("untrusted"), &Json::Null, &Json::Null)
        );
        assert_eq!((&blink["builds"], &before["builds"]), (&json!(false), &json!(false)), "neither has a recipe");
        t.write("rusty/project.toml", "id = \"rusty\"\nname = \"Rusty\"\nversion = \"1\"\nkit = \"rust\"\n");
        let rusty = describe(&project::load(&t.0.join("rusty")), &store);
        assert_eq!((&rusty["kit"], &rusty["status"]), (&json!("pythoncraft"), &json!("problem")));
        assert!(rusty["problem"].as_str().is_some_and(|why| why.contains("the kit \"rust\"")));
        store.trust(&m, false).unwrap();
        let after = describe(&m, &store);
        assert_eq!((after["status"].as_str(), &after["granted"]), (Some("trusted"), &json!(["read", "python"])));
        // One that names a workbook is that workbook's code, and its note is reported with it.
        t.write("budget/project.toml", "id = \"budget\"\nname = \"Budget\"\nversion = \"1\"\nworkbook = \"Budget.xlsm\"\n");
        t.write("budget/rebuilt.toml", "[[procedure]]\nmodule = \"Totals\"\nname = \"AddUp\"\nstate = \"left\"\n");
        let code = describe(&project::load(&t.0.join("budget")), &store);
        assert_eq!((&code["kind"], &code["workbook"], &code["rebuilt"]["left"]), (&json!("workbook"), &json!("Budget.xlsm"), &json!(1)));
        // A broken project says why.
        t.write("broken/readme.txt", "x");
        let broken = describe(&project::load(&t.0.join("broken")), &store);
        assert_eq!(broken["status"], json!("problem"));
        assert!(broken["problem"].as_str().is_some_and(|why| why.contains("project.toml")));
    }
}
