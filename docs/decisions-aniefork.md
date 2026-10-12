# Decisions (aniefork)

Decision records for the fork, [arcanii/gridcraft](https://github.com/arcanii/gridcraft). Upstream's
own are in [`decisions.md`](decisions.md), which stays identical to upstream. The fork's rules are
in [`AGENTS-aniefork.md`](../AGENTS-aniefork.md) and its log in
[`ROADMAP-aniefork.md`](../ROADMAP-aniefork.md).

Each record says what was decided, why, and what was turned down. Where a record says
**proposed**, that part is a recommendation the owner has not yet confirmed.

## ADR-0001: A Development subsystem (modules with web panels and Python)

| | |
|---|---|
| Status | **Accepted** by the owner, 2026-10-11, with one point left open (modules carried in workbooks) |
| Built so far | Phase 0 (File ▸ Options, the Developer tab), phase 1a (modules can be created, inspected and trusted), phase 1b (a trusted module's Python scripts run, in a process of their own), phase 1c (a trusted module's panel opens in the browser and calls GridCraft over a local bridge), phase 2a (a panel is told what happens in GridCraft; a Developer console shows what modules print and log) and phase 2b (the workbook as objects, for panels and scripts alike; a script calls the spreadsheet while it runs) |
| Still proposed | How the model is exposed and how trust works in detail (both below): the owner set the requirement, the mechanics are a recommendation |

### Context

People extend Excel with VBA, with Office Add-ins (web task panes) and lately with Python. Upstream
GridCraft has ruled VBA out, and lists a plugin or custom-function API as a gap that needs an owner
decision ([gaps.md](gaps.md), item 21). The fork wants its own answer: a **Development subsystem**
in which a user adds **modules** that give GridCraft new abilities.

What GridCraft already has, and this builds on:

- **Everything is a command.** Every action is a command with an id and JSON parameters
  (`cell.set`, `home.bold`, `data.sort`, 313 of them), and reading is too (`sheet.read`, `cell.get`,
  `document.inspect`).
- **Two front doors to those commands.** The control channel (JSON lines on a local port, opt-in
  with `--control`) drives the running app; the MCP server (`gridcraft-cli mcp`) offers the same
  commands to AI agents, headless or bridged to the app.
- **Recorded scripts.** A script is a named list of commands replayed in order. It has no
  variables, loops or user interface.

What is missing is a way for a person to write real code and a real interface against the
spreadsheet. MCP does not fill that gap: it lets an AI agent drive GridCraft from outside. It is
not a way for a developer to ship a tool to a spreadsheet user.

### The first developer model

The experience the owner asked for, which this record designs for:

1. The developer builds a control panel in TypeScript with ordinary HTML and CSS.
2. The panel passes data out with one call:
   `const result = await spreadsheet.runPython("my_script.py", data);`
3. The Python script does the work (pandas, a model, anything) and returns JSON to the panel.

### What the owner decided (2026-10-11)

| Question | Decision |
|---|---|
| Where panels appear | In the user's browser first; docked inside the GridCraft window later, over the same bridge |
| Which Python | The user's own. A setup wizard that installs Python for the user is wanted later (backlog) |
| Modules carried inside workbooks | **Open.** Microsoft embeds code in workbooks; GridCraft may find a different approach. See "Open: modules and workbooks" |
| Agents using modules | Backlog at first; **decided 2026-10-12: taken up as phase 2e, after the workbook as objects (2b)**, on the model of Anie_Maint's console. Until it is built the control channel and MCP cannot run a module's code |
| Fork or upstream | Fork-only until it is proven, then offered upstream in phases. It must be modular enough for that |

Two requirements came with the decisions, and shape the two sections after the next:

- **Expose the GridCraft model**, so that a module can do what a user could do, and more.
- **A careful security model**, in which the user chooses to trust a module or not.

### The shape

**1. A module is a folder.** It holds a manifest, a panel and scripts:

```
clean-data/
  module.toml          id, name, version, author, what it asks to be allowed
  panel/index.html     the panel: HTML, CSS and built JavaScript
  python/clean.py      scripts the panel may run
  requirements.txt     optional: the Python packages it needs
```

Modules the user installs live in a `modules` folder beside the user's settings. A module does
nothing until the user trusts it.

**2. Panels are web pages, served by GridCraft over a local bridge.** The desktop app runs a small
HTTP server on the loopback address only, with a random port for each module it serves (so the
browser keeps two modules' panels apart). It serves the files of the module's panel folder, a
`spreadsheet.js` that gives the panel its `spreadsheet` object, and TypeScript typings made from
the command list. Calls arrive as JSON. Those that need the workbook are queued to the UI thread,
in a queue of the same shape as the control channel's, and carried out once a frame. The panel
opens in the browser first; later the same panel is docked in the window in the system web view
(WebView2, WKWebView, WebKitGTK), unchanged.

TypeScript is the developer's authoring language; a module ships the JavaScript their own tools
build. GridCraft does not compile TypeScript.

**3. Python runs as a separate process, using the user's own Python.** `runPython` starts (or
reuses) a worker for that module, with the interpreter named in Developer ▸ Modules or found on
the system. The script must be one of the files in the module's `python` folder, and has a
`main(data)` function: data goes in and what `main` returns comes back, as JSON, one line each
way over the worker's standard input and output. What the script prints is kept apart from its
reply and handed to the caller. There is a time limit and a stop.

A separate process is what keeps the "never crash" rule: a script that crashes, hangs or runs out
of memory cannot take the workbook with it. A worker that stays alive between calls keeps pandas
loaded, which is what makes a second call feel instant. Packages from `requirements.txt` go into
an environment of the module's own, and only when the user asks.

**4. Developer features are off until asked for.** The Developer tab is hidden by default (File ▸
Options shows it), and every module starts untrusted.

**5. The code lives in a crate of its own, in pieces that can be offered upstream one at a time.**
A new `crates/aniefork-dev` at layer 6, beside `mcp`, with no dependency on the UI toolkit. Its
parts depend on each other in one direction only, so each can be proposed upstream once the one
before it is accepted:

| Piece | Depends on | Runs code? |
|---|---|---|
| Manifests, the module list, fingerprints and the trust store | nothing | no |
| The Python host | the first | yes: Python |
| The bridge, `spreadsheet.js` and the typings | the first | yes: panels |
| Docked panels, events, the console | the bridge | no more than the bridge |

The Developer tab, the Modules dialog and the console are in the UI crate's `aniefork` module, and
File ▸ Options is separate from all of it.

### Exposing the model (proposed)

The requirement: a module can do what a user could do, **and more**. Read here as three layers.

| Layer | What a module gets | When |
|---|---|---|
| **Everything a user can do** | Every command, by id: `spreadsheet.run("home.bold", {range: "A1:C1"})`. Undo, the action journal and every future command come with it | Phase 1 |
| **The workbook as objects** | `spreadsheet.workbook`, sheets, ranges, cells, tables, names, charts and styles to read and change as objects, built on the commands. Whatever the model holds and no command returns yet gets a read command, so nothing in a workbook is out of a module's reach | Phase 1 for values, formulas, formats and structure; the rest as needed |
| **More than a user can do** | Several changes as **one undo step**. **Events**: the selection moved, cells changed, a workbook opened or is about to be saved. **Additions to GridCraft itself**: worksheet functions written in a module, commands and ribbon buttons a module adds, its own panels | One undo step in phase 1; events in phase 2; additions in phase 3 |

Python gets the same model as the panel does: a script can call back into the spreadsheet
through an object of the same shape, instead of only receiving data and returning it.

**What phase 2b built of this: the workbook as objects, and Python calling back.**

- **A script imports `spreadsheet`** and has what a panel has: `run`, `batch`, `selection`,
  `read`, `write`. It is a module GridCraft gives the script's process, not a file. A call
  through it travels on the line the script's result will come back on, and the script waits
  for its answer.
- **The workbook as objects**, the same in a panel and in a script:
  `spreadsheet.sheet("Data").range("A1:C3")` is a handle that names cells and runs commands on
  them: `read`, `formulas`, `write`, `clear`, `select`, and `run(command, params)` for every
  other command, with the range filled in. `spreadsheet.workbook()` gives the workbook as it
  is now: its name, its sheets (each with its used range and its tables), the active sheet,
  the selection, the names. A handle costs nothing until it is used.
- **Built on the commands, with nothing added to the engine.** Every command already takes a
  range that names its sheet (`'My Data'!B2:C3`), so a range on another sheet is an address,
  and an object is a command with the address filled in. Whatever the objects don't name
  (charts, styles, tables as things to change) is still reached by `run`.
- **A script's calls are checked like a panel's.** The module is read from disk, and the
  user's decisions with it, for every call; the call goes into the same queue to the window,
  with what the module is granted at that moment. `python` cannot be limited, but what a
  script does through `spreadsheet` is: a module without `write` has a script that cannot
  write this way, and is told so in an error it can catch.

**What phase 2a built of this: events.** `spreadsheet.on(name, handler)` tells a panel when
something happens, until the function it resolves to is called:

| Event | Tells | When |
|---|---|---|
| `selectionChanged` | `{workbook, sheet, range, active}` | The selection moved, or another sheet was chosen |
| `workbookChanged` | `{workbook}` | Something in the workbook changed: an edit, an undo, a redo, a change a module made |
| `workbookActivated` | `{workbook}` | Another workbook became the one worked on: it was opened, or switched to |

An event says where and what kind, never what is in the cells: a handler that wants the values
asks for them, and is checked for `read` like any other call. Every event needs `read`, because
each says something about the open workbooks. Events of one kind that pile up are thinned to
the latest, so a panel is told where the selection is, not every place it has been. **"A
workbook is about to be saved" is not built**: it needs a place in the save path where a module
is waited for, and a decision on whether a module may hold a save up.

How they travel: the panel keeps one request open (`events`), which GridCraft answers when
something happens or after twenty seconds, and the panel asks again. No new kind of connection,
so the bridge's rules hold for it unchanged: the token, the module's own page, trust read from
disk for every request.

**What phase 1 built of this.** `spreadsheet.run(id, params)` reaches every command of the engine
(313 today). A module is a program calling GridCraft: commands never open a dialog for it, and the
window's own commands (the theme, the dialogs) are not offered. `spreadsheet.batch([...])` runs
several commands as one undo step, and if one fails none of them stays. `selection()`, `read()`
and `write()` are shorthands over commands. The workbook as objects is **not built yet**: a
module works through commands, and the typings list them by id with what each takes.

This is the reading of "and more" that the design assumes. If the owner means something else by
it (for instance reaching into the calculation engine, or changing how files are read and written)
it belongs in this table.

### Trust (proposed)

The requirement: the user chooses to trust a module or not. The mechanics:

- **A module starts untrusted, and an untrusted module is inert.** It is listed, and can be
  inspected, but its panel is not served and its scripts are not run.
- **The user decides, looking at what the module asks for.** The manifest names the abilities it
  wants. The Modules dialog shows them in plain words before the user chooses:

  | Asks for | Means |
  |---|---|
  | `read` | See what is in the open workbooks |
  | `write` | Change the open workbooks (every change can be undone) |
  | `files` | Open, save and export files |
  | `network` | Let its panel talk to the internet |
  | `python` | Run its Python scripts. **This cannot be limited**: a script can do anything a program the user runs can do |

  A module that was not granted an ability cannot use it. A command that only reports needs
  `read`; any other needs `write`; one that reads or writes a file (the `file.` commands, and any
  command given a `path`) needs `files`, and even then GridCraft's own settings folder, where
  the modules and the user's decisions are kept, is out of reach. A panel without `network` is
  served with a policy that makes the browser refuse its requests to anywhere but GridCraft.
  **In a browser that policy cannot stop a page from sending the user to another address**,
  which can carry data with it; that needs the panel docked in the window (phase 2), where
  GridCraft decides where a page may go. Until then `read` without `network` limits what a
  panel fetches, not where it can navigate.
- **Trust is given to exact contents.** Trusting a module records a fingerprint (a SHA-256 over
  every file in it). If any file changes, the module is untrusted again until the user looks and
  decides again. So an update, or tampering, can never ride on an old decision. A developer working
  on their own module can choose "trust, and keep trusting as I change it" for that one module.
- **Trust belongs to the fingerprint, not to where the module is kept.** That is what leaves the
  workbook question open without blocking anything.
- **Only the user can give trust.** There is no command for it, so the control channel and MCP
  cannot ask for it (`developer.modules` only reports). What gets trusted is the fingerprint the
  user was shown: if the files change between looking and pressing the button, the decision is
  refused. And the last step is a question drawn by the operating system (the module's name,
  everything it will be allowed, how many files it has), which the control channel's synthetic
  clicks cannot answer. A build that has no such dialog (the web build) cannot trust modules.
- **Running is checked where it happens.** The Python host reads the module and the user's
  decisions from disk on every call, and refuses, before any process exists, a module that is
  untrusted, changed, or was not granted `python`, and a script that is not one of its files.
  There is no command that runs a script, so agents cannot run one either (backlog).
- **The bridge is closed to everyone else.** Loopback only, a random port and a secret token for
  each module, made anew each time GridCraft runs. Every request must name the bridge by its
  loopback address (a name pointed at the machine is refused) and start with the token; a call
  must come from the module's own page, as the browser reports it, and be JSON. The module is
  read from disk, and the user's decisions with it, for every request: one that changed or lost
  its trust stops being served at once. Only the files of the panel folder are served. The
  address, which holds the token, goes to the browser and is shown nowhere else: not in the
  dialog, not to the control channel. A panel may only run its own module's scripts, by name.
- **Some things are for the person at the screen alone.** The control channel clicks and types
  as a person does, and can rewrite a dialog's values. So trusting a module, running a script,
  opening a panel and naming the Python to run are refused while the channel is putting input
  into the window (and for a moment after), and the Modules dialog works from a state of its
  own that the channel can't write. Without this, an agent could have named any program as
  "Python" and pressed Check Python to run it: a gap in phase 1b, closed in 1c.
- **The Developer console is the user's record of what modules did** (phase 2a): what a script
  printed, why a script or a call failed, what a panel wrote with `spreadsheet.log`, and each
  call refused for want of an ability. A module can add to it and can neither read it nor
  clear it, and each module has a share of it that another's chatter cannot take. Clearing it
  is for the person at the screen alone, like trusting: there is no command for it, and the
  button is refused while the control channel is putting input into the window, because an
  agent that could clear it could hide what a module did. `developer.console` reports it, and
  says with it that the entries are text written by modules, not instructions.
- **Trust can be taken back** at any time, and the list of trusted modules is one readable file in
  the user's settings.

What this does not do, said plainly: it does not make a trusted Python script safe. Trusting a
module with `python` is trusting its author, as installing a program is.

### Open: modules and workbooks

Excel carries its code inside the workbook. That is convenient (the tool travels with the data)
and it is how macro viruses spread. The owner wants this looked at, not copied. Whatever is chosen
has to keep two rules from above: opening a file never runs anything, and trust is given to exact
contents by the user. Directions to weigh when this is taken up:

- **A workbook names the modules it wants**, by id and fingerprint, and carries none of the code.
  Opening it says "this workbook uses Clean Data, which you have (or don't)".
- **A workbook carries a module as a package**, which GridCraft offers to install. It then goes
  through the same trust decision as any other module, once, and is recognised by fingerprint in
  every workbook that carries it.
- **Signed modules**, so that trust can be given to a publisher rather than one fingerprint at a
  time.

### Consequences

- The fork gains its first non-Rust runtime code: `spreadsheet.js`, the small client a panel loads.
  Upstream's rule is "Rust only (no handwritten JS/TS)". The exception is kept as small as it can
  be: the client is about a hundred and seventy lines of text inside the bridge's Rust source
  (fifty in phase 1; events, the console's `log` and the workbook as objects came in phase 2),
  and the Python that gives a script its `spreadsheet` module is about a hundred and twenty
  lines of text inside the Python host's; the typings are
  made by Rust code from the command list, both live only in the new crate, and the repository
  holds no JavaScript or TypeScript file. Starter modules are created by "New Module" from
  templates compiled into the app, so no sample panel or script is checked in. This exception
  has to be settled with upstream before the bridge is offered there.
- The crate gains one dependency, `getrandom`, for the bridge's tokens (already in the workspace
  through other crates). The HTTP server is written on the standard library, so there is no web
  framework to keep patched; it answers one request per connection, with every size bounded.
- Python and a browser become things a module needs, not things GridCraft needs. The app, its
  tests and its gate keep working with neither installed. The tests that run a script use the
  machine's own Python and skip themselves, saying so, where there is none; everything that
  decides whether a script may run is tested without it. The bridge is tested in-process over
  real connections, sent what a browser would send and what it never would.
- A script's process is not confined: it can start processes of its own, and stopping the script
  stops only the Python that GridCraft started.
- The web build has no local processes. There, panels can be frames in the page, but `runPython`
  reports that it is unavailable until a browser Python (Pyodide) is added.

### Alternatives turned down

| Alternative | Why not |
|---|---|
| VBA compatibility | Upstream ruled it out; a language and object model of that size is years of work, and it is the origin of macro viruses |
| Python inside the app's process (embedding CPython) | A crash or hang in a script would be a crash or hang of GridCraft; it also ties each build to one Python version |
| A Rust Python interpreter | It cannot load pandas or NumPy, which is the point of step 3 |
| Shipping a Python with GridCraft | Large, and ours to keep patched. The owner chose the user's own Python, with a setup wizard later |
| Panels described in a format GridCraft draws itself | Loses "TypeScript and standard CSS/HTML", the first thing asked for |
| A web view from the first day | It works on fewer platforms and couples the panel to window code before the bridge and the API are proven; it comes second, over the same bridge |
| Modules as MCP servers | MCP serves tools to an AI model. A panel needs a call that returns data to a page; borrowing MCP's shape would add ceremony and nothing else. Offering a module's scripts to agents as MCP tools is on the backlog |
| Trust given to a module's name or folder | A changed file would inherit the old decision. Trust goes to the fingerprint |

### Phases

| Phase | What | Done when |
|---|---|---|
| 0 | File ▸ Options; the Developer tab, hidden by default; this record | **Done**, 2026-10-11 |
| 1a | The crate's first piece: manifests, the module list, fingerprints, the trust store; Developer ▸ Modules to see modules and trust or stop trusting them; a "New Module" starter. Nothing runs yet | **Done**, 2026-10-11: a starter module can be created, inspected, trusted, changed (it becomes untrusted again) and trusted again |
| 1b | The Python host, and the operating-system confirmation for giving trust | **Done**, 2026-10-11: a trusted module's script runs on data and returns JSON, with a time limit; an untrusted one is refused. Developer ▸ Modules can run a script on the selection and show what it returned |
| 1c | The bridge, `spreadsheet.js`, the typings; panels open in the browser | **Done**, 2026-10-11: the starter module's panel, open in a browser, reads the selection, runs its Python script on it and writes the result back as one undo step. The workbook as objects (the second layer of "Exposing the model") moved to phase 2 |
| 2a | Events, and the Developer console | **Done**, 2026-10-12: the starter module's panel, open in a browser, shows the selection as it moves, and what its script prints and its panel logs shows in Developer ▸ Console |
| 2b | The workbook as objects (`spreadsheet.workbook`, sheets, ranges, tables, names), built on the commands; Python calling back into the spreadsheet through an object of the same shape | **Done**, 2026-10-12: a script changes the sheet itself through `import spreadsheet`, with what its module was granted, from a panel and from Developer ▸ Modules; a panel and a script work with the same `sheet`, `range` and `workbook` |
| 2c | Per-module Python environments: the packages of `requirements.txt` in an environment of the module's own, installed only when the user asks | The starter module's script uses pandas without the user installing anything by hand |
| 2d | Panels docked in the window, in the system web view (which is what lets `network` be enforced fully) | The same starter module runs docked, unchanged |
| 2e | Agents and modules over MCP (below). **Decided 2026-10-12, to follow 2b**; its mechanics are still a proposal | An agent asks to run a trusted module's script, and it runs only on the user's yes |
| 3 | Worksheet functions, commands and buttons added by modules; packaging and sharing; the web build | A module's Python function can be used in a formula |

### Agents and modules over MCP (phase 2e)

The owner asked (2026-10-12) whether the Development subsystem has an MCP, and pointed at
Anie_Maint as a model. Today it has none of its own. GridCraft's MCP server (`gridcraft-cli
mcp`, upstream's) reaches every command, and through the running app the fork's reporting ones
(`developer.modules`, `developer.console`); it cannot trust a module, run a script or open a
panel.

**Decided the same day: it is taken up, after phase 2b.** The workbook as objects and Python
calling back come first because they settle what an agent would be asking a module to do.
What follows is the shape proposed for it, which the owner has not gone through point by
point. Anie_Maint's three decisions map onto modules closely:

| Anie_Maint | Here |
|---|---|
| D17: a read-only MCP, with what remote machines wrote fenced as untrusted data | Reading modules and the Developer console. What scripts print is text a module wrote: it reaches an agent inside a fence that says so |
| D22: every action is a request, raised as a native confirm that says what will be done and that a machine asked; only the click carries it out; off by default, turned on by hand | "Run this script of this trusted module on this data" as a request. The confirm is the operating system's, which the fork already uses for trust and which synthetic input cannot answer. A cancel is an answer, and asking again waits longer each time |
| D25: a plan approved once, whose steps then run without a click, inside a window that closes | "This agent may run this script up to N times in the next T minutes", approved once |
| Never through the MCP: the console's keys and its configuration | Never through an agent: installing or trusting a module, granting an ability, naming the Python, turning this option on, clearing the console |

What an agent asked for, and what happened, would be written to the Developer console, which
is why the console came first.

### Backlog

- A setup wizard that installs Python for the user.
- (Letting agents use modules moved out of the backlog: it is phase 2e.)
- Modules and workbooks (the open point above).
