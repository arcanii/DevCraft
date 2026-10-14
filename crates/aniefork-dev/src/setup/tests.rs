//! The setups: what they name, and what they do with what comes, with a stand-in for the network.
//! One test, with the real downloads, runs only where `GRIDCRAFT_TEST_SETUPS` names a folder that
//! holds them (see [`real_downloads`]).

use std::collections::{HashMap, HashSet};
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

use super::*;
use crate::project::tests::TempDir;
use crate::trust::{Status, TrustStore};
use crate::{manifest, project};

fn leak<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

fn text(value: String) -> &'static str {
    Box::leak(value.into_boxed_str())
}

fn hash(bytes: &[u8]) -> &'static str {
    let mut sha = crate::sha256::Sha256::new();
    sha.update(bytes);
    text(sha.hex())
}

fn places(t: &TempDir) -> Places {
    Places { projects: t.0.join("projects"), trust: t.0.join("trusted-projects.json"), settings: t.0.join("developer.json") }
}

fn zip_of(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut zip = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for (name, bytes) in files {
        zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
        zip.write_all(bytes).unwrap();
    }
    zip.finish().unwrap().into_inner()
}

/// A gzipped tar in the POSIX form, its names under 100 bytes.
fn tar_gz_of(files: &[(&str, &[u8])]) -> Vec<u8> {
    let mut tar = Vec::new();
    for (name, bytes) in files {
        let mut header = [0u8; 512];
        header[..name.len()].copy_from_slice(name.as_bytes());
        header[100..108].copy_from_slice(b"0000644\0");
        header[124..136].copy_from_slice(format!("{:011o}\0", bytes.len()).as_bytes());
        header[156] = b'0';
        header[257..263].copy_from_slice(b"ustar\0");
        header[263..265].copy_from_slice(b"00");
        tar.extend_from_slice(&header);
        tar.extend_from_slice(bytes);
        tar.resize(tar.len().div_ceil(512) * 512, 0);
    }
    tar.extend_from_slice(&[0u8; 1024]);
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    gz.write_all(&tar).unwrap();
    gz.finish().unwrap()
}

/// A stand-in for the network: an answer for each address, and what was asked.
#[derive(Default)]
struct Stand {
    answers: HashMap<String, (u16, Option<String>, Vec<u8>, Option<u64>)>,
    asked: Mutex<Vec<String>>,
}

impl Stand {
    fn file(mut self, url: &str, bytes: Vec<u8>) -> Self {
        let length = Some(bytes.len() as u64);
        self.answers.insert(url.into(), (200, None, bytes, length));
        self
    }

    fn redirect(mut self, url: &str, to: &str) -> Self {
        self.answers.insert(url.into(), (302, Some(to.into()), Vec::new(), Some(0)));
        self
    }

    fn answer(mut self, url: &str, status: u16, bytes: Vec<u8>, length: Option<u64>) -> Self {
        self.answers.insert(url.into(), (status, None, bytes, length));
        self
    }

    fn asked(&self) -> Vec<String> {
        self.asked.lock().unwrap().clone()
    }
}

impl Transport for Stand {
    fn get(&self, url: &str) -> Result<Reply, String> {
        self.asked.lock().unwrap().push(url.to_string());
        let (status, location, bytes, length) = self.answers.get(url).cloned().ok_or("no such host")?;
        Ok(Reply { status, location, length, body: Box::new(Cursor::new(bytes)) })
    }
}

const CODE_URL: &str = "https://code.example.org/game.zip";
const DATA_URL: &str = "https://data.example.org/game-data.tar.gz";
const RECIPE: &str = "id = \"tiny-game\"\nname = \"Tiny Game\"\nversion = \"0.4.0\"\nkit = \"c\"\npermissions = []\n[panel]\nentry = \"panel/index.html\"\n[c]\nsources = [\"code\", \"src\"]\nentry = \"none\"\n";
const GAME_C: &[u8] = b"/* the game */\nint ticks;\nint lives = 3;\nvoid tick(void) { ticks++; }\n";
const LICENCE: &[u8] = b"Not ours.\n";
const GAME_DATA: &[u8] = b"levels and pictures";

/// A setup of a tiny game of the C kit: code from a zip, data from a gzipped tar handed over to
/// another host, two lines changed, and three files of DevCraft's.
fn tiny() -> (&'static Setup, Vec<u8>, Vec<u8>) {
    let code = zip_of(&[("game-1/src/game.c", GAME_C), ("game-1/LICENSE", LICENCE), ("game-1/src/other.c", b"left out\n")]);
    let data = tar_gz_of(&[("game-data/README", b"left out"), ("game-data/game.dat", GAME_DATA)]);
    let line = |n: usize| hash(GAME_C.split(|b| *b == b'\n').nth(n - 1).unwrap());
    let keep = |from: &'static str, to: &'static str, bytes: &[u8]| Keep { from, to, bytes: bytes.len() as u64, sha256: hash(bytes) };
    let setup = leak(Setup {
        id: "tiny-game",
        game: "Tiny",
        name: "Tiny Game",
        says: "A game, tiny.",
        code: Source {
            what: "Tiny's code",
            whose: "Tiny's own.",
            url: CODE_URL,
            hands_over_to: &[],
            bytes: code.len() as u64,
            sha256: None,
            archive: Archive::Zip,
            root: "game-1/",
            keep: leak([keep("src/game.c", "code/game.c", GAME_C), keep("LICENSE", "LICENSE", LICENCE)]),
        },
        data: leak([Data {
            key: "plain",
            name: "Plain",
            source: Source {
                what: "Tiny's data",
                whose: "Tiny's own, too.",
                url: DATA_URL,
                hands_over_to: &[".mirrors.example.net"],
                bytes: data.len() as u64,
                sha256: Some(hash(&data)),
                archive: Archive::TarGz,
                root: "game-data/",
                keep: leak([keep("game.dat", "panel/game.dat", GAME_DATA)]),
            },
        }]),
        changes: leak([
            Change { file: "code/game.c", line: 3, was: line(3), edit: Edit::Replace("int lives = 9; /* DevCraft's */") },
            Change { file: "code/game.c", line: 4, was: line(4), edit: Edit::After("/* after */\nint more;") },
        ]),
        files: leak([
            ("project.toml", RECIPE),
            ("panel/index.html", "<!doctype html><p>tiny</p>\n"),
            ("src/glue.c", "/* DevCraft's */\nint glue;\n"),
        ]),
    });
    (setup, code, data)
}

fn mirror(data: Vec<u8>) -> Stand {
    Stand::default()
        .redirect(DATA_URL, "https://cdn3.mirrors.example.net/tiny/game-data.tar.gz")
        .file("https://cdn3.mirrors.example.net/tiny/game-data.tar.gz", data)
}

#[test]
fn a_setup_downloads_checks_and_writes_an_untrusted_project_of_the_files_named() {
    let t = TempDir::new("setup-made");
    let at = places(&t);
    let (setup, code, data) = tiny();
    let stand = mirror(data).file(CODE_URL, code);
    let steps = Mutex::new(Vec::new());
    let said =
        run(&at, setup, &setup.data[0], &stand, &|step| steps.lock().unwrap().push(matches!(step, Step::Checking)), &AtomicBool::new(false)).unwrap();
    assert!(said.contains("Tiny Game is in your projects, not trusted"), "{said}");
    // Asked of the two addresses, and of the one the data's host handed it over to: no more.
    assert_eq!(stand.asked(), [CODE_URL, DATA_URL, "https://cdn3.mirrors.example.net/tiny/game-data.tar.gz"]);
    let steps = steps.lock().unwrap().clone();
    assert!(steps.first() == Some(&false) && steps.last() == Some(&true), "downloading, then checking");
    // The project: exactly the files kept and the files DevCraft writes, sound, and untrusted.
    let made = project::load(&at.projects.join("tiny-game"));
    assert!(made.is_sound(), "{:?}", made.problem);
    assert_eq!(made.files, ["LICENSE", "code/game.c", "panel/game.dat", "panel/index.html", "project.toml", "src/glue.c"]);
    assert_eq!(TrustStore::load(&at.trust).status(&made), Status::Untrusted);
    // The lines changed are the ones named; the rest is as it came.
    let game = std::fs::read_to_string(at.projects.join("tiny-game/code/game.c")).unwrap();
    assert_eq!(game, "/* the game */\nint ticks;\nint lives = 9; /* DevCraft's */\nvoid tick(void) { ticks++; }\n/* after */\nint more;\n");
    assert_eq!(std::fs::read(at.projects.join("tiny-game/panel/game.dat")).unwrap(), GAME_DATA);
    // Nothing is left where it was put together.
    assert!(!staging_dir(&at.settings).exists());
}

#[test]
fn nothing_is_kept_when_a_file_is_not_the_one_pinned() {
    let (setup, _, data) = tiny();
    let check = |code: Vec<u8>, data: Vec<u8>, why: &str| {
        let t = TempDir::new("setup-unlike");
        let at = places(&t);
        let stand = mirror(data).file(CODE_URL, code);
        let refused = run(&at, setup, &setup.data[0], &stand, &|_| {}, &AtomicBool::new(false)).unwrap_err();
        assert!(refused.contains(why), "{refused}");
        assert!(!at.projects.join("tiny-game").exists() && !staging_dir(&at.settings).join("tiny-game").exists());
    };
    // A file of the code that is not the one pinned (one byte other), and one that is missing.
    let changed =
        zip_of(&[("game-1/src/game.c", b"/* the game */\nint ticks;\nint lives = 4;\nvoid tick(void) { ticks++; }\n"), ("game-1/LICENSE", LICENCE)]);
    check(changed, data.clone(), "src/game.c from code.example.org is not the file DevCraft expects");
    check(zip_of(&[("game-1/src/game.c", GAME_C)]), data.clone(), "LICENSE is not in what came from code.example.org");
    // The data, whose whole archive is pinned: another archive is not unpacked at all.
    let (_, code, _) = tiny();
    let other = tar_gz_of(&[("game-data/game.dat", GAME_DATA), ("game-data/extra", b"x")]);
    check(code.clone(), other, "What came from data.example.org is not what DevCraft expects (its SHA-256 differs)");
    // What is not a zip at all.
    check(b"<html>not found</html>".to_vec(), data, "can't be opened as a zip");
}

#[test]
fn a_download_goes_only_where_the_question_named_and_takes_no_more_than_was_pinned() {
    let (setup, code, data) = tiny();
    let fetch = |stand: &Stand, source: &Source| download(stand, source, &|_| {}, &AtomicBool::new(false));
    let source = &setup.data[0].source;
    // Handed over to a host the question did not name, or to one that is not HTTPS.
    for elsewhere in [
        "https://mirrors.example.net.evil.org/x",
        "https://example.net/x",
        "http://cdn3.mirrors.example.net/x",
        "https://user@cdn3.mirrors.example.net/x",
    ] {
        let stand = Stand::default().redirect(DATA_URL, elsewhere);
        assert!(fetch(&stand, source).is_err(), "{elsewhere}");
        assert_eq!(stand.asked(), [DATA_URL], "{elsewhere}: nothing was asked of it");
    }
    let said = fetch(&Stand::default().redirect(DATA_URL, "https://example.net/x"), source).unwrap_err();
    assert_eq!(said, "data.example.org handed the download over to example.net, which the question did not name: nothing was taken from it.");
    // A host handed over to may hand it on, but not for ever.
    let mut round = Stand::default().redirect(DATA_URL, "https://a.mirrors.example.net/1");
    for n in 1..=MAX_HAND_OVERS + 1 {
        round = round.redirect(&format!("https://a.mirrors.example.net/{n}"), &format!("https://a.mirrors.example.net/{}", n + 1));
    }
    assert!(fetch(&round, source).unwrap_err().contains("more than 5 times"));
    // An answer that is not the file.
    assert_eq!(
        fetch(&Stand::default().answer(CODE_URL, 404, Vec::new(), None), &setup.code).unwrap_err(),
        "code.example.org answered 404, not the file: nothing was taken."
    );
    // More than the limit: refused before reading when the length says so, and stopped when it does not.
    let most = limit(source) as usize;
    let long = Stand::default().answer(DATA_URL, 200, vec![0; most + 1], Some(most as u64 + 1));
    assert!(fetch(&long, source).unwrap_err().contains("offered more than DevCraft expects"));
    let unsaid = Stand::default().answer(DATA_URL, 200, vec![0; most + 1], None);
    assert!(fetch(&unsaid, source).unwrap_err().contains("sent more than DevCraft expects"));
    // A zip whose bytes GitHub may change may be up to half as large again; data whose hash is pinned, not a byte more.
    assert_eq!((limit(&setup.code), limit(source)), (code.len() as u64 * 3 / 2, data.len() as u64));
    // Stopped by the person: nothing more is asked.
    let stop = AtomicBool::new(true);
    assert_eq!(download(&Stand::default().file(CODE_URL, code), &setup.code, &|_| {}, &stop).unwrap_err(), STOPPED);
    // An address that is not HTTPS is never asked.
    let plain = Source {
        what: "Plain",
        whose: "Nobody's.",
        url: "http://code.example.org/game.zip",
        hands_over_to: &[],
        bytes: 1,
        sha256: None,
        archive: Archive::Zip,
        root: "",
        keep: &[],
    };
    let stand = Stand::default();
    assert!(fetch(&stand, &plain).is_err() && stand.asked().is_empty());
}

#[test]
fn a_line_is_changed_only_where_its_number_and_hash_say() {
    let file = |text: &str| vec![("code/a.c".to_string(), text.as_bytes().to_vec())];
    let change = |line: usize, was: &'static str, edit: Edit| Change { file: "code/a.c", line, was, edit };
    // Replaced, keeping the line's own ending; put after the last line, which had none.
    let mut files = file("one\r\ntwo\r\nthree");
    apply(&mut files, &[change(2, hash(b"two"), Edit::Replace("TWO")), change(3, hash(b"three"), Edit::After("four"))]).unwrap();
    assert_eq!(String::from_utf8(files[0].1.clone()).unwrap(), "one\r\nTWO\r\nthree\nfour\n");
    // Changes to one file are made from its last line to its first, so each number is the release's.
    let mut files = file("a\nb\nc\n");
    apply(&mut files, &[change(1, hash(b"a"), Edit::After("x\ny")), change(3, hash(b"c"), Edit::Replace("C"))]).unwrap();
    assert_eq!(String::from_utf8(files[0].1.clone()).unwrap(), "a\nx\ny\nb\nC\n");
    // Another line than the one pinned, a line past the end, line 0, a file not there: refused.
    for wrong in [change(2, hash(b"a"), Edit::Replace("!")), change(9, hash(b"a"), Edit::Replace("!")), change(0, hash(b"a"), Edit::Replace("!"))] {
        assert!(apply(&mut file("a\nb\n"), &[wrong]).unwrap_err().contains("is not the line DevCraft changes"));
    }
    let elsewhere = Change { file: "code/b.c", line: 1, was: hash(b"a"), edit: Edit::Replace("!") };
    assert!(apply(&mut file("a\n"), &[elsewhere]).is_err());
}

#[test]
fn a_folder_that_is_there_is_never_written_over_and_nothing_is_downloaded_for_it() {
    let t = TempDir::new("setup-there");
    let at = places(&t);
    t.write("projects/tiny-game/mine.txt", "mine");
    let (setup, code, data) = tiny();
    let stand = mirror(data).file(CODE_URL, code);
    let refused = run(&at, setup, &setup.data[0], &stand, &|_| {}, &AtomicBool::new(false)).unwrap_err();
    assert_eq!(refused, "There is already a project folder named tiny-game: nothing was downloaded.");
    assert!(stand.asked().is_empty());
    assert_eq!(std::fs::read_to_string(at.projects.join("tiny-game/mine.txt")).unwrap(), "mine");
    // Nor when it was made while the downloads ran.
    let files = vec![("project.toml".to_string(), RECIPE.as_bytes().to_vec())];
    assert!(write(&at, setup, &files).unwrap_err().contains("already a project folder named tiny-game"));
}

/// A transport whose answers wait until it is let go.
struct Held {
    inner: Stand,
    go: Arc<(Mutex<bool>, Condvar)>,
}

impl Transport for Held {
    fn get(&self, url: &str) -> Result<Reply, String> {
        let (gate, turn) = &*self.go;
        let mut open = gate.lock().unwrap();
        while !*open {
            open = turn.wait(open).unwrap();
        }
        self.inner.get(url)
    }
}

#[test]
fn a_setup_asked_for_runs_beside_the_window_once_and_may_be_stopped() {
    let t = TempDir::new("setup-runner");
    let at = places(&t);
    let (setup, code, data) = tiny();
    let runner = Runner::new();
    assert_eq!(runner.progress(&at.projects, "tiny-game"), None, "none was asked for");
    let go = Arc::new((Mutex::new(false), Condvar::new()));
    let held = Arc::new(Held { inner: mirror(data.clone()).file(CODE_URL, code.clone()), go: go.clone() });
    let told = Arc::new(AtomicUsize::new(0));
    let counted = told.clone();
    runner
        .start_with(&at, setup, &setup.data[0], held.clone(), move || {
            counted.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
    assert_eq!(runner.running(), 1);
    assert!(matches!(runner.progress(&at.projects, "tiny-game"), Some(Progress::Fetching { ref from, got: 0, .. }) if from == "code.example.org"));
    assert_eq!(runner.start_with(&at, setup, &setup.data[0], held, || {}).unwrap_err(), "Tiny is being set up already.");
    // Stopped by the person: it ends, and keeps nothing.
    assert!(runner.stop(&at.projects, "tiny-game"));
    *go.0.lock().unwrap() = true;
    go.1.notify_all();
    let wait = |told: &AtomicUsize, times: usize| {
        for _ in 0..3000 {
            if told.load(Ordering::SeqCst) >= times {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    };
    wait(&told, 1);
    assert_eq!((told.load(Ordering::SeqCst), runner.running()), (1, 0));
    assert_eq!(runner.progress(&at.projects, "tiny-game"), Some(Progress::Done(Err(STOPPED.to_string()))));
    assert!(!at.projects.join("tiny-game").exists() && !runner.stop(&at.projects, "tiny-game"));
    // Asked again, it runs to its end.
    let again = told.clone();
    runner
        .start_with(&at, setup, &setup.data[0], Arc::new(mirror(data).file(CODE_URL, code)), move || {
            again.fetch_add(1, Ordering::SeqCst);
        })
        .unwrap();
    wait(&told, 2);
    let Some(Progress::Done(Ok(said))) = runner.progress(&at.projects, "tiny-game") else { panic!("it has ended well") };
    assert!(said.contains("Tiny Game is in your projects"), "{said}");
    assert!(runner.progress(&t.0.join("elsewhere"), "tiny-game").is_none(), "another folder's project is another project");
}

#[test]
fn every_setup_names_its_files_once_where_a_project_may_hold_them() {
    assert_eq!(ALL.map(|setup| setup.id), ["gridcraft-doom", "gridcraft-quake", "gridcraft-glquake"]);
    assert_eq!(DOOM.data.iter().map(|data| data.key).collect::<Vec<_>>(), ["shareware", "freedoom"], "the shareware first");
    for setup in ALL {
        assert!(manifest::valid_id(setup.id) && named(setup.id).is_some_and(|found| std::ptr::eq(found, setup)));
        for data in setup.data {
            let sources = [&setup.code, &data.source];
            let mut paths: Vec<&str> = setup.files.iter().map(|(path, _)| *path).collect();
            for source in sources {
                assert!(https_host(source.url).is_some(), "{}", source.url);
                assert!(source.bytes > 0 && !source.keep.is_empty());
                for keep in source.keep {
                    assert!(
                        keep.bytes > 0 && keep.sha256.len() == 64 && keep.sha256.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()),
                        "{}",
                        keep.from
                    );
                    paths.push(keep.to);
                }
                if let Some(whole) = source.sha256 {
                    assert_eq!(whole.len(), 64);
                }
            }
            for path in &paths {
                assert_eq!(manifest::inner_path(path).as_deref(), Ok(*path));
            }
            let all = paths.len();
            paths.sort_unstable();
            paths.dedup();
            assert_eq!(paths.len(), all, "{}: no path twice", setup.id);
            // A line changed is in a file kept of the game's code.
            for change in setup.changes {
                assert!(setup.code.keep.iter().any(|keep| keep.to == change.file) && change.line > 0 && change.was.len() == 64, "{}", change.file);
            }
        }
    }
    // What each keeps of its code, by count.
    assert_eq!(ALL.map(|setup| setup.code.keep.len()), [178, 113, 89]);
}

#[test]
fn the_question_names_every_address_every_size_and_what_is_kept() {
    for setup in ALL {
        for data in setup.data {
            let asked = question(setup, data);
            for source in [&setup.code, &data.source] {
                for said in [source.url.to_string(), size_says(source), keeps_says(source), source.whose.to_string()] {
                    assert!(asked.contains(&said), "{}: {said}", setup.id);
                }
            }
            assert!(asked.contains("not trusted") && asked.contains(setup.id), "{asked}");
        }
    }
    let freedoom = question(&DOOM, DOOM.data_named("freedoom").unwrap());
    assert!(freedoom.contains("github.com may hand it over from another of its servers (*.githubusercontent.com); from nowhere else."), "{freedoom}");
    let quake = question(&QUAKE, &QUAKE.data[0]);
    assert!(quake.contains("9,050,608 bytes") && quake.contains("PAK0.PAK (18,689,235 bytes) and SLICNSE.TXT (10,036 bytes)"), "{quake}");
    // What is kept is named as it is in what comes, not as the project names it.
    assert!(freedoom.contains("It keeps freedoom1.wad (28,795,076 bytes) and COPYING.txt (1,644 bytes) of it."), "{freedoom}");
    assert!(quake.contains("about 0.4 MB, and no more than 0.6 MB is taken") && quake.contains("113 files, 1,051,844 bytes"), "{quake}");
    assert!(quake.contains("archive.org may hand it over from another of its servers (*.archive.org)"), "{quake}");
}

#[test]
fn what_devcraft_writes_makes_a_sound_project_of_the_c_kit_for_each_game() {
    for setup in ALL {
        for data in setup.data {
            let t = TempDir::new("setup-glue");
            // The files kept stand in as placeholders: what is checked is the recipe and the paths.
            for (path, text) in setup.files {
                t.write(&format!("projects/{}/{path}", setup.id), text);
            }
            for keep in setup.code.keep.iter().chain(data.source.keep) {
                t.write(&format!("projects/{}/{}", setup.id, keep.to), "/* stands in */\n");
            }
            let made = project::load(&t.0.join("projects").join(setup.id));
            assert!(made.is_sound(), "{}: {:?}", setup.id, made.problem);
            assert_eq!((made.name(), made.manifest.as_ref().map(|m| m.version.as_str())), (setup.name, Some("0.4.0")));
            let manifest = made.manifest.as_ref().unwrap();
            assert_eq!(manifest.kit, "c");
            let recipe = manifest.c.as_ref().unwrap();
            assert!(recipe.sources_among(&made.files).unwrap().iter().any(|source| source.starts_with("src/")), "{}", setup.id);
            // The game's data is kept out of any repository the project is put in.
            let ignored = setup.files.iter().find(|(path, _)| *path == ".gitignore").map(|(_, text)| *text).unwrap();
            assert!(data.source.keep.iter().filter(|keep| keep.to.starts_with("panel/")).all(|keep| ignored.contains(keep.to)), "{}", setup.id);
        }
    }
    // Only GLQuake asks for the network, and names where.
    assert_eq!(
        ALL.map(|setup| setup.files.iter().any(|(path, text)| *path == "project.toml" && text.contains("\"network\"]"))),
        [false, false, true]
    );
    // What DevCraft writes says whose it is, and that nothing of id Software's code is in it.
    for setup in ALL {
        for (path, text) in setup.files.iter().filter(|(path, _)| path.ends_with(".c") || path.ends_with(".js")) {
            assert!(text.contains("DevCraft's licence (MIT OR Apache-2.0)"), "{}: {path}", setup.id);
        }
    }
}

/// The lines of C that say something: comments, blank lines and lone braces left out, and the
/// rest without its spaces. A block copied from another file shows as a run of these.
fn significant(text: &str) -> Vec<String> {
    text.lines()
        .map(|line| line.split("//").next().unwrap_or("").split_whitespace().collect::<String>())
        .filter(|line| !line.is_empty() && !line.starts_with("/*") && !line.starts_with('*') && !line.chars().all(|c| "{}();,".contains(c)))
        .filter(|line| !matches!(line.as_str(), "else" | "break;" | "return;"))
        .collect()
}

/// Where the real downloads are, for the test that uses them: `GRIDCRAFT_TEST_SETUPS` names a
/// folder that holds each one under its address with every character but letters, digits, `.`
/// and `-` made `_` (as `https___codeload.github.com_ozkl_doomgeneric_zip_e257a09…`).
fn real_downloads() -> Option<PathBuf> {
    std::env::var_os("GRIDCRAFT_TEST_SETUPS").map(PathBuf::from).filter(|dir| dir.is_dir())
}

struct Folder(PathBuf);

impl Transport for Folder {
    fn get(&self, url: &str) -> Result<Reply, String> {
        let name: String = url.chars().map(|c| if c.is_ascii_alphanumeric() || c == '.' || c == '-' { c } else { '_' }).collect();
        let bytes = std::fs::read(self.0.join(&name)).map_err(|e| format!("{name}: {e}"))?;
        Ok(Reply { status: 200, location: None, length: Some(bytes.len() as u64), body: Box::new(Cursor::new(bytes)) })
    }
}

/// With the real downloads (where a folder of them is named): each game is put together, every
/// pin holds, no run of three lines of what DevCraft wrote is a run of the game's code, and, where
/// `GRIDCRAFT_TEST_ZIG` names a zig, the kit builds it.
#[test]
fn with_the_real_downloads_each_game_is_put_together_holds_nothing_copied_and_builds() {
    let Some(dir) = real_downloads() else { return };
    let zig = std::env::var("GRIDCRAFT_TEST_ZIG").ok().filter(|zig| Path::new(zig).is_file());
    for setup in ALL {
        for data in setup.data {
            let t = TempDir::new("setup-real");
            let at = places(&t);
            let said = run(&at, setup, data, &Folder(dir.clone()), &|_| {}, &AtomicBool::new(false)).unwrap();
            assert!(said.contains("not trusted"), "{said}");
            let root = at.projects.join(setup.id);
            // No run of three lines of DevCraft's C is a run of three in any of the game's files.
            let theirs: Vec<String> = setup
                .code
                .keep
                .iter()
                .filter(|keep| keep.to.ends_with(".c") || keep.to.ends_with(".h"))
                .map(|keep| String::from_utf8_lossy(&std::fs::read(root.join(keep.to)).unwrap()).into_owned())
                .collect();
            let mut runs = HashSet::new();
            for file in &theirs {
                for three in significant(file).windows(3) {
                    runs.insert(three.join("\n"));
                }
            }
            for (path, text) in setup.files.iter().filter(|(path, _)| path.ends_with(".c") || path.ends_with(".h")) {
                let ours = significant(text);
                let copied: Vec<String> = ours.windows(3).map(|three| three.join("\n")).filter(|three| runs.contains(three)).collect();
                assert!(copied.is_empty(), "{}: {path} has a run of the game's own lines: {copied:?}", setup.id);
            }
            // Built by the kit, as the person would once they trust it.
            let Some(zig) = &zig else { continue };
            let found = project::load(&root);
            TrustStore::load(&at.trust).trust(&found, false).unwrap();
            let built = crate::c::build(&at, setup.id, zig).unwrap();
            assert!(built.program.is_file() && built.bytes > 100_000, "{}: {built:?}", setup.id);
            eprintln!("{} with {}: built from {} C files, {} bytes, in {} ms", setup.id, data.key, built.sources, built.bytes, built.millis);
        }
    }
}
