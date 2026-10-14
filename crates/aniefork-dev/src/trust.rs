//! The user's trust in projects: who has it, for exactly which contents, and what it allows.
//!
//! Trust is given to a project's **fingerprint**, never to its name or folder. A project whose
//! files change is no longer the thing the user trusted, and is untrusted until the user decides
//! again. The one exception is asked for by the user, project by project: "keep trusting it as I
//! change it", for a project they are writing themselves.
//!
//! The decisions are kept in one readable file in the user's settings. Anything wrong with that
//! file means nothing is trusted: the store fails closed.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::manifest::{Address, Manifest, Permission};
use crate::project::Project;

/// The largest trust file read (bytes).
const MAX_BYTES: u64 = 1024 * 1024;
const VERSION: u32 = 1;

/// One decision to trust.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    /// The fingerprint that was trusted.
    fingerprint: String,
    /// What the project asked for, and so was granted, at that moment.
    permissions: Vec<Permission>,
    /// The user chose to keep trusting it as its files change.
    #[serde(default)]
    follow_changes: bool,
    /// The addresses the user turned the network on for, as the project named them then. Empty:
    /// its network is off. It stands only while the project names exactly these.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    network: Vec<String>,
}

/// The addresses a project names, as they are kept and compared.
fn named(list: &[Address]) -> Vec<String> {
    list.iter().map(Address::text).collect()
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    /// By id. A file written while a project was called a module names them `modules`: it
    /// is read as it is, and written back under this name.
    #[serde(alias = "modules")]
    projects: BTreeMap<String, Record>,
}

/// Where a project stands with the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// It is broken and can't be trusted at all; the text says why.
    Problem(String),
    /// The user has not trusted it. It does nothing.
    Untrusted,
    /// The user trusted it, but its files have changed since. It does nothing until trusted again.
    Changed,
    /// The user trusts these contents. It may use what it was granted and still asks for.
    Trusted(Vec<Permission>),
}

impl Status {
    pub fn is_trusted(&self) -> bool {
        matches!(self, Status::Trusted(_))
    }

    /// A word for agents and the saved list: `problem`, `untrusted`, `changed` or `trusted`.
    pub fn code(&self) -> &'static str {
        match self {
            Status::Problem(_) => "problem",
            Status::Untrusted => "untrusted",
            Status::Changed => "changed",
            Status::Trusted(_) => "trusted",
        }
    }
}

/// The user's decisions, as loaded from (and saved to) one file.
#[derive(Clone, Debug)]
pub struct TrustStore {
    path: PathBuf,
    records: BTreeMap<String, Record>,
}

impl TrustStore {
    /// Loads the decisions. A file that is missing, too large, unreadable or of another version
    /// holds none: nothing is trusted.
    pub fn load(path: &Path) -> TrustStore {
        let small = fs::metadata(path).is_ok_and(|m| m.is_file() && m.len() <= MAX_BYTES);
        let file = small.then(|| fs::read_to_string(path).ok()).flatten().and_then(|text| serde_json::from_str::<File>(&text).ok());
        let records = file.filter(|f| f.version == VERSION).map(|f| f.projects).unwrap_or_default();
        TrustStore { path: path.to_path_buf(), records }
    }

    /// Where `project` stands.
    pub fn status(&self, project: &Project) -> Status {
        if let Some(problem) = &project.problem {
            return Status::Problem(problem.clone());
        }
        let (Some(manifest), Some(fingerprint)) = (&project.manifest, &project.fingerprint) else {
            return Status::Problem("it could not be read".into());
        };
        match self.records.get(&project.id) {
            None => Status::Untrusted,
            Some(record) if record.fingerprint == *fingerprint || record.follow_changes => {
                // Never more than was granted, and never more than it asks for now. And the
                // network of a project that names where it would reach is the person's to
                // turn on by itself: trusting it turns nothing on.
                let allowed = |p: &Permission| *p != Permission::Network || Self::network_stands(manifest, record);
                Status::Trusted(record.permissions.iter().copied().filter(|p| manifest.permissions.contains(p)).filter(allowed).collect())
            }
            Some(_) => Status::Changed,
        }
    }

    /// Whether the network a record holds is the network the project asks for now: always, for
    /// a project that names no addresses (the first kit's `network`, which is its page's); for
    /// one that names them, only when the person turned it on for exactly those.
    fn network_stands(manifest: &Manifest, record: &Record) -> bool {
        match &manifest.network {
            None => true,
            Some(list) => !list.is_empty() && record.network == named(list),
        }
    }

    /// Whether `project` names addresses to reach, and the person has turned its network on for
    /// them as they are now.
    pub fn network_on(&self, project: &Project) -> bool {
        let carried = project.manifest.as_ref().is_some_and(|manifest| manifest.network.is_some());
        carried && matches!(self.status(project), Status::Trusted(granted) if granted.contains(&Permission::Network))
    }

    /// Records the user's decision to turn the network of `project` on, for the addresses it
    /// names now, or off. On is for a project that is trusted as it is and asks for it; off is
    /// always fine. Trusting a project again turns it off: it is decided by itself, each time.
    pub fn set_network(&mut self, project: &Project, on: bool) -> Result<(), String> {
        let list = match (&project.manifest, on) {
            (_, false) => Vec::new(),
            (Some(manifest), true) => match (&manifest.network, manifest.permissions.contains(&Permission::Network)) {
                (Some(list), true) if !list.is_empty() => named(list),
                _ => return Err(format!("{} asks for no network.", project.name())),
            },
            (None, true) => return Err(format!("{} could not be read.", project.name())),
        };
        if on && !self.status(project).is_trusted() {
            return Err(format!("{} is not trusted as it is now, so its network stays off.", project.name()));
        }
        let Some(record) = self.records.get_mut(&project.id) else { return Ok(()) };
        let before = std::mem::replace(&mut record.network, list);
        self.save().inspect_err(|_| {
            if let Some(record) = self.records.get_mut(&project.id) {
                record.network = before;
            }
        })
    }

    /// The user chose to keep trusting this project as its files change.
    pub fn follows_changes(&self, id: &str) -> bool {
        self.records.get(id).is_some_and(|r| r.follow_changes)
    }

    /// Records the user's decision to trust `project` as it is now, with everything it asks for.
    /// Only a sound project can be trusted.
    pub fn trust(&mut self, project: &Project, follow_changes: bool) -> Result<(), String> {
        let (Some(manifest), Some(fingerprint), None) = (&project.manifest, &project.fingerprint, &project.problem) else {
            return Err(format!("{} can't be trusted: {}", project.name(), project.problem.as_deref().unwrap_or("it could not be read")));
        };
        // Its network, if it names addresses to reach, is off: that is turned on by itself.
        let record = Record { fingerprint: fingerprint.clone(), permissions: manifest.permissions.clone(), follow_changes, network: Vec::new() };
        let before = self.records.insert(project.id.clone(), record);
        self.save().inspect_err(|_| {
            // What is on disk decides: a decision that wasn't saved wasn't made.
            match before {
                Some(old) => self.records.insert(project.id.clone(), old),
                None => self.records.remove(&project.id),
            };
        })
    }

    /// Takes trust back. Fine for a project that had none.
    pub fn revoke(&mut self, id: &str) -> Result<(), String> {
        let Some(before) = self.records.remove(id) else { return Ok(()) };
        self.save().inspect_err(|_| {
            self.records.insert(id.to_string(), before);
        })
    }

    /// Writes the file whole, beside itself first, so a crash can't leave half a file.
    fn save(&self) -> Result<(), String> {
        let failed = |e: std::io::Error| format!("the trusted-projects list could not be saved: {e}");
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir).map_err(failed)?;
        }
        let file = File { version: VERSION, projects: self.records.clone() };
        let text = serde_json::to_string_pretty(&file).map_err(|e| format!("the trusted-projects list could not be saved: {e}"))?;
        let beside = self.path.with_extension("json.new");
        fs::write(&beside, text).map_err(failed)?;
        fs::rename(&beside, &self.path).map_err(|e| {
            let _ = fs::remove_file(&beside);
            failed(e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::tests::{MANIFEST, TempDir, sample};
    use crate::project::{load, scan};

    #[test]
    fn a_project_is_untrusted_until_the_user_trusts_it_and_the_decision_is_kept() {
        let t = TempDir::new("trust");
        let project = load(&sample(&t));
        let file = t.0.join("settings/trusted-projects.json");
        let mut store = TrustStore::load(&file);
        assert_eq!(store.status(&project), Status::Untrusted);
        store.trust(&project, false).unwrap();
        assert_eq!(store.status(&project), Status::Trusted(vec![Permission::Read, Permission::Python]));
        assert!(!store.follows_changes("clean-data"));
        // Another run of the app reads the same decision.
        let again = TrustStore::load(&file);
        assert!(again.status(&project).is_trusted());
        // Taking it back, twice.
        store.revoke("clean-data").unwrap();
        store.revoke("clean-data").unwrap();
        assert_eq!(TrustStore::load(&file).status(&project), Status::Untrusted);
    }

    #[test]
    fn changed_files_end_the_trust_until_the_user_decides_again() {
        let t = TempDir::new("changed");
        let dir = sample(&t);
        let mut store = TrustStore::load(&t.0.join("trusted-projects.json"));
        store.trust(&load(&dir), false).unwrap();
        t.write("clean-data/python/clean.py", "import os\n");
        assert_eq!(store.status(&load(&dir)), Status::Changed);
        // Putting the file back is the trusted contents again.
        t.write("clean-data/python/clean.py", "def main(data):\n    return data\n");
        assert!(store.status(&load(&dir)).is_trusted());
        // A new file, then deciding again.
        t.write("clean-data/python/more.py", "");
        assert_eq!(store.status(&load(&dir)), Status::Changed);
        store.trust(&load(&dir), false).unwrap();
        assert!(store.status(&load(&dir)).is_trusted());
        // Another project with the same name in another folder has no trust: it is another id.
        t.write("copy/project.toml", &MANIFEST.replace("clean-data", "copy"));
        t.write("copy/panel/index.html", "<h1>Clean Data</h1>");
        t.write("copy/python/clean.py", "def main(data):\n    return data\n");
        assert_eq!(store.status(&load(&t.0.join("copy"))), Status::Untrusted);
    }

    #[test]
    fn a_projects_network_is_turned_on_by_itself_for_the_addresses_it_names() {
        let t = TempDir::new("trust-network");
        let dir = t.0.join("projects/net");
        let write = |allow: &str| {
            t.write(
                "projects/net/project.toml",
                &format!("id = \"net\"\nname = \"Net\"\nversion = \"1\"\nkit = \"c\"\npermissions = [\"read\", \"network\"]\n[c]\nsources = [\"src\"]\n[network]\nallow = [{allow}]\n"),
            );
            t.write("projects/net/src/main.c", "int main(void) { return 0; }\n");
            load(&dir)
        };
        let file = t.0.join("trusted.json");
        let project = write("\"example.org:26000\"");
        assert!(project.is_sound(), "{:?}", project.problem);
        let mut store = TrustStore::load(&file);
        // Not trusted: its network can't be turned on, and is not on.
        assert!(store.set_network(&project, true).unwrap_err().contains("not trusted as it is now"));
        // Trusting it grants what it asks, and not the network: that is off until turned on.
        store.trust(&project, false).unwrap();
        assert_eq!(store.status(&project), Status::Trusted(vec![Permission::Read]));
        assert!(!store.network_on(&project));
        store.set_network(&project, true).unwrap();
        assert_eq!(store.status(&project), Status::Trusted(vec![Permission::Read, Permission::Network]));
        assert!(store.network_on(&project) && TrustStore::load(&file).network_on(&project), "the decision is kept");
        // Off again; and trusting it again turns it off, each time.
        store.set_network(&project, false).unwrap();
        assert!(!store.network_on(&project) && !TrustStore::load(&file).network_on(&project));
        store.set_network(&project, true).unwrap();
        store.trust(&project, false).unwrap();
        assert!(!store.network_on(&project));
        // The person follows its changes, and turns its network on for one address. The project
        // then names another: it is still trusted, and its network is off again.
        store.trust(&project, true).unwrap();
        store.set_network(&project, true).unwrap();
        let wider = write("\"example.org:26000\", \"192.168.1.1:80\"");
        assert_eq!(store.status(&wider), Status::Trusted(vec![Permission::Read]), "what was turned on was the one address");
        assert!(!store.network_on(&wider));
        // A project that asks for none has none to turn on; turning off is always fine.
        let plain = load(&sample(&t));
        store.trust(&plain, false).unwrap();
        assert!(store.set_network(&plain, true).unwrap_err().contains("asks for no network"));
        store.set_network(&plain, false).unwrap();
        // The first kit's `network` is its page's, with no list: trusting grants it, as before.
        t.write("projects/web/project.toml", "id = \"web\"\nname = \"Web\"\nversion = \"1\"\npermissions = [\"network\"]\n");
        let web = load(&t.0.join("projects/web"));
        store.trust(&web, false).unwrap();
        assert_eq!(store.status(&web), Status::Trusted(vec![Permission::Network]));
        assert!(!store.network_on(&web), "it names no addresses: nothing is carried for it");
    }

    #[test]
    fn following_changes_is_the_users_choice_and_never_grants_more() {
        let t = TempDir::new("follow");
        let dir = sample(&t);
        let mut store = TrustStore::load(&t.0.join("trusted-projects.json"));
        store.trust(&load(&dir), true).unwrap();
        assert!(store.follows_changes("clean-data"));
        t.write("clean-data/python/clean.py", "def main(data):\n    return None\n");
        assert!(store.status(&load(&dir)).is_trusted(), "its own developer's edits don't end the trust");
        // The manifest now asks for more: what was granted stays what was granted.
        t.write("clean-data/project.toml", &MANIFEST.replace("[\"read\", \"python\"]", "[\"read\", \"python\", \"files\", \"network\"]"));
        assert_eq!(store.status(&load(&dir)), Status::Trusted(vec![Permission::Read, Permission::Python]));
        // It asks for less: it gets less.
        t.write("clean-data/project.toml", &MANIFEST.replace("[\"read\", \"python\"]", "[\"read\"]"));
        assert_eq!(store.status(&load(&dir)), Status::Trusted(vec![Permission::Read]));
        // Trusting again without following: the next change ends it.
        store.trust(&load(&dir), false).unwrap();
        t.write("clean-data/python/clean.py", "x = 1\n");
        assert_eq!(store.status(&load(&dir)), Status::Changed);
    }

    #[test]
    fn a_broken_project_cannot_be_trusted() {
        let t = TempDir::new("broken");
        t.write("no-manifest/panel/index.html", "x");
        let project = load(&t.0.join("no-manifest"));
        let file = t.0.join("trusted-projects.json");
        let mut store = TrustStore::load(&file);
        assert!(matches!(store.status(&project), Status::Problem(_)));
        assert!(store.trust(&project, true).is_err());
        assert!(!file.exists(), "nothing was written");
        // A project that breaks after being trusted is a problem, not trusted.
        let dir = sample(&t);
        store.trust(&load(&dir), true).unwrap();
        t.write("clean-data/project.toml", "id = [");
        assert!(matches!(store.status(&load(&dir)), Status::Problem(_)));
    }

    #[test]
    fn a_damaged_or_foreign_trust_file_trusts_nothing() {
        let t = TempDir::new("damaged");
        let project = load(&sample(&t));
        let file = t.0.join("trusted-projects.json");
        let fingerprint = project.fingerprint.clone().unwrap();
        let good =
            format!(r#"{{"version": 1, "projects": {{"clean-data": {{"fingerprint": "{fingerprint}", "permissions": ["read", "python"]}}}}}}"#);
        fs::write(&file, &good).unwrap();
        assert!(TrustStore::load(&file).status(&project).is_trusted(), "a file as this version writes it is read");
        // So is one written while a project was called a module: the person's decisions stand,
        // and the next decision writes the file back under the name of now.
        fs::write(&file, good.replace("\"projects\"", "\"modules\"")).unwrap();
        let mut kept = TrustStore::load(&file);
        assert!(kept.status(&project).is_trusted(), "a file from before the name changed is read");
        kept.trust(&project, true).unwrap();
        let written = fs::read_to_string(&file).unwrap();
        assert!(written.contains("\"projects\"") && !written.contains("\"modules\""), "{written}");
        assert!(TrustStore::load(&file).follows_changes("clean-data"));
        fs::write(&file, &good).unwrap();
        for damaged in [
            String::new(),
            "not json".to_string(),
            "[]".to_string(),
            good.replace("\"version\": 1", "\"version\": 2"),
            good.replace("\"python\"", "\"everything\""),
            good.replace(&fingerprint, "0000"),
            good.replace("\"clean-data\"", "\"other\""),
            format!("{good}{}", " ".repeat(MAX_BYTES as usize)),
        ] {
            fs::write(&file, &damaged).unwrap();
            assert!(!TrustStore::load(&file).status(&project).is_trusted(), "{}…", damaged.chars().take(60).collect::<String>());
        }
        // A folder where the file should be: nothing is trusted, and trusting reports the failure.
        fs::remove_file(&file).unwrap();
        fs::create_dir_all(&file).unwrap();
        let mut store = TrustStore::load(&file);
        assert!(store.trust(&project, false).is_err());
        assert_eq!(store.status(&project), Status::Untrusted, "a decision that wasn't saved wasn't made");
    }

    #[test]
    fn every_project_in_a_folder_gets_its_own_status() {
        let t = TempDir::new("several");
        sample(&t);
        t.write("second/project.toml", "id = \"second\"\nname = \"Second\"\nversion = \"1\"\n");
        t.write("third/readme.txt", "no manifest");
        let mut store = TrustStore::load(&t.0.join("trusted-projects.json"));
        let projects = scan(&t.0);
        store.trust(&projects[1], false).unwrap();
        let codes: Vec<(&str, &str)> = projects.iter().map(|m| (m.id.as_str(), store.status(m).code())).collect();
        assert_eq!(codes, [("clean-data", "untrusted"), ("second", "trusted"), ("third", "problem")]);
        assert_eq!(store.status(&projects[1]), Status::Trusted(vec![]), "it asked for nothing");
    }
}
