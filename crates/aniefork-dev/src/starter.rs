//! "New Module": a starter module written from templates compiled into the app, so that a
//! developer begins from something that loads, and so that no sample panel or script has to be
//! kept in the repository.
//!
//! The starter is an ordinary module: it is created untrusted, like any other.

use std::fs;
use std::path::Path;

use crate::manifest;
use crate::module::{self, Module};

/// A module id made from a name: "Clean Data!" becomes `clean-data`. Empty when the name has no
/// letters or digits to make one from.
pub fn slug(name: &str) -> String {
    let mut id = String::new();
    for c in name.trim().chars() {
        if c.is_ascii_alphanumeric() {
            id.push(c.to_ascii_lowercase());
        } else if !id.is_empty() && !id.ends_with('-') {
            id.push('-');
        }
        if id.len() >= 64 {
            break;
        }
    }
    id.trim_end_matches('-').to_string()
}

/// A name as it can sit inside a quoted TOML string or an HTML page.
fn plain(name: &str) -> String {
    name.trim().chars().filter(|c| !c.is_control() && !matches!(c, '"' | '\\' | '<' | '>' | '&')).take(80).collect::<String>().trim().to_string()
}

fn manifest_text(id: &str, name: &str) -> String {
    format!(
        r#"# A GridCraft module. See docs/decisions-aniefork.md (ADR-0001).
id = "{id}"
name = "{name}"
version = "0.1.0"
author = ""
description = "A starter module: a panel, and a Python script it can run."

# What this module asks to be allowed. The user sees this list before trusting it.
#   read     see what is in the open workbooks
#   write    change the open workbooks
#   files    open, save and export files
#   network  let the panel talk to the internet
#   python   run the scripts below (this cannot be limited)
permissions = ["read", "write", "python"]

[panel]
entry = "panel/index.html"

[python]
scripts = "python"
"#
    )
}

fn panel_text(name: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>{name}</title>
  <style>
    body {{ font: 14px system-ui, sans-serif; margin: 16px; color: #222; }}
    h1 {{ font-size: 18px; }}
  </style>
</head>
<body>
  <h1>{name}</h1>
  <p>This is the module's panel. GridCraft serves this folder to the browser once the module is trusted.</p>
  <p>From here the panel will call the spreadsheet, for example:</p>
  <pre>const result = await spreadsheet.runPython("clean.py", data);</pre>
</body>
</html>
"#
    )
}

const SCRIPT: &str = r#""""A script this module's panel can run:

    const result = await spreadsheet.runPython("clean.py", data);

GridCraft calls main() with the data the panel sent (whatever JSON can hold)
and sends back what it returns.
"""


def main(data):
    rows = data.get("rows", []) if isinstance(data, dict) else []
    cleaned = [[cell.strip() if isinstance(cell, str) else cell for cell in row] for row in rows]
    return {"rows": cleaned}
"#;

/// Creates a starter module named `name` in `modules_dir` and returns it as read back from disk.
/// Refuses a name that makes no id, and never touches a folder that already exists.
pub fn create(modules_dir: &Path, name: &str) -> Result<Module, String> {
    let shown = plain(name);
    let id = slug(&shown);
    if shown.is_empty() || !manifest::valid_id(&id) {
        return Err("Give the module a name with at least one letter or digit.".into());
    }
    let dir = modules_dir.join(&id);
    if dir.exists() {
        return Err(format!("There is already a module folder named {id}."));
    }
    let failed = |e: std::io::Error| format!("The module could not be created: {e}");
    for folder in ["panel", "python"] {
        fs::create_dir_all(dir.join(folder)).map_err(failed)?;
    }
    for (path, text) in
        [(manifest::FILE, manifest_text(&id, &shown)), ("panel/index.html", panel_text(&shown)), ("python/clean.py", SCRIPT.to_string())]
    {
        fs::write(dir.join(path), text).map_err(failed)?;
    }
    let created = module::load(&dir);
    match &created.problem {
        Some(problem) => Err(format!("The module was created but can't be read back: {problem}")),
        None => Ok(created),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Permission;
    use crate::module::tests::TempDir;

    #[test]
    fn names_become_ids() {
        for (name, id) in [
            ("Clean Data", "clean-data"),
            ("  Clean   Data!  ", "clean-data"),
            ("Q3 report (v2)", "q3-report-v2"),
            ("ÜBER tool", "ber-tool"),
            ("--x--", "x"),
            ("日本語", ""),
            ("", ""),
            ("../../etc", "etc"),
        ] {
            assert_eq!(slug(name), id, "{name:?}");
            assert!(id.is_empty() || manifest::valid_id(id));
        }
        assert!(slug(&"long name ".repeat(40)).len() <= 64);
    }

    #[test]
    fn a_starter_module_loads_and_is_an_ordinary_module() {
        let t = TempDir::new("starter");
        let m = create(&t.0, "Clean Data").unwrap();
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!((m.id.as_str(), m.name()), ("clean-data", "Clean Data"));
        assert_eq!(m.files, ["module.toml", "panel/index.html", "python/clean.py"]);
        let manifest = m.manifest.unwrap();
        assert_eq!(manifest.permissions, [Permission::Read, Permission::Write, Permission::Python]);
        assert_eq!((manifest.panel.as_deref(), manifest.python.as_deref()), (Some("panel/index.html"), Some("python")));
        assert_eq!(module::scan(&t.0).len(), 1);
    }

    #[test]
    fn it_never_overwrites_and_refuses_names_that_make_no_id() {
        let t = TempDir::new("refuse");
        create(&t.0, "Clean Data").unwrap();
        std::fs::write(t.0.join("clean-data/python/clean.py"), "mine").unwrap();
        assert!(create(&t.0, "clean data").is_err(), "the same id");
        assert_eq!(std::fs::read_to_string(t.0.join("clean-data/python/clean.py")).unwrap(), "mine", "the existing module is untouched");
        for name in ["", "   ", "!!!", "日本語", "\u{7}"] {
            assert!(create(&t.0, name).is_err(), "{name:?}");
        }
        assert_eq!(module::scan(&t.0).len(), 1, "nothing else was created");
    }

    #[test]
    fn a_hostile_name_cannot_break_out_of_the_files_it_is_written_into() {
        let t = TempDir::new("hostile");
        let m = create(&t.0, "Evil\" \npermissions = [\"python\"] <script>alert(1)</script> ..\\..\\x").unwrap();
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!(m.dir.parent(), Some(t.0.as_path()), "created inside the modules folder");
        let manifest = m.manifest.unwrap();
        assert_eq!(manifest.permissions, [Permission::Read, Permission::Write, Permission::Python], "the name added nothing to the manifest");
        let page = std::fs::read_to_string(m.dir.join("panel/index.html")).unwrap();
        assert!(!page.contains("<script>"), "nor markup to the panel");
    }
}
