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
| Built so far | Phase 0 (File ▸ Options, the Developer tab), phase 1a (modules can be created, inspected and trusted) and phase 1b (a trusted module's Python scripts run, in a process of their own). Panels are not served yet |
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
| Agents using modules | Backlog. Until then the control channel and MCP cannot run a module's code |
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
HTTP server on the loopback address only, on a random port. It serves the module's `panel` folder,
a generated `spreadsheet.js` that gives the panel its `spreadsheet` object, and generated
TypeScript typings. Calls arrive as JSON and are queued to the UI thread through the same request
type the control channel uses. The panel opens in the browser first; later the same panel is
docked in the window in the system web view (WebView2, WKWebView, WebKitGTK), unchanged.

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

Python gets the same model as the panel does: in phase 2 a script can call back into the
spreadsheet through an object of the same shape, instead of only receiving data and returning it.

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

  A module that was not granted an ability cannot use it: the bridge refuses the call, and a panel
  without `network` is served with a policy that makes the browser block outside connections.
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
- **The bridge is closed to everyone else.** Loopback only, a random port, and a secret token made
  for each session and each module, checked on every call along with the request's origin. A panel
  may only run its own module's scripts, by name, from its own folder.
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
  be: the client and the typings are generated by Rust code from the command list, live only in
  the new crate, and nothing else in the repository is written in JavaScript. Starter modules are
  created by a "New Module" command from templates compiled into the app, so no sample panel or
  script is checked in. This exception has to be settled with upstream before the bridge is
  offered there.
- Python and a browser become things a module needs, not things GridCraft needs. The app, its
  tests and its gate keep working with neither installed. The tests that run a script use the
  machine's own Python and skip themselves, saying so, where there is none; everything that
  decides whether a script may run is tested without it. The bridge will be tested in-process
  over HTTP.
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
| 1c | The bridge, `spreadsheet.js`, the typings; panels open in the browser | A starter module reads the selection, runs a Python script on it and writes the result back as one undo step, from a panel in the browser |
| 2 | Panels docked in the window; events; a Developer console; per-module Python environments; Python calling back into the spreadsheet | The same starter module runs docked, reacts to the selection changing, and shows its script's output in the console |
| 3 | Worksheet functions, commands and buttons added by modules; packaging and sharing; the web build | A module's Python function can be used in a formula |

### Backlog

- A setup wizard that installs Python for the user.
- Letting agents use modules: an option that lets the control channel and MCP run a trusted
  module's scripts, but never install or trust one.
- Modules and workbooks (the open point above).
