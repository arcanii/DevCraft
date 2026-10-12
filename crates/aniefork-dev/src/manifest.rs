//! A module's manifest, `module.toml`: who it is and what it asks to be allowed.
//!
//! ```toml
//! id = "clean-data"                  # the module's folder name
//! name = "Clean Data"
//! version = "0.1.0"
//! author = "Ada Lovelace"            # optional
//! description = "Tidies a table"     # optional
//! permissions = ["read", "write", "python"]
//! workbook = "Budget.xlsm"           # optional: the workbook this is the code of, by its file's name
//!
//! [panel]
//! entry = "panel/index.html"         # optional: the page GridCraft serves as its panel
//!
//! [python]
//! scripts = "python"                 # optional: the folder its panel may run scripts from
//! ```
//!
//! A manifest is written by a stranger: everything in it is checked, and anything not understood
//! where it matters (a permission, a path that leaves the module) makes the manifest invalid
//! rather than being guessed at.

use serde::{Deserialize, Serialize};

/// The file's name in a module's folder.
pub const FILE: &str = "module.toml";
/// The largest manifest read (bytes).
pub const MAX_BYTES: u64 = 64 * 1024;

/// Something a module asks to be allowed to do. The user grants them together, by trusting it.
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

    /// Granting it is the same as running a program from the module's author.
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
    /// The panel's page, as a path inside the module (`/` separators).
    pub panel: Option<String>,
    /// The folder of scripts its panel may run, as a path inside the module.
    pub python: Option<String>,
    /// The workbook it is the code of, by its file's name. `None`: it is a library, for any
    /// workbook. It only says what the module is for: nothing is allowed or run because of it.
    pub workbook: Option<String>,
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
    panel: Option<RawPanel>,
    python: Option<RawPython>,
}

#[derive(Deserialize)]
struct RawPanel {
    entry: Option<String>,
}

#[derive(Deserialize)]
struct RawPython {
    scripts: Option<String>,
}

/// A module's id: lowercase letters, digits and hyphens, starting with a letter or digit, at most
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

/// A path inside the module: relative, `/` separators, plain names only. Nothing that could name a
/// file outside the module's folder on any system.
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
    }
    Ok(path.to_string())
}

/// The longest name of a workbook a manifest may give (characters).
pub const MAX_WORKBOOK: usize = 255;

/// The name of the workbook a module is the code of: a file's name, not a path, since a path
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
        assert_eq!(m.workbook, None, "a module that names no workbook is a library");
        // Keys it doesn't know are ignored: a newer module still loads.
        assert!(parse("id = \"x\"\nname = \"X\"\nversion = \"1\"\nfuture = true\n[panel]\nwidth = 300", "x").is_ok());
    }

    #[test]
    fn refuses_what_it_does_not_understand_or_trust() {
        let with = |from: &str, to: &str| GOOD.replace(from, to);
        let bad = [
            ("the folder is another module's", GOOD.to_string(), "other"),
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
    fn a_module_can_name_the_workbook_it_is_the_code_of() {
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
