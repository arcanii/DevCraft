# aniefork: this checkout is a fork

This repository is arcanii/DevCraft (until 2026-10-14, arcanii/gridcraft), a fork of
storytold/gridcraft (the `upstream` remote). The program shows itself as **DevCraft (GridCraft
fork)** (the owner, 2026-10-18; `crates/ui-egui/src/aniefork/about.rs`); its file, its crates
and its settings folder keep GridCraft's names.
[`CLAUDE.md`](CLAUDE.md) and [`AGENTS.md`](AGENTS.md) are upstream's files and stay unchanged here;
everything in them still applies (never crash, no Microsoft assets, clean room, quality gates).
This file says where the fork differs. Read it and [`ROADMAP-aniefork.md`](ROADMAP-aniefork.md) at
the start of a session. The sister forks arcanii/wordcraft and arcanii/pdfcraft follow the same
rules.

- **Log fork work in `ROADMAP-aniefork.md`, not in upstream's documents.** `ROADMAP.md`, `docs/`,
  `CLAUDE.md` and `AGENTS.md` stay identical to upstream so that merging upstream doesn't conflict.
  Where `CLAUDE.md` says to update the roadmap and parity documents when work lands, add a line to
  the log in `ROADMAP-aniefork.md` instead, and keep its "What the fork carries" table true.
- **Fork-private files carry `aniefork` in their path**, so upstream can never add the same one:
  - a document sits beside the upstream document it stands in for, with a `-aniefork` suffix:
    `ROADMAP.md` → `ROADMAP-aniefork.md`, `AGENTS.md` → `AGENTS-aniefork.md`,
    `docs/control-protocol.md` → `docs/control-protocol-aniefork.md`;
  - code goes in an `aniefork` module of the crate it belongs to
    (`crates/ui-egui/src/aniefork/`), with its tests.
- **Upstream files get hooks only:** a `mod` line, or a short call into the fork's module, with
  `(aniefork)` in the comment above it. Keep the logic on the fork's side of the call. Every hook is
  listed in `ROADMAP-aniefork.md`, so a merge conflict is settled by taking upstream's code and
  putting the hook back.
- **State the UI needs between frames** lives in the fork's module (egui's temporary data, keyed by
  an `aniefork.*` id) rather than in new fields on upstream's structs.
- **A branch meant as an upstream pull request** follows upstream's rules, its progress documents
  included. Cut it from `upstream/main`, not from the fork's `main`.
- **Other people's marks: one exception, and only in the fork.** Upstream's rule is that the
  repository holds nobody else's iconography. The fork's owner made one exception
  (2026-10-13): the badges in the Development page carry the marks of Python, Svelte and C,
  from Simple Icons. [`ATTRIBUTION-aniefork.md`](ATTRIBUTION-aniefork.md) lists them. Add
  another only on the owner's word, list it there, and never put any of them in a branch
  meant for upstream. **A second (2026-10-14): one picture of Doom running**, in the fork's
  README, as the C kit's demonstration: the shareware version, its data unmodified, said
  beside the picture to be id Software's and not the fork's. Doom's code and its game data
  stay out of the repository as before. **A third, the same day: one picture of Quake
  running**, on the same terms, as the C kit's second demonstration; and under it, on the
  owner's word, one of GLQuake, the third.
- **`README.md` at the root is the fork's own, and it is about DevCraft.** It is the one
  upstream file the fork replaces whole (the owner, 2026-10-14). **DevCraft** is the owner's
  name for what the fork specialises in: its development kits (PythonCraft is the first; the
  C Development Kit the second). When a merge conflicts in `README.md`, keep the fork's
  (`git checkout --ours README.md`), then read what upstream changed in theirs in case the
  fork's should say something of it. Its pictures are in `docs/images-aniefork/`, each with a
  row in `ATTRIBUTION-aniefork.md`, which `cargo xtask assets` reads here beside upstream's
  table. Say in it only what was seen working, and say what was not.
- **The fork carries none of upstream's marks.** Upstream's brand licence
  (`docs/brand/LICENSE-brand.txt`, which stays) asks of a published, modified version that it
  remove the ArtCraft marks and not present itself as an ArtCraft product; it may say in
  plain text that it is based on GridCraft by the ArtCraft team. So: the logo files in
  `docs/brand/` are deleted here (if a merge brings one back or changes one, delete it
  again); Help ▸ About and the Help menu say what the fork is, through
  `crates/ui-egui/src/aniefork/about.rs`; and nothing the fork writes uses the ArtCraft name
  except to say whose work GridCraft is. Upstream's own files that name the team in words
  (`AGENTS.md`, `CLAUDE.md`, `ATTRIBUTION.md`, `NOTICE`, the licences) stay as they are: the
  copyright notices must.
- **DevCraft has a version of its own**: `DEVCRAFT_VERSION` in `crates/aniefork-dev/src/lib.rs`
  (0.1.0 since 2026-10-14). It is the fork's, and is changed only when the owner says a
  version is made. GridCraft's version (the workspace's) is upstream's and is never changed
  here: a merge brings it.
- **GridCraft downloads for the C kit's setups, and for nothing else** (the owner, 2026-10-14).
  `crates/aniefork-dev/src/setup.rs` and `setup_fetch.rs` are its one way to download: HTTPS,
  only from the addresses a setup pins and the hosts it names, only on the person's Set Up after
  the operating system's question has named every address and size, and every file kept checked
  against the SHA-256 the setup holds. A new download is a new setup, on the owner's word, with
  its pins taken from files checked against their release's own record. **Doom's and Quake's
  code and their data never go into the repository**: a setup holds where to fetch them, what
  they must hash to, and the lines it changes by number and hash, never their text. The glue in
  `setup_files.rs` is DevCraft's own, and holds nothing of id Software's code or any port's: a
  change to it keeps it so (the test with the real downloads, `GRIDCRAFT_TEST_SETUPS`, says
  whether a run of three lines matches theirs).
- **Syncing with upstream:** `ROADMAP-aniefork.md` lists the steps and what can still conflict.
