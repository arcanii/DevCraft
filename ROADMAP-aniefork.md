# GridCraft aniefork roadmap

This is the fork's own record: [arcanii/gridcraft](https://github.com/arcanii/gridcraft), a fork of
[storytold/gridcraft](https://github.com/storytold/gridcraft). The sister forks
[arcanii/wordcraft](https://github.com/arcanii/wordcraft) and
[arcanii/pdfcraft](https://github.com/arcanii/pdfcraft) keep the same kind of file, and all three
follow the same rules ([`AGENTS-aniefork.md`](AGENTS-aniefork.md)).

Upstream's [ROADMAP.md](ROADMAP.md), `docs/`, `CLAUDE.md` and `AGENTS.md` are kept **identical to
upstream** here, so merging upstream doesn't conflict on bookkeeping. Work done in the fork is
logged in this file instead.

- **Fork work:** add a line to the log below, and keep the tables true. Do not edit `ROADMAP.md`
  or upstream's `docs/`.
- **A branch meant as an upstream pull request:** follow upstream's rules there (`CLAUDE.md`),
  including its progress documents. Cut the branch from `upstream/main`, not from the fork's `main`.

## Sync status

| | |
|---|---|
| Fork `main` is based on | upstream `13664fe` (GridCraft 0.4.0), 2026-10-09 |
| Last merge of upstream | 2026-10-12, upstream `3926d21` (GridCraft 0.5.0 and 352 commits after it) |
| Fork is behind upstream by | 0 commits at that merge |
| Fork carries | 3 changes upstream lacks (below) |

## What the fork carries

Changes in the fork that upstream does not have. Update the last column when one is offered or
merged; delete the row once a merge of upstream brings it back.

| Change | Where | Upstream |
|---|---|---|
| **Two settings on upstream's Options page:** the user name (the author of comments and notes) and whether the ribbon shows the Developer tab, with a command for each (`app.userName`, `developer.tab`). | `crates/ui-egui/src/aniefork/options.rs`, a hook in `backstage.rs` | Not offered |
| **The Developer tab**, hidden until Options turns it on: the entry to the Development subsystem designed in [ADR-0001](docs/decisions-aniefork.md). | `crates/ui-egui/src/aniefork/developer.rs`, hooks in `ribbon.rs` | Not offered; a direction of the fork's own |
| **Modules, trust, Python and panels** (the Development subsystem, phases 1a to 1c): modules are folders the user installs; each has a manifest saying what it asks to be allowed and a fingerprint of its contents; the user trusts a module or not, in Developer ▸ Modules, confirmed by the operating system; a trusted module's Python scripts run in a process of their own, and its panel opens in the browser and calls GridCraft over a local bridge. | The crate `crates/aniefork-dev`; `crates/ui-egui/src/aniefork/` (`modules.rs`, `panels.rs`, `presence.rs`); hooks in `xtask/src/layers.rs`, `crates/ui-egui/Cargo.toml`, `crates/ui-egui/src/lib.rs`, `dialogs.rs` and `apps/gridcraft/src/main.rs` | Not offered; fork-only until proven (ADR-0001) |

## What upstream took in, and what it built itself

The fork no longer carries these. They are kept here as the record of what happened to them.

| Change | What happened |
|---|---|
| **The active cell stays clear inside a selection** | Offered as [storytold/gridcraft#265](https://github.com/storytold/gridcraft/pull/265) and **merged** on 2026-10-11. The fork's copy (`fill_rects` and its hook in `grid.rs`) was deleted at the merge of 2026-10-12. |
| **Selection glide**, with View ▸ Animations | Offered as [storytold/gridcraft#266](https://github.com/storytold/gridcraft/pull/266) and **merged** on 2026-10-11. Upstream merged its `main` into the branch first, so the glide respects frozen-pane clipping and right-to-left sheets, and added Spanish, German and Czech strings. The fork's copy (`aniefork/selection.rs`, the `view.animations` command, the hooks in `grid.rs`, `control.rs`, `lib.rs` and `ribbon.rs`, and the `GRIDCRAFT_SELECTION_GLIDE_MS` override) was deleted at the merge. |
| **The File page** (Backstage view) | Offered as [storytold/gridcraft#337](https://github.com/storytold/gridcraft/pull/337) and **closed as a duplicate**: another contributor's #331 had added the same view, with more pages (Save As, Print, Options), translations and a control-channel method, and was merged minutes earlier. The fork's page (`aniefork/backstage.rs` and its hooks) was deleted at the merge; upstream's is used. The maintainers asked for one part of ours as a follow-up: see Open below. |
| **File ▸ Options** as a dialog, and a `file.options` command | Upstream now has an Options page in its File view and a `file.options` command that opens it. The fork's dialog and command were retired at the merge; its two settings moved onto upstream's page. |

### Open
- **A follow-up pull request the maintainers invited:** the recent list's "modified" column, read
  on a thread of its own so a slow drive never holds up the page, on top of upstream's File page.
  Cut it from `upstream/main`.
- Following the system's reduced-motion setting, and measuring Excel's glide, are upstream's to
  take up now; a change would be a pull request there.

## The fork's settings on the Options page

Upstream's Options page (File ▸ Options) holds the display theme, the interface language, the
formula bar, AutoSave and the calculation mode. The fork adds two settings at the end of its
General section, applied at once like upstream's own:

- **User name**: the author of the comments and notes the user adds. It is set when the field is
  left. `app.userName {name?}` is the command: without a name it only reports.
- **Show the Developer tab**. `developer.tab {on?}` is the command: without `on` it toggles.

A value that makes no sense is an error and changes nothing, and neither command opens anything.
Animations, which the fork's old dialog also held, is upstream's setting now (View ▸ Animations).

### Open
- The rest of what Excel's Options holds, as the features behind it come to exist: the direction
  Enter moves, the default font and sheets of a new workbook, R1C1 references.
- Translations of the fork's two labels on the page.

## The Developer tab and the Development subsystem

A ribbon tab for people who extend and automate GridCraft, hidden until File ▸ Options ▸ "Show the
Developer tab" turns it on, as Excel's is. Today it gathers what already exists for that audience:
the command palette, the action journal, how agents connect, and Options.

It is the entry to the **Development subsystem**: modules a user installs that add web panels and
Python to GridCraft. That is designed in [`docs/decisions-aniefork.md`](docs/decisions-aniefork.md)
(ADR-0001), which the owner **accepted on 2026-10-11**: panels in the browser first and docked
later, the user's own Python, fork-only until it is proven and then offered upstream in phases.
Whether a workbook can carry a module is left open, and letting agents use modules is on the
backlog. The record also sets out how the model is exposed to modules and how the user gives or
refuses trust; those two are still proposals.

**Phase 1a is built: modules and trust, with nothing running.**

- **A module is a folder** in `modules`, beside the user's settings, holding a `module.toml` (id,
  name, version, what it asks to be allowed), a panel and scripts. Developer ▸ Modules lists what
  is there; "New module" writes a starter from templates compiled into the app.
- **Every module starts untrusted, and an untrusted module is inert.** Review… shows what it asks
  for in plain words (running Python is marked as the one that cannot be limited), its files and
  its fingerprint; then the user trusts it or doesn't.
- **Trust is given to a fingerprint**, a SHA-256 over every file in the module. A changed file
  makes it untrusted again. For a module they are writing, the user can choose to keep trusting it
  as it changes; that never grants more than was granted.
- **Only the user gives trust.** There is no command for it. `developer.modules` reports the list
  to agents and nothing else; the dialog has no OK command for an agent to confirm; and what is
  trusted is the fingerprint that was on screen, so files swapped during the review are refused.
- **It fails closed.** A module that can't be read whole (no manifest, a path that leaves it, a
  link, an unknown permission, too large) can't be trusted. A damaged trust file trusts nothing.
- The decisions are in `trusted-modules.json` beside the settings, and a run without settings
  (the web build, `GRIDCRAFT_NO_PREFS`) has no modules.

**Phase 1b is built: a trusted module's Python scripts run.**

- **A script is a file in the module's `python` folder with a `main(data)` function.** GridCraft
  starts the user's own Python (the one named in Developer ▸ Modules, or the first found) with a
  small runner. The process stays alive between calls, so a module's imports are paid for once,
  and is started again when the module's files change. Each module has its own.
- **Nothing runs without trust.** Every call reads the module and the user's decisions from disk
  and refuses, before any process exists, a module that is untrusted, changed since it was
  trusted or not granted `python`, and a script that is not one of the module's own files.
- **The process protects GridCraft from the script.** A script that raises is reported with its
  traceback; one that runs past its time limit (60 s) is stopped; one that dies costs only its
  process; what it prints can't garble its reply. Scripts run off the UI thread and can be
  stopped. It does not limit what the script does to the computer: that is what trust is for.
- **Giving trust now needs the operating system's own confirmation**, a native yes/no dialog that
  states the module's name, everything it will be allowed and how many files it has. Synthetic
  clicks from the control channel can't answer it, and a build without one can't trust modules.
- **Developer ▸ Modules runs a script by hand**: "Run on the selection" hands the script the
  selected cells as `{range, sheet, rows}` and shows what it returned, what it printed, or why it
  failed. The sheet is not changed: writing back belongs to panels, in phase 1c. The dialog also
  says which Python is used and lets the user name another, and opens the modules folder.

**Phase 1c is built: a trusted module's panel opens in the browser and calls GridCraft.**

- **Open Panel**, on a trusted module in Developer ▸ Modules, serves the module's panel folder
  from a small server inside GridCraft and opens it in the user's browser. The server listens on
  the loopback address only, on a port of the module's own, behind a secret token made for that
  run. The address goes to the browser and is shown nowhere else.
- **The panel gets a `spreadsheet` object** from `spreadsheet.js`, served beside its page:
  `run(id, params)` for every command of the engine, `batch([...])` for several commands as one
  undo step (all of them, or none), `runPython(script, data)`, and `selection()`, `read()` and
  `write()` as shorthands. `spreadsheet.d.ts`, served beside it, lists every command for
  TypeScript. The starter module's panel now does the whole round: the selection, through its
  Python script, back into the sheet, undone with one Undo.
- **A module runs only what it was granted.** A command that only reports needs `read`, any
  other `write`, one that touches a file `files`. GridCraft's own settings folder is out of
  reach even with `files`. A panel without `network` is served with a policy that makes the
  browser refuse its requests to anywhere else.
- **Trust is checked on every request.** A module whose files changed, or whose trust was taken
  back, is served and answered nothing more, at once, with the panel still open.
- **Commands never open dialogs for a module**, and the window's own commands (the theme, the
  dialogs) are not offered to it: it is a program calling GridCraft, as an agent is.
- **Some things are for the person at the screen alone.** Trusting a module, running a script,
  opening a panel and naming the Python to run are refused while the control channel is putting
  input into the window, and the Modules dialog no longer works from values the channel can
  rewrite. This closes a gap phase 1b left: an agent could name any program as "Python" and
  press Check Python to run it.

The crate depends on nothing else in GridCraft, so it can be offered upstream as pieces of its
own. Next is phase 2: panels docked in the window, the workbook as objects, events.

### Open
- **`network` is only partly enforced in a browser.** The policy stops a panel's requests, but
  not the page sending the user to another address, which can carry data. Docking the panel in
  the window (phase 2) is what closes this.
- The workbook as objects is not built: a panel works through commands.
- A script's process is not confined, and stopping it stops only the Python GridCraft started,
  not processes the script started itself.
- The trust check hashes the whole module on every request. Fine for small modules; a large one
  will want the fingerprint cached against its files' times.
- A call that arrives while GridCraft shows a system dialog waits, and is given up after 30
  seconds.
- Wording of the fork's dialogs in other languages.

## Hooks in upstream files

Everything else the fork adds is in files of its own. After merging upstream, check these are
still there:

| File | Hook |
|---|---|
| `crates/ui-egui/src/lib.rs` | `pub mod aniefork;` |
| `crates/ui-egui/src/lib.rs`, `UiState` and its `Default` | the field `developer_tab: bool` (default `false`), after `animations` |
| `crates/ui-egui/src/lib.rs`, `Services` | the field `confirm`: a yes/no question asked by the operating system, which the UI can't answer for the user |
| `crates/ui-egui/src/lib.rs`, `run_ui_command`, first thing | `aniefork::run_command(self, id, p)` runs the fork's UI commands and returns early |
| `crates/ui-egui/src/lib.rs`, `logic`, right after `control::poll(self, ctx);` | `aniefork::poll(self, ctx);` does the fork's work of every frame: it notes whether the control channel is putting input into the window, and carries out the calls modules' panels made |
| `crates/ui-egui/src/control.rs`, `handle`, `engine.commands` | after upstream's UI commands are listed and before the listing is filtered: `v.extend(crate::aniefork::commands());` |
| `crates/ui-egui/src/ribbon.rs`, `show`, after `tabs` is made from `TABS` | `crate::aniefork::developer::tabs(app, &mut tabs);` adds the Developer tab when it is on |
| `crates/ui-egui/src/ribbon.rs`, `show`, the match on the active tab | the arm `"Developer" => crate::aniefork::developer::ribbon(app, ui)` |
| `crates/ui-egui/src/backstage.rs`, `options`, after the formula bar checkbox | `crate::aniefork::options_page(app, ui);` draws the fork's two settings |
| `crates/ui-egui/src/dialogs.rs`, `Dialog::open`, the last arm of the match (upstream's is `_ => return None`) | `name => …` asks `crate::aniefork::dialog(app, name, &params)?` for a title, starting values and a command, and builds a custom dialog from them |
| `crates/ui-egui/src/dialogs.rs`, `show`, before the `_ => form(…)` arm | `_ if crate::aniefork::draws_dialog(&d.name) => crate::aniefork::dialog_ui(…)` draws any of the fork's dialogs |
| `crates/ui-egui/Cargo.toml`, `[dependencies]` | `gridcraft-aniefork-dev = { path = "../aniefork-dev" }` |
| `apps/gridcraft/src/main.rs`, `services()` | `confirm` is a native message dialog (`rfd::MessageDialog`, Yes/No) |
| `xtask/src/layers.rs`, `TABLE` and `INTRA_LAYER_ORDER` | the crate `aniefork-dev` at layer 6, and the chain `["aniefork-dev", "ui-egui"]` so the UI crate may use it |
| `Cargo.lock` | the new crate's entry, with `getrandom` among its dependencies. When upstream changes dependencies, take upstream's file and let `cargo` add the entry back |

The fork changes no line of `grid.rs` any more.

## Syncing with upstream

```sh
git fetch upstream
git merge upstream/main
```

To look before merging (none of this touches the working tree):

```sh
git fetch upstream
git log --oneline main..upstream/main
git diff --stat main...upstream/main -- crates/ui-egui/src/lib.rs crates/ui-egui/src/grid.rs crates/ui-egui/src/control.rs crates/ui-egui/src/ribbon.rs
git merge-tree --write-tree --name-only main upstream/main
```

The log is what is new, the diff shows whether upstream changed the files that hold the hooks, and
the last command lists the files a merge would conflict in (it exits 1 when there are any).

**Backlog:** a scheduled check that does this, runs the gate on a trial merge in a separate folder
and writes a report, either on the workstation through the Claude app or as a GitHub workflow in
the fork. Decided 2026-10-11: check by hand for now.

Sync often: upstream lands many commits a day, and small merges are easy. What can still conflict:

- **The hooks above**, when upstream edits the lines around them. Take upstream's code and put
  the hook back. The 0.5.0 merge conflicted at three: the module list in `lib.rs` (both sides
  added modules), the command listing in `control.rs` (upstream now filters it) and the View tab
  in `ribbon.rs` (upstream replaced the Dark Mode checkbox with a Display theme picker). The hooks
  in `grid.rs`, `UiState` and `run_ui_command` merged by themselves.
- **What the hooks hand to upstream's code**, which no conflict flags. `aniefork::commands()`
  must list commands in the shape upstream's listing uses (0.5.0 added `params`), and
  `aniefork::tr` covers the interface languages upstream has: when upstream adds one, the fork's
  strings show in English there until they are added to that table.
- **`ATTRIBUTION.md`:** when both sides add assets (the fork has none so far).

After a merge, run `cargo xtask ci` before committing it, and update the tables above.

## Building

The workspace says `rust-version = "1.90"`, but egui 0.36 needs Rust 1.95 or newer; an older
toolchain fails with a dependency error (pdfcraft's fork found the same and carries a fix).
`cargo xtask ci` also needs the `wasm32-unknown-unknown` target.

On Windows the gate's last step fails on an untouched checkout: `cargo xtask parity --check`
compares the generated text with `docs/parity-checklist.md` (`docs/parity.md` before 0.5.0) byte
for byte, and with `core.autocrlf=true` the checked-out file has CRLF line endings. It is not a
sign that the catalog changed. The other steps (fmt, clippy, tests, assets, layers, wasm) are
meaningful there.

As merged on 2026-10-12, upstream's `main` (`3926d21`) fails three of its own tests on this
machine, in files the fork doesn't touch, on an untouched checkout as well: two in the engine
(`csv_save_keeps_the_separator_it_was_opened_with` and
`saving_an_opened_semicolon_csv_keeps_semicolons`: a twelve-digit number is saved to CSV as
`1.24412E+11`) and one in the UI crate (`translations_are_unique_and_preserve_unknown_text`: the
French catalog lacks "Different First Page"). They are upstream's to fix.

## Log

Newest first.

- **2026-10-12 (merge of upstream, 352 commits):** Upstream had merged the clear active cell
  (#265) and the selection glide (#266), and had built its own File page with an Options page
  (#331), closing ours (#337) as a duplicate. At the merge the fork's copies of all four were
  deleted and upstream's are used: `aniefork/selection.rs` and `aniefork/backstage.rs` are gone,
  the fork's `view.animations` and `file.options` commands are retired, and its Options dialog
  became two settings on upstream's Options page (the user name, and Show the Developer tab),
  with the commands `app.userName` and `developer.tab`. The five conflicts were all at hooks;
  upstream's code was taken and the hooks the fork still needs were put back (the table above).
  The fork's footprint in upstream's files is now nine files and about fifty lines. After the
  merge the fork's own tests pass, and every step of the gate passes but for three tests that
  upstream's `main` fails by itself (see Building).
- **2026-10-11 (Development subsystem, phase 1c: panels):** A trusted module's panel opens in
  the browser, served by a small HTTP server inside GridCraft (loopback only, a port and a
  secret token for each module, trust checked on every request), and calls GridCraft through a
  `spreadsheet` object: every command of the engine, several commands as one undo step, its own
  Python scripts. What a module may run follows what it was granted. The starter's panel takes
  the selection through its Python script and back into the sheet as one undo step. Also: what
  runs code or decides who may is now refused while the control channel is putting input into
  the window, which closes a gap from phase 1b (an agent could name any program as "Python" and
  have it run). Checked by 24 new tests (14 in the crate, over real connections; 10 through the
  window) and with the starter's panel in a real browser.
- **2026-10-11 (the File page offered upstream):** Sent as pull request #337 from the branch
  `file-page`, cut from `upstream/main` and independent of #265 and #266. The upstream version
  lives in `crates/ui-egui/src/backstage.rs` with its window tests in `tests/backstage.rs`,
  replaces upstream's `file_menu` outright, has no Options… in its column, and updates
  `docs/control-protocol.md` and `docs/ui-parity.md`. Print and PDF export on the desktop are
  a feature of their own, not part of it. When it merges and upstream is merged back, delete
  `aniefork/backstage.rs` and its two hooks, and keep only what puts Options… in the column.
- **2026-10-11 (the File page):** The File button opens a page over the whole window instead of
  a dropdown menu: a column of pages and actions on the left, Home with the workbooks to start
  from and the recent ones (with how long ago each was changed), and Open, Info and Export. It
  runs upstream's file commands and keeps the menu's labels, so upstream's File menu tests pass
  against it untouched; to agents it is a dialog named `backstage`. The earlier hook that added
  Options… to upstream's menu is gone: Options… is in the page's column. Checked in headless
  renders (light and dark), in the real app, and by 12 tests.
- **2026-10-11 (Development subsystem, phase 1b: Python):** A trusted module's Python scripts
  run, each module in a Python process of its own that stays alive between calls: one line of
  JSON in, one out, `main(data)` in the script. Every call is checked against the user's
  decisions on disk first, so an untrusted or changed module, or a script that isn't the
  module's, starts nothing. A failing script is reported, a hung one is stopped at its time
  limit, a dead one costs only its process. Giving trust now ends in the operating system's own
  yes/no dialog, which the control channel cannot answer. Developer ▸ Modules runs a script on
  the selection and shows the result, names the Python in use, and opens the modules folder.
  Checked with the machine's Python 3.14 and by 13 new tests (8 in the crate, 5 through the
  dialog); the tests that need Python skip themselves where there is none.
- **2026-10-11 (Development subsystem, phase 1a: modules and trust):** The subsystem's first
  piece, a crate of its own (`crates/aniefork-dev`) that runs no code: module manifests,
  fingerprints over a module's whole contents, and the user's decisions to trust, kept by
  fingerprint. Developer ▸ Modules lists the user's modules, creates a starter, and is the only
  place trust is given or taken back; `developer.modules` reports the list. A changed file ends
  the trust; a module swapped during review is refused; broken modules and a damaged trust file
  fail closed. Checked in headless renders and by 26 tests (20 in the crate, 6 through the real
  dialog).
- **2026-10-11 (File ▸ Options, the Developer tab, ADR-0001):** A settings dialog, opened from the
  File menu: theme, language, user name, Animations, and whether the ribbon shows the Developer
  tab. `file.options` is its command, and agents can open, fill in and confirm the dialog. The
  Developer tab is hidden by default and holds the developer tools that exist today. The
  Development subsystem it is the entry to (modules with web panels and Python) is designed in
  `docs/decisions-aniefork.md`, which the owner accepted the same day with one point left open
  (modules carried in workbooks); none of the subsystem is built yet.
  Checked in headless renders and by nine tests, one of them through the real File menu.
- **2026-10-11 (offered upstream):** Both changes were sent to upstream as pull requests from
  branches cut from `upstream/main` and rewritten in upstream's style (no `aniefork` paths):
  #265, the clear active cell, with the contributor entry for arcanii; and #266, the selection
  glide with View › Animations, stacked on #265. The upstream version keeps the glide's state on
  the grid state, puts the setting's strings in upstream's five catalogs, updates
  `docs/control-protocol.md` and `docs/ui-parity.md`, and leaves out the
  `GRIDCRAFT_SELECTION_GLIDE_MS` override. When they merge and upstream is merged back, delete
  the fork's copies (the `aniefork` module's parts and their hooks) and the table rows.
- **2026-10-11 (merge of upstream 0.5.0):** Merged 222 upstream commits (`9f896c1`). Three files
  conflicted, each at a hook, and were settled by taking upstream's code and putting the hook
  back: `lib.rs` (module list), `control.rs` (command listing) and `ribbon.rs` (the View tab,
  where upstream's Display theme picker replaced the Dark Mode checkbox; Animations is now under
  the picker). Upstream's command listing gained a `params` field and its interface became
  translatable, so the fork's command documents its parameter and the Animations label and hint
  are worded in Japanese, Chinese, Korean, Russian and Portuguese in a table of the fork's own.
  Gate after the merge: fmt, clippy, 600 tests, assets, layers and wasm pass; `parity` fails on
  Windows line endings as before.
- **2026-10-11 (M1, Animations setting):** The glide can be turned off: View › Animations, a
  checkbox under Dark Mode, saved with the UI preferences and on by default. `view.animations
  {on?}` does the same for agents. The setting's logic is in the `aniefork` module; upstream's
  files get the saved field and three one-line hooks (the command, its listing, the checkbox).
  Checked by clicking the real checkbox in the headless app (off: the outline jumps; on again: it
  glides) and by three more tests.
- **2026-10-11 (M1, active cell in a selection):** The active cell inside a selected range is no
  longer shaded with the rest of the range (upstream's line for it painted a transparent
  rectangle). The fill now goes around it: in every area of the selection, around the whole merged
  cell when it is one, and following the gliding outline of the current area. In the `aniefork`
  module, behind the existing hook in the fill loop of `paint_selection`. Checked in headless
  renders (light, dark, several areas, a merged cell, and for seams with gridlines off at three
  zoom levels) and by three tests.
- **2026-10-11 (M1, selection glide):** The selection outline glides to a new selection instead of
  jumping, like Excel's on Windows: 0.12 s, decelerating, from wherever it is drawn (so a held
  arrow key reads as one motion), with its fill and fill handle; a range grows edge by edge.
  Scrolling, zoom, other sheets and off-screen moves don't glide, and control-channel screenshots
  and the headless harness always see it at rest. The marching ants now run by the clock. All of
  it is in the new `aniefork` module of `ui-egui`, with hooks in `grid.rs` and `control.rs`.
  Checked in a frame-by-frame headless render, in the desktop app with the glide slowed to 10 s,
  and by twelve tests. This file and `AGENTS-aniefork.md` were started.
