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

**Not built, and open.**

- **There is no way to load another kit.** The list has two entries now, both compiled in
  (phase K1), and the window says so.
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
   a WebAssembly Dev kit later." So C runs first as a program of its own, as Python does.
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

**2. The C kit: C built by the person's compiler, and run as a program.**

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
- **It runs as a program of its own**, the way a script does: started by GridCraft after
  the trust check, in a process of its own, speaking to GridCraft over its pipes. It calls
  the spreadsheet through a small C file the kit gives a project (as the runner gives a
  Python script its `spreadsheet`), and every call is checked against what the project was
  granted at that moment. Unlike a script it may run until it is closed: the person starts
  it and stops it, and for an agent starting it is a request.
- **It can't be limited**, like Python: a program can do anything a program the person runs
  can do. So it is a thing a project asks to be allowed, worded as `python` is, and the
  window says so before trust is given.
- **Later, a WebAssembly kit** (the owner: "we can always make a WebAssembly Dev kit
  later"). Its reason stands: a WebAssembly module can do only what whoever runs it hands
  it, so its projects would be the first that are *held* to what they were granted. It
  would run in a project's panel, or inside GridCraft by an interpreter written in Rust.

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
- **Its window, at first, is its own**: the one `doomgeneric`'s port for Windows opens.
  Docked beside the sheet comes with the WebAssembly kit, or with frames carried over to
  GridCraft; not in the first showing.
- **Its live variables update in the cells** as it is played (the owner's addition), in a
  table (below): health, armour, ammunition, the monsters nearby, what was killed, the
  level's clock, where the player stands; and formulas and a chart over them that move
  with the game. It makes the showcase about the spreadsheet, not only beside it. Doom asks
  for `write` and to run its program.
- **Its game data is Freedoom's by default** (3-clause BSD, which may be passed on with its
  licence text; the shareware WAD may not be, and the rule on assets keeps it out of here
  in any case), kept with the Doom project, not in GridCraft's repository.

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
| K2 | The C kit builds: the compiler found or named, the recipe, a build as a request, what is built kept outside the project | K1. (zig builds `doomgeneric` as it stands: tried, 2026-10-13) |
| K3 | The C kit runs a program: started and stopped by the person, the spreadsheet called from C, held to nothing but its grants' checks | K2 |
| D1 | Doom, as the demonstration: `doomgeneric` cloned by hand, built, run in its own window; the table of its variables written four times a second, with formulas and a chart over it; then the "GridCraft Doom" repository | K3. (Built and run by hand, 2026-10-13) |
| P1 | Files a project changes itself | D1 shows what it needs |
| G1 | Source control, local: repositories among the projects, changes, a commit | |
| G2 | Clone from an address; fetch and pull | G1 |
| L1 | Live values that are not edits: a project publishes them, a function shows them (`RTD`) | A hook in upstream's formula code |
| later | Push and signing in to GitHub; a WebAssembly kit; Doom docked beside the sheet | |

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

### Backlog

- A setup wizard that installs Python for the user.
- Signing in to GitHub, and push (with source control; the recommendation is recorded there).
- (Letting agents use modules moved out of the backlog: it is phase 2e, whose own open
  points are listed with it.)
- Modules and workbooks (the open point above).
