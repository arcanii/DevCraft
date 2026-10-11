//! Modules on disk: finding them, reading them, and fingerprinting exactly what they contain.
//!
//! A module is a folder in the user's `modules` folder. Its **fingerprint** is a SHA-256 over
//! every file in it (each file's path, length and bytes, in a fixed order). Trust is given to a
//! fingerprint, so the fingerprint has to cover everything that could ever be served or run:
//! a module containing anything that isn't a plain file or folder (a link, which could point
//! outside it) is not fingerprinted at all, and so can't be trusted.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::manifest::{self, Manifest};
use crate::sha256::Sha256;

/// The most modules listed from one folder.
pub const MAX_MODULES: usize = 500;
/// The most files a module may hold.
pub const MAX_FILES: usize = 2_000;
/// The most bytes a module may hold in all.
pub const MAX_BYTES: u64 = 64 * 1024 * 1024;
/// How deep a module's folders may nest.
const MAX_DEPTH: usize = 16;

/// A module as found on disk. It may be broken: then `problem` says how, and it can't be trusted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    /// The folder's name. For a sound module it is also the manifest's id.
    pub id: String,
    pub dir: PathBuf,
    pub manifest: Option<Manifest>,
    /// 64 hexadecimal digits over the module's whole contents; `None` when they couldn't be read.
    pub fingerprint: Option<String>,
    /// Every file in it, as paths inside the module with `/` separators, sorted.
    pub files: Vec<String>,
    /// Why it can't be used, in words for the user.
    pub problem: Option<String>,
}

impl Module {
    /// Sound enough to be trusted: a valid manifest and a fingerprint.
    pub fn is_sound(&self) -> bool {
        self.problem.is_none() && self.manifest.is_some() && self.fingerprint.is_some()
    }

    /// The name to show: the manifest's, or the folder's when there is no valid manifest.
    pub fn name(&self) -> &str {
        self.manifest.as_ref().map_or(self.id.as_str(), |m| m.name.as_str())
    }
}

/// Lists a module's files (paths inside it), refusing anything that isn't a plain file or folder.
fn walk(dir: &Path, prefix: &str, depth: usize, files: &mut Vec<(String, u64)>) -> Result<(), String> {
    if depth > MAX_DEPTH {
        return Err(format!("its folders nest more than {MAX_DEPTH} deep"));
    }
    let entries = fs::read_dir(dir).map_err(|e| format!("a folder in it can't be read: {e}"))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("a folder in it can't be read: {e}"))?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else { return Err("it has a file whose name isn't text".into()) };
        let path = if prefix.is_empty() { name.to_string() } else { format!("{prefix}/{name}") };
        // `file_type` doesn't follow links: a link is neither a file nor a folder here.
        let kind = entry.file_type().map_err(|e| format!("{path} can't be read: {e}"))?;
        if kind.is_dir() {
            walk(&entry.path(), &path, depth + 1, files)?;
        } else if kind.is_file() {
            let size = entry.metadata().map_err(|e| format!("{path} can't be read: {e}"))?.len();
            files.push((path, size));
            if files.len() > MAX_FILES {
                return Err(format!("it holds more than {MAX_FILES} files"));
            }
        } else {
            return Err(format!("{path} is a link or a special file, which modules may not contain"));
        }
    }
    Ok(())
}

/// The fingerprint of the listed files: each one's path, length and bytes, in the list's order.
fn fingerprint(dir: &Path, files: &[(String, u64)]) -> Result<String, String> {
    let mut hash = Sha256::new();
    let mut total = 0u64;
    let mut buffer = vec![0u8; 64 * 1024];
    for (path, _) in files {
        let mut file = fs::File::open(dir.join(path)).map_err(|e| format!("{path} can't be read: {e}"))?;
        let mut length = 0u64;
        let mut contents = Sha256::new();
        loop {
            let n = file.read(&mut buffer).map_err(|e| format!("{path} can't be read: {e}"))?;
            if n == 0 {
                break;
            }
            length = length.saturating_add(n as u64);
            total = total.saturating_add(n as u64);
            if total > MAX_BYTES {
                return Err(format!("it holds more than {} MB", MAX_BYTES / (1024 * 1024)));
            }
            contents.update(buffer.get(..n).unwrap_or(&[]));
        }
        // Path, a separator no path can contain, the length as read, then the contents' digest:
        // no two different sets of files can produce the same stream.
        hash.update(path.as_bytes());
        hash.update(&[0]);
        hash.update(&length.to_be_bytes());
        hash.update(&contents.finish());
    }
    Ok(hash.hex())
}

/// Reads the module in `dir`. It never fails: a module that can't be read is returned with its
/// `problem` set.
pub fn load(dir: &Path) -> Module {
    let id = dir.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
    let mut module = Module { id: id.clone(), dir: dir.to_path_buf(), manifest: None, fingerprint: None, files: vec![], problem: None };
    let mut files = Vec::new();
    if let Err(problem) = walk(dir, "", 0, &mut files) {
        module.problem = Some(problem);
        return module;
    }
    files.sort();
    module.files = files.iter().map(|(path, _)| path.clone()).collect();
    match fingerprint(dir, &files) {
        Ok(f) => module.fingerprint = Some(f),
        Err(problem) => {
            module.problem = Some(problem);
            return module;
        }
    }
    let Some((_, size)) = files.iter().find(|(path, _)| path == manifest::FILE) else {
        module.problem = Some(format!("it has no {}", manifest::FILE));
        return module;
    };
    if *size > manifest::MAX_BYTES {
        module.problem = Some(format!("its {} is larger than {} KB", manifest::FILE, manifest::MAX_BYTES / 1024));
        return module;
    }
    let checked = fs::read_to_string(dir.join(manifest::FILE))
        .map_err(|e| format!("its {} can't be read: {e}", manifest::FILE))
        .and_then(|text| manifest::parse(&text, &id));
    match checked {
        Ok(m) => {
            // What the manifest points at has to be in the module.
            let missing_panel = m.panel.as_ref().filter(|entry| !module.files.contains(entry));
            let missing_scripts = m.python.as_ref().filter(|folder| !module.files.iter().any(|f| f.starts_with(&format!("{folder}/"))));
            if let Some(entry) = missing_panel {
                module.problem = Some(format!("its panel, {entry}, is not in it"));
            } else if let Some(folder) = missing_scripts {
                module.problem = Some(format!("its scripts folder, {folder}, is empty or not in it"));
            }
            module.manifest = Some(m);
        }
        Err(problem) => module.problem = Some(problem),
    }
    module
}

/// Every module in the user's modules folder, by folder name. A folder that isn't there yet holds
/// no modules.
pub fn scan(modules_dir: &Path) -> Vec<Module> {
    let Ok(entries) = fs::read_dir(modules_dir) else { return vec![] };
    let mut dirs: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .filter(|e| e.file_name().to_str().is_some_and(|n| !n.starts_with('.')))
        .map(|e| e.path())
        .collect();
    dirs.sort();
    dirs.truncate(MAX_MODULES);
    dirs.iter().map(|dir| load(dir)).collect()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A folder of its own for one test, removed when the test ends.
    pub(crate) struct TempDir(pub PathBuf);

    impl TempDir {
        pub(crate) fn new(tag: &str) -> TempDir {
            static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
            let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("gridcraft-dev-{}-{tag}-{n}", std::process::id()));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir_all(&dir).unwrap();
            TempDir(dir)
        }

        pub(crate) fn write(&self, path: &str, contents: &str) {
            let file = self.0.join(path);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(file, contents).unwrap();
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    pub(crate) const MANIFEST: &str = "id = \"clean-data\"\nname = \"Clean Data\"\nversion = \"0.1.0\"\npermissions = [\"read\", \"python\"]\n\n[panel]\nentry = \"panel/index.html\"\n\n[python]\nscripts = \"python\"\n";

    /// A sound module in `modules/clean-data`.
    pub(crate) fn sample(t: &TempDir) -> PathBuf {
        t.write("clean-data/module.toml", MANIFEST);
        t.write("clean-data/panel/index.html", "<h1>Clean Data</h1>");
        t.write("clean-data/python/clean.py", "def main(data):\n    return data\n");
        t.0.join("clean-data")
    }

    #[test]
    fn reads_a_module_and_lists_its_files() {
        let t = TempDir::new("read");
        let m = load(&sample(&t));
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!((m.id.as_str(), m.name()), ("clean-data", "Clean Data"));
        assert_eq!(m.files, ["module.toml", "panel/index.html", "python/clean.py"]);
        assert_eq!(m.fingerprint.as_ref().map(String::len), Some(64));
        assert_eq!(scan(&t.0), [m]);
        assert!(scan(&t.0.join("nowhere")).is_empty(), "a folder that isn't there holds no modules");
    }

    #[test]
    fn the_fingerprint_follows_every_file() {
        let t = TempDir::new("fingerprint");
        let dir = sample(&t);
        let print = || load(&dir).fingerprint.unwrap();
        let original = print();
        assert_eq!(print(), original, "the same contents give the same fingerprint");
        // A changed byte, a longer file, a new file, a renamed file, a changed manifest.
        t.write("clean-data/python/clean.py", "def main(data):\n    return datA\n");
        let changed = print();
        assert_ne!(changed, original);
        t.write("clean-data/python/clean.py", "def main(data):\n    return data\n");
        assert_eq!(print(), original, "and back again");
        t.write("clean-data/python/extra.py", "");
        assert_ne!(print(), original, "even an empty new file");
        fs::remove_file(dir.join("python/extra.py")).unwrap();
        fs::rename(dir.join("panel/index.html"), dir.join("panel/index2.html")).unwrap();
        assert_ne!(print(), original, "a renamed file");
        fs::rename(dir.join("panel/index2.html"), dir.join("panel/index.html")).unwrap();
        t.write("clean-data/module.toml", &MANIFEST.replace("0.1.0", "0.1.1"));
        assert_ne!(print(), original, "the manifest is part of it");
        // Moving bytes from the end of one file to the start of the next changes it too.
        let s = TempDir::new("split");
        s.write("m/a", "ab");
        s.write("m/b", "c");
        let one = load(&s.0.join("m")).fingerprint;
        s.write("m/a", "a");
        s.write("m/b", "bc");
        assert_ne!(load(&s.0.join("m")).fingerprint, one);
    }

    #[test]
    fn a_broken_module_says_what_is_wrong_and_is_not_sound() {
        let t = TempDir::new("broken");
        t.write("no-manifest/panel/index.html", "x");
        t.write("bad-manifest/module.toml", "id = [");
        t.write("wrong-id/module.toml", &MANIFEST.replace("[panel]\nentry = \"panel/index.html\"\n\n[python]\nscripts = \"python\"\n", ""));
        t.write("no-panel/module.toml", &MANIFEST.replace("clean-data", "no-panel").replace("[python]\nscripts = \"python\"\n", ""));
        t.write("no-scripts/module.toml", &MANIFEST.replace("clean-data", "no-scripts"));
        t.write("no-scripts/panel/index.html", "x");
        t.write(".hidden/module.toml", MANIFEST);
        t.write("just-a-file.txt", "not a module");
        let found = scan(&t.0);
        let ids: Vec<&str> = found.iter().map(|m| m.id.as_str()).collect();
        assert_eq!(ids, ["bad-manifest", "no-manifest", "no-panel", "no-scripts", "wrong-id"], "folders only, hidden ones skipped, sorted");
        for m in &found {
            assert!(!m.is_sound() && m.problem.as_ref().is_some_and(|p| !p.is_empty()), "{}: {:?}", m.id, m.problem);
        }
        let problem = |id: &str| found.iter().find(|m| m.id == id).and_then(|m| m.problem.clone()).unwrap();
        assert!(problem("no-manifest").contains("module.toml"));
        assert!(problem("wrong-id").contains("folder"));
        assert!(problem("no-panel").contains("panel/index.html"));
        assert!(problem("no-scripts").contains("scripts folder"));
    }

    #[cfg(unix)]
    #[test]
    fn a_module_containing_a_link_is_refused() {
        let t = TempDir::new("link");
        let dir = sample(&t);
        t.write("outside.py", "import os");
        std::os::unix::fs::symlink(t.0.join("outside.py"), dir.join("python/outside.py")).unwrap();
        let m = load(&dir);
        assert!(!m.is_sound() && m.fingerprint.is_none());
        assert!(m.problem.unwrap().contains("link"));
    }
}
