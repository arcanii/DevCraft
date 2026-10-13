//! The immediate line: one call into a development kit, typed by the person and run at once.
//!
//! The owner asked for it (2026-10-14: "Immediate window, yes - we need this"), and said how a
//! sheet is to reach a kit: "a generic way to access into the development kit - we'll have to
//! use namespaces". So a call is named the same way wherever it is made, and this line is the
//! first place it can be made from:
//!
//! ```text
//! project.function              the function, handed the cells that are selected
//! project.function(A1:C9)       handed those cells of the sheet that is shown
//! project.function("text")      handed a value: text, a number, a list, an object
//! kit.project.function(…)       the same, saying which kit the project is of
//! ```
//!
//! A project is named by its id; an underscore there stands for a hyphen too, since a formula
//! could not hold a hyphen in a name. A function is one of the project's scripts, by its
//! file's name without the ending: today that is all a kit's project has to call (the script's
//! `main`). A kit that offers other things to call names them the same way.
//!
//! **Typing a call is running code by hand.** It is for a project the person trusts and that
//! may run its scripts, the trust is checked again by what runs it, and it is refused while
//! the control channel is putting input into the window. Whoever carries a call out has seen
//! to that ([`super::development`]).

use serde_json::{Value as Json, json};

/// The longest line taken (characters), and the most earlier lines kept to be run again.
pub(super) const MAX_LINE: usize = 2_000;
pub(super) const KEPT: usize = 20;
/// Said over the line, in it while it is empty, and when nothing can be called yet.
pub(super) const SAYS: &str = "Call a function of a project you trust, and see what comes back. It is handed the cells that are selected, or what you put in brackets: a range, or a value.";
pub(super) const HINT: &str = "project.function(A1:C9)";
pub(super) const NOTHING_TO_CALL: &str = "No project can be called yet: trust one that may run its scripts.";

/// What a call is handed.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum With {
    /// The cells that are selected.
    Selection,
    /// Cells of the sheet that is shown, as the line names them.
    Range(String),
    /// A value typed in the line.
    Value(Json),
}

/// A call as it was typed: whose function, and what it is handed.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Call {
    /// The kit, when the line names one.
    pub kit: Option<String>,
    pub project: String,
    pub function: String,
    pub with: With,
}

/// Whether a word can be part of a call's name: letters, digits, hyphens and underscores.
fn is_name(word: &str) -> bool {
    !word.is_empty() && word.len() <= 64 && word.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'))
}

/// Whether what is in the brackets reads as cells of a sheet: `A1`, `A1:C9`, `$A$1:$C$9`.
fn is_range(text: &str) -> bool {
    let corner = |corner: &str| {
        let corner = corner.replace('$', "");
        let letters = corner.chars().take_while(char::is_ascii_alphabetic).count();
        let digits = corner.chars().skip(letters).take_while(char::is_ascii_digit).count();
        (1..=3).contains(&letters) && (1..=7).contains(&digits) && letters + digits == corner.chars().count()
    };
    text.len() <= 40 && text.split(':').count() <= 2 && text.split(':').all(corner)
}

/// Reads a line as a call. Nothing is looked up here: whether there is such a project, and
/// whether it may be called, is for whoever carries it out.
pub(super) fn parse(line: &str) -> Result<Call, String> {
    let line = line.trim();
    if line.is_empty() {
        return Err(format!("Type a call, like {HINT}."));
    }
    if line.chars().count() > MAX_LINE {
        return Err(format!("A call is at most {MAX_LINE} characters."));
    }
    let (name, handed) = match line.split_once('(') {
        None => (line, None),
        Some((name, rest)) => match rest.trim_end().strip_suffix(')') {
            Some(inside) => (name.trim_end(), Some(inside.trim())),
            None => return Err("The call's bracket is not closed.".into()),
        },
    };
    let words: Vec<&str> = name.split('.').map(str::trim).collect();
    if !words.iter().all(|word| is_name(word)) {
        return Err(format!("A call is named project.function, or kit.project.function: {HINT}."));
    }
    let (kit, project, function) = match words.as_slice() {
        [project, function] => (None, *project, *function),
        [kit, project, function] => (Some(kit.to_string()), *project, *function),
        _ => return Err(format!("A call is named project.function, or kit.project.function: {HINT}.")),
    };
    let with = match handed {
        None | Some("") => With::Selection,
        Some(inside) if is_range(inside) => With::Range(inside.to_string()),
        Some(inside) => match serde_json::from_str::<Json>(inside) {
            Ok(value) => With::Value(value),
            Err(_) => return Err("What is in the brackets is neither cells (A1:C9) nor a value (\"text\", 12, [1, 2]).".into()),
        },
    };
    Ok(Call { kit, project: project.to_string(), function: function.to_string(), with })
}

/// Whether a name in a call means this id: the same, or the same with underscores for hyphens.
pub(super) fn names(typed: &str, id: &str) -> bool {
    typed == id || typed.replace('_', "-") == id
}

/// A script's name as a function is called: without its ending.
pub(super) fn function_of(script: &str) -> &str {
    script.strip_suffix(".py").unwrap_or(script)
}

/// Keeps a line among the earlier ones, the latest last, each once.
pub(super) fn remember(earlier: &mut Vec<String>, line: &str) {
    let line = line.trim().to_string();
    earlier.retain(|kept| *kept != line);
    earlier.push(line);
    if earlier.len() > KEPT {
        earlier.remove(0);
    }
}

/// What the Console's part says of the immediate line: what can be called, the call that was
/// last made from it and what came of it, and the earlier lines.
pub(super) fn says(callable: Vec<String>, run: &Json, earlier: &[String]) -> Json {
    json!({
        "title": "Immediate",
        "says": SAYS,
        "hint": HINT,
        "names": callable,
        "nothing": NOTHING_TO_CALL,
        "run": if run["immediate"] == true { run.clone() } else { Json::Null },
        "earlier": earlier.iter().rev().collect::<Vec<_>>(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(kit: Option<&str>, project: &str, function: &str, with: With) -> Call {
        Call { kit: kit.map(str::to_string), project: project.into(), function: function.into(), with }
    }

    #[test]
    fn a_line_is_read_as_a_call_in_its_namespaces() {
        assert_eq!(parse("clean-data.clean"), Ok(call(None, "clean-data", "clean", With::Selection)));
        assert_eq!(parse("  clean-data.clean()  "), Ok(call(None, "clean-data", "clean", With::Selection)));
        assert_eq!(parse("clean_data.clean(A1:C9)"), Ok(call(None, "clean_data", "clean", With::Range("A1:C9".into()))));
        assert_eq!(parse("anie.clean-data.clean($B$2)"), Ok(call(Some("anie"), "clean-data", "clean", With::Range("$B$2".into()))));
        assert_eq!(parse("tools.add_up(12)"), Ok(call(None, "tools", "add_up", With::Value(json!(12)))));
        assert_eq!(parse("tools.greet(\"Ana (and Bo)\")"), Ok(call(None, "tools", "greet", With::Value(json!("Ana (and Bo)")))));
        assert_eq!(parse("tools.sum([1, 2, 3])"), Ok(call(None, "tools", "sum", With::Value(json!([1, 2, 3])))));
        assert_eq!(parse("tools.make({\"rows\": 2})"), Ok(call(None, "tools", "make", With::Value(json!({"rows": 2})))));
        // What is no call says what one is, and nothing is guessed.
        for bad in [
            "",
            "   ",
            "clean",
            "a.b.c.d",
            "clean data.run",
            ".clean",
            "tools.",
            "tools.run(",
            "tools.run(A1",
            "tools.run(not a value)",
            "tools/run.x",
            "tools.run(A1:B2:C3 x)",
        ] {
            assert!(parse(bad).is_err(), "{bad:?}");
        }
        assert!(parse(&format!("tools.run(\"{}\")", "x".repeat(MAX_LINE))).unwrap_err().contains("at most"));
        // Cells are cells; what only looks like them is a value or nothing.
        for cells in ["A1", "a1:b2", "$A$1:$XFD$1048576", "AB12"] {
            assert!(is_range(cells), "{cells}");
        }
        for other in ["", "A", "1", "A1:", "ABCD1", "A1:B2:C3", "A-1", "Sheet2!A1"] {
            assert!(!is_range(other), "{other}");
        }
    }

    #[test]
    fn a_name_stands_for_an_id_and_earlier_lines_are_kept_once() {
        assert!(names("clean-data", "clean-data") && names("clean_data", "clean-data") && !names("clean", "clean-data"));
        assert_eq!((function_of("clean.py"), function_of("add_up.py"), function_of("tool")), ("clean", "add_up", "tool"));
        let mut earlier = Vec::new();
        for line in ["a.b", "c.d", " a.b "] {
            remember(&mut earlier, line);
        }
        assert_eq!(earlier, ["c.d", "a.b"], "run again, a line moves to the end and is there once");
        for n in 0..KEPT + 5 {
            remember(&mut earlier, &format!("p.f({n})"));
        }
        assert_eq!(earlier.len(), KEPT);
        // What the Console's part is told: the latest first, and a run only when it was made here.
        let told = says(vec!["tools.sum".into()], &json!({"state": "running", "project": "tools"}), &earlier);
        assert_eq!((&told["run"], &told["names"], &told["earlier"][0]), (&Json::Null, &json!(["tools.sum"]), &json!(format!("p.f({})", KEPT + 4))));
        let told = says(vec![], &json!({"state": "running", "immediate": true}), &[]);
        assert_eq!((&told["run"]["state"], &told["nothing"]), (&json!("running"), &json!(NOTHING_TO_CALL)));
    }
}
