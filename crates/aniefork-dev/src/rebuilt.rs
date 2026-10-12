//! What a project says it rebuilds of a workbook's macros: `rebuilt.toml`, beside its
//! `module.toml`.
//!
//! GridCraft does not run VBA. What a workbook's macros did is rebuilt as a project (ADR-0001,
//! "Macros"), by a person or by an agent helping them. This file is the project's note of
//! what became of each procedure, so that the person can see, beside the macros, what is
//! done, what is done in part and what was left:
//!
//! ```toml
//! [[procedure]]
//! module = "Totals"            # the VBA module
//! name = "AddUp"               # the procedure in it
//! state = "rebuilt"            # rebuilt | partly | left
//! script = "add_up.py"         # the script that does it now, in the project's scripts folder
//! note = "Sums A1:A10 and writes the total below."
//! ```
//!
//! **It is the project's own word.** Whoever wrote the project wrote it, and nothing checks
//! that a script does what a procedure did. So it is shown as what the project says, it is
//! data to an agent, and nothing is allowed or run because of it. Like a manifest it is
//! written by a stranger: everything in it is checked, and a note that can't be understood is
//! not guessed at.

use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde_json::{Value as Json, json};

use crate::manifest;
use crate::module::Module;

/// The file's name in a project's folder.
pub const FILE: &str = "rebuilt.toml";
/// The largest note read (bytes).
pub const MAX_BYTES: u64 = 256 * 1024;
/// The most procedures a note may speak of.
pub const MAX_ENTRIES: usize = 2_000;
/// The longest name of a VBA module or procedure (characters). The language allows 255.
const MAX_NAME: usize = 255;
/// The longest note on one procedure (characters).
const MAX_NOTE: usize = 500;

/// What became of a procedure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// A script does what it did.
    Rebuilt,
    /// A script does part of it: the note says what is missing.
    Partly,
    /// It was not rebuilt, on purpose: the note says why.
    Left,
}

impl State {
    pub const ALL: [State; 3] = [State::Rebuilt, State::Partly, State::Left];

    /// The word the file uses.
    pub fn code(self) -> &'static str {
        match self {
            State::Rebuilt => "rebuilt",
            State::Partly => "partly",
            State::Left => "left",
        }
    }

    pub fn parse(code: &str) -> Option<State> {
        State::ALL.into_iter().find(|state| state.code() == code)
    }

    /// The words people are shown.
    pub fn label(self) -> &'static str {
        match self {
            State::Rebuilt => "Rebuilt",
            State::Partly => "Rebuilt in part",
            State::Left => "Left as it is",
        }
    }
}

/// What a note says of one procedure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The VBA module.
    pub module: String,
    /// The procedure in it.
    pub procedure: String,
    pub state: State,
    /// The script that does it now, as a path inside the project's scripts folder.
    pub script: Option<String>,
    pub note: String,
}

/// A checked note.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Note {
    pub entries: Vec<Entry>,
}

impl Note {
    /// How many procedures it says are in `state`.
    pub fn count(&self, state: State) -> usize {
        self.entries.iter().filter(|entry| entry.state == state).count()
    }
}

#[derive(Deserialize)]
struct Raw {
    procedure: Option<Vec<RawEntry>>,
}

#[derive(Deserialize)]
struct RawEntry {
    module: Option<String>,
    name: Option<String>,
    state: Option<String>,
    script: Option<String>,
    note: Option<String>,
}

/// A name or a note: one line of text for people, within `max` characters.
fn line(value: Option<String>, what: &str, max: usize, required: bool) -> Result<String, String> {
    let value = value.unwrap_or_default();
    let value = value.trim();
    if value.is_empty() && required {
        return Err(format!("has no {what}"));
    }
    if value.chars().count() > max {
        return Err(format!("has a {what} longer than {max} characters"));
    }
    if value.chars().any(char::is_control) {
        return Err(format!("has a {what} that contains control characters"));
    }
    Ok(value.to_string())
}

/// Reads and checks a note.
pub fn parse(source: &str) -> Result<Note, String> {
    if source.len() as u64 > MAX_BYTES {
        return Err(format!("{FILE} is larger than {} KB", MAX_BYTES / 1024));
    }
    let raw: Raw = toml::from_str(source).map_err(|e| format!("{FILE} can't be read: {}", e.message()))?;
    let listed = raw.procedure.unwrap_or_default();
    if listed.len() > MAX_ENTRIES {
        return Err(format!("{FILE} speaks of more than {MAX_ENTRIES} procedures"));
    }
    let mut entries = Vec::with_capacity(listed.len());
    for (index, entry) in listed.into_iter().enumerate() {
        let wrong = |why: String| format!("{FILE}: procedure {} {why}", index + 1);
        let state = entry.state.unwrap_or_default();
        let state = State::parse(&state).ok_or_else(|| wrong(format!("has the state {state:?}, which is not rebuilt, partly or left")))?;
        let script = match entry.script.as_deref().map(str::trim).filter(|script| !script.is_empty()) {
            Some(script) => Some(manifest::inner_path(script).map_err(|why| wrong(format!("names a script that will not do: {why}")))?),
            None => None,
        };
        entries.push(Entry {
            module: line(entry.module, "module", MAX_NAME, true).map_err(wrong)?,
            procedure: line(entry.name, "name", MAX_NAME, true).map_err(wrong)?,
            state,
            script,
            note: line(entry.note, "note", MAX_NOTE, false).map_err(wrong)?,
        });
    }
    Ok(Note { entries })
}

/// The note of the project in `dir`. `None` when it has none; an error when it has one that
/// can't be used.
pub fn read(dir: &Path) -> Option<Result<Note, String>> {
    let file = dir.join(FILE);
    // A link is never followed out of the project: only a plain file is read.
    let found = fs::symlink_metadata(&file).ok()?;
    if !found.is_file() {
        return Some(Err(format!("{FILE} is not a plain file")));
    }
    if found.len() > MAX_BYTES {
        return Some(Err(format!("{FILE} is larger than {} KB", MAX_BYTES / 1024)));
    }
    Some(fs::read_to_string(&file).map_err(|e| format!("{FILE} can't be read: {e}")).and_then(|text| parse(&text)))
}

/// Whether the script a note names is among the project's files.
fn has_script(project: &Module, script: &str) -> bool {
    let Some(folder) = project.manifest.as_ref().and_then(|manifest| manifest.python.as_deref()) else { return false };
    let path = format!("{folder}/{script}");
    project.files.contains(&path)
}

/// A project's note, as it is reported with the project: what it says of each procedure, how
/// many of each state, and whether a script it names is missing. `Null` when it has none.
pub fn describe(project: &Module) -> Json {
    match read(&project.dir) {
        None => Json::Null,
        Some(Err(problem)) => json!({"problem": problem, "procedures": [], "rebuilt": 0, "partly": 0, "left": 0}),
        Some(Ok(note)) => {
            let procedures: Vec<Json> = note
                .entries
                .iter()
                .map(|entry| {
                    json!({
                        "module": entry.module,
                        "name": entry.procedure,
                        "state": entry.state.code(),
                        "script": entry.script,
                        "scriptMissing": entry.script.as_deref().is_some_and(|script| !has_script(project, script)),
                        "note": entry.note,
                    })
                })
                .collect();
            json!({
                "problem": null,
                "procedures": procedures,
                "rebuilt": note.count(State::Rebuilt),
                "partly": note.count(State::Partly),
                "left": note.count(State::Left),
            })
        }
    }
}

/// What one project says became of one procedure of a workbook's macros.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claim {
    /// The VBA module and the procedure, as the note names them.
    pub module: String,
    pub procedure: String,
    /// The project that says so: its id, and its name.
    pub project: String,
    pub project_name: String,
    pub state: State,
    pub script: Option<String>,
    /// The script it names is not among the project's files.
    pub script_missing: bool,
    pub note: String,
}

impl Claim {
    /// Whether it speaks of this procedure of this module. VBA's names know no capitals.
    pub fn is_of(&self, module: &str, procedure: &str) -> bool {
        self.module.to_lowercase() == module.to_lowercase() && self.procedure.to_lowercase() == procedure.to_lowercase()
    }

    pub fn json(&self) -> Json {
        json!({
            "project": self.project,
            "projectName": self.project_name,
            "state": self.state.code(),
            "script": self.script,
            "scriptMissing": self.script_missing,
            "note": self.note,
        })
    }
}

/// What the projects that are the code of the workbook named `workbook` say of its macros:
/// every entry of every such project's note. Projects that can't be read, and notes that
/// can't, say nothing.
pub fn claims(projects: &[Module], workbook: &str) -> Vec<Claim> {
    let mut all = Vec::new();
    for project in projects {
        let Some(manifest) = &project.manifest else { continue };
        if !manifest.workbook.as_deref().is_some_and(|named| manifest::same_workbook(named, workbook)) {
            continue;
        }
        let Some(Ok(note)) = read(&project.dir) else { continue };
        for entry in note.entries {
            let script_missing = entry.script.as_deref().is_some_and(|script| !has_script(project, script));
            all.push(Claim {
                module: entry.module,
                procedure: entry.procedure,
                project: project.id.clone(),
                project_name: manifest.name.clone(),
                state: entry.state,
                script: entry.script,
                script_missing,
                note: entry.note,
            });
        }
    }
    all
}

/// Writes, beside each procedure of a VBA project's summary ([`crate::vba::summary`]), what
/// projects say became of it (`rebuilt`: a list, empty when none speaks of it). Returns how
/// many procedures there are, and of how many a project says it is rebuilt, whole or in part.
/// A `Declare` is not counted: it is a door to a library, with nothing in it to rebuild.
pub fn mark(summary: &mut Json, claims: &[Claim]) -> (usize, usize) {
    let (mut procedures, mut spoken_for) = (0, 0);
    let Some(modules) = summary.get_mut("modules").and_then(Json::as_array_mut) else { return (0, 0) };
    for module in modules {
        let name = module.get("name").and_then(Json::as_str).unwrap_or_default().to_string();
        let Some(listed) = module.get_mut("procedures").and_then(Json::as_array_mut) else { continue };
        for procedure in listed {
            let of = procedure.get("name").and_then(Json::as_str).unwrap_or_default().to_string();
            let said: Vec<&Claim> = claims.iter().filter(|claim| claim.is_of(&name, &of)).collect();
            if procedure.get("what").and_then(Json::as_str) != Some("Declare") {
                procedures += 1;
                if said.iter().any(|claim| claim.state != State::Left) {
                    spoken_for += 1;
                }
            }
            if let Some(fields) = procedure.as_object_mut() {
                fields.insert("rebuilt".into(), Json::Array(said.iter().map(|claim| claim.json()).collect()));
            }
        }
    }
    (procedures, spoken_for)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::tests::TempDir;
    use crate::module::{self};
    use crate::vba;

    const NOTE: &str = r#"
# What became of the macros of Budget.xlsm.
[[procedure]]
module = "Totals"
name = "AddUp"
state = "rebuilt"
script = "add_up.py"
note = "Sums A1:A10 and writes the total below."

[[procedure]]
module = "ThisWorkbook"
name = "Workbook_Open"
state = "left"
note = "Ran by itself when the workbook opened: nothing does here."

[[procedure]]
module = "Totals"
name = "Report"
state = "partly"
script = "report.py"
"#;

    fn project(t: &TempDir, id: &str, workbook: Option<&str>, note: Option<&str>) -> Module {
        let names = workbook.map(|name| format!("workbook = {name:?}\n")).unwrap_or_default();
        t.write(
            &format!("{id}/module.toml"),
            &format!("id = \"{id}\"\nname = \"The {id}\"\nversion = \"1\"\npermissions = [\"python\"]\n{names}\n[python]\nscripts = \"python\"\n"),
        );
        t.write(&format!("{id}/python/add_up.py"), "def main(data):\n    return 1\n");
        if let Some(note) = note {
            t.write(&format!("{id}/{FILE}"), note);
        }
        module::load(&t.0.join(id))
    }

    #[test]
    fn reads_a_note() {
        let note = parse(NOTE).unwrap();
        assert_eq!(note.entries.len(), 3);
        assert_eq!(
            note.entries[0],
            Entry {
                module: "Totals".into(),
                procedure: "AddUp".into(),
                state: State::Rebuilt,
                script: Some("add_up.py".into()),
                note: "Sums A1:A10 and writes the total below.".into(),
            }
        );
        assert_eq!((note.entries[1].state, note.entries[1].script.as_deref()), (State::Left, None));
        assert_eq!((note.count(State::Rebuilt), note.count(State::Partly), note.count(State::Left)), (1, 1, 1));
        // A note that speaks of nothing is a note; keys it doesn't know are ignored.
        assert_eq!(parse("").unwrap(), Note::default());
        assert_eq!(parse("workbook = \"x\"\n[[procedure]]\nmodule = \"M\"\nname = \"P\"\nstate = \"left\"\nfuture = 1").unwrap().entries.len(), 1);
        for state in State::ALL {
            assert_eq!(State::parse(state.code()), Some(state));
            assert!(!state.label().is_empty());
        }
    }

    #[test]
    fn refuses_what_it_does_not_understand() {
        let with = |from: &str, to: &str| NOTE.replace(from, to);
        let bad = [
            ("a state it doesn't know", with("\"rebuilt\"", "\"done\"")),
            ("no state", with("state = \"left\"\n", "")),
            ("no module", with("module = \"ThisWorkbook\"\n", "")),
            ("no name", with("name = \"AddUp\"", "name = \" \"")),
            ("a script that climbs out", with("add_up.py", "../../outside.py")),
            ("an absolute script", with("add_up.py", "/usr/bin/python")),
            ("a note with a line break", with("writes the total below.", "writes\\nthe total")),
            ("a name with control characters", with("AddUp", "Add\\u0007Up")),
            ("not TOML", "[[procedure".to_string()),
            ("procedures of the wrong type", "procedure = 7".to_string()),
        ];
        for (what, source) in bad {
            assert!(parse(&source).is_err(), "{what} should be refused");
        }
        assert!(parse(&with("Sums A1:A10 and writes the total below.", &"n".repeat(MAX_NOTE + 1))).is_err(), "a note too long");
        assert!(parse(&" ".repeat(MAX_BYTES as usize + 1)).is_err(), "a file too large");
        let many: String = (0..=MAX_ENTRIES).map(|n| format!("[[procedure]]\nmodule = \"M\"\nname = \"P{n}\"\nstate = \"left\"\n")).collect();
        assert!(parse(&many).unwrap_err().contains("more than 2000 procedures"));
        // Which entry is wrong is said.
        assert!(parse(&with("\"partly\"", "\"soon\"")).unwrap_err().contains("procedure 3 has the state \"soon\""));
    }

    #[test]
    fn a_projects_note_is_reported_with_it_and_a_missing_script_is_said() {
        let t = TempDir::new("rebuilt-describe");
        assert_eq!(describe(&project(&t, "none", None, None)), Json::Null, "a project without a note has none");
        let told = describe(&project(&t, "budget", Some("Budget.xlsm"), Some(NOTE)));
        assert_eq!((&told["problem"], &told["rebuilt"], &told["partly"], &told["left"]), (&Json::Null, &json!(1), &json!(1), &json!(1)));
        assert_eq!(
            told["procedures"][0],
            json!({"module": "Totals", "name": "AddUp", "state": "rebuilt", "script": "add_up.py", "scriptMissing": false, "note": "Sums A1:A10 and writes the total below."})
        );
        assert_eq!((&told["procedures"][2]["script"], &told["procedures"][2]["scriptMissing"]), (&json!("report.py"), &json!(true)));
        assert_eq!(told["procedures"][1]["scriptMissing"], false, "what names no script misses none");
        // A note that can't be read says why, and says nothing else.
        let broken = describe(&project(&t, "broken", Some("Budget.xlsm"), Some("[[procedure]]\nstate = \"done\"")));
        assert!(broken["problem"].as_str().unwrap().contains("procedure 1 has the state \"done\""), "{broken}");
        assert_eq!(broken["procedures"], json!([]));
    }

    #[test]
    fn only_the_projects_of_a_workbook_speak_of_its_macros() {
        let t = TempDir::new("rebuilt-claims");
        let projects = [
            project(&t, "budget", Some("Budget.xlsm"), Some(NOTE)),
            // Another workbook's code, a library, a project without a note, one whose note is broken.
            project(&t, "sales", Some("Sales.xlsm"), Some(NOTE)),
            project(&t, "library", None, Some(NOTE)),
            project(&t, "quiet", Some("Budget.xlsm"), None),
            project(&t, "broken", Some("Budget.xlsm"), Some("[[procedure]]\nstate = \"done\"")),
        ];
        let said = claims(&projects, "budget.XLSM");
        assert_eq!(said.len(), 3, "a file's name, whatever its capitals");
        assert!(said.iter().all(|claim| claim.project == "budget" && claim.project_name == "The budget"));
        assert!(said[0].is_of("totals", "ADDUP") && !said[0].is_of("Totals", "Report"), "VBA's names know no capitals");
        assert!(!said[0].script_missing && said[2].script_missing);
        assert!(claims(&projects, "Other.xlsm").is_empty() && claims(&projects, "").is_empty());
        assert_eq!(claims(&projects, "Sales.xlsm").len(), 3);
    }

    #[test]
    fn what_became_of_each_procedure_is_written_beside_it() {
        let t = TempDir::new("rebuilt-mark");
        let projects = [project(&t, "budget", Some("Budget.xlsm"), Some(NOTE))];
        let totals = "Sub AddUp()\r\nEnd Sub\r\n\r\nSub Report()\r\nEnd Sub\r\n\r\nSub Untouched()\r\nEnd Sub\r\n\r\nPrivate Declare PtrSafe Function GetTickCount Lib \"kernel32\" () As Long\r\n";
        let book = "Private Sub Workbook_Open()\r\nEnd Sub\r\n";
        let bytes = vba::testing::project("Budget", &[("Totals", vba::Kind::Standard, totals), ("ThisWorkbook", vba::Kind::Document, book)]);
        let mut summary = vba::summary(&vba::read(&bytes).unwrap());
        let (procedures, spoken_for) = mark(&mut summary, &claims(&projects, "Budget.xlsm"));
        assert_eq!((procedures, spoken_for), (4, 2), "the door to a library is not counted, and what was left is not rebuilt");
        let of = |module: usize, procedure: usize| summary["modules"][module]["procedures"][procedure]["rebuilt"].clone();
        assert_eq!(
            of(0, 0),
            json!([{"project": "budget", "projectName": "The budget", "state": "rebuilt", "script": "add_up.py", "scriptMissing": false, "note": "Sums A1:A10 and writes the total below."}])
        );
        assert_eq!((&of(0, 1)[0]["state"], &of(0, 1)[0]["scriptMissing"]), (&json!("partly"), &json!(true)));
        assert_eq!(of(0, 2), json!([]), "nobody speaks of it");
        assert_eq!(of(0, 3), json!([]));
        assert_eq!(of(1, 0)[0]["state"], "left");
        // Without projects, every procedure says that nobody speaks of it.
        let mut bare = vba::summary(&vba::read(&bytes).unwrap());
        assert_eq!(mark(&mut bare, &[]), (4, 0));
        assert_eq!(bare["modules"][0]["procedures"][0]["rebuilt"], json!([]));
    }
}
