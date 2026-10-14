# Asset attribution: what the fork holds that is not its own

This is the fork's own list ([arcanii/DevCraft](https://github.com/arcanii/DevCraft)), beside
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

## The fork's own pictures

The pictures in the fork's README ([`README.md`](README.md)). They are asset
files, so `cargo xtask assets` checks that each has a row here: in the fork the check reads this
table as well as upstream's. Each is a picture of this fork of GridCraft running in a settings
folder made for the purpose, showing upstream's original sample workbook and the fork's own
windows and sample projects. **Each shows the badges** with the marks of Python, Svelte and C
that the table above lists, and nothing else that is another party's.

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `docs/images-aniefork/devcraft-panel.png` | the fork's contributors | screenshot of this fork of GridCraft: the sales sample, the Developer tab, the Svelte Showcase project's panel docked | MIT OR Apache-2.0 | README picture |
| `docs/images-aniefork/devcraft-development.png` | the fork's contributors | screenshot of this fork of GridCraft: the Development window | MIT OR Apache-2.0 | README picture; shows the three marks listed above |
| `docs/images-aniefork/devcraft-project.png` | the fork's contributors | screenshot of this fork of GridCraft: a starter project's part of the Development window | MIT OR Apache-2.0 | README picture |
| `docs/images-aniefork/devcraft-console.png` | the fork's contributors | screenshot of this fork of GridCraft: the Console and its immediate line | MIT OR Apache-2.0 | README picture |
| `docs/images-aniefork/devcraft-doom.png` | the fork's contributors made the picture; **what it shows of Doom is id Software's** | screenshot of this fork of GridCraft with Doom running in a docked panel, built by the C Development Kit | the picture's own parts: MIT OR Apache-2.0; **Doom's artwork: not the fork's, and not licensed by it** | README picture; the second exception, below |

## A second exception: a picture of Doom running

**The owner's word (2026-10-14):** "We are allowed to show Doom running since it is the
unmodified shareware version - we want to be clear on this : it is not our Doom, but it is
running in a C DevelopmentKit in GridCraft."

So one picture in the repository shows another party's artwork: `devcraft-doom.png`, a third of
which is a frame of Doom as it plays.

- **Doom is id Software's**: the game, its name and its artwork.
- **What ran is the shareware version, its game data unmodified**: `doom1.wad`, version 1.9,
  4,196,020 bytes, SHA-1 `5b2e249b9c5133ec987b3ea77596381dc0d6bc1d`, which is the checksum
  published for it (checked on the file that ran, 2026-10-14). The engine that ran it is
  doomgeneric, a port of the source id Software released under the GPL, with one file of glue
  written for the demonstration.
- **Neither is in this repository**: not Doom's code, not its game data. The owner's rule on
  that stands as it was. Only the picture is here.
- **It is said wherever the picture is shown** that it is not the fork's Doom: the README
  says so beside it.
- **It is not to be offered upstream**, like the marks above.
- Another picture of Doom goes in only on the owner's word.

## Upstream's marks: removed from the fork

Upstream's [`ATTRIBUTION.md`](ATTRIBUTION.md) lists eight files in `docs/brand/` (the ArtCraft
logo and mark, in two forms and colours each). **They are not in the fork** since 2026-10-14:
upstream's brand licence ([`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt), which
is kept) asks a published, modified version to remove them. Upstream's table is left as upstream
has it, so those eight rows name files this repository does not hold.

