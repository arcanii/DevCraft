# Memory use: findings (aniefork)

Status: measured 2026-10-16 on macOS, on upstream `bb685e3` (GridCraft 0.7.0), the way the sister
fork measured WordCraft ([arcanii/wordcraft `docs/memory-aniefork.md`](https://github.com/arcanii/wordcraft/blob/main/docs/memory-aniefork.md),
whose first fix is storytold/wordcraft#2345). Three fixes are offered upstream, each from a branch
cut from `upstream/main`: [#1629](https://github.com/storytold/gridcraft/pull/1629), [#1630](https://github.com/storytold/gridcraft/pull/1630) and [#1631](https://github.com/storytold/gridcraft/pull/1631) (2026-10-16). A fourth, sharing a sheet's other parts between undo steps, is [#1643](https://github.com/storytold/gridcraft/pull/1643). Changes made from this record go upstream as pull requests, as
[`AGENTS-aniefork.md`](../AGENTS-aniefork.md) says.

## Summary

- **The engine is moderate.** 2 million numbers take about 200 MB (about 100 bytes a cell);
  1 million words, about 150 MB.
- **The interface starts at about 258 MB** with an empty workbook in the headless render, and
  **two-thirds of it is fonts**:
  - **Script fallback fonts** (Chinese, Japanese, Korean, Thai, Hebrew, symbols) are loaded at
    startup whatever the workbook holds: about 89 MB of files on a Mac. Without them the
    interface settles at 87 MB.
  - **Every system font is held twice.** egui copies an owned font's bytes into its text engine
    and keeps the original too. Sharing them instead: 258 → 158 MB, with identical pictures.
    **Offered as #1630.**
- **Undo is the multiplier.** Each undo step keeps the workbook as it was, sharing what didn't
  change (copy-on-write, 64-row bands of cells). Up to 100 steps, with no limit by size.
  - **Typing is cheap:** 200 cell edits on a 2-million-cell sheet added nothing measurable.
  - **A command over every row keeps a copy of every cell:** about 100–150 MB a step on that
    sheet. 20 sorts peaked at 3 GB; 100 would be about 10–15 GB (est.). Upstream PR #1263
    (open) caps the history by an estimate of the cells it keeps.
  - **Pictures were copied into every step.** One 20 MB picture and 50 cells typed on its sheet
    kept 992 MB. **Offered as #1629:** 22 MB. #1263's estimate counts cells only, so it would
    not have caught this.
  - **Typing copied the sheet's other parts too:** spilled values, row heights, notes, links,
    rules and validations, and the workbook's names and styles, into every step. 100,000 spilled
    cells cost 6.5 MB a typed cell; 10,000 notes 2.2 MB. **Offered as #1643:** 0.02–0.04 MB.
- **Floating pictures on screen were cached for good:** decoded at full size, in one cache for
  the whole app with no limit, kept after their workbook closed. 12 large pictures scrolled
  through kept 258 MB of textures, still there after the workbook closed. **Offered as #1631:**
  40 MB, and none after closing. The same cache told pictures apart by id and size alone, so a
  second workbook could show the first one's picture.

## Measurements

**Setup:**
- MacBook Pro, Apple M3 Max with 64 GB, macOS 27.0.1. Release builds of upstream `bb685e3`, with
  rustup's Rust 1.98.1, in a worktree of their own with its own `CARGO_TARGET_DIR`.
- [`measure_macos.py`](memory-aniefork/measure_macos.py) (wordcraft's sampler, unchanged): the
  physical footprint, which is Activity Monitor's "Memory". Its peak is the kernel's own lifetime
  maximum, so short CLI runs are measured exactly. On Apple silicon the footprint includes the
  process's GPU memory.
- Synthetic workbooks and scripts from [`make_books.py`](memory-aniefork/make_books.py).
- The engine alone through `gridcraft-cli`. The interface through the `snapshot` example
  (offscreen wgpu on Metal, 1440×900 at 2×), built with a 4-second hold after its picture so the
  settled footprint can be read. That hold, and the switches for the A/B rows, were in a
  throwaway copy only.

**Engine only (CLI), peak:**

| Situation | Peak |
|---|---|
| `info values-200k.csv` (2 M numbers) | 149 MB |
| `info values-200k.xlsx` | 199 MB |
| `info text-200k.xlsx` (1 M words) | 156 MB |
| `convert` either CSV to `.xlsx` | 218–226 MB |

**Undo, on `values-200k.xlsx`** (`run --script`; each step one undo step):

| Steps | Peak | Per step |
|---|---|---|
| 0, 1, 10 or 200 cell edits | 199–213 MB | about 0 |
| 1, 5, 20 sorts of A1:J200001 | 381, 938, 3,024 MB | about 140 MB |
| 1, 5, 20 bold toggles over A1:J200001 | 308, 741, 2,366 MB | about 110 MB |

**A 19 MB picture, then cell edits on its sheet** (blank workbook):

| Edits | Before | After the fix |
|---|---|---|
| 0 | 21 MB | 21 MB |
| 10 | 215 MB | 21 MB |
| 50 | 992 MB | 22 MB |

**Interface (headless), settled footprint.** Run twice; the runs agreed within 5 MB.
"No allocator cache" is the same run with `MallocLargeCache=0`, which makes macOS give freed large
blocks back at once:

| Situation | Settled | No allocator cache | Peak |
|---|---|---|---|
| Empty workbook, as upstream | 258 MB | 235 MB | 706 MB |
| Empty workbook, no script fallback fonts | 87 MB | 65 MB | 535 MB |
| Empty workbook, font bytes shared with egui (the fix) | 158 MB | 133 MB | 606 MB |
| The sample workbook, as upstream → fixed | 261 → 160 MB | | 709 → 608 MB |
| A cell in five scripts (Chinese, emoji, Thai, Devanagari, Arabic), as upstream → fixed | 258 → 158 MB | | 706 → 606 MB |

With the fix the three pictures are byte-for-byte the same as upstream's.

**Floating pictures on screen.** `pictures-12.xlsx` and `pictures-24.xlsx` (12 or 24 copies of
`noise.png`, 2600 × 2600, each drawn 640 points square), opened beside a blank workbook,
scrolled through one picture at a time with a picture rendered at each, then closed. A throwaway
example (not committed) did it with the `snapshot` example's harness; `edit.goTo` doesn't scroll
there, so it set the view's scroll. "Textures" is what egui holds (`tex_manager`); the footprint
is settled, with `MallocLargeCache=0` (opening a 243–487 MB file leaves large freed buffers
otherwise; with the cache on, the savings were about the same or larger):

| Pictures | Textures, scrolled | Footprint, scrolled | Textures, closed | Footprint, closed |
|---|---|---|---|---|
| 12, before → after | 258 → 40 MB | 738 → 509 MB | 258 → 0 MB | 504 → 236 MB |
| 24, before → after | 490 → 76 MB | 1,211 → 779 MB | 490 → 0 MB | 745 → 236 MB |

Inserting the same pictures one by one instead peaks at 7 GB for 24, before and after this fix
alike: each insert's undo step copies every picture already there (the undo fix above).

**What a cell edit copies besides the cells.** One workbook per part, built with
[`make_parts.py`](memory-aniefork/make_parts.py) through `gridcraft-cli run` and saved as `.xlsx`, then
opened and given 50 cell edits on the same sheet (`edits-50.jsonl`); kernel peak, upstream
`f1894c6` → the branch `undo-shares-sheet-parts` (the size at open varies by a few MB from run to run):

| Part | Open → 50 edits, before | After | Per edit, before |
|---|---|---|---|
| Spilled values, 100,000 cells | 17 → 343 MB | 17 → 18 MB | 6.5 MB |
| Custom row heights, 50,000 rows | 7 → 140 MB | 7 → 8 MB | 2.7 MB |
| Distinct styles (workbook), 5,000 | 22 → 138 MB | 22 → 24 MB | 2.3 MB |
| Notes, 10,000 | 58 → 170 MB | 58 → 58 MB | 2.2 MB |
| Hyperlinks, 10,000 | 20 → 101 MB | 20 → 22 MB | 1.6 MB |
| Conditional-format rules, 2,000 | 6 → 39 MB | 6 → 7 MB | 0.7 MB |
| Defined names (workbook), 5,000 | 5 → 36 MB | 5 → 6 MB | 0.6 MB |
| Validations, 2,000 | 5 → 27 MB | 5 → 6 MB | 0.4 MB |
| Merges, 5,000 | 4 → 8 MB | unchanged | 0.1 MB |

**Caveats:**
- **The peaks are the harness's.** Rendering the picture costs about 450 MB for a moment, as
  wordcraft found with its `ui_shot`. Compare the settled values.
- **The real app on the desktop was not measured:** it needs a GridCraft window, so ask the owner
  first. Expect its baseline to differ.
- **Not measured on Windows or Linux.** The font files differ there: on Windows the fallbacks are
  Yu Gothic, Microsoft YaHei, Malgun Gothic, Leelawadee and others.

## Where the memory goes

From a read-only survey of upstream `bb685e3`; the references below were checked by hand. Line
numbers will drift as upstream moves. Sizes marked "est." are estimates, not measurements.

### 1. Fonts

- **The fixed set, at every font rebuild:** `font_definitions_with` (`crates/ui-egui/src/theme.rs`)
  loads eight interface and cell faces and then `load_fallback` (`theme.rs:450`): one face per
  script for seven keys, whatever the text. `try_load` (`theme.rs:210`) reads each whole file with
  `std::fs::read`. On this Mac: Hiragino Sans GB (23.5 MB), Hiragino Sans W3 (7.8 MB), Apple SD
  Gothic Neo (55.3 MB), Thonburi, Arial Hebrew and Apple Symbols, about 89 MB; the role faces
  add about 16 MB.
- **Held twice:** epaint's `blob_from_font_data` (epaint 0.36.2 `text/fonts.rs:391`) does
  `data.clone().font`, which copies an owned `Vec`, while egui keeps the original `FontData`.
  Borrowed `'static` bytes are shared instead. When the glyph sheet fills, epaint builds its
  fonts again, so there are briefly three copies.
- **Read again on every rebuild:** at startup twice (the first frame builds the set again,
  `lib.rs` `fonts_set`), and each time the workbook's used-font list or the interface language
  changes.
- **Workbook fonts:** `bind_faces` (`crates/ui-egui/src/fonts.rs:285`) reads a face's file before
  checking whether the key is already bound, up to eight reads a family. One copy per (file,
  face), so a collection whose Regular and Bold are different faces is held twice, and twice
  more by egui. Files up to 64 MB (`MAX_FONT_FILE`, `fonts.rs:20`).
- **The installed-font scan** reads every font file up to 64 MB whole, one at a time, for its
  names: 368 files, 592 MB on this Mac. Freed as it goes; not measured whether the allocator
  keeps it.

### 2. Undo

- **What a step holds:** `HistoryEntry { wb: Arc<Workbook>, .. }`, pushed when the workbook
  changed (`crates/engine/src/lib.rs:353`), up to `UNDO_LIMIT` = 100 (`lib.rs:118`), with no limit
  by size.
- **What is shared:** sheets are `Arc`s, and a sheet's cells are copy-on-write bands of 64 rows
  (`crates/model/src/store.rs:14`). A command that touches every band copies every cell: sorts,
  formats over the whole range, row and column shifts.
- **What is copied at every edit** (`(*d.wb).clone()`, `crates/engine/src/cmd/mod.rs:334`, and
  `Arc::make_mut` of the touched sheet):
  - the workbook's names, styles, theme and custom lists, and **the VBA project's bytes**;
  - every field of the touched sheet beside the cells, among them **its pictures' bytes**
    (`images: Vec<Image>` with `data: Vec<u8>`), the spill map, comments, rows and columns,
    conditional formats, validations, tables, charts and shapes.
- **Cell pictures** are already `Arc<CellPicture>`, so they are shared.
- **Also pinned:** the last saved state, the clipboard and the Format Painter each hold a whole
  workbook.

### 3. Pictures

- **On screen:** `TEXTURES` (`crates/ui-egui/src/chartview.rs:252`) is a process-wide map of
  floating-picture textures with no limit and no eviction, kept after the workbook closes.
  Pictures are decoded at full size with no limits (`chartview.rs:338`), briefly in two or three
  copies. A picture that fails to decode is tried again every frame. Its key is the picture's id
  and byte length, and ids are per workbook, so two open workbooks can show each other's
  pictures (seen in a test). A picture larger than the GPU's texture limit isn't checked for.
- **Cell pictures** have a bounded cache (`crates/ui-egui/src/cell_pictures.rs`: 64 MB, least
  recently used first), decoded at source size.
- **On open:** an XLSX media part shown by several pictures becomes several copies.
- **PDF export:** floating pictures are decoded without limits (`decode_image`,
  `crates/engine/src/cmd/print.rs:1232`), once for every page they appear on.

### 4. Smaller items

- **The command journal** keeps every command's full parameters, a base64 file or picture
  included, capped at 10,000 entries but not by size (`lib.rs:344`; upstream issue #1228).
- **The style table** only grows, and is copied at every edit.
- **Saving** builds the whole `.xlsx` in memory, with each picture copied into it.
- **Conditional formatting** builds its statistics again every frame (`crates/ui-egui/src/grid.rs:723`)
  and clones them at each lookup (upstream issue #1053). More time than memory.

## What to do

### No downside: just do them (upstream pull requests)

1. **Pictures and macros shared between undo steps.** Offered as
   [storytold/gridcraft#1629](https://github.com/storytold/gridcraft/pull/1629), from the branch `undo-shares-picture-bytes`
   (`a09d3f6`, from `upstream/main` `bb685e3`).
   - **What changed:** `Image.data` and `Workbook.vba_project` are `Arc<Vec<u8>>`, as cell
     pictures already are, with base64 helpers for them (`gridcraft_model::b64::shared` and
     `opt`). JSON and XLSX are unchanged.
   - **Measured:** a 19 MB picture and 50 cell edits on its sheet, 992 → 22 MB.
   - **Test:** `undo_steps_share_picture_and_macro_bytes` (engine).
   - `cargo xtask ci` passes: all 8 steps, 1,612 tests.
2. **Interface fonts held once.** Offered as [storytold/gridcraft#1630](https://github.com/storytold/gridcraft/pull/1630), from the
   branch `font-bytes-shared-with-egui` (`c5baa82`, from `upstream/main` `bb685e3`).
   - **What changed:** `try_load` keeps each interface and fallback file it reads for the rest of
     the run and hands egui borrowed `'static` bytes, which egui shares instead of copying.
     Those faces are loaded at every rebuild anyway, so keeping them costs nothing, and rebuilds
     no longer read them again. A missing or broken file is looked for again next time.
   - **What didn't:** which faces load, and in what order. Workbook fonts (`bind_faces`) still
     load as before.
   - **Measured:** settled 258 → 158 MB with an empty workbook; the same saving with the sample
     and with five scripts; pictures identical.
   - **Test:** `interface_fonts_are_read_once_and_shared_with_egui`.
   - `cargo xtask ci` passes: all 8 steps, 1,612 tests.
   - **Overlap:** upstream PR #1238 (open) adds a loader for PingFang from macOS's font assets
     with `FontData::from_owned`; whichever lands second should load through `font_file`.
3. **Floating-picture textures bounded.** Offered as [storytold/gridcraft#1631](https://github.com/storytold/gridcraft/pull/1631), from
   the branch `picture-textures-bounded` (`3684349`, from `upstream/main` `bb685e3`).
   - **What changed:** a new `crates/ui-egui/src/floating_pictures.rs`, beside upstream's
     `cell_pictures.rs` and built the same way: a cache per egui context, keyed by workbook,
     picture id and size. It is bounded at 128 MB, dropping the pictures drawn least recently
     first, but never one drawn in the last frame, so no picture turns into a placeholder. A
     workbook's textures go when it closes (checked every frame, in `SheetApp::logic`). A
     picture is decoded with limits, after its header shows it within 16,384 pixels a side and
     64 megapixels, to fit the next size step of what is drawn: a power of two or three quarters
     of one, up to 4,096 or the GPU's limit. It is decoded again only when drawn larger than
     that. A picture that can't be decoded is remembered.
   - **Measured:** 12 large pictures scrolled through, textures 258 → 40 MB, footprint
     738 → 509 MB; closed, 504 → 236 MB (table above).
   - **Tests:** four in `floating_pictures.rs` (size steps, budget, closed workbooks, broken and
     oversized pictures) and `each_workbook_shows_its_own_picture` in
     `crates/ui-egui/tests/floating_pictures.rs`, which fails on upstream's code: the second
     workbook showed the first one's picture.
   - `cargo xtask ci` passes: all 8 steps, 1,616 tests.
   - **With the undo fix:** picture bytes become an `Arc` there. Either can land first; the key
     could then be the `Arc`, as cell pictures use. All three branches add rows at the same place
     in `ROADMAP.md`, so the second and third need that merged.
4. **Workbook fonts read only when needed:** check whether a face's key is bound before reading
   its file, and bind one copy per file.
5. **One copy of a media part** shown by several pictures, now that picture bytes are shared.
6. **The touched sheet's other parts shared.** Offered as [storytold/gridcraft#1643](https://github.com/storytold/gridcraft/pull/1643), from
   the branch `undo-shares-sheet-parts` (`5b3600d`, from `upstream/main` `f1894c6`).
   - **What changed:** a copy-on-write wrapper, `gridcraft_model::Shared<T>`: reads go straight
     through, the first write to a copy that shares the part gives that copy its own
     (`Arc::make_mut`). It holds the sheet's `rows`, `cols`, `spill`, `spill_ranges`, `comments`,
     `hyperlinks`, `cond_formats` and `validations`, and the workbook's `names` and `styles`. JSON
     and XLSX are unchanged. Code that reads or writes these parts compiles as before; the change
     outside the model is a handful of `.into()`s and two reads moved before writes.
   - **Two writes made conditional,** because each forced a copy on every edit: calc removed the
     edited cell from `spill_ranges` whether it had a spill or not, and Paste rewrote the spill maps
     and the notes whether the block held any.
   - **Measured:** the table in Measurements; every part now costs 0.02–0.04 MB a cell edit.
   - **Test:** `undo_steps_share_the_parts_an_edit_leaves_alone` (engine): a cell edit shares every
     part with the step before; a new note separates the notes only; undo gives the workbook back.
   - **Left as it was:** merges, tables, charts, shapes, pivots and the other small parts (merges
     measured at 0.1 MB an edit for 5,000).

### Trade-offs: candidates for an option or a decision upstream

| Change | Cost |
|---|---|
| A size limit on undo (#1263 proposes 128 MiB of estimated cells) | fewer undo steps after commands over large ranges |
| Script fallback fonts loaded when a character of the script first appears (−170 MB for a Latin workbook, measured as "no script fallback fonts") | GridCraft must know which scripts the cells and the interface show; a frame of empty boxes before the font arrives |
| A journal limited by size, without file payloads (#1228) | replaying a recorded journal needs the files again |
| Cells stored more densely than a map of rows of maps | a large change to the model |

## Seen in passing

- **A running total is slow.** `=B2*C2+SUM($A$2:A2)` filled down 100,000 rows took 100 s, and
  opening the saved file 160 s, with memory flat (about 380 MB). Each SUM reads its whole range
  again, so the work grows with the square of the rows. Not compared with Excel.

## Upstream context

| Item | State | What it is |
|---|---|---|
| #1227 | open | undo keeps about 16 MB per bulk edit on a large sheet |
| #1263 | open (PR) | caps the undo history by estimated cell storage (fixes #1227) |
| #1228 | open | the journal keeps full parameters, capped by count only |
| #1229 | open | base footprint about 130 times the file size |
| #1053 | open | conditional formatting clones its statistics at each lookup |
| #1226 | open | on the web, each File › Open keeps a document that's never released |
| #1238 | open (PR) | PingFang from macOS's font assets for Chinese interface text |
| #152 | merged | whole-sheet Fill and Remove Duplicates no longer run out of memory |
| #1629 | open (PR, ours) | undo steps share picture and macro bytes |
| #1630 | open (PR, ours) | interface fonts read once and shared with egui |
| #1631 | open (PR, ours) | floating pictures' textures bounded and kept per workbook |
| #1643 | open (PR, ours) | undo steps share a sheet's other parts and the workbook's names and styles |

## Still open

- **The real app on the desktop**, over a long session (needs a window: ask the owner first).
- **Windows and Linux:** not measured.
- **The installed-font scan:** traced, not measured.

## Measuring again

1. Build release copies of upstream's `main` in a separate worktree with its own
   `CARGO_TARGET_DIR`, with rustup's toolchain first on `PATH` (Homebrew's Rust has no wasm
   target, which `cargo xtask ci` needs):
   `cargo build --release -p gridcraft-cli` and
   `cargo build --release -p gridcraft-ui-egui --example snapshot`.
2. Make the workbooks: `python3 -I docs/memory-aniefork/make_books.py <dir>`, then
   `gridcraft-cli convert <dir>/values-200k.csv <dir>/values-200k.xlsx`.
3. Run each through the sampler:

   ```sh
   python3 -I docs/memory-aniefork/measure_macos.py "20 sorts" -- <gridcraft-cli> run --in <dir>/values-200k.xlsx --script <dir>/sorts-20.jsonl --quiet
   python3 -I docs/memory-aniefork/measure_macos.py "picture + 50 edits" -- <gridcraft-cli> run --script <dir>/picture-edits-50.jsonl --quiet
   python3 -I docs/memory-aniefork/measure_macos.py "blank, no cache" MallocLargeCache=0 -- <snapshot> out.png
   ```

   For a settled interface footprint, add a few seconds' sleep at the end of the `snapshot`
   example's `main`, in a copy that is not committed; without it the run ends at its peak.
