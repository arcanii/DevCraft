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
| Last merge of upstream | 2026-10-11, upstream `9f896c1` (GridCraft 0.5.0), 222 commits |
| Fork is behind upstream by | 0 commits at that merge |
| Fork carries | 6 changes upstream lacks (below) |

## What the fork carries

Changes in the fork that upstream does not have. Update the last column when one is offered or
merged; delete the row once a merge of upstream brings it back.

| Change | Where | Upstream |
|---|---|---|
| **Selection glide:** the selection outline slides to a new selection instead of jumping, like the cell cursor in Excel for Windows. First version; the timing is estimated, not yet measured against Excel. | `crates/ui-egui/src/aniefork/` (`selection.rs`; the Animations setting in `mod.rs`), hooks in `grid.rs`, `control.rs`, `lib.rs` and `ribbon.rs` (listed below) | Offered 2026-10-11 as [storytold/gridcraft#266](https://github.com/storytold/gridcraft/pull/266), stacked on #265. Branch `selection-glide`, rewritten in upstream's style |
| **The active cell stays clear inside a selection.** Upstream shades every cell of a selected range alike: its line for the clear cell paints a fully transparent rectangle, which changes nothing. The fill now leaves the active cell unshaded, so it is plain where typing will go. | `fill_rects` in `crates/ui-egui/src/aniefork/selection.rs`, one hook in `grid.rs` | Offered 2026-10-11 as [storytold/gridcraft#265](https://github.com/storytold/gridcraft/pull/265). Branch `active-cell-clear` |
| **File ▸ Options:** a settings dialog. Upstream has none (its catalog lists `file.options` as missing) and keeps its settings on the View tab. | `crates/ui-egui/src/aniefork/options.rs`, hooks in `dialogs.rs`; it opens from the File page | Not offered |
| **The File page:** the File button opens a page that fills the window (a column of pages and actions on the left; Home with the workbooks to start from and the recent ones) where upstream opens a dropdown menu. | `crates/ui-egui/src/aniefork/backstage.rs`, hooks in `ribbon.rs` and `dialogs.rs` | Offered 2026-10-11 as [storytold/gridcraft#337](https://github.com/storytold/gridcraft/pull/337). Branch `file-page`, rewritten in upstream's style, without Options… (the dialog is the fork's) |
| **The Developer tab**, hidden until Options turns it on: the entry to the Development subsystem designed in [ADR-0001](docs/decisions-aniefork.md). | `crates/ui-egui/src/aniefork/developer.rs`, hooks in `ribbon.rs` | Not offered; a direction of the fork's own |
| **Modules, trust and Python** (the Development subsystem, phases 1a and 1b): modules are folders the user installs; each has a manifest saying what it asks to be allowed and a fingerprint of its contents; the user trusts a module or not, in Developer ▸ Modules, confirmed by the operating system; a trusted module's Python scripts run in a process of their own. Panels are not served yet. | The crate `crates/aniefork-dev`; `crates/ui-egui/src/aniefork/modules.rs`; hooks in `xtask/src/layers.rs`, `crates/ui-egui/Cargo.toml`, `crates/ui-egui/src/lib.rs` and `apps/gridcraft/src/main.rs` | Not offered; fork-only until proven, then in phases ([ADR-0001](docs/decisions-aniefork.md)) |

## Selection glide

Excel for Windows doesn't move its cell cursor instantly: press an arrow key or click a cell and
the border slides to the new place; extend the selection and it grows. Upstream GridCraft jumps.
The fork glides. WordCraft's fork does the same for the text caret.

- **What glides:** the outline of the selection's current area, together with its fill and its
  fill handle, when the selection changes: arrows, Enter, Tab, Home/End, a click, a drag,
  Shift+arrows, the Name Box, Go To, any command. A new move takes over from wherever the outline
  is drawn, so a held arrow key reads as one continuous motion. A selection that grows or shrinks
  moves edge by edge; the anchored corner stays put.
- **What doesn't:** the same cells moving on screen (scrolling, zoom, a resized column or window)
  keep their outline glued to them. Another sheet or workbook, an outline that wasn't drawn the
  frame before, and moves that start or end off screen appear in place. An arrow key held at the
  edge of the window scrolls the sheet under an outline that stays where it is. The header
  highlight, the Name Box and the other areas of a multiple selection change at once.
- **Only painting waits.** The selection itself, hit testing (the fill handle, the border you drag)
  and the cell editor use the real cells immediately; typing is never held up by a glide.
- **Timing:** 0.12 s with a decelerating (ease-out cubic) curve, the same at any frame rate.
  `GRIDCRAFT_SELECTION_GLIDE_MS` overrides the duration: `0` turns the glide off, a large value (up
  to 10000) slows it down enough to study.
- **Turning it off:** View › **Animations** (a checkbox under the Display theme picker: settings
  live in the View tab, and there is no Options dialog). Its label and hint are worded in the
  languages upstream translates, from a table of the fork's own (`tr` in `aniefork/mod.rs`); any
  other language shows them in English. Off, the outline jumps as upstream's does. The choice
  is saved with the other UI preferences (`animations` in `ui.json`; on by default, and a file
  written before the setting existed leaves it on). Agents: `view.animations {on?}` sets or
  toggles it and returns `{on}`; it is listed by `engine.commands` and shown by `ui.inspect`.
- **Other off switches:** the glide stands down when egui's animations are off
  (`style.animation_time == 0`). The headless test harness (`egui_kittest`, so the offscreen
  `snapshot` example too) turns them off. In the desktop app, a screenshot asked for over the
  control channel (`ui.screenshot`) holds the outline on its cells from the request until the
  picture is delivered. Either way, scripted pictures never catch an outline in flight.
- **Marching ants:** the dashes around a copied range now advance by the clock (the same pace as
  before at rest) instead of once per painted frame; otherwise every glide, which repaints each
  frame, made them lurch forward.
- **Clean room:** the behaviour is from using Excel as a user; nothing was read from the Excel
  program. The duration and curve are our estimate.
- **Tests:** `cargo test -p gridcraft-ui-egui aniefork` (the motion, frame-rate independence,
  retargeting, scrolling and view changes, off-screen and far edges, hostile numbers, the outline
  gliding in the real grid, and a control-channel screenshot holding it still).

### Open
- Measure Excel's cell cursor (duration, curve, what long jumps and scrolling moves do) from a
  capture of its window and set `GLIDE_SECS` and the curve to match. WordCraft's fork has done
  this for Word's caret (its `ROADMAP-aniefork.md` has the method): 100 ms along
  `cubic-bezier(0,0,0,1)`, a second key snaps to where the caret was heading before gliding on,
  and clicks and scripted moves don't glide. Excel may or may not behave the same.
- Following the system's reduced-motion setting (Windows: Settings › Accessibility › Visual
  effects › Animation effects), which Excel does. Today the Animations setting is the user's own.
- An Options dialog (Excel: File › Options, where "Provide feedback with animation" lives).
  Neither the fork nor upstream has one; until then the setting is on the View tab.

## The active cell stays clear

In Excel the active cell of a selected range is not shaded: it shows where typing will go.
Upstream GridCraft shades the whole range alike. The fork leaves the active cell clear.

- **What is left clear:** the active cell in every area of the selection that contains it; the
  whole merged cell when the active cell is merged; a lone active cell among several areas
  (Ctrl+click) is not shaded at all. The other areas are shaded whole, and a single selected cell
  has no fill, as before.
- **How:** the fill is translucent, so a cell can't be cleared by painting over it. Each area is
  shaded as up to four rectangles around the active cell (above, below and beside it) that never
  overlap. Their joins fall on the lines that continue the cell's edges and don't show: checked
  with gridlines off at 100 %, 115 % and 137 % zoom, at 1× and 1.5× display scale.
- **With the glide:** the current area's fill follows its gliding outline; the clear cell is at
  the active cell's real place from the first frame.
- **Tests:** `cargo test -p gridcraft-ui-egui aniefork` (the rectangles around a hole in every
  position, hostile numbers, and the fill for a real sheet: one area, several areas, a merged
  active cell).

## The File page

What the File button opens: a page over the whole window, laid out the way Excel lays out its
File view (which it calls the Backstage view), in place of upstream's dropdown menu.

- **A column on the left** turns between the pages (Home, New, Open, Info, Export) and holds the
  actions that go back to the workbook (Save, Save As…, Close, and About and Options… at the
  foot). A round button at its head, or Escape, goes back without doing anything.
- **Home** shows what a new workbook can start from (an empty one and upstream's three samples,
  each under a small drawing made in code) and the workbooks opened before, each with its folder
  and how long ago it was changed. A click opens one; the row under the pointer offers to drop
  its workbook from the list. A workbook that can't be opened says why and stays listed.
- **Info** shows where the workbook is and the properties saved with it. A property is changed by
  typing: it is set through `file.properties`, as one undo step, when its field is left.
- **Export** hands over to the dialog upstream's Share menu opens (CSV, web page). A PDF is
  offered only on the web build: see Open below.
- **It has no commands of its own.** Every button runs one of upstream's (`file.new`,
  `file.open`, `file.save`…), and the buttons keep the labels upstream's menu had, so upstream's
  own tests of the File menu pass unchanged against the page.
- **To the rest of the app it is a dialog named `backstage`**, so what upstream does for dialogs
  holds: the grid and the ribbon's keys stand down while it is up, `ui.dialog {name:
  "backstage", page?}` opens it for an agent, `ui.dialog.set {field: "page", value}` turns the
  page, `ui.dialog.cancel` leaves, and `ui.inspect` reports it.
- **What is being typed into a cell is entered first** when the page opens, since the page is
  about that workbook. An entry the cell refuses keeps the user in the cell, with the reason.
- **The list tells ages, not dates** ("3 days ago"): the clock's time zone isn't known to the UI
  crate, and a date could be a day out. The files are asked about on a thread of their own, so a
  network drive that is slow or away never holds up the page.
- **A workbook saved under a name joins the list of recent ones** when the page opens. Upstream
  adds a workbook only when it is opened from a file.

### Open
- **Print and PDF.** The desktop app has no way to ask where a PDF should go (its save dialog
  offers only workbook formats), and upstream's `file.print` hands back a PDF that nothing sends
  to a printer yet. So the column has no Print, and Export offers PDF only on the web build.
- Dates in the list once the local time zone is known; pinning a workbook to the list.
- Upstream's title bar still opens its small Start dialog from the Home button there.
- Wording in other languages: upstream's labels follow the interface language, the page's own
  are in English.

## File ▸ Options

The settings dialog Excel has and upstream doesn't. It opens from the foot of the File page's
column. It holds the settings that work today:

- **General:** the display theme and the interface language (upstream's settings, changed through
  upstream's own commands, and still on the View tab too), and the user name, which is the author
  of comments and notes.
- **Feedback:** Animations (the selection glide).
- **Ribbon:** Show the Developer tab.

The dialog edits a copy and nothing changes until OK. `file.options` is the command behind it:
with parameters it changes the settings named (`theme`, `language`, `userName`, `animations`,
`developerTab`) and returns all of them; without parameters it only reports them and never opens
the dialog. A value that makes no sense is an error and changes nothing. It is a UI command of the
fork's, so upstream's generated command checklist is not affected.

Upstream's labels in the dialog follow the interface language; the fork's own are in English.

### Open
- The rest of what Excel's Options holds, as the features behind it come to exist: the direction
  Enter moves, the default font and sheets of a new workbook, R1C1 references. The engine has
  preferences for these, but nothing reads them yet.
- Translations of the fork's labels in the dialog.

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

The crate depends on nothing else in GridCraft, so it can be offered upstream as pieces of its
own. Next is phase 1c: the bridge, `spreadsheet.js`, and panels in the browser.

### Open
- A script's process is not confined, and stopping it stops only the Python GridCraft started,
  not processes the script started itself.
- The trust check hashes the whole module on every call. Fine for small modules; a large one will
  want the fingerprint cached against its files' times.
- Wording of the fork's dialogs in other languages.

## Hooks in upstream files

Everything else the fork adds is in files of its own. After merging upstream, check these are
still there:

| File | Hook |
|---|---|
| `crates/ui-egui/src/lib.rs` | `pub mod aniefork;` |
| `crates/ui-egui/src/grid.rs`, `paint_selection` | `cur` (the current area's rectangle) comes from `crate::aniefork::selection::glide(…)`, at the top of the function, instead of `geo.range_rect(sh, sel.current())` further down |
| `crates/ui-egui/src/grid.rs`, `paint_selection`, the fill loop | each area is filled with the rectangles from `crate::aniefork::selection::fill_rects(…)` (the current area at `cur`, the active cell left out) instead of its whole `geo.range_rect(sh, *r)` |
| `crates/ui-egui/src/grid.rs`, `show`, the marching ants | `marching_phase` comes from `crate::aniefork::selection::ants_phase(…)` instead of stepping by 0.5 per frame |
| `crates/ui-egui/src/control.rs`, after `struct Shots` | `Shots::in_flight()`, which the glide asks so that screenshots show a settled outline |
| `crates/ui-egui/src/control.rs`, `handle`, `engine.commands` | after upstream's UI commands are listed and before the listing is filtered: `v.extend(crate::aniefork::commands());` |
| `crates/ui-egui/src/lib.rs`, `UiState` and its `Default` | the fields `animations: bool` (default `true`) and `developer_tab: bool` (default `false`), between `formula_bar_expanded` and `status_bar` |
| `crates/ui-egui/src/lib.rs`, `run_ui_command`, first thing | `aniefork::run_command(self, id, p)` runs the fork's UI commands and returns early |
| `crates/ui-egui/src/ribbon.rs`, `view`, in the Display theme column, after the picker | `crate::aniefork::view_options(app, ui);` draws the fork's settings |
| `crates/ui-egui/src/ribbon.rs`, `file_menu`, after the File button is added | `if crate::aniefork::backstage::file_button(app, &file) { return; }` opens the fork's File page; upstream's menu below it is left as it is, and no longer runs |
| `crates/ui-egui/src/dialogs.rs`, `show`, first thing | `if crate::aniefork::backstage::show(app, ctx) { return; }` draws the File page over the whole window when it is the open dialog |
| `crates/ui-egui/src/ribbon.rs`, `show`, after `tabs` is made from `TABS` | `crate::aniefork::developer::tabs(app, &mut tabs);` adds the Developer tab when it is on |
| `crates/ui-egui/src/ribbon.rs`, `show`, the match on the active tab | the arm `"Developer" => crate::aniefork::developer::ribbon(app, ui)` |
| `crates/ui-egui/src/dialogs.rs`, `Dialog::open`, the last arm of the match (upstream's is `_ => return None`) | `name => …` asks `crate::aniefork::dialog(app, name, &params)?` for a title, starting values and a command, and builds a custom dialog from them; every fork dialog comes through here |
| `crates/ui-egui/src/dialogs.rs`, `show`, before the `_ => form(…)` arm | `_ if crate::aniefork::draws_dialog(&d.name) => crate::aniefork::dialog_ui(…)` draws any of the fork's dialogs |
| `crates/ui-egui/Cargo.toml`, `[dependencies]` | `gridcraft-aniefork-dev = { path = "../aniefork-dev" }` |
| `crates/ui-egui/src/lib.rs`, `Services` | the field `confirm`: a yes/no question asked by the operating system, which the UI can't answer for the user |
| `apps/gridcraft/src/main.rs`, `services()` | `confirm` is a native message dialog (`rfd::MessageDialog`, Yes/No) |
| `xtask/src/layers.rs`, `TABLE` and `INTRA_LAYER_ORDER` | the crate `aniefork-dev` at layer 6, and the chain `["aniefork-dev", "ui-egui"]` so the UI crate may use it |
| `Cargo.lock` | the new crate's entry. When upstream changes dependencies, take upstream's file and let `cargo` add the entry back |

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

## Log

Newest first.

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
