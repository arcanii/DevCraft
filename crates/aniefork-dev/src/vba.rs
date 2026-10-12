//! Looking inside a workbook's VBA project (`xl/vbaProject.bin`): its modules and their source,
//! to be read. **GridCraft never runs macros**, and nothing here changes a project: the workbook
//! keeps it byte for byte.
//!
//! This is the first step of bringing what a workbook's macros do into GridCraft (ADR-0001 in
//! `docs/decisions-aniefork.md`): before anything can be rebuilt as a project, a person and an
//! agent have to be able to read what is there.
//!
//! A project is a compound file (the container Office documents used before they were zip
//! packages). In it, the `VBA/dir` stream lists the modules, and each module has a stream that
//! holds its source, compressed, after a compiled form GridCraft has no use for. The format is
//! Microsoft's published \[MS-OVBA\]; this follows that document.
//!
//! **Everything in a project is hostile until read.** It comes from a file anyone may have
//! made. So: the project, each stream and what each stream unpacks to all have a most they
//! may be (the compression can make a few bytes into thousands, so what is unpacked is counted
//! as it is unpacked, and stops); no count or offset from the file is trusted; the container
//! is read by another crate (`cfb`), and a fault in it is caught rather than left to take the
//! app down. A module that can't be read is listed with why, and the rest are still shown.

use serde_json::{Value as Json, json};

/// The largest project looked inside (bytes).
pub const MAX_PROJECT_BYTES: usize = 32 * 1024 * 1024;
/// The largest stream of a project that is read (bytes, as stored).
pub const MAX_STREAM_BYTES: u64 = 8 * 1024 * 1024;
/// The most a module's source may unpack to (bytes). What is longer is cut, and says so.
pub const MAX_SOURCE_BYTES: usize = 2 * 1024 * 1024;
/// The most all the modules' source together may unpack to (bytes).
pub const MAX_TOTAL_BYTES: usize = 16 * 1024 * 1024;
/// The most the list of modules may unpack to (bytes).
#[cfg(not(target_arch = "wasm32"))]
const MAX_DIR_BYTES: usize = 4 * 1024 * 1024;
/// The most modules listed.
pub const MAX_MODULES: usize = 500;
/// The longest name kept (characters).
const MAX_NAME: usize = 255;

/// What kind of module it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A plain module of procedures.
    Standard,
    /// A class module.
    Class,
    /// The code behind the workbook or one of its sheets.
    Document,
    /// The code behind a form.
    Form,
    /// The code behind some object; the project doesn't say which kind.
    Object,
}

impl Kind {
    pub fn code(self) -> &'static str {
        match self {
            Kind::Standard => "module",
            Kind::Class => "class",
            Kind::Document => "document",
            Kind::Form => "form",
            Kind::Object => "object",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Kind::Standard => "Module",
            Kind::Class => "Class module",
            Kind::Document => "Workbook or sheet",
            Kind::Form => "Form",
            Kind::Object => "Object module",
        }
    }
}

/// One module of a project.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub kind: Kind,
    /// Its source, as text. Empty when it could not be read.
    pub source: String,
    /// The source is longer than GridCraft reads: this is its beginning.
    pub cut: bool,
    /// Why it could not be read.
    pub problem: Option<String>,
}

/// A VBA project, as far as it could be read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    /// The code page its text is stored in.
    pub code_page: u16,
    pub modules: Vec<Module>,
    /// The libraries it refers to, by name.
    pub references: Vec<String>,
}

/// A procedure found in a module's source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Procedure {
    /// `Sub`, `Function`, `Property Get`, `Property Let`, `Property Set`, or `Declare` for one
    /// that lives in a library outside the workbook.
    pub what: &'static str,
    pub name: String,
    /// The line it begins on, from 1.
    pub line: usize,
    /// How many lines it spans.
    pub lines: usize,
    /// Excel would run it by itself: it answers an event, or has a name Excel looks for.
    pub event: bool,
    /// For a `Declare`: the library it names.
    pub library: Option<String>,
}

/// Names Excel runs by itself when they are in a plain module.
const AUTO: [&str; 6] = ["auto_open", "auto_close", "auto_activate", "auto_deactivate", "autoexec", "autoopen"];

/// The procedures of a module, found by reading its lines. This is not a parser of the
/// language: it finds where procedures begin and end, which is enough to list them.
pub fn outline(source: &str, kind: Kind) -> Vec<Procedure> {
    let mut found: Vec<Procedure> = Vec::new();
    let mut open: Option<usize> = None;
    let mut continued = false;
    for (index, line) in source.lines().enumerate() {
        // A line that continues the one before is not the start of anything.
        let continues = std::mem::replace(&mut continued, line.trim_end().ends_with(" _"));
        if continues {
            continue;
        }
        let text = line.trim_start();
        // The first words of the line, as they are written and without their capitals.
        let words: Vec<&str> = text.split_whitespace().take(8).collect();
        let lower: Vec<String> = words.iter().map(|word| word.to_ascii_lowercase()).collect();
        let word = |at: usize| lower.get(at).map_or("", String::as_str);
        if text.starts_with('\'') || word(0) == "rem" {
            continue;
        }
        if word(0) == "end" && matches!(word(1), "sub" | "function" | "property") {
            if let Some(procedure) = open.take().and_then(|at| found.get_mut(at)) {
                procedure.lines = (index + 1).saturating_sub(procedure.line).saturating_add(1);
            }
            continue;
        }
        let mut at = 0;
        while matches!(word(at), "public" | "private" | "friend" | "static" | "global") {
            at += 1;
        }
        let declared = word(at) == "declare";
        if declared {
            at += if word(at + 1) == "ptrsafe" { 2 } else { 1 };
        }
        let what = match (word(at), declared) {
            ("sub" | "function", true) => "Declare",
            ("sub", false) => "Sub",
            ("function", false) => "Function",
            ("property", false) => {
                at += 1;
                match word(at) {
                    "get" => "Property Get",
                    "let" => "Property Let",
                    "set" => "Property Set",
                    _ => continue,
                }
            }
            _ => continue,
        };
        // Its name: the next word, as far as a name goes.
        let Some(named) = words.get(at + 1) else { continue };
        let name: String = named.chars().take_while(|c| c.is_alphanumeric() || *c == '_').take(MAX_NAME).collect();
        if name.is_empty() {
            continue;
        }
        // For one that lives in a library: the library, as the line names it.
        let library = declared.then(|| {
            let from = text.to_ascii_lowercase().find("lib \"").map(|found| found + 5);
            from.and_then(|from| text.get(from..)).and_then(|rest| rest.split('"').next()).unwrap_or("").to_string()
        });
        let lowered = name.to_ascii_lowercase();
        let event = what == "Sub" && (AUTO.contains(&lowered.as_str()) || (kind != Kind::Standard && lowered.contains('_')));
        found.push(Procedure { what, name, line: index + 1, lines: 1, event, library });
        if !declared {
            open = Some(found.len() - 1);
        }
    }
    found
}

/// A project as `developer.macros` reports it and an agent is told of it: its modules, what
/// kind each is, and the procedures in each. Not their source.
pub fn summary(project: &Project) -> Json {
    let modules: Vec<Json> = project
        .modules
        .iter()
        .map(|module| {
            let procedures: Vec<Json> = outline(&module.source, module.kind)
                .iter()
                .map(|p| json!({"what": p.what, "name": p.name, "line": p.line, "lines": p.lines, "runsByItselfInExcel": p.event, "library": p.library}))
                .collect();
            json!({
                "name": module.name,
                "kind": module.kind.code(),
                "lines": module.source.lines().count(),
                "cut": module.cut,
                "problem": module.problem,
                "procedures": procedures,
            })
        })
        .collect();
    json!({"name": project.name, "codePage": project.code_page, "references": project.references, "modules": modules})
}

/// Unpacks a compressed container (\[MS-OVBA\] 2.4.1), up to `most` bytes: what it unpacks to,
/// and whether there was more.
///
/// A container is a byte 1, then chunks. A chunk has a header (how long it is, and whether it
/// is packed) and then either plain bytes, or groups of eight tokens after a byte of flags: a
/// token is one plain byte, or two bytes that say "copy this many bytes from that far back in
/// this chunk". How the two bytes divide between "how many" and "how far" depends on how much
/// of the chunk has been unpacked.
pub fn decompress(data: &[u8], most: usize) -> Result<(Vec<u8>, bool), String> {
    let damaged = || "it is not packed as a VBA project packs things".to_string();
    let (first, mut rest) = data.split_first().ok_or_else(damaged)?;
    if *first != 1 {
        return Err(damaged());
    }
    let mut out: Vec<u8> = Vec::new();
    while let (Some(low), Some(high)) = (rest.first(), rest.get(1)) {
        let header = u16::from_le_bytes([*low, *high]);
        // A chunk is as long as its header says, or as much as is left of a file cut short.
        let length = (usize::from(header & 0x0FFF) + 3).min(rest.len());
        let chunk = rest.get(2..length).unwrap_or(&[]);
        rest = rest.get(length..).unwrap_or(&[]);
        let start = out.len();
        if header & 0x8000 == 0 {
            // Plain bytes.
            out.extend_from_slice(chunk.get(..chunk.len().min(4096)).unwrap_or(&[]));
        } else {
            let mut at = 0;
            while let Some(flags) = chunk.get(at).copied() {
                at += 1;
                for bit in 0..8 {
                    let Some(byte) = chunk.get(at).copied() else { break };
                    if flags >> bit & 1 == 0 {
                        out.push(byte);
                        at += 1;
                    } else {
                        let Some(second) = chunk.get(at + 1).copied() else {
                            at = chunk.len();
                            break;
                        };
                        at += 2;
                        let token = u16::from_le_bytes([byte, second]);
                        let done = out.len() - start;
                        // How many of the token's bits say "how far back": the more of the
                        // chunk there is to reach back into, the more of them.
                        let mut bits = 4u32;
                        while (1usize << bits) < done && bits < 12 {
                            bits += 1;
                        }
                        let length_mask = 0xFFFFu16 >> bits;
                        let length = usize::from(token & length_mask) + 3;
                        let back = usize::from((token & !length_mask) >> (16 - bits)) + 1;
                        if back > done {
                            return Err(damaged());
                        }
                        for _ in 0..length {
                            let Some(copied) = out.get(out.len() - back).copied() else { return Err(damaged()) };
                            out.push(copied);
                        }
                    }
                    // No more than a chunk may hold, and no more than is wanted.
                    if out.len() - start > 4096 {
                        return Err(damaged());
                    }
                    if out.len() >= most {
                        let more = out.len() > most || at < chunk.len() || !rest.is_empty();
                        out.truncate(most);
                        return Ok((out, more));
                    }
                }
            }
        }
        if out.len() >= most {
            let more = out.len() > most || !rest.is_empty();
            out.truncate(most);
            return Ok((out, more));
        }
    }
    Ok((out, false))
}

/// Text stored in a Windows code page, as text. What the code page has no character for is
/// shown as the mark for that.
#[cfg(not(target_arch = "wasm32"))]
fn decode(bytes: &[u8], code_page: u16) -> String {
    let label = match code_page {
        874 => "windows-874",
        932 => "shift_jis",
        936 => "gbk",
        949 => "euc-kr",
        950 => "big5",
        1250 => "windows-1250",
        1251 => "windows-1251",
        1253 => "windows-1253",
        1254 => "windows-1254",
        1255 => "windows-1255",
        1256 => "windows-1256",
        1257 => "windows-1257",
        1258 => "windows-1258",
        10000 => "macintosh",
        65001 => "utf-8",
        // 1252, and anything unknown: the code page of most projects.
        _ => "windows-1252",
    };
    let encoding = encoding_rs::Encoding::for_label(label.as_bytes()).unwrap_or(encoding_rs::WINDOWS_1252);
    encoding.decode_without_bom_handling(bytes).0.into_owned()
}

/// Text stored as UTF-16, as text.
#[cfg(not(target_arch = "wasm32"))]
fn utf16(bytes: &[u8]) -> String {
    let units: Vec<u16> = bytes.as_chunks::<2>().0.iter().map(|pair| u16::from_le_bytes(*pair)).collect();
    String::from_utf16_lossy(&units)
}

/// A name from a project, kept to what can be shown.
#[cfg(not(target_arch = "wasm32"))]
fn name(text: &str) -> String {
    text.chars().filter(|c| crate::agents::shows(*c)).take(MAX_NAME).collect()
}

/// A module as the list of modules names it.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Listed {
    name: String,
    /// The name of its stream in the `VBA` storage.
    stream: String,
    /// Where in that stream its source begins.
    offset: usize,
    /// The list says it is the code behind an object, not a plain module.
    object: bool,
}

/// What the list of modules (`VBA/dir`, unpacked) says: the code page, the project's name, the
/// libraries it refers to and its modules.
#[cfg(not(target_arch = "wasm32"))]
fn listing(dir: &[u8]) -> (u16, String, Vec<String>, Vec<Listed>) {
    let (mut code_page, mut project, mut references, mut modules) = (1252u16, String::new(), Vec::new(), Vec::new());
    let mut module: Option<Listed> = None;
    let mut at = 0usize;
    // A record is a kind (two bytes), a length (four) and that many bytes.
    while let (Some(kind), Some(length)) = (dir.get(at..at + 2), dir.get(at + 2..at + 6)) {
        let kind = u16::from_le_bytes([kind.first().copied().unwrap_or(0), kind.get(1).copied().unwrap_or(0)]);
        let length = u32::from_le_bytes([
            length.first().copied().unwrap_or(0),
            length.get(1).copied().unwrap_or(0),
            length.get(2).copied().unwrap_or(0),
            length.get(3).copied().unwrap_or(0),
        ]) as usize;
        // The record of the project's version says four where it holds six.
        let length = if kind == 0x0009 { 6 } else { length };
        let Some(end) = (at + 6).checked_add(length) else { break };
        let Some(data) = dir.get(at + 6..end) else { break };
        at = end;
        match kind {
            0x0003 => code_page = data.first().zip(data.get(1)).map_or(code_page, |(low, high)| u16::from_le_bytes([*low, *high])),
            0x0004 => project = name(&decode(data, code_page)),
            // A library's name, then the same in UTF-16.
            0x0016 if module.is_none() && references.len() < 64 => references.push(name(&decode(data, code_page))),
            0x0019 => module = Some(Listed { name: name(&decode(data, code_page)), ..Listed::default() }),
            0x0047 if !data.is_empty() => {
                if let Some(module) = &mut module {
                    module.name = name(&utf16(data));
                }
            }
            0x001A => {
                if let Some(module) = &mut module {
                    module.stream = decode(data, code_page);
                }
            }
            0x0032 if !data.is_empty() => {
                if let Some(module) = &mut module {
                    module.stream = utf16(data);
                }
            }
            0x0031 => {
                if let (Some(module), Some(offset)) = (&mut module, data.get(..4)) {
                    module.offset = u32::from_le_bytes([
                        offset.first().copied().unwrap_or(0),
                        offset.get(1).copied().unwrap_or(0),
                        offset.get(2).copied().unwrap_or(0),
                        offset.get(3).copied().unwrap_or(0),
                    ]) as usize;
                }
            }
            0x0022 => {
                if let Some(module) = &mut module {
                    module.object = true;
                }
            }
            // The end of a module's records.
            0x002B => {
                if let Some(module) = module.take()
                    && modules.len() < MAX_MODULES
                {
                    modules.push(module);
                }
            }
            // The end of the list.
            0x0010 => break,
            _ => {}
        }
    }
    (code_page, project, references, modules)
}

/// What the `PROJECT` stream says each module is: lines such as `Module=Module1`,
/// `Class=Class1`, `Document=Sheet1/&H00000000`, `BaseClass=UserForm1`.
#[cfg(not(target_arch = "wasm32"))]
fn kinds(text: &str) -> Vec<(String, Kind)> {
    let mut kinds = Vec::new();
    for line in text.lines().take(4 * MAX_MODULES) {
        let Some((key, value)) = line.split_once('=') else { continue };
        let kind = match key.trim() {
            "Module" => Kind::Standard,
            "Class" => Kind::Class,
            "Document" => Kind::Document,
            "BaseClass" => Kind::Form,
            _ => continue,
        };
        let named = value.split('/').next().unwrap_or("").trim();
        kinds.push((named.to_ascii_lowercase(), kind));
    }
    kinds
}

/// Reads a VBA project: its modules and their source. An error when nothing of it could be
/// read; a module that could not be read is listed with why.
#[cfg(not(target_arch = "wasm32"))]
pub fn read(bytes: &[u8]) -> Result<Project, String> {
    if bytes.len() > MAX_PROJECT_BYTES {
        return Err(format!(
            "The VBA project is {} MB, more than the {} MB GridCraft looks inside.",
            bytes.len() / (1024 * 1024),
            MAX_PROJECT_BYTES / (1024 * 1024)
        ));
    }
    // The container is read by another crate. A project built to break it must not take
    // GridCraft down with it.
    match std::panic::catch_unwind(|| read_checked(bytes)) {
        Ok(read) => read,
        Err(_) => Err("The VBA project could not be read: the file that holds it is damaged.".into()),
    }
}

/// The web build looks inside nothing.
#[cfg(target_arch = "wasm32")]
pub fn read(_bytes: &[u8]) -> Result<Project, String> {
    Err("This copy of GridCraft can't look inside a VBA project.".into())
}

#[cfg(not(target_arch = "wasm32"))]
fn read_checked(bytes: &[u8]) -> Result<Project, String> {
    use std::io::Read;

    let mut file = cfb::CompoundFile::open(std::io::Cursor::new(bytes)).map_err(|e| format!("The VBA project could not be read: {e}"))?;
    // One stream of the file, when it is there and not too long.
    let mut stream = |path: &str| -> Result<Option<Vec<u8>>, String> {
        if !file.is_stream(path) {
            return Ok(None);
        }
        let mut found = file.open_stream(path).map_err(|e| e.to_string())?;
        if found.len() > MAX_STREAM_BYTES {
            return Err(format!("it is {} MB, more than GridCraft reads", found.len() / (1024 * 1024)));
        }
        let mut held = Vec::new();
        (&mut found).take(MAX_STREAM_BYTES).read_to_end(&mut held).map_err(|e| e.to_string())?;
        Ok(Some(held))
    };
    let dir = stream("/VBA/dir").map_err(|why| format!("The VBA project's list of modules could not be read: {why}."))?;
    let dir = dir.ok_or("This is not a VBA project GridCraft can read: it has no list of modules.")?;
    let (dir, _) = decompress(&dir, MAX_DIR_BYTES).map_err(|why| format!("The VBA project's list of modules could not be read: {why}."))?;
    let (code_page, project, references, listed) = listing(&dir);
    let said = stream("/PROJECT").ok().flatten().map(|text| kinds(&decode(&text, code_page))).unwrap_or_default();
    let mut modules = Vec::with_capacity(listed.len());
    let mut left = MAX_TOTAL_BYTES;
    for listed in listed {
        let kind = match said.iter().find(|(named, _)| *named == listed.name.to_ascii_lowercase()) {
            Some((_, kind)) => *kind,
            None if listed.object => Kind::Object,
            None => Kind::Standard,
        };
        let mut module = Module { name: listed.name.clone(), kind, source: String::new(), cut: false, problem: None };
        // A name that would be a path is not a stream of the `VBA` storage.
        let plain = !listed.stream.is_empty() && !listed.stream.contains(['/', '\\']);
        let unpacked = match stream(&format!("/VBA/{}", listed.stream)) {
            _ if !plain => Err("the project names no stream for it".to_string()),
            _ if left == 0 => Err("the project's modules are longer together than GridCraft reads".to_string()),
            Ok(Some(held)) => decompress(held.get(listed.offset..).unwrap_or(&[]), MAX_SOURCE_BYTES.min(left)),
            Ok(None) => Err("its source is not in the project".to_string()),
            Err(why) => Err(why),
        };
        match unpacked {
            Ok((source, cut)) => {
                left = left.saturating_sub(source.len());
                module.source = decode(&source, code_page);
                module.cut = cut;
            }
            Err(why) => module.problem = Some(format!("Its source could not be read: {why}.")),
        }
        modules.push(module);
    }
    Ok(Project { name: project, code_page, modules, references })
}

/// Building a VBA project, for tests here and in the crates that use this one. It writes what
/// [`read`] reads, not everything Excel would want of a project.
#[cfg(not(target_arch = "wasm32"))]
pub mod testing {
    use std::io::Write;

    use super::Kind;

    /// Packs bytes as a compressed container whose tokens are all plain bytes.
    pub fn pack(data: &[u8]) -> Vec<u8> {
        let mut out = vec![1u8];
        // A chunk holds a header and at most 4096 bytes: 3640 plain bytes with their flags.
        for part in data.chunks(3640) {
            let mut chunk = Vec::new();
            for group in part.chunks(8) {
                chunk.push(0);
                chunk.extend_from_slice(group);
            }
            let header = 0xB000u16 | (chunk.len() as u16 + 2 - 3);
            out.extend_from_slice(&header.to_le_bytes());
            out.extend_from_slice(&chunk);
        }
        out
    }

    fn record(out: &mut Vec<u8>, kind: u16, data: &[u8]) {
        out.extend_from_slice(&kind.to_le_bytes());
        out.extend_from_slice(&(data.len() as u32).to_le_bytes());
        out.extend_from_slice(data);
    }

    fn wide(text: &str) -> Vec<u8> {
        text.encode_utf16().flat_map(u16::to_le_bytes).collect()
    }

    /// The list of modules, as `VBA/dir` holds it before it is packed.
    pub fn listing(project: &str, modules: &[(&str, Kind, &str)]) -> Vec<u8> {
        let mut dir = Vec::new();
        record(&mut dir, 0x0001, &1u32.to_le_bytes());
        record(&mut dir, 0x0003, &1252u16.to_le_bytes());
        record(&mut dir, 0x0004, project.as_bytes());
        // The version: it says four bytes and holds six.
        dir.extend_from_slice(&[0x09, 0x00, 0x04, 0x00, 0x00, 0x00, 1, 0, 0, 0, 2, 0]);
        record(&mut dir, 0x0016, b"stdole");
        record(&mut dir, 0x003E, &wide("stdole"));
        record(&mut dir, 0x000F, &(modules.len() as u16).to_le_bytes());
        record(&mut dir, 0x0013, &0xFFFFu16.to_le_bytes());
        for (name, kind, _) in modules {
            record(&mut dir, 0x0019, name.as_bytes());
            record(&mut dir, 0x0047, &wide(name));
            record(&mut dir, 0x001A, name.as_bytes());
            record(&mut dir, 0x0032, &wide(name));
            record(&mut dir, 0x0031, &4u32.to_le_bytes());
            record(&mut dir, if *kind == Kind::Standard { 0x0021 } else { 0x0022 }, &[]);
            record(&mut dir, 0x002B, &[]);
        }
        record(&mut dir, 0x0010, &[]);
        dir
    }

    /// A VBA project with these modules (name, kind, source), as `xl/vbaProject.bin` would hold
    /// it. The source is stored in code page 1252.
    pub fn project(name: &str, modules: &[(&str, Kind, &str)]) -> Vec<u8> {
        let build = || -> std::io::Result<Vec<u8>> {
            let mut file = cfb::CompoundFile::create(std::io::Cursor::new(Vec::new()))?;
            file.create_storage("/VBA")?;
            file.create_stream("/VBA/dir")?.write_all(&pack(&listing(name, modules)))?;
            let mut said = String::from("ID=\"{00000000-0000-0000-0000-000000000000}\"\r\n");
            for (module, kind, source) in modules {
                let key = match kind {
                    Kind::Standard => "Module",
                    Kind::Class => "Class",
                    Kind::Form => "BaseClass",
                    Kind::Document | Kind::Object => "Document",
                };
                let tail = if matches!(kind, Kind::Document | Kind::Object) { "/&H00000000" } else { "" };
                said.push_str(&format!("{key}={module}{tail}\r\n"));
                // Four bytes of what Excel compiled, which nobody reads here, then the source.
                let mut stream = vec![0xAA, 0xBB, 0xCC, 0xDD];
                let stored: Vec<u8> = source.chars().map(|c| if (c as u32) < 256 { c as u8 } else { b'?' }).collect();
                stream.extend_from_slice(&pack(&stored));
                file.create_stream(format!("/VBA/{module}"))?.write_all(&stream)?;
            }
            file.create_stream("/PROJECT")?.write_all(said.as_bytes())?;
            file.flush()?;
            Ok(file.into_inner().into_inner())
        };
        build().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELLO: &str = "Attribute VB_Name = \"Module1\"\r\nOption Explicit\r\n\r\nPublic Sub Hello()\r\n    MsgBox \"Hello\"\r\nEnd Sub\r\n\r\nPrivate Function Twice(ByVal n As Long) As Long\r\n    Twice = n * 2\r\nEnd Function\r\n";
    const BOOK: &str = "Private Sub Workbook_Open()\r\n    Call Hello\r\nEnd Sub\r\n";

    #[test]
    fn what_was_packed_is_unpacked_as_the_format_says() {
        // The three examples the format's own document gives.
        let plain = [
            0x01, 0x19, 0xB0, 0x00, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68, 0x00, 0x69, 0x6A, 0x6B, 0x6C, 0x6D, 0x6E, 0x6F, 0x70, 0x00, 0x71,
            0x72, 0x73, 0x74, 0x75, 0x76, 0x2E,
        ];
        assert_eq!(decompress(&plain, 1000).unwrap(), (b"abcdefghijklmnopqrstuv.".to_vec(), false));
        let copies = [
            0x01, 0x2F, 0xB0, 0x00, 0x23, 0x61, 0x61, 0x61, 0x62, 0x63, 0x64, 0x65, 0x82, 0x66, 0x00, 0x70, 0x61, 0x67, 0x68, 0x69, 0x6A, 0x01, 0x38,
            0x08, 0x61, 0x6B, 0x6C, 0x00, 0x30, 0x6D, 0x6E, 0x6F, 0x70, 0x06, 0x71, 0x02, 0x70, 0x04, 0x10, 0x72, 0x73, 0x74, 0x75, 0x76, 0x10, 0x77,
            0x78, 0x79, 0x7A, 0x00, 0x3C,
        ];
        assert_eq!(decompress(&copies, 1000).unwrap(), (b"#aaabcdefaaaaghijaaaaaklaaamnopqaaaaaaaaaaaarstuvwxyzaaa".to_vec(), false));
        let run = [0x01, 0x03, 0xB0, 0x02, 0x61, 0x45, 0x00];
        assert_eq!(decompress(&run, 1000).unwrap(), (vec![b'a'; 73], false));
        // A chunk that is not packed: its bytes as they are.
        let mut stored = vec![0x01, 0xFF, 0x3F];
        stored.extend(std::iter::repeat_n(b'z', 4096));
        assert_eq!(decompress(&stored, 10_000).unwrap(), (vec![b'z'; 4096], false));
        // What the test builder packs comes back, whatever its length.
        for length in [0, 1, 7, 8, 9, 3639, 3640, 3641, 10_000] {
            let data: Vec<u8> = (0..length).map(|n| (n % 251) as u8).collect();
            assert_eq!(decompress(&testing::pack(&data), 1 << 20).unwrap(), (data, false), "{length}");
        }
    }

    #[test]
    fn a_few_bytes_built_to_unpack_to_a_great_many_are_stopped() {
        // Each chunk is six bytes and unpacks to 4096: a megabyte of them would be 700 MB.
        let mut bomb = vec![0x01];
        for _ in 0..200_000 {
            bomb.extend_from_slice(&[0x03, 0xB0, 0x02, 0x61, 0xFC, 0x0F]);
        }
        let (out, cut) = decompress(&bomb, 100_000).unwrap();
        assert_eq!((out.len(), cut), (100_000, true), "it stops at what was asked for");
        assert!(out.iter().all(|byte| *byte == b'a'));
        // A whole module is cut at the most a module may be, and says so.
        let long = "x = 1\r\n".repeat(MAX_SOURCE_BYTES / 7 + 10);
        let project =
            read(&testing::project("Long", &[("Big", Kind::Standard, &long), ("After", Kind::Standard, "Sub A()\r\nEnd Sub\r\n")])).unwrap();
        assert!(project.modules[0].cut && project.modules[0].source.len() == MAX_SOURCE_BYTES);
        assert!(!project.modules[1].cut && project.modules[1].source.starts_with("Sub A()"), "and the next one is still read");
    }

    #[test]
    fn what_is_damaged_is_an_error_and_never_more() {
        for bad in [&[][..], &[0x00], &[0x02, 0x03, 0xB0], &[0x01, 0x03, 0xB0, 0x02, 0x61, 0x45], &[0x01, 0x03, 0xB0, 0x01, 0x45, 0x00]] {
            let _ = decompress(bad, 1000);
        }
        // A copy from before the start of its chunk.
        assert!(decompress(&[0x01, 0x03, 0xB0, 0x01, 0x45, 0x00, 0x61], 1000).is_err());
        assert!(decompress(&[0x00, 0x01], 1000).is_err(), "not a container at all");
        // Not a compound file; one with nothing in it; one cut short at every length.
        assert!(read(b"not a compound file").unwrap_err().contains("could not be read"));
        assert!(read(&[]).is_err());
        let whole = testing::project("P", &[("Module1", Kind::Standard, HELLO)]);
        for length in (0..whole.len()).step_by(97) {
            let _ = read(whole.get(..length).unwrap());
        }
        // One with every byte in turn set to something else, a few hundred of them.
        for at in (0..whole.len()).step_by((whole.len() / 300).max(1)) {
            let mut bent = whole.clone();
            bent[at] ^= 0xA5;
            let _ = read(&bent);
        }
        assert!(read(&vec![0u8; MAX_PROJECT_BYTES + 1]).unwrap_err().contains("more than the 32 MB"));
        // A list of modules that is garbage lists nothing, and reads on to its end.
        let (code_page, name, references, modules) = listing(&[0xFF; 1000]);
        assert_eq!((code_page, name.as_str(), references.len(), modules.len()), (1252, "", 0, 0));
        let _ = listing(&[0x19, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x41]);
    }

    #[test]
    fn a_project_is_read_as_its_modules_and_their_source() {
        let bytes = testing::project(
            "Sales",
            &[("Module1", Kind::Standard, HELLO), ("ThisWorkbook", Kind::Document, BOOK), ("Ledger", Kind::Class, ""), ("Entry", Kind::Form, "")],
        );
        let project = read(&bytes).unwrap();
        assert_eq!((project.name.as_str(), project.code_page, project.references.as_slice()), ("Sales", 1252, &["stdole".to_string()][..]));
        let listed: Vec<(&str, Kind)> = project.modules.iter().map(|module| (module.name.as_str(), module.kind)).collect();
        assert_eq!(listed, [("Module1", Kind::Standard), ("ThisWorkbook", Kind::Document), ("Ledger", Kind::Class), ("Entry", Kind::Form)]);
        assert_eq!(project.modules[0].source, HELLO, "the source, without what Excel compiled before it");
        assert_eq!(project.modules[1].source, BOOK);
        assert!(project.modules.iter().all(|module| module.problem.is_none() && !module.cut));
        // Text in the project's code page comes out as text.
        assert_eq!(decode(&[0x63, 0x61, 0x66, 0xE9, 0x20, 0x80], 1252), "café €");
        assert_eq!(decode(&[0x93, 0xFA, 0x96, 0x7B], 932), "日本");
        // A module whose stream is not there is listed, with why.
        let mut dir = testing::listing("P", &[("Module1", Kind::Standard, ""), ("Gone", Kind::Object, "")]);
        dir.truncate(dir.len());
        let (_, _, _, modules) = listing(&dir);
        assert_eq!(
            modules.iter().map(|module| (module.name.as_str(), module.offset, module.object)).collect::<Vec<_>>(),
            [("Module1", 4, false), ("Gone", 4, true)]
        );
        // What it reports of a project: the modules and their procedures, not their source.
        let told = summary(&project);
        assert_eq!((&told["name"], &told["modules"][0]["kind"], &told["modules"][0]["lines"]), (&json!("Sales"), &json!("module"), &json!(10)));
        assert_eq!(
            told["modules"][0]["procedures"][1],
            json!({"what": "Function", "name": "Twice", "line": 8, "lines": 3, "runsByItselfInExcel": false, "library": null})
        );
        assert_eq!(told["modules"][1]["procedures"][0]["runsByItselfInExcel"], true);
        assert!(!told.to_string().contains("MsgBox"));
    }

    #[test]
    fn the_procedures_of_a_module_are_found_by_its_lines() {
        let source = "Attribute VB_Name = \"M\"\r\n' Sub NotThis()\r\nRem Function NorThis()\r\nPublic Sub Hello(ByVal who As String, _\r\n        Optional n As Long)\r\n    MsgBox \"Sub Fake()\"\r\nEnd Sub\r\nPrivate Declare PtrSafe Function GetTickCount Lib \"kernel32\" () As Long\r\nFriend Static Function Twice(n) As Long\r\nEnd Function\r\nPublic Property Get Total() As Double\r\nEnd Property\r\nProperty Let Total(v As Double)\r\nEnd Property\r\nSub Auto_Open()\r\nEnd Sub\r\nsub lower_case()\r\nend sub\r\n";
        let found = outline(source, Kind::Standard);
        let told: Vec<(&str, &str, usize, usize, bool)> = found.iter().map(|p| (p.what, p.name.as_str(), p.line, p.lines, p.event)).collect();
        assert_eq!(
            told,
            [
                ("Sub", "Hello", 4, 4, false),
                ("Declare", "GetTickCount", 8, 1, false),
                ("Function", "Twice", 9, 2, false),
                ("Property Get", "Total", 11, 2, false),
                ("Property Let", "Total", 13, 2, false),
                ("Sub", "Auto_Open", 15, 2, true),
                ("Sub", "lower_case", 17, 2, false),
            ]
        );
        assert_eq!(found[1].library.as_deref(), Some("kernel32"));
        // Behind the workbook, a sheet or a form, a procedure named for an event answers it.
        let behind = outline("Private Sub Workbook_Open()\r\nEnd Sub\r\nPrivate Sub Helper()\r\nEnd Sub\r\n", Kind::Document);
        assert_eq!(behind.iter().map(|p| (p.name.as_str(), p.event)).collect::<Vec<_>>(), [("Workbook_Open", true), ("Helper", false)]);
        assert!(outline("", Kind::Standard).is_empty() && outline("Sub\r\nFunction (\r\nProperty\r\n", Kind::Standard).is_empty());
    }
}
