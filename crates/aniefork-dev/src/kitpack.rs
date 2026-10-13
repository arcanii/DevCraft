//! A development kit as a package file: read, loaded into the settings folder, and removed.
//!
//! The owner decided (2026-10-13) that a kit is loaded from a package file, and how: a zip
//! named `.gridkit` with a `kit.toml` at its root, for a kit that **describes and runs nothing
//! of its own**. Its projects are projects of one of the kits GridCraft is built with (its
//! `base`), with that kit's limits on what they may ask; the package brings the kit's words,
//! and a starter: the files a new project of it begins with (`starter/`).
//!
//! Nothing in a package is run. Loading one unpacks it into a folder of its own beside the
//! projects folder (`kits/<id>/`), where [`crate::kit`] finds it; a project made from its
//! starter is a project like any other, which does nothing until the person trusts it.
//!
//! A package is somebody else's file, so reading it is bounded: so many files, so many
//! bytes each and in all, counted as they are unpacked (a zip can say one size and hold
//! another); and no name in it that could leave the folder it is unpacked into.
//!
//! Loading and removing are the person's to do, by hand, each confirmed by the operating
//! system's own question: whoever calls [`install`] and [`remove`] has seen to that.

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::kit::{self, Described, Kit};
use crate::sha256::Sha256;

/// The ending of a kit package's name.
pub const EXTENSION: &str = "gridkit";
/// The largest package read (bytes), the most files in one, the largest one file unpacks to,
/// and the most they unpack to in all.
pub const MAX_PACKAGE: u64 = 16 * 1024 * 1024;
pub const MAX_FILES: usize = 400;
pub const MAX_FILE: u64 = 4 * 1024 * 1024;
pub const MAX_UNPACKED: u64 = 32 * 1024 * 1024;
/// The longest name of a file in a package, and how deep it may lie.
const MAX_NAME: usize = 200;
const MAX_DEPTH: usize = 8;
/// Where a package keeps the files a new project begins with.
pub const STARTER: &str = "starter/";
/// What a package may hold at its root beside `kit.toml` and its starter.
const ALSO: [&str; 3] = ["README.md", "LICENSE", "LICENSE.txt"];

/// A package as it was read: what the kit says of itself, and the files it holds.
#[derive(Clone, Debug)]
pub struct Package {
    pub said: Described,
    /// Each file's name in the package, with `/` between folders, and what it holds.
    pub files: Vec<(String, Vec<u8>)>,
    /// What the files unpack to, in bytes.
    pub bytes: u64,
    /// 64 hexadecimal digits over every file's name and contents.
    pub fingerprint: String,
}

impl Package {
    /// How many files its starter has.
    pub fn starter(&self) -> usize {
        self.files.iter().filter(|(name, _)| name.starts_with(STARTER)).count()
    }
}

/// Whether a name in a package is one a file of a kit can have: plain letters, digits, dots,
/// hyphens and underscores, in folders, with nothing that leaves the folder it is unpacked into.
fn sound_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= MAX_NAME
        && name.split('/').count() <= MAX_DEPTH
        && name.split('/').all(|part| {
            !part.is_empty()
                && part != "."
                && part != ".."
                && !part.starts_with('.')
                && !part.ends_with('.')
                && part.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
        })
}

/// Whether a file of this name has a place in a package.
fn has_a_place(name: &str) -> bool {
    name == kit::KIT_FILE || ALSO.contains(&name) || name.strip_prefix(STARTER).is_some_and(|rest| !rest.is_empty())
}

/// Reads a package file. Everything is checked here: that it is a zip of the right ending,
/// what it holds and how much, and what its `kit.toml` says.
pub fn read(file: &Path) -> Result<Package, String> {
    let named = file.file_name().and_then(|name| name.to_str()).unwrap_or("");
    if !named.to_ascii_lowercase().ends_with(&format!(".{EXTENSION}")) {
        return Err(format!("A kit package is a file whose name ends in .{EXTENSION}."));
    }
    let size = std::fs::metadata(file).map_err(|e| format!("The package can't be read: {e}"))?.len();
    if size > MAX_PACKAGE {
        return Err(format!("The package is larger than {} MB.", MAX_PACKAGE / (1024 * 1024)));
    }
    let opened = std::fs::File::open(file).map_err(|e| format!("The package can't be read: {e}"))?;
    let mut archive = zip::ZipArchive::new(opened).map_err(|_| "That file is not a kit package: it can't be opened as one.".to_string())?;
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    let mut bytes = 0u64;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(|_| "The package is damaged: one of its files can't be read.".to_string())?;
        if entry.is_dir() {
            continue;
        }
        let name = entry.name().to_string();
        if !sound_name(&name) {
            return Err(format!("The package holds a file whose name a kit's file can't have: {:?}.", name.chars().take(80).collect::<String>()));
        }
        if !has_a_place(&name) {
            return Err(format!("The package holds {name}, which has no place in a kit: a kit is a {}, and a starter.", kit::KIT_FILE));
        }
        if files.iter().any(|(other, _)| other.eq_ignore_ascii_case(&name)) {
            return Err(format!("The package holds {name} twice."));
        }
        if files.len() >= MAX_FILES {
            return Err(format!("The package holds more than {MAX_FILES} files."));
        }
        // Counted as it is unpacked: what an entry says of its own size is not taken on trust.
        let mut held = Vec::new();
        (&mut entry).take(MAX_FILE + 1).read_to_end(&mut held).map_err(|_| format!("The package is damaged: {name} can't be unpacked."))?;
        if held.len() as u64 > MAX_FILE {
            return Err(format!("{name} is larger than {} MB.", MAX_FILE / (1024 * 1024)));
        }
        bytes = bytes.saturating_add(held.len() as u64);
        if bytes > MAX_UNPACKED {
            return Err(format!("The package unpacks to more than {} MB.", MAX_UNPACKED / (1024 * 1024)));
        }
        files.push((name, held));
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let Some((_, said)) = files.iter().find(|(name, _)| name == kit::KIT_FILE) else {
        return Err(format!("That file is not a kit package: it has no {} at its root.", kit::KIT_FILE));
    };
    let text = std::str::from_utf8(said).map_err(|_| format!("The package's {} is not text.", kit::KIT_FILE))?;
    let said = kit::describe(text).map_err(|why| format!("The package can't be loaded: {why}."))?;
    let mut hash = Sha256::new();
    for (name, held) in &files {
        hash.update(name.as_bytes());
        hash.update(&[0]);
        hash.update(&(held.len() as u64).to_le_bytes());
        hash.update(held);
    }
    Ok(Package { said, files, bytes, fingerprint: hash.hex() })
}

/// Makes a package of a kit's folder, for whoever writes a kit: its `kit.toml`, its starter,
/// and the few files a kit may hold beside them. Nothing else in the folder is taken. What was
/// written is read back as a package before it is called done, so a package this makes is one
/// [`read`] takes.
pub fn pack(kit_dir: &Path, to: &Path) -> Result<Package, String> {
    use std::io::Write;

    let named = to.file_name().and_then(|name| name.to_str()).unwrap_or("");
    if !named.to_ascii_lowercase().ends_with(&format!(".{EXTENSION}")) {
        return Err(format!("A kit package is a file whose name ends in .{EXTENSION}."));
    }
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    for name in std::iter::once(kit::KIT_FILE).chain(ALSO) {
        match std::fs::symlink_metadata(kit_dir.join(name)) {
            Ok(kind) if kind.is_file() && kind.len() <= MAX_FILE => {
                files.push((name.to_string(), std::fs::read(kit_dir.join(name)).map_err(|e| format!("{name} can't be read: {e}"))?));
            }
            Ok(_) => return Err(format!("{name} is not a file a kit can hold.")),
            Err(_) if name == kit::KIT_FILE => return Err(format!("That folder is not a kit: it has no {}.", kit::KIT_FILE)),
            Err(_) => {}
        }
    }
    let mut waiting = vec![(kit_dir.join("starter"), STARTER.to_string())];
    while let Some((next, prefix)) = waiting.pop() {
        let Ok(entries) = std::fs::read_dir(&next) else { continue };
        for entry in entries.flatten() {
            let Some(name) = entry.file_name().to_str().map(str::to_string) else { continue };
            let path = format!("{prefix}{name}");
            let kind = std::fs::symlink_metadata(entry.path()).map_err(|e| format!("{path} can't be read: {e}"))?;
            if !sound_name(&path) {
                return Err(format!("{path} has a name a kit's file can't have."));
            }
            if kind.is_dir() {
                waiting.push((entry.path(), format!("{path}/")));
            } else if kind.is_file() {
                if kind.len() > MAX_FILE || files.len() >= MAX_FILES {
                    return Err(format!("The kit holds more than a package can: {path}."));
                }
                files.push((path.clone(), std::fs::read(entry.path()).map_err(|e| format!("{path} can't be read: {e}"))?));
            }
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let failed = |e: std::io::Error| format!("The package could not be written: {e}");
    let mut zip = zip::ZipWriter::new(std::fs::File::create(to).map_err(failed)?);
    for (name, held) in &files {
        zip.start_file(name.as_str(), zip::write::SimpleFileOptions::default()).map_err(|e| format!("The package could not be written: {e}"))?;
        zip.write_all(held).map_err(failed)?;
    }
    zip.finish().map_err(|e| format!("The package could not be written: {e}"))?;
    read(to)
}

/// What the operating system asks before a package is loaded: what it is, in full.
pub fn question(package: &Package) -> String {
    let said = &package.said;
    let base = kit::named(&said.base).map_or_else(|| format!("the kit {:?}", said.base), Kit::called);
    let mut text = format!("Load the development kit \"{}\"", said.name);
    if !said.version.is_empty() {
        text.push_str(&format!(" {}", said.version));
    }
    if !said.author.is_empty() {
        text.push_str(&format!(", by {}", said.author));
    }
    text.push_str(&format!("?\n\nIt describes projects that run by {base}, with that kit's limits on what a project may ask.\n"));
    text.push_str(
        match package.starter() {
            0 => "It brings no starter of its own: a new project of it begins as one of that kit does.\n".to_string(),
            count => format!("It brings a starter of {count} file{}, which a new project of it begins with.\n", if count == 1 { "" } else { "s" }),
        }
        .as_str(),
    );
    text.push_str("\nNothing in it is run. A project made with it does nothing until you trust that project.");
    text
}

/// The folder a kit of this id is kept in, for projects kept in `projects_dir`.
fn folder(projects_dir: &Path, id: &str) -> Result<PathBuf, String> {
    let kits = kit::folder_beside(projects_dir).ok_or("Development kits can't be loaded in this run.")?;
    if !crate::manifest::valid_id(id) || kit::named(id).is_some() {
        return Err("That is not a kit that was loaded from a package.".into());
    }
    Ok(kits.join(id))
}

/// Unpacks a package into its own folder among the kits, and gives the kit as it is then
/// listed. A kit of the same id that is already there is not written over.
pub fn install(projects_dir: &Path, package: &Package) -> Result<&'static Kit, String> {
    let dir = folder(projects_dir, &package.said.id)?;
    if dir.exists() {
        return Err(format!("A development kit named {} is already loaded. Remove it before loading it again.", package.said.id));
    }
    if kit::all(projects_dir).len() >= kit::MAX_KITS {
        return Err(format!("There are {} development kits already: remove one first.", kit::MAX_KITS));
    }
    let written = (|| -> std::io::Result<()> {
        for (name, held) in &package.files {
            let file = dir.join(name);
            if let Some(parent) = file.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(file, held)?;
        }
        Ok(())
    })();
    let loaded = written.map_err(|e| format!("The kit could not be unpacked: {e}")).and_then(|()| {
        kit::forget();
        kit::named_in(projects_dir, &package.said.id).ok_or_else(|| "The kit was unpacked but can't be read back.".to_string())
    });
    if loaded.is_err() {
        // What was written here is only what this call wrote: it is taken away again.
        let _ = std::fs::remove_dir_all(&dir);
        kit::forget();
    }
    loaded
}

/// Removes a kit that was loaded from a package: its own folder among the kits, and nothing
/// else. Its projects stay where they are; they say their kit is not loaded. Returns its name.
pub fn remove(projects_dir: &Path, id: &str) -> Result<String, String> {
    let dir = folder(projects_dir, id)?;
    let name = kit::named_in(projects_dir, id).map_or_else(|| id.to_string(), |kit| kit.name.to_string());
    // A folder, and not a link to one: only what a package unpacked is taken away.
    let found = std::fs::symlink_metadata(&dir).map_err(|_| "That development kit is not loaded.".to_string())?;
    if !found.is_dir() {
        return Err("That development kit is not loaded.".into());
    }
    std::fs::remove_dir_all(&dir).map_err(|e| format!("The kit could not be removed: {e}"))?;
    kit::forget();
    Ok(name)
}

/// The files a new project of a loaded kit begins with: its starter, each by its path in the
/// project. Empty for a kit that brought none.
pub fn starter_files(projects_dir: &Path, kit: &Kit) -> Vec<(String, Vec<u8>)> {
    let Ok(dir) = folder(projects_dir, kit.id).map(|dir| dir.join("starter")) else { return vec![] };
    let mut found = Vec::new();
    let mut waiting = vec![(dir.clone(), String::new())];
    let mut bytes = 0u64;
    while let Some((next, prefix)) = waiting.pop() {
        let Ok(entries) = std::fs::read_dir(&next) else { continue };
        for entry in entries.flatten() {
            let Some(name) = entry.file_name().to_str().map(str::to_string) else { continue };
            let path = format!("{prefix}{name}");
            let Ok(kind) = std::fs::symlink_metadata(entry.path()) else { continue };
            if !sound_name(&path) || found.len() >= MAX_FILES {
                continue;
            }
            if kind.is_dir() {
                waiting.push((entry.path(), format!("{path}/")));
            } else if kind.is_file()
                && kind.len() <= MAX_FILE
                && bytes.saturating_add(kind.len()) <= MAX_UNPACKED
                && let Ok(held) = std::fs::read(entry.path())
            {
                bytes = bytes.saturating_add(held.len() as u64);
                found.push((path, held));
            }
        }
    }
    found.sort_by(|a, b| a.0.cmp(&b.0));
    found
}

#[cfg(test)]
pub(crate) mod tests {
    use std::io::Write;

    use super::*;
    use crate::project::tests::TempDir;

    /// A package file named `name` in `dir`, holding `files`.
    pub(crate) fn zipped(dir: &Path, name: &str, files: &[(&str, &[u8])]) -> PathBuf {
        let path = dir.join(name);
        let mut zip = zip::ZipWriter::new(std::fs::File::create(&path).unwrap());
        for (name, held) in files {
            zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
            zip.write_all(held).unwrap();
        }
        zip.finish().unwrap();
        path
    }

    pub(crate) const KIT: &str = "id = \"dashboards\"\nname = \"Dashboards Kit\"\nversion = \"0.2.0\"\nauthor = \"Ana\"\ndescription = \"Panels that chart what is selected.\"\nmade_of = [\"Svelte\", \"Python\"]\nbase = \"pythoncraft\"\n";
    pub(crate) const STARTER_MANIFEST: &str = "id = \"{{id}}\"\nname = \"{{name}}\"\nversion = \"0.1.0\"\nkit = \"dashboards\"\npermissions = [\"read\"]\n\n[panel]\nentry = \"panel/index.html\"\n";

    #[test]
    fn a_package_is_read_whole_and_says_what_it_is() {
        let t = TempDir::new("kitpack-read");
        let file = zipped(
            &t.0,
            "dashboards.gridkit",
            &[
                ("kit.toml", KIT.as_bytes()),
                ("README.md", b"# Dashboards"),
                ("starter/project.toml", STARTER_MANIFEST.as_bytes()),
                ("starter/panel/index.html", b"<p>{{name}}</p>"),
            ],
        );
        let package = read(&file).unwrap();
        assert_eq!(
            (package.said.id.as_str(), package.said.name.as_str(), package.said.base.as_str()),
            ("dashboards", "Dashboards Kit", "pythoncraft")
        );
        assert_eq!((package.files.len(), package.starter()), (4, 2));
        assert_eq!(package.fingerprint.len(), 64);
        assert_eq!(package.bytes, package.files.iter().map(|(_, held)| held.len() as u64).sum::<u64>());
        let asked = question(&package);
        assert!(asked.starts_with("Load the development kit \"Dashboards Kit\" 0.2.0, by Ana?"), "{asked}");
        assert!(asked.contains("run by PythonCraft, with") && asked.contains("a starter of 2 files") && asked.contains("Nothing in it is run."));
        // A package made before the first kit was named stands on it by the id it had.
        let earlier = zipped(&t.0, "earlier.gridkit", &[("kit.toml", KIT.replace("pythoncraft", "anie").as_bytes())]);
        let earlier = read(&earlier).unwrap();
        assert_eq!(earlier.said.base, "anie");
        assert!(question(&earlier).contains("run by PythonCraft, with"));
        // The same files give the same fingerprint; another file gives another.
        let again = zipped(&t.0, "again.gridkit", &[("kit.toml", KIT.as_bytes()), ("README.md", b"# Dashboards!")]);
        assert_ne!(read(&again).unwrap().fingerprint, package.fingerprint);
    }

    #[test]
    fn what_is_not_a_kit_package_is_refused_before_anything_is_written() {
        let t = TempDir::new("kitpack-refused");
        let refused = |name: &str, files: &[(&str, &[u8])]| read(&zipped(&t.0, name, files)).unwrap_err();
        assert!(refused("kit.zip", &[("kit.toml", KIT.as_bytes())]).contains("ends in .gridkit"));
        assert!(refused("a.gridkit", &[("README.md", b"x")]).contains("no kit.toml at its root"));
        // Names that would leave the folder, hide, or have no place in a kit.
        for bad in ["../kit.toml", "starter/../../x", "/etc/x", "starter\\x", "starter/.hidden", "C:x", "starter/a b"] {
            let why = refused("b.gridkit", &[("kit.toml", KIT.as_bytes()), (bad, b"x")]);
            assert!(why.contains("a kit's file can't have"), "{bad}: {why}");
        }
        for stray in ["run.exe", "bin/tool", "kits/kit.toml"] {
            let why = refused("c.gridkit", &[("kit.toml", KIT.as_bytes()), (stray, b"x")]);
            assert!(why.contains("has no place in a kit"), "{stray}: {why}");
        }
        // A file larger than a kit's file may be, whatever the entry says of its size.
        let big = vec![b'x'; (MAX_FILE + 1) as usize];
        assert!(refused("d.gridkit", &[("kit.toml", KIT.as_bytes()), ("starter/big.txt", &big)]).contains("is larger than 4 MB"));
        // What its kit.toml says has to be a kit: a name, an id of its own, a kit it stands on.
        for (said, why) in [
            ("id = \"anie\"\nname = \"X\"\nbase = \"anie\"\n", "is the name of a kit GridCraft is built with"),
            ("id = \"Bad Id\"\nname = \"X\"\nbase = \"anie\"\n", "id"),
            ("id = \"x\"\nname = \"\"\nbase = \"anie\"\n", "has no name"),
            ("id = \"x\"\nname = \"X\"\nbase = \"rust\"\n", "which GridCraft is not built with"),
            ("id = \"x\"\nname = \"X\"\nbase = \"anie\"\nrun = \"tool.exe\"\n", "kit.toml can't be read"),
            ("id = \"x\"\nname = \"X\"\n", "kit.toml can't be read"),
        ] {
            let got = refused("e.gridkit", &[("kit.toml", said.as_bytes())]);
            assert!(got.contains(why), "{said:?}: {got}");
        }
        // Something that is no zip at all, and a file that is not there.
        std::fs::write(t.0.join("f.gridkit"), b"not a zip").unwrap();
        assert!(read(&t.0.join("f.gridkit")).unwrap_err().contains("not a kit package"));
        assert!(read(&t.0.join("nowhere.gridkit")).is_err());
    }

    #[test]
    fn a_kit_is_loaded_into_a_folder_of_its_own_listed_and_removed() {
        let t = TempDir::new("kitpack-install");
        let projects = t.0.join("projects");
        std::fs::create_dir_all(&projects).unwrap();
        let file = zipped(&t.0, "dashboards.gridkit", &[("kit.toml", KIT.as_bytes()), ("starter/project.toml", STARTER_MANIFEST.as_bytes())]);
        let package = read(&file).unwrap();
        assert_eq!(kit::all(&projects).len(), 2, "only the kits GridCraft is built with");
        let loaded = install(&projects, &package).unwrap();
        assert_eq!((loaded.id, loaded.name, loaded.base, loaded.built_in), ("dashboards", "Dashboards Kit", "pythoncraft", false));
        assert_eq!((loaded.version, loaded.author, loaded.made_of), ("0.2.0", "Ana", ["Svelte", "Python"].as_slice()));
        // It stands on the first kit: what its projects may hold and ask is that kit's.
        assert_eq!((loaded.permissions, loaded.scripts, loaded.builds, loaded.panels), (kit::PYTHONCRAFT.permissions, true, false, true));
        assert!(t.0.join("kits/dashboards/kit.toml").is_file() && t.0.join("kits/dashboards/starter/project.toml").is_file());
        let listed: Vec<&str> = kit::all(&projects).iter().map(|kit| kit.id).collect();
        assert_eq!(listed, ["pythoncraft", "c", "dashboards"]);
        assert_eq!(starter_files(&projects, loaded).len(), 1);
        // Loaded twice, it is not written over.
        assert!(install(&projects, &package).unwrap_err().contains("already loaded"));
        // Removing takes its folder away and nothing else; the built-in kits can't be removed.
        std::fs::write(t.0.join("kits/keep.txt"), "x").unwrap();
        assert!(remove(&projects, "pythoncraft").is_err() && remove(&projects, "anie").is_err());
        assert!(remove(&projects, "../projects").is_err() && remove(&projects, "nowhere").is_err());
        assert_eq!(remove(&projects, "dashboards").unwrap(), "Dashboards Kit");
        assert!(!t.0.join("kits/dashboards").exists() && t.0.join("kits/keep.txt").is_file() && projects.is_dir());
        assert_eq!(kit::all(&projects).len(), 2);
        // In a folder that is not a projects folder of GridCraft's, no kit is loaded at all.
        assert!(install(&t.0, &package).unwrap_err().contains("can't be loaded in this run"));
    }

    #[test]
    fn a_kits_folder_is_packed_into_a_package_that_reads_back() {
        let t = TempDir::new("kitpack-pack");
        let making = t.0.join("making");
        std::fs::create_dir_all(making.join("starter/panel")).unwrap();
        std::fs::write(making.join("kit.toml"), KIT).unwrap();
        std::fs::write(making.join("README.md"), "# Dashboards").unwrap();
        std::fs::write(making.join("starter/project.toml"), STARTER_MANIFEST).unwrap();
        std::fs::write(making.join("starter/panel/index.html"), "<p>{{name}}</p>").unwrap();
        // What a kit does not hold is left where it is.
        std::fs::write(making.join("notes.txt"), "not part of the kit").unwrap();
        let package = pack(&making, &t.0.join("dashboards.gridkit")).unwrap();
        let names: Vec<&str> = package.files.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names, ["README.md", "kit.toml", "starter/panel/index.html", "starter/project.toml"]);
        assert_eq!(read(&t.0.join("dashboards.gridkit")).unwrap().fingerprint, package.fingerprint);
        // A folder that is no kit, a name that is no package's, a starter file with a name no kit's file has.
        assert!(pack(&t.0, &t.0.join("x.gridkit")).unwrap_err().contains("has no kit.toml"));
        assert!(pack(&making, &t.0.join("x.zip")).unwrap_err().contains("ends in .gridkit"));
        std::fs::write(making.join("starter/a b.txt"), "x").unwrap();
        assert!(pack(&making, &t.0.join("y.gridkit")).unwrap_err().contains("a name a kit's file can't have"));
    }

    #[test]
    fn a_project_of_a_loaded_kit_begins_from_its_starter_and_is_a_project_of_that_kit() {
        let t = TempDir::new("kitpack-starter");
        let projects = t.0.join("projects");
        std::fs::create_dir_all(&projects).unwrap();
        let file = zipped(
            &t.0,
            "dashboards.gridkit",
            &[
                ("kit.toml", KIT.as_bytes()),
                ("starter/project.toml", STARTER_MANIFEST.as_bytes()),
                ("starter/panel/index.html", b"<h1>{{name}}</h1><p>{{id}}</p>"),
                ("starter/panel/mark.bin", &[0xff, 0xfe, 0x00, 0x7b]),
            ],
        );
        let kit = install(&projects, &read(&file).unwrap()).unwrap();
        let made = crate::starter::create_in(&projects, kit, "Sales Board", None).unwrap();
        assert!(made.is_sound(), "{:?}", made.problem);
        let manifest = made.manifest.as_ref().unwrap();
        assert_eq!(
            (made.id.as_str(), manifest.name.as_str(), manifest.kit.as_str(), manifest.workbook.as_deref()),
            ("sales-board", "Sales Board", "dashboards", None)
        );
        assert_eq!(made.files, ["panel/index.html", "panel/mark.bin", "project.toml"]);
        assert_eq!(std::fs::read_to_string(projects.join("sales-board/panel/index.html")).unwrap(), "<h1>Sales Board</h1><p>sales-board</p>");
        assert_eq!(
            std::fs::read(projects.join("sales-board/panel/mark.bin")).unwrap(),
            [0xff, 0xfe, 0x00, 0x7b],
            "what is not text is copied as it is"
        );
        // It is listed under its kit, and is the code of a workbook when it is made as one.
        assert_eq!(kit::of_in(&projects, made.manifest.as_ref()).id, "dashboards");
        let code = crate::starter::create_in(&projects, kit, "Budget Board", Some("Budget.xlsm")).unwrap();
        assert_eq!(code.manifest.as_ref().unwrap().workbook.as_deref(), Some("Budget.xlsm"));
        assert_eq!(code.manifest.as_ref().unwrap().kit, "dashboards");
        // A kit that brought no starter begins a project as the kit it stands on does.
        let bare = zipped(
            &t.0,
            "plain.gridkit",
            &[(
                "kit.toml",
                b"id = \"plain\"
name = \"Plain Kit\"
base = \"c\"
",
            )],
        );
        let plain = install(&projects, &read(&bare).unwrap()).unwrap();
        assert_eq!((plain.base, plain.builds, plain.scripts), ("c", true, false));
        let blink = crate::starter::create_in(&projects, plain, "Blink", None).unwrap();
        assert!(blink.is_sound(), "{:?}", blink.problem);
        assert_eq!(
            (blink.manifest.as_ref().unwrap().kit.as_str(), blink.files.as_slice()),
            ("plain", ["panel/index.html".to_string(), "project.toml".to_string(), "src/main.c".to_string()].as_slice())
        );
        // Once its kit is removed, a project says so and can't be used; loaded again, it can.
        remove(&projects, "dashboards").unwrap();
        let orphan = crate::project::load(&projects.join("sales-board"));
        assert!(!orphan.is_sound());
        assert!(
            orphan.problem.as_deref().unwrap().contains("\"dashboards\", which this GridCraft doesn't have: it is not loaded"),
            "{:?}",
            orphan.problem
        );
        install(&projects, &read(&file).unwrap()).unwrap();
        assert!(crate::project::load(&projects.join("sales-board")).is_sound());
        // A starter that does not make a project leaves nothing behind.
        let broken = zipped(
            &t.0,
            "broken.gridkit",
            &[
                (
                    "kit.toml",
                    b"id = \"broken\"
name = \"Broken\"
base = \"anie\"
",
                ),
                ("starter/project.toml", b"this is not a manifest"),
            ],
        );
        let broken = install(&projects, &read(&broken).unwrap()).unwrap();
        assert!(crate::starter::create_in(&projects, broken, "Nothing", None).unwrap_err().contains("can't be read back"));
        assert!(!projects.join("nothing").exists());
    }
}
