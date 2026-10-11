//! The user's trust in modules: who has it, for exactly which contents, and what it allows.
//!
//! Trust is given to a module's **fingerprint**, never to its name or folder. A module whose
//! files change is no longer the thing the user trusted, and is untrusted until the user decides
//! again. The one exception is asked for by the user, module by module: "keep trusting it as I
//! change it", for a module they are writing themselves.
//!
//! The decisions are kept in one readable file in the user's settings. Anything wrong with that
//! file means nothing is trusted: the store fails closed.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::manifest::Permission;
use crate::module::Module;

/// The largest trust file read (bytes).
const MAX_BYTES: u64 = 1024 * 1024;
const VERSION: u32 = 1;

/// One decision to trust.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    /// The fingerprint that was trusted.
    fingerprint: String,
    /// What the module asked for, and so was granted, at that moment.
    permissions: Vec<Permission>,
    /// The user chose to keep trusting it as its files change.
    #[serde(default)]
    follow_changes: bool,
}

#[derive(Serialize, Deserialize)]
struct File {
    version: u32,
    modules: BTreeMap<String, Record>,
}

/// Where a module stands with the user.
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
        let records = file.filter(|f| f.version == VERSION).map(|f| f.modules).unwrap_or_default();
        TrustStore { path: path.to_path_buf(), records }
    }

    /// Where `module` stands.
    pub fn status(&self, module: &Module) -> Status {
        if let Some(problem) = &module.problem {
            return Status::Problem(problem.clone());
        }
        let (Some(manifest), Some(fingerprint)) = (&module.manifest, &module.fingerprint) else {
            return Status::Problem("it could not be read".into());
        };
        match self.records.get(&module.id) {
            None => Status::Untrusted,
            Some(record) if record.fingerprint == *fingerprint || record.follow_changes => {
                // Never more than was granted, and never more than it asks for now.
                Status::Trusted(record.permissions.iter().copied().filter(|p| manifest.permissions.contains(p)).collect())
            }
            Some(_) => Status::Changed,
        }
    }

    /// The user chose to keep trusting this module as its files change.
    pub fn follows_changes(&self, id: &str) -> bool {
        self.records.get(id).is_some_and(|r| r.follow_changes)
    }

    /// Records the user's decision to trust `module` as it is now, with everything it asks for.
    /// Only a sound module can be trusted.
    pub fn trust(&mut self, module: &Module, follow_changes: bool) -> Result<(), String> {
        let (Some(manifest), Some(fingerprint), None) = (&module.manifest, &module.fingerprint, &module.problem) else {
            return Err(format!("{} can't be trusted: {}", module.name(), module.problem.as_deref().unwrap_or("it could not be read")));
        };
        let record = Record { fingerprint: fingerprint.clone(), permissions: manifest.permissions.clone(), follow_changes };
        let before = self.records.insert(module.id.clone(), record);
        self.save().inspect_err(|_| {
            // What is on disk decides: a decision that wasn't saved wasn't made.
            match before {
                Some(old) => self.records.insert(module.id.clone(), old),
                None => self.records.remove(&module.id),
            };
        })
    }

    /// Takes trust back. Fine for a module that had none.
    pub fn revoke(&mut self, id: &str) -> Result<(), String> {
        let Some(before) = self.records.remove(id) else { return Ok(()) };
        self.save().inspect_err(|_| {
            self.records.insert(id.to_string(), before);
        })
    }

    /// Writes the file whole, beside itself first, so a crash can't leave half a file.
    fn save(&self) -> Result<(), String> {
        let failed = |e: std::io::Error| format!("the trusted-modules list could not be saved: {e}");
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir).map_err(failed)?;
        }
        let file = File { version: VERSION, modules: self.records.clone() };
        let text = serde_json::to_string_pretty(&file).map_err(|e| format!("the trusted-modules list could not be saved: {e}"))?;
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
    use crate::module::tests::{MANIFEST, TempDir, sample};
    use crate::module::{load, scan};

    #[test]
    fn a_module_is_untrusted_until_the_user_trusts_it_and_the_decision_is_kept() {
        let t = TempDir::new("trust");
        let module = load(&sample(&t));
        let file = t.0.join("settings/trusted-modules.json");
        let mut store = TrustStore::load(&file);
        assert_eq!(store.status(&module), Status::Untrusted);
        store.trust(&module, false).unwrap();
        assert_eq!(store.status(&module), Status::Trusted(vec![Permission::Read, Permission::Python]));
        assert!(!store.follows_changes("clean-data"));
        // Another run of the app reads the same decision.
        let again = TrustStore::load(&file);
        assert!(again.status(&module).is_trusted());
        // Taking it back, twice.
        store.revoke("clean-data").unwrap();
        store.revoke("clean-data").unwrap();
        assert_eq!(TrustStore::load(&file).status(&module), Status::Untrusted);
    }

    #[test]
    fn changed_files_end_the_trust_until_the_user_decides_again() {
        let t = TempDir::new("changed");
        let dir = sample(&t);
        let mut store = TrustStore::load(&t.0.join("trusted-modules.json"));
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
        // Another module with the same name in another folder has no trust: it is another id.
        t.write("copy/module.toml", &MANIFEST.replace("clean-data", "copy"));
        t.write("copy/panel/index.html", "<h1>Clean Data</h1>");
        t.write("copy/python/clean.py", "def main(data):\n    return data\n");
        assert_eq!(store.status(&load(&t.0.join("copy"))), Status::Untrusted);
    }

    #[test]
    fn following_changes_is_the_users_choice_and_never_grants_more() {
        let t = TempDir::new("follow");
        let dir = sample(&t);
        let mut store = TrustStore::load(&t.0.join("trusted-modules.json"));
        store.trust(&load(&dir), true).unwrap();
        assert!(store.follows_changes("clean-data"));
        t.write("clean-data/python/clean.py", "def main(data):\n    return None\n");
        assert!(store.status(&load(&dir)).is_trusted(), "its own developer's edits don't end the trust");
        // The manifest now asks for more: what was granted stays what was granted.
        t.write("clean-data/module.toml", &MANIFEST.replace("[\"read\", \"python\"]", "[\"read\", \"python\", \"files\", \"network\"]"));
        assert_eq!(store.status(&load(&dir)), Status::Trusted(vec![Permission::Read, Permission::Python]));
        // It asks for less: it gets less.
        t.write("clean-data/module.toml", &MANIFEST.replace("[\"read\", \"python\"]", "[\"read\"]"));
        assert_eq!(store.status(&load(&dir)), Status::Trusted(vec![Permission::Read]));
        // Trusting again without following: the next change ends it.
        store.trust(&load(&dir), false).unwrap();
        t.write("clean-data/python/clean.py", "x = 1\n");
        assert_eq!(store.status(&load(&dir)), Status::Changed);
    }

    #[test]
    fn a_broken_module_cannot_be_trusted() {
        let t = TempDir::new("broken");
        t.write("no-manifest/panel/index.html", "x");
        let module = load(&t.0.join("no-manifest"));
        let file = t.0.join("trusted-modules.json");
        let mut store = TrustStore::load(&file);
        assert!(matches!(store.status(&module), Status::Problem(_)));
        assert!(store.trust(&module, true).is_err());
        assert!(!file.exists(), "nothing was written");
        // A module that breaks after being trusted is a problem, not trusted.
        let dir = sample(&t);
        store.trust(&load(&dir), true).unwrap();
        t.write("clean-data/module.toml", "id = [");
        assert!(matches!(store.status(&load(&dir)), Status::Problem(_)));
    }

    #[test]
    fn a_damaged_or_foreign_trust_file_trusts_nothing() {
        let t = TempDir::new("damaged");
        let module = load(&sample(&t));
        let file = t.0.join("trusted-modules.json");
        let fingerprint = module.fingerprint.clone().unwrap();
        let good = format!(r#"{{"version": 1, "modules": {{"clean-data": {{"fingerprint": "{fingerprint}", "permissions": ["read", "python"]}}}}}}"#);
        fs::write(&file, &good).unwrap();
        assert!(TrustStore::load(&file).status(&module).is_trusted(), "a file as this version writes it is read");
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
            assert!(!TrustStore::load(&file).status(&module).is_trusted(), "{}…", damaged.chars().take(60).collect::<String>());
        }
        // A folder where the file should be: nothing is trusted, and trusting reports the failure.
        fs::remove_file(&file).unwrap();
        fs::create_dir_all(&file).unwrap();
        let mut store = TrustStore::load(&file);
        assert!(store.trust(&module, false).is_err());
        assert_eq!(store.status(&module), Status::Untrusted, "a decision that wasn't saved wasn't made");
    }

    #[test]
    fn every_module_in_a_folder_gets_its_own_status() {
        let t = TempDir::new("several");
        sample(&t);
        t.write("second/module.toml", "id = \"second\"\nname = \"Second\"\nversion = \"1\"\n");
        t.write("third/readme.txt", "no manifest");
        let mut store = TrustStore::load(&t.0.join("trusted-modules.json"));
        let modules = scan(&t.0);
        store.trust(&modules[1], false).unwrap();
        let codes: Vec<(&str, &str)> = modules.iter().map(|m| (m.id.as_str(), store.status(m).code())).collect();
        assert_eq!(codes, [("clean-data", "untrusted"), ("second", "trusted"), ("third", "problem")]);
        assert_eq!(store.status(&modules[1]), Status::Trusted(vec![]), "it asked for nothing");
    }
}
