# Decisions (aniefork)

Decision records for the fork, [arcanii/gridcraft](https://github.com/arcanii/gridcraft). Upstream's
own are in [`decisions.md`](decisions.md), which stays identical to upstream. The fork's rules are
in [`AGENTS-aniefork.md`](../AGENTS-aniefork.md) and its log in
[`ROADMAP-aniefork.md`](../ROADMAP-aniefork.md).

Each record says what was decided, why, and what was turned down. Where a record says
**proposed**, that part is a recommendation the owner has not yet confirmed.

## ADR-0001: A Development subsystem (modules with web panels and Python)

> **A module is called a project since 2026-10-13** (the owner's decision; "Development kits
> and projects" below has it). This record is left as it was written: where it says module,
> `module.toml`, `create_module` or Developer ▸ Modules, read project, `project.toml`,
> `create_project` and Developer ▸ Projects. A VBA module is still a module.

| | |
|---|---|
| Status | **Accepted** by the owner, 2026-10-11, with one point left open (modules carried in workbooks) |
| Built so far | Phase 0 (File ▸ Options, the Developer tab), phase 1a (modules can be created, inspected and trusted), phase 1b (a trusted module's Python scripts run, in a process of their own), phase 1c (a trusted module's panel opens in the browser and calls GridCraft over a local bridge), phase 2a (a panel is told what happens in GridCraft; a Developer console shows what modules print and log), phase 2b (the workbook as objects, for panels and scripts alike; a script calls the spreadsheet while it runs), the first slice of phase 2e (a Development server for agents, a second MCP server; the Development window; how far agents may go is the user's to set) and the first slice of phase 2d (a panel docked in the window, in the system web view, on Windows; a Svelte showcase) |
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
| Agents and the development tools | Backlog at first. **Decided 2026-10-12**, in four points: a second MCP server in the Development area, apart from GridCraft's own; a Development management window, opened from the Developer tab; the user sets how far agents may go, up to full control; approved plans come after single requests. Built as phase 2e (below) |
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
  which can carry data with it; that needs the panel docked in the window, where GridCraft
  decides where a page may go. In a browser `read` without `network` limits what a panel
  fetches, not where it can navigate. Docked (phase 2d, on Windows) a panel without
  `network` is kept to its own bridge: no other address, no other window, no download.
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
  Since phase 2e an agent on the Development server can ask for a module to be trusted. The
  user answers, on a card that shows what the module would be allowed and then to the
  operating system's question, unless they set agents to full control. Trust asked for is
  given to the contents the agent asked about, and never "as it changes".
- **Running is checked where it happens.** The Python host reads the module and the user's
  decisions from disk on every call, and refuses, before any process exists, a module that is
  untrusted, changed, or was not granted `python`, and a script that is not one of its files.
  There is no command that runs a script. An agent asks for one to be run through the
  Development server (phase 2e), and the host checks that run as it checks any other.
- **What was checked is what runs** (phase 2e). Whoever runs a module's code holds the module
  from before the trust check until the code has finished, and a change to its files through
  GridCraft is refused meanwhile. A file changed by another program in that moment is outside
  what GridCraft can see: trusting a module is trusting that its folder is the user's own.
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
  "Python" and pressed Check Python to run it: a gap in phase 1b, closed in 1c. The same
  holds for how far agents may go, the Development server's port and token, and the answers
  to what agents ask (phase 2e). And GridCraft's own settings folder is out of the control
  channel's reach: it can open and save files anywhere else.
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

**Taken up on 2026-10-13**: the owner asked for a workbook that carries a project, with what
was built of it for each platform. The plan, and what it has to hold to, is in "A project that
travels in a workbook, builds for a platform, and a viewer for its files", below.

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
- The Development server (phase 2e) adds no dependency: it speaks MCP over the HTTP reading
  the bridge already had, now shared between the two, and the JSON-RPC is written by hand as
  upstream's MCP server's is. The app gains a listening port when the user turns agents on,
  and at no other time.
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
| 2d | Panels docked in the window, in the system web view (which is what lets `network` be enforced fully) | **First slice done**, 2026-10-12, on Windows: a trusted module's panel is docked beside the sheet over the same bridge, kept to its own address without `network`, and the Svelte showcase runs there. **Second slice done** the same day: the Development window as a Svelte page in a window of its own. macOS and Linux are open |
| 2e | Agents and the development tools: the Development server, a second MCP server, and the Development window (below) | **First slice done**, 2026-10-12: an agent asks to write a module's file or run its script, the user reads the request on a card and answers, and it is carried out only on their yes, at a level they set. **Second slice done** the same day: approved plans, answered once and carried out step by step (below) |
| 3 | Worksheet functions, commands and buttons added by modules; packaging and sharing; the web build | A module's Python function can be used in a formula |

### Agents and the development tools: the Development server (phase 2e)

**What the owner decided (2026-10-12).** Asked how agents should reach the Development
subsystem, with Anie_Maint's console as the model, the owner set four points:

| Point | Decision |
|---|---|
| Which server | **A second MCP server, in the Development area.** Its purpose is another than that of GridCraft's own MCP server, which drives the spreadsheet: this one helps drive development work inside it. It reaches every development tool, not only the first kind of module (a TypeScript panel with Python), and can help manage them |
| Where it is managed | **A Development management window**, opened from the Developer tab: where the server is set up, and the development tools and projects are managed |
| How far agents go | **The user sets it, all the way up to full control** |
| Plans approved once (Anie_Maint's D25) | Staged in after single requests work. Built as the second slice (below) |

**What the first slice built.**

- **The server is in the running app.** `agents` in the crate is an MCP server over HTTP on
  the loopback address (one JSON-RPC message a request, answered as JSON; port 7980 unless
  the user chose another). It is in the app because the app holds the workbook, the modules
  and the question the operating system puts. Nothing listens until the user turns it on.
- **How far agents may go is a level the user sets**, by hand, in the Development window:

  | Level | What an agent may do |
  |---|---|
  | Off | Nothing: nothing listens. The default |
  | Read only | Read modules, their files and the Developer console |
  | Ask me each time | Ask for anything; each thing is done only on the user's yes |
  | Build, and ask before running | Create modules and change their files without asking; running a script, opening a panel and trusting a module still ask |
  | Full control | Everything without asking, trust and code included. It lasts until GridCraft closes: it is never kept in the settings |

  Raising the level ends in the operating system's question; lowering it asks nothing, and
  refuses what was waiting and is no longer allowed.
- **Every action is a request.** The tools that read: `development_status`, `list_modules`,
  `read_module_file`, `read_console`, `reference` (how to write a module, in GridCraft's own
  words, with the panel's typings), `request_status`, `request_log`. The tools that ask:
  `create_module`, `write_module_file`, `delete_module_file`, `run_script`, `open_panel`,
  `trust_module`, `stop_trusting_module`. A tool that asks puts a request in a queue for the
  app and waits a while for its outcome; the app decides, from the level as it is at that
  moment, whether it is carried out at once, shown to the user, or refused. A request the
  user doesn't answer lapses after twenty minutes. A no is an answer: after one the agent
  must wait before asking for anything, twice as long with each no in a row.
- **A request is answered on a card**, in the manner of a coding agent's permission prompt
  (the owner asked for that, 2026-10-12): what it is, the thing itself in a block (for a
  file that is there, the lines it would take out and put in, or the whole file), what a yes
  would mean for the module's trust, and then yes, yes and don't ask again, or no with a
  word for the agent on what to do differently.
- **One click, or two** (the owner, 2026-10-12). A yes is confirmed by the operating system's
  own question when GridCraft runs with the control channel, which can click in the window
  as a person does. Without the channel nothing but the user can click there, and the yes
  on the card is enough. Raising the level, and trusting a module by hand, ask the operating
  system either way.
- **"Yes, and don't ask again"** (the owner, 2026-10-12) covers one kind of thing in one
  module until GridCraft closes: changes to that module's files, or running its scripts and
  opening its panel. It is not offered for trusting a module, which is asked each time; nor
  for a module kept trusted as its files change, where what is written runs with nobody
  looking. A change that goes through unasked takes the module's trust back, as at "Build".
  The window lists what is not asked about, with a way to be asked again; lowering the level
  ends all of it.
- **What is on the card is what is answered.** Only the user's own click puts a request's
  card on screen or takes it off; the window opens on a card by itself only when a single
  request waits. The card and the operating system's question name the request by its number
  and by a mark of what it holds (the start of a SHA-256). A card can't be said yes to in its
  first moment on screen, nor when what it holds is too long to show whole.
- **What modules wrote is fenced.** A module's files, what its scripts print and return, the
  console's entries and why a module is broken were written by a module or its author. They
  reach an agent between markers that say so, carrying a number made for that answer, so
  that text inside can't close the fence. Without such a number nothing is handed over.
- **A change nobody looked at ends the trust in a module.** At "Build" an agent changes
  files unasked. Such a change takes the module's trust back, whatever the files hold
  afterwards: putting the old contents back does not bring it back. And a module's files are
  not changed through GridCraft while one of its scripts is being checked or run: what was
  checked is what runs. For a module the user keeps trusting as its files change, changing
  its files asks at every level but full control, and so does taking its trust back.
- **Everything is on the record:** each request and what became of it, in the Developer
  console and in `agent-log.jsonl` beside the settings.

**Never through an agent, a command or the control channel:** the level, the port, the token
(which is kept in a file of its own, copied to the clipboard when the user asks for the
setup, and neither drawn in the window nor reported); the answer to a request; choosing to
keep trusting a module as its files change; naming the Python; clearing the console.
`developer.agents` reports how the server stands, and nothing sets it.

**How this maps onto Anie_Maint.**

| Anie_Maint | Here |
|---|---|
| D17: a read-only MCP, with what remote machines wrote fenced as untrusted data | The reading tools, and the fence round what modules wrote |
| D22: every action is a request, raised as a native confirm that says what will be done and that a machine asked; off by default, turned on by hand | Every action is a request. The whole of it is shown on a card in the window, because the operating system's dialog (`rfd`) cannot show a file; the last step is that dialog. A no is an answer, and asking again waits longer each time |
| D38: the confirm counts down and cancels itself | A request lapses after twenty minutes, and its card says how long it has |
| D25: a plan approved once | The second slice: approved plans, below |
| The console MCP on a named pipe only the user's account can open | Not here: see what is open, below |

**Who can reach it, said plainly.**

- A web page cannot: a request must name the server by its loopback address, carry no word
  from a browser of where it comes from, and carry the token.
- Another user of the same computer cannot, without the token. But **the port is a fixed
  one**: on a computer other people use at the same time, a program of theirs that took the
  port while GridCraft was closed would be sent the token by an agent that tried to connect.
  A pipe only the user's account can open, as Anie_Maint's console has, would close that; it
  needs code this crate forbids itself (`unsafe`) or a relay program, and is not built.
- The control channel cannot read or write GridCraft's own settings folder (a hook added
  with this slice): not the token, not the user's decisions, not a module's files.
- **An agent that already runs programs as the user is not confined by any of this.** Claude
  Code with its shell could read the token and the settings itself. The levels bound what is
  done through GridCraft, and record it; they are not a sandbox round such an agent.
- At full control an agent can trust a module and run its code: that is running any program
  as the user, which is what the level says before it is chosen.

**Found by an adversarial review before the first commit, and closed:** a file could be
changed between the trust check and the moment Python read it (now refused while a script
is checked or run, and a change nobody looked at ends the trust); the control channel could
open the token file as a workbook, or save one over the settings (now out of its reach); a
list of messages in one request could ask for more than could be answered (one message a
request); a server that was turned off still answered clients already connected, and a
client sending a byte now and then held a connection (now neither); two requests alike but
for their contents could be swapped under the user's eyes by closing and opening the window
(the window no longer chooses a card when several wait, and requests are numbered and
marked); at "Build" an agent could take back the trust of a module kept trusted as it
changes, change it unasked and ask for trust again (taking that trust back now asks); why a
module is broken, which quotes its manifest, was told outside the fence.

**Approved plans (the second slice, 2026-10-12).** The owner asked for them next. The shape
is Anie_Maint's D25, fitted to what a plan is made of here.

- **A plan is proposed like any request** (`propose_plan`): a title, a purpose (the one
  sentence read first), up to 20 steps, and how long it may run (60 minutes unless it says,
  240 at most). A step is one of the seven tools that ask for a single thing, with its
  arguments. A plan is sent whole in the call, not named as a file: there is no folder of
  reviewed plans here as there is in Anie_Maint.
- **It is followed through before anyone is asked** (`sights`): each step must fit what the
  steps before it leave. A file deleted must be there by then; a module created must not be;
  a script runs, and a panel opens, only in a module that will be trusted at that point (so a
  plan that changes a module's files and then runs its script needs a step that trusts it
  between). A plan that doesn't fit is refused with the step and the reason, and the person
  is never shown it.
- **It is answered once, on one card:** its purpose, then every step whole, each as it will
  find things once the steps before it are done. A file written twice is compared with what
  the earlier step wrote. A step that trusts a module shows what the module will ask to be
  allowed (from the module.toml as the plan leaves it) and the files it will hold. What
  trusts a module or runs code is said again under the block, each by its step. The card
  and the operating system's question carry the request's number and a mark of everything
  the plan holds. A plan too long to show whole, or one that no longer fits things as they
  are, can't be approved.
- **A yes approves it and does nothing else.** Each step is carried out later, when the agent
  asks with `run_plan_step`, which sends the plan's number and the step's and nothing more:
  what a step does is the server's own copy of what the person was shown. Steps go in order,
  one at a time, without another question.
- **Each step is checked when it is to be done**, twice. As the same request made by itself
  would be, against things as they are (the module is trusted, the script is there). And
  its module must stand as the plan needs it: as it stood when the plan was approved, or as
  the plan's last step there left it (a fingerprint the app keeps with the plan). Anything
  else that changed the module since (the person, another program, a script writing into
  its own folder) stops the step. That is not a failure of the plan: the step can be asked
  for again once it is put right.
- **A step that trusts a module** trusts it as the plan's steps leave it. The agent names no
  fingerprint; the server works it out when the step is to be done, and the check above
  makes it the fingerprint of what the person was shown. This is the one place where trust
  is given without a question of its own. It is why a plan is asked about at every level but
  full control, whatever its steps are, and is never covered by "don't ask again".
- **A plan ends** when its last step is done, when a step is carried out and fails, when its
  time runs out, when the person lowers the level, when GridCraft closes, and when the
  person (Stop Plan, in the window) or an agent (`stop_plan`) stops it. Ending undoes
  nothing. One plan at a time, and one proposal.
- **What it changes at each level:** nothing at read only (refused) or at full control (a
  plan is approved without a question there, like everything else). At "Ask me each time"
  and "Build" it is the way to do several things on one answer.

| Anie_Maint's D25 | Here |
|---|---|
| A plan is a file, reviewed and committed, named by path | The plan is sent whole in the call |
| Every file pinned by its SHA-256, every agent by its certificate | Every module pinned by its fingerprint: at approval, then after each step of the plan that changes it |
| Preconditions and expectations a step declares (`pre`, `expect`) | Not declared. The precondition is the same for every step (the two checks above); a step passes when its tool succeeds |
| A step that fails halts the plan, or runs the one recovery step declared for it | It ends the plan. Recovery steps are not built |
| A window of 60 minutes, 240 at most; one plan at a time; Stop in the console | The same |
| Only the plan's id and the step's are sent | The same |

**The threat model for plans, stated.** An agent whose instructions were tampered with can
propose plans, each shown whole and approved or not, and run the steps of an approved one in
order, or stop it. It cannot add, reorder or change a step, or bring anything of its own to a
step when it runs. So the worst it can do with an approved plan is carry it out as approved,
or stop early: **a plan should be safe to leave after any step**, which the card says. Any
client holding the token can run or stop the plan; clients are not told apart. What a step's
script prints or returns can't become a step. The checks at run time rest on GridCraft's own
reading of the files; a program running as the user could change a file after the check and
before the step, as with any single request.

**Found by an adversarial review of plans before the commit, and closed.**

- **A plan trusted its module as it stood at the click, not as it stood when the plan was
  proposed** (the one finding of weight). At "Build" an agent could change a file the plan
  doesn't write while the card waited; the card looked the same, and the plan's trust step
  would have trusted the change. A plan now holds how each of its modules stood when it was
  proposed (in its mark too), and is approved only if they stand so still: as trust asked
  for by itself is refused when the module changed after it was asked. A plan's trust step
  now says that the files the plan doesn't write are trusted as they are, and how to read
  them first.
- **Another spelling of a file's name** (`MODULE.TOML`, where the file system takes it for
  `module.toml`) was followed through as another file: a trust step would have shown the old
  permissions. A file is now reached only under the name the module has it by, in a plan
  and outside one.
- **A script's run left compiled files in its module** (`__pycache__`) unless the user's
  environment said otherwise, so that the module was no longer the one trusted, and a plan's
  later steps there stopped. Python is now started so that it writes none. (Older than plans:
  it made any module "changed" after its first run.)
- **What a file holds that doesn't show** (a carriage return inside a line, which is a new
  line to Python and nothing to the eye; characters that turn the order of text) was drawn
  as nothing. It is now written out where it stands (`⟨U+000D⟩`), and the card says the file
  holds such characters. (Older than plans.)
- A change with more changed lines than a card shows was cut in the view of its changes:
  such a change is now shown as the whole file. A request too long to show whole is refused
  where it is answered, not only by a button that can't be pressed.
- A plan's purpose and title were the agent's words in GridCraft's voice: the purpose is now
  quoted as what the agent says, and neither may hold a character that doesn't show.
- Smaller: a step the app never began stayed "being carried out" (it goes back to waiting);
  a trust step's fingerprint is checked against the plan's own; following a plan through no
  longer takes a module for "kept trusted as it changes" after a step that ends that; a stop
  that may be the control channel's is not recorded as the person's; lowering the level takes
  hold before it is saved, so a settings file that can't be written can't keep agents where
  they were; a plan's time is also kept by the calendar's clock, for a computer that slept;
  a card's mark is worked out once, not at every frame.
- Left open by it: long lines in a block are not wrapped (the block scrolls sideways).

**Open.**

- **Plans:** recovery steps and declared expectations (a step that passes only when its
  script returns what was expected); a plan kept as a file a person reviews first; more than
  one plan at a time. A script that writes into its own module's folder stops the plan's
  later steps on that module, which is the cautious reading and may prove too strict.
- **A transport only the user's account can open** (above), and a relay for agents that only
  speak MCP over standard input and output.
- **The other development tools.** The server reaches modules, their files, scripts and
  panels, trust and the console. The command palette, the action journal and recorded
  scripts are not on it yet, nor is naming the Python (the user's, by hand). "Projects", in
  the owner's words, are read here as the modules a user is writing; if more is meant
  (folders outside the modules folder, a build step) it belongs here.
- **How the window is built** was settled the same day: the Development window stays drawn
  with the app's own toolkit, like every other dialog (the place where a request is answered
  should not be a page a script could drive), and Svelte is showcased in a module's panel,
  docked in the window (phase 2d, below). **Changed later that day**, when the owner asked
  for the window itself in Svelte: see "The Development window as a page", which says how
  the reason is kept to.
- The token is on the clipboard after the user copies the setup: whatever can paste can
  read it.
- The Trust card shows what the module asks and holds, not what changed since it was last
  trusted.
- It has been used over real connections by a test client, not yet by Claude Code itself.

### Panels docked in the window, and the Svelte showcase (phase 2d)

**What the owner decided (2026-10-12).** Asked whether the fork's own windows should be built
with a web toolkit such as Svelte, the owner chose to showcase Svelte and the web view in the
Developer section. So the windows GridCraft draws itself stay as they are, and the web toolkit
is shown where it belongs: in a module's panel, docked in the window.

**What the first slice built.**

- **A trusted module's panel is docked beside the sheet**, in the system's web view, on
  Windows (WebView2). Developer ▸ Modules has "Dock Panel" beside "Open Panel". The page is
  the one a browser would be given, from the same bridge with the same checks: docking
  changes where a panel is shown, not what it may do.
- **The window decides where a docked page may go**, which a browser doesn't let it. A module
  that wasn't granted `network` is kept to its own bridge: another address is refused, no
  other window opens, nothing is downloaded. That is the part of `network` the browser
  could not enforce.
- **Two halves.** The UI crate knows which panel is docked and where its place is
  (`aniefork::dock`): a pane on the right, with the panel's name, a way to send it to the
  browser instead, and a way to close it. The desktop shell has the window, and so the web
  view (`apps/gridcraft/src/aniefork.rs`, through the `wry` crate): once a frame it puts the
  view where the UI wants it. A shell without a web view never says it has one, and docking
  says so; the UI crate gains no dependency and stays swappable.
- **The web view gives way to what the window draws.** It is a window of its own laid over
  GridCraft's, so anything GridCraft drew there would be hidden under it: while a dialog, a
  message, a menu or the File view is up, the view is hidden.
- **Docking a panel is opening it:** the user's alone to do, refused while the control
  channel is putting input into the window. Whoever starts GridCraft can name a trusted
  module to dock as it starts (`GRIDCRAFT_DOCK`), which is how it was checked.
- **The Svelte showcase** is a module the Development window creates: a Svelte app that
  follows the selection (its range, a summary of its numbers, bars), and writes to the
  sheet. Like any module it is created untrusted; it asks for `read` and `write`, runs no
  Python and loads nothing from the network. Once the user has trusted it, the window docks
  it. Its Svelte source goes into the module with the built panel, so that whoever makes
  the module can change it.

**What it costs.**

- **A dependency, on Windows only:** `wry`, in the desktop app's crate. Other systems build
  as before and open panels in the browser.
- **The fork's second exception to "Rust only".** The showcase's Svelte source and its built
  script (55 KB, which holds the parts of Svelte it uses) are text inside a generated Rust
  file, `showcase_files.rs`, as the panel's `spreadsheet.js` is text inside the bridge's.
  The repository still holds no JavaScript file, and GridCraft still compiles none: the
  script was built once, with npm, outside the repository. What it holds of other people's
  work (Svelte, clsx, esm-env: MIT) is named with its notices in a file every showcase
  module carries. This is fork-only: it is not something to offer upstream as it is.
- The web view keeps its cache in a folder of its own beside the settings.

**Open.**

- macOS and Linux (WKWebView, WebKitGTK), which `wry` also reaches.
- One panel at a time, on the right, and it is not docked again when GridCraft next starts.
- Keyboard focus moves into the web view on a click and back on a click in the sheet;
  nothing more was done about it. Links that would open a window are refused, not handed to
  the browser.
- A module's `node_modules` can't live inside the module (a module holds 2000 files, all of
  them part of what is trusted): the showcase's source is built somewhere else and its
  output copied in.
- Checked in the real window on Windows 11 with the showcase. The starter module's panel,
  which runs Python, was not tried docked.

### The Development window as a page (phase 2d, second slice)

**What the owner asked (2026-10-12).** With a picture of the Development window: can it
appear as a new window, in Svelte, contained in Electron?

**What was decided.**

- **A window of its own, in Svelte: yes.** On Windows the Development window is now a second
  window of the app that stays open beside the sheet, showing a page written in Svelte. It
  is GridCraft's own dialog elsewhere, and wherever the page can't be shown; the user can
  also choose the dialog ("Use GridCraft's built-in window").
- **Electron: no. The system's web view instead** (WebView2, through `wry`, in a window
  made by `tao`). It is what the owner's Anie_Maint console already runs on, under Tauri,
  and on Windows it is the same engine Electron carries. Electron would be a second program
  with its own copy of that engine (some hundred megabytes), written in JavaScript on its
  own side too, and the window where requests are answered would then be a client of
  GridCraft over a channel between two programs: a way to answer a request that anything
  holding the channel could use, which is what this design keeps from existing. The page
  does not depend on the choice: one file of it (`host.js`) knows how it speaks to its host.

**This changes what was recorded earlier the same day**, that the Development window stays
drawn with the app's own toolkit because the place where a request is answered should not be
a page a script could drive. The reason stands; the page is built so that no script but its
own drives it:

- it is loaded from files compiled into the app, and nothing else is served to it;
- its address ends in `.invalid`, which no network resolves: were it ever not served from
  the app, nothing would load in its place;
- every answer carries a policy that allows the page's own script and stylesheet and nothing
  else (no script written into the page, no `eval`, nothing from the network);
- it can go to no other address, open no window, download nothing; the developer tools, the
  browser's keys and its menu are off;
- a message is taken only from the page's own address, and only a known message: anything
  else is dropped;
- everything a request holds is drawn as text, never as markup (the page's rules say so,
  and a test checks the built script for what would break them);
- the control channel can't reach it: what it clicks and types goes into GridCraft's first
  window, never into the web view.

What does not depend on the page at all: the token is never part of what the page is told;
raising the level, and a yes while the control channel is open, end in the operating
system's own question; a yes in a card's first moment on screen, or to another card than the
one on screen, answers nothing (GridCraft keeps the time and the card).

**How it is built.**

- **What a card says is said once** (`card.rs`), as blocks: words, a file's lines, the lines
  a change takes out and puts in, a plan's steps. GridCraft's own window draws the blocks
  with the app's toolkit; the page is handed them as JSON and draws them. Neither words a
  request for itself, so the two can't come to say different things.
- **A click in the page goes the way a click in the dialog goes** (`devpage.rs`, into the
  same `act`): the same checks, the same questions.
- **The shell has the window** (`apps/gridcraft/src/aniefork/window.rs`): a thread of its
  own with its own message loop, since the app's main loop belongs to its first window. Once
  a frame the UI thread takes what the page said and tells it what to show when that
  changed.
- **The page's built files are text in a generated Rust file** (`page.rs`), with the Svelte
  project they are built from kept beside them (for tests and tools only: the app doesn't
  hold the source). Two tools among the app's tests write the project out and take the built
  page back in; a test checks that the file is what the tool writes, and that the built
  script is of that source.
- `GRIDCRAFT_DEVELOPMENT=1` opens the window as GridCraft starts.

**What it costs.**

- A dependency, on Windows only: `tao`, with what it brings.
- **The fork's third exception to "Rust only"**: 58 KB of built script and stylesheet, and
  the Svelte source, as text in a Rust file. Fork-only.
- **Two windows to keep alike.** What they say is shared; how they are laid out is not, and
  a part added to one has to be added to the other.

**Said plainly.**

- A flaw in the web view's engine, or in how Svelte writes text into a page, could let what
  a request holds act in the page, and so click for the person. The policy is there for
  that case; it is still a larger surface than a window that only draws text.
- Another program running as the user can put clicks into this window, as into any window.
- It was checked in the real window with clicks posted to the web view by a program, not by
  hand; and on Windows 11 only.

**Open.** macOS and Linux (the same two crates reach them). The window's place and size are
not remembered. The keyboard alone reaches everything in it as a web page does, with no
shortcuts of its own. The bar under the ribbon still says a request is waiting while its card
is open in the page.

### Macros (VBA): reading them, and bringing what they do into GridCraft

**What the owner asked (2026-10-12 and 13).** A viewer for the VBA a workbook carries, as the
next slice: "we need to look inside the vbaProject.bin for a conversion strategy". And with
it: can information be written into `vbaProject.bin` so that Excel ignores it but keeps it;
should what GridCraft adds go into a file format of its own, breaking with Excel; a phased
way of bringing VBA into a GridCraft sheet, of which the first phase would be an agent
rebuilding the VBA project as a module (Python and a Svelte panel); and whether there are
frameworks for parsing VBA.

**What does not change.** GridCraft does not run macros, and opening a file never runs
anything. A workbook's VBA project is kept byte for byte and written back only into `.xlsm`
and `.xltm` (upstream's behaviour). Nothing here writes to a project.

**What the first slice built: the project can be read.**

- **A reader** (`gridcraft_aniefork_dev::vba`): the project's modules, what kind each is (a
  plain module, a class, the code behind the workbook or a sheet, a form), their source as
  text in the project's code page, the libraries it refers to, and an outline of each
  module: its procedures, which of them Excel would run by itself (event handlers,
  `Auto_Open`), and which live in a library outside the workbook (`Declare`). The outline is
  found by reading lines; it is not a parser of the language.
- **Developer ▸ Macros** shows them: the modules on the left, the chosen one's source on the
  right with its lines numbered, and what would not show by itself written out. Reading
  only. (Upstream has a notice named "macros" that says a workbook's VBA is kept; this is
  the window behind it, under another name.)
- **`developer.macros`** reports the same, and with `{module}` that module's source.
- **Two reading tools on the Development server**: `list_macros` and `read_macro` (twenty
  tools). They read the project of the workbook that is open, and hand its text over inside
  the fence: it was written by whoever made the workbook, and is data. **"Read only" now
  reaches the open workbook's macros**, and the level's own words say so. A macro's source
  can hold what its author put there (a password, a server's address): that is what an agent
  at that level can now read.

**Why not the `ovba` crate, which the owner named ("or equivalent").** It reads a project
well, and unpacks every stream into memory with no upper bound. The format's compression
turns six bytes into four thousand, so a project built for it would make GridCraft ask for
gigabytes and be ended by the system: a crash on opening a hostile file, which is the one
thing upstream's rules put above features. The reader here is the equivalent: it stands on
the same container crate (`cfb`), follows Microsoft's published \[MS-OVBA\] for the rest, and
counts what it unpacks as it unpacks it. A project is at most 32 MB, a stream 8 MB, a
module's source 2 MB (what is longer is cut, and says so), all sources together 16 MB. The
container crate's own faults are caught. The format's three worked examples unpack as the
document says; a project made by Excel 16 was read whole.

**What Excel keeps, found by asking it** (2026-10-13; the installed Excel 16.0, run hidden
with macros off, on three small workbooks made for the purpose and kept outside the
repository):

| What was added to an `.xlsm` | Opened and saved again | A macro edited in Excel, then saved |
|---|---|---|
| A stream at the top of `vbaProject.bin` | kept | **gone** |
| A storage of its own in `vbaProject.bin`, with a stream in it | kept | **gone** |
| A stream inside the project's `VBA` storage | kept | **gone** |
| A custom XML part (`customXml/item1.xml`), added through Excel | kept | kept |
| A part of the package that nothing declares | Excel **refused to open** the workbook | |

So Excel copies a project through untouched until someone edits a macro, and then writes
it anew from what it knows: anything else in it is lost at that moment, without a word.
`vbaProject.bin` is not a place to keep anything. A custom XML part is: it is what the file
format gives programs for their own data, and Excel kept it both times. A file dropped into
the package undeclared breaks the workbook for Excel. (One machine, one version, small
cases: enough to rule the first out, and to be checked again for the second when it is
built on.)

**Decided (2026-10-13).** The owner: "phases stand". With them stands what phase V3 rests
on, which is the first point here. The second question put with them, whether "Read only"
should reach a workbook's macros or a level of its own should, was not answered: it stays
as built.

- **No file format of GridCraft's own.** The workbook stays an `.xlsm` that Excel opens:
  its VBA untouched for those who use Excel, and what GridCraft adds carried where Excel
  keeps it. A format of its own would buy nothing the package can't hold, and cost every
  user the ability to hand the file to someone with Excel.
- **A module travels in a custom XML part, when it travels at all.** That is the open
  question of "modules and workbooks" above, now with a place found for it. What must hold
  whatever the answer: a module that arrives in a workbook arrives untrusted, shown for
  what it is, and opening the workbook runs nothing.
- **Phases.**

| Phase | What | Stands |
|---|---|---|
| V0 | Read the project: the viewer, the command, the agent's two tools | **Built**, 2026-10-13 |
| V1 | An agent rebuilds what the macros do as a module (Python scripts, a panel), through the requests and plans that exist: it reads the VBA and the workbook, proposes a plan, the person reads every file and trusts it. With it, a note the module keeps of which procedure became which script | **Built**, 2026-10-13, and tried once (below) |
| V2 | What Excel ran by itself (`Workbook_Open`, `Worksheet_Change`) answered by the module's events, for a module the person trusts, with a visible switch | Needs the events of phase 2a taken further ("a workbook was opened by this module's workbook"), and a decision on what may run without a click |
| V3 | The module carried in the workbook (a custom XML part), so that the workbook and what replaces its macros are one file | Needs the decision above, and Excel asked again at more length |
| later | Help with the language itself: a real parser, for mechanical translation of the easy cases | Only if V1 shows the need |

- **Parsers of VBA.** None in Rust was found. There are grammars for other tools:
  an ANTLR `vba.g4` (the one in SSlinky's VBA-LanguageServer; a Python wrapper,
  `antlr4-vba-parser`, calls itself a proof of concept), a tree-sitter grammar
  (`arrmee-wt/tree-sitter-vba`), and Rubberduck's, the fullest, which is under a licence
  GridCraft can't take code from. Their licences were not checked one by one. For V1 none is
  needed: an agent reads source as text, and what helps it is what the outline already
  gives (what there is, what runs by itself, what reaches outside). A parser earns its place
  when GridCraft itself has to translate, which is "later" in the table.

**V1, built and tried (2026-10-13): an agent rebuilds the macros as a project.**

- **A project can say which workbook it is the code of**: `workbook = "Budget.xlsm"` in its
  `module.toml`. A file's name only, never a path (a path is another on every computer). It
  says what the project is for and nothing else: nothing is allowed or run because of it.
- **The note of what became of each procedure** is `rebuilt.toml`, beside `module.toml`
  (`gridcraft_aniefork_dev::rebuilt`):

  ```toml
  [[procedure]]
  module = "Totals"            # the VBA module
  name = "AddUp"               # the procedure in it
  state = "rebuilt"            # rebuilt | partly | left
  script = "add_up.py"         # the script that does it now, in the scripts folder
  note = "Sums A1:A10 and writes the total below."
  ```

  It is checked like a manifest (256 KB, 2000 entries, names and notes of one line, a script
  that is a path inside the project) and a note that can't be understood says nothing. **It
  is the project's own word**: nothing checks that a script does what a procedure did, so it
  is shown as what the project says, and it is data to an agent.
- **Where it shows.** Developer ▸ Macros says, beside each module, what a project says
  became of its procedures, and how many of the workbook's procedures are spoken for; a
  script that is named and not there is said. The project's own part of the Development
  window lists the note. `developer.macros` and the agent's `list_macros` carry it
  (`rebuilt`, beside each procedure), so that an agent picks up where the work was left.
- **What the agent is told.** `reference` has a topic `macros`: read first; what each kind
  of procedure becomes (a script; the panel for a form; an event is rebuilt as a script the
  person runs, since GridCraft runs nothing by itself; a door into a Windows library is
  left, with a note); one plan for the whole; the two files' shapes; how VBA's ways of
  reaching the sheet read in a script; and not to carry a password or an address out of the
  VBA.
- **The way in, for a person**: Developer ▸ Macros has "Copy a Request for Your Agent". It
  copies a few fixed words that send the agent to the Development server and that topic.
  They hold nothing of the macros, and no token.
- **Nothing else was needed**, as expected: the plan, its one card and its steps are those
  of phase 2e.

**The trial** (2026-10-13). A copy of GridCraft with settings of its own, agent access at
"Ask me each time", and the workbook Excel made for the first slice (`AddUp`, which sums
`A1:A10` and shows the total; a private function nothing calls; an empty `Workbook_Open`).
An agent's client, speaking the server's protocol and nothing else, read how things stood,
the `macros` topic and the three modules, and proposed one plan of eight steps: create the
project, write its `module.toml` (naming the workbook; `read` and `python`), write
`add_up.py`, delete the starter's script and panel, write `rebuilt.toml`, trust, run. The
plan was read on its card and approved with one click in the window; the eight steps were
carried out without another question. The script gave `Total: 0` on the workbook as Excel
made it (text in `A1`, nothing below), and `Total: 42.5` on a copy with 5, 7.5 and 30 under
it, which is what the macro's `Sum` gives. The window then showed the project under
"Workbook code", trusted, and "1 of the 3 is rebuilt".

What the trial was not: the agent was the fork's own developer's, and the clicks in the
window were its doing too, in that copy. Claude Code itself, set up by a person, has not
done this yet. And the workbook was small: one procedure of five lines.

**Open.**

- The outline misses what a line-reader misses: a procedure written on one line, or split
  by conditional compilation.
- A form's layout (its controls) is not read: only its code. Nor are a project's password
  or its signature looked at.
- A project protected with a password still has its source in the file, and it is shown:
  that protection is Excel's editor's, not the file's.
- **Whether "Read only" should reach macros** is the owner's to say (above).
- The rebuild was tried once, on a small workbook, by the developer's own agent: not by
  Claude Code set up by a person, and not on macros with forms, events that do something,
  or calls into libraries.
- Nothing compares a script with the procedure it says it rebuilds. The note is a claim.
- An agent that rebuilds has no way to try the macro itself (GridCraft runs none), so it
  can't compare what the two give: the person has to.
- Only the workbook that is open is read; an agent is told of another when it is opened.

### Development kits and projects: the shape of the Development window

**What the owner asked (2026-10-13).** "For the development menu, apply a nice design: we
want a top level: Development kit (ours is the Python - Svelte UI). These would be loaded
mods, and others can be loaded in. Then inside our Development Kit, it would have a list of
projects. These could be workbook code or libraries. The MCP should be at the Development
top level." And the kit's name: "Anie Development Kit".

**The shape.**

```text
Development                    the whole, at a glance
  MCP Server                   agents: how far they may go, and what they ask
  Development Kits
    Anie Development Kit       Python and Svelte: the kit this build has
      Workbook code            projects that are the code of one workbook
      Libraries                projects for any workbook
```

- **A development kit** is a way of building on GridCraft: what a project is made of, and
  how it is run. The kits are listed (`gridcraft_aniefork_dev::kit`), not assumed to be
  one. The one this build has is the Anie Development Kit: Python scripts and a web panel,
  which can be a Svelte app.
- **A project** is what the kit's files and the Development server's tools call a module.
  It is one of two kinds, by what its `module.toml` says: **workbook code**, when it names
  a workbook, or a **library**. The kind decides nothing.
- **The MCP server is not a kit's.** It stands at the top, beside the kits, with everything
  the Agents part held: the level, the connection, what waits, the plans.
- **The window** has that list down its side and one part beside it: the top (the server,
  the kits and the open workbook with its macros, each at a glance), the server, the kit
  (what it runs with, its projects by kind, a new project, the showcase) and a project
  (what it is for, where it stands, what it asks, what is in it, what it says it rebuilt).
  A request's card still takes the place of all of it. A new project can be made in the
  kit's part as the code of the workbook that is open: GridCraft says which workbook that
  is, not the page.
- **Both windows draw the same** (`kits.rs` says it once, as `card.rs` does for a card):
  GridCraft's own, and the page, whose Svelte app was redrawn around the list.
- **Changed on 2026-10-13, by the owner's review**: the list down the side stops at the kits,
  a kit's projects are one list, and no project is called a library. See "The Development
  window's look and words".

**Not built, and open.**

- **There is no way to load another kit.** The list has two entries now, both compiled in
  (phase K1), and the window says so.
  (2026-10-13: the owner decided a kit is loaded from a package file. See "A development kit
  from a package file".)
  How a kit that isn't compiled into GridCraft would be loaded is not decided. What is
  known: it can't be a library loaded into the process (the workspace forbids `unsafe`, and
  a kit's code would then be GridCraft's own); the two things of this kind GridCraft
  already does are a process of its own spoken to over a pipe (Python) and a client that
  connects (an agent). A kit as a process that speaks a small protocol would be of a piece
  with both. That is a proposal, not a decision.
- **"Loaded" can't be undone**: the kit can't be turned off as a whole.
- **One word: decided and done (2026-10-13).** The owner: "rename module to project". It
  is project everywhere now: the windows' words and a request's card; the manifest,
  `project.toml`; the server's tools (`create_project`, `write_project_file`,
  `trust_project` and the rest) and their argument `project`; `developer.projects`; the
  folder `projects` and `trusted-projects.json`; the code. Three things keep the older
  word because they are something else: a VBA module (and `module = "…"` in
  `rebuilt.toml`), a Python module, a source file of the code.
  **What a person made before is read where it is**, and nothing is renamed for them: a
  project whose manifest is `module.toml` (renaming the file would change the fingerprint
  they trusted), a settings folder with `modules` and `trusted-modules.json`, a trust file
  whose list is named `modules`. A folder with both manifests is refused, since nobody
  could tell which one counts. What is not kept: the server's older tool names. An agent
  set up for `create_module` is told the tool is unknown; nothing had been released with
  those names.
- **Reviewing, trusting and running a project** is still done in Modules: the project's
  part has the way there.
  (2026-10-13: in the page it is done in the project's own part. See "The Development
  window's look and words".)
- **A project names its workbook by the file's name.** Two workbooks of one name are the
  same to it. That is enough to say what a project is for; it is not enough for anything
  that would run by itself (phase V2), which needs the decision of "modules and workbooks".

### A second kit (C), Doom as its showcase, and source control (planned)

**What the owner asked (2026-10-13).** "For our roadmap, let's show another Development kit.
Let's make a C dev kit and port Doom in as a project. We'll also need to embed git (github)
functionality inside the development system (as a top level functionality)." And, of what
Doom is to show: "we can demonstrate live variables updating in the spreadsheet."

**What the owner decided (2026-10-13)**, on the points put to them:

1. **Doom is not in this repository.** "We wouldn't put Doom in the repo - this is just a
   demonstration of a dev kit", and: "we just make a GridCraft Doom repo to demonstrate the
   GridCraft development kit system."
2. **Freedoom's game data by default**, "but we should allow for a project to change project
   files as a feature."
3. **The easiest way first.** "We should take the easiest approach first. We can always make
   a WebAssembly Dev kit later." So C was to run first as a program of its own, as Python
   does. **Changed the same day, once Doom docked had been asked for and tried: "switch to
   WebAssembly first"** (the section after K1's, below).
4. **The compiler is the person's, and named.** "As this is a development kit, we can
   specify what the compiler is (and use external like we are doing for Python)."
5. **Push later.**
6. **Signing in to GitHub: on the backlog**, with a recommendation asked for (below).
7. **The kit before source control.** "We can manually clone doomgeneric until the dev kit
   is self hosting."

Not answered, and taken the easiest way until told otherwise: how live values reach the
cells (by plain writes first; see below).

**Nothing of this is built.** The window it leads to:

```text
Development
  MCP Server
  Source Control            later: git, and GitHub
  Development Kits
    Anie Development Kit    Python · Svelte
    C Development Kit       C, built by the compiler the person names
      Doom                  a project, cloned from a repository of its own
```

**1. What a kit is, once there are two.** Today the kits are a list of one, and the rest of
the crate *is* the Anie kit. A second kit makes the line plain. What every kit shares: a
project is a folder with a `project.toml`, a fingerprint, the person's trust, what it asks
to be allowed, the Development server's requests and plans, the console. What a kit brings:
what a project's sources are, how they are built, how they are run, its starter, and what an
agent is told about writing for it. `project.toml` gains `kit = "c"`; without the line a
project is the Anie kit's, so every project there is stays as it is. The C kit is compiled
into GridCraft like the first. That shows a second kit; it does not settle how a kit that is
not compiled in gets loaded, which stays open.

**2. The C kit: C built by the person's compiler, and run as WebAssembly in the project's
panel.** (As first planned it ran as a program of its own: the owner changed that on
2026-10-13, and the three points on running, below, are as they stand since.)

- **The compiler is external and named**, as the Python is: found, or named in the window.
  First a compiler that takes the usual command line (`zig cc`, clang, gcc); Microsoft's
  after. **This machine has none on its path** (looked, 2026-10-13). zig is the candidate:
  one download without an installer, the same on every system, and it brings the C library
  and the Windows headers with it. **It builds `doomgeneric` as it stands** (tried the same
  day: below). GridCraft ships no compiler.
- **A build is not harmless, so it is a request like running a script.** It runs the
  person's compiler over files an agent may have written, and `#include` of a file outside
  the project puts that file's lines into the compiler's messages, which the agent would
  then read. So: a build asks the person at "Ask" and "Build"; what the compiler says
  reaches an agent fenced; and the recipe is a list in `project.toml` (the sources, include
  folders inside the project, a few named options), never a makefile or a command line of
  the project's own.
- **What is built is kept outside the project**, beside the settings, under the fingerprint
  of the sources it was built from. A build then never changes what the person trusted, and
  a program built from other sources than the ones trusted now can't be the one that runs.
- **It runs as WebAssembly, in the project's panel**: the page the kit serves starts the
  program and hands it the little of a system it asks for. The page makes the calls to the
  spreadsheet, and every call is checked against what the project was granted at that
  moment, as for any panel.
- **It is held to what it was granted.** A WebAssembly program can do only what whoever
  runs it hands it: a C project is the first kind that can't do more than it asked. It
  needs no leave of its own to run, as `python` does.
- **A program of its own is put aside, not ruled out**: for what a page can't do, later,
  as a thing a project asks to be allowed, worded as `python` is. Inside GridCraft by an
  interpreter written in Rust (for C that needs no panel) stays a maybe.

**The trial with zig (2026-10-13), on the owner's word** ("go ahead and try zig with
doomgeneric"). Done in a scratch folder outside the repository: nothing was installed, put
on the path, or added here.

- **What was fetched.** zig 0.17.0 for Windows from ziglang.org (one zip of 100 MB, 399 MB
  unpacked; its SHA-256 matched the one ziglang.org publishes), and `doomgeneric` cloned
  from GitHub as it stood (`e257a09`, 2026-10-11; 8 MB).
- **What was built.** The 81 files its makefile for Windows lists, by one `zig cc` command
  that names them (the makefile was read for the list and the options, not run: there is
  no `make` here), for `x86_64-windows-gnu`, with `-Os`, the makefile's two definitions,
  and the system's drawing library to link. **25 seconds from nothing, no errors**, two
  warnings (both in Doom's own code), and a program of 453 KB.
- **It runs** (the same day, on the owner's word). Freedoom 0.13.0 was fetched from its
  releases on GitHub (one zip of 24 MB; its SHA-256 matched the one published beside it),
  and the built program started on `freedoom1.wad` from a scratch folder. It opened its
  window, titled "Freedoom: Phase 1", and played Freedoom's recorded demo by itself; it
  was stopped after 50 seconds. It keeps its settings and saved games in the folder it is
  started from, and wrote nowhere else. Nobody played it: the keys were not tried.
- **What it tells the kit.**
  - A list of sources and a few named options is all Doom's build needs, so the recipe as
    a list in `project.toml` holds for the showcase. With 81 files, the list should be able
    to name a folder.
  - The target has to be said, and zig keeps a cache (61 MB after these builds; by default
    in the person's own folders, which the kit can leave or point elsewhere).
  - **zig stops a program at undefined behaviour unless the build asks for optimisation or
    says otherwise**, and Doom's arithmetic overflows on purpose. Checked with ten lines of
    C that add one to the largest `int`: built plainly, the program stops with "signed
    integer overflow"; with `-Os`, or with `-fno-sanitize=undefined`, it wraps and goes on.
    The trial passed both. The kit's named options need the second, or a project built
    without optimisation to find a fault would stop for this instead.
  - Doom prints its start-up lines to its standard output. So the calls to the spreadsheet
    can't share that pipe as it is: the kit's C file has to keep the pipe for the calls
    and send what the program prints elsewhere, as the runner does for a Python script's
    `print`. For phase K3.

**3. Doom.**

- **A repository of its own, "GridCraft Doom"**, made to demonstrate the kit: `doomgeneric`
  (GPL-2.0; a port writes five functions: start, draw a frame, sleep, the clock, a key),
  its `project.toml`, and the little that reports the game's variables. Under the GPL, as
  Doom's code requires. GridCraft's repository holds the kit and nothing of Doom: no GPL
  code is copied in (upstream's rule, and the fork's), and GridCraft is to that project
  what an editor is to a GPL program opened in it. Until GridCraft has source control,
  `doomgeneric` is cloned by hand into the projects folder. Making the repository on GitHub
  is for when the demonstration works, on the owner's word.
- **It is docked beside the sheet**, in its project's panel (it was first shown in a
  window of its own, as a program built for Windows).
- **Its live variables update in the cells** as it is played (the owner's addition), in a
  table (below): health, armour, ammunition, the monsters nearby, what was killed, the
  level's clock, where the player stands; and formulas and a chart over them that move
  with the game. It makes the showcase about the spreadsheet, not only beside it. Doom asks
  for `read` and `write`, and nothing else.
- **Its game data is Freedoom's by default** (3-clause BSD, which may be passed on with its
  licence text; the shareware WAD is id Software's, under terms of its own, and the rule on
  assets keeps it out of here in any case), kept with the Doom project, not in GridCraft's
  repository. For the demonstration on their own machine the owner asked for the shareware
  data.

**The table of the game's variables** (the owner, 2026-10-13: "can we expose some of the
internal variables in a table that will update with the game? Variables like Health, Ammo,
Nearby monsters, etc - something to demonstrate the linkage"). Yes. Every one of them is a
variable the game already keeps, and the project's one file of glue can read them with no
file of Doom's own changed. Looked up in `doomgeneric`'s source, and **tried as far as
building**: a scratch file that has the program's loop, reads the variables below four times
a second and writes them as one call (`range.setValues`, the call a Python script makes) was
built with zig together with the 81 files, with no warning of its own. **Run the same day**
on Freedoom's demo: 169 lines of values in 50 seconds, and they move as the game does
(health 100, 85, 81, then 100 again at a medical kit; the pistol given up for the shotgun;
17 of the level's 192 monsters killed), in step with the game's own status bar. They were
written to the program's output: nothing puts them in a sheet until the kit can run a
program (phase K3).

A sheet named for the game, one row a variable, the values in one column:

| Rows | What | Where the game keeps it |
|---|---|---|
| Health, Armor | The player's | The player's record |
| Weapon, Ammo | The weapon in hand, by name, and what is left for it | The player's record, and the game's table of weapons |
| Bullets, Shells, Rockets, Cells | Each kind of ammunition carried | The player's record |
| Monsters nearby | Alive, and within 1024 of the map's units of the player | Counted from the game's list of the things it moves |
| Monsters in sight | Of those, the ones with a clear line to the player | The game's own test of sight |
| Monsters alive | On the level | The same list |
| Kills, Items, Secrets | Each with the level's total beside it | The player's record, and the level's totals |
| Level, Time | Which map, and the seconds played in it | The game's state |
| X, Y, Facing | Where the player stands, and which way | The player's thing on the map |

- **"Monsters nearby" is the one that is worked out**, not read: the glue walks the game's
  list of the things it moves each time it writes. The rest are read as they stand.
- **What the workbook does with them is the workbook's**: a bar on health, a line that says
  so when something is in sight, kills as a share of the total, and a chart of health and
  of the monsters nearby over the last minute, from rows the project adds once a second.
  That is the linkage shown: the game moves a cell, and formulas, formats and a chart
  follow it.
- **Four times a second**, all the values in one call. Until values can be published
  (below), each call is a step of undo: 240 for a minute of play.
- Doom plays its recorded demos by itself when nobody is at the keys, and the variables
  move then too (seen: Freedoom's demo plays in `doomgeneric`): the table can be seen
  updating before anyone plays.
- **"Nearby" at 1024 units is too wide to say much**: on the demo's level it counted 29 to
  40 monsters throughout, while "in sight" moved between 1 and 5. The distance is to be
  made smaller, or the workbook's to set.

**Files a project changes itself** (decision 2: "we should allow for a project to change
project files as a feature"). As things are, any change to a project's files ends its
trust: that is the point of the fingerprint. Read here as: a project may have files that
are its *data*, which it or the person may change without that (another WAD, a saved game,
what it fetched), apart from the files that are its code. A sketch, to be designed when it
is built, with what must hold whatever the design:

- The data has a place of its own that the fingerprint leaves out, and the window says
  which files those are.
- Nothing in it is ever served to a panel as a page or a script, or taken as a source to
  build: otherwise what nobody reviewed would run with what the project was granted.
- A program that can't be limited reads its data as it likes, and data can be made to
  attack the program that reads it (Doom's own reading of a WAD was never hardened). So an
  agent writing into a project's data is a request that asks, like running it; it is not
  something "Build" does unasked.
- If this reading is not what was meant (for one: that a project should be able to rewrite
  its own *code*), it needs saying before it is built: that would be a different thing,
  and it would cut across trust.

**Live variables: two ways for a running project to put values in cells.**

- **By writing them, with what exists.** The project calls the granted `write` a few times
  a second, all the values in one call, into a sheet of its own. It needs nothing new. What
  it costs: **every call is an edit**. Each is a step of undo (a minute of play is hundreds
  of them, and Ctrl+Z walks back through the game), and the workbook counts as changed.
  Good enough to show it; not how it should stay.
- **By publishing them, with cells that show them.** The project hands GridCraft named
  values ("health", "ammo"); a cell shows one through a function, and is worked out again
  when the value changes. Nothing is edited: no undo steps, nothing saved but the formula,
  and the person decides where a value appears. Excel has a function for exactly this,
  `RTD` (real-time data). Upstream lists `RTD` as one it does not take, because Excel's
  reaches a program through COM; here the same name could mean "a value a trusted project
  publishes". It needs the engine to take values from outside a workbook, which is a hook
  in upstream's formula code and a larger piece of work; it would serve both kits (a
  Python project's readings or a long job's progress as much as Doom's health).
- **The other direction** (a cell that changes the game: type a number into "health") needs
  `read` and the event that says the workbook changed. After the first way works.

**4. Source control: git, and GitHub, at the top. After the kit.**

- **In slices.** Local first: which projects are repositories, the branch, what changed
  (in the view a request's card already uses for changes), a commit. Then clone from an
  address (the project arrives untrusted, like any that arrives), fetch and pull. Push and
  signing in are later (decided). Then for agents: reading tools (status, changes, log), a
  commit as a request, and a push that always asks, since it leaves the computer.
- **By a library inside GridCraft, not by running `git`.** Run in a folder that came from
  elsewhere, `git` runs what that folder's own settings and hooks name; that is a known
  way for a copied folder to run code. `gitoxide` is written in Rust and clones, fetches,
  compares and commits; its own status page lists push among what is still to be built,
  which fits push being later.
- **A project's `.git` is not part of the project**, and this is needed from the first
  slice of the kit, since `doomgeneric` is cloned by hand: `.git` changes with almost
  every git command and holds many files, so counted in the fingerprint it would end trust
  again and again. It is left out of the fingerprint, never served to a panel, never a
  source to build, and out of reach of the project-file tools. GridCraft's own git, when
  there is one, never runs anything a repository names: the two rules stand together.
- **Signing in to GitHub (backlog). Recommended**: the credential manager that git already
  uses on the computer (on Windows, the one that comes with Git). The person signs in the
  way they already do, GridCraft keeps no secret of its own and registers nothing with
  GitHub, and the token never passes through a window, an agent or a project. GitHub's
  sign-in for devices would be the way without git installed; it needs an app registered
  with GitHub.

**Phases.**

| Phase | What | Waits for |
|---|---|---|
| K1 | Kits as two: `kit = "…"` in `project.toml`, the Anie kit behind the line unchanged, the C kit listed; a project's `.git` left out of it | **Built 2026-10-13** (below) |
| K2 | The C kit builds, to WebAssembly: the compiler found or named (zig first), the recipe as a list in `project.toml`, what is built kept under the fingerprint of its sources; then a build as an agent's request | **First slice built 2026-10-13** (below): the person builds. An agent's request is the second |
| K3 | The C kit runs what it built in the project's panel: a C project may have a panel, and the kit serves beside its page what the program asks of a system | **Built 2026-10-13** (below), and seen: the starter, built by the kit, running in its panel docked beside the sheet |
| D1 | Doom, as the demonstration: `doomgeneric` cloned by hand, built by the kit, docked beside the sheet; the table of its variables written four times a second, with formulas and charts over it; then the "GridCraft Doom" repository | **Seen 2026-10-13** (below): built by the kit, docked, the sheet filling, with a table of the game's memory. The repository waits for the owner's word |
| P1 | Files a project changes itself | D1 shows what it needs |
| G1 | Source control, local: repositories among the projects, changes, a commit | |
| G2 | Clone from an address; fetch and pull | G1 |
| L1 | Live values that are not edits: a project publishes them, a function shows them (`RTD`) | A hook in upstream's formula code |
| later | Push and signing in to GitHub; C as a program of its own, for what a page can't do | |

**Still the owner's to say.**

- Whether "a project changes project files" is read rightly above.
- Live variables by plain writes first (taken so, as the easiest), or the function first.
- The go-ahead, later, to make the "GridCraft Doom" repository on GitHub.

### Kits as two (phase K1)

**Built 2026-10-13, on the owner's word ("Start K1").**

**1. Two kits.** `kit::loaded()` lists the **Anie Development Kit** and the **C Development
Kit**, both compiled in. A kit now says more of itself than its name: what one of its
projects is, what it can't do yet, what its projects may ask to be allowed, and whether they
may have a panel or scripts.

- **`kit = "…"` in `project.toml`** says which kit a project is of. Without the line it is
  the Anie kit's, so every project there was stays as it is. The manifest only checks that
  the word is a kit's id; whether this build has that kit is the kit's to say
  (`kit::check`, run when a project is read).
- **A project is one its kit can have, or it can't be used.** A C project that asks for
  `python` or `network`, or has a `[panel]` or `[python]` section, is listed with what is
  wrong and can't be trusted: nothing of the first kit can be made to run for a project
  that says it is of another. So is a project of a kit this build doesn't have (it is
  listed under the first kit, as a problem).
- **The C kit is listed, and no more.** It has a starter of its own (a `project.toml` that
  asks for nothing, and `src/main.c`, into which only the project's id is written), its
  projects can be made, read and trusted, and its part of the window says in so many words
  that GridCraft can't build or run one yet. Both windows draw a kit's part from what the
  kit says: the Python line and the showcase are the first kit's and are left out of the
  second's.
- **Agents** are told of both kits (`development_status`), each project's kit
  (`list_projects`), and what a kit is (`reference`, topic `project`). `create_project`
  still makes the first kit's starter only: an agent can't make a C project in one call.

**2. What git keeps in a project is not part of it.** A file or folder named `.git`, at any
depth and whatever its capitals, is left out of a project's files and of its fingerprint: it
is not looked into at all (so however much git keeps, the project's limits are about the
project). Without this a project kept in git would stop being the one that was trusted with
almost every git command.

- **It holds only with its other half**: nothing in `.git` is ever served to a panel, run,
  read out or written through GridCraft. `manifest::inner_path`, which every path into a
  project goes through, refuses it: by its name, by the short name Windows gives it
  (`GIT~1`), and by spellings an older Mac's file system takes for it. A manifest can't
  point into it, an agent can't read it (git's settings can hold a token), write it (a hook
  written there would run the next time the person used git, and nothing would show it had
  changed) or delete it, and a plan can't hold such a step.
- **Git's ordinary files** (`.gitignore` and the like) are the project's, counted and
  fingerprinted like any other.
- **What it does not cover**: a program the person runs themselves in the folder. GridCraft
  has no git of its own yet; when it has, it must never run what a repository names.

**Checked.** The gate's eight steps, 1577 tests, none failing. New tests for the kits, the
manifest's line, a project against its kit, what git keeps (in a project, from an agent,
from a panel), the C starter, and both windows. In the app (an isolated copy): the
Development window lists the two kits, the second with what it can't do yet.

### A panel may run WebAssembly; Doom in a panel; C as WebAssembly first

**2026-10-13. Asked for by the owner:** "can we make the screen dockable inside a gridcraft
sheet and extract some of the variables to auto update in the sheet?" **And decided by them
the same day, when it had been tried: "switch to WebAssembly first".**

**1. A panel may run WebAssembly.** The rules a panel's page is served under (when the
project was not granted `network`) gain `'wasm-unsafe-eval'`. A browser otherwise refuses
WebAssembly under such rules. It gives a panel nothing it didn't have: its own scripts
already run, WebAssembly reaches the world only through them, what it loads still comes
from the bridge alone, and text is still not run as script. It is what C needs to run in a
panel.

**2. Doom in a panel, with its variables in a sheet (tried).** The shortest way to Doom
docked was not the one decided as easiest before docking was asked for: Doom built as
**WebAssembly**, run in a project's panel, which GridCraft already docks beside the sheet.

- **The project** ("GridCraft Doom", outside this repository; nothing of it is here): one
  file of C that gives `doomgeneric` its five functions and gives the page a step of the
  game, the picture, the keys and the variables; a page that runs it; and the data. Built
  by hand with zig (`-target wasm32-wasi`): the same 80 files and that one, 394 KB. The one
  thing the browser lacked was `system()`, which the file gives as a function that does
  nothing.
- **It is held to what it was granted.** The game is WebAssembly: it can reach nothing but
  what its page hands it (a clock, a place to print, and files kept in memory: the data,
  and what it saves until the page is closed). The page reads the variables out of it and
  makes the calls, each checked against `read` and `write`, which is all the project asks.
- **The sheet**: a table of 23 variables written four times a second; cells worked out
  from them (a bar of health, kills as a share, "IN SIGHT: 2", "LOW" for ammunition); the
  last minute second by second; and two charts over it. Made by the page in one step of
  undo when the person links it. "Nearby" is within 512 units now: 1 to 12 through the
  demo.
- **The data, at the owner's wish**: the shareware episode's (`doom1.wad`, version 1.9),
  fetched from Debian's archive and checked against the known checksums. "This is more
  nostalgic." It is id Software's, given out free under its own terms, which are not an
  open licence: it is in no repository of ours, and a person fetches it for themselves.
  (An earlier line here said it may not be passed on: that was more than is known.)
- **What was seen.** In Node, on a clock stepped by hand: it starts, plays the demo, and
  gives the same values second for second as the program built for Windows did. In a
  browser, served under the panel's own rules with a stand-in for `spreadsheet.js`: it
  loads, draws and plays, and makes the calls. In GridCraft itself (an isolated copy): the
  project is read, trusted there and offered for docking, and the very commands the page
  makes give the sheet, the worked-out cells and the charts.
- **What was not seen: the panel docked in GridCraft, playing and writing.** Docking is the
  person's alone: a click posted to the window did not take, and the control channel is
  refused by design. The project was put in the owner's own projects folder at their word,
  untrusted, for them to trust and dock. Known before it is seen: the screen stands still
  for a second while the picture melts between levels; there is no sound; a page that is
  not being shown runs slowly.

**3. C as WebAssembly first: decided.** The C kit builds a project's C to WebAssembly and
runs it in the project's panel, in place of a program of its own first.

- **Why it is the better first**: it is what Doom docked needs; a WebAssembly program is
  held to what its project was granted, where a program of its own can't be limited at
  all; it needs no new thing to ask to be allowed; and it runs wherever a panel does.
- **What it changes in what was built**: a C project will have a panel, which K1 refuses
  for now (`kit::C`), and the kit's words in the window say WebAssembly and the panel.
- **What the kit will bring** (phases K2 and K3, below in the table): the build, by the
  compiler the person names (zig first: it has the C library a WebAssembly program needs);
  and, served beside a C project's page as `spreadsheet.js` is, the little of a system the
  program asks for: a clock, a place to print, files in memory. The trial's page has that
  in its own script: it is what the kit takes over.
- **Where what is built is kept** was left for K2, and is settled there (below): beside
  the settings, under the fingerprint of the files it was built from. The trial's
  `doom.wasm` is a file of the project, in its fingerprint, which a build by the kit must
  not be: a build would then end the trust it needed.
- **A program of its own is put aside, not ruled out**: for what a page can't do (sound
  devices, long work without a window), as a thing a project asks to be allowed, worded as
  `python` is.

**Checked.** The panel's rules: the crate's tests (a panel is served with the word and
without `'unsafe-eval'`), and the gate's eight steps on the whole. The trial: as above.

### The C kit builds (phase K2, first slice)

**Built 2026-10-13, on the owner's word ("Start K2").**

**1. The recipe is a list, never a command.** A C project's `project.toml` has `[c]`:
`sources` (C files, or folders: every `.c` file directly in a folder), and optionally
`include` (folders), `define` (a C name, or a name and a plain value), `optimize` (`size`,
`speed`, `none`) and `entry` (`main`, or `none` when its page is to call what it exports).
GridCraft writes the compiler's command line itself: nothing a project says becomes an
option of its own choosing, and what a recipe names has to be in the project, or the project
can't be used. A recipe is the C kit's: a project of the first kit that has one is refused.

**2. The compiler is the person's, found or named**, like the Python: zig, used as `zig cc`,
which carries the C library a WebAssembly program needs. It is named in the C kit's part of
the Development window, **by hand only**: it is a program GridCraft will run, so the control
channel can't name it (as it can't name the Python). GridCraft ships none.

**3. A build is for a project the person trusts as it is**, like running a script: it runs
their compiler over the project's files. `c::build` decides that for itself, reading the
project and the person's decisions fresh from disk. Asking for one is the person's alone
(the Build button, in the project's part of either window); the control channel is refused.
It runs beside the window, which says how long it has taken and how it ended. **An agent
can't ask for a build yet** (the second slice: a request, with what the compiler says
fenced).

**4. A project is built from its own files and the compiler's, and from nothing else.**
Each source is compiled by itself, and the compiler says every file it read for it
(`#include`, `#embed`). One that is neither the project's nor the compiler's own fails the
build, and nothing is kept. Inline assembly is turned off: the assembler can take a file
in (`.incbin`) without the compiler saying so. What git keeps in the project is not the
project's here either. So a program can hold nothing of the computer but what the person
reviewed when they trusted the project: without this, a project "held to what it was
granted" when it runs could have carried the person's files inside it from the day it was
built. **It rests on the compiler telling the truth about what it read**: a guard, tested
against the four ways tried, not a proof.

**5. What is built is kept outside the project**, beside the settings, under the
fingerprint of the files it was built from: `built/<id>/<fingerprint>/program.wasm`. A
build writes nothing into the project (zig's own cache is beside what is built too), so it
never ends the trust it needed. What was built from other files than the project holds now
is shown as such ("build it again") and is not the project's; it is removed at the next
build. The project is read again when the build ends: files changed by hand meanwhile give
no program. This settles the question left open: K3 serves the program kept under the
fingerprint of the project as it is trusted then, and no other.

**6. Tried: Doom, built by the kit.** `doomgeneric`'s 80 files of Doom and the one file of
glue, as a C project with a five-line recipe (`sources = ["doom", "src"]`, an include
folder, one definition, `entry = "none"`): 81 sources, 394 KB, **8.6 seconds** on a new
cache (eight compilers at once), and the program plays the shareware demo in Node with the
same values second for second as the one built by hand. Nothing was written into the
project.

**What it found on the way.**

- **A compiler can be damaged, and says so badly.** The first copy of zig in the scratch
  folder stopped building: its 130 empty files (headers that are empty on purpose) had gone,
  all at 14:11:54 on 2026-10-13, six minutes before anything of this work touched it.
  What removed them is not known; a second copy, unpacked beside it, kept its own. The
  kit shows the compiler's own words when a build fails, which is how it was found.
- **Dependency flags on one command for many sources name only one of them**, which is why
  each source is compiled by itself.

**Not built.** An agent asking for a build. Running what was built (K3). A compiler other
than zig. A build can't be stopped once started (each step has ten minutes). Nothing clears
zig's cache (52 MB after Doom).

**Checked.** Tests: the recipe (and what it refuses), the sources of a folder, the
compiler's list of files read, the store, who may build, and both windows; with a zig on the
machine (`GRIDCRAFT_TEST_ZIG`, or one that is found), real builds: a starter, a second build
after a change, the four ways in from outside, a mistake in the C. Without one those check
nothing, and say nothing: the gate has no compiler. The gate's eight steps pass, 1590 tests
with a zig named. In the app (an isolated copy, the projects trusted there by its own
file): Build, pressed in the page on Doom laid out as a C project, gave "Built:
program.wasm, 385 KB, from 81 C files, in 7.7 s, by zig 0.17.0", and the program was
kept under the project's fingerprint.

### The C kit runs what it built (phase K3)

**Built 2026-10-13, as the next task the owner named.**

**1. A C project has a panel, and the panel is where its program runs.** `[panel]` in a C
project's manifest is no longer a reason it can't be used (`kit::C.panels`). The kit has
nothing left that it tells the person it can't do: what is left, an agent asking for a build,
is not the person's concern, and an agent is told of it in `reference`.

**2. GridCraft serves two things beside a C project's page that are not files of the
project**, as `spreadsheet.js` is not, and under every check the page itself is served under
(the token, the bridge's own address, and the person's trust read from disk for each
request):

- **`program.wasm`**: the program kept under the fingerprint of the files the project holds
  at that request, which is as the person trusts them, and no other
  (`c::program_bytes`). When nothing was built from those files the answer is 404, which the
  loader turns into words that say to build. What was built from other files stays where it
  is and is never handed out: a project that changed after it was built has no program
  until it is built again, and that holds for one the person lets follow its changes too.
- **`program.js`**: the loader (`c_program::LOADER`).

Only a project whose kit builds and that has a recipe is served them. A project of the first
kit keeps its own files of those names. For a C project the two names are the kit's: a file
of its panel by one of them is not what is served.

**3. The loader hands the program a small system, and nothing else** (`CProgram.load`,
`CProgram.start`): a clock, a place to print (a line at a time, handed to the page), random
numbers, its arguments, and files kept in memory: the ones the page hands it, which it can
read and can't write over, and the ones it writes itself, which are gone with the page.
Anything else the program was built to ask for is answered "there is no such thing". It has
no way to the spreadsheet, to the network or to the computer's files. What the program is to
do with the workbook, its page does for it through `spreadsheet`, each call checked against
what the project was granted at that moment. That is what "held to what it was granted" rests
on: the program is WebAssembly, its page is served under the panel's rules, and the bridge is
the one door out of the page.

- **It is JavaScript, as text in Rust source**: the fork's fourth exception to upstream's
  "Rust only", after `spreadsheet.js`, the showcase and the Development window's page. A
  page is what runs a WebAssembly program, and a page is written in JavaScript. The
  repository still holds no JavaScript file.
- **It was written and tried before it was taken in**, in the session before this one and
  outside the repository: in Node on three programs the kit had built (one that prints, one
  that uses files and its arguments and ends with 3, and Doom), and Doom in a browser under
  the panel's rules. `c_program.rs` is that text, with a test of what it holds and of what
  it never names, and with one change made here: when the program is refused, the loader
  now says GridCraft's own sentence of why ("… has changed since you trusted it …") in
  place of "could not be loaded (403)". The three programs were not run again after that
  change; the starter was (below).

**4. The C starter has a panel.** `panel/index.html` loads the two scripts, runs the program
and shows what it printed, or why there is no program. The project still asks for nothing.
Only its id is written into the page's script; its name is in the title and the heading, as
text.

**5. A project's part of the Development window says of its panel, and shows it**: **Dock
Panel** and **Open in Browser**, for a project of either kit that the person trusts as it
is, each where this copy of GridCraft has the means (a web view; a browser to hand the
address to). By hand only, as in Projects: refused while the control channel is putting
input into the window, and both ways decide for themselves whether the project is trusted
(`dock::dock`, `panels::open`). For a C project the part also says whether there is a
program for the files it holds now.

- **In both windows**: GridCraft's own, and the page (`showPanel`), which was built again
  for it on the owner's word (`npm install --ignore-scripts`, with the three versions
  `page.rs` names, and `npm run build`).

**6. An agent is told** what a C project holds now (C sources and a panel), that its panel
runs the program, that a program belongs to the files it was built from, and how a page
uses `CProgram` (`reference`, topics `project` and `panel`). The kit no longer reports
something it can't do. An agent still can't ask for a build, and `create_project` still
makes the first kit's starter only.

**Checked.** Tests, new and changed: the loader's text; the kit and a project against it; the
store's `program_bytes`; the bridge over real connections (the loader and the program are
served only so, a program built from other files is not served, a first-kit project keeps its
own files of those names), with a few bytes standing in for a program; the C starter and a
hostile name against its page; what an agent is told; both windows' words, the panel's part
by trust and by the means at hand, and Dock Panel and Open in Browser by hand and refused to
the control channel, with the docked address then serving the starter's page, the loader and
a program kept under the project's fingerprint. The gate's eight steps pass, 1595 tests, with
a zig named (`GRIDCRAFT_TEST_ZIG`), so the build tests build: one of them takes five seconds
with it, and no time without.

**Seen, in GridCraft itself** (an isolated copy with settings of its own; the starter made
and trusted there by that folder's own file, standing in for the person; zig named there).
zig was put on the machine for it, where the owner said (`C:\util\Tools\zig-0.17.0`, unpacked
from the archive fetched earlier that day, its checksum checked again; not on the path).

- **The page draws the panel's part**, and **Dock Panel**, pressed in the page, docked the
  starter's panel beside the sheet. Nothing was built yet, and the panel said so in the
  loader's words: "This project has not been built from the files it holds now. Build it in
  GridCraft …".
- **Build**, pressed in the page: "Built: program.wasm, 3 KB, from 1 C file, in 6.4 s, by
  zig 0.17.0", and the part then said the panel runs the program built from the files it
  holds, and that it is docked.
- **Run It Again**, pressed in the docked panel: **"Hello from blink."**, and the Developer
  console had the page's line "blink ran and ended with 0". **A C program, built by the
  kit, running in a panel docked in GridCraft: seen for the first time.**
- **A source changed** (by hand, in the project's folder): the panel was given no program
  (403), though the earlier one was still kept. **Trusted again and not built again**: "has
  not been built from the files it holds now". **Built again in the page**: the changed
  program's line, and only the new program was kept.
- **Docked as GridCraft starts** (`GRIDCRAFT_DOCK`): the program ran. After another change
  the panel said "Blink has changed since you trusted it. Review it again in Developer >
  Projects."
- In Node, the starter as the kit built it, with the loader as it is now: its line, and 0.

**Not seen.** Open in Browser was not pressed in the app (the tests press it, with a
stand-in for the browser). GridCraft's own window (the one that is not the page) was not
looked at: clicks posted to it do not land, and its buttons are pressed only in the tests.
Doom was not built or run here: that is D1, and running it is for the owner's word. A
program that reads files or its arguments was not run in the web view, only the starter,
which prints.

**Open.**

- **A large program.** The loader makes the program from its bytes in one step on the page's
  own thread, which browsers limit by size (to a few megabytes in Chromium, as far as is
  known: not tried), while the kit keeps programs of up to 64 MB. Doom is 394 KB.
- **A program that never returns holds its page**: `run()` is `main` to its end. Doom is
  built without a `main` and stepped by its page.
- The program is read whole from disk for each request, and a browser keeps no copy.
- A C project can't ask for `network`, so its page loads and calls nothing but the bridge.
  Whether one should be able to is not decided.

### Doom, built by the kit and docked (phase D1)

**2026-10-13, on the owner's word:** "Doom built by the kit and docked - go ahead", and "we
want to also expose some elements from the game memory into the spreadsheet so we can show
it is connected."

**Nothing of it is in this repository.** The project is `G:\gridcraft-doom` (0.3.0 now): a
C-kit project with `doomgeneric`'s Doom, one file of glue, a panel, and the shareware data.
It was run in copies of GridCraft with settings of their own, the project trusted there by
that folder's own file; the pictures of it are kept outside the repository too.

**1. Seen: Doom, built by the kit, running in a panel docked in GridCraft.** 81 C files, 386
KB, under seven seconds. Docked as GridCraft started, it loaded the data (4 MB, handed to
the program as a file kept in memory), started, and played its recorded demos by itself;
the Developer console had its page's line "DOOM Shareware started." It is the program K3
serves: built from the project's files as they were trusted, and run by GridCraft's loader.

**2. The sheet.** "Link to a sheet named Doom", pressed in the docked panel, made the sheet
and started the writing: the 23 variables, the cells worked out from them, the last minute
second by second, and two charts over it, as the page was written to do before the kit could
run it.

**3. The game's memory in the cells** (the owner's addition). The table of variables is the
game's own code reading its variables and writing them out. Beside that there is now a
second road, which shows the linkage itself:

- **The game says only where.** One function more in the glue gives, for twelve places (the
  game's clock and the level's, health, armour, two kinds of ammunition, kills, the red
  flash of being hurt, where the player stands, how fast, which way), a name, the address,
  and how the four bytes there are read. It hands over no value.
- **The page reads the bytes itself**, out of the program's memory (`program.bytes`), and
  writes them as they lie: `0x0102D848`, `56 00 00 00`.
- **The sheet works the number out**, with a formula of its own (`MID` to turn the bytes
  round, `HEX2DEC`, and the sign): 86. The same 86 was in the table of variables and on the
  game's status bar. Doom's fractions (65536 to the unit) and its angles are worked out so
  too.
- The player's body is a thing the game makes anew: its address was another in another run
  (`0x0149CDB4`, then `0x014AC7CC`), and the cells followed.

**What it found.**

- **A minute of Doom empties the undo list.** Every write is a step of undo, four a second,
  and the list keeps a hundred: 25 seconds after linking, everything the person did before is
  gone from it. Plain writes show the linkage and are not how it can stay: this is the case
  for live values that are not edits (L1).
- A window's title changes when its workbook is edited, which a script that finds a window
  by its title has to allow for.

**Not seen.** Nobody played: no key was sent to the game, so the keys are untried in the
docked panel. There is no sound. It was not run in the owner's own GridCraft, whose projects
folder holds the earlier form of it (a project of the first kit): putting this one there, and
trusting, building and docking it, are the owner's. In the first run the sheet was linked by
a click that was not this session's (the window is on the owner's screen); in a second run
nothing was written for 40 seconds, until the click posted here.

**Next for it.** The other direction (a cell that changes the game) needs `read` and the
event that says the workbook changed, and the loader lets a page write into the program's
memory as it reads it. The "GridCraft Doom" repository, on the owner's word; Freedoom's data
is what it could carry.

### A project that travels in a workbook, builds for a platform, and a viewer for its files (planned)

**What the owner asked (2026-10-13).** "For developers we want them to be able to compile a
module for a platform, example: G:\gridcraft-doom\ can be compiled for Windows x86-64, the
binary, files and meta data stored inside the spreadsheet (we may need to make a new
extension: xlscraft for example) ... the embedded program will only run when it detects the
gridcraft is running on the supported platform. This operates like snap to some degree, but
it can work on Windows, MacOS, Linux. The developer can also choose to leave the module as
code, and it should contain all the dependencies list required to build it. Of course the
developer could compile for platforms and also leave the source code available (for a
platform that the developer hasn't built for). We should provide a file viewer with syntax
highlighting provided by the development kit. The development kit could opt to provide their
own file editor."

**Nothing of this is built, and nothing here is decided but what the owner said.** What
follows is what it changes, what must hold, and a shape to say yes or no to.

**1. What it changes in what was decided.**

- **A workbook carries a project.** That was left open ("Open: modules and workbooks",
  above). It is the second of the directions listed there, with more in the package: the
  project's files, what describes it, and what was built of it.
- **A file format of GridCraft's own.** On the same day the macros' phases were taken "with
  no file format of GridCraft's own". A new extension reopens that.
- **Programs of their own.** K3 runs C as WebAssembly, and "a program of its own is put
  aside, not ruled out". A build for Windows x86-64 is a program of its own. K3 stands; this
  brings the other back onto the plan.

**2. What has to hold, whatever the design.**

- **Opening a workbook runs nothing.** Not when the platform is the right one either. A
  project a workbook carries is found, listed and inert, like one that arrives any other way.
- **Trust is the person's, and is given to exact contents.** A carried project is trusted
  as any other is: by its fingerprint, the one that was on screen.
- **What was built elsewhere can't be checked against its source.** K2's guard ("built
  from its own files and the compiler's") holds for what the person's own compiler builds
  on their machine. A build that arrives in a workbook was made by whoever made the
  workbook. So it is trusted as itself, its bytes in the fingerprint, and the window has to
  say so: that nobody here can tell it is what the source beside it would build.
- **What a build can do depends on what it is.** A WebAssembly build is held to what the
  project was granted. A build for a platform can't be limited at all, as a Python script
  can't. So it is a thing a project asks to be allowed, worded as `python` is, and shown as
  the one that cannot be limited.

**3. A shape.**

- **Targets.** A recipe says what it is built for: `wasm`, which is today's (one build that
  runs wherever GridCraft does and is held to its grants: the "works on Windows, macOS and
  Linux" already there), and platforms (`windows-x86_64`, `macos-aarch64`, `linux-x86_64`
  and so on). zig builds for every one of them from any one machine, so a developer on
  Windows can build all of them. (Its build of `doomgeneric` for Windows was tried on
  2026-10-13; builds for another system than the one it runs on were not.)
- **What runs.** For a project the person trusts, GridCraft takes the build for the platform
  it is running on; without one, the `wasm` build; without that, and with the source and a
  compiler, it offers to build; otherwise it says there is nothing for this platform, and
  runs nothing.
- **How a program for a platform runs.** As a script does: a process of its own that talks
  to GridCraft over a pipe, each call checked against what was granted. It has no panel to
  draw in. Doom docked beside the sheet stays the WebAssembly build: a window of another
  program can't be docked the way a page can.
- **In the workbook.** A workbook is a package of parts. A project is parts of its own in
  it: its files, the builds by target, and a list of them with their hashes. **A workbook
  that carries a project says so in its name**, as Excel's `.xlsm` says a workbook carries
  macros: an extension of its own (`.xlscraft` is the owner's example), the same package
  otherwise. Saved as `.xlsx`, the project is left out, and GridCraft says so first. What
  Excel does with such a file when it opens and saves it is to be asked of Excel, as was
  done for macros (it keeps a custom XML part; binary parts were not asked about).
- **Opening one.** A bar says the workbook carries a project and where it stands, as for
  macros. Review shows it as Projects does. Trusted, it runs from the workbook's own copy,
  checked against the fingerprint each time; "take a copy into my projects" makes it a
  project to work on.
- **Left as code.** A project can travel as source alone. It then says what building it
  needs (`[requires]`: the kit and its version, the compiler and its version; for the first
  kit the Python and the packages, which is phase 2c's ground). Built on the person's
  machine by their own compiler, it is under K2's guard, and the person can read what runs:
  this is the stronger form, and the one to prefer where a compiler can be expected.
- **A viewer for a project's files.** In a project's part of the Development window, and in
  Review, where it serves the decision to trust: each file as text, with its syntax
  coloured. **The kit says how its files are read**: for each kind of file, what a comment,
  a string, a number and a keyword is. That is a description GridCraft's own drawing uses,
  never code that runs, and it is written here: no highlighter is copied in. A file is
  drawn as text and nothing in it is followed.
- **A kit's own editor.** A kit may bring one in place of the viewer. For the kits compiled
  in, that is a part of GridCraft; for a kit that is loaded, it waits on how kits are
  loaded, which is not decided. A change made in any editor ends the project's trust as any
  change does.

**4. Phases.**

| Phase | What | Waits for |
|---|---|---|
| F1 | The file viewer, with highlighting the kit describes; in both windows and in Review | **Built 2026-10-13**, in two slices (their own sections, below): in both windows, then in Review |
| T1 | Targets in the recipe; a build kept for each; zig building for other systems | K2 |
| R1 | A build for a platform runs: a process and a pipe, and a permission of its own | T1, and the owner's word on programs that can't be limited |
| W1 | A workbook carries a project as source: the parts, the extension, the bar, review and trust | The extension; Excel asked what it keeps |
| W2 | Carried builds, taken by platform; trust in what was built elsewhere | W1, T1 |
| Q1 | `[requires]`: what building a project needs | Phase 2c, for Python |
| E1 | A kit's own editor | How kits are loaded |
| S1 | A developer signs a project; GridCraft checks the signature; a person agrees to trust the developer who signed | The design, after the review of prior art (below) |
| S2 | An organisation's server that GridCraft asks whom to trust | S1 |

R1 and W2 wait for S1 now: a program for a platform runs only signed (decided, below).

**5. What the owner decided (2026-10-13)**, on the six points put to them:

1. **The extension is `.xlscraft`**, "for now".
2. **A program for a platform is signed by its developer.** "Native binaries should be
   signed by the developer (for example corporate IT) so the user will know it can be
   trusted. We'll have to build in the trust system later. Ideally there should be an
   enterprise server that gridcraft can connect to and verify. For individuals, it really
   should be to agree to trust the developer who signed it."
3. **A program for a platform need not show a picture** in GridCraft's window: "It doesn't
   have to, this is more of an option." So a process and a pipe is enough for R1.
4. **A trusted, carried project runs from the workbook's own copy.**
5. **Publishers and signatures: "Let's review prior art eg snap."** Reviewed in the next
   section.
6. **The order as recommended**: F1, the file viewer, first.

**Still the owner's to say**, after the review: the points at the end of the next section.

### Trusting the developer who signed: prior art (reviewed 2026-10-13)

**Asked for by the owner**: "Let's review prior art eg snap." What follows is what eight
systems do about the same question (a program arrives from someone else: whose word is
taken for it, and what can it then do), read from their own documentation where it could be
found, and what GridCraft could take from each. The sources are at the end. Nothing here is
built or decided.

**1. What each one does.**

| System | What is signed, by whom | Where the trust starts | What limits the program |
|---|---|---|---|
| **snap** | The store signs statements ("assertions") that chain together: an account, the account's key, a declaration of the snap (its id, name, publisher), and a revision that names the hash of the exact file. | The store's own key, known to the installer. The store vouches for who the publisher is, and marks the verified ones. | **Separately from the signature**: a snap is confined by default and reaches more only through named interfaces. One that asks to be unconfined ("classic") is reviewed by people first, its publisher vetted, and the leave can be taken back. |
| **snap, for an organisation** | The same. | An organisation runs its own store or proxy: its devices are registered with it, it decides which revisions they get, and it can run cut off from the internet. | The same. |
| **Flatpak** | Every commit of a repository, and its summary, by the repository's key. | The person adds a repository with its key, once. Trust is in a source, not in a file. | A sandbox, with portals. |
| **AppImage** | A signature can be put inside the file. It is optional, and running the file does not check it: a separate tool does. | Nowhere, unless someone checks. | Nothing. |
| **Office macros** | The VBA project, by its developer's certificate. | A list of trusted publishers: certificates in Windows' own store, which an administrator can hand out by policy. The policy can run signed macros only, and only from listed publishers, and can stop people adding publishers themselves. | Nothing: a macro that runs can do anything. Since 2022 a macro in a file that came from the internet is blocked with no button to enable it. |
| **Windows programs** | The file, by a certificate a certificate authority issued, with a timestamp. | The authorities Windows trusts. That shows who and that nothing changed; whether to warn is decided separately, by reputation. | Nothing. |
| **macOS programs** | The program, by a developer's certificate Apple issued; and Apple's own note that it scanned this exact build ("notarization"), which can be stapled to it. | Apple. The system checks at first launch, asking Apple when no note is attached. | The sandbox, for those that opt in. |
| **Android** | The package, by a key the developer made themselves. | **The first install**: whatever key signed the app then is the only key that may update it. A store can hold the key for the developer. | A sandbox and permissions. |
| **Sigstore, TUF** | A signature tied to who signed (an identity an organisation's sign-in vouches for) in place of a key kept for years, with every signature written to a public log. TUF: several keys, a number of which must agree, the most important kept offline. | An identity provider and the log's root. | Not their subject. |
| **VS Code** | The marketplace signs each extension; the editor checks at install, and asks before installing from a publisher not yet trusted. A folder of code is untrusted until the person says otherwise, and runs nothing till then. | The marketplace, and the person's own list of publishers and of folders. | Nothing for extensions; for folders, "restricted mode". |

**2. What they teach.**

- **A signature says who, and that nothing changed. It never says safe.** Every one of
  these systems that takes safety seriously has a second thing beside the signature: a
  sandbox (snap, Flatpak, Android), a scan (Apple), people who review (snap's unconfined
  ones). GridCraft has its second thing for WebAssembly, which is held to what was
  granted, and none for a program for a platform. So the owner's rule fits what the others
  found: the unconfined kind is the exception, and it is the kind that must be signed.
- **A signature nobody checks protects nothing** (AppImage). The check has to be
  GridCraft's own, made every time before a program runs, and a program for a platform
  with no signature, or a broken one, must have no button that runs it anyway. Office had
  such a button ("Enable content"), people learned to press it, and in 2022 it was taken
  away for files that came from the internet.
- **Two ways to start trust, and they suit the two cases the owner named.**
  - *For a person*: trust on first meeting, then hold the developer to the same key
    (Android; SSH does the same). "Agree to trust the developer who signed it" is this. It
    needs the developer shown honestly: the name is only what the key's owner wrote, and
    the key's own fingerprint is the one thing that can't be made up.
  - *For an organisation*: a list of whose signatures count, handed out and kept by the
    organisation, which people can't add to (Office's trusted publishers under policy; an
    organisation's own snap store). "An enterprise server that GridCraft can connect to and
    verify" is this.
- **Trusting a developer is much wider than trusting a project.** It covers everything
  they sign from then on. Office's own documentation warns of exactly that, and that its
  list is shared with the rest of Windows. GridCraft's list should be its own, say how
  wide a yes is, and let a person trust one project without trusting its developer.
- **Keys get lost and stolen.** Android: a lost key means no more updates. The newer
  systems answer with keys that live for minutes and an identity behind them (Sigstore),
  with several keys of which some must agree (TUF), and with a way to say "no longer"
  that reaches every machine (snap's store can take a leave back). An organisation's
  server is where this belongs: it can say today that a key is no longer good.
- **A badge is not a check.** "Verified" on a publisher means a store looked at something
  once (for VS Code, that they own a web address). Researchers have shown such badges
  staying on things that changed. Whatever GridCraft shows of a developer should say what
  was checked, by whom, and no more.
- **Being cut off has to work** (snap's offline store; Apple's attached note). A workbook
  is opened on aeroplanes. What the server said is kept, with a date after which it must
  be asked again.

**3. A shape for GridCraft**, to say yes or no to.

- **What is signed** is the project exactly as it is carried: the list of its files and of
  what was built of it, each with its hash, which is what the fingerprint is taken over
  already. One signature over that, kept beside it in the workbook. So a signature and the
  person's own trust speak of the same thing, and one changed byte ends both.
- **A developer** is a key. With it travels the name they give themselves. GridCraft shows
  the name as theirs to claim, the key's fingerprint as the fact, and whoever vouches for
  the pair, when someone does.
- **A person** who opens a workbook that carries a signed project is shown who signed it,
  and can do one of three things, each confirmed by the operating system as trust is today:
  nothing; trust this project (as now, by its fingerprint); or trust this developer, which
  is said to cover whatever that key signs until it is taken back. GridCraft keeps its own
  list of developers a person trusts, beside the list of projects.
- **An organisation** sets GridCraft up, by a setting a person can't change, with its
  server's address and its key. The server says whose keys are the organisation's
  developers', and which no longer are; what it says is signed with the organisation's
  key, kept with its date, and good offline until that date passes. The organisation can
  also say that nothing else runs.
- **A program for a platform runs only if** its project is signed, the signature is good,
  and the signer is trusted in one of those two ways. Without that it has no button. The
  one exception is the person's own: a program their own compiler built on their machine
  from a project they trust, as K2 builds.
- **A WebAssembly build** needs no signature to be trusted as a project is today: it is
  held to what was granted. A signature on it still says who made it.
- **What a yes never does**: limit the program. The words at the moment of trust have to
  say that a program for a platform can do whatever the person can.

**4. What the owner decided (2026-10-13)**, on the four points put to them after the review:

1. **Keys first.** A developer signs with a key of their own. Certificates that an
   authority issues are for later, if an organisation asks.
2. **Where an organisation has set GridCraft up, a person does not trust a developer
   themselves**: "No - this would be at the organization level. This would help control
   EUC." (EUC: end-user computing, what people in an organisation build for themselves
   outside its IT.) Only the organisation's list counts there.
3. **An unsigned program for a platform has no way to run** on a machine other than the one
   that built it. "None."
4. **A developer's name is never shown as checked** without a server behind it. "No."

The design these make is in "Signing, and whose word is taken for a developer", below.

**Sources** (read 2026-10-13).

- snap: Canonical, [Where eagles snap: snap security overview](https://canonical.com/blog/where-eagles-snap-snap-security-overview); Ubuntu Core's reference on the [account-key](https://documentation.ubuntu.com/core/reference/assertions/account-key/) and [snap-declaration](https://documentation.ubuntu.com/core/reference/assertions/snap-declaration/) assertions; [Reviewing classic confinement snaps](https://snapcraft.io/docs/reference/administration/reviewing-classic-confinement-snaps/); the [Enterprise Store documentation](https://documentation.ubuntu.com/enterprise-store). The order in which the installer walks the chain is put together here from those pages: none of them sets it out in one place.
- Flatpak: the [flatpak build-sign](https://man7.org/linux/man-pages/man1/flatpak-build-sign.1.html) manual. AppImage: [Signing AppImages](https://docs.appimage.org/packaging-guide/optional/signatures.html).
- Office: Microsoft, [Macros from the internet are blocked by default in Office](https://learn.microsoft.com/en-us/microsoft-365-apps/security/internet-macros-blocked) and [Trusted publishers for Office files](https://learn.microsoft.com/deployoffice/security/trusted-publisher); the Australian Signals Directorate's [Microsoft Office macro hardening](https://blueprint.asd.gov.au/design/endpoints/windows/security/microsoft-office-macro-hardening/).
- Windows: Microsoft, [Code signing options](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/code-signing-options). macOS: Apple, [Apple Platform Security](https://support.apple.com/guide/security/sec3ad8e6e53); how a build is refused after the fact was not found in what was read.
- Android: [Sign your app](https://developer.android.com/studio/publish/app-signing).
- Sigstore: [signing overview](https://docs.sigstore.dev/cosign/signing/overview/). TUF: [the specification](https://theupdateframework.io/specification).
- VS Code: [Extension runtime security](https://code.visualstudio.com/docs/configure/extensions/extension-runtime-security); OX Security, [Can you trust that verified symbol?](https://www.ox.security/blog/can-you-trust-that-verified-symbol-exploiting-ide-extensions-is-easier-than-it-should-be/)
- Said from what is generally known, not from these pages: that Flatpak, macOS and Android
  confine programs, and that SSH holds a server to the key it first showed.

### A viewer for a project's files (phase F1, first slice)

**Built 2026-10-13, first in the order the owner confirmed. Not committed until the owner
says.**

**1. A kit describes the languages its projects are written in** (`Kit.languages`,
`highlight::Language`). For each: the endings of the files it is for, what begins a comment,
how quoted text begins and ends, the words the language keeps for itself, whether it has
numbers, and the shape of its lines (a program; tags; keys and sections; prose with
headings). **A description is data.** Nothing a kit or a project brings is run to show a
file. The first kit describes Python, HTML (Svelte with it), JavaScript, CSS, TOML, JSON and
Markdown. The C kit describes C and the same others, for the page that runs the program,
and not Python: a `.py` in a C project is shown plain.

**2. GridCraft does the colouring itself** (`highlight::spans`, `highlight::show`): one pass
over the text that finds comments, quoted text, numbers, the language's own words, and what
the shape gives weight to (a tag, a key, a section, a heading). It is a plain reading, not a
parser: it can be wrong about what a line means and still shows the text as it is. It was
written here, and no highlighter or grammar is copied in. It reads any text at all, once
through: tested with quotes and comments that never close, other scripts, control
characters, and long runs of openings.

**3. A project's part of the Development window offers its files and shows the one that is
chosen**, in both windows: each line with its number, each piece in the colour of what it
is.

- **Only the project's own files**: the ones in its list, which is what was fingerprinted.
  Nothing of what git keeps there, and nothing outside it, by any spelling. Only text, and
  only so much: 256 KB, 2,000 lines, 1,000 characters to a line, and it says when it left
  something out. What is shown is the file as it is on disk now.
- **It needs no trust.** A project that is not trusted, or has changed, is read the same:
  reading before trusting is what the viewer is for.
- **Choosing a file to read is the person's.** It is refused while the control channel is
  putting input into the window. The channel is kept out of the projects folder already,
  and a file put on screen could be read off a picture of the window.
- **A file is drawn as text.** GridCraft's own window lays it out as text. The page is told
  the lines in pieces, each with a word for what it is, and draws each piece as text in a
  class from a fixed set. Nothing in a file is followed, loaded or run.
- Going to another part of the window closes the file.

**Not built.** The viewer in Review (Projects), where trust is decided: the second slice. A
kit's own editor (E1). Finding a word in a file. More than five kinds of piece: a page's
own script and styles are read as markup, and C's preprocessor lines as C.

**Checked.** Tests: each language against a sample; any text read without loss (what is
shown is the text again, piece by piece); the limits; a kit's languages and that an ending
is read as one language; only a project's own files read out, and only so much; the
window's file list and the file as it is told, changed files, files that are no text or too
long; reading a file by hand and refused to the control channel, in GridCraft's own window;
the page's two messages, and what is not one. The gate's eight steps pass, 1604 tests, with
a zig named. **In the app** (an isolated copy): the page drew the C starter's source with
its numbers and colours, for a project that was not trusted at the time.

**Not seen.** GridCraft's own window drawing a file: its test finds the file's text there,
not its colours. A first-kit project's files in the app. The light theme.

### The viewer in Review (phase F1, second slice)

**Built 2026-10-13, on the owner's word ("Then on to the viewer"). Not committed until the
owner says.**

**1. A project that is being reviewed offers its files to read**, in Developer ▸ Projects,
under what it asks to be allowed and above the choice to trust it: the same viewer, with the
highlighting the project's kit describes. This is where it matters most: the person reads
what they are about to trust.

**2. Reading must not move what is being reviewed.** Trust is given to the fingerprint the
person was shown when they pressed Review, and a project whose files changed since is
refused. Every action in the Projects window reads the list of projects again, and the first
form of this slice let reading a file do so too: a project that changed during the review
then had its new fingerprint put in place of the reviewed one by a click on a file, and
Trust took it. **A test written for the slice found it before it was kept.** Reading a file
and closing it now leave the list as it was.

- **A file is shown as it is on disk now.** When the project is no longer the one whose
  fingerprint is under review, the viewer says so in words above the file ("This project
  changed while you were looking at it …"), and trusting is still refused.
- **Choosing a file is the person's**, as in the Development window: refused to the
  control channel.
- With a file open, the list of projects is given more room, so that the choice under the
  file stays in reach.

**Checked.** One test, in the window: nothing is offered until a project is reviewed; a
file is refused to the control channel and read by hand; another takes its place; closing
leaves the review as it was, and nothing was trusted or asked; then the project's file is
changed on disk while it is read: the file is shown as it is now with the warning, and
Trust is refused with "changed while you were looking at it". The gate's eight steps pass,
1605 tests.

**Not seen.** The Projects window is GridCraft's own, where posted clicks do not land: this
slice was not looked at in the app at all. Its test finds the words, not how it looks.

### Signing, and whose word is taken for a developer (phases S1 and S2, designed)

**Designed 2026-10-13 from the owner's decisions** (the six on the plan and the four after
the review of prior art). **Nothing of it is built**: "We'll have to build in the trust
system later." It is set down so that what is built before it (targets, a workbook that
carries a project) leaves the right room, and so that the owner can say no to any of it.

**1. The two ways GridCraft can stand.**

- **A person's own GridCraft.** They trust a project by its fingerprint, as today. They may
  also agree to trust a developer: then what that developer signs is trusted.
- **An organisation's GridCraft**: set up by the organisation, by a setting the person at
  the screen can't change. There **only the organisation says whose signatures count**. A
  person there trusts no developer themselves (decided). This is what lets an organisation
  govern what its people build and pass round: the control of end-user computing the owner
  named.

**2. A developer is a key** (decided: keys first).

- An Ed25519 key pair, made in GridCraft or brought to it. The private half stays with the
  developer: beside their settings, readable by their account alone, never in a project, a
  workbook, a request or a picture, like the Development server's token.
- **Who a key is** is its fingerprint: a hash of the public half, shown in short groups.
  That is the fact. The name that travels with it is what its owner wrote.
- **A name is shown as claimed**, in those words, **unless an organisation's server vouches
  for the pair** (decided). There is no badge without a server behind it.

**3. What is signed is the project, exactly as it is.**

- A **seal**: the project's id; its fingerprint, which already covers every file; for a
  project that carries builds, each build's target and hash; the signer's public key and
  the name they give; when they say they signed. The signature is over those, written out
  in one fixed way.
- So a seal and a person's own trust speak of the same thing. One changed byte ends both.
- **Where a seal is kept.** In a workbook, in a part of its own beside the project's. In a
  project's folder, in one file that is left out of the fingerprint, as what git keeps is,
  and for the same reason (it could not sign itself). Unlike what git keeps, it is read:
  but only to be checked against the key, and a seal that does not check is as good as
  none. Nothing in it is ever shown as true before it has checked.

**4. When a seal counts.**

| | A person's own GridCraft | An organisation's GridCraft |
|---|---|---|
| A project, WebAssembly or scripts, no seal | Trusted by its fingerprint, as today: the person's to decide (decided) | As the organisation says. Where it has not said: not at all |
| A project with a good seal from a developer on the list | Trusted without being asked again | Trusted |
| A good seal from a developer not on the list | The person is shown who signed, and may trust the project, or the developer | Not trusted. There is no way for the person to add the developer |
| A program for a platform, no seal or a bad one | **Never runs** (decided), but for one the person's own compiler built on this machine from a project they trust | The same |
| A program for a platform, good seal, developer on the list | Runs, having said in so many words that it can do whatever the person can | The same |

- **Agreeing to trust a developer** is confirmed by the operating system, as trusting a
  project is, and says how wide it is: everything that key signs, from now on, until it is
  taken back. The person's list of developers is GridCraft's own, beside their list of
  projects. No command gives it, and the control channel can't.
- **A seal never limits a program.** It says who, and that nothing changed.
- **Who is in control** (decided 2026-10-13, when the design was put to the owner): "If
  GridCraft is individual, then yes - the user can control it." In a person's own GridCraft
  the person decides all of it: a project with no seal is theirs to trust by its
  fingerprint, as today, and so is a developer. In an organisation's GridCraft the
  organisation decides all of it, that too. (The owner spoke of the person's own GridCraft.
  That an organisation's is the organisation's to decide in the same way, and that it
  trusts nothing unsealed until the organisation says it may, is read from that and from
  "this would be at the organization level": it is for the owner to say if it is wrong.)

**5. An organisation's server** (S2).

- **The setting**: the server's address and the organisation's own public key, in a place
  only an administrator writes (to be chosen for each system: where machine-wide policy
  lives). With it set, GridCraft is the organisation's: the person's own list of developers
  is not consulted, and can't be added to.
- **What the server says**: the developers whose keys are the organisation's, each with the
  name the organisation gives them (which is then shown as vouched for, by the
  organisation); the keys that are no longer good; whether a project with no seal may be
  trusted at all; and until when this is good. All of it signed with the organisation's
  key. GridCraft takes nothing from the server that does not check against that key, so
  the server need not be trusted for more than being reachable.
- **Cut off**, GridCraft goes by what it was last told, until that passes its date. After
  it, what needed the list does not run, and says why.
- **A key taken back** stops counting when GridCraft next hears of it: for what it signed
  before as well.
- It asks the server for the list. It does not tell the server which workbooks a person
  opens.

**6. What it needs from what is built first.**

- **W1** (a workbook carries a project): a part for the seal, and the parts list such that
  the fingerprint is taken the same way in a workbook as in a folder.
- **T1** (targets): each build's hash, by target, in a list the seal can cover.
- **K2's store**: a program built on this machine is known as such by where it is kept, not
  by anything the project says.

**7. In slices.**

| Slice | What | Waits for |
|---|---|---|
| S1a | Keys and seals: make a key, sign a project, check a seal; a project's part says who sealed it, the name as claimed. Nothing yet depends on a seal | A signature library chosen, and its licence checked |
| S1b | A person's list of developers: agreeing to trust one, taking it back, and a sealed project trusted by it | S1a |
| S2a | An organisation's setting and a list read from a file: the rules of an organisation's GridCraft, without a server | S1b |
| S2b | The server: asking it, checking what it says, keeping it till its date, keys taken back | S2a |

**8. Not decided, and not asked yet.**

- Where a developer's private key lives when the system has a place for secrets, and
  whether a key can be kept on a device.
- A lost key: there is no way to carry trust from an old key to a new one but the
  organisation's list, or each person's yes again.
- Counter-signing: more than one seal on a project (the developer's, and the
  organisation's own once it has reviewed it), as Apple adds its note to a developer's
  signature.

### The Development window's look and words (the owner's review, 2026-10-13, first part)

**What the owner asked**, with three pictures of the ribbon and the window: "let's do a
review of the UX for development - start with better icons. Development should be tools
(like a wrench) or tool box. Let's use the nice dialog - the gridcraft default dialogs are
ugly. Can we use shield icons for Development kits - don't put projects (not libraries) in
the left side). Development kits should be surrounded by a bubble to clearly distinguish
them. Inside the Development kit it should reference Projects (not Libraries). A project can
be a library if it can be referenced from other projects or spreadsheet. We interchange
Project and Library which is confusing".

**What follows from it. This changes the shape recorded above** ("Development kits and
projects").

- **Development is a toolbox.** Its button in the ribbon carried upstream's ring-and-dot,
  which is upstream's picture for settings. It carries a toolbox now, and so does the top of
  the window.
- **A kit carries a shield and stands in a frame of its own**, in the list down the side, at
  the top of the window and at the head of its own part.
- **The list down the side stops at the kits.** A kit's projects are listed in the kit's own
  part and nowhere else. The kit's row says how many it has. While a project's part is shown,
  its kit's row is marked: it is the way back. So is each place in the line above a part's
  title ("Development › Development Kits › C Development Kit"), which could not be clicked
  before.
- **One word: project.** A kit's part lists its projects as one list, under "Projects". A
  project that is one workbook's code says so beside its name ("Code of Budget.xlsm"); one
  that names no workbook says nothing there, and its own part says "For any workbook".
- **"Library" is kept for something else**: a project that other projects, or a spreadsheet,
  can refer to. None can yet. Nothing in a `project.toml` says so and nothing is built for
  it, so until there is such a thing no window and no report uses the word of a project.

**What was built.**

- **Two icons of the fork's own** (`crates/ui-egui/src/aniefork/icons.rs`), drawn in code on
  upstream's grid of sixteen, and original: a toolbox and a shield. Upstream's list of icons
  is left as it is (a variant added there would meet every icon upstream adds); the ribbon's
  Development button is drawn by the fork, laid out as upstream's large buttons are. The page
  draws the same two shapes from the same points (`Icon.svelte`): shapes in the page, not
  picture files.
- **What both windows are told** (`kits.rs`): the list is the top, the MCP server, the
  heading and one row for each kit (what it is made of, how many projects, whether its part
  or one of its projects is shown). A kit's part has `projects`, one list, where it had two
  groups; each project there says what it is for when it is a workbook's code. A part says
  where each place above it leads.
- **Both windows draw it**: GridCraft's own (`development.rs`) and the page (`Nav.svelte`,
  `Kit.svelte`, `Overview.svelte`, `App.svelte`, the stylesheet). In GridCraft's own window a
  project's name in its kit's part is a button drawn as a link: the toolkit's own link says
  it is a label to whoever asks what it is.
- **The words.** In the code the kind that names no workbook is `General`, shown as "For any
  workbook". The starter's comment in `project.toml` no longer says "library".

**What changes for agents.** A project's `kind`, in `developer.projects` and in the
Development server's `list_projects`, was `"workbook"` or
`"library"`. It is `"workbook"` or `"general"` now. The server's instructions tell an agent to
call it a project either way. Nothing was released with the older word.

**Checked.** The fork's tests for what the windows are told, for both windows' drawing and
for the page's messages, changed with it; one new test says no kit and no kind uses the word
library. In GridCraft's own window a test now reaches a project the way a person does:
through its kit's part.

**Seen** (2026-10-13, in a copy of GridCraft with settings of its own, after the merge of
upstream of that day; `target/k3-prep/seen/ux/`): the toolbox on the ribbon's Development
button; the page's list with the two kits in their frames and no project in it; the top of
the window with the kits and their shields; a kit's part with one Projects list and "Code of
Book1 · open" beside a project that names the open workbook; a project's part with its
kit's row marked and "For any workbook"; the line above the title taking the window back to
the kit; GridCraft's own window with the same list.

**Not seen.** The light theme, in either window. The icons at other display scales than this
machine's 150%. A kit with many projects.

**Badges, and the marks in them (the same day).** The owner pointed at a collection of the
badges projects carry on their front page (https://ileriayo.github.io/markdown-badges/) and
asked for the like "for the languages / UI elements"; told they were first built without
their logos, the owner said: "we want the logos".

- **What those badges are**: pictures fetched from shields.io, each a name in capitals on a
  colour, with its subject's logo from Simple Icons. The Development page takes nothing from
  the network (its policy allows its own script and stylesheet and no more), so a badge here
  is drawn by the page itself.
- **What was built**: what a kit is made of ("Python", "Svelte", "C") is a badge, in a kit's
  part and at the top of the window. GridCraft says the name and a code from a fixed few
  (`icons.rs`: `badge_for`); the page has the colour for each code in its stylesheet and the
  mark for each in `logos.js`, and draws a grey badge with the name alone for a code it does
  not know. GridCraft's own window draws the badge with the name alone: it has nothing that
  fills such a shape.
- **The marks are the fork's exception to upstream's rule on assets.** Upstream's `CLAUDE.md`
  says no other party's iconography goes in the repository, ever, and calls it the most
  important rule there. The owner was told so and asked how they wanted the logos, and chose:
  from Simple Icons, in the fork only. So three shapes of other people's marks are in the
  page's source: Python's (the Python Software Foundation's), Svelte's, and the C in its
  hexagon, each unchanged, as one path, from Simple Icons 16.35.0 (CC0 1.0).
  `ATTRIBUTION-aniefork.md` lists them, with where they came from and their hashes, and the
  page's own `THIRD-PARTY.md` says the same. **They must not be in a branch offered
  upstream.**
- **Said plainly.** CC0 covers the drawings, not the marks: those stay their owners', each
  with its own terms. They are shown here unchanged, in one colour, to name the thing they
  stand for, which is how such badges are used everywhere; whether that keeps to each
  owner's terms was not checked with a lawyer or with the owners. `cargo xtask assets` does
  not see them (it checks files, and these are text in source), so the list is kept by hand.
- **How they were taken in**: three files fetched on the owner's word, read as text; of each
  only the path was kept, after checking it holds drawing commands and numbers and nothing
  else. A test checks the same of what the repository holds, that the built page holds each
  mark, and that there is one for every badge that has a colour.
- **Seen** (the same isolated copy, `target/k3-prep/seen/ux/`): the three badges with their
  marks, in a kit's part and at the top of the window. **Not seen**: GridCraft's own window's
  badges (a test finds them by name); the light theme.

**The same day: no link to GridCraft's own window, a filter over the kits, and how much
space they use.** The owner: "let's remove that link to use GridCraft's window", and "We
should be able to FTS to filter dev kits, and we should have a summary of n dev kits and how
much space they use".

- **The page no longer offers GridCraft's own window.** The link at the foot of its list is
  gone, and the message it sent is no longer one GridCraft knows. GridCraft's own window is
  what a shell without a web view shows, and what opens if the page can't be shown. (This
  changes "the user can also choose the dialog" in "The Development window as a page".)
- **A filter over the kits.** A field over the kits in the list; a kit is shown when what it
  says of itself and of its projects (its name, what it is made of, its sentences, its
  projects' names, ids, descriptions and workbooks) holds every word typed. So "python"
  finds the first kit, and a project's name finds the kit it is in. The top of the window
  shows the same kits. In the page what is typed stays in the page: nothing is sent to
  GridCraft for it. Both windows have it.
- **How many, and the space they use.** Over the kits: "2 kits · 5 KB"; each kit says its
  own, at the top of the window and in its part. A kit's space is its projects' files, and
  for a kit that builds also what was built from them, with what the compiler keeps beside
  it (zig's cache, which is the larger part by far). It is measured at most every fifteen
  seconds and again after anything is done in the window; a measure looks at sixty thousand
  files at most, and says "more than" when it had to stop.
- **Seen** (the same isolated copy): the summary, the field, the list without the link, and
  "blink" typed into the field leaving the one kit whose project is named so, in the list
  and at the top. **Not seen**: GridCraft's own window's field (its test filters by it); a
  kit with a compiler's cache of real size, and how long measuring that takes.

**The page houses what Projects, the Console and Macros did in windows of GridCraft's own
(built the same day).** Asked what "the nice dialog" meant, the owner: "I meant using the
svelte dialog for housing the development feature dialogs - the standard gridcraft dialogs
are quite plain."

- **A project is trusted in its own part of the page.** This changes what was recorded above
  ("Reviewing, trusting and running a project is still done in Projects"). What holds:
  - **What is trusted is what was on screen.** GridCraft notes the project's fingerprint
    when its part is first shown. Trust is offered only while the project still has it;
    when its files change, the part says so and offers "Look Again", which notes it afresh.
    The page never names a fingerprint: it sends the project's id and whether trust follows
    its changes, and GridCraft trusts the one it noted, for the project whose part is shown
    and no other.
  - **It ends in the operating system's own question**, as before, with what the project
    will be allowed to do. A no trusts nothing.
  - The control channel can't reach the page, so nothing but a click there asks for it.
  - Stopping the trust is a click, and asks nobody: it only tightens.
- **A trusted project's scripts are run from its part**, on the selection, with what came
  back and what was printed shown there as text; a run can be stopped.
- **The Python that runs scripts is named in its kit's part**, as the C compiler is.
- **The Console and Macros are parts of the window**, in the list beside the MCP server:
  they are no kit's. The Console shows its entries (the latest two hundred), by project,
  copies them, and is cleared by a click there. Macros shows the open workbook's modules
  and the one being read, as numbered lines of text, with what projects say they rebuilt
  of it, and copies the request for an agent.
- **The ribbon's Projects, Console and Macros open the page** at the top, the Console and
  Macros, where the page is. A project is reached through its kit.
- **GridCraft's own windows stay** for a shell without a web view, and are what its own
  Development window opens. They are not changed.

**Checked.** Tests of what the page is told and of every message it can send for this: trust
refused after the files changed, until looked at again; refused for a project that is not
the one shown, and for a name that is no id; the operating system asked once for each yes
and no; a change after trust shown as one; the Python kept; the Console's part, its copy
and its clearing; the Macros part with and without macros.

**Seen** (the isolated copy): the list with the Console and Macros in it; the Console's
part, empty; a kit's part with the Python's field and no button to another window; a
project's part with its Trust section. **Not seen**: a click on "Trust This Project" (it
ends in the operating system's question, and nobody was there to answer it); a script run
from the part; the Console with entries; Macros with a workbook that carries some.

**A fault found by looking, which no test would have found.** The project's part drew
nothing at first: its source took in a helper named `state` and then used the rune
`$state`, which Svelte reads as a store named `state`. It built without a word and failed
when shown. The page's own code is run by no test of the repository: every part that
changes has to be looked at.

### A development kit from a package file (phases P1 to P3, built)

**The owner approved the plan as recommended** (2026-10-13: "1 yes"): a kit that describes
and runs nothing of its own; a zip named `.gridkit`; a removed kit's projects stay and say
so; no signature for such a kit.

**What a package is.** A zip whose name ends in `.gridkit`, with a `kit.toml` at its root:

```toml
id = "dashboards"                 # lowercase letters, digits, hyphens; not a built-in kit's
name = "Dashboards Kit"
version = "0.1.0"
author = "…"
description = "…"                 # what it is, in a sentence or two
made_of = ["Svelte", "Python"]    # shown as badges
base = "anie"                     # the built-in kit its projects are projects of
project_is = "…"                  # optional: what one of its projects is
```

and, optionally, `starter/` (the files a new project of it begins with, where `{{id}}` and
`{{name}}` are written in), `README.md` and a `LICENSE`. Nothing else has a place in it, and
a word in `kit.toml` that GridCraft does not know is refused, not passed over.

**What was built.**

- **Reading a package** (`crates/aniefork-dev/src/kitpack.rs`), bounded: 16 MB the file, 400
  files, 4 MB each and 32 MB in all, counted as they are unpacked (an entry's own word for
  its size is not taken); names of plain letters in folders, with nothing that could leave
  the folder it is unpacked into; a fingerprint over every file.
- **The list of kits is no longer fixed in the code** (`kit.rs`): it is the two GridCraft is
  built with and those found in `kits/<id>/kit.toml` beside the projects folder. Which kits
  there are is asked for a projects folder (`all`, `named_in`, `of_in`, `check_in`), so a
  project is checked against the kits beside it, and tests with settings folders of their
  own do not meet. A kit from a package takes everything that decides from its base: what a
  project may ask, whether it has scripts, a panel, a build, and the languages of its files.
- **Loading** ("+" over the kits): the shell opens the operating system's own file dialog
  (the page names no file), GridCraft reads the package, and the operating system asks, in
  words that say what it is, what it stands on, what it brings, and that nothing in it is
  run. On a yes it is unpacked into `kits/<id>/` and the window turns to it. A kit of an id
  that is already loaded is not written over.
- **Removing** ("Remove Kit" in its part): asked the same way; its own folder is taken away
  and nothing else. **Its projects stay**, and say that their kit is not loaded: they can't
  be trusted or run until it is loaded again. The built-in kits can't be removed.
- Both are by hand: refused to the control channel, and no agent has a tool for either.
- **A new project of a loaded kit** begins from the kit's starter, or, for a kit that brought
  none, from its base kit's, naming this kit. A starter that does not make a sound project
  leaves nothing behind.
- **Packing** (`kitpack::pack`): a kit's folder into a package, read back before it is called
  done. For whoever writes a kit; the window has no button for it yet.
- **What agents are told**: `development_status` lists the loaded kits too.
- The summary over the kits counts a loaded kit's own folder in the space it uses.

**Checked.** Tests of the reader (what is not a package, names that would leave the folder,
files with no place in a kit, sizes, what a `kit.toml` must say), of loading, listing and
removing, of a project made from a starter and from a base, of a project whose kit was
removed and loaded again, and of the whole way through the page: no file dialog, nothing
chosen, a file that is no package, a no and a yes from the operating system, the kit's part,
a project made in it, removal, and that a built-in kit can't be removed.

**Seen** (the isolated copy, with the sample kit of `target/k3-prep/sample-kit/` put where
loading would put it): the "+", "3 kits", the loaded kit in the list, and its part with its
badges, what it stands on and Remove Kit. **Not seen**: a click on "+" (it opens the
operating system's file dialog, on the owner's screen) or on Remove Kit (its question).

**Not built.** Languages for the file viewer from a package (a kit from a package reads its
files as its base does). A button to pack a kit. A kit with code of its own (P4), which waits
for signing. A loaded kit can't be updated in place: it is removed and loaded again.

### Upstream's pull request 990: GridCraft would run VBA (looked at 2026-10-13)

**What the owner asked.** "Look at https://github.com/storytold/gridcraft/pull/990. Should we
integrate, or handle via a plugin system?"

**What it is.** A pull request to upstream named "Vba macros", by Walker-00, opened on
2026-10-13: 13,423 lines added in 72 files, in two commits, with no description. When it was
read it had no review, no check had run on it, and GitHub reported that it conflicts with
upstream's own `main`. It is not in the merge of upstream made the same day. Its second
commit is another matter (lookups over shared ranges recalculate faster).

It makes GridCraft run the macros a workbook carries:

- **A new crate, `gridcraft-vba`**, at layer 1: a reader of the VBA project and of UserForm
  designs, a lexer and a parser of the language, an interpreter and the language's own
  functions. The interpreter reaches the world only through one trait, `Host`, which its
  caller implements. It depends on `core`, `numfmt` and `thiserror`, and forbids `unsafe`.
- **Excel's object model, in the engine** (`crates/engine/src/vba/`): `Application`,
  `Workbooks`, `Sheets`, `Range` and the rest, as that `Host`, over the session's commands.
- **In the UI**: a thread per workbook that runs the interpreter and hands each call to the
  UI thread, UserForms drawn from the file's designs, controls on sheets, Alt+F8 and Alt+F11.
- **Commands**: `developer.runMacro`, `developer.enableMacros`, `developer.macroSource`,
  `developer.visualBasic`; on the control channel `ui.macros.enable`, `ui.macros.run`,
  `ui.macros.state`.

**What it says of safety, and what was found reading it.** Its own words: macros stay off
until the person clicks Enable Content, and there is "no file, process, registry, COM or DLL
access": `Shell`, `Kill`, `CreateObject` and the like are errors, a `Declare`d function
returns 0. Read against this fork's rules:

- **Enabling and running are commands.** `developer.enableMacros`, and `developer.runMacro`
  with `enable: true`, are commands of the engine: the command line, the MCP server and the
  control channel reach them, and the control channel has `ui.macros.enable` and
  `ui.macros.run` besides. So an agent, or anything that holds the control channel, can turn a
  workbook's macros on and run them with nobody at the screen. Here, running code is the
  person's alone to do, by hand.
- **Enable Content runs `Workbook_Open` or `Auto_Open` at once**, and is a switch on the
  workbook as it is open: it is not a decision about what the macros are (nothing is hashed),
  and it is not kept.
- **A macro can write a file where it likes**: `SaveAs` with a path of its choosing goes to
  `file.saveAs`. It can print. "No file access" has that door in it.
- **The stop is a key**: the interpreter asks its host between statements whether to stop
  (Esc), and calls go 400 deep at most. A macro that never ends, run without a window, has
  nobody to press it.
- **It takes away the notice** that says GridCraft keeps a workbook's macros and does not run
  them. This fork's Macros window, its `developer.macros`, the agents' `list_macros` and
  `read_macro`, and this record all say the same, and would all be untrue.

**Recommended: neither, as asked. Take it only when upstream has, and then govern it as a
kit.**

- **Not into the fork now.** It is not upstream's yet. It lives wholly in upstream's files
  (the engine, the UI's `lib.rs`, `xlsx`, the two rules files), which the fork keeps as
  upstream has them; taken early, it would be 13,000 lines nobody has reviewed, in the files
  where every later merge of upstream would then conflict, and it would conflict again with
  whatever upstream finally merges. Where its code came from is not said either; upstream's
  clean-room rule is upstream's to check.
- **Not a second runtime behind a plugin.** An interpreter makes thousands of small calls
  into the object model and draws forms on the UI's own thread: as a separate process (the
  only shape a kit from outside could take here, since nothing is loaded into GridCraft's
  process) that is slow, and it would be the fork's own copy of an object model upstream
  would also have. And how a kit from outside is loaded is still undecided.
- **If upstream merges it, it arrives with a merge of upstream, and the fork puts its own
  rules on it through hooks.** The pull request already has the seam a kit needs: everything
  a macro does goes through `Host`. So: integrate the code, and govern it by the kit model.
  - **A third kit in the Development window**, "VBA", built in by upstream: a workbook's VBA
    project is that kit's workbook code.
  - **Enable Content becomes this fork's trust**: given by hand, for the fingerprint of the
    `vbaProject.bin` the person was shown, confirmed by the operating system's own question,
    and asked again when the project changes. Refused to the control channel and to agents:
    a guard on the three commands, and the control channel's two methods refused where
    `control_refuses` already stands.
  - **What a macro may do is what it was allowed**, as for a project: reading the workbook,
    writing to it, and (apart, and said plainly) saving a file under a name of its choosing
    and printing.
  - **Opening a file still runs nothing.** The pull request agrees.
  - The fork's reader of VBA, its Macros window and the rebuilding of macros as projects stay:
    they are the way off VBA, and they read a project without running it.

**What is the owner's to say.** Whether this fork wants macros to run at all. Until now the
direction was to read them and rebuild what they do as projects. With upstream's runtime
there would be two ways for one workbook: run its macros as they are, trusted; or rebuild
them. They can stand side by side.

**Fetched and built, on the owner's word** ("can we download 990 and test it out?", the same
day). The pull request's head (`9eac084`, on upstream's 0.6.1, 407 commits behind upstream's
`main`) is a local branch, `pr-990`, checked out in a folder of its own, `G:\gridcraft-pr990`.
Before anything was compiled its diff was searched: it adds no build script, no procedural
macro and no package from outside (its one new crate depends on two of GridCraft's own and
`thiserror`); it starts no process and opens no connection; it writes no file but through
`file.save` and `file.saveAs`; it starts one thread for each workbook; it reads a few
environment variables in its tests. Then it was built (release), which runs none of its code.

**It was not run.** Starting it runs thirteen thousand lines nobody has reviewed, as the
person at this machine: that is the owner's to do. `G:\gridcraft-pr990\run-isolated.cmd`
starts it with a settings folder of its own beside it. To be tried on copies of workbooks:
a macro there can save a file under any name it likes.

**Not done.** Nothing of it is in the fork, and nothing was said on the pull request: a
comment there is the owner's to ask for.

**Set aside (the same day).** The owner started it and looked: "the VBA system is really
basic, so we should ignore it for now and continue with our development kit structure."
Nothing of it is taken. On 2026-10-14 the owner said to remove the branch `pr-990` and the
folder `G:\gridcraft-pr990`; deleting them is left to the owner's own hand, with the command
for it.

### Calling a project from the sheet, and an immediate line (to design)

**What the owner asked (2026-10-13).** "From the sheet, do we have a way to call function in
the project? Do we need to introduce a gridcraft immediate window / terminal?" And, on
plugins: "Does gridcraft have a plugin system? It would make sense for functions to be
tested as plugins before being incorporated into the main system."

**What there is.** Upstream has no plugin system: its own `docs/gaps.md` says "no plugin or
custom function API", and its roadmap lists one as work not begun. Its worksheet functions
are a table compiled into the program. In the fork, a project's script runs from its panel,
from its own part of the window (on the selection, by hand), and when an agent asks. **No
formula can call into a project.**

**A formula that calls a project** is what the owner named a library: a project a
spreadsheet can refer to. What it would take:

- **A door in the calculation.** One hook where upstream looks a function's name up: a name
  not in its table is asked of whoever registered (the fork), low in upstream's layers, so
  through something the engine is handed, not a crate it depends on.
- **Two kinds of function, which are not alike.**
  - *WebAssembly* (the C kit's, or any language's): sealed, exact, bounded in time and
    memory. It can calculate with the sheet, like a built-in. It needs a WebAssembly runtime
    inside GridCraft (today one runs only in a panel's web view, so nothing would calculate
    with the panel closed, or from the command line): a dependency to choose.
  - *Python* (the first kit's): a program of the person's that can do anything, and slow to
    call. It can't run because a workbook was opened: opening a file runs nothing. So its
    results are values the person asks to be worked out (a button, or a recalculation they
    start), kept in the workbook, worked out beside the window and not in it.
- **Trust.** Only a trusted project's functions calculate. Without it a cell keeps its last
  value and says why.
- **Names**: by project, as in `=BLINK.TWICE(A1)`. What a workbook shows where the project is
  not installed: its last values.

**An immediate line**: worth having, small. In the Console's part, one line where the person
calls a function of a trusted project (with the selection, or with what they type) and reads
what came back, and the last few lines they ran. It is running code by hand: for a project
they trust that may run Python, and refused to the control channel. **A terminal is not
proposed**: a shell is not GridCraft's to be.

**The owner's to say**: whether WebAssembly functions come first (they are the ones that can
calculate by themselves) or Python's (the first kit's, asked for by hand); and whether a
WebAssembly runtime inside GridCraft is wanted.

### The immediate line, and how a kit is called by name (built 2026-10-14)

**What the owner decided (2026-10-14).** On the immediate window: "yes - we need this." On
functions a sheet can call: "Sheet functions should be a generic way to access into the
development kit - we'll have to use namespaces."

**So a call into a kit is named one way, wherever it is made from**, and the immediate line
is the first place it can be made from:

```text
project.function              the function, handed the cells that are selected
project.function(A1:C9)       handed those cells of the sheet that is shown
project.function("text")      handed a value: text, a number, a list, an object
kit.project.function(…)       the same, saying which kit the project is of
```

- A project is named by its id. **An underscore there stands for a hyphen too**, because a
  formula could not hold a hyphen in a name: `clean_data` is `clean-data`.
- A function is one of the project's scripts, by its file's name without the ending: today
  that is all a project has to call (the script's `main`). When a kit offers other things
  to call (a function inside a script, a function a WebAssembly program exports), they are
  named the same way, under the same project.
- The kit's name is optional while a project's id says which project is meant, which it
  always does today: every project is in one folder.

**What was built.** In the Console's part of the page, a line: what is typed there is read
as a call (`aniefork/immediate.rs`), the project and its function are found, and the script
is run beside the window with what the call hands it. What came back and what it printed are
shown under the line as text, the run can be stopped, and the last twenty lines are kept to
run again. The part names what can be called now.

- **Typing a call is running code by hand.** Only for a project the person trusts as it is
  and that may run its scripts; the trust is checked again by what runs it. Refused while
  the control channel is putting input into the window; no agent has a tool for it.
- One script runs at a time, from here or from a project's part.
- GridCraft's own Console window does not have the line: the page does.

**Checked.** Tests of reading a line (the namespaces, what is handed, what is no call), of
a name standing for an id, and through the page: nothing callable before trust; a line that
is no call, a project there is none of, one not trusted, one named under another kit, a
function it has not, each saying why; refused to what drives the window; a call that starts
the script and ends; the line kept; the same run shown in the project's own part.

**Seen** (the isolated copy, with a project trusted in that copy's own file):
`clean-data.count(A1:B3)` typed into the line, run by this machine's Python, and "3" come
back for three rows of the sheet.

**Not built: a formula that calls a project.** The names are settled above; the rest of
what it takes is in the section before this one, and two things of it were put to the
owner: whether a function that is a person's Python may be worked out without being asked
(it can't run because a workbook was opened), and whether GridCraft gets a WebAssembly
runtime of its own. **The owner's answer (2026-10-14): "We can defer these to a second
phase."** So the first phase is what is built: a project is called by hand, from the
immediate line and from its own part. Neither point is decided, and a formula that calls a
project is not started until they are taken up again. **Upstream has an open series of pull requests, "Python in cells"**
(#469, #477, #482 and more, by another contributor, none merged when looked at): Python
written into a cell, run in worker processes of its own. It is not the same thing as a
formula that calls a project's function, and if upstream takes it the two will have to sit
side by side. Only its titles and one description were read.

### The first kit is named PythonCraft (2026-10-14)

**What the owner decided.** "Let's name our first DevKit : PythonCraft - which is the
Python / Svelte UI, and showcase this in the PR."

**So the kit that was the Anie Development Kit is PythonCraft**, in every window, in what
agents are told, and in what a person is asked. The sections above this one were written
before and call it by the name it had.

- **Its id is `pythoncraft`**: what a `project.toml` says with `kit = "..."`, what a kit's
  package says it stands on, and the first word of a call that names its kit
  (`pythoncraft.clean_data.clean(A1:C9)`).
- **`anie`, the id it had, is still read** as the same kit, in a manifest and in a package
  (`manifest::kit_id`, `kit::named`). Nothing that was made has to be changed: a project's
  files are as they were, so its fingerprint is, and so is the person's trust in it. No kit
  from a package can take the id `anie` for itself.
- **A name stands in a sentence as it reads** (`Kit::called`): "a project of the C
  Development Kit", but "a project of PythonCraft", which is a name of its own.
- **In a call, a kit and a project are named whatever the capitals**
  (`PythonCraft.Clean_Data.clean`), as a function of a sheet is. An id has no capitals, so
  nothing is made ambiguous. A function's name is still exactly its script's.
- The C Development Kit keeps its name: the owner named the first kit only.

**Checked.** The gate, whole (1,886 tests): the kit's name and id wherever they are said;
the earlier id read in a manifest and in a package, and refused as a new kit's id; a call
that names the kit as it is shown.

### Offering it upstream, in stages (a strategy)

**What the owner asked (2026-10-13).** "We need a strategy to PR into the main for this - in
a clear staged way. Introduce the python / svelte Development kit as the first priority
(since it is probably the most useful)."

**What stands in the way, said first.**

- **Upstream's rule is Rust only**: no handwritten JavaScript. A panel needs a script that
  gives its page the spreadsheet (`spreadsheet.js`, text inside Rust here), and Python needs
  its own few lines. Upstream has to say yes to that before any of it is offered; nothing
  else in the first stages breaks a rule of theirs.
- **Upstream is for every system.** Docking a panel in the window, and the Development
  window as a page, are Windows only here. They are not in the first stages.
- **Some of it is the fork's alone and stays**: the Svelte page (upstream's windows are drawn
  by its own toolkit), the marks in the badges (their rule on iconography), Doom.
- **Their time.** The fork's #549 has waited without a review; a contributor's pull request
  of 13,000 lines (990) sits unreviewed. Small pull requests, each whole by itself, are the
  ones that get read.

**The stages.** Each is one pull request, cut from upstream's `main` (not the fork's), with
upstream's own names (no "aniefork"), its tests, its documents brought up to date, and
upstream's gate passing. Each leaves GridCraft whole if the next never comes.

| Stage | What it adds | What runs | Size here, with tests |
|---|---|---|---|
| **U0** | **No code: a proposal**, as an issue. What a project is, how trust works, the stages, and the one thing asked of their rules (a panel's script, as text). Their plugin API is on their own roadmap: this answers it | Nothing | A page |
| **U1** | **Projects that do nothing yet**: the folder and its manifest, what a project asks to be allowed, its fingerprint, the person's trust (given by hand, confirmed by the operating system), a report command, the Developer tab (hidden until Options), the Projects window with review and a starter | Nothing: no code of a project runs | About 4,500 lines |
| **U2** | **A panel**: a trusted project's page, served from the person's own machine to their browser, with the script that gives it the spreadsheet; reading and writing as it was allowed; told when the selection changes | A web page, in the browser | About 3,300 lines |
| **U3** | **Python**: a trusted project's scripts, run by the person's own Python, from its panel and from Projects; the Console | The person's Python | About 2,000 lines |
| **U4** | A project's files, read in review with their syntax coloured | Nothing | Small |
| **U5** | A panel docked in the window, on every system upstream builds for | A web view | To be measured: needs macOS and Linux first |
| **U6** | Agents: the Development server | An agent's requests, answered by the person | About 10,800 lines: a proposal of its own |
| Later | Kits (the C kit, kits from packages), macros to read | | |

**Before U1 can be cut**, in the fork: the names (the crate, the modules, the settings
folder, "Anie Development Kit": the owner's to choose what upstream is offered), and the
hooks, which become upstream's own code.

**As each is merged**, the fork drops its own copy at the next merge of upstream and keeps
only what upstream has not taken, as it did with the selection glide.

**Changed by the owner (2026-10-14): no proposal first, and not yet.** "Is a U0 needed? We
should merge it with real functionality - we need to test this feature more before adding."
Asked whether upstream has pull requests that are only proposals: it has none that were
found, and no discussions board; people bring working code, often large and in series, and
the few issues that only suggest something mostly go unanswered. So:

- **U0 is dropped.**
- **Nothing is offered until it has been used more here.**
- **The first offer carries something that works**: U1 to U3 together are the first kit
  (projects, a panel, Python). Whether that is one pull request or a series that is opened
  together, as others do upstream, is to be settled when it is cut. It is then some ten
  thousand lines, and upstream's rule on JavaScript is met in the pull request itself, not
  before it: that risk is taken knowingly.

**What stands (the owner, 2026-10-14, last): it stays in the fork.** "Ok - let's keep it in
our fork for now. This would be a feature of our fork, and we pull down upstream updates."
So **nothing is offered upstream**, the fork keeps the whole of it, and upstream's work is
merged into the fork as before. And why: "This allows us to develop it our own way, and make
it more mature."

Earlier the same day the owner had named the kit PythonCraft "and showcase this in the PR",
said "We want to be upfront that this is a large feature, meant to provide a modular
development framework to match VBA capabilities and more", and "go ahead when ready and
publish / push the PR when ready". A first offer was begun on that word and stopped on the
later one, before anything was pushed or said on GitHub. What it left, and what was learned:

- **A scratch copy**, on the local branch `pythoncraft-wip` (never pushed), in a working copy
  of its own outside the repository's folder. It has upstream's names (the crate as
  `crates/devkit`, the UI module `devkit`), none of the fork's records and no web view in
  the shell; a pass that takes out the page, docking and the selection settings was stopped
  half way, so it does not build. It is a place to begin from at most, and can be removed.
- **Upstream's own words on this** (its `docs/gaps.md`, item 21, and
  `docs/target-app-parity.md`): "no plugin or custom function API", estimated at 25 to 40
  hours "plus an owner decision"; VBA is "out of scope (owner decision)"; "A plugin or
  custom-function API (for example LAMBDA libraries and WASM custom functions) is the
  open-equivalent path". So an offer would be asking for that decision.
- **Their sizes**: what is merged there is mostly under 500 lines; the largest pull request
  open was 13,400. The first kit alone, cut as this section says, is ten to twelve thousand.
- **What an offer has to say for itself**: the script that gives a panel the spreadsheet,
  the starter's page and the showcase's built Svelte app are JavaScript, kept as text in
  Rust files, and upstream's rule is "Rust only (no handwritten JS/TS)".
- **What the cut takes**, should it be wanted again: in order, the page with docking and
  the settings that are not this feature's; the agents' server; the C kit, kits from
  packages and the macros; then every word that names the fork, the owner or this record,
  and the names kept for what was made before (`anie`, `module.toml`, `modules`). Each is
  woven through `development.rs`, `kits.rs` and their tests, which is most of the work.

**The stages above are kept as the way to do it, if the owner says to.**

### DevCraft: what the fork is for, and its own README (2026-10-14)

**What the owner decided.** "Ok - on our fork, let's look to make a focused README.md
showcasing the Development Kit. Our fork can specialize in it, and we call it \"DevCraft\"."
Then: "Let's get some screenshots showcasing DevCraft (Python + Svelte) and Doom", and "Doom
is the C DevKit".

- **DevCraft is the name of what the fork specialises in**: its development kits, as a
  whole. PythonCraft is the first kit and the C Development Kit the second, whose
  demonstration is Doom. (Read so from the owner's words; nothing in the program is named
  DevCraft yet, and whether a window should be is theirs to say.)
- **The README is `README.md` at the root, in place of upstream's.** It was first made as
  `.github/README.md`, which GitHub shows before the root one and which left upstream's
  file untouched. That hid upstream's README and removed nothing, and upstream's brand
  licence asks for removal (below), so on the owner's word the fork's README took the
  root file's place. It is the one upstream file the fork replaces whole: a merge in which
  upstream changed theirs conflicts there, and is settled by keeping the fork's.
- **It says only what was seen**, and what was not: built and run on Windows 11 only; what
  is Windows only; what is not built. An agent's full control is said as it is: the README
  first said no agent could trust a project, which the Development server's own words
  contradict, and was corrected before anyone read it.
- **Its pictures** were made in the isolated copy (the Svelte Showcase made there and
  trusted in that copy's own file; the Doom project taken from the earlier trial's copy).
  Four are in `docs/images-aniefork/`, each with a row in `ATTRIBUTION-aniefork.md`, and
  the asset check reads that table in the fork (a hook in `xtask/src/assets.rs`). The
  pictures of the kit's own part are left out: they show the path of the folder they were
  made in.
- **Doom's picture is in, on the owner's word**: "We are allowed to show Doom running since
  it is the unmodified shareware version - we want to be clear on this : it is not our
  Doom, but it is running in a C DevelopmentKit in GridCraft." It is the fork's second
  exception to the rule on other parties' images (the first was three marks), listed in
  `ATTRIBUTION-aniefork.md`. The README says beside the picture that Doom is id Software's,
  that what ran is the shareware version with its data unmodified, and that neither its
  code nor its data is in the repository. **Checked before it was said**: the `doom1.wad`
  that ran is 4,196,020 bytes with the SHA-1 published for version 1.9 of the shareware.
  What is not id's own program: the engine is doomgeneric, a port of the released source,
  with one file of glue; the README says that too, so that "unmodified" is claimed of the
  data and of nothing else.
- **Upstream's marks, looked at on the owner's word.** Upstream's brand licence
  (`docs/brand/LICENSE-brand.txt`): "If you distribute or publish a modified version of
  GridCraft, or any work derived from it, you must remove the ArtCraft Marks from it (or
  replace them with your own) and must not call it ArtCraft or present it as an ArtCraft
  product. You may say, in plain text, that your work is based on GridCraft by the
  ArtCraft team. A fork kept only to propose changes back to this repository may keep the
  marks unchanged." The marks are "the ArtCraft name, wordmark and mark". The fork is
  public, and since 2026-10-14 it is no longer kept only to propose changes back. Where
  the marks are in it:
  - **the eight image files in `docs/brand/`**, used by one thing: the logo at the top of
    upstream's `README.md`;
  - **upstream's `README.md`** itself: the logo, "By the ArtCraft team", the other Crafting
    Apps;
  - **in the program**: the About window says "Part of ArtCraft." and links to
    getartcraft.com and "Join the ArtCraft Discord" (`crates/ui-egui/src/dialogs.rs`); the
    Help menu on macOS has the same two links (`apps/gridcraft/src/native_menu.rs`); one
    translation of the Discord line;
  - in words only, which the licence allows: `AGENTS.md`, `CLAUDE.md`, `ATTRIBUTION.md`,
    and the fork's README, which says GridCraft is the ArtCraft team's work and that the
    fork is not theirs.
  **The owner's word (2026-10-14):** "Yes - let's do it - we need to do it anyways from
  their rules. We reference upstream of course." "Delete their logo files." And yes to
  the program's own words. **Done:**
  - the fork's README is the root `README.md`; upstream's is no longer in the fork;
  - the eight logo files are deleted. The licence's own text stays in `docs/brand/`;
  - Help ▸ About says, under the name and version, what the program is, "This is a fork
    of GridCraft, with DevCraft: development kits for the spreadsheet.", the fork's page,
    "Based on GridCraft by the ArtCraft team." and upstream's repository. The Help menu
    on macOS has the fork's page and upstream's repository. Neither sends anyone to
    upstream's site or community for a program that is not upstream's. It is
    `aniefork/about.rs`, with a test that the words name the team once and only so;
  - **two hooks that take upstream's lines out**, which no hook did before
    (`dialogs.rs`, three lines; `native_menu.rs`, two). After a conflict there,
    upstream's lines go again.
  **What is left, and why**: the copyright notices in `LICENSE-MIT` and `NOTICE` (they
  must stay); upstream's documents that name the team in words; one line of upstream's
  Portuguese table, now unused; the program's name, GridCraft, which the licence does not
  list among the marks; and the addresses in `packaging/` (getartcraft.com and the
  Discord, in what an installer says of itself), which matter only if the fork gives out
  builds, and it gives out none. The sister forks have the same files and were not
  touched.
- **The repository is `arcanii/DevCraft`.** The owner asked whether to rename it, was told
  what stood for and against (the program it builds is still GridCraft in every window,
  in its settings folder and in its file names; GitHub sends the old address on, so it
  can be done at any time; a name ending in Craft beside upstream's family of apps may
  be read as one of theirs, which their licence asks a fork not to suggest), and renamed
  it on GitHub the same day. So: the README is headed DevCraft and says in its first lines
  that it is a fork of GridCraft by the ArtCraft team and that the program is still
  GridCraft; the About window says "This is DevCraft, a fork of GridCraft with development
  kits for the spreadsheet."; every address the fork gives of itself is the new one; and
  the local `origin` points there. The folder on disk, the crates, the program and its
  settings folder keep their names: renaming those would touch upstream's files
  everywhere, and is not asked.

**Checked**: the gate, whole, on all of it (1,888 tests). **Seen**: on GitHub, the fork's
README as the front page with its five pictures, in the form the owner pushed
(`.github/README.md`); in the isolated copy, Help ▸ About with the fork's words and its two
links, and none of upstream's three lines. **Not seen**: the README at the root as GitHub
draws it (that needs a push, which is the owner's); the Help menu on macOS, which was never
compiled here.

### Quake, built by the C kit: the second challenge (2026-10-14)

**What the owner asked.** "Can we test Quake in the C Development Kit - this is challenge 2",
and yes to the two downloads it took and to where it lives.

**It is not in this repository, and never will be**, as Doom is not: Quake's code is under
the GPL and its game data is id Software's. The project is `G:\gridcraft-quake` (its own
README says what it is and what is in it).

**What was taken, and from where.** quakegeneric (github.com/erysdren/quakegeneric, commit
`13052102`, GPL-2 or later): id Software's WinQuake source made portable, as doomgeneric is
for Doom. And the game data, `pak0.pak` of the shareware version 1.06, out of the shareware
release as the Internet Archive holds it (`msdos_Quake106_shareware`): 18,689,235 bytes, MD5
`5906e5998fc3d896ddaf5e6a62e03abb`, which is the checksum remembered for it. Only the data
file and the licence's text were taken out of that archive; none of its programs was unpacked
or run.

**What happened.**

- **The kit built it without a change to the kit**: 69 files, 2.4 MB of WebAssembly, in about
  nine seconds.
- **Two things in Quake's code met the kit's rule that a project names its files and a few
  settings, and never the compiler's command line:**
  - *The date and time of the build*, which Quake prints in two places: the kit's compiler
    refuses them (a build must come out the same each time). Two lines changed.
  - *`setjmp` and `longjmp`*. The compiler has them for WebAssembly only with an option, and
    a project can't ask for one. Quake jumps out of a frame on an error, and also, with
    nothing wrong, each time a demo ends. The project carries its own `setjmp.h`, found
    before the compiler's (the recipe's folders are searched first), in which a jump stops
    the program; and the one jump that is no error is taken out of Quake's code (two places).
- **The first run played a whole demo and then stopped** on the next one's Loading plaque:
  "memory access out of bounds". Found outside the window, with the kit's loader in Node and
  a line added to a copy of it to say where: without the jump, Quake read on in the demo
  past its end, and a demo lies inside the pak, so what it read was the next file, taken as
  a message of a nonsense length, which overran a buffer. The demo that ended is now closed
  at the place the jump was. (A first guess, the stack running into the program's data, was
  wrong: the kit's programs have 16 MiB of stack, laid first, where running out stops the
  program at once.)

**Seen.** In the isolated copy: Quake docked beside the sheet, playing its three demos one
after another (the Necropolis, the Grisly Grotto, the Door To Chthon), and "Link to a sheet
named Quake" making the sheet: 21 variables four times a second, nine places of the game's
memory with the bytes there and the number a formula works out of them, and two charts. In
Node, with the kit's loader: the three demos round for seven minutes of the game's time; and
a new game begun through the game's own menu with keys (Esc, Enter, Enter), in which walking
forward moved the player, turning right turned them, and firing used shells. So the server's
side of Quake, its own little language included, runs too, not only a recording.

**Not seen.** Anyone playing by hand in the docked panel: the keys were handed over in Node,
through the same function the page calls. **Not there**: sound, a mouse, a saved game kept
from one run to the next.

**What the kit learned from it.**

- **A jump out of a frame is common in old C, and the kit had none.** It has one now, on
  the owner's word: the section after this one. Quake is built with it, and its own jumps
  are back as they were written.
- **A program that stops is stopped for good.** Quake would go back to its console after an
  error; here the page says what happened and that is the end. With a jump there would be
  less to stop for.
- **The loader in Node was the way to find the fault**: what the page's part of the window
  shows of a stopped program is one line. A copy of the loader that says where was made in
  scratch; whether the loader should say so itself, to a developer, is a thing to decide.
- **The shareware licence of Quake names screenshots**: no commercial use of the game "or
  any portion thereof, such as a screen display or a screenshot". The owner asked how a
  magazine could show one, and whether a use that is not commercial is fair; told how that
  is usually reasoned (reporting and comment, one frame, nothing sold, nothing of the game
  replaced; decided case by case, and a doctrine of one country), and that it was no legal
  advice, they said: "Yes, let's add the Quake picture - if someone buys the game the
  proceeds will go to the original publisher, not to me. This is fair use, since I'm
  actually unintentionally promoting the game for them." **So one picture of Quake is in
  the README**, the fork's third exception to the rule on other parties' images
  (`ATTRIBUTION-aniefork.md`), with "It is not our Quake either" said beside it. It is of
  the build that uses the kit's `setjmp` setting.

### A recipe may say its C jumps: `setjmp` (built 2026-10-14)

**What the owner decided.** Asked, after Quake, whether the C kit should gain a setting for
`setjmp`: "Yes."

**What it is.** One more named setting in a C project's recipe:

```toml
[c]
sources = ["src"]
setjmp = true        # its C uses setjmp and longjmp
```

It is yes or no, and nothing else: a recipe is still a list, never a command, and nothing a
project says becomes an option of its own choosing.

**What GridCraft does for it** (`crates/aniefork-dev/src/c.rs`):

- each of the project's files is compiled with `-mexception-handling -mllvm
  -wasm-enable-sjlj`: in WebAssembly a jump is made of exceptions, and the compiler makes one
  only when told;
- the parts are put together with `-mexception-handling`, so that the compiler's library
  comes with its own part of a jump;
- **and with a tag, which the compiler's library throws with and does not define**
  (`__c_longjmp`). With this zig (0.17.0) the putting together fails for want of it. GridCraft
  supplies it: three lines of assembly of its own (`JUMP_TAG`), written into the build's
  folder and built beside the project's parts. The project's own files are still built with
  no assembly at all.

**What it does not change.** The program is no less shut in: it gains no way out, only a way
back up its own stack. Nothing is asked of the loader or of the page. A program whose recipe
does not say `setjmp` is built exactly as before.

**Found by trying** (a program of twenty lines, in scratch, then the kit's own loader in
Node): the first option alone leaves the header refusing; both leave the tag undefined; with
the tag, a jump from five calls deep comes back with what it carried.

**Checked.** The gate, whole. The recipe: the word is read, is no when not said, and is
refused when it is anything but yes or no. With the compiler: a program that jumps does not
build as any project is built (the compiler's own header says why), builds when its recipe
says `setjmp`, counts no source that is not the project's, and leaves nothing in the project.

**Seen, with Quake** (the reason for it): Quake's own jumps put back as id Software wrote
them, its recipe saying `setjmp = true`, built by the kit, and run. In Node: the three demos
round for seven minutes of the game's time, each ending in a jump, and a new game through
the menu, walking and firing. Docked in the isolated copy: the three demos one after another
in the window's own web view, and the sheet following. Of Quake's code only the two lines
that print the date of the build are changed now.

**Not seen**: an error inside the game, after which Quake should now go back to its console
and the program go on. No way was found to make one happen on purpose in a demo.

**What is left of what Quake taught** (ADR-0001, "Quake"): a program that stops shows one
line of why; sound; a mouse.

### What trying Quake by hand asked of the Development window (2026-10-14)

The owner put Quake in their own GridCraft ("put everything in the real projects folder":
the project's 121 files went into their projects folder, each checked the same as in
`G:\gridcraft-quake`, and nothing else of their settings was written), and tried it. What
they met, and asked for:

- **"I get a zig error when I test it."** They had named the folder zig is in
  (`C:\util\Tools\zig-0.17.0`) and not the program in it, and were told it "did not answer
  as zig does". **The kit takes either now** (`c::find`): a folder with zig inside is the
  compiler; a folder with none says so, and says what to name. Zig comes as a folder, and
  the folder is what a person has in hand.
- **"For the C compiler, can we add a dialog to find the compiler instead of just typing it
  in?"** **Browse…** beside the field opens the operating system's own file dialog
  (`Act::PickCompiler`; the shell says how, as it does for a kit's package). What is chosen
  is kept and asked what it is, exactly as a name typed is. It is by hand: what drives the
  window can't open the dialog, since what is chosen is a program GridCraft will run.
- **"On the development window, should we add a refresh button (or add a check if any files
  change)?"** The check is there already and was said so: while the window is open what it
  shows is worked out again a few times a second, and the projects folder is read again at
  most once a second, so a project put there appears by itself. No button was added: it
  would do nothing the window does not do. (The picture they sent was taken as the files
  were still being copied.)
- **"Let's maintain a devCraft marketing version different than GridCraft (start at 0.1.0) -
  This can be seen from the Development menu."** `DEVCRAFT_VERSION` (`aniefork-dev`), 0.1.0,
  changed by hand when the owner says a version is made. The top of the Development window
  says "DevCraft 0.1.0 · GridCraft 0.6.1", and Help ▸ About says which DevCraft under
  GridCraft's own version. GridCraft's version stays upstream's.
- **"We need a better dialog icon for the Development dialogs."** The Development window had
  no icon of its own, so the system's blank one. It has the toolbox now, in its title bar
  and on the taskbar: the same toolbox as the ribbon's button and the page's heading, drawn
  dot by dot in code (`apps/gridcraft/src/aniefork/icon.rs`), so the repository holds no
  picture file for it.

**Checked.** The compiler by its folder, with the real zig, and a folder with none; the
dialog refused to what drives the window, nothing kept when nothing is chosen, what is
chosen kept and asked; the versions as they are said; the toolbox's picture (its colours
where they belong, clear around it, the same on both sides, at three sizes).

### Quake played by hand, and what was asked next: the mouse, leaving, OpenGL, a network (2026-10-14)

**The owner played it**: "I ran Quake by hand, connected to a Quake sheet." So the panel's own
handling of the keys, which nothing here had seen, works. Four things came of it.

**1. The mouse (built, in the Quake project, 0.3.0).** "Is there a way we can support mouse
capture - we can use key combo to unlock it, mouse click in window to capture?" A click on
the picture gives it the keys, as before; a click when it has them gives the game the mouse
(the browser's pointer lock), until Esc, which is the browser's own way out and which no
page can take away, or Ctrl+Alt+M. While the game has it, moving it looks about (Quake's own
`+mlook` is held for it) and its buttons are Quake's. Nothing of the kit changed: the page
hands the game how far the mouse moved, as it hands it the keys. **Seen in Node**: with the
mouse handed over, 300 dots across turned the player twenty degrees, as Quake's own numbers
say it should, and the left button fired once. **Not seen**: the mouse held in the docked
panel. It was not tried from here on purpose: a click posted to the picture could take the
real pointer of the machine the owner was using.

**2. Leaving the game (built, 0.3.0).** "When I exit the game it just freezes." Quake's own
Quit ends the program, as it should, and the page showed its last picture and one line. It
now dims the picture, says "You left Quake", and offers "Start Quake again", which loads the
page anew. **Seen in Node**: Quit through the game's menu ends the program with 0. **Not
seen**: the page's own words for it.

**3. OpenGL (looked into, not built).** "Is there a way we can expose OpenGL bindings for 3D
games like Quake?" There is, and it is the kit's to give, not a project's:

- A C program here calls only what the kit's loader hands it (`c_program.rs`): a clock, a
  place to print, files in memory. Anything else it asks for is answered "not there". A
  program that draws does so today by filling a picture the page then copies to a canvas,
  dot by dot: the processor does all of it.
- The web view has WebGL 2, which is OpenGL ES 3 in all but name. **The loader could hand a
  program those calls**, bound to a canvas the page names (`CProgram.load({ canvas })`): each
  GL call a function the program imports, with the numbers GL uses for textures, buffers and
  shaders kept in a table on the page's side. That is some hundreds of functions for the
  whole of it, and a few dozen for a first program that draws lit triangles. It lets nothing
  out: a page may draw with WebGL already.
- **Quake's own OpenGL (GLQuake) is the old kind** (`glBegin`, a stack of matrices), which
  WebGL does not have. It would need either a port of Quake written for OpenGL ES 2, or a
  layer of C that turns the old calls into the new, built with the project.
- So, in stages: (G1) the loader hands a small part of OpenGL ES to a program, and the C
  starter can draw a turning triangle; (G2) the part grows to what a real game needs; (G3) a
  Quake that draws with it. G1 is days, not hours; G3 is the long one.

**4. A network (looked into, not built).** "Can we investigate adding networking function to
the sandbox? What would this take?"

- **A page can't open the kind of connection games use.** Quake speaks UDP. A page may make
  web requests, WebSockets and the like, and nothing else: so a program in a panel can't
  reach a Quake server, or another GridCraft, by itself, whatever the kit allows it.
- **GridCraft can, on its behalf.** The bridge that serves a panel is GridCraft's own program
  on the person's machine (`bridge.rs`). It could carry packets for a project: the program
  hands the page a packet and an address, the page hands it to the bridge, and the bridge
  sends it and keeps what comes back for the page to collect. Games ask for packets without
  waiting, once a frame, which suits a page.
- **What it costs is the kit's promise.** A C program "can do only what its page hands it",
  and so a project that may read the workbook can't send it anywhere. With a network it
  could. So it would have to be: a permission a C project asks for, which it can't today
  (`network` is the first kit's, and the C kit's projects may ask for `read`, `write` and
  `files`); the addresses it may reach named in its `project.toml` and shown before trust,
  with GridCraft holding it to them; the machine itself and the home network closed unless
  named, so a project can't knock on what is behind the person's router; and limits on how
  much. The page of a project without it stays as it is: it may talk to GridCraft and
  nothing else.
- **In stages**: (N1) the permission and the list of addresses, in the manifest and in
  review, with nothing sent yet; (N2) the bridge carries packets, and the loader hands a
  program the few calls for it; (N3) Quake's own network part written on those calls, and
  two GridCrafts on one network playing each other. N1 is small. N2 is the large one: the
  bridge is a small server written by hand, and every line of this is security.
- **What would be decided first, by the owner**: whether a project that may read the
  workbook may have a network at all, or only one that may not.

### A network for a C project, stage N1: asked for, named, and turned on by itself (built 2026-10-14)

**What the owner decided.** To the two stages put to them, yes; and of the question that had
to come first: "the user can turn on networking explicitly. Yes a project can read the
workbook and have network."

**So a C project may ask for the network, and trusting it does not give it.** Three things,
each apart:

1. **It asks**, in its `project.toml`: `network` among its permissions, and where to:

   ```toml
   permissions = ["read", "network"]

   [network]
   allow = ["quake.example.org:26000", "192.168.1.20:26000"]
   ```

   An address is a host, by name or by four numbers, and a port. **Never a pattern**: no `*`,
   no range, no scheme, no path; at most sixteen. A name that ends in a number must be four
   numbers, so that no two readers take it for different hosts. A C project that asks and
   names nowhere, or names somewhere and does not ask, is not a sound project. The first
   kit's `network` is another thing (its page may talk to the internet), takes no list, and
   is as it was.
2. **The person trusts it**, as any project: it is granted what it asks, **and not the
   network**.
3. **The person turns its network on**, in the project's part, by hand: a question of the
   operating system's own says where to, in full, and what stands against it (it may see
   the open workbooks, so it could send them there; an address is on the person's own
   machine or network, so it could reach what is behind their router). The project is read
   again after the answer: what is turned on is what was asked about.

**What is kept is the addresses, not a yes.** The trust file keeps, beside the decision to
trust, the addresses the network was turned on for. It stands only while the project names
exactly those: a project the person lets change as it goes, which then names another
address, has its network off again. Trusting a project again turns it off, each time.
Turning it off is anyone's, since it only tightens; turning it on is refused to what drives
the window, and no agent has a way to it.

**Its page stays shut in.** GridCraft is to carry the program's packets; the page talks to
GridCraft and to nothing else, network on or off. Before this, the page's rules followed
`network` alone: a C project with it on would have had a page let out. Both places that
decide it (the bridge's rules for a page, and where a docked page may go) now ask whether
it is the page's own `network`.

**Nothing is sent yet**, and the part says so in as many words. This stage is the asking,
the naming and the deciding. Carrying packets is N2: the bridge, and the calls the loader
hands a program.

**Checked.** Addresses: what is one, what is on the person's own machine or network, and
twenty-two things that are not one. The kit: asking and naming go together, and are the C
kit's. The trust file: trusting grants no network; on, off, kept across a reload; off
again when trusted again, and when the addresses change under a project that is followed.
A project's page with its network on is served under the same rules as any. The window:
off and not offered before trust; refused to what drives it, with no question put; the
question's words; no leaves it off, yes turns it on; off by anyone; the page's message.

**Seen** (the isolated copy, a project made for it and trusted in that copy's own file): the
project's part with its Network section, both addresses, both warnings, "Off", and the
line that nothing is carried yet. **Not seen**: the operating system's question, and the
part with the network on: the button was not pressed from here.

**For N2, noted now.** A name can stand for a number on the person's own network: whoever
sends must look at the number it turns out to be, not only at the name. How much may be
sent, and how fast. Whether what a program receives may come from anywhere, or only from
where it sent to.

### OpenGL for a C program, stage G1: a part of OpenGL ES 3, and a showcase that draws (built 2026-10-14)

**What the owner decided.** Asked whether to begin on it: "Yes."

**A C program may draw.** Until now a program drew by filling a picture that its page copied
to a canvas dot by dot. Now:

- **The page names a canvas** when it starts the program: `CProgram.load({ canvas })`.
- **The loader answers the program's OpenGL calls with that canvas's WebGL 2**, which is
  OpenGL ES 3 as a page has it (`c_program.rs`). What OpenGL names by number (a shader, a
  buffer, a texture) the loader keeps under that number. With no canvas named, the calls
  do nothing.
- **The kit supplies the header**: `<GLES3/gl3.h>`, GridCraft's own (`c::GL_HEADER`), put
  into the folder the compiler works in for every build and searched after the project's
  own folders, so that a project's own header of that name wins. Each call in it is one the
  program asks of whoever runs it, by its own name. It is text inside Rust source, as the
  loader is.
- **It is a part, not the whole**: 60 calls. Enough for shaders, buffers of
  corners, textures, depth and blending: what a small 3D program needs. The names and
  numbers are OpenGL's own, so code written for OpenGL ES reads the same.

**It lets nothing out.** The program draws on the one canvas the page handed it. A page may
draw with WebGL already; the program gains no way to the sheet, the network or the files.
Nothing was added to what a project asks to be allowed.

**The C kit has a showcase** (`triangle.rs`): Turning Triangle. One file of C makes two
shaders and three coloured corners and draws them turned; its page has a canvas and calls
`frame` for each picture. Created from the C kit's part ("Create Showcase", which each kit
that has one answers with its own), untrusted like any project, and built before its panel
has anything to run: the part says which of the two is wanting.

**Checked.** The loader answers exactly the calls the header declares, and no other; it
draws on one canvas only. The showcase is a sound project that asks for nothing, and every
call it makes is declared. With the compiler: it builds with the kit's header, what it asks
of its page is drawing calls by name, nothing of the kit's is left in the project, and a
project's own header of the same name wins. The windows: each kit's own showcase, and the
three states of the C kit's (not trusted, not built, ready).

**Seen.** First in a browser, with the loader and the program served as the bridge serves
them: the program made its shaders ("Ready to draw."), and the dots read back from the
canvas were the background at a corner, red near the top corner, cream and blue near the
other two, and after half a turn the red at the bottom. Then **docked in the isolated
copy**, built by the kit (29 KB): the triangle in the panel, and a second later turned.

**Not seen**: "Create Showcase" pressed in the C kit's part (the project was put into the
sandbox as the button would put it); textures, depth and blending, which the showcase does
not use.

**Next, if the owner says**: G2, the part grown to what a real game needs, and looked at
with a program that uses textures and depth; G3, a Quake that draws with it, which needs a
Quake written for OpenGL ES 2 or a layer of C that turns the old calls into the new.

### OpenGL for a C program, stages G2 and G3: the kit's part grown, and GLQuake drawn with it (built 2026-10-14)

**What the owner decided.** "Let's do OpenGL first then networking"; and, asked whether
GLQuake's renderer could be downloaded from id Software's release: "then download the quake gl
components needed".

**G2: the kit's part grown to what a game needs** (`c::GL_HEADER`, and the loader in
`c_program.rs`): 78 calls, where there were 60. Added: the depth range (`glDepthRangef`),
reading the picture back (`glReadPixels`), `glColorMask`, `glPolygonOffset`, `glLineWidth`,
`glBlendFuncSeparate`, `glBlendEquation`, the stencil (`glStencilFunc`, `glStencilOp`,
`glStencilMask`, `glClearStencil`), `glFinish`, `glFlush`, `glGetIntegerv`, `glTexParameterf`,
`glUniform1fv`, `glUniform2fv` and `glVertexAttrib4f`; and the numbers they take (the
comparisons, the blend factors and equations, the stencil's operations, `GL_ALPHA`,
`GL_LUMINANCE_ALPHA`, the two mipmap filters left out before, and a few more).

- **A picture handed over is read at the size it is**: with the rows begun where the program's
  `glPixelStorei` says, and alpha and luminance-and-alpha pictures at one and two bytes a dot.
  Before, every row was taken as tight and an alpha picture as four bytes a dot, which reads
  past the picture. A picture read back is written into the program's memory the same way.
- **`glGetIntegerv` answers what WebGL 2 answers**, and names a thing of the page's (a buffer, a
  texture) by the number the program knows it by.

It lets nothing more out: every call still draws on, or reads from, the one canvas the page
named.

**G3: GLQuake, a project outside the repository** (`G:\gridcraft-glquake`; its README says what
it is). From id Software's release (github.com/id-Software/Quake, commit `bf4ac424`, its
`WinQuake` folder): the files GLQuake's own build compiles, with the parts that talk to a
machine taken as id Software's "null" ones, downloaded on the owner's word and each checked
against the git object id the release's own tree gives for it (123 files). The project keeps
87: the 47 C files built and the 40 headers they read.

- **OpenGL 1, made of the kit's OpenGL ES 3** (`gl/gl1.c`, written for the project and under its
  licence). A shape given a corner at a time is turned into triangles as the corners come, each
  put through the modelview matrix there; the triangles are gathered and drawn many at a time,
  what was gathered being drawn whenever something that changes how it is drawn changes; one
  shader does the fixed recipe (the texture environment for each kind of texture, the alpha
  test); a table holds the program's names for its textures beside OpenGL ES's. **It is the
  project's, not the kit's**: what one program needed. Whether the kit should offer OpenGL 1
  itself (`<GL/gl.h>`), for other games of the time, is the owner's to say.
- **What the kit's compiler refused, and what was changed for it**: five places of id
  Software's code, each marked. The date and time of the build, twice; two variables that two
  files each define (the compilers of 1996 made one of each); and one line at the end of
  `quakedef.h` that reads a header of the project's declaring what GLQuake calls before
  anything declares it. The recipe says `define = ["GLQUAKE"]` and `setjmp = true`.
- **What a canvas is not**: a card of 1997 kept the picture before last, and a canvas begins each
  picture empty. GLQuake drew its status bar again only on two or three pictures, and drew
  each picture into half of a depth buffer it never cleared (the "z trick"). The project's glue
  asks for the status bar on each picture and turns the z trick off.

**Seen.** In a browser (the browser pane), with the loader and the program served as the bridge
serves them: the demos playing, the console and the menu over the faded game, E1M1, E1M4 and
E1M5 started from the console, the sky's two layers of cloud, models, particles, the status bar,
and the canvas following the panel's size. About 37 draws a picture, and a step of the game with
its picture about a tenth of a millisecond (0.8 ms at the worst) on this computer. Then **docked
in the isolated copy, built by the kit** (48 sources, about three seconds, 2.4 MB): playing its
demos, and linked to a sheet, which filled with its variables, the bytes of its memory, the
formulas worked out of them and the two charts.

**Checked.** With the compiler: a program that holds every call the header declares is built,
and asks for each by its exact name. The loader answers exactly the calls the header declares
(as before).

**Not seen**: water (no picture of it was caught); GLQuake played by hand (no click is posted to
a game's picture in the isolated copy: it could take the machine's real pointer); the stencil,
the colour mask, the polygon offset and the other calls GLQuake does not make, beyond the test
above. Agents are not yet told that a C program may draw (neither in G1 nor here).

**Next, if the owner says**: networking, stage N2, which is the owner's order. Whether GLQuake
goes into the owner's real projects folder, as Quake did, and whether a picture of it goes in the
README.

### What Windows calls the program: DevCraft (2026-10-14)

**What the owner decided.** They asked whether the process, which Task Manager listed as
GridCraft, should be DevCraft. Offered the version information alone (recommended) or renaming
the program: "Go with recommended".

- **The version information of `gridcraft.exe`** (`apps/gridcraft/build.rs`, a hook in an
  upstream file): `ProductName` "DevCraft", `FileDescription` "DevCraft (GridCraft with
  development kits)". That is what Task Manager and the file's properties show. Seen: Windows
  describes the running isolated copy so.
- **Not changed**: the program's file (`gridcraft.exe`), its settings folder
  (`%APPDATA%\GridCraft`), the window's title, the version information's `CompanyName`
  (upstream's company), and upstream's files for installers in `packaging/`. Renaming the file
  would move the person's settings, their trusted projects and what was built, and would
  conflict at every merge of upstream.

### OpenGL 1, the kit's own: `<GL/gl.h>` (built 2026-10-14)

**What the owner decided.** Asked whether the kit should offer OpenGL 1 itself, rather than
GLQuake carrying its own: "Yes the kit should offer OpenGL itself as a regular devkit feature."

- **`gl1.rs`**: `<GL/gl.h>`, `<GL/glu.h>` and the C that makes OpenGL 1 of OpenGL ES 3. It is the
  layer written for GLQuake, now the kit's, with three calls of GLU added (`gluPerspective`,
  `gluOrtho2D`, `gluLookAt`). Text inside Rust source: the repository holds no C file.
- **Built into a program that includes it, and only then.** The kit writes the two headers
  beside `<GLES3/gl3.h>` for every build. The compiler says what each source read; when one read
  the kit's `GL/`, the kit compiles its OpenGL 1 as the project's sources are compiled and puts
  it in. A program that does not include it is built as before. A project's own `<GL/gl.h>`
  wins, and then the kit's is neither read nor built in. The recipe says nothing of it.
- **It gains a program nothing**: it draws through the loader's OpenGL ES 3, on the canvas the
  page names.
- **Agents are told now that a C program may draw**: `canvas` in `CProgram.load`, and the two
  headers (the topic "panel"). Neither G1 nor G2 had told them.
- **GLQuake 0.2.0 has no OpenGL of its own**: its `gl/` folder is gone, and its recipe builds
  `quake` and `src`.

**Checked.** The headers declare nothing the C does not define, and every call OpenGL 1 shares
with OpenGL ES 3 is renamed and answered under the new name. With the compiler: a program
written for OpenGL 1 (a view set with `gluPerspective`, a quad a corner at a time) is built with
it, asks the page for OpenGL ES 3's calls and for none of OpenGL 1's, and counts one source of
its own; a program that does not include it asks the page for nothing; a project's own
`<GL/gl.h>` wins.

**Seen.** GLQuake 0.2.0 built by the kit with its OpenGL 1 (47 sources, about three seconds),
docked in the isolated copy, playing its demos and filling a linked sheet. That is the README's
picture.

### A picture of GLQuake in the README (2026-10-14)

**The owner's word:** "Add a GLQuake to the README.md (same exception)". So
`devcraft-glquake.png` is in the README under the third exception (`ATTRIBUTION-aniefork.md`),
with "Still not our Quake" beside it, and the README says that a C program may draw with
OpenGL ES 3 or with OpenGL 1. GLQuake 0.2.0 is in the owner's real projects folder, on their
word, untrusted until they trust it; building it there needs a GridCraft built with the kit's
OpenGL 1.

### A network for a C program, stage N2: GridCraft carries its packets (built 2026-10-14)

**What the owner decided.** "Then on to networking", after OpenGL; N1 had settled who turns it
on (the person, by hand, apart from trusting) and that a project that reads the workbook may
have one.

**A program sends and takes in packets** (UDP), with three calls the kit supplies for every
build, `<gridcraft/net.h>` (`net::HEADER`): `net_send("host:port", data, length)`,
`net_receive(from, from_room, data, room)` and `net_status()`. None of them waits: a game asks
once a frame.

- **The loader** keeps what the program sends and what has come, and swaps them with GridCraft
  in one small request to `net`, GridCraft's own address beside the program: at most once a
  frame while there is something to send, every two frames while there is not, and every two
  seconds while the network is off, to see whether it was turned on. A program that never calls
  them makes no such request. The page reaches nothing more: the packets go to GridCraft, as the
  page's other calls do. `program.network()` tells the page what `net_status` tells the program.
- **The bridge** answers `net` only from the project's own page, and only bytes, and only while
  the project is trusted as it is on disk and the person has its network on for the addresses it
  names now. **That is read again at most every two seconds** (`NET_RECHECK`), not on every
  request as for everything else a bridge serves: an exchange comes many times a second, and
  fingerprinting a project like GLQuake's (18 MB) that often would cost more than the game. A
  change to the project, or its network turned off, stops its packets within two seconds.
- **The carrier** (`net.rs`), one a project, opened while its network is on:
  - sends only to the addresses the project names, or back to an address that wrote to it;
  - takes in only from the hosts it has sent to, and drops the rest;
  - looks at the number a name turns out to stand for: a name the person was told is out on the
    internet is not sent to when it stands for their own machine or network;
  - sends at most 4096 bytes a packet, and 400 packets or 512 KB a second;
  - says each refusal once, in the Developer console, under the project's name, and says there
    when it starts and stops carrying.
- **The windows say what is so now**: "While it is on, GridCraft carries its program's packets
  to these addresses and takes in what comes back from them". Agents are told the three calls.

**Checked.** Packets as they travel between the loader and the bridge. Which numbers are the
person's own machine or network. With sockets on this computer: a packet goes where the project
names and its answer comes back; nothing goes anywhere else, and nothing comes in from a host it
did not send to; the host it sent to may write from another port and be answered there, as a
game's server does; a name that stands for the person's own network is not sent to; so much a
second and no more. Through the bridge, over HTTP: nothing while the project names no address,
nothing while its network is off, carried when it is on, refused elsewhere and said in the
console, only from the page's own address, only by POST, only packets; nothing again when it is
turned off, and when the project changes. The loader, in Node with a stand-in for GridCraft:
nothing without GridCraft's address; "not known" until GridCraft answers, then on; a packet sent
and its answer collected with who sent it; one too long refused; off when GridCraft says no.

**Seen.** In the isolated copy, built by the kit (`net-echo`, 20 KB) and docked, its network turned on in that copy's own trust file, with an echo server on this computer at the address it names: its program sent a packet a second, the server got each from one socket, and the panel showed each answer and who sent it (25 sent, 25 received). The Developer console said that GridCraft carries its packets to that address, and that a packet to an address it does not name was not sent. A file of the project changed under it: within the two seconds its packets stopped (one more got out), and the console said why.

**Known limits, for a later stage.** Each exchange is a connection of its own (the bridge
answers one request a connection): some 30 to 60 a second while a program uses the network,
which the system can bear; a connection that carries many is the better way. An address on the
internet is sent to from a socket open on every address of the machine, and Windows may ask the
person, with its own question, whether GridCraft may use the network: not seen here, where only
this computer's own address was used. IPv6 is carried and was not tried. **Quake over the
network (N3) is not done**: its own network part, written on these calls, and two GridCrafts
playing each other.

### Backlog

- A setup wizard that installs Python for the user.
- Signing in to GitHub, and push (with source control; the recommendation is recorded there).
- (Letting agents use modules moved out of the backlog: it is phase 2e, whose own open
  points are listed with it.)
- Modules and workbooks (the open point above).
