//! A project's manifest, `project.toml`: who it is and what it asks to be allowed.
//!
//! ```toml
//! id = "clean-data"                  # the project's folder name
//! name = "Clean Data"
//! version = "0.1.0"
//! author = "Ada Lovelace"            # optional
//! description = "Tidies a table"     # optional
//! permissions = ["read", "write", "python"]
//! workbook = "Budget.xlsm"           # optional: the workbook this is the code of, by its file's name
//! kit = "pythoncraft"                # optional: the development kit it is a project of
//!
//! [panel]
//! entry = "panel/index.html"         # optional: the page GridCraft serves as its panel
//!
//! [python]
//! scripts = "python"                 # optional: the folder its panel may run scripts from
//!
//! [c]                                # optional, for a project of the C kit: what is built
//! sources = ["src"]                  # (`crate::c` has the whole of it)
//! ```
//!
//! A manifest is written by a stranger: everything in it is checked, and anything not understood
//! where it matters (a permission, a path that leaves the project) makes the manifest invalid
//! rather than being guessed at.

use serde::{Deserialize, Serialize};

/// The file's name in a project's folder.
pub const FILE: &str = "project.toml";
/// The name it had while a project was called a module. A folder that has it, and no
/// `project.toml`, is read as before: renaming the file for the person would change what
/// they trusted.
pub const LEGACY_FILE: &str = "module.toml";

/// Which of a project's files is its manifest: `project.toml`, or the older name where that
/// is all there is. A folder with both is refused: nobody could tell which of them counts.
pub fn file_among<'a>(files: impl IntoIterator<Item = &'a str>) -> Result<&'static str, String> {
    let (mut now, mut before) = (false, false);
    for file in files {
        now |= file == FILE;
        before |= file == LEGACY_FILE;
    }
    match (now, before) {
        (true, false) => Ok(FILE),
        (false, true) => Ok(LEGACY_FILE),
        (true, true) => Err(format!("it has both a {FILE} and a {LEGACY_FILE}, and only one of them may say what it is")),
        (false, false) => Err(format!("it has no {FILE}")),
    }
}

/// Whether a path inside a project is its manifest, under either name.
pub fn is_manifest(path: &str) -> bool {
    path == FILE || path == LEGACY_FILE
}
/// The largest manifest read (bytes).
pub const MAX_BYTES: u64 = 64 * 1024;
/// The kit of a project whose manifest names none: the one there was before there were two.
pub const DEFAULT_KIT: &str = "pythoncraft";
/// That kit's id until it was named PythonCraft (2026-10-14). A manifest, or a kit's package,
/// that names it so is still read: nothing that was made has to be changed, or trusted again.
pub const EARLIER_KIT: &str = "anie";

/// A kit's id as it is now, for one as a file names it.
pub fn kit_id(named: &str) -> &str {
    if named == EARLIER_KIT { DEFAULT_KIT } else { named }
}

/// Something a project asks to be allowed to do. The user grants them together, by trusting it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Permission {
    Read,
    Write,
    Files,
    Network,
    Python,
}

impl Permission {
    pub const ALL: [Permission; 5] = [Permission::Read, Permission::Write, Permission::Files, Permission::Network, Permission::Python];

    /// The word a manifest uses.
    pub fn code(self) -> &'static str {
        match self {
            Permission::Read => "read",
            Permission::Write => "write",
            Permission::Files => "files",
            Permission::Network => "network",
            Permission::Python => "python",
        }
    }

    pub fn parse(code: &str) -> Option<Permission> {
        Permission::ALL.into_iter().find(|p| p.code() == code)
    }

    /// What granting it means, in the words the user is shown before deciding.
    pub fn meaning(self) -> &'static str {
        match self {
            Permission::Read => "See what is in your open workbooks",
            Permission::Write => "Change your open workbooks (every change can be undone)",
            Permission::Files => "Open, save and export files",
            Permission::Network => "Let its panel talk to the internet",
            Permission::Python => "Run its Python scripts. This cannot be limited: a script can do anything a program you run can do",
        }
    }

    /// Granting it is the same as running a program from the project's author.
    pub fn is_unlimited(self) -> bool {
        self == Permission::Python
    }
}

/// A checked manifest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Manifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    /// What it asks for: sorted, each once.
    pub permissions: Vec<Permission>,
    /// The panel's page, as a path inside the project (`/` separators).
    pub panel: Option<String>,
    /// The folder of scripts its panel may run, as a path inside the project.
    pub python: Option<String>,
    /// The workbook it is the code of, by its file's name. `None`: it is for any workbook.
    /// It only says what the project is for: nothing is allowed or run because of it.
    pub workbook: Option<String>,
    /// The development kit it is a project of, by the kit's id ([`DEFAULT_KIT`] when it names
    /// none). Whether this build has such a kit, and what a project of it may hold, is for
    /// `kit::check` to say.
    pub kit: String,
    /// What of it is built, and how: the C kit's recipe.
    pub c: Option<crate::c::Recipe>,
    /// The addresses it would reach over the network, through GridCraft (`[network] allow`).
    /// `None` when it has no such list. Naming them allows nothing: the person turns the
    /// network on for exactly these, apart from trusting the project ([`crate::trust`]).
    pub network: Option<Vec<Address>>,
}

/// The most addresses a project may name to reach.
pub const MAX_ADDRESSES: usize = 16;

/// One place on the network a project asks to reach: a host, by its name or its number, and
/// a port. Never a pattern: nothing here names every host, or every port.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address {
    pub host: String,
    pub port: u16,
}

impl Address {
    /// Reads `host:port`. A host is a name (lowercase letters, digits and hyphens, in parts
    /// joined by dots) or four numbers. A name whose last part is all digits must be four
    /// numbers: nothing is left that two readers could take for different hosts.
    pub fn parse(text: &str) -> Result<Address, String> {
        let refuse = |why: &str| format!("its network address {:?} {why}", text.chars().take(80).collect::<String>());
        let (host, port) = text.rsplit_once(':').ok_or_else(|| refuse("is not a host and a port, like example.org:26000"))?;
        let port = port
            .parse::<u16>()
            .ok()
            .filter(|number| *number != 0 && number.to_string() == port)
            .ok_or_else(|| refuse("has no port from 1 to 65535"))?;
        let parts: Vec<&str> = host.split('.').collect();
        let named = !host.is_empty()
            && host.len() <= 253
            && parts.iter().all(|part| {
                !part.is_empty()
                    && part.len() <= 63
                    && !part.starts_with('-')
                    && !part.ends_with('-')
                    && part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            });
        if !named {
            return Err(refuse("does not name a host: lowercase letters, digits, hyphens and dots"));
        }
        let numbered = parts.last().is_some_and(|last| last.chars().all(|c| c.is_ascii_digit()));
        if numbered && host.parse::<std::net::Ipv4Addr>().is_err() {
            return Err(refuse("ends in a number and is not four numbers, like 192.168.1.20"));
        }
        Ok(Address { host: host.to_string(), port })
    }

    /// As it is written and shown: `host:port`.
    pub fn text(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Whether it is on the person's own machine or their own network, as far as its name or
    /// number says: such an address reaches what is behind their router. (A name can still
    /// turn out to stand for a number that is: whoever carries packets checks that as well.)
    pub fn is_local(&self) -> bool {
        if let Ok(number) = self.host.parse::<std::net::Ipv4Addr>() {
            return number.is_loopback() || number.is_private() || number.is_link_local() || number.is_unspecified() || number.is_broadcast();
        }
        let home = [".localhost", ".local", ".lan", ".home", ".internal", ".home.arpa"];
        self.host == "localhost" || !self.host.contains('.') || home.iter().any(|ending| self.host.ends_with(ending))
    }
}

#[derive(Deserialize)]
struct Raw {
    id: Option<String>,
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    description: Option<String>,
    permissions: Option<Vec<String>>,
    workbook: Option<String>,
    kit: Option<String>,
    c: Option<crate::c::Raw>,
    panel: Option<RawPanel>,
    python: Option<RawPython>,
    network: Option<RawNetwork>,
}

#[derive(Deserialize)]
struct RawNetwork {
    allow: Option<Vec<String>>,
}

/// The addresses a `[network]` table names: at least one, each once, in the order written.
fn addresses(raw: RawNetwork) -> Result<Vec<Address>, String> {
    let named = raw.allow.unwrap_or_default();
    if named.is_empty() {
        return Err("its [network] names no address it would reach (allow = [\"host:port\"])".into());
    }
    if named.len() > MAX_ADDRESSES {
        return Err(format!("its [network] names more than {MAX_ADDRESSES} addresses"));
    }
    let mut found: Vec<Address> = Vec::new();
    for text in &named {
        let address = Address::parse(text)?;
        if !found.contains(&address) {
            found.push(address);
        }
    }
    Ok(found)
}

#[derive(Deserialize)]
struct RawPanel {
    entry: Option<String>,
}

#[derive(Deserialize)]
struct RawPython {
    scripts: Option<String>,
}

/// A project's id: lowercase letters, digits and hyphens, starting with a letter or digit, at most
/// 64 characters. It is also the folder's name and part of URLs, so nothing else is allowed.
pub fn valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    let first = chars.next().is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    first && id.len() <= 64 && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// A line of text for people: not empty, no control characters, not too long.
fn text(value: Option<String>, what: &str, max: usize, required: bool) -> Result<String, String> {
    let value = value.unwrap_or_default();
    let value = value.trim();
    if value.is_empty() && required {
        return Err(format!("it has no {what}"));
    }
    if value.chars().count() > max {
        return Err(format!("its {what} is longer than {max} characters"));
    }
    if value.chars().any(char::is_control) {
        return Err(format!("its {what} contains control characters"));
    }
    Ok(value.to_string())
}

/// Whether a file or folder of this name, anywhere in a project's folder, is what git keeps
/// there (`.git`, whatever its capitals). It is not part of the project: it changes with almost
/// everything git does, so it is left out of the project's files and of its fingerprint. That
/// is safe only because nothing in it is ever served, run, built, read out or written: see
/// [`inner_path`], which every path into a project goes through.
pub fn kept_by_git(name: &str) -> bool {
    name.eq_ignore_ascii_case(".git")
}

/// Whether a name could reach what git keeps on some file system: `.git` itself, the short
/// name Windows gives it (`GIT~1`), or either with characters in it that a Mac's older file
/// system leaves out when it compares names.
fn reaches_git(part: &str) -> bool {
    let left_out = |c: &char| matches!(c, '\u{200c}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{206a}'..='\u{206f}' | '\u{feff}');
    let seen: String = part.chars().filter(|c| !left_out(c)).collect();
    let short = seen.to_ascii_lowercase().strip_prefix("git~").is_some_and(|number| !number.is_empty() && number.chars().all(|c| c.is_ascii_digit()));
    kept_by_git(&seen) || short
}

/// A path inside the project: relative, `/` separators, plain names only. Nothing that could name a
/// file outside the project's folder on any system, and nothing in what git keeps there.
pub fn inner_path(path: &str) -> Result<String, String> {
    let bad = |why: &str| Err(format!("the path {path:?} {why}"));
    if path.is_empty() || path.len() > 200 {
        return bad("is empty or too long");
    }
    if path.starts_with('/') || path.contains('\\') || path.contains(':') {
        return bad("must be relative, with / between folders");
    }
    for part in path.split('/') {
        if part.is_empty() || part == "." || part == ".." {
            return bad("must not contain empty, . or .. parts");
        }
        if part.chars().any(|c| c.is_control()) || part.starts_with(' ') || part.ends_with(' ') || part.ends_with('.') {
            return bad("has a part that is not a plain name");
        }
        if reaches_git(part) {
            return bad("is in what git keeps, which is not part of a project");
        }
    }
    Ok(path.to_string())
}

/// The longest name of a workbook a manifest may give (characters).
pub const MAX_WORKBOOK: usize = 255;

/// The name of the workbook a project is the code of: a file's name, not a path, since a path
/// is another on every computer. Empty is none.
pub fn workbook_name(name: &str) -> Result<Option<String>, String> {
    let name = name.trim();
    if name.is_empty() {
        return Ok(None);
    }
    if name.chars().count() > MAX_WORKBOOK {
        return Err(format!("its workbook's name is longer than {MAX_WORKBOOK} characters"));
    }
    if name.chars().any(|c| c.is_control() || matches!(c, '/' | '\\' | ':')) {
        return Err(format!("its workbook, {name:?}, is not a file's name (a name only, without folders)"));
    }
    Ok(Some(name.to_string()))
}

/// Whether two names are the same workbook's: a file's name, whatever its capitals.
pub fn same_workbook(one: &str, other: &str) -> bool {
    one.trim().to_lowercase() == other.trim().to_lowercase()
}

/// Reads and checks a manifest. `folder` is the name of the folder it was found in, which the id
/// must match.
pub fn parse(source: &str, folder: &str) -> Result<Manifest, String> {
    if source.len() as u64 > MAX_BYTES {
        return Err(format!("{FILE} is larger than {} KB", MAX_BYTES / 1024));
    }
    let raw: Raw = toml::from_str(source).map_err(|e| format!("{FILE} can't be read: {}", e.message()))?;
    let id = raw.id.unwrap_or_default();
    if !valid_id(&id) {
        return Err(format!("its id {id:?} is not lowercase letters, digits and hyphens (at most 64)"));
    }
    if id != folder {
        return Err(format!("its id {id:?} is not its folder's name, {folder:?}"));
    }
    let mut permissions = Vec::new();
    for code in raw.permissions.unwrap_or_default() {
        let p = Permission::parse(&code).ok_or_else(|| format!("it asks for {code:?}, which GridCraft doesn't know"))?;
        if !permissions.contains(&p) {
            permissions.push(p);
        }
    }
    permissions.sort();
    let kit = match raw.kit {
        None => DEFAULT_KIT.to_string(),
        Some(kit) if valid_id(&kit) => kit_id(&kit).to_string(),
        Some(kit) => return Err(format!("its kit, {kit:?}, is not a kit's id (lowercase letters, digits and hyphens)")),
    };
    Ok(Manifest {
        id,
        name: text(raw.name, "name", 80, true)?,
        version: text(raw.version, "version", 32, true)?,
        author: text(raw.author, "author", 80, false)?,
        description: text(raw.description, "description", 500, false)?,
        permissions,
        panel: raw.panel.and_then(|p| p.entry).map(|p| inner_path(&p)).transpose()?,
        python: raw.python.and_then(|p| p.scripts).map(|p| inner_path(&p)).transpose()?,
        workbook: raw.workbook.map(|name| workbook_name(&name)).transpose()?.flatten(),
        kit,
        c: raw.c.map(crate::c::Recipe::read).transpose()?,
        network: raw.network.map(addresses).transpose()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = r#"
id = "clean-data"
name = "Clean Data"
version = "0.1.0"
author = "Ada Lovelace"
description = "Tidies a table"
permissions = ["write", "read", "python", "read"]

[panel]
entry = "panel/index.html"

[python]
scripts = "python"
"#;

    #[test]
    fn reads_a_manifest() {
        let m = parse(GOOD, "clean-data").unwrap();
        assert_eq!((m.id.as_str(), m.name.as_str(), m.version.as_str(), m.author.as_str()), ("clean-data", "Clean Data", "0.1.0", "Ada Lovelace"));
        assert_eq!(m.permissions, [Permission::Read, Permission::Write, Permission::Python], "sorted, each once");
        assert_eq!((m.panel.as_deref(), m.python.as_deref()), (Some("panel/index.html"), Some("python")));
        // The least a manifest can say: it asks for nothing and has no panel or scripts.
        let m = parse("id = \"x\"\nname = \"X\"\nversion = \"1\"", "x").unwrap();
        assert!(m.permissions.is_empty() && m.panel.is_none() && m.python.is_none() && m.author.is_empty());
        assert_eq!(m.workbook, None, "a project that names no workbook is for any");
        assert_eq!(m.kit, DEFAULT_KIT, "and one that names no kit is of the kit there was before there were two");
        assert_eq!(m.c, None, "and it has no C recipe");
        // A recipe is read with the rest, and a recipe that won't do makes the manifest invalid.
        let built = parse("id = \"x\"\nname = \"X\"\nversion = \"1\"\nkit = \"c\"\n[c]\nsources = [\"src\"]", "x").unwrap();
        assert_eq!(built.c.map(|recipe| recipe.sources), Some(vec!["src".to_string()]));
        assert!(parse("id = \"x\"\nname = \"X\"\nversion = \"1\"\nkit = \"c\"\n[c]\nsources = [\"../x.c\"]", "x").is_err());
        assert!(
            parse("id = \"x\"\nname = \"X\"\nversion = \"1\"\nkit = \"c\"\n[c]\ndefine = [\"A\"]", "x").unwrap_err().contains("names no sources")
        );
        // Keys it doesn't know are ignored: a newer project still loads.
        assert!(parse("id = \"x\"\nname = \"X\"\nversion = \"1\"\nfuture = true\n[panel]\nwidth = 300", "x").is_ok());
    }

    #[test]
    fn refuses_what_it_does_not_understand_or_trust() {
        let with = |from: &str, to: &str| GOOD.replace(from, to);
        let bad = [
            ("the folder is another project's", GOOD.to_string(), "other"),
            ("an unknown permission", with("\"python\"", "\"everything\""), "clean-data"),
            ("a path that climbs out", with("panel/index.html", "../secrets.html"), "clean-data"),
            ("an absolute path", with("panel/index.html", "/etc/passwd"), "clean-data"),
            ("a drive path", with("scripts = \"python\"", "scripts = \"C:/Windows\""), "clean-data"),
            ("a backslash path", with("panel/index.html", "panel\\\\index.html"), "clean-data"),
            ("no name", with("name = \"Clean Data\"", "name = \"  \""), "clean-data"),
            ("no version", with("version = \"0.1.0\"", ""), "clean-data"),
            ("a name with control characters", with("Clean Data", "Clean\\u0007Data"), "clean-data"),
            ("not TOML", "id = [".to_string(), "clean-data"),
            ("permissions of the wrong type", with("[\"write\", \"read\", \"python\", \"read\"]", "\"read\""), "clean-data"),
        ];
        for (what, source, folder) in bad {
            assert!(parse(&source, folder).is_err(), "{what} should be refused");
        }
        assert!(parse(&format!("id = \"x\"\nname = \"{}\"\nversion = \"1\"", "n".repeat(81)), "x").is_err(), "a name too long");
        assert!(parse(&" ".repeat(MAX_BYTES as usize + 1), "x").is_err(), "a manifest too large");
    }

    #[test]
    fn a_project_can_name_the_workbook_it_is_the_code_of() {
        let named = |name: &str| parse(&format!("id = \"x\"\nname = \"X\"\nversion = \"1\"\nworkbook = {name}"), "x");
        assert_eq!(named("\"Budget.xlsm\"").unwrap().workbook.as_deref(), Some("Budget.xlsm"));
        assert_eq!(named("\"  Q3 report (v2).xlsm \"").unwrap().workbook.as_deref(), Some("Q3 report (v2).xlsm"));
        assert_eq!(named("\"   \"").unwrap().workbook, None, "an empty name is none");
        // A name only: a path is another on every computer, and could say more than a name.
        for bad in ["\"C:/Users/ada/Budget.xlsm\"", "\"../Budget.xlsm\"", "'folder\\Budget.xlsm'", "\"a\\u0007b.xlsm\"", "7"] {
            assert!(named(bad).is_err(), "{bad}");
        }
        assert!(named(&format!("\"{}\"", "n".repeat(MAX_WORKBOOK + 1))).is_err(), "a name too long");
        assert!(same_workbook("Budget.xlsm", " budget.XLSM ") && same_workbook("Übersicht.xlsm", "übersicht.xlsm"));
        assert!(!same_workbook("Budget.xlsm", "Budget.xlsx") && !same_workbook("Budget.xlsm", ""));
    }

    #[test]
    fn a_project_names_the_addresses_it_would_reach_and_nothing_wider() {
        let with = |more: &str| parse(&format!("id = \"x\"\nname = \"X\"\nversion = \"1\"\n{more}"), "x");
        assert_eq!(with("").unwrap().network, None, "a project that names none has no list");
        let named = with("[network]\nallow = [\"quake.example.org:26000\", \"192.168.1.20:26000\", \"quake.example.org:26000\"]").unwrap();
        let list = named.network.unwrap();
        assert_eq!(list.iter().map(Address::text).collect::<Vec<_>>(), ["quake.example.org:26000", "192.168.1.20:26000"], "each once, in order");
        assert_eq!((list[0].is_local(), list[1].is_local()), (false, true));
        // What is on the person's own machine or network, by name or by number.
        for (host, local) in [
            ("localhost", true),
            ("printer", true),
            ("nas.local", true),
            ("router.lan", true),
            ("127.0.0.1", true),
            ("10.1.2.3", true),
            ("172.16.0.9", true),
            ("169.254.1.1", true),
            ("0.0.0.0", true),
            ("8.8.8.8", false),
            ("172.32.0.1", false),
            ("example.org", false),
            ("local.example.org", false),
        ] {
            assert_eq!(Address::parse(&format!("{host}:80")).unwrap().is_local(), local, "{host}");
        }
        // Never a pattern, a path, a scheme or a number two readers could take two ways.
        for bad in [
            "",
            "example.org",
            "example.org:",
            "example.org:0",
            "example.org:65536",
            "example.org:80/path",
            "*:80",
            "*.example.org:80",
            "Example.org:80",
            "udp://example.org:80",
            "exa mple.org:80",
            "-a.org:80",
            "a-.org:80",
            "a..org:80",
            ".org:80",
            "1.2.3:80",
            "999.1.1.1:80",
            "0x7f.0.0.1:80",
            "example.1:80",
            "[::1]:80",
            "example.org:+80",
            "example.org:080",
        ] {
            assert!(Address::parse(bad).is_err(), "{bad:?}");
        }
        for bad in ["[network]", "[network]\nallow = []", "[network]\nallow = [\"example.org\"]", "[network]\nallow = \"example.org:80\""] {
            assert!(with(bad).is_err(), "{bad:?}");
        }
        let many: Vec<String> = (0..=MAX_ADDRESSES).map(|n| format!("\"h{n}.example.org:80\"")).collect();
        assert!(with(&format!("[network]\nallow = [{}]", many.join(","))).unwrap_err().contains("more than 16"));
    }

    #[test]
    fn a_project_can_name_its_kit() {
        let named = |kit: &str| parse(&format!("id = \"x\"\nname = \"X\"\nversion = \"1\"\nkit = {kit}"), "x");
        assert_eq!(named("\"c\"").unwrap().kit, "c");
        assert_eq!(named("\"pythoncraft\"").unwrap().kit, "pythoncraft");
        // The first kit by the id it had before it was named: read as its id now.
        assert_eq!(named("\"anie\"").unwrap().kit, "pythoncraft");
        assert_eq!((kit_id("anie"), kit_id("pythoncraft"), kit_id("c"), kit_id("")), ("pythoncraft", "pythoncraft", "c", ""));
        // Any id is read: whether this build has such a kit is not the manifest's to say.
        assert_eq!(named("\"some-other-kit\"").unwrap().kit, "some-other-kit");
        for bad in ["\"\"", "\"C\"", "\"c kit\"", "\"../c\"", "7", "[\"c\"]"] {
            assert!(named(bad).is_err(), "{bad}");
        }
        assert!(named("\"C\"").unwrap_err().contains("is not a kit's id"));
    }

    #[test]
    fn what_git_keeps_is_not_a_path_into_a_project() {
        assert!(kept_by_git(".git") && kept_by_git(".GIT") && kept_by_git(".Git"));
        assert!(!kept_by_git(".gitignore") && !kept_by_git("git") && !kept_by_git(".git2") && !kept_by_git("") && !kept_by_git(".github"));
        for path in [
            ".git",
            ".git/config",
            ".git/hooks/pre-commit",
            ".GIT/config",
            "src/.git/config",
            "vendor/doom/.Git/HEAD",
            // The short name Windows gives the folder, and names an older Mac takes for it.
            "git~1/config",
            "GIT~1/hooks/pre-commit",
            "src/git~12/x",
            ".g\u{200c}it/config",
            "\u{feff}.git/config",
            "gi\u{200d}t~1/config",
        ] {
            let refused = inner_path(path).unwrap_err();
            assert!(refused.contains("what git keeps"), "{path:?}: {refused}");
        }
        // What only looks like it is an ordinary file.
        for path in [".gitignore", ".gitattributes", ".gitmodules", ".github/workflows/ci.yml", "git/x", "src/git~x/y", "digit~1/x", "git~/x"] {
            assert_eq!(inner_path(path).as_deref(), Ok(path));
        }
        // A manifest can't point into it either.
        let pointing = "id = \"x\"\nname = \"X\"\nversion = \"1\"\n[panel]\nentry = \".git/index.html\"";
        assert!(parse(pointing, "x").unwrap_err().contains("what git keeps"));
    }

    #[test]
    fn the_manifest_is_project_toml_or_the_name_it_had_and_never_both() {
        assert_eq!(file_among(["panel/index.html", "project.toml", "python/clean.py"]), Ok(FILE));
        assert_eq!(file_among(["module.toml", "panel/index.html"]), Ok(LEGACY_FILE), "what was made before the name changed is read");
        assert!(file_among(["module.toml", "project.toml"]).unwrap_err().contains("both a project.toml and a module.toml"));
        assert_eq!(file_among(["panel/index.html"]).unwrap_err(), "it has no project.toml");
        assert_eq!(file_among([]).unwrap_err(), "it has no project.toml");
        // Only at the top of the folder, and only these two spellings.
        assert!(file_among(["panel/project.toml", "Project.toml", "project.toml.bak"]).is_err());
        assert!(is_manifest("project.toml") && is_manifest("module.toml") && !is_manifest("panel/project.toml") && !is_manifest("rebuilt.toml"));
    }

    #[test]
    fn ids_and_inner_paths() {
        for id in ["a", "clean-data", "x2", "0day", &"a".repeat(64)] {
            assert!(valid_id(id), "{id}");
        }
        for id in ["", "-x", "Clean", "a b", "a_b", "a/b", "a.b", "..", "ünï", &"a".repeat(65)] {
            assert!(!valid_id(id), "{id:?}");
        }
        for path in ["a", "panel/index.html", "python/lib/util.py", "a b/c.txt"] {
            assert_eq!(inner_path(path).as_deref(), Ok(path));
        }
        for path in ["", "/a", "a/", "a//b", "./a", "a/./b", "..", "a/../b", "a\\b", "C:/a", "a:b", "a/b.", " a", "a /b", "a\0b", &"a/".repeat(101)] {
            assert!(inner_path(path).is_err(), "{path:?}");
        }
    }

    #[test]
    fn permissions_have_a_word_and_a_meaning() {
        for p in Permission::ALL {
            assert_eq!(Permission::parse(p.code()), Some(p));
            assert!(!p.meaning().is_empty());
            assert_eq!(serde_json::to_value(p).unwrap(), serde_json::json!(p.code()), "saved as the manifest's word");
        }
        assert_eq!(Permission::parse("Read"), None);
        assert!(Permission::Python.is_unlimited() && !Permission::Write.is_unlimited());
    }
}
