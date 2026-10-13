//! The C Development Kit at work: a project's recipe, the person's compiler, and building a
//! project's C into one WebAssembly program.
//!
//! ```toml
//! kit = "c"
//!
//! [c]
//! sources = ["src"]            # C files, or folders: every .c file directly in a folder
//! include = ["src"]            # optional: folders searched for #include "..."
//! define = ["NAME", "N=2"]     # optional: what -D takes
//! optimize = "size"            # optional: size | speed | none
//! entry = "main"               # optional: main (it has a main that runs), or none (its page
//!                              # calls the functions it exports)
//! ```
//!
//! **The recipe is a list, never a command.** A project says which of its files are built and
//! with which few named options. GridCraft writes the compiler's command line itself: nothing a
//! project says becomes an option of its own choosing.
//!
//! **The compiler is the person's**, found or named like the Python: `zig`, used as `zig cc`,
//! which carries the C library a WebAssembly program needs. GridCraft ships none.
//!
//! **A build runs the person's compiler over a project's files, so it is for a project the
//! person trusts as it is**, like running a script. [`build`] decides that for itself, reading
//! the project and the person's decisions fresh from disk.
//!
//! **A project is built from its own files and the compiler's, and from nothing else.** Each
//! source is compiled by itself and the compiler says every file it read for it (`#include`,
//! `#embed`): one outside the project and outside the compiler's own files fails the build, and
//! nothing of it is kept. Inline assembly is turned off, since the assembler can take a file in
//! without the compiler saying so. So what is built can hold nothing of the computer but what
//! the person reviewed when they trusted the project. (It rests on the compiler telling the
//! truth about what it read: a guard, not a proof.)
//!
//! **What is built is kept outside the project**, beside the settings, under the fingerprint of
//! the files it was built from: `built/<id>/<fingerprint>/program.wasm`. A build never changes
//! what the person trusted, and what was built from other files than the project holds now is
//! not what is found for it.
//!
//! Nothing here runs what was built. The project's panel does: the bridge serves, beside the
//! page, the program kept for the project as it is trusted then ([`program_bytes`]) and the
//! loader that starts it ([`crate::c_program`]).

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::manifest;

/// The most folders a recipe may name for `#include`.
pub const MAX_INCLUDES: usize = 32;
/// The most definitions a recipe may make.
pub const MAX_DEFINES: usize = 64;
/// The name of what is built.
pub const OUTPUT: &str = "program.wasm";
/// The largest program kept (bytes).
pub const MAX_OUTPUT_BYTES: u64 = 64 * 1024 * 1024;

/// What the compiler is asked to make the program good at.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Optimize {
    #[default]
    Size,
    Speed,
    None,
}

impl Optimize {
    pub const ALL: [Optimize; 3] = [Optimize::Size, Optimize::Speed, Optimize::None];

    /// The word a recipe uses.
    pub fn code(self) -> &'static str {
        match self {
            Optimize::Size => "size",
            Optimize::Speed => "speed",
            Optimize::None => "none",
        }
    }

    pub fn parse(code: &str) -> Option<Optimize> {
        Optimize::ALL.into_iter().find(|o| o.code() == code)
    }

    /// What the compiler is told. (The web build runs no compiler.)
    #[cfg(not(target_arch = "wasm32"))]
    fn flag(self) -> &'static str {
        match self {
            Optimize::Size => "-Os",
            Optimize::Speed => "-O2",
            Optimize::None => "-O0",
        }
    }
}

/// How the program is started.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Entry {
    /// It has a `main`, which runs to its end.
    #[default]
    Main,
    /// It has none: whoever runs it calls the functions it exports.
    None,
}

impl Entry {
    pub const ALL: [Entry; 2] = [Entry::Main, Entry::None];

    pub fn code(self) -> &'static str {
        match self {
            Entry::Main => "main",
            Entry::None => "none",
        }
    }

    pub fn parse(code: &str) -> Option<Entry> {
        Entry::ALL.into_iter().find(|e| e.code() == code)
    }
}

/// A checked recipe: `[c]` in a project's manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recipe {
    /// C files, or folders of them, as paths inside the project.
    pub sources: Vec<String>,
    /// Folders searched for `#include "..."`, as paths inside the project.
    pub include: Vec<String>,
    /// `NAME` or `NAME=value`, as `-D` takes them.
    pub define: Vec<String>,
    pub optimize: Optimize,
    pub entry: Entry,
}

/// `[c]` as it is written.
#[derive(Deserialize)]
pub(crate) struct Raw {
    sources: Option<Vec<String>>,
    include: Option<Vec<String>>,
    define: Option<Vec<String>>,
    optimize: Option<String>,
    entry: Option<String>,
}

/// A definition: a C name, or a name and a plain value. Nothing that could be read as another
/// option, a path or a quoted string.
fn definition(text: &str) -> Result<String, String> {
    let (name, value) = text.split_once('=').map_or((text, None), |(name, value)| (name, Some(value)));
    let mut letters = name.chars();
    let starts = letters.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_');
    if !starts || name.len() > 64 || !letters.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(format!("its definition {text:?} does not begin with a C name"));
    }
    if let Some(value) = value
        && (value.len() > 64 || !value.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '+' | '-')))
    {
        return Err(format!("its definition {text:?} has a value that is not letters, digits and _ . + -"));
    }
    Ok(text.to_string())
}

impl Recipe {
    /// Checks a recipe as it was written. Whether the files it names are in the project is for
    /// [`Recipe::sources_among`] to say, when the project's files are known.
    pub(crate) fn read(raw: Raw) -> Result<Recipe, String> {
        let sources = raw.sources.unwrap_or_default();
        if sources.is_empty() {
            return Err("its C recipe names no sources".into());
        }
        if sources.len() > crate::project::MAX_FILES {
            return Err(format!("its C recipe names more than {} sources", crate::project::MAX_FILES));
        }
        let include = raw.include.unwrap_or_default();
        if include.len() > MAX_INCLUDES {
            return Err(format!("its C recipe names more than {MAX_INCLUDES} include folders"));
        }
        let define = raw.define.unwrap_or_default();
        if define.len() > MAX_DEFINES {
            return Err(format!("its C recipe makes more than {MAX_DEFINES} definitions"));
        }
        let optimize = match raw.optimize.as_deref() {
            None => Optimize::default(),
            Some(word) => Optimize::parse(word).ok_or_else(|| format!("its C recipe optimizes for {word:?}, which is not size, speed or none"))?,
        };
        let entry = match raw.entry.as_deref() {
            None => Entry::default(),
            Some(word) => Entry::parse(word).ok_or_else(|| format!("its C recipe's entry is {word:?}, which is not main or none"))?,
        };
        Ok(Recipe {
            sources: sources.iter().map(|path| manifest::inner_path(path)).collect::<Result<_, _>>()?,
            include: include.iter().map(|path| manifest::inner_path(path)).collect::<Result<_, _>>()?,
            define: define.iter().map(|text| definition(text)).collect::<Result<_, _>>()?,
            optimize,
            entry,
        })
    }

    /// The C files that are built, as paths inside the project, given the project's files: each
    /// source that is a file, and for each that is a folder every `.c` file directly in it. In
    /// the recipe's order, a folder's files in the order of their names, each file once.
    pub fn sources_among(&self, files: &[String]) -> Result<Vec<String>, String> {
        let mut found: Vec<String> = Vec::new();
        for source in &self.sources {
            if files.contains(source) {
                if !source.ends_with(".c") {
                    return Err(format!("its source, {source}, is not a .c file"));
                }
                if !found.contains(source) {
                    found.push(source.clone());
                }
                continue;
            }
            let folder = format!("{source}/");
            if !files.iter().any(|file| file.starts_with(&folder)) {
                return Err(format!("its source, {source}, is not in it"));
            }
            // `files` is sorted, so a folder's files come in the order of their names.
            let inside: Vec<&String> =
                files.iter().filter(|file| file.strip_prefix(&folder).is_some_and(|rest| !rest.contains('/') && rest.ends_with(".c"))).collect();
            if inside.is_empty() {
                return Err(format!("its sources folder, {source}, holds no .c file"));
            }
            for file in inside {
                if !found.contains(file) {
                    found.push(file.clone());
                }
            }
        }
        Ok(found)
    }

    /// Whether each folder named for `#include` is a folder of the project.
    pub fn includes_among(&self, files: &[String]) -> Result<(), String> {
        match self.include.iter().find(|folder| !files.iter().any(|file| file.starts_with(&format!("{folder}/")))) {
            Some(folder) => Err(format!("its include folder, {folder}, is not in it")),
            None => Ok(()),
        }
    }
}

/// Where what is built is kept: beside the settings, never in a project.
pub fn built_dir(settings: &Path) -> PathBuf {
    settings.with_file_name("built")
}

/// The program built from a project as it is now: the one kept under its fingerprint.
pub fn program_for(settings: &Path, id: &str, fingerprint: &str) -> PathBuf {
    built_dir(settings).join(id).join(fingerprint).join(OUTPUT)
}

/// A program that was built, as it is kept.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Built {
    /// Where it is.
    pub program: PathBuf,
    pub bytes: u64,
    /// How many C files it was built from.
    pub sources: usize,
    /// The compiler that built it, as it names itself.
    pub compiler: String,
    /// How long the build took, in milliseconds.
    pub millis: u64,
}

/// How a project stands as to being built.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Standing {
    /// Nothing was built from it.
    NotBuilt,
    /// A program is kept that was built from the files it holds now.
    Built { bytes: u64 },
    /// A program is kept, built from other files than it holds now. It is not the project's.
    Stale,
}

/// How the project `id`, whose files have this fingerprint, stands. Only what the file system
/// says: nothing is read or run.
pub fn standing(settings: &Path, id: &str, fingerprint: Option<&str>) -> Standing {
    if !manifest::valid_id(id) {
        return Standing::NotBuilt;
    }
    let kept = |path: &Path| std::fs::metadata(path).ok().filter(|found| found.is_file()).map(|found| found.len());
    if let Some(bytes) = fingerprint.filter(|print| is_fingerprint(print)).and_then(|print| kept(&program_for(settings, id, print))) {
        return Standing::Built { bytes };
    }
    let others = std::fs::read_dir(built_dir(settings).join(id)).map(|entries| entries.flatten().any(|entry| entry.path().join(OUTPUT).is_file()));
    if others.unwrap_or(false) { Standing::Stale } else { Standing::NotBuilt }
}

/// The program built from the project `id` as its files are now (they have this fingerprint),
/// for whoever serves it to the project's panel. `None` when nothing was built from these
/// files: what was built from others is not this project's, and is never what is handed out.
/// Also for what is no project's id or no fingerprint, and for what is kept there but is not
/// a plain file of a program's size (a link is not followed).
pub fn program_bytes(settings: &Path, id: &str, fingerprint: &str) -> Option<Vec<u8>> {
    if !manifest::valid_id(id) || !is_fingerprint(fingerprint) {
        return None;
    }
    let kept = program_for(settings, id, fingerprint);
    let found = std::fs::symlink_metadata(&kept).ok()?;
    if !found.is_file() || found.len() > MAX_OUTPUT_BYTES {
        return None;
    }
    std::fs::read(&kept).ok()
}

/// 64 hexadecimal digits: what a fingerprint is, and so what a folder kept under one is named.
fn is_fingerprint(text: &str) -> bool {
    text.len() == 64 && text.chars().all(|c| c.is_ascii_hexdigit())
}

/// The paths a compiler's dependency file names, after the object it is for.
///
/// The file is written as a rule for `make`: `object: file file \` over several lines, a space
/// inside a name written `\ `, `#` as `\#` and `$` as `$$`. A backslash before anything else is
/// the backslash of a Windows path.
pub fn dependencies(text: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut word = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.peek() {
                Some('\n') => {
                    chars.next();
                    words.push(std::mem::take(&mut word));
                }
                Some('\r') => {
                    chars.next();
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    words.push(std::mem::take(&mut word));
                }
                Some(' ') | Some('#') => word.extend(chars.next()),
                _ => word.push('\\'),
            },
            '$' if chars.peek() == Some(&'$') => {
                chars.next();
                word.push('$');
            }
            c if c.is_whitespace() => words.push(std::mem::take(&mut word)),
            c => word.push(c),
        }
    }
    words.push(word);
    // Everything up to the word that ends the rule's target (`object:`) is the target.
    let after = words.iter().position(|word| word.ends_with(':')).map_or(0, |at| at + 1);
    words.into_iter().skip(after).filter(|word| !word.is_empty()).collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub use host::{Builder, Compiler, Progress, build, find};

#[cfg(target_arch = "wasm32")]
pub use web::{Builder, Compiler, Progress, build, find};

/// The web build has no processes: there is no compiler to run.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::*;
    use crate::bridge::Places;

    const NONE: &str = "A C project can't be built from this copy of GridCraft.";

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Compiler {
        pub program: String,
        pub version: String,
    }

    impl Compiler {
        /// It in a few words, for people.
        pub fn describe(&self) -> String {
            format!("zig {}", self.version)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Progress {
        Building { millis: u64 },
        Done(Result<Built, String>),
    }

    pub fn find(_configured: &str) -> Result<Compiler, String> {
        Err(NONE.into())
    }

    pub fn build(_places: &Places, _id: &str, _configured: &str) -> Result<Built, String> {
        Err(NONE.into())
    }

    #[derive(Default)]
    pub struct Builder;

    impl Builder {
        pub fn global() -> &'static Builder {
            static BUILDER: Builder = Builder;
            &BUILDER
        }

        pub fn start(&self, _places: &Places, _id: &str, _configured: &str, _done: impl Fn() + Send + 'static) -> Result<(), String> {
            Err(NONE.into())
        }

        pub fn progress(&self, _projects: &Path, _id: &str) -> Option<Progress> {
            None
        }

        pub fn building(&self) -> usize {
            0
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod host {
    use std::collections::HashMap;
    use std::fs;
    use std::io::Read;
    use std::process::{Command, Stdio};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};
    use std::time::{Duration, Instant};

    use super::*;
    use crate::bridge::Places;
    use crate::trust::TrustStore;
    use crate::{kit, project};

    /// How long a compiler has to say what it is.
    const PROBE_TIMEOUT: Duration = Duration::from_secs(20);
    /// How long one file may take to compile, and the whole to link.
    const STEP_TIMEOUT: Duration = Duration::from_secs(600);
    /// How much of what the compiler says is kept (bytes): the beginning, where the first error is.
    const SAID_KEPT: usize = 16 * 1024;
    /// The most compilers run at once.
    const MAX_AT_ONCE: usize = 8;

    /// A C compiler that answered when asked what it is.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Compiler {
        /// The path or command it was started by.
        pub program: String,
        pub version: String,
        /// Where its own files are (its headers and its C library), as the file system names it.
        lib: PathBuf,
    }

    impl Compiler {
        /// It in a few words, for people.
        pub fn describe(&self) -> String {
            format!("zig {}", self.version)
        }
    }

    fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
        m.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The clock. This is the build that has processes, and so a clock to read.
    #[allow(clippy::disallowed_methods)]
    fn now() -> Instant {
        Instant::now()
    }

    /// A command that never opens a console window of its own (Windows), with nothing inherited
    /// on its standard streams.
    fn command(program: &str) -> Command {
        let mut cmd = Command::new(program);
        cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        cmd
    }

    /// What a process did: whether it ended well, and what it printed (both streams, the
    /// beginning of each).
    struct Ran {
        ok: bool,
        printed: String,
        complained: String,
    }

    /// Reads a stream to its end on a thread of its own, keeping its beginning. A process that
    /// prints more than a pipe holds would otherwise wait for a reader for ever.
    fn drain(stream: Option<impl Read + Send + 'static>) -> std::thread::JoinHandle<Vec<u8>> {
        std::thread::spawn(move || {
            let mut kept = Vec::new();
            let Some(mut stream) = stream else { return kept };
            let mut buffer = [0u8; 8192];
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        let room = SAID_KEPT.saturating_sub(kept.len());
                        kept.extend_from_slice(buffer.get(..n.min(room)).unwrap_or(&[]));
                    }
                }
            }
            kept
        })
    }

    /// Runs a command to its end, or stops it when it takes too long.
    fn run(mut cmd: Command, timeout: Duration) -> Result<Ran, String> {
        let mut child = cmd.spawn().map_err(|e| format!("it could not be started ({e})"))?;
        let (printed, complained) = (drain(child.stdout.take()), drain(child.stderr.take()));
        let deadline = now() + timeout;
        let status = loop {
            match child.try_wait() {
                Ok(Some(status)) => break Some(status),
                Ok(None) if now() < deadline => std::thread::sleep(Duration::from_millis(15)),
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break None;
                }
            }
        };
        let text = |read: std::thread::JoinHandle<Vec<u8>>| String::from_utf8_lossy(&read.join().unwrap_or_default()).into_owned();
        let (printed, complained) = (text(printed), text(complained));
        match status {
            Some(status) => Ok(Ran { ok: status.success(), printed, complained }),
            None => Err(format!("it took longer than {} minutes and was stopped", timeout.as_secs() / 60)),
        }
    }

    /// The first quoted text after `key` in what `zig env` prints: `.lib_dir = "..."` in newer
    /// versions, `"lib_dir": "..."` in older ones. A doubled backslash is one.
    fn quoted_after(text: &str, key: &str) -> Option<String> {
        let rest = text.split_once(key)?.1;
        let rest = rest.split_once(['=', ':'])?.1;
        let rest = rest.trim_start().strip_prefix('"')?;
        let mut value = String::new();
        let mut chars = rest.chars();
        while let Some(c) = chars.next() {
            match c {
                '"' => return Some(value),
                '\\' => value.push(chars.next()?),
                c => value.push(c),
            }
        }
        None
    }

    /// Asks one candidate what it is. `None` unless it answers as zig does, in time: a version,
    /// and where its own files are.
    fn probe(program: &str) -> Option<Compiler> {
        let mut asked = command(program);
        asked.arg("version");
        let answered = run(asked, PROBE_TIMEOUT).ok().filter(|ran| ran.ok)?;
        let version = answered.printed.lines().next()?.trim().to_string();
        let numbered = version.split('.').take(2).all(|part| !part.is_empty() && part.chars().all(|c| c.is_ascii_digit()));
        if !numbered || version.split('.').count() < 3 || version.len() > 64 || version.chars().any(|c| c.is_control() || c.is_whitespace()) {
            return None;
        }
        let mut asked = command(program);
        asked.arg("env");
        let told = run(asked, PROBE_TIMEOUT).ok().filter(|ran| ran.ok)?;
        let lib = fs::canonicalize(quoted_after(&told.printed, "lib_dir")?).ok().filter(|lib| lib.is_dir())?;
        Some(Compiler { program: program.to_string(), version, lib })
    }

    /// Finds the compiler to use: the person's choice when there is one, or `zig` where
    /// programs are looked for.
    pub fn find(configured: &str) -> Result<Compiler, String> {
        let configured = configured.trim();
        if !configured.is_empty() {
            return probe(configured).ok_or_else(|| format!("{configured} did not answer as zig does. (The C kit builds with zig, used as zig cc.)"));
        }
        probe("zig").ok_or_else(|| {
            "No C compiler was found. Install zig (ziglang.org), or say where it is in the C Development Kit's part of Developer ▸ Development."
                .to_string()
        })
    }

    /// Whether `path` is `root` or inside it, as the file system names both. A path that can't
    /// be named (it is not there) is inside nothing.
    fn within(path: &Path, root: &Path) -> bool {
        fs::canonicalize(path).is_ok_and(|real| real.starts_with(root))
    }

    /// What the compiler said, with the project's own folder taken out of the paths: its files
    /// are then named as the project names them.
    fn said(ran: &Ran, dir: &Path) -> String {
        let mut text = format!("{}{}", ran.complained, ran.printed);
        for spelling in [dir.display().to_string(), dir.display().to_string().replace('\\', "/")] {
            for separator in ['\\', '/'] {
                text = text.replace(&format!("{spelling}{separator}"), "");
            }
        }
        text.trim().to_string()
    }

    /// The options every compile and the link share. `cwd` is where it runs, and where the
    /// compiler keeps its own cache: never in the project.
    fn zig_cc(compiler: &Compiler, work: &Path, cache: &Path) -> Command {
        let mut cmd = command(&compiler.program);
        cmd.current_dir(work);
        cmd.env("ZIG_GLOBAL_CACHE_DIR", cache.join("global")).env("ZIG_LOCAL_CACHE_DIR", cache.join("local"));
        cmd.args(["cc", "-target", "wasm32-wasi"]);
        cmd
    }

    /// Builds the project `id` and keeps the program under the fingerprint of the files it was
    /// built from. It decides for itself whether the project may be built: it is read fresh from
    /// disk, and so are the person's decisions. `configured` is the compiler the person named
    /// (empty: the one that is found).
    ///
    /// The project's files are held as they are for as long as the build runs (GridCraft changes
    /// none of them meanwhile), and the project is read again at the end: files changed by hand
    /// while it was being built give no program.
    pub fn build(places: &Places, id: &str, configured: &str) -> Result<Built, String> {
        let began = now();
        if !manifest::valid_id(id) {
            return Err(format!("{id:?} is not a project's id."));
        }
        let dir = places.projects.join(id);
        let in_use = project::in_use(&dir);
        let _held = in_use.read().unwrap_or_else(PoisonError::into_inner);
        let found = project::load(&dir);
        let name = found.name().to_string();
        if !TrustStore::load(&places.trust).status(&found).is_trusted() {
            return Err(format!("{name} is not trusted as it is now. Building runs your compiler over its files: review and trust it first."));
        }
        let (Some(manifest), Some(fingerprint)) = (found.manifest.as_ref(), found.fingerprint.as_deref()) else {
            return Err(format!("{name} can't be read as a project."));
        };
        // A project of the C kit, or of a kit that stands on it.
        if kit::of_in(found.dir.parent().unwrap_or(&found.dir), Some(manifest)).base != kit::C.id {
            return Err(format!("{name} is not a project of {}.", kit::C.called()));
        }
        let Some(recipe) = manifest.c.as_ref() else {
            return Err(format!("{name} has no C recipe: its project.toml names no sources to build."));
        };
        let sources = recipe.sources_among(&found.files).map_err(|why| format!("{name} can't be built: {why}."))?;
        recipe.includes_among(&found.files).map_err(|why| format!("{name} can't be built: {why}."))?;
        let compiler = find(configured)?;
        let real_dir = fs::canonicalize(&dir).map_err(|e| format!("{name}'s folder can't be read ({e})."))?;

        // Where the compiler works: beside what is built, in a folder of this build's own.
        let built = built_dir(&places.settings);
        let cache = built.join(".cache");
        let work = built.join(".work").join(format!("{id}-{}", crate::http::token().map(|t| t.chars().take(16).collect::<String>())?));
        let failed = |e: std::io::Error| format!("The build's folder could not be made ({e}).");
        fs::create_dir_all(&work).map_err(failed)?;
        fs::create_dir_all(cache.join("global")).and_then(|()| fs::create_dir_all(cache.join("local"))).map_err(failed)?;
        let outcome = compile_and_link(&compiler, recipe, &sources, &dir, &real_dir, &work, &cache, &name);
        let outcome = outcome.and_then(|()| {
            let made = work.join(OUTPUT);
            let bytes = fs::metadata(&made).map(|found| found.len()).map_err(|_| format!("{name} was built, and the compiler left no program."))?;
            let mut magic = [0u8; 4];
            let is_wasm = fs::File::open(&made).and_then(|mut file| file.read_exact(&mut magic)).is_ok() && magic == *b"\0asm";
            if !is_wasm || bytes > MAX_OUTPUT_BYTES {
                return Err(format!("{name} was built, and what the compiler left is not a WebAssembly program GridCraft keeps."));
            }
            // The files it was built from have to be the ones the project holds still.
            if project::load(&dir).fingerprint.as_deref() != Some(fingerprint) {
                return Err(format!("{name}'s files changed while it was being built: nothing was kept. Build it again."));
            }
            let kept = program_for(&places.settings, id, fingerprint);
            let folder = kept.parent().ok_or("The program has no place to be kept.")?;
            let failed = |e: std::io::Error| format!("The program could not be kept ({e}).");
            fs::create_dir_all(folder).map_err(failed)?;
            let _ = fs::remove_file(&kept);
            fs::rename(&made, &kept).or_else(|_| fs::copy(&made, &kept).map(|_| ())).map_err(failed)?;
            // What was built from other files than these is no longer the project's.
            if let Ok(others) = fs::read_dir(built.join(id)) {
                for other in others.flatten().filter(|other| other.path() != folder) {
                    let _ = fs::remove_dir_all(other.path());
                }
            }
            Ok(Built {
                program: kept,
                bytes,
                sources: sources.len(),
                compiler: compiler.describe(),
                millis: began.elapsed().as_millis().min(u128::from(u64::MAX)) as u64,
            })
        });
        let _ = fs::remove_dir_all(&work);
        outcome
    }

    /// Compiles each source by itself, checks what each one read, and links them into the
    /// program, in `work`.
    #[allow(clippy::too_many_arguments)]
    fn compile_and_link(
        compiler: &Compiler,
        recipe: &Recipe,
        sources: &[String],
        dir: &Path,
        real_dir: &Path,
        work: &Path,
        cache: &Path,
        name: &str,
    ) -> Result<(), String> {
        let real_cache = fs::canonicalize(cache).map_err(|e| format!("The build's folder can't be read ({e})."))?;
        let one = |index: usize, source: &String| -> Result<(), String> {
            let mut cmd = zig_cc(compiler, work, cache);
            cmd.arg(recipe.optimize.flag());
            // Doom's arithmetic overflows on purpose, and so does much old C: the compiler would
            // otherwise stop a program that does. And no assembly: the assembler can take a file
            // in without the compiler saying so.
            cmd.args(["-fno-sanitize=undefined", "-fno-gnu-inline-asm"]);
            for folder in &recipe.include {
                cmd.arg("-I").arg(dir.join(folder));
            }
            for definition in &recipe.define {
                cmd.arg(format!("-D{definition}"));
            }
            cmd.arg("-c").arg(dir.join(source)).arg("-o").arg(format!("{index}.o")).args(["-MD", "-MF"]).arg(format!("{index}.d"));
            let ran = run(cmd, STEP_TIMEOUT).map_err(|why| format!("{name} was not built. Compiling {source}: {why}."))?;
            if !ran.ok {
                return Err(format!("{name} was not built: {source} does not compile.\n{}", said(&ran, dir)));
            }
            // Every file the compiler read for it is the project's, or the compiler's own.
            let read = fs::read_to_string(work.join(format!("{index}.d")))
                .map_err(|_| format!("{name} was not built: the compiler did not say what {source} reads."))?;
            for file in dependencies(&read) {
                let path = Path::new(&file);
                let inside = fs::canonicalize(path).ok().and_then(|real| real.strip_prefix(real_dir).ok().map(Path::to_path_buf));
                let in_project =
                    inside.is_some_and(|inner| !inner.components().any(|part| part.as_os_str().to_str().is_none_or(manifest::kept_by_git)));
                if !in_project && !within(path, &compiler.lib) && !within(path, &real_cache) {
                    return Err(format!(
                        "{name} was not built: {source} reads a file that is not the project's, {file}. A project is built from its own files and the compiler's."
                    ));
                }
            }
            Ok(())
        };
        // Several at once: each is a process of its own.
        let (next, first_failure) = (AtomicUsize::new(0), Mutex::new(None::<String>));
        let at_once = std::thread::available_parallelism().map_or(2, usize::from).clamp(1, MAX_AT_ONCE).min(sources.len().max(1));
        std::thread::scope(|scope| {
            for _ in 0..at_once {
                scope.spawn(|| {
                    loop {
                        let index = next.fetch_add(1, Ordering::SeqCst);
                        let Some(source) = sources.get(index) else { break };
                        if lock(&first_failure).is_some() {
                            break;
                        }
                        if let Err(why) = one(index, source) {
                            lock(&first_failure).get_or_insert(why);
                            break;
                        }
                    }
                });
            }
        });
        if let Some(why) = lock(&first_failure).take() {
            return Err(why);
        }
        let mut cmd = zig_cc(compiler, work, cache);
        cmd.arg(recipe.optimize.flag());
        if recipe.entry == Entry::None {
            cmd.arg("-mexec-model=reactor");
        }
        cmd.args((0..sources.len()).map(|index| format!("{index}.o"))).arg("-o").arg(OUTPUT);
        let ran = run(cmd, STEP_TIMEOUT).map_err(|why| format!("{name} was not built. Putting it together: {why}."))?;
        if !ran.ok {
            return Err(format!("{name} was not built: its parts do not go together.\n{}", said(&ran, dir)));
        }
        Ok(())
    }

    /// How a build that was started stands.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Progress {
        /// It is running, and has been for this long.
        Building { millis: u64 },
        /// It ended: the program, or why there is none.
        Done(Result<Built, String>),
    }

    enum State {
        Building(Instant),
        Done(Result<Built, String>),
    }

    /// Builds that were asked for, each on a thread of its own: a build takes seconds or
    /// minutes, and the window that asked goes on being drawn.
    #[derive(Default)]
    pub struct Builder {
        builds: Mutex<HashMap<(PathBuf, String), Arc<Mutex<State>>>>,
    }

    impl Builder {
        pub fn new() -> Self {
            Self::default()
        }

        /// The one the app uses.
        pub fn global() -> &'static Builder {
            static BUILDER: OnceLock<Builder> = OnceLock::new();
            BUILDER.get_or_init(Builder::new)
        }

        /// Starts building the project `id`. `done` is called when it has ended. Refused while
        /// that project is being built already.
        pub fn start(&self, places: &Places, id: &str, configured: &str, done: impl Fn() + Send + 'static) -> Result<(), String> {
            if !manifest::valid_id(id) {
                return Err(format!("{id:?} is not a project's id."));
            }
            let key = (places.projects.clone(), id.to_string());
            let state = Arc::new(Mutex::new(State::Building(now())));
            {
                let mut builds = lock(&self.builds);
                if builds.get(&key).is_some_and(|state| matches!(*lock(state), State::Building(_))) {
                    return Err(format!("{id} is being built already."));
                }
                builds.insert(key, state.clone());
            }
            let (places, id, configured) = (places.clone(), id.to_string(), configured.to_string());
            let spawned = std::thread::Builder::new().name("gridcraft-c-build".into()).spawn(move || {
                let outcome = build(&places, &id, &configured);
                *lock(&state) = State::Done(outcome);
                done();
            });
            spawned.map(|_| ()).map_err(|e| format!("The build could not be started ({e})."))
        }

        /// How the last build asked for of the project `id` stands. `None`: none was asked for
        /// since GridCraft started.
        pub fn progress(&self, projects: &Path, id: &str) -> Option<Progress> {
            let state = lock(&self.builds).get(&(projects.to_path_buf(), id.to_string())).cloned()?;
            let state = lock(&state);
            Some(match &*state {
                State::Building(since) => Progress::Building { millis: since.elapsed().as_millis().min(u128::from(u64::MAX)) as u64 },
                State::Done(outcome) => Progress::Done(outcome.clone()),
            })
        }

        /// How many builds are running.
        pub fn building(&self) -> usize {
            lock(&self.builds).values().filter(|state| matches!(*lock(state), State::Building(_))).count()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn where_zig_keeps_its_own_files_is_read_from_what_it_says_in_either_form() {
            let newer =
                ".{\n    .zig_exe = \"C:\\\\tools\\\\zig\\\\zig.exe\",\n    .lib_dir = \"C:\\\\tools\\\\zig\\\\lib\",\n    .std_dir = \"x\",\n}";
            assert_eq!(quoted_after(newer, "lib_dir").as_deref(), Some("C:\\tools\\zig\\lib"));
            let older = "{\n \"zig_exe\": \"/usr/bin/zig\",\n \"lib_dir\": \"/usr/lib/zig\",\n \"std_dir\": \"/usr/lib/zig/std\"\n}";
            assert_eq!(quoted_after(older, "lib_dir").as_deref(), Some("/usr/lib/zig"));
            for broken in ["", "lib_dir", ".lib_dir = 7", ".lib_dir = \"never closed", "\"lib_dir\" \"no sign\""] {
                assert_eq!(quoted_after(broken, "lib_dir"), None, "{broken:?}");
            }
        }

        #[test]
        fn a_program_that_is_no_compiler_is_not_taken_for_one() {
            // Nothing by that name; and a program that answers, but not as zig does.
            assert!(probe("gridcraft-no-such-compiler").is_none());
            assert!(find("gridcraft-no-such-compiler").unwrap_err().contains("did not answer as zig does"));
            let other = if cfg!(windows) { "cmd" } else { "sh" };
            assert!(probe(other).is_none());
        }

        use crate::project::tests::TempDir;
        use crate::starter;

        /// A project folder, the person's decisions and the settings, in a folder of one test's own.
        fn places(t: &TempDir) -> Places {
            Places { projects: t.0.join("projects"), trust: t.0.join("trusted-projects.json"), settings: t.0.join("developer.json") }
        }

        fn trust(at: &Places, id: &str) {
            let found = project::load(&at.projects.join(id));
            assert!(found.is_sound(), "{:?}", found.problem);
            TrustStore::load(&at.trust).trust(&found, false).unwrap();
        }

        /// The zig these tests build with: the one `GRIDCRAFT_TEST_ZIG` names, or the one that is
        /// found. `None` on a machine that has none: the tests that need one then check nothing.
        fn zig() -> Option<String> {
            let named = std::env::var("GRIDCRAFT_TEST_ZIG").unwrap_or_default();
            find(&named).ok().map(|_| named)
        }

        #[test]
        fn only_a_trusted_c_project_with_a_recipe_is_built() {
            let t = TempDir::new("c-may");
            let at = places(&t);
            starter::create_in(&at.projects, &kit::C, "Blink", None).unwrap();
            // Not trusted: the compiler is not run over its files. (No compiler is looked for either.)
            let refused = build(&at, "blink", "gridcraft-no-such-compiler").unwrap_err();
            assert!(refused.contains("Blink is not trusted as it is now") && refused.contains("review and trust it first"), "{refused}");
            assert!(!built_dir(&at.settings).exists(), "nothing was made");
            // Trusted, and then changed: it is no longer the project that was trusted.
            trust(&at, "blink");
            t.write("projects/blink/src/main.c", "int main(void) { return 1; }\n");
            assert!(build(&at, "blink", "gridcraft-no-such-compiler").unwrap_err().contains("not trusted as it is now"));
            // Trusted as it is, and no compiler by that name.
            trust(&at, "blink");
            assert!(build(&at, "blink", "gridcraft-no-such-compiler").unwrap_err().contains("did not answer as zig does"));
            // A project of the first kit is not built, trusted or not; nor one without a recipe.
            starter::create_in(&at.projects, &kit::PYTHONCRAFT, "Clean Data", None).unwrap();
            trust(&at, "clean-data");
            assert_eq!(build(&at, "clean-data", "").unwrap_err(), "Clean Data is not a project of the C Development Kit.");
            t.write("projects/bare/project.toml", "id = \"bare\"\nname = \"Bare\"\nversion = \"1\"\nkit = \"c\"\n");
            trust(&at, "bare");
            assert!(build(&at, "bare", "").unwrap_err().contains("Bare has no C recipe"));
            for id in ["", "../blink", "Blink", "no-such-project"] {
                assert!(build(&at, id, "").is_err(), "{id:?}");
            }
            assert!(!built_dir(&at.settings).join("blink").exists() && !built_dir(&at.settings).join("clean-data").exists());
        }

        #[test]
        fn a_build_asked_for_runs_beside_the_window_and_says_how_it_ended() {
            let t = TempDir::new("c-builder");
            let at = places(&t);
            starter::create_in(&at.projects, &kit::C, "Blink", None).unwrap();
            let builder = Builder::new();
            assert_eq!(builder.progress(&at.projects, "blink"), None, "none was asked for");
            let told = Arc::new(AtomicUsize::new(0));
            let counted = told.clone();
            builder
                .start(&at, "blink", "", move || {
                    counted.fetch_add(1, Ordering::SeqCst);
                })
                .unwrap();
            // It is not trusted, so it ends at once: and the one who asked is told, once.
            let deadline = now() + Duration::from_secs(30);
            while told.load(Ordering::SeqCst) == 0 && now() < deadline {
                std::thread::sleep(Duration::from_millis(10));
            }
            assert_eq!((told.load(Ordering::SeqCst), builder.building()), (1, 0));
            let Some(Progress::Done(Err(why))) = builder.progress(&at.projects, "blink") else { panic!("it has ended") };
            assert!(why.contains("not trusted as it is now"), "{why}");
            assert_eq!(builder.progress(&t.0.join("elsewhere"), "blink"), None, "another folder's project is another project");
            assert!(builder.start(&at, "../blink", "", || {}).is_err());
        }

        #[test]
        fn a_trusted_c_project_is_built_and_the_program_is_kept_under_its_fingerprint() {
            let Some(zig) = zig() else { return };
            let t = TempDir::new("c-build");
            let at = places(&t);
            let made = starter::create_in(&at.projects, &kit::C, "Blink", None).unwrap();
            trust(&at, "blink");
            let built = build(&at, "blink", &zig).unwrap();
            let print = made.fingerprint.clone().unwrap();
            assert_eq!(built.program, program_for(&at.settings, "blink", &print));
            assert_eq!((built.sources, built.bytes), (1, fs::metadata(&built.program).unwrap().len()));
            assert!(built.compiler.starts_with("zig ") && built.bytes > 1000);
            assert_eq!(fs::read(&built.program).unwrap().get(..4), Some(b"\0asm".as_slice()), "a WebAssembly program");
            assert_eq!(standing(&at.settings, "blink", Some(&print)), Standing::Built { bytes: built.bytes });
            // The project is as it was: nothing was written into it, and it is still trusted.
            let after = project::load(&at.projects.join("blink"));
            assert_eq!((after.fingerprint.as_deref(), after.files.as_slice()), (Some(print.as_str()), made.files.as_slice()));
            assert!(TrustStore::load(&at.trust).status(&after).is_trusted());
            // The folder the compiler worked in is gone; what it keeps for itself stays, for next time.
            let work = built_dir(&at.settings).join(".work");
            assert!(fs::read_dir(&work).map(|mut left| left.next().is_none()).unwrap_or(true));
            // Its files change: the program kept is no longer the project's, and it is not built
            // again until the person has looked.
            t.write("projects/blink/src/more.c", "int more(void) { return 2; }\n");
            let changed = project::load(&at.projects.join("blink")).fingerprint.unwrap();
            assert_eq!(standing(&at.settings, "blink", Some(&changed)), Standing::Stale);
            assert!(build(&at, "blink", &zig).unwrap_err().contains("not trusted as it is now"));
            trust(&at, "blink");
            let again = build(&at, "blink", &zig).unwrap();
            assert_eq!((again.sources, again.program.clone()), (2, program_for(&at.settings, "blink", &changed)), "every .c file in src");
            assert!(!program_for(&at.settings, "blink", &print).exists(), "what was built from the files before is gone");
            assert_eq!(standing(&at.settings, "blink", Some(&changed)), Standing::Built { bytes: again.bytes });
        }

        #[test]
        fn a_build_takes_nothing_in_from_outside_the_project() {
            let Some(zig) = zig() else { return };
            let t = TempDir::new("c-outside");
            let at = places(&t);
            starter::create_in(&at.projects, &kit::C, "Sly", None).unwrap();
            t.write("outside/secret.h", "#define SECRET 42\n");
            let outside = t.0.join("outside/secret.h").display().to_string().replace('\\', "/");
            // A header from outside the project: the compiler says it read it, and nothing is kept.
            t.write("projects/sly/src/main.c", &format!("#include \"{outside}\"\nint main(void) {{ return SECRET; }}\n"));
            trust(&at, "sly");
            let refused = build(&at, "sly", &zig).unwrap_err();
            assert!(refused.contains("src/main.c reads a file that is not the project's") && refused.contains("secret.h"), "{refused}");
            assert_eq!(standing(&at.settings, "sly", project::load(&at.projects.join("sly")).fingerprint.as_deref()), Standing::NotBuilt);
            // The same file's bytes, taken in whole.
            t.write(
                "projects/sly/src/main.c",
                &format!("static const char held[] = {{\n#embed \"{outside}\"\n, 0}};\nint main(void) {{ return held[0]; }}\n"),
            );
            trust(&at, "sly");
            assert!(build(&at, "sly", &zig).unwrap_err().contains("reads a file that is not the project's"));
            // The assembler's own way in is closed: there is no inline assembly.
            t.write("projects/sly/src/main.c", &format!("__asm__(\".incbin \\\"{outside}\\\"\");\nint main(void) {{ return 0; }}\n"));
            trust(&at, "sly");
            let refused = build(&at, "sly", &zig).unwrap_err();
            assert!(refused.contains("src/main.c does not compile") && refused.contains("inline assembly is disabled"), "{refused}");
            // What git keeps in the project is not the project's either.
            t.write("projects/sly/.git/hidden.h", "#define HIDDEN 1\n");
            t.write("projects/sly/src/main.c", "#include \"../.git/hidden.h\"\nint main(void) { return HIDDEN; }\n");
            trust(&at, "sly");
            assert!(build(&at, "sly", &zig).unwrap_err().contains("reads a file that is not the project's"));
            // A mistake in the C is said with the file's name as the project has it, and its own headers are its own.
            t.write("projects/sly/src/twice.h", "int twice(int);\n");
            t.write("projects/sly/src/main.c", "#include \"twice.h\"\nint main(void) { return twice(2) }\n");
            trust(&at, "sly");
            let wrong = build(&at, "sly", &zig).unwrap_err();
            assert!(
                wrong.contains("src/main.c does not compile") && wrong.contains("src/main.c:2:") && !wrong.contains(&t.0.display().to_string()),
                "{wrong}"
            );
            t.write("projects/sly/src/main.c", "#include \"twice.h\"\nint twice(int x) { return 2 * x; }\nint main(void) { return twice(2); }\n");
            trust(&at, "sly");
            assert_eq!(build(&at, "sly", &zig).unwrap().sources, 1);
            assert!(!built_dir(&at.settings).join(".work").read_dir().is_ok_and(|mut left| left.next().is_some()), "no build left its folder behind");
        }

        #[test]
        fn a_command_that_runs_too_long_is_stopped_and_what_it_prints_is_kept_short() {
            let mut slow = if cfg!(windows) { command("ping") } else { command("sleep") };
            if cfg!(windows) {
                slow.args(["-n", "30", "127.0.0.1"]);
            } else {
                slow.arg("30");
            }
            let began = now();
            let stopped = run(slow, Duration::from_millis(300)).err().unwrap();
            assert!(stopped.contains("was stopped") && began.elapsed() < Duration::from_secs(10), "{stopped}");
            assert!(run(command("gridcraft-no-such-program"), Duration::from_secs(5)).err().unwrap().contains("could not be started"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(text: &str) -> Result<Recipe, String> {
        #[derive(Deserialize)]
        struct Holder {
            c: Raw,
        }
        let holder: Holder = toml::from_str(text).map_err(|e| e.message().to_string())?;
        Recipe::read(holder.c)
    }

    fn files(names: &[&str]) -> Vec<String> {
        let mut all: Vec<String> = names.iter().map(|name| name.to_string()).collect();
        all.sort();
        all
    }

    #[test]
    fn a_recipe_is_a_list_of_sources_and_a_few_named_options() {
        let plain = read("[c]\nsources = [\"src/main.c\"]").unwrap();
        assert_eq!(plain.sources, ["src/main.c"]);
        assert!(plain.include.is_empty() && plain.define.is_empty());
        assert_eq!((plain.optimize, plain.entry), (Optimize::Size, Entry::Main), "what it is when nothing is said");
        let full = read("[c]\nsources = [\"src\", \"port/glue.c\"]\ninclude = [\"src\", \"vendor/doom\"]\ndefine = [\"_DEFAULT_SOURCE\", \"LEVEL=2\", \"V=1.9-b\"]\noptimize = \"speed\"\nentry = \"none\"").unwrap();
        assert_eq!((full.sources.len(), full.include.len(), full.define.len()), (2, 2, 3));
        assert_eq!((full.optimize, full.entry), (Optimize::Speed, Entry::None));
        for (optimize, flag) in [(Optimize::Size, "-Os"), (Optimize::Speed, "-O2"), (Optimize::None, "-O0")] {
            assert_eq!((Optimize::parse(optimize.code()), optimize.flag()), (Some(optimize), flag));
        }
        for entry in Entry::ALL {
            assert_eq!(Entry::parse(entry.code()), Some(entry));
        }
    }

    #[test]
    fn a_recipe_can_say_nothing_that_becomes_an_option_of_its_own() {
        let with = |line: &str| read(&format!("[c]\nsources = [\"src/main.c\"]\n{line}"));
        for bad in [
            // No sources, or sources that are not paths inside the project.
            "[c]",
            "[c]\nsources = []",
            "[c]\nsources = [\"../outside.c\"]",
            "[c]\nsources = [\"C:/Windows/x.c\"]",
            "[c]\nsources = [\"/etc/x.c\"]",
            "[c]\nsources = [\".git/x.c\"]",
            "[c]\nsources = \"src/main.c\"",
        ] {
            assert!(read(bad).is_err(), "{bad:?}");
        }
        for bad in [
            "include = [\"../include\"]",
            "include = [\"/usr/include\"]",
            // A definition is a name, or a name and a plain value: no option, path or quote.
            "define = [\"-o evil\"]",
            "define = [\"X -include /etc/passwd\"]",
            "define = [\"X=\\\"quoted\\\"\"]",
            "define = [\"X=/etc/passwd\"]",
            "define = [\"X=a b\"]",
            "define = [\"9LIVES\"]",
            "define = [\"\"]",
            "define = [\"@response\"]",
            "optimize = \"-O3 -fplugin=x\"",
            "optimize = \"fast\"",
            "entry = \"_start\"",
            "entry = 7",
        ] {
            assert!(with(bad).is_err(), "{bad:?}");
        }
        assert!(with(&format!("define = [{}]", vec!["\"A\""; MAX_DEFINES + 1].join(","))).is_err());
        assert!(with(&format!("include = [{}]", vec!["\"src\""; MAX_INCLUDES + 1].join(","))).is_err());
        // What it doesn't know it ignores, like the rest of a manifest: flags is not a key it has.
        assert!(with("flags = [\"-fplugin=evil\"]").is_ok());
    }

    #[test]
    fn the_sources_are_the_files_named_and_the_c_files_of_the_folders_named() {
        let project =
            files(&["project.toml", "src/main.c", "src/b.c", "src/a.c", "src/notes.txt", "src/deep/skipped.c", "port/glue.c", "include/x.h"]);
        let recipe = |sources: &str| read(&format!("[c]\nsources = {sources}\ninclude = [\"include\"]")).unwrap();
        assert_eq!(recipe("[\"src/main.c\"]").sources_among(&project).unwrap(), ["src/main.c"]);
        // A folder: every .c file directly in it, by name; not what is deeper, not what is not C.
        assert_eq!(recipe("[\"src\"]").sources_among(&project).unwrap(), ["src/a.c", "src/b.c", "src/main.c"]);
        // In the recipe's order, each file once.
        assert_eq!(
            recipe("[\"port/glue.c\", \"src\", \"src/a.c\"]").sources_among(&project).unwrap(),
            ["port/glue.c", "src/a.c", "src/b.c", "src/main.c"]
        );
        assert_eq!(recipe("[\"src/deep\"]").sources_among(&project).unwrap(), ["src/deep/skipped.c"]);
        assert_eq!(recipe("[\"src/gone.c\"]").sources_among(&project).unwrap_err(), "its source, src/gone.c, is not in it");
        assert_eq!(recipe("[\"src/notes.txt\"]").sources_among(&project).unwrap_err(), "its source, src/notes.txt, is not a .c file");
        assert_eq!(recipe("[\"include\"]").sources_among(&project).unwrap_err(), "its sources folder, include, holds no .c file");
        assert!(recipe("[\"src\"]").includes_among(&project).is_ok());
        let missing = read("[c]\nsources = [\"src\"]\ninclude = [\"headers\"]").unwrap();
        assert_eq!(missing.includes_among(&project).unwrap_err(), "its include folder, headers, is not in it");
    }

    #[test]
    fn the_files_a_compile_read_are_taken_from_what_the_compiler_wrote() {
        // As zig writes it on Windows: the object first, with a drive's colon in its name.
        let windows = "C:\\Users\\ada\\built\\.cache\\local\\tmp\\973a-main.o: \\\n  C:/Users/ada/projects/blink/src/main.c \\\n  C:/tools/zig/lib/libc/include/wasm-wasi-musl/stdio.h \\\n  C:/Users/ada/projects/blink/inc/twice.h\n";
        assert_eq!(
            dependencies(windows),
            [
                "C:/Users/ada/projects/blink/src/main.c",
                "C:/tools/zig/lib/libc/include/wasm-wasi-musl/stdio.h",
                "C:/Users/ada/projects/blink/inc/twice.h"
            ]
        );
        // A space in a name is written with a backslash before it; so is #; a dollar is doubled.
        let odd = "0.o: /home/ada/my\\ projects/a\\#b/x.c /tmp/pri$$ce.h \\\r\n /etc/passwd\n";
        assert_eq!(dependencies(odd), ["/home/ada/my projects/a#b/x.c", "/tmp/pri$ce.h", "/etc/passwd"]);
        // A Windows path's own backslashes stay.
        assert_eq!(dependencies("0.o: C:\\outside\\secret.txt"), ["C:\\outside\\secret.txt"]);
        assert!(dependencies("").is_empty() && dependencies("0.o:").is_empty() && dependencies("0.o: \\\n").is_empty());
    }

    #[test]
    fn what_is_built_is_kept_beside_the_settings_under_the_fingerprint_it_was_built_from() {
        let t = crate::project::tests::TempDir::new("c-kept");
        let settings = t.0.join("developer.json");
        let (print, other) = ("a".repeat(64), "b".repeat(64));
        assert_eq!(built_dir(&settings), t.0.join("built"));
        assert_eq!(program_for(&settings, "blink", &print), t.0.join("built").join("blink").join(&print).join("program.wasm"));
        assert_eq!(standing(&settings, "blink", Some(&print)), Standing::NotBuilt);
        // A program kept under another fingerprint is not this project's: it is stale.
        t.write(&format!("built/blink/{other}/program.wasm"), "\0asm....");
        assert_eq!(standing(&settings, "blink", Some(&print)), Standing::Stale);
        assert_eq!(standing(&settings, "blink", None), Standing::Stale, "a project that can't be fingerprinted has none of its own");
        t.write(&format!("built/blink/{print}/program.wasm"), "\0asm........");
        assert_eq!(standing(&settings, "blink", Some(&print)), Standing::Built { bytes: 12 });
        // Another project's, and names that are no project's or no fingerprint's, find nothing.
        assert_eq!(standing(&settings, "other", Some(&print)), Standing::NotBuilt);
        assert_eq!(standing(&settings, "../blink", Some(&print)), Standing::NotBuilt);
        assert_eq!(standing(&settings, "blink", Some("../../blink/x")), Standing::Stale, "not a fingerprint: only the others are seen");
        // What is handed out to be run is the program kept under the project's own fingerprint,
        // and never one kept under another.
        assert_eq!(program_bytes(&settings, "blink", &print).as_deref(), Some("\0asm........".as_bytes()));
        assert_eq!(program_bytes(&settings, "blink", &other).as_deref(), Some("\0asm....".as_bytes()));
        let unbuilt = "c".repeat(64);
        assert_eq!(program_bytes(&settings, "blink", &unbuilt), None, "nothing was built from those files");
        for (id, fingerprint) in
            [("other", print.as_str()), ("../blink", print.as_str()), ("blink", "../../blink/x"), ("blink", ""), ("", print.as_str())]
        {
            assert_eq!(program_bytes(&settings, id, fingerprint), None, "{id:?} {fingerprint:?}");
        }
        // A folder where the program would be is not a program.
        std::fs::create_dir_all(program_for(&settings, "blink", &unbuilt)).unwrap();
        assert_eq!(program_bytes(&settings, "blink", &unbuilt), None);
    }
}
