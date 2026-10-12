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

/// The line of a manifest that names the workbook the module is the code of. The name is
/// written so that nothing in it can end the quotes it sits in.
fn workbook_line(workbook: Option<&str>) -> String {
    let Some(name) = workbook else { return String::new() };
    let quoted: String =
        name.chars().filter(|c| !c.is_control()).flat_map(|c| if matches!(c, '"' | '\\') { vec!['\\', c] } else { vec![c] }).collect();
    format!(
        "\n# The workbook this is the code of, by its file's name. Without this line it is a library,\n# for any workbook.\nworkbook = \"{quoted}\"\n"
    )
}

fn manifest_text(id: &str, name: &str, workbook: Option<&str>) -> String {
    let workbook = workbook_line(workbook);
    format!(
        r#"# A GridCraft module. See docs/decisions-aniefork.md (ADR-0001).
id = "{id}"
name = "{name}"
version = "0.1.0"
author = ""
description = "A starter module: a panel, and a Python script it can run."
{workbook}
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
    body {{ font: 14px system-ui, sans-serif; margin: 20px; color: #222; max-width: 40em; }}
    h1 {{ font-size: 18px; }}
    button {{ font: inherit; padding: 6px 12px; margin-right: 6px; }}
    pre {{ background: #f4f4f4; padding: 10px; white-space: pre-wrap; min-height: 3em; }}
  </style>
</head>
<body>
  <h1>{name}</h1>
  <p>Select some cells in GridCraft, then:</p>
  <p>
    <button id="clean">Clean the selection</button>
    <button id="show">Show the selection</button>
  </p>
  <p id="where">The selection shows here as it moves.</p>
  <pre id="out"></pre>

  <!-- GridCraft serves spreadsheet.js beside this page: it gives the page its `spreadsheet`
       object. The types of that object, for TypeScript, are served beside it as
       spreadsheet.d.ts. -->
  <script src="spreadsheet.js"></script>
  <script>
    const out = document.getElementById("out");
    const say = (value) => {{
      out.textContent = typeof value === "string" ? value : JSON.stringify(value, null, 2);
    }};

    document.getElementById("show").onclick = async () => {{
      try {{
        say(await spreadsheet.selection());
      }} catch (error) {{
        say(String(error));
      }}
    }};

    document.getElementById("clean").onclick = async () => {{
      try {{
        const data = await spreadsheet.selection();
        const result = await spreadsheet.runPython("clean.py", data);
        // One call is one undo step. Several commands that should be undone together go in
        // spreadsheet.batch([...]). Every command a user can give is spreadsheet.run(id, params).
        await spreadsheet.write(data.range, result.rows);
        say("Cleaned " + data.range + ". Undo in GridCraft puts it back.");
        // A line for Developer > Console in GridCraft. What clean.py prints goes there too.
        spreadsheet.log("Cleaned", data.range);
      }} catch (error) {{
        say(String(error));
      }}
    }};

    // The panel is told when the selection moves. The other events are workbookChanged and
    // workbookActivated; the function this resolves to stops the telling.
    spreadsheet
      .on("selectionChanged", (event) => {{
        document.getElementById("where").textContent = "Selected: " + event.sheet + "!" + event.range;
      }})
      .catch((error) => say(String(error)));
  </script>
</body>
</html>
"#
    )
}

const SCRIPT: &str = r#""""A script this module's panel can run:

    const result = await spreadsheet.runPython("clean.py", data);

GridCraft calls main() with the data the panel sent (whatever JSON can hold)
and sends back what it returns.

A script can also work on the sheet itself, with what the module was granted:

    import spreadsheet
    spreadsheet.write(data["range"], cleaned)          # or spreadsheet.run(id, params)
    spreadsheet.sheet("Data").range("A1:C3").read()    # the workbook as objects
"""


def main(data):
    rows = data.get("rows", []) if isinstance(data, dict) else []
    cleaned = [[cell.strip() if isinstance(cell, str) else cell for cell in row] for row in rows]
    # What a script prints shows in Developer > Console in GridCraft.
    print(f"cleaned {len(cleaned)} rows")
    return {"rows": cleaned}
"#;

/// The id a starter module named `name` is given: empty when the name makes none.
pub fn id_for(name: &str) -> String {
    slug(&plain(name))
}

/// What a starter module named `name` holds: its id, and its files (path and text). `None` when
/// the name makes no id.
pub fn files(name: &str) -> Option<(String, Vec<(&'static str, String)>)> {
    files_for(name, None)
}

/// The same, for a module that is the code of the workbook named `workbook`.
fn files_for(name: &str, workbook: Option<&str>) -> Option<(String, Vec<(&'static str, String)>)> {
    let shown = plain(name);
    let id = slug(&shown);
    if shown.is_empty() || !manifest::valid_id(&id) {
        return None;
    }
    let files = vec![
        (manifest::FILE, manifest_text(&id, &shown, workbook)),
        ("panel/index.html", panel_text(&shown)),
        ("python/clean.py", SCRIPT.to_string()),
    ];
    Some((id, files))
}

/// Creates a starter module named `name` in `modules_dir` and returns it as read back from disk.
/// Refuses a name that makes no id, and never touches a folder that already exists.
pub fn create(modules_dir: &Path, name: &str) -> Result<Module, String> {
    create_for(modules_dir, name, None)
}

/// The same, for a module that is the code of the workbook named `workbook` (a library when
/// there is none). A name that can't be a workbook's is refused before anything is written.
pub fn create_for(modules_dir: &Path, name: &str, workbook: Option<&str>) -> Result<Module, String> {
    let workbook = match workbook {
        Some(named) => manifest::workbook_name(named).map_err(|why| format!("The module can't be that workbook's code: {why}."))?,
        None => None,
    };
    let Some((id, files)) = files_for(name, workbook.as_deref()) else {
        return Err("Give the module a name with at least one letter or digit.".into());
    };
    let dir = modules_dir.join(&id);
    if dir.exists() {
        return Err(format!("There is already a module folder named {id}."));
    }
    let failed = |e: std::io::Error| format!("The module could not be created: {e}");
    for folder in ["panel", "python"] {
        fs::create_dir_all(dir.join(folder)).map_err(failed)?;
    }
    for (path, text) in files {
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
    fn what_a_starter_holds_is_known_before_it_is_written() {
        let t = TempDir::new("starter-files");
        let (id, listed) = files("Clean \"Data\"").unwrap();
        assert_eq!((id.as_str(), id_for("Clean \"Data\"").as_str()), ("clean-data", "clean-data"), "the id is the one the module is created with");
        let made = create(&t.0, "Clean \"Data\"").unwrap();
        assert_eq!(made.id, id);
        assert_eq!(made.files, listed.iter().map(|(path, _)| path.to_string()).collect::<Vec<_>>());
        for (path, text) in listed {
            assert_eq!(std::fs::read_to_string(made.dir.join(path)).unwrap(), text, "{path}");
        }
        assert!(files("!!!").is_none() && id_for("!!!").is_empty());
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
    fn a_starter_can_be_the_code_of_a_workbook() {
        let t = TempDir::new("starter-workbook");
        let m = create_for(&t.0, "Budget Macros", Some("Budget.xlsm")).unwrap();
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!(m.manifest.as_ref().unwrap().workbook.as_deref(), Some("Budget.xlsm"));
        // Without a workbook, and with an empty one, it is a library like any starter.
        assert_eq!(create_for(&t.0, "Tools", None).unwrap().manifest.unwrap().workbook, None);
        assert_eq!(create_for(&t.0, "More Tools", Some("  ")).unwrap().manifest.unwrap().workbook, None);
        // A name with quotes in it stays a name: it adds nothing to the manifest.
        let odd = create_for(&t.0, "Odd", Some("My \"best\" book.xlsm\"\npermissions = [\"network\"] #")).unwrap_err();
        assert!(odd.contains("not a file's name"), "a line break is refused: {odd}");
        let quoted = create_for(&t.0, "Quoted", Some("My \"best\" book.xlsm")).unwrap();
        let manifest = quoted.manifest.unwrap();
        assert_eq!(manifest.workbook.as_deref(), Some("My \"best\" book.xlsm"));
        assert_eq!(manifest.permissions, [Permission::Read, Permission::Write, Permission::Python]);
        // What can't be a workbook's name is refused before anything is written.
        assert!(create_for(&t.0, "Elsewhere", Some("C:/books/Budget.xlsm")).unwrap_err().contains("can't be that workbook's code"));
        assert!(!t.0.join("elsewhere").exists());
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
        assert!(!page.contains("<script>alert") && !page.contains("</script> ..") && !page.contains('\\'), "nor markup to the panel");
        assert_eq!(page.matches("<script").count(), 2, "the panel's scripts are the starter's own two");
    }
}
