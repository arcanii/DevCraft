//! Setups: Doom, Quake and GLQuake as projects of the C Development Kit, in a few clicks.
//!
//! **What the owner decided (2026-10-14)**, so that anyone can have the three games the kit was
//! tried with: the glue written for each game (its C, its panel, its recipe, its README) is
//! DevCraft's own text (`setup_files.rs`), with nothing of id Software's code in it;
//! GridCraft downloads the games' code and their data itself, pinned, after a question of the
//! operating system's that names every address and size, on the person's Set Up alone; every
//! file kept is checked against the hash DevCraft holds for it, and anything that does not match
//! is not kept; and the project made is untrusted, like the showcases.
//!
//! So a setup names where each part of its game comes from:
//!
//! - **the game's code**, as one zip GitHub packs of a commit. GitHub does not promise that zip's
//!   bytes stay the same, so the zip itself is not pinned: each file kept of it is, by size and
//!   SHA-256 (taken from the files as downloaded, each first checked against the git object id the
//!   release's own tree gives for it), and the zip is taken only up to half as much again as it
//!   measured when it was pinned;
//! - **the game's data**, as the archive it is given out in, pinned whole and file by file.
//!
//! Of each download only the files named are kept; the rest (a DOS release's programs among them)
//! is never written anywhere. The lines of a game's code that are changed are named by their
//! number and the SHA-256 of what they hold as released, never by their text, and the lines put in
//! are DevCraft's own. Then the files DevCraft writes for the game are added, and the project is
//! written into the projects folder, untrusted: trusting and building it are the person's.
//!
//! Neither game's code nor its data is in this repository: only where to fetch them, what they
//! must hash to, and which lines are changed. This is GridCraft's one way to download (HTTPS,
//! [`crate::setup_fetch`]), and it is used for nothing else.

use crate::bridge::Places;
#[cfg(not(target_arch = "wasm32"))]
use crate::project::Project;
use crate::setup_files as files;

/// What a download is packed as.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Archive {
    Zip,
    /// A tar file, gzipped: how Debian keeps a package's sources.
    TarGz,
}

/// One file kept of a download: where it is in the archive (under its root), where it goes in
/// the project, and what it must be.
#[derive(Debug)]
pub struct Keep {
    pub from: &'static str,
    pub to: &'static str,
    pub bytes: u64,
    pub sha256: &'static str,
}

/// One download.
#[derive(Debug)]
pub struct Source {
    /// What it is, as the question and the window say it.
    pub what: &'static str,
    /// Whose it is, and under what licence.
    pub whose: &'static str,
    /// The one address it is fetched from: HTTPS, nothing else.
    pub url: &'static str,
    /// The hosts that host may hand the download over to (a redirect): a name, or `.name` for any
    /// host under it. Any other is refused.
    pub hands_over_to: &'static [&'static str],
    /// Its size: exactly, where its hash is pinned; as it measured, where it is not.
    pub bytes: u64,
    /// The SHA-256 of the whole download, where it is fixed.
    pub sha256: Option<&'static str>,
    pub archive: Archive,
    /// The folder inside the archive the files below are named under ("" for none).
    pub root: &'static str,
    pub keep: &'static [Keep],
}

/// What is done to one line of a game's code.
#[derive(Debug)]
pub enum Edit {
    /// The line is replaced by this one.
    Replace(&'static str),
    /// These lines are put after it.
    After(&'static str),
}

/// A line of a game's code that is changed: by its file in the project, its number, and the
/// SHA-256 of what it holds as released (without its line ending).
#[derive(Debug)]
pub struct Change {
    pub file: &'static str,
    pub line: usize,
    pub was: &'static str,
    pub edit: Edit,
}

/// The game's data, where a setup offers more than one.
#[derive(Debug)]
pub struct Data {
    /// How the window and the page name the choice.
    pub key: &'static str,
    pub name: &'static str,
    pub source: Source,
}

/// One setup.
#[derive(Debug)]
pub struct Setup {
    /// The project's id, and its folder's name.
    pub id: &'static str,
    /// The game, as the window names it.
    pub game: &'static str,
    /// The project's name.
    pub name: &'static str,
    /// What it is, in a line.
    pub says: &'static str,
    pub code: Source,
    /// Its data: the first is offered first.
    pub data: &'static [Data],
    pub changes: &'static [Change],
    /// What DevCraft writes into the project, by path in it.
    pub files: &'static [(&'static str, &'static str)],
}

impl Setup {
    /// One of its data by its key.
    pub fn data_named(&self, key: &str) -> Option<&Data> {
        self.data.iter().find(|data| data.key == key)
    }
}

/// The shareware Quake, which Quake and GLQuake both play.
const SHAREWARE_QUAKE: Source = Source {
    what: "The shareware Quake 1.06, as the Internet Archive keeps it",
    whose: "Quake's data is id Software's, under their shareware licence (SLICNSE.TXT, kept with it).",
    url: "https://archive.org/download/msdos_Quake106_shareware/msdos_Quake106_shareware.zip",
    hands_over_to: &[".archive.org"],
    bytes: 9_050_608,
    sha256: Some("920f4609801d0bdbea5b6738cec49da846df4ff8ce0d46c901ea080dd4437833"),
    archive: Archive::Zip,
    root: "",
    keep: &[
        Keep {
            from: "ID1/PAK0.PAK",
            to: "panel/id1/pak0.pak",
            bytes: 18_689_235,
            sha256: "35a9c55e5e5a284a159ad2a62e0e8def23d829561fe2f54eb402dbc0a9a946af",
        },
        Keep { from: "SLICNSE.TXT", to: "SLICNSE.TXT", bytes: 10_036, sha256: "070cdf6a6410adef8fb5f83a4e5ccdb9e2301d2e48d460bb3a67a0f5ba9d70a8" },
    ],
};

/// The line Quake prints the date and time of its build with, which the kit's compiler refuses (a
/// build must come out the same each time): the same in quakegeneric and in id Software's release.
const BUILD_DATE_LINE: &str = "3d491b3d475c256fdf038113fa303b0d005a5370dbb79327f135a023265be168";

pub static DOOM: Setup = Setup {
    id: "gridcraft-doom",
    game: "Doom",
    name: "GridCraft Doom",
    says: "Doom, docked beside the sheet: its health, ammunition and monsters, and the bytes of its memory, in a sheet as it is played.",
    code: Source {
        what: "doomgeneric's Doom at commit e257a09, packed by GitHub as one zip",
        whose: "Doom is id Software's; doomgeneric is ozkl's port of its released code, under the GNU General Public License, version 2.",
        url: "https://codeload.github.com/ozkl/doomgeneric/zip/e257a09527062bfb986b6e5e0780839b6cbd5b71",
        hands_over_to: &[],
        bytes: 3_239_384,
        sha256: None,
        archive: Archive::Zip,
        root: "doomgeneric-e257a09527062bfb986b6e5e0780839b6cbd5b71/",
        keep: files::DOOM_CODE,
    },
    data: &[
        Data {
            key: "shareware",
            name: "Shareware Doom",
            source: Source {
                what: "The shareware Doom 1.9, as Debian keeps it",
                whose: "Shareware Doom is id Software's, given out free under their own terms.",
                url: "https://deb.debian.org/debian/pool/non-free/d/doom-wad-shareware/doom-wad-shareware_1.9.fixed.orig.tar.gz",
                hands_over_to: &[],
                bytes: 1_756_095,
                sha256: Some("e02c8b5e01be7373d4c53f82556118e2aaaf8f83fa2af5eee1efadf9c55c4eb1"),
                archive: Archive::TarGz,
                root: "doom-wad-shareware-1.9.fixed/",
                keep: &[Keep {
                    from: "doom1.wad",
                    to: "panel/doom1.wad",
                    bytes: 4_196_020,
                    sha256: "1d7d43be501e67d927e415e0b8f3e29c3bf33075e859721816f652a526cac771",
                }],
            },
        },
        Data {
            key: "freedoom",
            name: "Freedoom",
            source: Source {
                what: "Freedoom 0.13.0, from its releases on GitHub",
                whose: "Freedoom is the Freedoom project's, under its own licence, a BSD one (kept with it as COPYING-freedoom.txt).",
                url: "https://github.com/freedoom/freedoom/releases/download/v0.13.0/freedoom-0.13.0.zip",
                hands_over_to: &[".githubusercontent.com"],
                bytes: 24_143_781,
                sha256: Some("3f9b264f3e3ce503b4fb7f6bdcb1f419d93c7b546f4df3e874dd878db9688f59"),
                archive: Archive::Zip,
                root: "freedoom-0.13.0/",
                keep: &[
                    Keep {
                        from: "freedoom1.wad",
                        to: "panel/doom1.wad",
                        bytes: 28_795_076,
                        sha256: "7323bcc168c5a45ff10749b339960e98314740a734c30d4b9f3337001f9e703d",
                    },
                    Keep {
                        from: "COPYING.txt",
                        to: "COPYING-freedoom.txt",
                        bytes: 1_644,
                        sha256: "7c62f2c520769c798774f416637eec4921e5f0aafdac1245ae7c8a8cf65fe102",
                    },
                ],
            },
        },
    ],
    changes: &[],
    files: files::DOOM_FILES,
};

pub static QUAKE: Setup = Setup {
    id: "gridcraft-quake",
    game: "Quake",
    name: "GridCraft Quake",
    says: "Quake, drawn dot by dot as it first was, docked beside the sheet: its variables, and the bytes of its memory, in a sheet as it is played.",
    code: Source {
        what: "quakegeneric's Quake at commit 1305210, packed by GitHub as one zip",
        whose: "Quake is id Software's; quakegeneric is erysdren's port of its released code, under the GNU General Public License, version 2 or later.",
        url: "https://codeload.github.com/erysdren/quakegeneric/zip/13052102577c629650cf07a46151a4b6e1b19c3c",
        hands_over_to: &[],
        bytes: 412_967,
        sha256: None,
        archive: Archive::Zip,
        root: "quakegeneric-13052102577c629650cf07a46151a4b6e1b19c3c/",
        keep: files::QUAKE_CODE,
    },
    data: &[Data { key: "shareware", name: "Shareware Quake", source: SHAREWARE_QUAKE }],
    changes: &[
        Change {
            file: "quake/host.c",
            line: 816,
            was: BUILD_DATE_LINE,
            edit: Edit::Replace(
                r#"	Con_Printf ("This Quake was built by DevCraft's C Development Kit.\n");	// GridCraft Quake: in place of the build's date and time, which the kit's compiler refuses"#,
            ),
        },
        Change {
            file: "quake/host_cmd.c",
            line: 708,
            was: BUILD_DATE_LINE,
            edit: Edit::Replace(
                r#"	Con_Printf ("This Quake was built by DevCraft's C Development Kit.\n");	// GridCraft Quake: in place of the build's date and time, which the kit's compiler refuses"#,
            ),
        },
    ],
    files: files::QUAKE_FILES,
};

pub static GLQUAKE: Setup = Setup {
    id: "gridcraft-glquake",
    game: "GLQuake",
    name: "GridCraft GLQuake",
    says: "GLQuake, drawn with OpenGL, docked beside the sheet with its variables in a sheet; and it plays another GridCraft over the network.",
    code: Source {
        what: "id Software's release of Quake's source at commit bf4ac42, packed by GitHub as one zip",
        whose: "Quake is id Software's, and so is this release of its code, under the GNU General Public License, version 2 or later.",
        url: "https://codeload.github.com/id-Software/Quake/zip/bf4ac424ce754894ac8f1dae6a3981954bc9852d",
        hands_over_to: &[],
        bytes: 3_413_501,
        sha256: None,
        archive: Archive::Zip,
        root: "Quake-bf4ac424ce754894ac8f1dae6a3981954bc9852d/",
        keep: files::GLQUAKE_CODE,
    },
    data: &[Data { key: "shareware", name: "Shareware Quake", source: SHAREWARE_QUAKE }],
    changes: &[
        Change {
            file: "quake/host.c",
            line: 871,
            was: BUILD_DATE_LINE,
            edit: Edit::Replace(
                r#"	Con_Printf ("This GLQuake was built by DevCraft's C Development Kit.\n");	// GridCraft GLQuake: in place of the build's date and time, which the kit's compiler refuses"#,
            ),
        },
        Change {
            file: "quake/host_cmd.c",
            line: 952,
            was: BUILD_DATE_LINE,
            edit: Edit::Replace(
                r#"	Con_Printf ("This GLQuake was built by DevCraft's C Development Kit.\n");	// GridCraft GLQuake: in place of the build's date and time, which the kit's compiler refuses"#,
            ),
        },
        Change {
            file: "quake/host_cmd.c",
            line: 1892,
            was: "e7ce68e2dfcc522b2fc2a20b478e7659724bdf6e17a2c20dba02c7c819e28ab8",
            edit: Edit::Replace(
                r#"	Cmd_AddCommand ("connect", GridCraft_Connect_f);	// GridCraft GLQuake: the project's connect, which asks the server first, a frame at a time (src/net_gridcraft.c)"#,
            ),
        },
        Change {
            file: "quake/gl_warp.c",
            line: 26,
            was: "21c107b398650319a14b74db2aabff7d4c8d4d74bd121676e3999f37b9a72524",
            edit: Edit::Replace(
                "extern int skytexturenum;\t// GridCraft GLQuake: gl_rsurf.c defines it too; the compilers of 1996 made one of the two, and the kit's refuses two",
            ),
        },
        Change {
            file: "quake/gl_screen.c",
            line: 106,
            was: "8dc668191f53019de4b8960baacb1a852179390735c3178e3b461ed5fccc0e0b",
            edit: Edit::Replace(
                "extern int sb_lines;\t// GridCraft GLQuake: sbar.c defines it too; the compilers of 1996 made one of the two, and the kit's refuses two",
            ),
        },
        Change {
            file: "quake/net_dgrm.c",
            line: 1306,
            was: "2bf57beff6a7530e7f50d93779854207ff299d3c2bc3289709b3d23fef282352",
            edit: Edit::Replace(
                "\t\twhile (0);\t// GridCraft GLQuake: no waiting here for the server's answer, in which a page takes in nothing: the project's connect asked a frame before",
            ),
        },
        Change {
            file: "quake/net_main.c",
            line: 953,
            was: "648ad4ec82c22290e4ae9f2a23cb58e9dd3eee7fd954bcfa1ab6d19435f3c436",
            edit: Edit::Replace(
                "\t\tpp->procedure ();\t// GridCraft GLQuake: none of the poll procedures takes an argument, and WebAssembly stops a call that hands one over",
            ),
        },
        Change {
            file: "quake/quakedef.h",
            line: 335,
            was: "64ca278983fb4a6ada67c63ccef7f544e60c318a32b40db96e29ed347a97ccb7",
            edit: Edit::After(
                "\n// GridCraft GLQuake: what GLQuake's files call before anything declares it, which the kit's compiler refuses\n#include \"gridcraft_glquake.h\"",
            ),
        },
    ],
    files: files::GLQUAKE_FILES,
};

/// Every setup, in the order the window lists them.
pub static ALL: [&Setup; 3] = [&DOOM, &QUAKE, &GLQUAKE];

/// A setup by its project's id.
pub fn named(id: &str) -> Option<&'static Setup> {
    ALL.iter().copied().find(|setup| setup.id == id)
}

/// The most a download may send: its size, where its hash is fixed; half as much again as it
/// measured, where only the files in it are.
pub fn limit(source: &Source) -> u64 {
    match source.sha256 {
        Some(_) => source.bytes,
        None => source.bytes.saturating_add(source.bytes / 2),
    }
}

/// The host of an `https://` address, in lower case. `None` for any other address, or one that
/// names a port or a user.
pub fn https_host(url: &str) -> Option<String> {
    let rest = url.strip_prefix("https://")?;
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let host = rest.get(..end)?;
    let fine = !host.is_empty() && host.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-');
    fine.then(|| host.to_ascii_lowercase())
}

/// Whether a download from `source` may be handed over to `host`.
pub fn may_hand_over(source: &Source, host: &str) -> bool {
    https_host(source.url).is_some_and(|own| own == host)
        || source.hands_over_to.iter().any(|named| match named.strip_prefix('.') {
            Some(under) => host.len() > named.len() && host.ends_with(named) && !under.is_empty(),
            None => host == *named,
        })
}

/// A number of bytes with its thousands marked: "9,050,608".
pub fn bytes_text(bytes: u64) -> String {
    let digits = bytes.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (n, digit) in digits.chars().enumerate() {
        if n > 0 && (digits.len() - n).is_multiple_of(3) {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// Megabytes, to one place: "3.1 MB".
fn megabytes(bytes: u64) -> String {
    format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
}

/// How big a download is, as the question says it.
pub fn size_says(source: &Source) -> String {
    match source.sha256 {
        Some(_) => format!("{} bytes", bytes_text(source.bytes)),
        None => format!("about {}, and no more than {} is taken", megabytes(source.bytes), megabytes(limit(source))),
    }
}

/// What is kept of a download: its few files by their names in it, or how many and how much.
pub fn keeps_says(source: &Source) -> String {
    let name = |keep: &Keep| keep.from.rsplit('/').next().unwrap_or(keep.from).to_string();
    match source.keep {
        [one] => format!("{} ({} bytes)", name(one), bytes_text(one.bytes)),
        [one, two] => format!("{} ({} bytes) and {} ({} bytes)", name(one), bytes_text(one.bytes), name(two), bytes_text(two.bytes)),
        all => format!("{} files, {} bytes", all.len(), bytes_text(all.iter().map(|keep| keep.bytes).sum())),
    }
}

/// Where else a download may come from, said: `None` when nowhere.
pub fn handed_over_says(source: &Source) -> Option<String> {
    let host = https_host(source.url)?;
    let others: Vec<String> =
        source.hands_over_to.iter().map(|named| if named.starts_with('.') { format!("*{named}") } else { (*named).to_string() }).collect();
    (!others.is_empty()).then(|| format!("{host} may hand it over from another of its servers ({}); from nowhere else.", others.join(", ")))
}

/// The operating system's question before a setup downloads anything: it names every address,
/// and every size.
pub fn question(setup: &Setup, data: &Data) -> String {
    let mut text = format!("Set up {} as a project?\n\nGridCraft will download, from these addresses only:\n", setup.game);
    for (n, source) in [&setup.code, &data.source].into_iter().enumerate() {
        text.push_str(&format!(
            "\n{}. {}\n   {}: {}. It keeps {} of it.\n   {}\n",
            n + 1,
            source.url,
            source.what,
            size_says(source),
            keeps_says(source),
            source.whose
        ));
        if let Some(other) = handed_over_says(source) {
            text.push_str(&format!("   {other}\n"));
        }
    }
    text.push_str(&format!(
        "\nEvery file kept is checked against the SHA-256 DevCraft holds for it: if one does not match, nothing is kept. \
         GridCraft then writes the project {} into your projects folder, with what DevCraft writes for it (its C, its panel, \
         its recipe and its README). It is not trusted: nothing of it runs until you have reviewed it, trusted it and built it.",
        setup.id
    ));
    text
}

/// Where a setup is put together before its project is moved into the projects folder: beside
/// the settings, never in the projects folder, so that a project half written is never listed.
pub fn staging_dir(settings: &std::path::Path) -> std::path::PathBuf {
    settings.with_file_name("setups")
}

/// How a setup asked for stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Progress {
    /// Downloading from this host: so many bytes of at most so many.
    Fetching { from: String, got: u64, of: u64 },
    /// Downloaded: checking every file kept, and writing the project.
    Checking,
    /// It ended: what was made, or why nothing was.
    Done(Result<String, String>),
}

#[cfg(not(target_arch = "wasm32"))]
pub use host::*;

#[cfg(target_arch = "wasm32")]
pub use web::*;

/// The web build keeps no settings, and so sets nothing up.
#[cfg(target_arch = "wasm32")]
mod web {
    use super::{Data, Places, Progress, Setup};
    use std::path::Path;

    pub const NONE: &str = "Setups need GridCraft for the desktop: the web build keeps no projects.";

    #[derive(Default)]
    pub struct Runner;

    impl Runner {
        pub fn global() -> &'static Runner {
            static RUNNER: Runner = Runner;
            &RUNNER
        }

        pub fn start(&self, _places: &Places, _setup: &'static Setup, _data: &'static Data, _done: impl Fn() + Send + 'static) -> Result<(), String> {
            Err(NONE.into())
        }

        pub fn progress(&self, _projects: &Path, _id: &str) -> Option<Progress> {
            None
        }

        pub fn running(&self) -> usize {
            0
        }

        pub fn stop(&self, _projects: &Path, _id: &str) -> bool {
            false
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod host {
    use std::collections::HashMap;
    use std::fs;
    use std::io::{Cursor, Read};
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};

    use super::{Archive, Change, Data, Edit, Keep, Places, Progress, Project, Setup, Source, https_host, limit, may_hand_over, staging_dir};
    use crate::console::{Console, Kind};
    use crate::sha256::Sha256;
    use crate::{manifest, project};

    /// The most redirects a download follows.
    pub const MAX_HAND_OVERS: usize = 5;
    /// The most a gzipped tar is unpacked to.
    const MAX_UNPACKED: u64 = 256 * 1024 * 1024;
    /// What a download stopped by the person says.
    pub const STOPPED: &str = "Stopped: nothing was kept.";

    /// One answer to a GET, before its body is read: what the setup needs of HTTPS. The real one
    /// is [`crate::setup_fetch::Https`]; tests hand in their own.
    pub struct Reply {
        pub status: u16,
        /// Where a redirect points.
        pub location: Option<String>,
        /// The length the server says the body has.
        pub length: Option<u64>,
        pub body: Box<dyn Read + Send>,
    }

    /// Whatever fetches an address: one GET, following no redirect.
    pub trait Transport: Send + Sync {
        fn get(&self, url: &str) -> Result<Reply, String>;
    }

    fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
        mutex.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn sha256(bytes: &[u8]) -> String {
        let mut hash = Sha256::new();
        hash.update(bytes);
        hash.hex()
    }

    /// Downloads one source: from its address, or from a host it may hand over to; taking no
    /// more than its [`limit`]; saying how far it has got; stopping when asked.
    pub fn download(transport: &dyn Transport, source: &Source, got: &dyn Fn(u64), stop: &AtomicBool) -> Result<Vec<u8>, String> {
        let first = https_host(source.url).ok_or_else(|| format!("{} is not an address GridCraft downloads from.", source.url))?;
        let most = limit(source);
        let mut url = source.url.to_string();
        let mut host = first.clone();
        for _ in 0..=MAX_HAND_OVERS {
            if stop.load(Ordering::SeqCst) {
                return Err(STOPPED.into());
            }
            let reply = transport.get(&url).map_err(|why| format!("{host} could not be reached: {why}"))?;
            match reply.status {
                200 => return read_body(reply, &host, most, got, stop),
                301 | 302 | 303 | 307 | 308 => {
                    let Some(next) = reply.location.filter(|next| !next.is_empty()) else {
                        return Err(format!("{host} handed the download over without saying where."));
                    };
                    let next = if next.starts_with('/') { format!("https://{host}{next}") } else { next };
                    let Some(next_host) = https_host(&next) else {
                        return Err(format!("{host} handed the download over to an address that is not HTTPS: nothing was taken from it."));
                    };
                    if !may_hand_over(source, &next_host) {
                        return Err(format!(
                            "{first} handed the download over to {next_host}, which the question did not name: nothing was taken from it."
                        ));
                    }
                    url = next;
                    host = next_host;
                }
                status => return Err(format!("{host} answered {status}, not the file: nothing was taken.")),
            }
        }
        Err(format!("{first} handed the download over more than {MAX_HAND_OVERS} times: nothing was taken."))
    }

    fn read_body(mut reply: Reply, host: &str, most: u64, got: &dyn Fn(u64), stop: &AtomicBool) -> Result<Vec<u8>, String> {
        if reply.length.is_some_and(|length| length > most) {
            return Err(format!("{host} offered more than DevCraft expects of it: nothing was taken."));
        }
        let mut held = Vec::with_capacity(usize::try_from(reply.length.unwrap_or(0).min(most)).unwrap_or(0));
        let mut buffer = vec![0u8; 64 * 1024];
        loop {
            if stop.load(Ordering::SeqCst) {
                return Err(STOPPED.into());
            }
            let read = reply.body.read(&mut buffer).map_err(|why| format!("The download from {host} broke off: {why}"))?;
            if read == 0 {
                return Ok(held);
            }
            let chunk = buffer.get(..read).ok_or_else(|| format!("The download from {host} broke off."))?;
            if u64::try_from(held.len().saturating_add(read)).unwrap_or(u64::MAX) > most {
                return Err(format!("{host} sent more than DevCraft expects of it: nothing was kept."));
            }
            held.extend_from_slice(chunk);
            got(u64::try_from(held.len()).unwrap_or(u64::MAX));
        }
    }

    /// The files kept of one download, each checked, where each goes in the project.
    pub fn unpack(source: &Source, bytes: &[u8]) -> Result<Vec<(&'static str, Vec<u8>)>, String> {
        let host = https_host(source.url).unwrap_or_default();
        if let Some(want) = source.sha256
            && sha256(bytes) != want
        {
            return Err(format!("What came from {host} is not what DevCraft expects (its SHA-256 differs): nothing of it was kept."));
        }
        let kept = match source.archive {
            Archive::Zip => from_zip(source, bytes, &host)?,
            Archive::TarGz => from_tar_gz(source, bytes, &host)?,
        };
        // Every file named was found, once.
        for keep in source.keep {
            if kept.iter().filter(|(to, _)| *to == keep.to).count() != 1 {
                return Err(format!("{} is not in what came from {host}: nothing of it was kept.", keep.from));
            }
        }
        Ok(kept)
    }

    fn checked(keep: &Keep, data: Vec<u8>, host: &str) -> Result<(&'static str, Vec<u8>), String> {
        if u64::try_from(data.len()).ok() != Some(keep.bytes) || sha256(&data) != keep.sha256 {
            return Err(format!(
                "{} from {host} is not the file DevCraft expects (its size or its SHA-256 differs): nothing of this setup was kept.",
                keep.from
            ));
        }
        Ok((keep.to, data))
    }

    fn from_zip(source: &Source, bytes: &[u8], host: &str) -> Result<Vec<(&'static str, Vec<u8>)>, String> {
        let damaged = || format!("What came from {host} can't be opened as a zip: nothing of it was kept.");
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).map_err(|_| damaged())?;
        let mut kept = Vec::with_capacity(source.keep.len());
        for keep in source.keep {
            let name = format!("{}{}", source.root, keep.from);
            let mut entry = archive.by_name(&name).map_err(|_| format!("{} is not in what came from {host}: nothing of it was kept.", keep.from))?;
            if entry.size() != keep.bytes {
                return Err(format!("{} from {host} is not the size DevCraft expects: nothing of this setup was kept.", keep.from));
            }
            let mut data = Vec::with_capacity(usize::try_from(keep.bytes).unwrap_or(0));
            (&mut entry).take(keep.bytes.saturating_add(1)).read_to_end(&mut data).map_err(|_| damaged())?;
            kept.push(checked(keep, data, host)?);
        }
        Ok(kept)
    }

    /// Reads a block of a tar file: `false` at its very end.
    fn block(from: &mut impl Read, into: &mut [u8; 512]) -> Result<bool, ()> {
        let mut filled = 0;
        while filled < into.len() {
            let rest = into.get_mut(filled..).ok_or(())?;
            match from.read(rest) {
                Ok(0) if filled == 0 => return Ok(false),
                Ok(0) | Err(_) => return Err(()),
                Ok(read) => filled += read,
            }
        }
        Ok(true)
    }

    /// A tar header's text field: up to its first zero.
    fn field(header: &[u8; 512], at: std::ops::Range<usize>) -> &[u8] {
        let raw = header.get(at).unwrap_or(&[]);
        raw.split(|byte| *byte == 0).next().unwrap_or(&[])
    }

    fn from_tar_gz(source: &Source, bytes: &[u8], host: &str) -> Result<Vec<(&'static str, Vec<u8>)>, String> {
        let damaged = || format!("What came from {host} can't be unpacked as a gzipped tar: nothing of it was kept.");
        let mut tar = flate2::read::GzDecoder::new(bytes).take(MAX_UNPACKED);
        let mut kept = Vec::with_capacity(source.keep.len());
        let mut header = [0u8; 512];
        while block(&mut tar, &mut header).map_err(|()| damaged())? {
            if header.iter().all(|byte| *byte == 0) {
                break;
            }
            let name = String::from_utf8_lossy(field(&header, 0..100)).into_owned();
            let prefix = String::from_utf8_lossy(field(&header, 345..500)).into_owned();
            // A POSIX tar may put the start of a long name in its prefix; a GNU tar keeps other
            // things there.
            let posix = header.get(257..263) == Some(&b"ustar\0"[..]);
            let name = if posix && !prefix.is_empty() { format!("{prefix}/{name}") } else { name };
            let octal = String::from_utf8_lossy(field(&header, 124..136)).trim().to_string();
            let size = u64::from_str_radix(&octal, 8).map_err(|_| damaged())?;
            let padded = size.checked_add(511).map(|size| size / 512 * 512).ok_or_else(damaged)?;
            let regular = matches!(header.get(156), Some(b'0') | Some(0));
            let wanted = name.strip_prefix(source.root).and_then(|inner| source.keep.iter().find(|keep| keep.from == inner)).filter(|_| regular);
            match wanted {
                Some(keep) => {
                    if size != keep.bytes {
                        return Err(format!("{} from {host} is not the size DevCraft expects: nothing of this setup was kept.", keep.from));
                    }
                    let mut data = Vec::with_capacity(usize::try_from(size).unwrap_or(0));
                    (&mut tar).take(size).read_to_end(&mut data).map_err(|_| damaged())?;
                    std::io::copy(&mut (&mut tar).take(padded - size), &mut std::io::sink()).map_err(|_| damaged())?;
                    kept.push(checked(keep, data, host)?);
                }
                None => {
                    let skipped = std::io::copy(&mut (&mut tar).take(padded), &mut std::io::sink()).map_err(|_| damaged())?;
                    if skipped != padded {
                        return Err(damaged());
                    }
                }
            }
        }
        Ok(kept)
    }

    /// A file's lines, each with its ending.
    fn lines(bytes: &[u8]) -> Vec<Vec<u8>> {
        bytes.split_inclusive(|byte| *byte == b'\n').map(<[u8]>::to_vec).collect()
    }

    /// A line without its ending, and its ending.
    fn ending(line: &[u8]) -> (&[u8], &[u8]) {
        let cut = if line.ends_with(b"\r\n") {
            2
        } else if line.ends_with(b"\n") {
            1
        } else {
            0
        };
        line.split_at(line.len() - cut)
    }

    /// Makes the changes a setup makes to its game's code: each line found by its number and
    /// checked by its hash, a file's from its last to its first so that each number is still the
    /// release's.
    pub fn apply(files: &mut [(String, Vec<u8>)], changes: &[Change]) -> Result<(), String> {
        let mut order: Vec<&Change> = changes.iter().collect();
        order.sort_by(|a, b| a.file.cmp(b.file).then(b.line.cmp(&a.line)));
        for change in order {
            let not_there = || format!("Line {} of {} is not the line DevCraft changes: nothing of this setup was kept.", change.line, change.file);
            let (_, bytes) = files.iter_mut().find(|(path, _)| path == change.file).ok_or_else(not_there)?;
            let mut all = lines(bytes);
            let at = change.line.checked_sub(1).ok_or_else(not_there)?;
            let line = all.get(at).ok_or_else(not_there)?;
            let (text, end) = ending(line);
            if sha256(text) != change.was {
                return Err(not_there());
            }
            let text = text.to_vec();
            let end = if end.is_empty() { b"\n".to_vec() } else { end.to_vec() };
            match change.edit {
                Edit::Replace(now) => {
                    let mut replaced = now.as_bytes().to_vec();
                    replaced.extend_from_slice(&end);
                    if let Some(slot) = all.get_mut(at) {
                        *slot = replaced;
                    }
                }
                Edit::After(added) => {
                    if let Some(slot) = all.get_mut(at) {
                        let mut whole = text.clone();
                        whole.extend_from_slice(&end);
                        *slot = whole;
                    }
                    let mut put = Vec::new();
                    for line in added.split('\n') {
                        let mut line = line.as_bytes().to_vec();
                        line.extend_from_slice(&end);
                        put.push(line);
                    }
                    let after = at.saturating_add(1).min(all.len());
                    all.splice(after..after, put);
                }
            }
            *bytes = all.concat();
        }
        Ok(())
    }

    /// The project's files, by path in it: what is kept of the downloads, changed where the
    /// setup says, and what DevCraft writes.
    pub fn assemble(setup: &Setup, data: &Data, code: &[u8], data_bytes: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
        let mut files: Vec<(String, Vec<u8>)> = Vec::new();
        for (source, bytes) in [(&setup.code, code), (&data.source, data_bytes)] {
            files.extend(unpack(source, bytes)?.into_iter().map(|(to, bytes)| (to.to_string(), bytes)));
        }
        apply(&mut files, setup.changes)?;
        files.extend(setup.files.iter().map(|(path, text)| ((*path).to_string(), text.as_bytes().to_vec())));
        files.sort_by(|a, b| a.0.cmp(&b.0));
        if files.windows(2).any(|pair| matches!(pair, [a, b] if a.0 == b.0)) {
            return Err(format!("DevCraft's setup of {} names one file twice: nothing was written.", setup.game));
        }
        Ok(files)
    }

    /// Writes the project: put together beside the settings, then moved into the projects
    /// folder whole. Never over a folder that is there; untrusted, like any new project.
    pub fn write(places: &Places, setup: &Setup, files: &[(String, Vec<u8>)]) -> Result<Project, String> {
        let target = places.projects.join(setup.id);
        if target.exists() {
            return Err(format!("There is already a project folder named {}: nothing was written over it.", setup.id));
        }
        let staging = staging_dir(&places.settings).join(setup.id);
        let _ = fs::remove_dir_all(&staging);
        let outcome = put_together(&staging, &target, &places.projects, files).map_err(|why| format!("{} could not be written: {why}", setup.name));
        let _ = fs::remove_dir_all(&staging);
        // The folder setups are put together in goes too, once nothing is in it.
        let _ = fs::remove_dir(staging_dir(&places.settings));
        outcome?;
        let made = project::load(&target);
        match &made.problem {
            Some(problem) => Err(format!("{} was written, but can't be used: {problem}.", setup.name)),
            None => Ok(made),
        }
    }

    fn put_together(staging: &Path, target: &Path, projects: &Path, files: &[(String, Vec<u8>)]) -> Result<(), String> {
        for (path, bytes) in files {
            let inner = manifest::inner_path(path)?;
            let file = inner.split('/').fold(staging.to_path_buf(), |at, part| at.join(part));
            if let Some(folder) = file.parent() {
                fs::create_dir_all(folder).map_err(|e| e.to_string())?;
            }
            fs::write(&file, bytes).map_err(|e| e.to_string())?;
        }
        fs::create_dir_all(projects).map_err(|e| e.to_string())?;
        if target.exists() {
            return Err(format!("a folder named {} was made meanwhile", target.display()));
        }
        fs::rename(staging, target).map_err(|e| e.to_string())
    }

    /// A step of a setup, as it is said while it runs.
    pub enum Step {
        Fetching { from: String, got: u64, of: u64 },
        Checking,
    }

    /// A whole setup: downloads, checks, puts the project together and writes it. What it did
    /// is said in the Developer console under the project's id.
    pub fn run(
        places: &Places,
        setup: &Setup,
        data: &Data,
        transport: &dyn Transport,
        step: &dyn Fn(Step),
        stop: &AtomicBool,
    ) -> Result<String, String> {
        let say = |text: &str| Console::global().push(setup.id, "setup", Kind::Log, text);
        if places.projects.join(setup.id).exists() {
            return Err(format!("There is already a project folder named {}: nothing was downloaded.", setup.id));
        }
        let mut fetched = Vec::with_capacity(2);
        for source in [&setup.code, &data.source] {
            let from = https_host(source.url).unwrap_or_default();
            let of = limit(source);
            say(&format!("Downloading from {from}: {}.", source.what));
            step(Step::Fetching { from: from.clone(), got: 0, of });
            let bytes = download(transport, source, &|got| step(Step::Fetching { from: from.clone(), got, of }), stop)?;
            say(&format!("{} bytes came from {from}.", super::bytes_text(u64::try_from(bytes.len()).unwrap_or(u64::MAX))));
            fetched.push(bytes);
        }
        step(Step::Checking);
        let (code, data_bytes) = match fetched.as_slice() {
            [code, data_bytes] => (code, data_bytes),
            _ => return Err("The downloads went missing.".into()),
        };
        let files = assemble(setup, data, code, data_bytes)?;
        let kept = setup.code.keep.len() + data.source.keep.len();
        say(&format!("Every one of the {kept} files kept matches the SHA-256 DevCraft holds for it."));
        let made = write(places, setup, &files)?;
        let done = format!("{} is in your projects, not trusted: review and trust it in its own part, then build it.", made.name());
        say(&done);
        Ok(done)
    }

    struct Run {
        state: Mutex<Progress>,
        stop: AtomicBool,
    }

    /// Setups that were asked for, each on a thread of its own: a download takes seconds or
    /// minutes, and the window that asked goes on being drawn.
    #[derive(Default)]
    pub struct Runner {
        runs: Mutex<HashMap<(PathBuf, String), Arc<Run>>>,
    }

    impl Runner {
        pub fn new() -> Self {
            Self::default()
        }

        /// The one the app uses.
        pub fn global() -> &'static Runner {
            static RUNNER: OnceLock<Runner> = OnceLock::new();
            RUNNER.get_or_init(Runner::new)
        }

        /// Starts a setup, downloading over HTTPS. `done` is called when it has ended.
        pub fn start(&self, places: &Places, setup: &'static Setup, data: &'static Data, done: impl Fn() + Send + 'static) -> Result<(), String> {
            self.start_with(places, setup, data, Arc::new(crate::setup_fetch::Https::new()), done)
        }

        /// Starts a setup that downloads with `transport`. Refused while that one is running.
        pub fn start_with(
            &self,
            places: &Places,
            setup: &'static Setup,
            data: &'static Data,
            transport: Arc<dyn Transport>,
            done: impl Fn() + Send + 'static,
        ) -> Result<(), String> {
            let key = (places.projects.clone(), setup.id.to_string());
            let first = Progress::Fetching { from: https_host(setup.code.url).unwrap_or_default(), got: 0, of: limit(&setup.code) };
            let run = Arc::new(Run { state: Mutex::new(first), stop: AtomicBool::new(false) });
            {
                let mut runs = lock(&self.runs);
                if runs.get(&key).is_some_and(|run| !matches!(*lock(&run.state), Progress::Done(_))) {
                    return Err(format!("{} is being set up already.", setup.game));
                }
                runs.insert(key, run.clone());
            }
            let places = places.clone();
            let spawned = std::thread::Builder::new().name("gridcraft-setup".into()).spawn(move || {
                let step = |step: Step| {
                    *lock(&run.state) = match step {
                        Step::Fetching { from, got, of } => Progress::Fetching { from, got, of },
                        Step::Checking => Progress::Checking,
                    }
                };
                let outcome = run_guarded(&places, setup, data, transport.as_ref(), &step, &run.stop);
                if let Err(why) = &outcome {
                    Console::global().push(setup.id, "setup", Kind::Error, &format!("{} was not set up. {why}", setup.game));
                }
                *lock(&run.state) = Progress::Done(outcome);
                done();
            });
            spawned.map(|_| ()).map_err(|e| format!("The setup could not be started ({e})."))
        }

        /// How the last setup asked for of the project `id` stands. `None`: none was asked for
        /// since GridCraft started.
        pub fn progress(&self, projects: &Path, id: &str) -> Option<Progress> {
            let run = lock(&self.runs).get(&(projects.to_path_buf(), id.to_string())).cloned()?;
            let state = lock(&run.state).clone();
            Some(state)
        }

        /// How many setups are running.
        pub fn running(&self) -> usize {
            lock(&self.runs).values().filter(|run| !matches!(*lock(&run.state), Progress::Done(_))).count()
        }

        /// Asks a running setup to stop. Whether one was running.
        pub fn stop(&self, projects: &Path, id: &str) -> bool {
            let Some(run) = lock(&self.runs).get(&(projects.to_path_buf(), id.to_string())).cloned() else { return false };
            let running = !matches!(*lock(&run.state), Progress::Done(_));
            if running {
                run.stop.store(true, Ordering::SeqCst);
            }
            running
        }
    }

    /// [`run`], with a panic in it (there should be none) said as a setup that failed.
    fn run_guarded(
        places: &Places,
        setup: &Setup,
        data: &Data,
        transport: &dyn Transport,
        step: &dyn Fn(Step),
        stop: &AtomicBool,
    ) -> Result<String, String> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(places, setup, data, transport, step, stop)))
            .unwrap_or_else(|_| Err("The setup stopped on a fault of GridCraft's: nothing was kept.".into()))
    }
}

#[cfg(test)]
mod tests;
