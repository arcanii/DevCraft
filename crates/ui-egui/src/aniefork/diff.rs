//! A line-by-line comparison of two texts: what a change to a file would take out and put in.
//! It is what the Development window shows when an agent asks to write over a file.

/// One line of a comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Line<'a> {
    /// In both texts: its number in the old one and in the new.
    Same(usize, usize, &'a str),
    /// Only in the old text: its number there.
    Gone(usize, &'a str),
    /// Only in the new text: its number there.
    New(usize, &'a str),
}

/// A line of what is shown: a line of the comparison, or a stretch of unchanged lines left out.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Row<'a> {
    Line(Line<'a>),
    /// So many lines that are the same in both.
    Skipped(usize),
}

/// The most cells of the table a comparison may fill. Texts whose differing middles are longer
/// than this allows are not compared line by line.
const MAX_CELLS: usize = 2_000_000;

/// Compares `old` and `new` line by line: every line of both, in order, marked as kept, taken
/// out or put in. `None` when they are too long, and too different, to compare.
pub fn lines<'a>(old: &'a str, new: &'a str) -> Option<Vec<Line<'a>>> {
    let a: Vec<&str> = old.lines().collect();
    let b: Vec<&str> = new.lines().collect();
    // What they begin and end with alike needs no comparing.
    let head = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let (a_rest, b_rest) = (a.get(head..).unwrap_or(&[]), b.get(head..).unwrap_or(&[]));
    let tail = a_rest.iter().rev().zip(b_rest.iter().rev()).take_while(|(x, y)| x == y).count();
    let (a_mid, b_mid) = (a_rest.get(..a_rest.len() - tail).unwrap_or(&[]), b_rest.get(..b_rest.len() - tail).unwrap_or(&[]));
    let (n, m) = (a_mid.len(), b_mid.len());
    let width = m + 1;
    if (n + 1).checked_mul(width).is_none_or(|cells| cells > MAX_CELLS) {
        return None;
    }
    // table[i][j]: how many lines the rest of the old middle from i and of the new from j share.
    let mut table = vec![0u32; (n + 1) * width];
    let cell = |table: &[u32], i: usize, j: usize| table.get(i * width + j).copied().unwrap_or(0);
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            let same = a_mid.get(i) == b_mid.get(j);
            let value = if same { cell(&table, i + 1, j + 1) + 1 } else { cell(&table, i + 1, j).max(cell(&table, i, j + 1)) };
            if let Some(slot) = table.get_mut(i * width + j) {
                *slot = value;
            }
        }
    }
    let mut out = Vec::with_capacity(a.len().max(b.len()));
    for (index, line) in a.iter().take(head).enumerate() {
        out.push(Line::Same(index + 1, index + 1, line));
    }
    let (mut i, mut j) = (0, 0);
    while i < n || j < m {
        match (a_mid.get(i), b_mid.get(j)) {
            (Some(x), Some(y)) if x == y => {
                out.push(Line::Same(head + i + 1, head + j + 1, x));
                i += 1;
                j += 1;
            }
            (Some(x), Some(_)) if cell(&table, i + 1, j) >= cell(&table, i, j + 1) => {
                out.push(Line::Gone(head + i + 1, x));
                i += 1;
            }
            (Some(x), None) => {
                out.push(Line::Gone(head + i + 1, x));
                i += 1;
            }
            (_, Some(y)) => {
                out.push(Line::New(head + j + 1, y));
                j += 1;
            }
            (None, None) => break,
        }
    }
    for k in 0..tail {
        if let Some(line) = a.get(head + n + k) {
            out.push(Line::Same(head + n + k + 1, head + m + k + 1, line));
        }
    }
    Some(out)
}

/// The lines of a comparison worth showing: every change, with `around` unchanged lines before
/// and after it, and a count in place of each longer stretch that is the same.
pub fn shown<'a>(lines: &[Line<'a>], around: usize) -> Vec<Row<'a>> {
    let changed = |line: &Line| !matches!(line, Line::Same(..));
    // For each line: is a change within `around` lines of it?
    let mut near = vec![false; lines.len()];
    for (index, line) in lines.iter().enumerate() {
        if changed(line) {
            let (from, to) = (index.saturating_sub(around), (index + around).min(lines.len().saturating_sub(1)));
            for slot in near.iter_mut().take(to + 1).skip(from) {
                *slot = true;
            }
        }
    }
    let mut rows = Vec::new();
    let mut skipped = 0;
    for (line, near) in lines.iter().zip(near) {
        if near {
            if skipped > 0 {
                rows.push(Row::Skipped(skipped));
                skipped = 0;
            }
            rows.push(Row::Line(*line));
        } else {
            skipped += 1;
        }
    }
    if skipped > 0 {
        rows.push(Row::Skipped(skipped));
    }
    rows
}

/// How many lines a comparison takes out and puts in.
pub fn counts(lines: &[Line]) -> (usize, usize) {
    let gone = lines.iter().filter(|line| matches!(line, Line::Gone(..))).count();
    let new = lines.iter().filter(|line| matches!(line, Line::New(..))).count();
    (gone, new)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A comparison as text: one line each, marked ` `, `-` or `+`.
    fn told(old: &str, new: &str) -> String {
        let mark = |line: &Line| match line {
            Line::Same(_, _, text) => format!(" {text}"),
            Line::Gone(_, text) => format!("-{text}"),
            Line::New(_, text) => format!("+{text}"),
        };
        lines(old, new).unwrap().iter().map(mark).collect::<Vec<_>>().join("\n")
    }

    #[test]
    fn a_comparison_keeps_what_is_the_same_and_marks_what_changed() {
        assert_eq!(told("a\nb\nc\n", "a\nb\nc\n"), " a\n b\n c");
        assert_eq!(told("a\nb\nc\n", "a\nB\nc\n"), " a\n-b\n+B\n c");
        assert_eq!(told("a\nb\nc", "a\nc"), " a\n-b\n c");
        assert_eq!(told("a\nc", "a\nb\nc"), " a\n+b\n c");
        assert_eq!(told("", "a\nb"), "+a\n+b");
        assert_eq!(told("a\nb", ""), "-a\n-b");
        assert_eq!(told("", ""), "");
        // Lines that moved: the longest run that stayed in order is kept.
        assert_eq!(told("1\n2\n3\n4", "3\n4\n1\n2"), "-1\n-2\n 3\n 4\n+1\n+2");
        assert_eq!(told("x\na\nb\ny", "p\na\nq\nb\nr"), "-x\n+p\n a\n+q\n b\n-y\n+r");
    }

    #[test]
    fn every_line_of_both_texts_is_in_the_comparison_with_its_number() {
        let old = "one\ntwo\nthree\nfour\nfive";
        let new = "one\n2\nthree\nfour\nfive\nsix";
        let compared = lines(old, new).unwrap();
        assert_eq!(
            compared,
            [
                Line::Same(1, 1, "one"),
                Line::Gone(2, "two"),
                Line::New(2, "2"),
                Line::Same(3, 3, "three"),
                Line::Same(4, 4, "four"),
                Line::Same(5, 5, "five"),
                Line::New(6, "six"),
            ]
        );
        assert_eq!(counts(&compared), (1, 2));
        // Taking the lines that aren't new gives back the old text, and the other way round.
        let olds: Vec<&str> = compared
            .iter()
            .filter_map(|line| match line {
                Line::Same(_, _, text) | Line::Gone(_, text) => Some(*text),
                Line::New(..) => None,
            })
            .collect();
        let news: Vec<&str> = compared
            .iter()
            .filter_map(|line| match line {
                Line::Same(_, _, text) | Line::New(_, text) => Some(*text),
                Line::Gone(..) => None,
            })
            .collect();
        assert_eq!((olds.join("\n"), news.join("\n")), (old.to_string(), new.to_string()));
    }

    #[test]
    fn nothing_between_two_texts_is_lost_whatever_they_are() {
        // A small generator: texts of a few different lines, every pair compared.
        let texts: Vec<String> = (0..60u32)
            .map(|seed| {
                let mut state = seed.wrapping_mul(2_654_435_761).wrapping_add(12_345);
                let mut next = || {
                    state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                    (state >> 16) % 5
                };
                let count = next() + next();
                (0..count).map(|_| format!("line {}", next())).collect::<Vec<_>>().join("\n")
            })
            .collect();
        for old in &texts {
            for new in &texts {
                let compared = lines(old, new).unwrap();
                let side = |keep_old: bool| -> Vec<&str> {
                    compared
                        .iter()
                        .filter_map(|line| match line {
                            Line::Same(_, _, text) => Some(*text),
                            Line::Gone(_, text) => keep_old.then_some(*text),
                            Line::New(_, text) => (!keep_old).then_some(*text),
                        })
                        .collect()
                };
                assert_eq!(side(true), old.lines().collect::<Vec<_>>(), "{old:?} -> {new:?}");
                assert_eq!(side(false), new.lines().collect::<Vec<_>>(), "{old:?} -> {new:?}");
            }
        }
    }

    #[test]
    fn long_texts_that_differ_little_are_compared_and_ones_that_differ_wholly_are_not() {
        let long: String = (0..50_000).map(|n| format!("row {n}\n")).collect();
        let edited = long.replacen("row 25000\n", "row twenty-five thousand\n", 1);
        let compared = lines(&long, &edited).unwrap();
        assert_eq!(counts(&compared), (1, 1));
        assert_eq!(compared.len(), 50_001);
        // What is shown of it is the change and a little around it.
        let rows = shown(&compared, 3);
        assert_eq!(rows.len(), 3 + 2 + 3 + 2);
        assert_eq!(rows.first(), Some(&Row::Skipped(24_997)));
        assert_eq!(rows.last(), Some(&Row::Skipped(24_996)));
        assert!(
            rows.contains(&Row::Line(Line::Gone(25_001, "row 25000"))) && rows.contains(&Row::Line(Line::New(25_001, "row twenty-five thousand")))
        );
        // Two long texts with nothing in common are too much to compare: the caller shows the new one whole.
        let other: String = (0..5_000).map(|n| format!("other {n}\n")).collect();
        assert_eq!(lines(&long, &other), None);
    }

    #[test]
    fn what_is_shown_is_each_change_with_a_little_around_it() {
        let old: String = (1..=20).map(|n| format!("{n}\n")).collect();
        let new = old.replacen("5\n", "five\n", 1).replacen("15\n", "", 1);
        let compared = lines(&old, &new).unwrap();
        let rows = shown(&compared, 1);
        let told: Vec<String> = rows
            .iter()
            .map(|row| match row {
                Row::Skipped(count) => format!("…{count}"),
                Row::Line(Line::Same(_, _, text)) => format!(" {text}"),
                Row::Line(Line::Gone(_, text)) => format!("-{text}"),
                Row::Line(Line::New(_, text)) => format!("+{text}"),
            })
            .collect();
        assert_eq!(told, ["…3", " 4", "-5", "+five", " 6", "…7", " 14", "-15", " 16", "…4"]);
        // Nothing changed: nothing to show but how much is the same.
        assert_eq!(shown(&lines("a\nb", "a\nb").unwrap(), 3), [Row::Skipped(2)]);
        assert_eq!(shown(&[], 3), []);
        // With no room around, only the changes.
        assert_eq!(shown(&lines("a\nb\nc", "a\nB\nc").unwrap(), 0).len(), 4);
    }
}
