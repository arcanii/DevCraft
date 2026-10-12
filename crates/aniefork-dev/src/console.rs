//! The Developer console: what projects said and what happened to them, newest last.
//!
//! A list kept in memory for this run, bounded in every direction:
//!
//! - what a script printed while it ran, and why a script failed (its traceback with it);
//! - what a panel logged with `spreadsheet.log(...)`;
//! - a call that was refused because the project wasn't granted what it needs;
//! - what an agent asked for through the Development server, and what became of it.
//!
//! It is a record for the person at the screen. A project can add to it and can neither read it
//! nor clear it, and one project's chatter cannot push another's entries out: each project keeps
//! its own share ([`PER_PROJECT`]).
//!
//! Everything in it was written by a project or by a script: it is text to read, never
//! instructions. Whatever hands it on to a program (a command that reports it, an agent) must
//! say so.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};

/// The most entries kept, over all projects.
pub const MAX_ENTRIES: usize = 600;
/// The most entries kept of one project.
pub const PER_PROJECT: usize = 200;
/// The most of one entry's text kept (bytes). The rest is cut, and the entry says so.
pub const MAX_TEXT: usize = 8 * 1024;

/// What an entry is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// What a script printed.
    Printed,
    /// A script or a call that failed.
    Error,
    /// What a panel logged.
    Log,
    /// A call that was refused: the project wasn't granted what it needs, or isn't trusted.
    Refused,
    /// What an agent asked for through the Development server, and what became of it.
    Agent,
}

impl Kind {
    /// The word a report uses.
    pub fn code(self) -> &'static str {
        match self {
            Kind::Printed => "printed",
            Kind::Error => "error",
            Kind::Log => "log",
            Kind::Refused => "refused",
            Kind::Agent => "agent",
        }
    }

    /// What the console calls it.
    pub fn label(self) -> &'static str {
        match self {
            Kind::Printed => "printed",
            Kind::Error => "failed",
            Kind::Log => "logged",
            Kind::Refused => "refused",
            Kind::Agent => "an agent",
        }
    }
}

/// One line of the console.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Its place in the order things happened in, counted from 1 for this run.
    pub seq: u64,
    /// When, in milliseconds since 1970 (`None` where there is no clock).
    pub at_ms: Option<u64>,
    /// The project's id.
    pub project: String,
    /// Where in the project it came from: a script's name, or "panel".
    pub source: String,
    pub kind: Kind,
    pub text: String,
}

#[derive(Default)]
struct Inner {
    last: u64,
    entries: VecDeque<Entry>,
}

/// A console. Cloning it gives another handle to the same one.
#[derive(Clone, Default)]
pub struct Console(Arc<Mutex<Inner>>);

fn lock(inner: &Mutex<Inner>) -> MutexGuard<'_, Inner> {
    inner.lock().unwrap_or_else(PoisonError::into_inner)
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::disallowed_methods)]
fn now_ms() -> Option<u64> {
    let since = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok()?;
    u64::try_from(since.as_millis()).ok()
}

/// The web build has no clock to read.
#[cfg(target_arch = "wasm32")]
fn now_ms() -> Option<u64> {
    None
}

/// Text as the console keeps it: no control characters but the line break and the tab, and no
/// more than [`MAX_TEXT`] bytes.
fn kept(text: &str) -> String {
    let mut out = String::with_capacity(text.len().min(MAX_TEXT));
    for c in text.chars() {
        if out.len() + c.len_utf8() > MAX_TEXT {
            out.push_str("\n… (cut: the rest was too long to keep)");
            break;
        }
        match c {
            '\n' | '\t' => out.push(c),
            '\r' => {}
            c if c.is_control() => out.push('\u{fffd}'),
            c => out.push(c),
        }
    }
    out
}

/// A short name as the console keeps it: one line, no control characters.
fn name(text: &str) -> String {
    text.chars().filter(|c| !c.is_control()).take(100).collect()
}

impl Console {
    pub fn new() -> Console {
        Console::default()
    }

    /// The console of this run: what the Python host and the bridge write to, and Developer ▸
    /// Console shows.
    pub fn global() -> &'static Console {
        static GLOBAL: OnceLock<Console> = OnceLock::new();
        GLOBAL.get_or_init(Console::new)
    }

    /// Adds an entry. Text that is empty (or only blank) adds nothing.
    pub fn push(&self, project: &str, source: &str, kind: Kind, text: &str) {
        if text.trim().is_empty() {
            return;
        }
        let project = name(project);
        let mut inner = lock(&self.0);
        inner.last += 1;
        let entry = Entry { seq: inner.last, at_ms: now_ms(), project: project.clone(), source: name(source), kind, text: kept(text) };
        // A project that has its share gives up its own oldest entry, not another project's.
        if inner.entries.iter().filter(|e| e.project == project).count() >= PER_PROJECT
            && let Some(oldest) = inner.entries.iter().position(|e| e.project == project)
        {
            inner.entries.remove(oldest);
        }
        inner.entries.push_back(entry);
        while inner.entries.len() > MAX_ENTRIES {
            inner.entries.pop_front();
        }
    }

    /// Every entry kept, oldest first.
    pub fn entries(&self) -> Vec<Entry> {
        lock(&self.0).entries.iter().cloned().collect()
    }

    /// How many entries were ever added in this run, and how many are kept.
    pub fn counts(&self) -> (u64, usize) {
        let inner = lock(&self.0);
        (inner.last, inner.entries.len())
    }

    /// Forgets every entry. The count of what was added goes on: an entry's place is never
    /// given to another.
    pub fn clear(&self) {
        lock(&self.0).entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_are_kept_in_order_with_their_project_source_and_kind() {
        let console = Console::new();
        assert_eq!(console.counts(), (0, 0));
        console.push("clean-data", "clean.py", Kind::Printed, "cleaning 3 rows\n");
        console.push("clean-data", "panel", Kind::Log, "done");
        console.push("other", "run.py", Kind::Error, "The script failed: boom");
        let entries = console.entries();
        assert_eq!(entries.iter().map(|e| e.seq).collect::<Vec<_>>(), [1, 2, 3]);
        assert_eq!(
            entries.iter().map(|e| (e.project.as_str(), e.source.as_str(), e.kind, e.text.as_str())).collect::<Vec<_>>(),
            [
                ("clean-data", "clean.py", Kind::Printed, "cleaning 3 rows\n"),
                ("clean-data", "panel", Kind::Log, "done"),
                ("other", "run.py", Kind::Error, "The script failed: boom"),
            ]
        );
        assert!(entries.iter().all(|e| e.at_ms.is_some_and(|ms| ms > 1_600_000_000_000)), "each says when");
        // Another handle is the same console.
        let again = console.clone();
        again.push("other", "panel", Kind::Refused, "no");
        assert_eq!(console.counts(), (4, 4));
        // Nothing to say adds nothing.
        console.push("other", "panel", Kind::Log, "  \n");
        console.push("other", "panel", Kind::Log, "");
        assert_eq!(console.counts(), (4, 4));
        // Clearing forgets the entries, not the count: a place is never given to another entry.
        console.clear();
        assert_eq!(console.counts(), (4, 0));
        console.push("other", "panel", Kind::Log, "after");
        assert_eq!(console.entries()[0].seq, 5);
        for kind in [Kind::Printed, Kind::Error, Kind::Log, Kind::Refused] {
            assert!(!kind.code().is_empty() && !kind.label().is_empty());
        }
    }

    #[test]
    fn text_is_kept_readable_and_bounded() {
        let console = Console::new();
        // Control characters can't redraw the console; line breaks and tabs are text.
        console.push("m", "s.py", Kind::Printed, "a\x1b[31mred\x07\tb\r\nnext\u{0}");
        let text = console.entries()[0].text.clone();
        assert_eq!(text, "a\u{fffd}[31mred\u{fffd}\tb\nnext\u{fffd}");
        // A long text is cut, on a whole character, and says so.
        console.push("m", "s.py", Kind::Printed, &"é".repeat(MAX_TEXT));
        let long = console.entries()[1].text.clone();
        assert!(long.len() <= MAX_TEXT + 60 && long.ends_with("too long to keep)"), "{} bytes", long.len());
        assert!(long.starts_with("éééé"));
        // Names are one line, and short.
        console.push("pro\nject", &"s".repeat(500), Kind::Log, "x");
        let entry = console.entries()[2].clone();
        assert_eq!(entry.project, "project");
        assert_eq!(entry.source.len(), 100);
    }

    #[test]
    fn a_chatty_project_gives_up_its_own_entries_not_anothers() {
        let console = Console::new();
        console.push("quiet", "panel", Kind::Log, "the one thing I said");
        for n in 0..PER_PROJECT + 50 {
            console.push("chatty", "panel", Kind::Log, &format!("line {n}"));
        }
        let entries = console.entries();
        assert_eq!(entries.iter().filter(|e| e.project == "chatty").count(), PER_PROJECT);
        assert_eq!(entries[0].text, "the one thing I said", "the quiet project's entry is still there");
        assert_eq!(entries.last().unwrap().text, format!("line {}", PER_PROJECT + 49), "and the chatty one's latest");
        assert_eq!(entries[1].text, "line 50", "its oldest went first");
        // Over all projects the console is bounded too.
        for project in 0..10 {
            for n in 0..PER_PROJECT {
                console.push(&format!("m{project}"), "panel", Kind::Log, &format!("{n}"));
            }
        }
        assert_eq!(console.counts().1, MAX_ENTRIES);
    }

    #[test]
    fn many_threads_can_write_at_once() {
        let console = Console::new();
        let writers: Vec<_> = (0..8)
            .map(|t| {
                let console = console.clone();
                std::thread::spawn(move || {
                    for n in 0..20 {
                        console.push(&format!("m{t}"), "panel", Kind::Log, &format!("{n}"));
                    }
                })
            })
            .collect();
        for writer in writers {
            writer.join().unwrap();
        }
        let entries = console.entries();
        assert_eq!(entries.len(), 160);
        let mut seqs: Vec<u64> = entries.iter().map(|e| e.seq).collect();
        seqs.dedup();
        assert_eq!(seqs.len(), 160, "every entry has a place of its own");
        assert!(seqs.windows(2).all(|w| w[0] < w[1]), "and they are in order");
    }
}
