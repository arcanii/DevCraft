# aniefork: this checkout is a fork

This repository is arcanii/gridcraft, a fork of storytold/gridcraft (the `upstream` remote).
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
  meant for upstream.
- **Syncing with upstream:** `ROADMAP-aniefork.md` lists the steps and what can still conflict.
