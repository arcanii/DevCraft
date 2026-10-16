# The ribbon next to WordCraft's (aniefork)

Status: compared 2026-10-16, GridCraft upstream `f1894c6` against WordCraft upstream `eae2421`, from
both codes and from headless renders of both windows. The owner chose the first upgrade, gaps 1 and
2 (Customize Ribbon and the Quick Access Toolbar): offered as [storytold/gridcraft#1641](https://github.com/storytold/gridcraft/pull/1641) from
the branch `customize-ribbon-and-toolbar` (`bd2153a`; see "Built"). Each upgrade is meant to go upstream as a pull request, as
[`AGENTS-aniefork.md`](../AGENTS-aniefork.md) says. Both apps use the
same `craft-ui` revision and neither builds its ribbon from it: the difference is in each app's own
`crates/ui-egui`.

## Summary

- **WordCraft's ribbon is ahead in what the user can change and in how it explains itself:**
  Customize Ribbon, a Quick Access Toolbar the user fills, group titles with dialog launchers,
  more contextual tabs, ScreenTips that say where a command lives, an inline search box,
  scroll arrows when it doesn't fit, and buttons that report themselves to screen readers.
- **GridCraft's ribbon is ahead in KeyTips:** a 364-sequence table checked by a generator in CI,
  with one generated test per sequence (WordCraft generates letters and has 12 tests), and in
  its Excel-style combos (type-to-filter, a live number-format sample).
- **Neither** shrinks groups (large → small → one button) as the window narrows, shows a live
  preview on hover, or moves focus through the ribbon with the arrow keys.
- **Several gaps are already open upstream** as pull requests by others (below); the rest are
  free to take.

## What WordCraft has and GridCraft lacks

| # | Gap | WordCraft | GridCraft | Open upstream |
|---|---|---|---|---|
| 1 | **Customize Ribbon** | dialog: show, hide and reorder tabs, custom tabs and groups holding any commands, rename, Reset; right-click on the ribbon; saved per user; `tools.customizeRibbon` for agents (`dialogs_ribbon.rs`, engine `ribbon.rs`) | not found | none |
| 2 | **Quick Access Toolbar the user fills** | add, remove and reorder from its … menu, right-click any ribbon button, or Customize Ribbon; numbered KeyTips | a fixed set; "More" holds five commands and Print | none |
| 3 | **Group titles and dialog launchers** | each group titled (Clipboard, Font, Paragraph…), launcher at the corner of seven groups | no titles, no launchers; dialogs through "More…" items. GridCraft's ribbon looks like Excel for Mac's, which hides group titles by default (an option shows them); Excel for Windows shows them. Not checked against the running Excel here | none |
| 4 | **Contextual tabs** | Table Design, Table Layout, Picture Format, Shape Format, SmartArt Design, Chart Design, Equation, Outlining | Table Design, Chart Design, Header & Footer; no Picture Format, Shape Format or PivotTable tabs (`docs/ui-parity.md` claims Format and PivotTable, the code has neither); an expired tab leaves a blank ribbon | none |
| 5 | **ScreenTips** | title with the shortcut (following custom keys), where it lives (Home › Font), "Not available right now" | plain strings with ⌘ written in | #1194 (shortcut per platform) |
| 6 | **Screen readers** | small and menu buttons, tabs and gallery tiles report name, role, enabled, selected | only Paste, Borders, Pictures and Dark Mode are named; tests find buttons by their tooltips | none |
| 7 | **Doesn't fit** | ‹ › arrows scroll the ribbon; tabs that don't fit go into a » menu | clipped at the right edge with no sign: at 1,000 pt wide Cell Styles, Cells and Editing can't be reached | #456 (arrows), #675 (groups collapse into dropdowns) |
| 8 | **Search** | box in the title bar ("Search commands and help"), Alt+Q | a magnifier opens a modal dialog | #1339 (Alt+Q box) |
| 9 | **In-ribbon gallery** | Styles: a row of tiles rendered by the engine, ▲▼, … | galleries only as dropdowns (Cell Styles, Format as Table) | none |
| 10 | **Buttons follow their command** | greyed and inert while the command's `enabled` says no | the ribbon never reads `enabled`. Of 340 commands, 319 are enabled whenever a workbook is open, so this matters only once the engine says more (Merge with one cell, Delete Comment with none…) | none |
| 11 | **Theme checks** | WCAG contrast test, a test banning hard-coded colours | some icons have fixed fills that don't follow dark mode | #456 (contrast test) |
| 12 | **Layout check** | `ribbon_groups_fit_above_their_labels`, every tab in every language | none | #456, #675 (`tests/ribbon_layout.rs`) |

Also seen in the renders: WordCraft labels Cut, Copy and Format Painter beside their icons; GridCraft
shows the icons only.

## Built: Customize the Ribbon and the Quick Access Toolbar (gaps 1 and 2)

Offered as [storytold/gridcraft#1641](https://github.com/storytold/gridcraft/pull/1641), from the branch `customize-ribbon-and-toolbar` (`bd2153a`),
cut from `upstream/main` `f1894c6`. GridCraft's own code,
with WordCraft's as a reference for what it does (the repositories share no code):

- **The layout** (`crates/ui-egui/src/customize.rs`, `RibbonLayout`): tab order, hidden tabs, tabs and
  groups of the user's own, the toolbar's commands. It is part of `UiState`, so it's saved in
  `ui.json`. Read leniently: a broken value gives the default, and the rest of `ui.json` still
  reads. Sanitised: unknown tabs and commands are dropped, duplicates removed, labels at most 64
  characters, at most 32 tabs, 64 groups, 64 commands a group and 64 on the toolbar. A built-in tab
  missing from the saved order (one a later version adds) goes after the built-in tab before it.
- **The toolbar** in the title bar: the user's commands after AutoSave and Home, Undo with its
  list. Its … menu ticks Excel's common commands on and off and opens the dialog. A command with no
  icon in GridCraft's set gets a lettered tile. Not drawn over the File view, which has its own Save.
- **Right-click** on any ribbon control that runs a command (through the `control_keytip` hook
  they all call) adds it to the toolbar or takes it off. Right-click on a tab, the ribbon's empty
  space or a toolbar button offers Customize Quick Access Toolbar…, Customize the Ribbon… and
  Collapse the Ribbon.
- **The dialog** (Customize Ribbon and Toolbar), with Ribbon and Quick Access Toolbar pages: commands
  by tab, with a search box, Add >> and << Remove; tabs with a Show box each, the user's groups and
  their commands under their tab, New Tab, New Group, Up, Down, Name, Reset. Changes apply at once,
  as File › Options does here.
- **Custom groups** follow their tab's own controls: labelled small buttons, three to a column. A
  custom tab holds only its groups. A hidden tab has no KeyTip, and when the tab showing is hidden
  the ribbon moves to the first tab shown.
- **Agents:** `ui.customizeRibbon {layout?, reset?}` → `{layout, tabs}` and
  `ui.quickAccessToolbar {commands?, add?, remove?, reset?}` → `{commands}`; neither opens a dialog.
- **Tests:** seven unit tests (hostile layouts, tab order, groups, toolbar, icons, shortcuts,
  `ui.json` round trip) and four in `crates/ui-egui/tests/customize_ribbon.rs` (a ribbon button onto
  the toolbar by right-click and off again, the … menu, agents' commands and hostile input, the
  dialog filling the toolbar and a new tab and hiding a tab).
- **Looked at:** headless renders of the toolbar, a custom tab, the right-click menu and both dialog
  pages, light and dark.
- **Checks:** `cargo xtask ci` passes on the branch: all 8 steps, 1,622 tests.
- **Left out:** the toolbar below the ribbon, KeyTips on the toolbar and on custom groups, import and
  export of a customisation file, translations of the new words (they show in English until the
  catalogs have them), screen-reader names for the built-in ribbon buttons (gap 6: naming them all
  makes tests that look buttons up by name ambiguous, so it wants its own change).

## What GridCraft has that WordCraft lacks

- **KeyTips:** `keytips/keytips.json` → `cargo xtask keytips` → a generated table and 366 generated
  tests, checked in CI; prefixes, two-letter units, Excel 2003 paths, pending sequences that go
  live when their command lands. WordCraft's KeyTips are one level deep: a menu control's letter
  runs the command instead of opening the menu.
- **Combos:** type-to-filter font, size and number-format boxes; the number format shows a live
  sample of the value.
- **A reason for every disabled command** in the engine (`CommandInfo::disabled_reason`), which
  WordCraft's tooltip would show as a generic "Not available right now".

## Upstream pull requests on the ribbon, by others (open on 2026-10-16)

#456 overflow arrows, split button, theme tokens with a contrast test · #675 groups collapse into
dropdowns when narrow · #1194 shortcuts per platform in tooltips · #1339 Alt+Q search box · #899
the mouse wheel switches tabs · #1180 menu hover highlight · #942 Wrap/Merge captions clickable ·
#1154 tab transitions · #961, #952, #1179, #1573, #393 fonts in the font box · #1068 currency menu ·
#1550 Recommended Charts · #1214 Format Painter · #1549 AutoSave.

## How this was compared

- Two read-only surveys of each app's `crates/ui-egui` on the same fifteen points (declaration,
  adaptive layout, galleries, tooltips, KeyTips, contextual tabs, launchers, display modes,
  QAT and customisation, look, keyboard and accessibility, localisation, state, tests, polish).
- Headless renders: WordCraft's `ui_shot` and GridCraft's `snapshot` example, at 1440 × 900 and
  1000 × 700 points, Home and Insert tabs, and WordCraft's Customize Ribbon dialog. Not committed:
  renders of the apps stay out of the repository.
