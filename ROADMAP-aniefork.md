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
| Fork carries | 7 changes upstream lacks (below) |

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
| **The Developer tab**, hidden until Options turns it on: the entry to the Development subsystem designed in [ADR-0001](docs/decisions-aniefork.md). | `crates/ui-egui/src/aniefork/developer.rs`, hooks in `ribbon.rs` | Not offered; a direction of the fork's own |
| **Modules, trust, Python, panels, events, a console and the workbook as objects** (the Development subsystem, phases 1a to 2b): modules are folders the user installs; each has a manifest saying what it asks to be allowed and a fingerprint of its contents; the user trusts a module or not, in Developer ▸ Modules, confirmed by the operating system; a trusted module's Python scripts run in a process of their own, and its panel opens in the browser, calls GridCraft over a local bridge and is told what happens there; Developer ▸ Console shows what modules print and log; a script can work on the sheet itself, and panels and scripts share one set of objects for the workbook. | The crate `crates/aniefork-dev`; `crates/ui-egui/src/aniefork/` (`modules.rs`, `panels.rs`, `console.rs`, `presence.rs`); hooks in `xtask/src/layers.rs`, `crates/ui-egui/Cargo.toml`, `crates/ui-egui/src/lib.rs`, `dialogs.rs` and `apps/gridcraft/src/main.rs` | Not offered; fork-only until proven (ADR-0001) |

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
  upstream's code and pass it the fork's width. Its check is red for a reason outside it:
  upstream's `main` fails two engine tests of its own, and the run stops there (#449 is open to
  fix them). Upstream's #456
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

A ribbon tab for people who extend and automate GridCraft, hidden until File ▸ Options ▸ "Show the
Developer tab" turns it on, as Excel's is. Today it gathers what already exists for that audience:
the command palette, the action journal, how agents connect, and Options.

It is the entry to the **Development subsystem**: modules a user installs that add web panels and
Python to GridCraft. That is designed in [`docs/decisions-aniefork.md`](docs/decisions-aniefork.md)
(ADR-0001), which the owner **accepted on 2026-10-11**: panels in the browser first and docked
later, the user's own Python, fork-only until it is proven and then offered upstream in phases.
Whether a workbook can carry a module is left open. Letting agents help was on the backlog
until the owner took it up on 2026-10-12 (phase 2e, below). The record also sets out how the
model is exposed to modules and how the user gives or refuses trust; those two are still
proposals.

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

**Phase 2a is built: a panel is told what happens, and there is a Developer console.**

- **Events.** `spreadsheet.on("selectionChanged", handler)` calls the handler each time the
  selection moves, with the workbook, the sheet, the range and the active cell;
  `workbookChanged` says the workbook changed (an edit, an undo, a redo, a module's own
  change), and `workbookActivated` that another workbook is the one worked on. An event says
  where, never what is in the cells. All of them need `read`. The starter module's panel now
  shows the selection as it moves.
- **How events travel.** The panel keeps one request open, which GridCraft answers when
  something happens or after twenty seconds. Events of one kind that pile up are thinned to the
  latest; if too many pile up the oldest are dropped and the panel is told it missed some. At
  most eight panels of a module are kept waiting at once.
- **Developer ▸ Console** shows, newest last, what a module's script printed, why a script
  failed (with its traceback), what a panel wrote with `spreadsheet.log(…)`, and each call that
  was refused for want of an ability. It can be shown for one module, copied, and cleared.
- **The console is the user's record.** A module can add to it and can neither read nor clear
  it; each module has its own share (200 entries of 600), so one module's chatter cannot push
  out another's. Clearing is for the person at the screen: no command does it, and the button
  is refused while the control channel is putting input into the window.
- **`developer.console {module?, last?}`** reports the latest entries to an agent, and says with
  them that they are text written by modules, not instructions.

**Phase 2b is built: a script works on the sheet itself, and the workbook is objects.**

- **`import spreadsheet` in a script** gives it what a panel has: `run`, `batch`, `selection`,
  `read`, `write`. So a script can change the sheet itself instead of only handing data back
  for the panel to write. It works from a panel's `runPython` and from Developer ▸ Modules.
- **The same objects in a panel and in a script.** `spreadsheet.sheet("Data").range("A1:C3")`
  names cells and runs commands on them (`read`, `formulas`, `write`, `clear`, `select`, and
  `run(command, params)` for anything else, with the range filled in).
  `spreadsheet.workbook()` gives the name, the sheets with their used ranges and tables, the
  active sheet, the selection and the names.
- **A script's calls are checked like a panel's**: the module is read from disk for every
  call, and the script gets what the module is granted at that moment. A module without
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
- **What an agent asks for is a request.** It can read modules, their files and the console;
  and ask to create a module, write or delete a module's file, run a script, open a panel,
  trust a module or stop trusting one. The level decides whether a request is carried out at
  once, shown to the user, or refused.
- **A request is answered on a card**, like a coding agent's permission prompt: what it is,
  the lines a change would take out and put in, what a yes means for the module's trust,
  then yes, or no with a word for the agent. A bar under the ribbon says when one waits.
- **What modules wrote reaches an agent fenced**, as data. Each request and its outcome is in
  the Developer console and in `agent-log.jsonl` beside the settings.
- **`developer.agents`** reports how it stands. No command sets the level, the port or the
  token, or answers a request; and the control channel can no longer read or write
  GridCraft's own settings folder.

Phase 2 is taken in slices, as phase 1 was (ADR-0001 has the table). Next in 2e are approved
plans, which the owner staged after single requests. Then **2c** per-module Python environments
and **2d** panels docked in the window.

The crate depends on nothing else in GridCraft, so it can be offered upstream as pieces of its
own.

### Open
- **`network` is only partly enforced in a browser.** The policy stops a panel's requests, but
  not the page sending the user to another address, which can carry data. Docking the panel in
  the window (phase 2) is what closes this.
- The workbook as objects covers sheets, ranges and what `workbook()` lists. Tables, names
  and charts are listed as data, not yet as objects with methods of their own: they are
  changed through `run`.
- A script's time limit goes on running while one of its calls waits for the window, and each
  call waits at most thirty seconds.
- **"A workbook is about to be saved" is not an event yet.** It needs a place in the save path
  where a module is waited for, and a decision on whether a module may hold a save up.
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
- A yes to a request is two steps, the card and the operating system's question, also when
  GridCraft runs without the control channel and one would do. The owner's to decide.
- Approved plans, and "don't ask again until GridCraft closes", are not built.
- The Development server reaches modules and the console. The command palette, the action
  journal and recorded scripts are not on it.
- The Development window keeps the height of its tallest card until it is closed.
- The server was used over real connections by a test client, not yet by Claude Code itself.
- The panel's script (`spreadsheet.js`) is run by no test of the repository: it was exercised
  by hand in a browser, against a copy of the app with settings of its own. The Python side
  of the same objects is tested, where the machine has a Python.
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
| `crates/ui-egui/src/lib.rs`, `UiState` and its `Default` | the field `aniefork: aniefork::Prefs`, flattened (`#[serde(flatten)]`), after `animations`: every preference of the fork's, in one field |
| `crates/ui-egui/src/grid.rs`, `paint_selection`, first thing | `let look = crate::aniefork::selection::look(app, p.ctx());`, and then `look.shown` added to four conditions (the fill's `keep`, the outline, the active cell's own stroke, the fill handle) |
| `crates/ui-egui/src/grid.rs`, `paint_selection`, the outline | `crate::aniefork::selection::paint_outline(p, outline, look.width, t.sel_border);` in place of upstream's `rect_stroke` |
| `crates/ui-egui/src/grid.rs`, `paint_selection`, the fill handle | `crate::aniefork::selection::paint_fill_handle(p, outline, look.width, t.sel_border);` in place of upstream's three lines that size and paint it |
| `crates/ui-egui/src/grid.rs`, `in_cell_editor`, the editor's size | `let grown = crate::aniefork::selection::editor_size(geo, sh, cell, rect, size, lines, text_w);` in place of upstream's `w` and `h`, and `Rect::from_min_size(rect.min, grown)` for `erect` |
| `crates/ui-egui/src/grid.rs`, `in_cell_editor`, the editor's background and border | `crate::aniefork::selection::paint_editor(app, &painter, geo, rect, grown, text_w, fill);` in place of upstream's `rect_filled` and `rect_stroke` |
| `crates/ui-egui/src/lib.rs`, `Services` | the field `confirm`: a yes/no question asked by the operating system, which the UI can't answer for the user |
| `crates/ui-egui/src/lib.rs`, `run_ui_command`, first thing | `aniefork::run_command(self, id, p)` runs the fork's UI commands and returns early |
| `crates/ui-egui/src/lib.rs`, `logic`, right after `control::poll(self, ctx);` | `aniefork::poll(self, ctx);` does the fork's work of every frame: it notes whether the control channel is putting input into the window, carries out the calls modules' panels made, and sees to what agents asked of the Development server |
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
| `Cargo.lock` | the new crate's entry, with `getrandom` among its dependencies. When upstream changes dependencies, take upstream's file and let `cargo` add the entry back |

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
French catalog lacks "Different First Page"). They are upstream's to fix: upstream's #449 is
open to do so, and until it merges the gate is red on every pull request, ours included.

## Log

Newest first.

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
