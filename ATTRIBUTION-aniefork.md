# Asset attribution: what the fork holds that is not its own

This is the fork's own list ([arcanii/gridcraft](https://github.com/arcanii/gridcraft)), beside
upstream's [`ATTRIBUTION.md`](ATTRIBUTION.md), which stays as upstream has it. It lists what the
fork adds to the repository that somebody else drew.

**It records an exception.** Upstream's rule ([`CLAUDE.md`](CLAUDE.md), "Assets and iconography")
is that the repository holds no other party's iconography, ever. The fork's owner decided on
2026-10-13 that the badges in the Development window carry the marks of what a development kit
is made of ("we want the logos"), from Simple Icons, in the fork only. So:

- **These are not to be offered upstream.** A branch meant as a pull request to
  `storytold/gridcraft` must not contain them.
- **`cargo xtask assets` does not see them.** It checks asset *files* against upstream's table,
  and these are not files: each is one path of drawing commands, as text in the Development
  page's source. This list is kept by hand.
- Everything else the fork draws is its own, drawn in code: the toolbox and the shield
  (`crates/ui-egui/src/aniefork/icons.rs`, and `Icon.svelte` in the page).

| Asset | Whose mark it is | Source | Licence of the drawing | Where it is, and what for |
|---|---|---|---|---|
| The Python mark (two snakes), one path | The Python Software Foundation. "Python" and the Python logos are trademarks of the PSF. | [Simple Icons](https://simpleicons.org) 16.35.0, `icons/python.svg`, unchanged | CC0 1.0 Universal | `src/logos.js` in the Development page's project (`apps/gridcraft/src/aniefork/page.rs`, built into its `app.js`). In the badge "Python", beside the name of a kit whose projects hold Python scripts. |
| The Svelte mark, one path | The Svelte project. | Simple Icons 16.35.0, `icons/svelte.svg`, unchanged | CC0 1.0 Universal | The same file. In the badge "Svelte", for a kit whose projects can have a Svelte panel. |
| The C mark (the letter in a hexagon), one path | Nobody's in particular: it is how the language is commonly marked. | Simple Icons 16.35.0, `icons/c.svg`, unchanged | CC0 1.0 Universal | The same file. In the badge "C", for a kit whose projects are written in C. |

**What the licence covers, and what it does not.** Simple Icons releases its drawings under
CC0 1.0. That gives no right to the marks themselves: they stay their owners', and each owner
has its own terms for how its mark is shown. Here each is shown unchanged, in one colour, at
the size of a line of text, only to name the thing it stands for. GridCraft is not affiliated
with, or endorsed by, the Python Software Foundation or the Svelte project. Whether that use
keeps to each owner's terms was not checked with a lawyer or with the owners.

**How they were taken in.** Three files, fetched on 2026-10-13 from
`https://cdn.jsdelivr.net/npm/simple-icons@16.35.0/icons/` (the package's licence field:
`CC0-1.0`), read as text. Of each only the path was kept, after checking that the file was one
titled path on a grid of 24 and that the path holds drawing commands and numbers and nothing
else. A test of the shell's checks the same of what the repository holds.

| File | SHA-256 |
|---|---|
| `python.svg` | `ad9468e1c4903f73ae7eebfbe980f0f727a10db695be3d914e7d8bd25356a862` |
| `svelte.svg` | `a38cef0f4fc36bc63519ce34d24a91022f19ec6998ea402c5e155c7479a46f4c` |
| `c.svg` | `fde4284070eedf24f3acc2f78368b8c25a204f9a1b8831caaae4652c3e716c88` |
