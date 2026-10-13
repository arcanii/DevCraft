//! How a project's files are read for showing: a kit's description of the languages its
//! projects are written in, and the colouring that is done from it.
//!
//! A development kit says, for each kind of file its projects hold, what a comment, a string, a
//! number and a keyword is ([`Language`]). That is a description, not code: nothing a kit or a
//! project brings is run to show a file. [`spans`] reads a text against a description and says
//! which stretches of it are what, and [`show`] cuts that into lines for whoever draws them.
//! A file is shown as text: nothing in it is followed, loaded or run.
//!
//! It is a plain reading, not a parser: it knows comments, quoted text, numbers, the words a
//! language keeps for itself, and a few things of a file's shape (a tag, a key, a heading). It
//! can be wrong about what a line means and still be right about what it shows.
//!
//! Written here. No highlighter and no grammar is copied in.

use crate::kit::Kit;

/// The largest file shown (bytes). A longer one is said to be too long.
pub const MAX_BYTES: u64 = 256 * 1024;
/// The most lines of a file shown. The rest is said to be left out.
pub const MAX_LINES: usize = 2_000;
/// The most of one line shown (characters). The rest of the line is left out.
pub const MAX_LINE_CHARS: usize = 1_000;

/// What a stretch of a file is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Comment,
    /// Quoted text.
    Text,
    Number,
    /// A word the language keeps for itself.
    Keyword,
    /// A name the file's shape gives weight to: a tag, a key, a section, a heading.
    Name,
}

impl Kind {
    pub const ALL: [Kind; 5] = [Kind::Comment, Kind::Text, Kind::Number, Kind::Keyword, Kind::Name];

    /// The word whoever draws is told.
    pub fn code(self) -> &'static str {
        match self {
            Kind::Comment => "comment",
            Kind::Text => "string",
            Kind::Number => "number",
            Kind::Keyword => "keyword",
            Kind::Name => "name",
        }
    }
}

/// How quoted text begins and ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quote {
    pub open: &'static str,
    pub close: &'static str,
    /// A backslash takes the next character out of play.
    pub escape: bool,
    /// It may run over several lines. Otherwise it ends with its line, closed or not.
    pub lines: bool,
}

const fn quote(mark: &'static str) -> Quote {
    Quote { open: mark, close: mark, escape: true, lines: false }
}

const fn long_quote(mark: &'static str) -> Quote {
    Quote { open: mark, close: mark, escape: true, lines: true }
}

/// What the lines of a file are, beyond comments, quotes, numbers and keywords.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// A program: nothing more.
    Code,
    /// Tags: the word after `<` or `</` is a name.
    Markup,
    /// Keys and sections: `key = value`, and `[section]` on a line of its own.
    Keys,
    /// Prose with headings: a line that begins with `#`.
    Prose,
}

/// A language, as a kit describes it.
#[derive(Debug, PartialEq, Eq)]
pub struct Language {
    /// What it is called.
    pub name: &'static str,
    /// The files it is for: endings without their dot, in small letters.
    pub endings: &'static [&'static str],
    /// What begins a comment that runs to the end of its line.
    pub line_comments: &'static [&'static str],
    /// What begins and ends a comment that can run over lines.
    pub block_comments: &'static [(&'static str, &'static str)],
    pub quotes: &'static [Quote],
    /// The words it keeps for itself.
    pub keywords: &'static [&'static str],
    pub numbers: bool,
    pub shape: Shape,
}

pub const C: Language = Language {
    name: "C",
    endings: &["c", "h"],
    line_comments: &["//"],
    block_comments: &[("/*", "*/")],
    quotes: &[quote("\""), quote("'")],
    keywords: &[
        "auto", "bool", "break", "case", "char", "const", "continue", "default", "define", "do", "double", "elif", "else", "endif", "enum", "extern",
        "false", "float", "for", "goto", "if", "ifdef", "ifndef", "include", "inline", "int", "long", "pragma", "register", "restrict", "return",
        "short", "signed", "sizeof", "static", "struct", "switch", "true", "typedef", "undef", "union", "unsigned", "void", "volatile", "while",
    ],
    numbers: true,
    shape: Shape::Code,
};

pub const PYTHON: Language = Language {
    name: "Python",
    endings: &["py"],
    line_comments: &["#"],
    block_comments: &[],
    quotes: &[long_quote("\"\"\""), long_quote("'''"), quote("\""), quote("'")],
    keywords: &[
        "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue", "def", "del", "elif", "else", "except",
        "finally", "for", "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
        "with", "yield",
    ],
    numbers: true,
    shape: Shape::Code,
};

pub const JAVASCRIPT: Language = Language {
    name: "JavaScript",
    endings: &["js", "mjs", "ts"],
    line_comments: &["//"],
    block_comments: &[("/*", "*/")],
    quotes: &[quote("\""), quote("'"), long_quote("`")],
    keywords: &[
        "async",
        "await",
        "break",
        "case",
        "catch",
        "class",
        "const",
        "continue",
        "default",
        "delete",
        "do",
        "else",
        "export",
        "extends",
        "false",
        "finally",
        "for",
        "from",
        "function",
        "if",
        "import",
        "in",
        "instanceof",
        "let",
        "new",
        "null",
        "of",
        "return",
        "static",
        "super",
        "switch",
        "this",
        "throw",
        "true",
        "try",
        "typeof",
        "undefined",
        "var",
        "void",
        "while",
        "yield",
    ],
    numbers: true,
    shape: Shape::Code,
};

pub const MARKUP: Language = Language {
    name: "HTML",
    endings: &["html", "htm", "svelte", "svg", "xml"],
    line_comments: &[],
    block_comments: &[("<!--", "-->")],
    // Ended with the line: an apostrophe in a page's own words must not colour the lines after it.
    quotes: &[Quote { open: "\"", close: "\"", escape: false, lines: false }, Quote { open: "'", close: "'", escape: false, lines: false }],
    keywords: &[],
    numbers: false,
    shape: Shape::Markup,
};

pub const CSS: Language = Language {
    name: "CSS",
    endings: &["css"],
    line_comments: &[],
    block_comments: &[("/*", "*/")],
    quotes: &[quote("\""), quote("'")],
    keywords: &[],
    numbers: true,
    shape: Shape::Code,
};

pub const TOML: Language = Language {
    name: "TOML",
    endings: &["toml"],
    line_comments: &["#"],
    block_comments: &[],
    quotes: &[
        long_quote("\"\"\""),
        Quote { open: "'''", close: "'''", escape: false, lines: true },
        quote("\""),
        Quote { open: "'", close: "'", escape: false, lines: false },
    ],
    keywords: &["false", "true"],
    numbers: true,
    shape: Shape::Keys,
};

pub const JSON: Language = Language {
    name: "JSON",
    endings: &["json", "map"],
    line_comments: &[],
    block_comments: &[],
    quotes: &[quote("\"")],
    keywords: &["false", "null", "true"],
    numbers: true,
    shape: Shape::Code,
};

pub const MARKDOWN: Language = Language {
    name: "Markdown",
    endings: &["md"],
    line_comments: &[],
    block_comments: &[],
    quotes: &[Quote { open: "```", close: "```", escape: false, lines: true }, Quote { open: "`", close: "`", escape: false, lines: false }],
    keywords: &[],
    numbers: false,
    shape: Shape::Prose,
};

/// The language one of a kit's files is read as, by the file's ending: one of those the kit
/// describes. `None` for a file the kit says nothing of, which is shown plain.
pub fn language_for(kit: &Kit, path: &str) -> Option<&'static Language> {
    let name = path.rsplit('/').next().unwrap_or(path);
    let ending = name.rsplit_once('.').map(|(_, ending)| ending.to_ascii_lowercase())?;
    kit.languages.iter().copied().find(|language| language.endings.contains(&ending.as_str()))
}

/// A stretch of a text that is something: where it begins and ends (bytes), and what it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub kind: Kind,
}

fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// Where the quoted text that begins `rest` ends, counted from its beginning.
fn quote_end(rest: &str, quote: &Quote) -> usize {
    let mut at = quote.open.len();
    while let Some(after) = rest.get(at..) {
        if after.starts_with(quote.close) {
            return at + quote.close.len();
        }
        let Some(next) = after.chars().next() else { break };
        if next == '\n' && !quote.lines {
            return at;
        }
        at += next.len_utf8();
        if next == '\\' && quote.escape {
            // The character a backslash takes out of play, unless it would end the line.
            match rest.get(at..).and_then(|after| after.chars().next()) {
                Some('\n') if !quote.lines => {}
                Some(taken) => at += taken.len_utf8(),
                None => {}
            }
        }
    }
    rest.len()
}

/// What a line's shape makes of its beginning: where a name in it begins and ends.
fn line_name(shape: Shape, rest: &str) -> Option<(usize, usize)> {
    let line = rest.split('\n').next().unwrap_or(rest);
    let lead = line.len() - line.trim_start_matches([' ', '\t']).len();
    let after = line.get(lead..)?;
    match shape {
        Shape::Keys if after.starts_with('[') => {
            let close = after.rfind(']')?;
            Some((lead, lead + close + 1))
        }
        Shape::Keys => {
            let key = after.bytes().take_while(|byte| is_word(*byte) || matches!(byte, b'-' | b'.')).count();
            let then = after.get(key..)?.trim_start_matches([' ', '\t']);
            (key > 0 && then.starts_with('=')).then_some((lead, lead + key))
        }
        Shape::Prose if lead <= 3 && after.starts_with('#') => Some((lead, line.trim_end_matches('\r').len())),
        _ => None,
    }
}

/// The stretches of `text` that are something in `language`, in order and never overlapping.
/// What lies between them is plain. It reads any text at all, and once through.
pub fn spans(language: &Language, text: &str) -> Vec<Span> {
    let bytes = text.as_bytes();
    let mut found = Vec::new();
    let mut at = 0;
    let mut line_begins = true;
    while let (Some(rest), Some(&byte)) = (text.get(at..), bytes.get(at)) {
        if std::mem::take(&mut line_begins)
            && let Some((start, end)) = line_name(language.shape, rest)
            && end > start
        {
            found.push(Span { start: at + start, end: at + end, kind: Kind::Name });
            at += end;
            continue;
        }
        if byte == b'\n' {
            line_begins = true;
            at += 1;
            continue;
        }
        if let Some((open, close)) = language.block_comments.iter().find(|(open, _)| rest.starts_with(open)) {
            let end = rest.get(open.len()..).and_then(|after| after.find(close)).map_or(rest.len(), |closed| open.len() + closed + close.len());
            found.push(Span { start: at, end: at + end, kind: Kind::Comment });
            at += end;
            continue;
        }
        if language.line_comments.iter().any(|open| rest.starts_with(open)) {
            let end = rest.find('\n').unwrap_or(rest.len());
            found.push(Span { start: at, end: at + end, kind: Kind::Comment });
            at += end;
            continue;
        }
        // The longest opening that fits: three quotes before one.
        if let Some(quote) = language.quotes.iter().filter(|quote| rest.starts_with(quote.open)).max_by_key(|quote| quote.open.len()) {
            let end = quote_end(rest, quote).max(quote.open.len());
            found.push(Span { start: at, end: at + end, kind: Kind::Text });
            at += end;
            continue;
        }
        if language.numbers && byte.is_ascii_digit() {
            let end = rest.bytes().take_while(|byte| is_word(*byte) || *byte == b'.').count();
            found.push(Span { start: at, end: at + end, kind: Kind::Number });
            at += end;
            continue;
        }
        if byte.is_ascii_alphabetic() || byte == b'_' {
            let end = rest.bytes().take_while(|byte| is_word(*byte)).count();
            let word = rest.get(..end).unwrap_or_default();
            let tag = language.shape == Shape::Markup && (text.get(..at).is_some_and(|before| before.ends_with('<') || before.ends_with("</")));
            if tag {
                // A tag's name runs on through what a word does not hold.
                let end = rest.bytes().take_while(|byte| is_word(*byte) || matches!(byte, b'-' | b':')).count();
                found.push(Span { start: at, end: at + end, kind: Kind::Name });
                at += end;
                continue;
            }
            if language.keywords.contains(&word) {
                found.push(Span { start: at, end: at + end, kind: Kind::Keyword });
            }
            at += end;
            continue;
        }
        at += rest.chars().next().map_or(1, char::len_utf8);
    }
    found
}

/// A piece of a line: its text, and what it is (`None`: plain).
pub type Piece = (String, Option<Kind>);

/// A file as it is shown: its lines, each in pieces.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shown {
    /// The language it was read as. `None`: the kit says nothing of such a file, and it is plain.
    pub language: Option<&'static str>,
    pub lines: Vec<Vec<Piece>>,
    /// How many lines the file has, shown or not.
    pub of_lines: usize,
}

impl Shown {
    /// Whether lines were left out.
    pub fn cut(&self) -> bool {
        self.of_lines > self.lines.len()
    }
}

/// Adds a stretch of the text to the lines, breaking it where the text's own lines break.
fn add(lines: &mut Vec<Vec<Piece>>, widths: &mut Vec<usize>, stretch: &str, kind: Option<Kind>) {
    for (n, part) in stretch.split('\n').enumerate() {
        if n > 0 {
            lines.push(Vec::new());
            widths.push(0);
        }
        let part = part.strip_suffix('\r').unwrap_or(part);
        if part.is_empty() || lines.len() > MAX_LINES {
            continue;
        }
        let (Some(line), Some(width)) = (lines.last_mut(), widths.last_mut()) else { continue };
        let room = MAX_LINE_CHARS.saturating_sub(*width);
        if room == 0 {
            continue;
        }
        let long = part.chars().count();
        if long > room {
            let kept: String = part.chars().take(room).collect();
            line.push((kept, kind));
            line.push((" …".to_string(), None));
            *width = MAX_LINE_CHARS;
        } else {
            line.push((part.to_string(), kind));
            *width += long;
        }
    }
}

/// A file of one of a kit's projects, as it is shown: read as the language the kit describes
/// for such a file, or plain. At most [`MAX_LINES`] lines, each at most [`MAX_LINE_CHARS`] long.
pub fn show(kit: &Kit, path: &str, text: &str) -> Shown {
    let language = language_for(kit, path);
    let mut lines: Vec<Vec<Piece>> = vec![Vec::new()];
    let mut widths = vec![0usize];
    let mut at = 0;
    for span in language.map(|language| spans(language, text)).unwrap_or_default() {
        if lines.len() > MAX_LINES {
            break;
        }
        add(&mut lines, &mut widths, text.get(at..span.start).unwrap_or_default(), None);
        add(&mut lines, &mut widths, text.get(span.start..span.end).unwrap_or_default(), Some(span.kind));
        at = span.end;
    }
    if lines.len() <= MAX_LINES {
        add(&mut lines, &mut widths, text.get(at..).unwrap_or_default(), None);
    }
    // A text that ends with its last line's end has no line after it.
    let of_lines = text.split('\n').count() - usize::from(text.ends_with('\n') || text.is_empty());
    lines.truncate(of_lines.min(MAX_LINES));
    Shown { language: language.map(|language| language.name), lines, of_lines }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kit;

    fn read(language: &Language, text: &str) -> Vec<(String, Kind)> {
        spans(language, text).into_iter().map(|span| (text[span.start..span.end].to_string(), span.kind)).collect()
    }

    fn all(language: &Language, text: &str, kind: Kind) -> Vec<String> {
        read(language, text).into_iter().filter(|(_, found)| *found == kind).map(|(text, _)| text).collect()
    }

    #[test]
    fn a_c_file_is_read_as_comments_text_numbers_and_the_languages_own_words() {
        let text = "/* A program.\n   Two lines. */\n#include <stdio.h>\n\nint main(void)\n{\n    // say it\n    puts(\"Hello \\\"you\\\" // not a comment\");\n    return 0x1F + 'a' + 3.5f; /* done */\n}\n";
        assert_eq!(all(&C, text, Kind::Comment), ["/* A program.\n   Two lines. */", "// say it", "/* done */"]);
        assert_eq!(all(&C, text, Kind::Text), ["\"Hello \\\"you\\\" // not a comment\"", "'a'"]);
        assert_eq!(all(&C, text, Kind::Number), ["0x1F", "3.5f"]);
        assert_eq!(all(&C, text, Kind::Keyword), ["include", "int", "void", "return"]);
        // A word that only holds a keyword is not one, and a digit inside a name is no number.
        assert!(read(&C, "int interval2 = printf;").iter().all(|(text, kind)| *kind == Kind::Keyword && text == "int"));
    }

    #[test]
    fn each_language_is_read_by_its_own_description() {
        let python =
            "\"\"\"A script.\n\nIt has # no comment here.\n\"\"\"\nimport os  # the system\n\ndef main(data):\n    return {'rows': 2, \"n\": None}\n";
        assert_eq!(all(&PYTHON, python, Kind::Text), ["\"\"\"A script.\n\nIt has # no comment here.\n\"\"\"", "'rows'", "\"n\""]);
        assert_eq!(all(&PYTHON, python, Kind::Comment), ["# the system"]);
        assert_eq!(all(&PYTHON, python, Kind::Keyword), ["import", "def", "return", "None"]);
        let script = "const say = (line) => `it said\n${line}`; // twice\nlet n = 2;\n";
        assert_eq!(all(&JAVASCRIPT, script, Kind::Text), ["`it said\n${line}`"]);
        assert_eq!(all(&JAVASCRIPT, script, Kind::Keyword), ["const", "let"]);
        let page = "<!doctype html>\n<!-- a page -->\n<div class=\"box\" id='a'>if x</div>\n<my-part a:b=\"1\"/>\n";
        assert_eq!(all(&MARKUP, page, Kind::Name), ["div", "div", "my-part"]);
        assert_eq!(all(&MARKUP, page, Kind::Comment), ["<!-- a page -->"]);
        assert_eq!(all(&MARKUP, page, Kind::Text), ["\"box\"", "'a'", "\"1\""]);
        assert!(all(&MARKUP, page, Kind::Keyword).is_empty(), "a page keeps no words for itself");
        let manifest = "# A project.\nid = \"blink\"\nversion.major = 1\npermissions = [\"read\", 'write']\n\n  [panel]\nentry = \"panel/index.html\"  # its page\non = true\n";
        assert_eq!(all(&TOML, manifest, Kind::Name), ["id", "version.major", "permissions", "[panel]", "entry", "on"]);
        assert_eq!(all(&TOML, manifest, Kind::Comment), ["# A project.", "# its page"]);
        assert_eq!(all(&TOML, manifest, Kind::Keyword), ["true"]);
        assert_eq!(all(&TOML, manifest, Kind::Number), ["1"]);
        let notes = "# Doom\n\nBuilt with `zig`, not # this.\n   ## Using it\n    # too far in to be a heading\n```\n# code, not a heading\n```\n";
        assert_eq!(all(&MARKDOWN, notes, Kind::Name), ["# Doom", "## Using it"]);
        assert_eq!(all(&MARKDOWN, notes, Kind::Text), ["`zig`", "```\n# code, not a heading\n```"]);
        let data = "{\"a\": [1, -2.5e3, true, null], \"b\": \"x\"}";
        assert_eq!(all(&JSON, data, Kind::Number), ["1", "2.5e3"]);
        assert_eq!(all(&JSON, data, Kind::Keyword), ["true", "null"]);
        assert_eq!(all(&CSS, "a { color: #fff; width: 12px } /* x */", Kind::Comment), ["/* x */"]);
    }

    #[test]
    fn any_text_at_all_is_read_and_nothing_of_it_is_lost() {
        let odd = [
            "",
            "\n",
            "\r\n\r\n",
            "\"never closed",
            "/* never closed",
            "'a\\",
            "\"a\\\nb\"",
            "\"\"\"",
            "`",
            "<",
            "</",
            "[",
            "= 1",
            "#",
            "日本語 \"テキスト\" // コメント\n",
            "é1 = 2é",
            "\u{0}\u{1}\u{7f}\"\u{0}\"",
            "a\tb\u{2028}c",
            "0x",
            "1.2.3.4",
            "\\\\\\",
            "]]]] = [[[",
            "    \t  ",
        ];
        for language in [&C, &PYTHON, &JAVASCRIPT, &MARKUP, &CSS, &TOML, &JSON, &MARKDOWN] {
            for text in odd {
                let found = spans(language, text);
                let mut at = 0;
                for span in &found {
                    assert!(span.start >= at && span.end > span.start && span.end <= text.len(), "{} on {text:?}: {span:?}", language.name);
                    assert!(text.is_char_boundary(span.start) && text.is_char_boundary(span.end), "{} on {text:?}", language.name);
                    at = span.end;
                }
                // What is shown is the text again, piece by piece, whatever it was read as.
                let mut anie = kit::ANIE;
                let only: &'static [&'static Language] = Box::leak(Box::new([language]));
                anie.languages = only;
                let name = format!("x.{}", language.endings[0]);
                let shown = show(&anie, &name, text);
                let again: Vec<String> = shown.lines.iter().map(|line| line.iter().map(|(piece, _)| piece.as_str()).collect()).collect();
                let lines: Vec<&str> = text.lines().collect();
                assert_eq!(again, lines, "{} on {text:?}", language.name);
            }
        }
        // A long run of openings is read once through, not once for each.
        let many = "/* ".repeat(50_000);
        assert_eq!(spans(&C, &many).len(), 1);
        assert_eq!(spans(&MARKDOWN, &"`".repeat(100_001)).len(), 16_667);
    }

    #[test]
    fn a_file_is_shown_in_lines_and_only_so_much_of_it() {
        let shown = show(&kit::C, "src/main.c", "int a; // one\r\nchar *b = \"two\";\n");
        assert_eq!(shown.language, Some("C"));
        assert_eq!(
            shown.lines,
            vec![
                vec![("int".to_string(), Some(Kind::Keyword)), (" a; ".to_string(), None), ("// one".to_string(), Some(Kind::Comment))],
                vec![
                    ("char".to_string(), Some(Kind::Keyword)),
                    (" *b = ".to_string(), None),
                    ("\"two\"".to_string(), Some(Kind::Text)),
                    (";".to_string(), None)
                ],
            ]
        );
        assert!(!shown.cut() && shown.of_lines == 2);
        // A comment over several lines is a piece of each of them.
        let over = show(&kit::C, "a.h", "/* one\n\n   three */ int x;");
        assert_eq!(over.lines[0], [("/* one".to_string(), Some(Kind::Comment))]);
        assert_eq!(over.lines[1], []);
        assert_eq!(over.lines[2][0], ("   three */".to_string(), Some(Kind::Comment)));
        assert_eq!(over.of_lines, 3);
        // A file the kit says nothing of is shown plain: a C project's Python is not read as Python.
        let plain = show(&kit::C, "notes.py", "import os  # x\n");
        assert_eq!((plain.language, &plain.lines), (None, &vec![vec![("import os  # x".to_string(), None)]]));
        assert_eq!(show(&kit::ANIE, "notes.py", "import os\n").language, Some("Python"));
        assert_eq!(show(&kit::C, "Makefile", "all:\n").language, None, "no ending, no language");
        // Only so many lines, and only so much of a line.
        let long = "x = 1\n".repeat(MAX_LINES + 50);
        let cut = show(&kit::ANIE, "long.py", &long);
        assert_eq!((cut.lines.len(), cut.of_lines, cut.cut()), (MAX_LINES, MAX_LINES + 50, true));
        let wide = format!("# {}\nx = 1\n", "é".repeat(MAX_LINE_CHARS * 3));
        let narrow = show(&kit::ANIE, "wide.py", &wide);
        let first: usize = narrow.lines[0].iter().map(|(piece, _)| piece.chars().count()).sum();
        assert!(first <= MAX_LINE_CHARS + 2 && narrow.lines[0].last().unwrap().0 == " …", "{first}");
        assert_eq!(narrow.lines[1][0].0, "x = ", "the line after a long one is whole");
        assert_eq!(show(&kit::C, "empty.c", "").lines, Vec::<Vec<Piece>>::new());
    }

    #[test]
    fn a_kit_describes_the_languages_of_its_own_files() {
        // Each kit reads what its projects are written in, and what every project holds.
        for (kit, path, name) in [
            (&kit::C, "src/main.c", Some("C")),
            (&kit::C, "doom/doomdef.H", Some("C")),
            (&kit::C, "panel/index.html", Some("HTML")),
            (&kit::C, "panel/doom.js", Some("JavaScript")),
            (&kit::C, "project.toml", Some("TOML")),
            (&kit::C, "README.md", Some("Markdown")),
            (&kit::C, "python/clean.py", None),
            (&kit::ANIE, "python/clean.py", Some("Python")),
            (&kit::ANIE, "source/src/App.svelte", Some("HTML")),
            (&kit::ANIE, "panel/app.css", Some("CSS")),
            (&kit::ANIE, "rebuilt.toml", Some("TOML")),
            (&kit::ANIE, "src/main.c", None),
            (&kit::ANIE, "panel/doom1.wad", None),
            (&kit::ANIE, ".gitignore", None),
        ] {
            assert_eq!(language_for(kit, path).map(|language| language.name), name, "{} {path}", kit.id);
        }
        for kit in kit::loaded() {
            assert!(!kit.languages.is_empty(), "{}", kit.id);
            let mut endings: Vec<&str> = kit.languages.iter().flat_map(|language| language.endings.iter().copied()).collect();
            let all = endings.len();
            endings.sort_unstable();
            endings.dedup();
            assert_eq!(endings.len(), all, "{}: an ending is read as one language", kit.id);
            for language in kit.languages {
                assert!(!language.name.is_empty() && !language.endings.is_empty());
                assert!(language.endings.iter().all(|ending| *ending == ending.to_ascii_lowercase() && !ending.contains('.')));
                // An opening that is empty would never be got past.
                assert!(language.line_comments.iter().all(|open| !open.is_empty()));
                assert!(language.block_comments.iter().all(|(open, close)| !open.is_empty() && !close.is_empty()));
                assert!(language.quotes.iter().all(|quote| !quote.open.is_empty() && !quote.close.is_empty()));
            }
        }
        assert_eq!(Kind::ALL.map(Kind::code), ["comment", "string", "number", "keyword", "name"]);
    }
}
