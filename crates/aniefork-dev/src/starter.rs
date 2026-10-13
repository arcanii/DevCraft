//! "New Project": a starter project written from templates compiled into the app, so that a
//! developer begins from something that loads, and so that no sample panel or script has to be
//! kept in the repository.
//!
//! The starter is an ordinary project: it is created untrusted, like any other.
//!
//! Each kit has its own: PythonCraft's is a panel and a Python script it can run; the C kit's
//! is a program of a few lines and a panel that runs it.

use std::fs;
use std::path::Path;

use crate::kit::{self, Kit};
use crate::manifest;
use crate::project::{self, Project};

/// A project id made from a name: "Clean Data!" becomes `clean-data`. Empty when the name has no
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

/// The line of a manifest that names the workbook the project is the code of. The name is
/// written so that nothing in it can end the quotes it sits in.
fn workbook_line(workbook: Option<&str>) -> String {
    let Some(name) = workbook else { return String::new() };
    let quoted: String =
        name.chars().filter(|c| !c.is_control()).flat_map(|c| if matches!(c, '"' | '\\') { vec!['\\', c] } else { vec![c] }).collect();
    format!("\n# The workbook this is the code of, by its file's name. Without this line it is for any\n# workbook.\nworkbook = \"{quoted}\"\n")
}

fn manifest_text(id: &str, name: &str, workbook: Option<&str>) -> String {
    let workbook = workbook_line(workbook);
    format!(
        r#"# A GridCraft project. See docs/decisions-aniefork.md (ADR-0001).
id = "{id}"
name = "{name}"
version = "0.1.0"
author = ""
description = "A starter project: a panel, and a Python script it can run."
{workbook}
# What this project asks to be allowed. The user sees this list before trusting it.
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

/// The manifest of a C project. It asks for nothing: its panel runs the program and shows
/// what it prints, which needs no leave. Its recipe builds every C file in `src`.
fn c_manifest_text(id: &str, name: &str, workbook: Option<&str>) -> String {
    let workbook = workbook_line(workbook);
    let kit = kit::C.id;
    format!(
        r#"# A GridCraft project of the C Development Kit. See docs/decisions-aniefork.md (ADR-0001).
id = "{id}"
name = "{name}"
version = "0.1.0"
author = ""
description = "A starter project: a program written in C, and a panel that runs it."

# The development kit this is a project of. Without this line a project is a PythonCraft
# project (Python scripts and a web panel).
kit = "{kit}"
{workbook}
# What this project asks to be allowed. The user sees this list before trusting it.
#   read     see what is in the open workbooks
#   write    change the open workbooks
#   files    open, save and export files
# It asks for nothing so far: its panel runs the program and shows what it prints. The program
# itself can reach nothing: what it is to do with a workbook, its panel's page does for it,
# with what is asked for here.
permissions = []

# The page that runs the program GridCraft built. GridCraft serves two things beside it that
# are not files of this project: program.wasm (built from these files as you trust them now)
# and program.js (the loader that runs it).
[panel]
entry = "panel/index.html"

# What is built, as WebAssembly, by the compiler you name in GridCraft (zig). A list, never a
# command: GridCraft writes the compiler's command line itself.
#   sources   C files, or folders (every .c file directly in a folder)
#   include   folders searched for #include "..."          (optional)
#   define    names, or NAME=value                          (optional)
#   optimize  size | speed | none                           (optional: size)
#   entry     main, or none when its page calls what it exports (optional: main)
[c]
sources = ["src"]
"#
    )
}

/// The page a C project starts from: it loads the program GridCraft built, runs it, and shows
/// what it printed. The name is written where a page holds text (the title and the heading);
/// only the id, which is plain, is written into the script.
fn c_panel_text(id: &str, name: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>{name}</title>
  <style>
    body {{ font: 14px system-ui, sans-serif; margin: 20px; color: #222; max-width: 40em; }}
    h1 {{ font-size: 18px; }}
    button {{ font: inherit; padding: 6px 12px; }}
    #out {{ font: 13px ui-monospace, monospace; background: #f4f4f4; padding: 10px; min-height: 3em; white-space: pre-wrap; }}
  </style>
</head>
<body>
  <h1>{name}</h1>
  <p>This panel runs the program GridCraft built from this project's C. What it prints:</p>
  <div id="out"></div>
  <p><button id="run">Run It Again</button></p>

  <!-- GridCraft serves these two beside this page: neither is a file of the project.
       program.js gives the page CProgram, which loads program.wasm (the program built from
       this project's files as you trust them now) and runs it. The program can reach nothing
       but what this page hands it: a clock, a place to print, and files kept in memory.
       spreadsheet.js gives the page its spreadsheet object, for what the project was granted. -->
  <script src="spreadsheet.js"></script>
  <script src="program.js"></script>
  <script>
    const out = document.getElementById("out");
    const say = (line) => {{
      const row = document.createElement("div");
      row.textContent = line;
      out.append(row);
    }};

    async function run() {{
      out.replaceChildren();
      try {{
        // Each line the program prints is handed to print.
        const program = await CProgram.load({{ args: ["{id}"], print: say }});
        // main(), to its end. A program built with entry = "none" has no main: its page calls
        // what it exports (program.exports.step()), and looks into its memory with
        // program.string(pointer) and program.bytes(pointer, length).
        const code = program.run();
        if (code !== 0) say("(it ended with " + code + ")");
        // A line for Developer > Console in GridCraft.
        spreadsheet.log("{id} ran and ended with", code);
        // What the program is to do with the workbook, this page does for it. Ask for "write"
        // in project.toml, and then, for one:
        //   await spreadsheet.write("A1", [[out.textContent]]);
      }} catch (error) {{
        // A project that was not built from the files it holds now has no program: it says so.
        say(String(error.message || error));
      }}
    }}

    document.getElementById("run").onclick = run;
    run();
  </script>
</body>
</html>
"#
    )
}

/// The program a C project starts from. Only the project's id is written into it: an id is
/// lowercase letters, digits and hyphens, which can end neither a comment nor a string.
fn c_source_text(id: &str) -> String {
    format!(
        r#"/*
 * {id}: a project of GridCraft's C Development Kit.
 *
 * GridCraft builds this project as WebAssembly once you trust it (Developer, Development,
 * the project, Build), and the project's panel (panel/index.html) runs what was built:
 * what this prints shows there. After a change, trust it as it is and build it again.
 */
#include <stdio.h>

int main(void)
{{
    puts("Hello from {id}.");
    return 0;
}}
"#
    )
}

const SCRIPT: &str = r#""""A script this project's panel can run:

    const result = await spreadsheet.runPython("clean.py", data);

GridCraft calls main() with the data the panel sent (whatever JSON can hold)
and sends back what it returns.

A script can also work on the sheet itself, with what the project was granted:

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

/// The id a starter project named `name` is given: empty when the name makes none.
pub fn id_for(name: &str) -> String {
    slug(&plain(name))
}

/// What a starter project named `name` holds: its id, and its files (path and text). `None` when
/// the name makes no id.
pub fn files(name: &str) -> Option<(String, Vec<(&'static str, String)>)> {
    files_for(&kit::PYTHONCRAFT, name, None)
}

/// The same, for a project of `kit` that is the code of the workbook named `workbook`.
fn files_for(kit: &Kit, name: &str, workbook: Option<&str>) -> Option<(String, Vec<(&'static str, String)>)> {
    let shown = plain(name);
    let id = slug(&shown);
    if shown.is_empty() || !manifest::valid_id(&id) {
        return None;
    }
    let files = if kit.id == kit::C.id {
        vec![
            (manifest::FILE, c_manifest_text(&id, &shown, workbook)),
            ("src/main.c", c_source_text(&id)),
            ("panel/index.html", c_panel_text(&id, &shown)),
        ]
    } else {
        vec![
            (manifest::FILE, manifest_text(&id, &shown, workbook)),
            ("panel/index.html", panel_text(&shown)),
            ("python/clean.py", SCRIPT.to_string()),
        ]
    };
    Some((id, files))
}

/// Creates a starter project named `name` in `projects_dir` and returns it as read back from disk.
/// Refuses a name that makes no id, and never touches a folder that already exists.
pub fn create(projects_dir: &Path, name: &str) -> Result<Project, String> {
    create_for(projects_dir, name, None)
}

/// The same, for a project that is the code of the workbook named `workbook` (for any workbook
/// when there is none). A name that can't be a workbook's is refused before anything is written.
pub fn create_for(projects_dir: &Path, name: &str, workbook: Option<&str>) -> Result<Project, String> {
    create_in(projects_dir, &kit::PYTHONCRAFT, name, workbook)
}

/// Puts lines into a manifest right after the line that gives its id, where a manifest's own
/// words stand before any of its tables. A line it already has for the same word is dropped.
#[cfg(not(target_arch = "wasm32"))]
fn after_id(text: &str, lines: &[String]) -> String {
    let words: Vec<&str> = lines
        .iter()
        .filter_map(|line| line.trim_start_matches('\n').lines().find(|l| !l.starts_with('#') && !l.is_empty())?.split('=').next())
        .map(str::trim)
        .collect();
    let mut out = Vec::new();
    for line in text.lines() {
        if words.iter().any(|word| line.split('=').next().is_some_and(|said| said.trim() == *word) && line.contains('=')) {
            continue;
        }
        out.push(line.to_string());
        if line.trim_start().starts_with("id") && line.split('=').next().is_some_and(|said| said.trim() == "id") {
            out.extend(lines.iter().map(|line| line.trim_matches('\n').to_string()));
        }
    }
    out.join("\n") + "\n"
}

/// The files a new project of a kit from a package begins with: the kit's own starter, with
/// the project's id and name written in where it says `{{id}}` and `{{name}}`; or, for a kit
/// that brought none, the starter of the kit it stands on, naming this kit.
#[cfg(not(target_arch = "wasm32"))]
fn packaged_files(projects_dir: &Path, kit: &Kit, name: &str, workbook: Option<&str>) -> Option<(String, Vec<(String, Vec<u8>)>)> {
    let shown = plain(name);
    let id = slug(&shown);
    if shown.is_empty() || !manifest::valid_id(&id) {
        return None;
    }
    let named = vec![format!("kit = \"{}\"", kit.id)];
    let own = crate::kitpack::starter_files(projects_dir, kit);
    let mut files: Vec<(String, Vec<u8>)> = if own.is_empty() {
        let base = kit::named(kit.base).unwrap_or(&kit::PYTHONCRAFT);
        files_for(base, name, None)?.1.into_iter().map(|(path, text)| (path.to_string(), text.into_bytes())).collect()
    } else {
        // Text is written with the project's own id and name; anything else is copied as it is.
        own.into_iter()
            .map(|(path, held)| match String::from_utf8(held) {
                Ok(text) => (path, text.replace("{{id}}", &id).replace("{{name}}", &shown).into_bytes()),
                Err(not_text) => (path, not_text.into_bytes()),
            })
            .collect()
    };
    // Its manifest names this kit, and the workbook it is the code of when it is one's.
    for (path, held) in &mut files {
        if path == manifest::FILE
            && let Ok(text) = std::str::from_utf8(held)
        {
            let mut lines = named.clone();
            let workbook = workbook_line(workbook);
            if !workbook.is_empty() {
                lines.push(workbook.trim_matches('\n').lines().last().unwrap_or("").to_string());
            }
            *held = after_id(text, &lines).into_bytes();
        }
    }
    Some((id, files))
}

/// The same, for a project of `kit`: each kit has a starter of its own.
pub fn create_in(projects_dir: &Path, kit: &Kit, name: &str, workbook: Option<&str>) -> Result<Project, String> {
    let workbook = match workbook {
        Some(named) => manifest::workbook_name(named).map_err(|why| format!("The project can't be that workbook's code: {why}."))?,
        None => None,
    };
    #[cfg(not(target_arch = "wasm32"))]
    let made = if kit.built_in {
        files_for(kit, name, workbook.as_deref())
            .map(|(id, files)| (id, files.into_iter().map(|(path, text)| (path.to_string(), text.into_bytes())).collect::<Vec<_>>()))
    } else {
        packaged_files(projects_dir, kit, name, workbook.as_deref())
    };
    #[cfg(target_arch = "wasm32")]
    let made = files_for(kit, name, workbook.as_deref())
        .map(|(id, files)| (id, files.into_iter().map(|(path, text)| (path.to_string(), text.into_bytes())).collect::<Vec<_>>()));
    let Some((id, files)) = made else {
        return Err("Give the project a name with at least one letter or digit.".into());
    };
    let dir = projects_dir.join(&id);
    if dir.exists() {
        return Err(format!("There is already a project folder named {id}."));
    }
    let failed = |e: std::io::Error| format!("The project could not be created: {e}");
    fs::create_dir_all(&dir).map_err(failed)?;
    for (path, held) in files {
        if let Some(parent) = dir.join(&path).parent() {
            fs::create_dir_all(parent).map_err(failed)?;
        }
        fs::write(dir.join(&path), held).map_err(failed)?;
    }
    let created = project::load(&dir);
    match &created.problem {
        Some(problem) => {
            // What a kit's starter made has to be a project. One that is not is taken away
            // again: it is only what this call wrote.
            if !kit.built_in {
                let _ = fs::remove_dir_all(&dir);
            }
            Err(format!("The project was created but can't be read back: {problem}"))
        }
        None => Ok(created),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Permission;
    use crate::project::tests::TempDir;

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
        assert_eq!((id.as_str(), id_for("Clean \"Data\"").as_str()), ("clean-data", "clean-data"), "the id is the one the project is created with");
        let made = create(&t.0, "Clean \"Data\"").unwrap();
        assert_eq!(made.id, id);
        // A project's files are listed in order, whatever order they were written in.
        let mut named: Vec<String> = listed.iter().map(|(path, _)| path.to_string()).collect();
        named.sort();
        assert_eq!(made.files, named);
        for (path, text) in listed {
            assert_eq!(std::fs::read_to_string(made.dir.join(path)).unwrap(), text, "{path}");
        }
        assert!(files("!!!").is_none() && id_for("!!!").is_empty());
    }

    #[test]
    fn a_starter_project_loads_and_is_an_ordinary_project() {
        let t = TempDir::new("starter");
        let m = create(&t.0, "Clean Data").unwrap();
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!((m.id.as_str(), m.name()), ("clean-data", "Clean Data"));
        assert_eq!(m.files, ["panel/index.html", "project.toml", "python/clean.py"]);
        let manifest = m.manifest.unwrap();
        assert_eq!(manifest.permissions, [Permission::Read, Permission::Write, Permission::Python]);
        assert_eq!((manifest.panel.as_deref(), manifest.python.as_deref()), (Some("panel/index.html"), Some("python")));
        assert_eq!(project::scan(&t.0).len(), 1);
    }

    #[test]
    fn a_starter_can_be_the_code_of_a_workbook() {
        let t = TempDir::new("starter-workbook");
        let m = create_for(&t.0, "Budget Macros", Some("Budget.xlsm")).unwrap();
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!(m.manifest.as_ref().unwrap().workbook.as_deref(), Some("Budget.xlsm"));
        // Without a workbook, and with an empty one, it is for any workbook like any starter.
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
    fn the_c_kit_has_a_starter_of_its_own() {
        let t = TempDir::new("starter-c");
        let m = create_in(&t.0, &kit::C, "Blink", None).unwrap();
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!(m.files, ["panel/index.html", "project.toml", "src/main.c"], "a manifest, a program and the panel that runs it: no scripts");
        let manifest = m.manifest.clone().unwrap();
        assert_eq!((manifest.kit.as_str(), manifest.name.as_str()), ("c", "Blink"));
        assert_eq!(kit::of(Some(&manifest)), &kit::C);
        assert!(manifest.permissions.is_empty(), "it asks for nothing: its panel shows what the program prints, and no more");
        // Its recipe builds what is in src: the one file it starts with.
        let recipe = manifest.c.clone().unwrap();
        assert_eq!(
            (recipe.sources.as_slice(), recipe.sources_among(&m.files).unwrap()),
            (["src".to_string()].as_slice(), vec!["src/main.c".to_string()])
        );
        assert_eq!((recipe.optimize, recipe.entry), (crate::c::Optimize::Size, crate::c::Entry::Main));
        assert_eq!((manifest.panel.as_deref(), manifest.python, manifest.workbook), (Some("panel/index.html"), None, None));
        let source = std::fs::read_to_string(m.dir.join("src/main.c")).unwrap();
        assert!(source.contains("puts(\"Hello from blink.\");") && source.contains("int main(void)"));
        // Its panel loads the program GridCraft built, with the loader GridCraft serves, runs
        // it and shows what it printed. Neither of the two is a file of the project.
        let page = std::fs::read_to_string(m.dir.join("panel/index.html")).unwrap();
        for part in [
            "<title>Blink</title>",
            "<script src=\"program.js\"></script>",
            "<script src=\"spreadsheet.js\"></script>",
            "CProgram.load({ args: [\"blink\"], print: say })",
            "program.run()",
        ] {
            assert!(page.contains(part), "{part}");
        }
        assert!(!m.files.iter().any(|file| file.ends_with("program.js") || file.ends_with("program.wasm")));
        // It can be the code of a workbook, like any project.
        let doom = create_in(&t.0, &kit::C, "Doom", Some("Doom.xlsx")).unwrap();
        assert_eq!(doom.manifest.unwrap().workbook.as_deref(), Some("Doom.xlsx"));
        // The first kit's starter is what it was.
        let first = create_in(&t.0, &kit::PYTHONCRAFT, "Clean Data", None).unwrap();
        assert_eq!(first.files, ["panel/index.html", "project.toml", "python/clean.py"]);
        assert_eq!(first.manifest.unwrap().kit, "pythoncraft");
        // A name can't write itself into the program: only the id, which is plain, is put there.
        let odd = create_in(&t.0, &kit::C, "Evil */ int system(const char *); /* \" \\ %s", None).unwrap();
        assert!(odd.is_sound(), "{:?}", odd.problem);
        assert_eq!(odd.id, "evil-int-system-const-char-s");
        let source = std::fs::read_to_string(odd.dir.join("src/main.c")).unwrap();
        assert_eq!(source.matches("*/").count(), 1, "the one comment ends once");
        assert!(!source.contains("system") || source.contains("evil-int-system-const-char-s"));
        assert_eq!(source.matches('"').count(), 2, "and the one string is the starter's own");
        assert!(odd.manifest.unwrap().permissions.is_empty());
        // Nor into the page's script: the name is in the title and the heading, as text, and
        // only the id is in the script.
        let shown = plain("Evil */ int system(const char *); /* \" \\ %s");
        let page = std::fs::read_to_string(odd.dir.join("panel/index.html")).unwrap();
        assert_eq!(page.matches(&shown).count(), 2, "{shown:?}");
        assert_eq!(page.matches("<script").count(), 3, "the page's scripts are the starter's own three");
        assert!(!page.contains('\\') && page.contains("args: [\"evil-int-system-const-char-s\"]"));
        let marked = create_in(&t.0, &kit::C, "Mark <script>alert(1)</script> & \"co\"", None).unwrap();
        let page = std::fs::read_to_string(marked.dir.join("panel/index.html")).unwrap();
        assert!(marked.is_sound() && !page.contains("<script>alert") && page.matches("<script").count() == 3, "{:?}", marked.problem);
        // The same names are refused, and nothing is written over.
        assert!(create_in(&t.0, &kit::C, "blink", None).unwrap_err().contains("already a project folder named blink"));
        assert!(create_in(&t.0, &kit::C, "!!!", None).is_err());
    }

    #[test]
    fn it_never_overwrites_and_refuses_names_that_make_no_id() {
        let t = TempDir::new("refuse");
        create(&t.0, "Clean Data").unwrap();
        std::fs::write(t.0.join("clean-data/python/clean.py"), "mine").unwrap();
        assert!(create(&t.0, "clean data").is_err(), "the same id");
        assert_eq!(std::fs::read_to_string(t.0.join("clean-data/python/clean.py")).unwrap(), "mine", "the existing project is untouched");
        for name in ["", "   ", "!!!", "日本語", "\u{7}"] {
            assert!(create(&t.0, name).is_err(), "{name:?}");
        }
        assert_eq!(project::scan(&t.0).len(), 1, "nothing else was created");
    }

    #[test]
    fn a_hostile_name_cannot_break_out_of_the_files_it_is_written_into() {
        let t = TempDir::new("hostile");
        let m = create(&t.0, "Evil\" \npermissions = [\"python\"] <script>alert(1)</script> ..\\..\\x").unwrap();
        assert!(m.is_sound(), "{:?}", m.problem);
        assert_eq!(m.dir.parent(), Some(t.0.as_path()), "created inside the projects folder");
        let manifest = m.manifest.unwrap();
        assert_eq!(manifest.permissions, [Permission::Read, Permission::Write, Permission::Python], "the name added nothing to the manifest");
        let page = std::fs::read_to_string(m.dir.join("panel/index.html")).unwrap();
        assert!(!page.contains("<script>alert") && !page.contains("</script> ..") && !page.contains('\\'), "nor markup to the panel");
        assert_eq!(page.matches("<script").count(), 2, "the panel's scripts are the starter's own two");
    }
}
