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
| Last merge of upstream | 2026-10-13, upstream `fc45b7f` (GridCraft 0.6.1: 6 commits since the merge before) |
| Fork is behind upstream by | 0 commits at that merge |
| Fork carries | 13 changes upstream lacks (below) |

## What the fork carries

Changes in the fork that upstream does not have. Update the last column when one is offered or
merged; delete the row once a merge of upstream brings it back.

| Change | Where | Upstream |
|---|---|---|
| **On upstream's Options page:** the user name (the author of comments and notes) and whether the ribbon shows the Developer tab, with a command for each (`app.userName`, `developer.tab`); and an Animations section holding upstream's switch for the selection glide, as wordcraft's Options has. | `crates/ui-egui/src/aniefork/options.rs`, a hook in `backstage.rs` | Not offered |
| **Options for the selection:** the weight of its outline (thin by default, which is Excel's; standard is upstream's; hairline), and hiding it while another window is the active one, as Excel does, which is off until chosen. On the Options page's Selection section. | `crates/ui-egui/src/aniefork/` (`selection.rs`, `prefs.rs`, `options.rs`), hooks in `grid.rs` and `lib.rs` | Not offered |
| **The selection's outline lies on the gridlines.** Upstream centres it half a point outside the cells, which puts its bottom and right sides a point past the gridlines they should cover; on a display at 125% or 150% a grey line shows inside them. Here each side covers its gridline and reaches outward from it, a whole number of device pixels wide, which is where Excel was measured to put it. | `Bars` and `paint_outline` in `crates/ui-egui/src/aniefork/selection.rs`, a hook in `grid.rs` | Offered: [storytold/gridcraft#549](https://github.com/storytold/gridcraft/pull/549) (open), its second commit |
| **The fill handle is sized to the outline**: three times its weight, set on the outline's inner corner, which is how Excel's measured. Upstream's is always six points, a heavy square beside a thin outline. | `fill_handle` and `paint_fill_handle` in `crates/ui-egui/src/aniefork/selection.rs`, a hook in `grid.rs` | Not offered: it follows the fork's choice of weight, and upstream has one weight |
| **The highlight stays where it is while typing.** Upstream draws the border of a cell being edited half a point further out than the selection's outline, makes the editor a few points taller than a default row, and stretches the border round the text as it grows: the box left the gridlines the moment typing began, and then followed the text. Here the highlight neither moves nor grows. Text that runs past the cell is laid over the cells after it, a whole cell at a time, and clears the lines it runs over, as Excel does. | `editor_size` and `paint_editor` in `crates/ui-egui/src/aniefork/selection.rs`, hooks in `grid.rs` | Offered: [storytold/gridcraft#549](https://github.com/storytold/gridcraft/pull/549) (open) |
| **The Development server and the Development window** (the Development subsystem, phase 2e): a second MCP server, inside the running app, for agents that help with development work. Off until the user turns it on, in Developer ▸ Development, where they set how far agents may go (read only, ask each time, build and ask before running, full control until GridCraft closes). What an agent asks for is shown on a card, with the lines a change to a file would take out and put in, and answered there. | `crates/aniefork-dev/src/` (`agents.rs`, `http.rs`), `crates/ui-egui/src/aniefork/` (`agents.rs`, `development.rs`, `diff.rs`), a hook in `control.rs` | Not offered; a direction of the fork's own |
| **Development kits and projects: the shape of the Development window** (the owner's design): the window has a list down its side: Development, the MCP server, the development kits, and under the Anie Development Kit its projects, as workbook code or libraries. Each has a part of its own, and a new project is made in the kit's part. Both windows show it: GridCraft's own, and the page. | `crates/ui-egui/src/aniefork/kits.rs`, changes in `development.rs`, `devpage.rs`, `projects.rs`, `card.rs`; the page in `apps/gridcraft/src/aniefork/page.rs`; the kits are listed in `crates/aniefork-dev/src/kit.rs` | Not offered; a direction of the fork's own |
| **An agent rebuilds a workbook's macros as a project** (phase V1 of bringing macros into GridCraft): a project can name the workbook it is the code of, and keep a note of which VBA procedure became which script (`rebuilt.toml`); the agent's `list_macros` reports the note, and `reference` has a topic on rebuilding; Developer ▸ Macros shows it beside the macros. | `crates/aniefork-dev/src/rebuilt.rs`, `kit.rs`, changes in `manifest.rs`, `starter.rs`, `agents.rs`; `crates/ui-egui/src/aniefork/macros.rs` | Not offered; a direction of the fork's own |
| **Macros (VBA), to read** (the first step of bringing what a workbook's macros do into GridCraft): Developer ▸ Macros shows the modules of the VBA project a workbook carries and their source; `developer.macros` reports them; an agent reads them through two tools of the Development server. Nothing is run, and nothing in the project is changed. | `crates/aniefork-dev/src/vba.rs`, `crates/ui-egui/src/aniefork/macros.rs`, a button in `developer.rs` | Could be offered: the reader depends on nothing of the fork's. Not offered yet |
| **The Development window as a Svelte page, in a window of its own** (phase 2d, second slice, on Windows): Developer ▸ Development opens a second window that stays beside the sheet, showing the same window as a page in the system's web view. What a card says is said once (`card.rs`) and drawn by both; a click in the page goes through the same `act`. Elsewhere, and by the user's choice, it is GridCraft's own dialog. | `crates/ui-egui/src/aniefork/` (`card.rs`, `devpage.rs`), `apps/gridcraft/src/aniefork/` (`window.rs`, `page.rs`), a line in `apps/gridcraft/Cargo.toml` | Not offered: fork-only (Svelte source and a built script as text, a second windowing crate) |
| **A panel docked in the window, and the Svelte showcase** (the Development subsystem, phase 2d, on Windows): a trusted project's panel is shown beside the sheet in the system's web view (WebView2), over the same bridge; a project without `network` is kept to its own address there. Developer ▸ Development creates a showcase project whose panel is a Svelte app and, once the user has trusted it, docks it. | `crates/ui-egui/src/aniefork/dock.rs`, `apps/gridcraft/src/aniefork.rs`, `crates/aniefork-dev/src/` (`showcase.rs`, `showcase_files.rs`), hooks in `lib.rs`, `apps/gridcraft/src/main.rs` and `apps/gridcraft/Cargo.toml` | Not offered: fork-only (it carries Svelte source and a built script as text, against upstream's "Rust only") |
| **The Developer tab**, hidden until Options turns it on: the entry to the Development subsystem designed in [ADR-0001](docs/decisions-aniefork.md). | `crates/ui-egui/src/aniefork/developer.rs`, hooks in `ribbon.rs` | Not offered; a direction of the fork's own |
| **Projects, trust, Python, panels, events, a console and the workbook as objects** (the Development subsystem, phases 1a to 2b): projects are folders the user installs; each has a manifest saying what it asks to be allowed and a fingerprint of its contents; the user trusts a project or not, in Developer ▸ Projects, confirmed by the operating system; a trusted project's Python scripts run in a process of their own, and its panel opens in the browser, calls GridCraft over a local bridge and is told what happens there; Developer ▸ Console shows what projects print and log; a script can work on the sheet itself, and panels and scripts share one set of objects for the workbook. | The crate `crates/aniefork-dev`; `crates/ui-egui/src/aniefork/` (`projects.rs`, `panels.rs`, `console.rs`, `presence.rs`); hooks in `xtask/src/layers.rs`, `crates/ui-egui/Cargo.toml`, `crates/ui-egui/src/lib.rs`, `dialogs.rs` and `apps/gridcraft/src/main.rs` | Not offered; fork-only until proven (ADR-0001) |

## What upstream took in, and what it built itself

The fork no longer carries these. They are kept here as the record of what happened to them.

| Change | What happened |
|---|---|
| **The active cell stays clear inside a selection** | Offered as [storytold/gridcraft#265](https://github.com/storytold/gridcraft/pull/265) and **merged** on 2026-10-11. The fork's copy (`fill_rects` and its hook in `grid.rs`) was deleted at the merge of 2026-10-12. |
| **Selection glide**, with View ▸ Animations | Offered as [storytold/gridcraft#266](https://github.com/storytold/gridcraft/pull/266) and **merged** on 2026-10-11. Upstream merged its `main` into the branch first, so the glide respects frozen-pane clipping and right-to-left sheets, and added Spanish, German and Czech strings. The fork's copy (`aniefork/selection.rs`, the `view.animations` command, the hooks in `grid.rs`, `control.rs`, `lib.rs` and `ribbon.rs`, and the `GRIDCRAFT_SELECTION_GLIDE_MS` override) was deleted at the merge. |
| **The File page** (Backstage view) | Offered as [storytold/gridcraft#337](https://github.com/storytold/gridcraft/pull/337) and **closed as a duplicate**: another contributor's #331 had added the same view, with more pages (Save As, Print, Options), translations and a control-channel method, and was merged minutes earlier. The fork's page (`aniefork/backstage.rs` and its hooks) was deleted at the merge; upstream's is used. The maintainers asked for one part of ours as a follow-up: see Open below. |
| **File ▸ Options** as a dialog, and a `file.options` command | Upstream now has an Options page in its File view and a `file.options` command that opens it. The fork's dialog and command were retired at the merge; its two settings moved onto upstream's page. |

### Open
- **The follow-up the maintainers invited** is offered as
  [storytold/gridcraft#463](https://github.com/storytold/gridcraft/pull/463) (2026-10-12, branch
  `recent-modified`): the recent list's "modified" column on upstream's File page, read on a
  thread of its own so a slow drive never holds up the view, with its ten strings in upstream's
  nine catalogs. The fork's `main` does not carry it: it arrives with the merge after upstream
  takes it.
- **The outline on the gridlines, and the highlight that stays on its cell while typing**, are
  offered together as
  [storytold/gridcraft#549](https://github.com/storytold/gridcraft/pull/549) (2026-10-12, branch
  `editor-highlight`, two commits), written into upstream's `grid.rs` without the fork's choice
  of weight: upstream's outline stays two points. There the renderer's rule is checked without
  a GPU, by tessellating what was painted (`tests/selection_outline.rs`), which the fork could
  take over from its own pixel test at the merge. When upstream takes it, the fork's `Bars`,
  `editor_size` and `paint_editor` reduce to the weight and the fill handle: at that merge, keep
  upstream's code and pass it the fork's width. Its check was red for a reason outside it
  (upstream's `main` failed two engine tests of its own). Upstream fixed them in 0.6.0, and on
  2026-10-13 this branch and #463's were brought up to upstream's `main` (0.6.1) by a merge,
  after upstream's own gate passed on each result here. Neither has a review yet. Upstream's #456
  (and the pull requests stacked on it) changes the height line of the same function to cap the
  editor at five lines; it is not the same work, but whichever lands second needs a rebase.
- Following the system's reduced-motion setting is now an open pull request of someone else's:
  upstream's #504 puts "Animation effects" (Use system setting, On, Off) on File ▸ Options and
  turns `animations` from a switch into a choice of three. Measuring Excel's glide is still
  upstream's to take up.

## The fork's settings on the Options page

Upstream's Options page (File ▸ Options) holds the display theme, the interface language, the
formula bar, AutoSave and the calculation mode. The fork adds two settings at the end of its
General section, applied at once like upstream's own:

- **User name**: the author of the comments and notes the user adds. It is set when the field is
  left. `app.userName {name?}` is the command: without a name it only reports.
- **Show the Developer tab**. `developer.tab {on?}` is the command: without `on` it toggles.

A value that makes no sense is an error and changes nothing, and neither command opens anything.

Then a **Selection** section:

- **Outline**: Thin (a point and a half, the default), Standard (upstream's two points) or
  Hairline (one). `view.selectionOutline {weight?}` is the command: without a weight it only
  reports. The border
  of a cell being typed into is drawn as heavy as the outline. The outline is always a whole
  number of device pixels wide, the nearest to its points: on a display at 150% that is three,
  two and two. **Thin is what Excel draws** there (below).
- **Hide the selection while another window is active**, as Excel does: the outline, the
  shading of a range and the fill handle go, and come back when GridCraft is the active window
  again. The marching ants around a copied range stay. `view.hideSelectionWhenInactive {on?}`
  is the command: without `on` it toggles. A screenshot an agent takes of an inactive window
  shows no selection while this is on.

The outline is Thin unless chosen otherwise: it is what Excel draws, and the reason for the
option. Hiding is off until chosen. Upstream's tests of the selection ask where the outline is,
not how heavy, and pass with either.

The fork's preferences are saved in `ui.json` beside upstream's, through one field of `UiState`
(`aniefork`, flattened), so upstream's struct has one line for however many the fork adds. A
saved value that makes no sense reads as its default and costs no other preference.

**Only a choice is saved.** A value equal to its default is not written, so a default the fork
changes later reaches everyone who never chose otherwise. The fork's first builds did write the
default down (`"selectionOutline": "standard"` in every `ui.json` they saved), which would have
kept those files on Standard when the default became Thin. The weight is therefore saved under
a key of its own, `selectionOutlineWeight`; under the old key `"standard"` is not believed, and
the other two weights, which could only have been chosen, still are.

After that comes a section of its own, **Animations**, as the sister fork wordcraft has on its
Options page: one checkbox, "Glide the selection outline to the cell you move to instead of
jumping there". The setting and its command (`view.animations`) are upstream's; the heading and
the checkbox use upstream's own words for it, so they follow the interface language.

**Where the outline is.** Each side lies on the gridline that bounds the cells and reaches
outward from it, so what is inside the outline is exactly the inside of the cells, at any
weight and any display scale. That is Excel's placement, measured from a capture of Excel on a
display at 150% (2026-10-12, a cell being edited):

| | Excel at 150% | GridCraft at 150%, before | Now |
|---|---|---|---|
| A gridline | 1 device pixel | 2 (a point and a half, spread over two) | unchanged: upstream's |
| The outline | 2 pixels: the gridline's and the one outside it, on each side | 3 pixels; at the top and left on the gridline and one outside, at the bottom and right a pixel further out, leaving a grey line inside | Standard 3, Thin 2, Hairline 2, each on its gridline on all four sides |
| While typing past the cell | the right side cleared, the top line running on to the outer corner | the same | the same |

Upstream draws the outline as a stroke centred half a point outside the cells. The fork paints
four bars instead, placed from where the renderer puts each gridline (`gridline` in
`selection.rs` repeats the renderer's rule for moving a line onto device pixels). A test renders
the grid at 100%, 125%, 150% and 200% and checks, pixel by pixel and for each weight, that every
side covers its gridline, reaches only outward, and leaves no gridline showing inside. If a new
egui changes its rule, that test fails and the rule is repeated anew.

**The fill handle** is three times the outline's weight, in whole pixels, centred on the
outline's inner corner so that it reaches as far into the cell as out of it, with a clear
margin of half the weight (a pixel at least) between it and the outline's lines. In the same
capture Excel's is six pixels to an outline of two, on that corner, with about a pixel clear.
With Thin at 150% that is the fork's six pixels, where upstream's six points were nine; with
Standard it is still nine. It is never smaller than four points, so a hairline outline keeps a
handle that can be seen and caught. The pointer still catches it in upstream's nine points
round the cells' corner, which the handle stays inside.

**While typing**, the highlight is the selection's own outline: the same place, the same weight,
and no larger than the cell however long the text gets.

- Text that needs more room than the cell has is laid over the cells after it, a whole column
  at a time, so the editor always ends on a gridline. A cell it covers is covered entirely: none
  of its contents show beside what is being typed. They come back when the editor closes.
- The lines the text runs over are cleared: the highlight's right side once the text reaches
  it, and the gridlines between the cells it covers. The highlight's top, left and bottom sides
  stay where they are.
- Lines added with Alt+Enter go the same way downwards: whole rows, and the highlight's bottom
  side is cleared. Two lines of the default font take two rows of the default height.
- In a frozen pane the editor has only its own pane to grow over, and is cut off at its edge.

### Open
- **Excel's outline is measured at one display scale only**, 150%: two device pixels. Whether
  it is two at 100% and what it is at 200% are not known; Thin gives two, two and three.
- **Excel's gridlines are one device pixel at 150%; GridCraft's are a point and a half**, which
  the renderer spreads over two. That is upstream's drawing and the fork leaves it, so at 150%
  Thin and Hairline are the same two pixels. A one-pixel gridline would be a hook in upstream's
  `paint_cells`.
- **Excel keeps the fill handle showing while a cell is being edited**, below the edit box;
  upstream hides it, and so does the fork.
- **On a right-to-left sheet** the outline is mirrored after it is placed, so at a fractional
  display scale it can land a pixel off the gridline there.
- **Typing past a cell follows what Excel was seen to do sideways**: the highlight stays and
  the line clears. Downwards (Alt+Enter) the same rule is applied without having been checked
  against Excel.
- **The editor takes the next cell a little before the text reaches the line**: twelve points
  before, which is what upstream's text field keeps clear at its edges. The line itself clears
  only when the text reaches it, so with an empty neighbour nothing shows early; a neighbour
  with contents loses them about a letter and a half sooner than in Excel.
- **The ribbon's View tab still has upstream's Animations checkbox**, so the setting shows in
  two places. Taking it off the View tab is upstream's change to make: the checkbox is upstream's
  code and one of upstream's tests (`tests/selection_glide.rs`) clicks it there, so removing it
  in the fork would mean editing both. **Not to be offered upstream:** upstream's open #504
  already puts the setting on the Options page (and keeps the View tab's box), so a pull request
  of ours would duplicate it. When #504 merges, the fork's Animations section goes: it reads
  `animations` as a switch, which #504 turns into a choice of three, and upstream's page will
  have the setting itself.
- The rest of what Excel's Options holds, as the features behind it come to exist: the direction
  Enter moves, the default font and sheets of a new workbook, R1C1 references.
- Translations of the fork's two labels on the page.

## The Developer tab and the Development subsystem

**One word (2026-10-13): what was a *module* is a *project*.** The owner asked for it once
the Development window said project. It is the word everywhere now: in the windows, in
`project.toml`, in the Development server's tools and their arguments, in `developer.projects`,
in the folder `projects` and `trusted-projects.json`, and in the code. This section says
project throughout; the log below, and ADR-0001's text from before that day, say module as
they were written.

A ribbon tab for people who extend and automate GridCraft, hidden until File ▸ Options ▸ "Show the
Developer tab" turns it on, as Excel's is. Today it gathers what already exists for that audience:
the command palette, the action journal, how agents connect, and Options.

It is the entry to the **Development subsystem**: projects a user installs that add web panels and
Python to GridCraft. That is designed in [`docs/decisions-aniefork.md`](docs/decisions-aniefork.md)
(ADR-0001), which the owner **accepted on 2026-10-11**: panels in the browser first and docked
later, the user's own Python, fork-only until it is proven and then offered upstream in phases.
Whether a workbook can carry a project is left open. Letting agents help was on the backlog
until the owner took it up on 2026-10-12 (phase 2e, below). The record also sets out how the
model is exposed to projects and how the user gives or refuses trust; those two are still
proposals.

**Phase 1a is built: projects and trust, with nothing running.**

- **A project is a folder** in `projects`, beside the user's settings, holding a `project.toml` (id,
  name, version, what it asks to be allowed), a panel and scripts. Developer ▸ Projects lists what
  is there; "New project" writes a starter from templates compiled into the app.
- **Every project starts untrusted, and an untrusted project is inert.** Review… shows what it asks
  for in plain words (running Python is marked as the one that cannot be limited), its files and
  its fingerprint; then the user trusts it or doesn't.
- **Trust is given to a fingerprint**, a SHA-256 over every file in the project. A changed file
  makes it untrusted again. For a project they are writing, the user can choose to keep trusting it
  as it changes; that never grants more than was granted.
- **Only the user gives trust.** There is no command for it. `developer.projects` reports the list
  to agents and nothing else; the dialog has no OK command for an agent to confirm; and what is
  trusted is the fingerprint that was on screen, so files swapped during the review are refused.
- **It fails closed.** A project that can't be read whole (no manifest, a path that leaves it, a
  link, an unknown permission, too large) can't be trusted. A damaged trust file trusts nothing.
- The decisions are in `trusted-projects.json` beside the settings, and a run without settings
  (the web build, `GRIDCRAFT_NO_PREFS`) has no projects.

**Phase 1b is built: a trusted project's Python scripts run.**

- **A script is a file in the project's `python` folder with a `main(data)` function.** GridCraft
  starts the user's own Python (the one named in Developer ▸ Projects, or the first found) with a
  small runner. The process stays alive between calls, so a project's imports are paid for once,
  and is started again when the project's files change. Each project has its own.
- **Nothing runs without trust.** Every call reads the project and the user's decisions from disk
  and refuses, before any process exists, a project that is untrusted, changed since it was
  trusted or not granted `python`, and a script that is not one of the project's own files.
- **The process protects GridCraft from the script.** A script that raises is reported with its
  traceback; one that runs past its time limit (60 s) is stopped; one that dies costs only its
  process; what it prints can't garble its reply. Scripts run off the UI thread and can be
  stopped. It does not limit what the script does to the computer: that is what trust is for.
- **Giving trust now needs the operating system's own confirmation**, a native yes/no dialog that
  states the project's name, everything it will be allowed and how many files it has. Synthetic
  clicks from the control channel can't answer it, and a build without one can't trust projects.
- **Developer ▸ Projects runs a script by hand**: "Run on the selection" hands the script the
  selected cells as `{range, sheet, rows}` and shows what it returned, what it printed, or why it
  failed. The sheet is not changed: writing back belongs to panels, in phase 1c. The dialog also
  says which Python is used and lets the user name another, and opens the projects folder.

**Phase 1c is built: a trusted project's panel opens in the browser and calls GridCraft.**

- **Open Panel**, on a trusted project in Developer ▸ Projects, serves the project's panel folder
  from a small server inside GridCraft and opens it in the user's browser. The server listens on
  the loopback address only, on a port of the project's own, behind a secret token made for that
  run. The address goes to the browser and is shown nowhere else.
- **The panel gets a `spreadsheet` object** from `spreadsheet.js`, served beside its page:
  `run(id, params)` for every command of the engine, `batch([...])` for several commands as one
  undo step (all of them, or none), `runPython(script, data)`, and `selection()`, `read()` and
  `write()` as shorthands. `spreadsheet.d.ts`, served beside it, lists every command for
  TypeScript. The starter project's panel now does the whole round: the selection, through its
  Python script, back into the sheet, undone with one Undo.
- **A project runs only what it was granted.** A command that only reports needs `read`, any
  other `write`, one that touches a file `files`. GridCraft's own settings folder is out of
  reach even with `files`. A panel without `network` is served with a policy that makes the
  browser refuse its requests to anywhere else.
- **Trust is checked on every request.** A project whose files changed, or whose trust was taken
  back, is served and answered nothing more, at once, with the panel still open.
- **Commands never open dialogs for a project**, and the window's own commands (the theme, the
  dialogs) are not offered to it: it is a program calling GridCraft, as an agent is.
- **Some things are for the person at the screen alone.** Trusting a project, running a script,
  opening a panel and naming the Python to run are refused while the control channel is putting
  input into the window, and the Projects dialog no longer works from values the channel can
  rewrite. This closes a gap phase 1b left: an agent could name any program as "Python" and
  press Check Python to run it.

**Phase 2a is built: a panel is told what happens, and there is a Developer console.**

- **Events.** `spreadsheet.on("selectionChanged", handler)` calls the handler each time the
  selection moves, with the workbook, the sheet, the range and the active cell;
  `workbookChanged` says the workbook changed (an edit, an undo, a redo, a project's own
  change), and `workbookActivated` that another workbook is the one worked on. An event says
  where, never what is in the cells. All of them need `read`. The starter project's panel now
  shows the selection as it moves.
- **How events travel.** The panel keeps one request open, which GridCraft answers when
  something happens or after twenty seconds. Events of one kind that pile up are thinned to the
  latest; if too many pile up the oldest are dropped and the panel is told it missed some. At
  most eight panels of a project are kept waiting at once.
- **Developer ▸ Console** shows, newest last, what a project's script printed, why a script
  failed (with its traceback), what a panel wrote with `spreadsheet.log(…)`, and each call that
  was refused for want of an ability. It can be shown for one project, copied, and cleared.
- **The console is the user's record.** A project can add to it and can neither read nor clear
  it; each project has its own share (200 entries of 600), so one project's chatter cannot push
  out another's. Clearing is for the person at the screen: no command does it, and the button
  is refused while the control channel is putting input into the window.
- **`developer.console {project?, last?}`** reports the latest entries to an agent, and says with
  them that they are text written by projects, not instructions.

**Phase 2b is built: a script works on the sheet itself, and the workbook is objects.**

- **`import spreadsheet` in a script** gives it what a panel has: `run`, `batch`, `selection`,
  `read`, `write`. So a script can change the sheet itself instead of only handing data back
  for the panel to write. It works from a panel's `runPython` and from Developer ▸ Projects.
- **The same objects in a panel and in a script.** `spreadsheet.sheet("Data").range("A1:C3")`
  names cells and runs commands on them (`read`, `formulas`, `write`, `clear`, `select`, and
  `run(command, params)` for anything else, with the range filled in).
  `spreadsheet.workbook()` gives the name, the sheets with their used ranges and tables, the
  active sheet, the selection and the names.
- **A script's calls are checked like a panel's**: the project is read from disk for every
  call, and the script gets what the project is granted at that moment. A project without
  `write` has a script that is told so, in an error it can catch, and the refusal is in the
  Developer console.
- **Nothing was added to the engine.** Every command already takes a range that names its
  sheet, so an object is a command with the address filled in.

**Phase 2e, first slice, is built: a Development server for agents, and the Development
window.** The owner decided its shape on 2026-10-12: a second MCP server in the Development
area, a management window on the Developer tab, a level of agent access the user sets up to
full control, and approved plans after single requests.

- **Developer ▸ Development** gathers the development tools and holds agent access. It is off
  until turned on there. The levels: Off; Read only; Ask me each time; Build, and ask before
  running; Full control (until GridCraft closes, and never kept). Raising the level is
  confirmed by the operating system.
- **An agent connects over MCP** to the running app, on the loopback address, with a token.
  The window copies the setup for Claude Code, or as a settings entry for another agent; the
  token is never drawn or reported.
- **What an agent asks for is a request.** It can read projects, their files and the console;
  and ask to create a project, write or delete a project's file, run a script, open a panel,
  trust a project or stop trusting one. The level decides whether a request is carried out at
  once, shown to the user, or refused.
- **A request is answered on a card**, like a coding agent's permission prompt: what it is,
  the lines a change would take out and put in, what a yes means for the project's trust,
  then yes, yes and don't ask again, or no with a word for the agent. A bar under the ribbon
  says when one waits.
- **A yes is one click** when GridCraft runs without the control channel, and is confirmed by
  the operating system when it runs with it. **"Don't ask again"** covers changes to one
  project's files, or running its code, until GridCraft closes; the window lists what is not
  asked about.
- **What projects wrote reaches an agent fenced**, as data. Each request and its outcome is in
  the Developer console and in `agent-log.jsonl` beside the settings.
- **`developer.agents`** reports how it stands. No command sets the level, the port or the
  token, or answers a request; and the control channel can no longer read or write
  GridCraft's own settings folder.

**Phase 2d, first slice, is built: a panel docked in the window, and a Svelte showcase.**
The owner asked on 2026-10-12 for Svelte and the web view to be showcased in the Developer
section.

- **Developer ▸ Projects ▸ Dock Panel** shows a trusted project's panel beside the sheet, in
  the system's web view (WebView2), on Windows. It is the page a browser would be given,
  from the same bridge. Without `network` the page is kept to its own address: that is the
  part of `network` a browser could not enforce.
- **The view gives way** to dialogs, messages, menus and the File view, which it would
  otherwise hide.
- **Developer ▸ Development ▸ Create Showcase** makes a project whose panel is a Svelte app:
  it follows the selection, sums it up, draws its numbers as bars, and writes to the sheet.
  It is created untrusted like any project; once trusted, "Dock Showcase" shows it. Its
  Svelte source is in the project, with how to rebuild it.
- **`GRIDCRAFT_DOCK=<project>`** docks a trusted project's panel as GridCraft starts.

**Phase 2e, second slice, is built: approved plans.** The owner asked for them next, on
2026-10-12. The shape is Anie_Maint's D25.

- **An agent proposes a plan** (`propose_plan`): a title, a purpose, up to 20 steps, each one
  of the things it could ask for singly. It is followed through before anyone is asked, so a
  plan that could not be carried out is refused with the step and the reason.
- **The person answers once, on one card** that shows every step whole, each as it will find
  things once the steps before it are done, and says by itself what trusts a project or runs
  code. A yes approves the plan and does nothing else.
- **The agent then carries the steps out** (`run_plan_step`), in order, by the plan's number
  and the step's and nothing more: what a step does is GridCraft's own copy of what was
  shown. No step asks again. Each is checked when it is to be done, and waits if its project
  is not as the plan's earlier steps left it.
- **The window shows the plan**, where each step stands, how long it has, and **Stop Plan**.
  A plan also ends when a step fails, when its time runs out (60 minutes unless it says, 240
  at most), when the level is lowered and when GridCraft closes. One plan at a time.
- Four tools more (eighteen): `propose_plan`, `run_plan_step`, `plan_status`, `stop_plan`.

**Phase 2d, second slice, is built: the Development window as a Svelte page.** The owner
asked for it as a new window in Svelte, and whether in Electron (2026-10-12).

- **Developer ▸ Development** opens a window of its own, on Windows, that stays open beside
  the sheet: the level, the setup, what waits, the plan, the tools, and a request's card
  with its changes and its numbered choices. It keeps up by itself as agents ask.
- **Not Electron: the system's web view** (WebView2, through `wry` and `tao`), which is what
  Anie_Maint's console runs on. Electron would add a second program and a channel to it
  that could answer requests. ADR-0001 says more.
- **The page decides nothing.** GridCraft says what it shows, in the same words as its own
  window (`card.rs`), and takes its clicks through the same code. The page is served from
  the app's own files under a policy that allows nothing else; the token never reaches it.
- **GridCraft's own window is still there**: on other systems, when the page can't be shown,
  and by the link at the foot of the page.
- `GRIDCRAFT_DEVELOPMENT=1` opens it as GridCraft starts.

**Macros (VBA) can be read.** The owner asked for a viewer as the next slice, towards
rebuilding what a workbook's macros do as a project (2026-10-12).

- **Developer ▸ Macros** lists the modules of the VBA project the workbook carries and shows
  each one's source: what kind it is, its procedures, which of them Excel would run by
  itself, and which reach a library outside the workbook.
- **An agent can read them** (`list_macros`, `read_macro`), from "Read only" up: as data,
  inside the fence. That is what the first phase of rebuilding needs.
- **GridCraft still runs no macro**, and keeps the project byte for byte.
- **Excel was asked what it keeps** (ADR-0001 has the table): anything added to
  `vbaProject.bin` is lost the moment a macro is edited in Excel; a custom XML part is kept.
  So nothing will be written into a VBA project, and a project of ours that is to travel
  with a workbook has a place that Excel keeps.
- **Decided (2026-10-13):** the phases V1 to V3 in ADR-0001 stand, and with them no file
  format of GridCraft's own.

**An agent rebuilds a workbook's macros as a project** (phase V1, 2026-10-13).

- **A project says which workbook it is the code of** (`workbook = "…"` in `project.toml`,
  a file's name), and keeps a note of what became of each procedure: `rebuilt.toml`, with
  rebuilt, partly or left, the script and a line of why.
- **The note is shown as the project's own word**: beside the macros in Developer ▸ Macros,
  in the project's part of the Development window, and to an agent in `list_macros`.
- **The agent is told how** (`reference`, topic `macros`), and a person hands it the task
  with "Copy a Request for Your Agent" in Developer ▸ Macros.
- **Tried once**, in a copy of GridCraft with settings of its own: an agent read the
  macros of a workbook Excel made, proposed one plan of eight steps, and after one yes the
  project was written, trusted and run, giving what the macro's sum gives. The agent was
  the developer's own, not Claude Code set up by a person.

**The Development window is laid out by kit and project** (the owner's design, 2026-10-13).

- **Development** at the top: the whole at a glance.
- **MCP Server** under it, beside the kits and not in one: the level, the connection, what
  waits for an answer, the plans.
- **Development Kits**: the kits that are loaded. This build has one, the **Anie
  Development Kit** (Python and Svelte). There is no way to load another yet, and the
  window says so.
- **Projects** under the kit, by what they are for: **workbook code** and **libraries**. A
  project has a part of its own; a new one is made in the kit's part.
- Both windows show it: GridCraft's own, and the page, redrawn around the list.

Phase 2 is taken in slices, as phase 1 was (ADR-0001 has the table). Next are **2c**
per-project Python environments, and the rest of **2d**: macOS and Linux.

The crate depends on nothing else in GridCraft, so it can be offered upstream as pieces of its
own.

### Open
- **`network` is only partly enforced in a browser.** The policy stops a panel's requests, but
  not the page sending the user to another address, which can carry data. A panel docked in
  the window is kept to its own address; a panel opened in the browser is not.
- **Docked panels are Windows only**, one at a time, and are not docked again at the next
  start. The web view is hidden while anything of GridCraft's own is drawn over its place.
- The showcase's built script is 55 KB of generated text in `showcase_files.rs`. To change the
  showcase: change its source, build it with npm outside the repository, and regenerate the
  file with all of it together.
- The workbook as objects covers sheets, ranges and what `workbook()` lists. Tables, names
  and charts are listed as data, not yet as objects with methods of their own: they are
  changed through `run`.
- A script's time limit goes on running while one of its calls waits for the window, and each
  call waits at most thirty seconds.
- **"A workbook is about to be saved" is not an event yet.** It needs a place in the save path
  where a project is waited for, and a decision on whether a project may hold a save up.
- An event says a workbook changed, not which cells: telling the range needs the engine to say
  what a command touched.
- The Developer console is kept in memory for the run. It is not written to a file; what
  agents asked for is, in `agent-log.jsonl`. `developer.console` hands the entries to the
  control channel with a note that they are data; the Development server fences them.
- **The Development server's port is a fixed one.** On a computer other people use at the same
  time, a program of theirs that took the port while GridCraft was closed would be sent the
  token by an agent that tried to connect. A transport only the user's account can open is not
  built (ADR-0001 says why).
- **An agent that already runs programs as the user** (Claude Code with its shell) is not
  confined by the levels: it could read the token and the settings itself. They bound and
  record what is done through GridCraft.
- **Plans:** a step that fails ends its plan (no recovery steps, no declared
  expectations); a plan is sent in the call, not kept as a file to review first; one at a
  time. A step that trusts a project does so without a question of its own: the plan's card
  is where that is read. Any client holding the token can run or stop an approved plan.
- The Development server reaches projects and the console. The command palette, the action
  journal and recorded scripts are not on it.
- The Development window keeps the height of its tallest card until it is closed (GridCraft's
  own; the page scrolls).
- **Macros are read, not understood.** The outline is found line by line; a form's layout is
  not read; a project's password hides nothing from this (the source is in the file). "Read
  only" now lets an agent read the open workbook's macros, with whatever their author left
  in them: whether that should be a level of its own is not decided.
- **What a project says it rebuilt is a claim.** Nothing compares a script with the
  procedure, and GridCraft can't run the macro to compare what the two give. The rebuild
  was tried once, on one small procedure.
- **Kits: one, and no way to load another.** How one would be loaded is not decided
  (ADR-0001 has what is known). The kit can't be turned off as a whole.
- **What was made while a project was called a module is read where it is**: a
  `module.toml`, a `modules` folder, `trusted-modules.json`. Nothing is renamed for the
  person. An agent set up with the server's older tool names (`create_module` and the like)
  finds them gone: they are `create_project` and the like now.
- **A project names its workbook by the file's name**: two workbooks of one name are the
  same to it.
- Reviewing, trusting and running a project is still done in Projects, not in its own part
  of the Development window.
- **The Development window as a page is Windows only**, and a second thing to keep alike:
  what the two windows say is shared, how they are laid out is not. Its place and size are
  not remembered. To change the page: `cargo test -p gridcraft -- --ignored
  write_the_pages_source`, change and build it with npm in `target/development-page`, then
  `cargo test -p gridcraft -- --ignored take_the_built_page`.
- A flaw in the web view's engine could let what a request holds act in the page. The
  page's policy is there for that; it is still more surface than a window that draws text.
- The server was used over real connections by a test client, not yet by Claude Code itself.
- The panel's script (`spreadsheet.js`) is run by no test of the repository: it was exercised
  by hand in a browser, against a copy of the app with settings of its own. The Python side
  of the same objects is tested, where the machine has a Python.
- A script's process is not confined, and stopping it stops only the Python GridCraft started,
  not processes the script started itself.
- The trust check hashes the whole project on every request. Fine for small projects; a large one
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
| `crates/ui-egui/src/lib.rs`, `UiState` and its `Default` | the field `aniefork: aniefork::Prefs`, flattened (`#[serde(flatten)]`), after `animations`: every preference of the fork's, in one field |
| `crates/ui-egui/src/grid.rs`, `paint_selection`, first thing | `let look = crate::aniefork::selection::look(app, p.ctx());`, and then `look.shown` added to four conditions (the fill's `keep`, the outline, the active cell's own stroke, the fill handle) |
| `crates/ui-egui/src/grid.rs`, `paint_selection`, the outline | `crate::aniefork::selection::paint_outline(p, outline, look.width, t.sel_border);` in place of upstream's `rect_stroke` |
| `crates/ui-egui/src/grid.rs`, `paint_selection`, the fill handle | `crate::aniefork::selection::paint_fill_handle(p, outline, look.width, t.sel_border);` in place of upstream's three lines that size and paint it |
| `crates/ui-egui/src/grid.rs`, `in_cell_editor`, the editor's size | `let grown = crate::aniefork::selection::editor_size(geo, sh, cell, rect, size, lines, text_w);` in place of upstream's `w` and `h`, and `Rect::from_min_size(rect.min, grown)` for `erect` |
| `crates/ui-egui/src/grid.rs`, `in_cell_editor`, the editor's background and border | `crate::aniefork::selection::paint_editor(app, &painter, geo, rect, grown, text_w, fill);` in place of upstream's `rect_filled` and `rect_stroke` |
| `crates/ui-egui/src/lib.rs`, `Services` | the field `confirm`: a yes/no question asked by the operating system, which the UI can't answer for the user |
| `crates/ui-egui/src/lib.rs`, `run_ui_command`, first thing | `aniefork::run_command(self, id, p)` runs the fork's UI commands and returns early |
| `crates/ui-egui/src/lib.rs`, `logic`, right after `control::poll(self, ctx);` | `aniefork::poll(self, ctx);` does the fork's work of every frame: it notes whether the control channel is putting input into the window, carries out the calls projects' panels made, and sees to what agents asked of the Development server |
| `crates/ui-egui/src/lib.rs`, `ui`, after `panes::show(self, ui);` | `aniefork::dock::show(self, ui);` draws the pane of a project's docked panel, before the grid takes what is left |
| `apps/gridcraft/src/main.rs`, the `mod` lines | `mod aniefork;`: the fork's part of the shell |
| `apps/gridcraft/src/main.rs`, `App::ui`, after `self.0.ui(ui);` | `aniefork::frame(&mut self.0, ui.ctx(), _frame);` puts the docked panel's web view in its place |
| `apps/gridcraft/Cargo.toml`, the Windows dependencies | `wry = { version = "0.57", default-features = false }`: the system web view |
| `apps/gridcraft/Cargo.toml`, the Windows dependencies | `tao = { version = "0.37", default-features = false, features = ["rwh_06"] }`: a second window with a message loop of its own, for the Development window as a page |
| `crates/ui-egui/src/control.rs`, `handle`, first thing | `if let Some(why) = crate::aniefork::control_refuses(p) { return err(why); }`: GridCraft's own settings folder is out of the control channel's reach |
| `crates/ui-egui/src/control.rs`, `handle`, `engine.commands` | after upstream's UI commands are listed and before the listing is filtered: `v.extend(crate::aniefork::commands());` |
| `crates/ui-egui/src/ribbon.rs`, `show`, after `tabs` is made from `TABS` | `crate::aniefork::developer::tabs(app, &mut tabs);` adds the Developer tab when it is on |
| `crates/ui-egui/src/ribbon.rs`, `show`, the match on the active tab | the arm `"Developer" => crate::aniefork::developer::ribbon(app, ui)` |
| `crates/ui-egui/src/backstage.rs`, `options`, after the formula bar checkbox | `crate::aniefork::options_page(app, ui);` draws the fork's two settings |
| `crates/ui-egui/src/dialogs.rs`, `Dialog::open`, the last arm of the match (upstream's is `_ => return None`) | `name => …` asks `crate::aniefork::dialog(app, name, &params)?` for a title, starting values and a command, and builds a custom dialog from them |
| `crates/ui-egui/src/dialogs.rs`, `show`, before the `_ => form(…)` arm | `_ if crate::aniefork::draws_dialog(&d.name) => crate::aniefork::dialog_ui(…)` draws any of the fork's dialogs |
| `crates/ui-egui/Cargo.toml`, `[dependencies]` | `gridcraft-aniefork-dev = { path = "../aniefork-dev" }` |
| `apps/gridcraft/src/main.rs`, `services()` | `confirm` is a native message dialog (`rfd::MessageDialog`, Yes/No) |
| `xtask/src/layers.rs`, `TABLE` and `INTRA_LAYER_ORDER` | the crate `aniefork-dev` at layer 6, and the chain `["aniefork-dev", "ui-egui"]` so the UI crate may use it |
| `Cargo.lock` | the new crate's entry, with `getrandom` among its dependencies; and `wry` and `tao` with what they bring (`webview2-com`, `dpi`, `windows` and others); and `cfb` and `encoding_rs`, for reading a VBA project. When upstream changes dependencies, take upstream's file and let `cargo` add the entry back |

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
  in `grid.rs`, `UiState` and `run_ui_command` merged by themselves. The 0.6.1 merge conflicted
  at one: `services()` in the app's `main.rs`, where upstream replaced the `print` service with
  `printers` and `print_native`, on the lines right above the fork's `confirm`.
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

On Windows the gate's last two steps fail on an untouched checkout: `cargo xtask parity --check`
compares the generated text with `docs/parity-checklist.md` (`docs/parity.md` before 0.5.0) byte
for byte, and `cargo xtask keytips --check` does the same for its two generated files; with
`core.autocrlf=true` the checked-out files have CRLF line endings. It is not a sign that the
catalog or the key tips changed. To check them there: run `cargo xtask parity` and
`cargo xtask keytips`, look at `git diff --ignore-cr-at-eol` (nothing means they are up to
date), and restore the files. The other steps (fmt, clippy, tests, assets, layers, wasm) are
meaningful there.

As merged on 2026-10-13, upstream's `main` (`fc45b7f`, 0.6.1) passes every one of its tests
on this machine, and so does the fork with it merged. The three that failed at the merge
before (two in the engine's CSV saving, one for a line missing from the French catalog) were
fixed upstream in 0.6.0; while they failed, the gate was red on every pull request, ours
included.

## Log

Newest first.

- **2026-10-13 (merge of upstream 0.6.1):** The owner asked for the sync. Upstream `fc45b7f`,
  6 commits: printing through its shared `craft-print` library, with printers found and jobs
  sent from the app; the print commands listed on the control channel; delimited files that
  keep a number whole (the fix for the two CSV tests); the 0.6.0 and 0.6.1 releases. One
  conflict, in `apps/gridcraft/src/main.rs`: upstream replaced the `print` service with
  `printers` and `print_native` on the lines above the fork's `confirm`. Resolved as the rule
  says: upstream's code, and the hook put back after it. Every hook is there after the merge
  (counted, file by file, before and after). What the hooks hand to upstream's code needed
  nothing: the two new commands are listed in the shape the fork's are, and no interface
  language was added. `Cargo.lock` gained upstream's new packages and the fork's crate its
  new version. Checked: fmt, clippy, all 1568 tests of the workspace (none failing, for the
  first time on this machine), assets, layers, wasm; the two generated-file checks by content
  (see Building). The two open pull requests were brought up to upstream's `main` the same
  day, each after upstream's own gate passed on the result: #463 and #549.
- **2026-10-13 (module becomes project):** The owner asked for one word. Renamed, in the
  fork's code and nowhere else: the words in every window and on a request's card; the
  manifest, `project.toml`; the Development server's tools (`list_projects`,
  `read_project_file`, `create_project`, `write_project_file`, `delete_project_file`,
  `trust_project`, `stop_trusting_project`), their argument `project`, the `reference` topic
  and the keys of `development_status`; `developer.projects` and the Projects window;
  `developer.console {project?}`; the key `project` a panel's `spreadsheet.info()` gives; the
  folder `projects` and `trusted-projects.json`; and the types and files of the code
  (`project.rs`, `projects.rs`). Kept, because they mean something else: a VBA module (and
  `module = "…"` in `rebuilt.toml`), a Python module, a source file of the code. **What was
  made before is read where it is**: a project whose manifest is `module.toml`, a settings
  folder with `modules` and `trusted-modules.json`, a trust file that says `modules`; a
  folder with both manifests is refused. Nothing is renamed for the person, since renaming a
  file changes the fingerprint they trusted. The page and the showcase were rebuilt from
  their renamed sources. Five comments of the fork's hooks in upstream files say project
  (no hook added or moved). Found by a test on the way: the rename had turned the key of a
  VBA module into `project` in one place. Checked by 3 more tests (the two manifests, the
  older names in the settings folder, an older trust file) with the rest following the
  names, and in the real window on both layouts: a settings folder from before the rename,
  whose project is listed and still trusted, and a fresh one, where an agent's
  `create_project` was approved and wrote `projects/…/project.toml`.
- **2026-10-13 (the Development window by kit and project):** The owner gave the
  Development window its shape: Development at the top, the MCP server under it, the
  development kits, and in ours (named by the owner: the Anie Development Kit) a list of
  projects, as workbook code or libraries. Built: the window's list and parts, said once
  for both windows (`kits.rs`); the page redrawn around the list (five new Svelte files,
  the window a little wider); a new project made in the kit's part; Developer ▸ Macros
  showing what was rebuilt and copying a request for an agent. No hook in an upstream file,
  no new dependency. Found on the way: a separator between a list and what stands beside
  it made GridCraft's own Development and Macros windows grow taller with every frame; the
  line is painted now, and both windows are tested to keep their size. Checked by 6 more
  tests in the UI (with older ones changed to follow the window), and in the real window:
  a plan of eight steps proposed over the server's protocol, approved on its card with one
  click, carried out, and the script's answer compared with what the macro's sum gives.
- **2026-10-13 (an agent rebuilds macros, phase V1):** The owner said the phases for
  macros stand, and asked for the agent rebuild next. In the Development crate: a project's
  `workbook` line and its kind, workbook code or library (`manifest.rs`, and `kit.rs`,
  which also lists the kits: one); the note `rebuilt.toml`, and what the projects of a
  workbook say became of each procedure (`rebuilt.rs`); a starter that can be a workbook's
  code; on the server, `reference` topic `macros`, `list_macros` with what was rebuilt,
  `development_status` with the kits, and a module listed with its kind, its workbook and
  its note. No hook in an upstream file, no new dependency. Checked by 10 more tests.
- **2026-10-13 (macros, to read):** The owner asked for a VBA viewer next. A reader for the
  project a workbook carries (`vba.rs`: the container through the `cfb` crate, the rest from
  Microsoft's published format, every length bounded and counted as it is unpacked, which
  is why it is not the `ovba` crate); Developer ▸ Macros; `developer.macros`; and
  `list_macros` and `read_macro` on the Development server (twenty tools), at "Read only"
  and up. No hook in an upstream file. Excel 16.0 was asked, black box, what it keeps of
  things added to an `.xlsm`: streams in `vbaProject.bin` until a macro is edited, a custom
  XML part always, and it refuses a workbook with an undeclared part. Checked by 10 more
  tests (5 in the reader, 1 in the server, 4 in the UI), against the format's three worked
  examples, against a project Excel made, and on screen in the real window with that
  workbook.
- **2026-10-12 (Development subsystem, phase 2d, second slice: the Development window as a
  Svelte page):** The owner asked for the window as a new window in Svelte, and whether in
  Electron. Built in the system's web view instead (ADR-0001 says why): a second window with
  its own thread and message loop (`tao`), a page compiled into the app and served under a
  policy that allows only itself, state handed to it as JSON and its clicks taken through
  the dialog's own `act`. To keep the two windows saying one thing, what a card says moved
  out of the dialog into `card.rs`, which both draw from; the dialog's 97 tests passed
  unchanged through that. One hook: a line in the app's `Cargo.toml`. Checked by 7 more
  tests (2 in the UI: the card's blocks as the page is given them, and the page's half; 5 in
  the app: what is served, the policy, the page's own address, how it is told things, the
  generated file), and in the real window on
  Windows 11 with settings of its own: the page showed the app's colours, kept up as an
  agent asked, and with clicks posted to the web view a plan was looked at, approved in one
  click and carried out (create, write, trust, run).
- **2026-10-12 (plans, after an adversarial review):** One finding of weight, closed: a plan
  now holds how its modules stood when it was proposed and is approved only for that.
  Also closed: one spelling for a file; Python writes no compiled files into a module; what
  doesn't show in a file is written out; a change too long to show as changes is shown
  whole; a plan's purpose is quoted as the agent's; and smaller ones (ADR-0001 lists them).
  6 more tests (2 in the crate, 4 in the UI), and more asked of the ones that were there.
- **2026-10-12 (Development subsystem, phase 2e, second slice: approved plans):** The owner
  asked for the next slice. An agent proposes a plan of up to 20 steps; it is followed
  through first (`sights`), shown whole on one card, and answered once; its steps are then
  carried out by number, in order, without another question, each checked against things as
  they are and against how the plan left its module (a fingerprint kept with the plan). A
  step that fails ends the plan; so do its time, a lowered level, Stop Plan, `stop_plan` and
  closing. Four tools more (eighteen). In the crate: `Action::Plan`, the follow-through, the
  plan's state in the server. In the UI: the card, the Plans part of the window, and the
  checks before a step. No hook added. Also: a module's id on a card was made from its name
  in another way than the module is created with (a name with a quote or an ampersand
  showed one id and made another); both now use `starter::id_for`. Checked by 11 more tests
  (8 in the crate, 3 in the UI, one of which runs a real script as a plan's fourth step),
  and on screen in a window with settings of its own.
- **2026-10-12 (Development subsystem, phase 2d, first slice: a docked panel and the Svelte
  showcase):** The owner asked for Svelte and the web view to be showcased in the Developer
  section. A trusted module's panel can now be docked beside the sheet in the system's web
  view (WebView2, through `wry`, Windows only), over the same bridge, and is kept to its own
  address unless the module was granted `network`. The UI crate knows what is docked and
  where; the desktop shell has the web view. Developer ▸ Development creates a showcase
  module whose panel is a Svelte app (source and built script both in the module, generated
  into `showcase_files.rs`). Hooks: one in `lib.rs`, two in the app's `main.rs`, one line in
  its `Cargo.toml`. Checked by 8 more tests (2 in the crate, 4 in the UI, 2 in the app), in
  the real window on Windows 11 (the showcase docked, following the selection, giving way to
  a dialog), and in a browser against a copy of the app with settings of its own (both of
  the panel's buttons wrote to the sheet).
- **2026-10-12 (the Development server: the owner's two answers on the card):** A yes to a
  request is one click when GridCraft runs without the control channel, since nothing else
  can click in its window then; with the channel it is still confirmed by the operating
  system. And the card has a third choice, "yes, and don't ask again", for changes to one
  module's files or for running its code, until GridCraft closes: kept by the server, listed
  in the window with a way to be asked again, ended by lowering the level, never offered for
  trusting a module or for a module kept trusted as it changes. Checked by 4 more tests (1 in
  the crate, 3 in the UI).
- **2026-10-12 (Development subsystem, phase 2e, first slice: the Development server and the
  Development window):** The owner decided the shape the same day: a second MCP server in the
  Development area, a management window, a level of agent access up to full control, plans
  later; and asked for the request dialog to be like a coding agent's permission prompt. Built:
  the server in the crate (MCP over HTTP on the loopback address, a token, fourteen tools,
  requests with lapse and back-off, a fence round what modules wrote); Developer ▸ Development
  (the level, the setup, who connected, the tools); the card a request is answered on, with a
  line-by-line comparison for a file that is there. An adversarial review before the commit
  found seven things worth closing, and they are closed (ADR-0001 lists them): among them a
  file that could be changed between the trust check and Python reading it, and the control
  channel opening the token file. One new hook in an upstream file (`control.rs`: the settings
  folder is out of the control channel's reach). Checked by 50 more tests (24 in the crate, 26
  in the UI, with the machine's Python), and by a client over a real connection against a copy
  of the app with settings of its own. Not yet tried with Claude Code itself.
- **2026-10-12 (Development subsystem, phase 2b: the workbook as objects, and Python calling
  back):** A script imports `spreadsheet` and works on the sheet itself, with what its module
  was granted, checked for every call as a panel's are. Panels and scripts share `sheet`,
  `range` and `workbook`, which are commands with the address filled in: nothing was added to
  the engine. No new hook in an upstream file. Checked by 7 more tests (5 in the crate, 2 in
  the UI, with the machine's Python) and by hand in a browser against a copy of the app with
  settings of its own: the panel read, wrote, formatted and cleared cells through the objects,
  and a script it ran stamped the sheet itself.
- **2026-10-12 (Development subsystem, phase 2a: events and the console):** A panel is told
  when the selection moves, the workbook changes or another workbook is the one worked on
  (`spreadsheet.on`), over one request kept open on the same bridge; and Developer ▸ Console
  shows what modules' scripts print, what their panels log and what was refused. Clearing the
  console is the user's alone. No new hook in an upstream file. Checked by 17 more tests (11 in
  the crate, 6 in the UI) and by hand in a browser against a copy of the app with settings of
  its own: the starter panel showed the selection as it moved, cleaned it through Python, and
  its print and its log were in the console. Phase 2 is split into slices 2a to 2e. The owner
  decided the same day that 2e (agents and modules over MCP, on Anie_Maint's model) is taken
  up, after 2b.
- **2026-10-12 (the fill handle, and the outline offered upstream):** The fill handle is sized
  to the outline, three times its weight on the outline's inner corner: six pixels with Thin
  at 150%, as Excel's measured, where upstream's was nine. And the outline's place on the
  gridlines went into pull request #549 as a second commit, at upstream's one weight, with a
  test that asks the renderer where it puts the gridlines at five display scales. #549's check
  failed on upstream's own two CSV tests in the engine, before reaching the UI crate; nothing
  in the pull request causes it.
- **2026-10-12 (Thin by default):** The selection's outline is Thin unless chosen otherwise:
  Excel's two pixels on a display at 150%. Preferences equal to their default are no longer
  written to `ui.json`, and the weight moved to the key `selectionOutlineWeight`, because the
  first builds had written `"standard"` into every file whether or not anyone chose it. Checked
  on a copy of such a file: it reads as Thin, with the Developer tab's setting kept.
- **2026-10-12 (the outline on the gridlines):** A capture of Excel at 150% gave the measure:
  its outline is two device pixels, the gridline's own and the one outside it, on every side.
  Upstream's is three there and sits a pixel off the gridline at the bottom and right. The fork
  now paints the outline as four bars placed on the gridlines, a whole number of pixels wide at
  any display scale; Thin is Excel's two pixels at 150%. The highlight of a cell being typed
  into is the same bars. Checked by rendering at 100%, 125%, 150% and 200% for each weight
  (3 more tests, one by pixel) and by reading the pixels of a 150% render against the capture.
- **2026-10-12 (the highlight offered upstream):** Pull request #549, from the branch
  `editor-highlight` cut from `upstream/main`: the highlight of a cell being typed into is the
  selection's own outline, stays on the cell, and loses only the lines the text runs over. The
  open pull requests were read for the same work first (all of them, by their diffs): none does
  this; #456 touches the same function for another purpose. The same reading found that moving
  the Animations switch onto the Options page is already offered by someone else (#504), so the
  fork does not offer it, and that #463 has no duplicate.
- **2026-10-12 (typing past a cell):** The highlight no longer grows with the text. Upstream
  stretched the editor's border round the text; Excel leaves the highlight on the cell and
  clears the line the text runs over. The fork now does that: the editor grows over the cells
  after it a whole cell at a time, the highlight's right side goes when the text reaches it,
  and the gridlines under the text go with it. The same downwards for Alt+Enter lines, where
  two lines now take two rows (upstream's four points over took a third). Two hooks in
  `in_cell_editor` replace the two before. Checked in renders at each weight (fits, nearly,
  over, far, two lines) and by 5 more tests, two of them by pixel.
- **2026-10-12 (the selection: options, and a fix):** Two options on File ▸ Options ▸ Selection:
  the outline can be drawn thinner (Thin, Hairline), and the selection can be hidden while
  another window is active, as Excel does. Both are off until chosen. And a fix: the border of a
  cell being typed into now sits exactly where the selection's outline sits, on the gridlines.
  Upstream drew it half a point further out and made the editor about three points taller than a
  default row, so the box grew when typing began. The fork's preferences moved into one
  flattened field of `UiState`; the Developer tab's saved key is unchanged. Checked by pixel in
  renders (each weight, selected and typing; inactive with and without the option) and by 12
  new tests.
- **2026-10-12 (Options ▸ Animations):** File ▸ Options has an Animations section with the
  switch for the selection glide, where wordcraft has its own. It is drawn by the fork's hook on
  upstream's Options page and runs upstream's `view.animations`. Upstream's checkbox on the View
  tab is still there (see Open, under the Options section).
- **2026-10-12 (the modified column offered upstream):** Pull request #463, from the branch
  `recent-modified` cut from `upstream/main`: the recent list on upstream's File page tells when
  each workbook was last changed, asked of the file system on a thread of its own. It is the
  part of the fork's File page that the maintainers asked for when they closed #337. Not in the
  fork's `main`.
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
