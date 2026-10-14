//! The files DevCraft writes into each setup's project, and what it keeps of the code it downloads:
//! each file by where it is in the release, where it goes in the project, its size and its SHA-256.
//!
//! **The texts are DevCraft's own**, under the repository's licence: the C each game needs of a port,
//! its panel, its recipe and its README. Nothing in them is id Software's code, nor any port's: what
//! the games need done as their own machine parts did it, these say in their own way ([`crate::setup`]
//! has the rule, and a test that checks it against the downloads). They were first written for
//! the projects outside the repository (2026-10-14) and rewritten here; edit them here.
//!
//! The sizes and hashes were taken from the files as downloaded at the commits named in [`crate::setup`],
//! each first checked against the git object id the release's own tree gives for it.

use crate::setup::Keep;

/// What DevCraft writes into the doom project, by path in it.
#[rustfmt::skip]
pub(crate) const DOOM_FILES: &[(&str, &str)] = &[
    ("project.toml", r#"# GridCraft Doom: Doom in a panel beside the sheet, with the game's variables in cells.
# A demonstration of the C Development Kit in DevCraft, which set it up. Not part of GridCraft or DevCraft.
id = "gridcraft-doom"
name = "GridCraft Doom"
version = "0.4.0"
author = ""
description = "Doom, docked beside the sheet. Its health, ammunition and monsters, and the bytes of its memory, update in a sheet as it is played."

# The C Development Kit: GridCraft builds the C below as one WebAssembly program, and the
# panel runs it. It can reach nothing but what its page hands it.
kit = "c"

# What this project asks to be allowed.
#   read   to see whether the workbook already has a sheet named "Doom"
#   write  to make that sheet, and to write the game's variables and the bytes of its memory into it
permissions = ["read", "write"]

# What is built: doomgeneric's Doom (the folder doom), and the one file written for this project.
[c]
sources = ["doom", "src"]
include = ["doom"]
define = ["_DEFAULT_SOURCE"]
entry = "none"

[panel]
entry = "panel/index.html"
"#),
    ("README.md", r#"# GridCraft Doom

Doom in a panel docked beside a GridCraft sheet, with the game's own variables (health,
ammunition, the monsters nearby and in sight, kills, the level and its clock) written into
cells as it is played, formulas and charts that follow them, and a table of the game's memory
itself: where a few of those variables lie, the bytes there, and the numbers the sheet works
out of them.

It is a demonstration of the C Development Kit in DevCraft, a fork of GridCraft
(https://github.com/arcanii/DevCraft). **It is not our Doom**: Doom is id Software's game, and
its name and its artwork are theirs. This project is not part of GridCraft or DevCraft.

## How it was made

DevCraft's C Development Kit set it up when Set Up was pressed in the kit's part of Developer ▸
Development. Every file below that DevCraft did not write was downloaded from the place named
and checked against the SHA-256 DevCraft holds for it; nothing that did not match was kept.

- **Doom's code** (`doom/`, 177 files, and `LICENSE`) is doomgeneric
  (https://github.com/ozkl/doomgeneric), a port of id Software's released source, at commit
  `e257a09527062bfb986b6e5e0780839b6cbd5b71`. None of it is changed.
- **The game's data** (`panel/doom1.wad`) is the one chosen at Set Up:
  - the shareware episode, version 1.9, unmodified, out of Debian's archive
    (`doom-wad-shareware_1.9.fixed.orig.tar.gz`); or
  - Freedoom: Phase 1 (`freedoom1.wad` of Freedoom 0.13.0, from its releases on GitHub),
    renamed, with Freedoom's licence beside it as `COPYING-freedoom.txt`.
- **What DevCraft wrote for it**: `project.toml`, `src/gridcraft_doom.c` (what doomgeneric asks
  of a port, the game's variables as the rows of a table, and where a few of them lie in
  memory), `panel/index.html` and `panel/doom.js` (the panel: it draws the game, hands it the
  keys, writes the variables to the sheet, and reads the bytes of the game's memory for it),
  this README and `.gitignore`.

There is no built program in it. GridCraft builds one from `doom/` and `src/` when you press
Build, keeps it beside your settings, and serves it to the panel as `program.wasm`, with the
loader that runs it (`program.js`).

## The sheet

"Link to a sheet named Doom" makes a sheet with three things in it.

- **The game's variables**, 23 of them, as the game's own code reads them, four times a second.
  Beside them a few cells the sheet works out (a bar of health, kills as a share, what is in
  sight), and two charts over the last minute.
- **The game's memory.** For twelve places, the game says only *where* each is. The page reads
  the four bytes at that address out of the program's memory itself and writes them as they
  lie (`56 00 00 00`), and a formula in the sheet turns them round and works the number out
  (86). It is the same 86 the first table and the game's status bar show, by another road:
  that the two agree is the point. The player's own body is a thing the game makes anew on
  each map, so its address changes from level to level.
- Every write is a step of undo.

## Using it

In GridCraft: review the project and trust it in its part of Developer ▸ Development, press
Build (GridCraft needs zig for that: name it in the C Development Kit's part), and dock its
panel. Click the picture to play; "Link to a sheet named Doom" makes the sheet and starts the
writing.

Every change to the folder ends your trust in it until you look again: that is GridCraft's
rule for every project.

## Licence

- **What DevCraft wrote** (listed above) is DevCraft's, under the MIT licence or the Apache
  License, version 2.0, at your choice.
- **Doom's code** (`doom/`) is under the GNU General Public License, version 2: `LICENSE` is
  that licence. A program built from this project holds both, and is under that licence as a
  whole.
- **The shareware `doom1.wad`** is id Software's, given out free under their own terms, which
  are not an open licence: keep it out of any repository (`.gitignore` does). **Freedoom's
  data** is under Freedoom's own licence, `COPYING-freedoom.txt`.
"#),
    (".gitignore", r#"# The game's data is not for a repository (the shareware WAD is id Software's): Set Up put it here.
panel/doom1.wad
"#),
    ("src/gridcraft_doom.c", r#"/*
 * GridCraft Doom: the one file of C that carries Doom into a GridCraft project's panel.
 *
 * Written for DevCraft's C Development Kit, which writes it into the project when the person
 * sets Doom up, and under DevCraft's licence (MIT OR Apache-2.0). Doom itself is doomgeneric
 * (https://github.com/ozkl/doomgeneric, in doom/), a port of id Software's released code, under
 * the GNU General Public License, version 2: a program built from this project holds both, and
 * is under that licence as a whole.
 *
 * None of Doom's files is changed. This file gives doomgeneric the five functions a port
 * writes, and gives the panel's page what it needs: a step of the game, the picture, the keys,
 * and the game's own variables as the rows of a table.
 *
 * Built as WebAssembly: it can do nothing but what the page that runs it hands it.
 */
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include "doomgeneric.h"
#include "doomdef.h"
#include "doomstat.h"
#include "d_items.h"
#include "d_loop.h"
#include "d_player.h"
#include "i_timer.h"
#include "p_local.h"
#include "p_mobj.h"

#define EXPORT(name) __attribute__((export_name(#name)))

/* What "nearby" means, in the map's units. A doorway is 64 wide. */
#define NEARBY_UNITS 512

/* ---- what doomgeneric asks of a port ---------------------------------------------------- */

#define KEYS 64
static unsigned short key_queue[KEYS];
static unsigned key_put, key_take;
static char window_title[80];

void DG_Init(void)
{
}

/* The page takes the picture after each step: there is nothing to do when one is ready. */
void DG_DrawFrame(void)
{
}

/* A page can't sleep. It asks for a step only when one is due (gd_due), so nothing waits. */
void DG_SleepMs(uint32_t ms)
{
    (void) ms;
}

uint32_t DG_GetTicksMs(void)
{
    struct timespec now;
    clock_gettime(CLOCK_MONOTONIC, &now);
    return (uint32_t) (now.tv_sec * 1000 + now.tv_nsec / 1000000);
}

int DG_GetKey(int *pressed, unsigned char *key)
{
    unsigned short held;
    if (key_take == key_put)
        return 0;
    held = key_queue[key_take];
    key_take = (key_take + 1) % KEYS;
    *pressed = held >> 8;
    *key = (unsigned char) (held & 0xff);
    return 1;
}

void DG_SetWindowTitle(const char *title)
{
    snprintf(window_title, sizeof window_title, "%s", title);
}

/*
 * Doom's code for stopping on an error would run another program to show a message. A page's
 * program can run none, and has no such function: here is one that does nothing.
 */
int system(const char *command)
{
    (void) command;
    return -1;
}

/* ---- what the page asks ------------------------------------------------------------------ */

static int last_step = -1;
static int started;

/* Starts the game on the data the page put at /doom1.wad. */
EXPORT(gd_start) void gd_start(void)
{
    static char *args[] = {"doom", "-iwad", "/doom1.wad", NULL};
    doomgeneric_Create(3, args);
    started = 1;
}

/* Whether the game's clock has moved on since the last step: 35 times a second. */
EXPORT(gd_due) int gd_due(void)
{
    return started && I_GetTime() != last_step;
}

EXPORT(gd_step) void gd_step(void)
{
    if (!started)
        return;
    doomgeneric_Tick();
    last_step = I_GetTime();
}

/* A key went down (1) or came up (0): `key` is one of Doom's own codes. */
EXPORT(gd_key) void gd_key(int pressed, int key)
{
    unsigned next = (key_put + 1) % KEYS;
    if (next == key_take)
        return;
    key_queue[key_put] = (unsigned short) (((pressed ? 1 : 0) << 8) | (key & 0xff));
    key_put = next;
}

EXPORT(gd_width) int gd_width(void)
{
    return DOOMGENERIC_RESX;
}

EXPORT(gd_height) int gd_height(void)
{
    return DOOMGENERIC_RESY;
}

/* The picture as a page's canvas takes it: red, green, blue, and all of it opaque. */
EXPORT(gd_picture) const uint8_t *gd_picture(void)
{
    static uint8_t picture[DOOMGENERIC_RESX * DOOMGENERIC_RESY * 4];
    int i;
    if (DG_ScreenBuffer == NULL)
        return picture;
    for (i = 0; i < DOOMGENERIC_RESX * DOOMGENERIC_RESY; i++) {
        uint32_t pixel = DG_ScreenBuffer[i];
        picture[i * 4 + 0] = (uint8_t) (pixel >> 16);
        picture[i * 4 + 1] = (uint8_t) (pixel >> 8);
        picture[i * 4 + 2] = (uint8_t) pixel;
        picture[i * 4 + 3] = 255;
    }
    return picture;
}

EXPORT(gd_title) const char *gd_title(void)
{
    return window_title;
}

/* ---- the game's variables, as the rows of a table ---------------------------------------- */

static const char *const WEAPONS[NUMWEAPONS] = {
    "Fist", "Pistol", "Shotgun", "Chaingun", "Rocket launcher",
    "Plasma rifle", "BFG", "Chainsaw", "Super shotgun",
};

typedef struct {
    int alive;
    int nearby;
    int in_sight;
} monsters_t;

/* A thinker of the level as a thing on the map (a monster, an item, a player), or NULL when it is
   another kind (a door, a lift, a light): the game moves things with P_MobjThinker. */
static mobj_t *as_thing(thinker_t *thinker)
{
    return thinker->function.acp1 == (actionf_p1) P_MobjThinker ? (mobj_t *) thinker : NULL;
}

/* The monsters of the level that are alive, counted from the game's ring of thinkers, which
   begins and ends at thinkercap: how many, how many are near the player, and how many in sight. */
static monsters_t count_monsters(mobj_t *player)
{
    monsters_t m = {0, 0, 0};
    thinker_t *at = thinkercap.next;

    while (at != NULL && at != &thinkercap) {
        mobj_t *thing = as_thing(at);
        int counted = thing != NULL && (thing->flags & MF_COUNTKILL) && thing->health > 0;

        at = at->next;
        if (!counted)
            continue;
        m.alive++;
        if (P_AproxDistance(thing->x - player->x, thing->y - player->y) > NEARBY_UNITS * FRACUNIT)
            continue;
        m.nearby++;
        m.in_sight += P_CheckSight(player, thing) ? 1 : 0;
    }
    return m;
}

/* The names of the table's rows, one a line: what gd_values gives a value for, in its order. */
EXPORT(gd_names) const char *gd_names(void)
{
    return "State\nLevel\nTime (s)\nHealth\nArmor\nWeapon\nAmmo\nBullets\nShells\nRockets\nCells\n"
           "Monsters nearby\nMonsters in sight\nMonsters alive\nKills\nKills of\nItems\nItems of\n"
           "Secrets\nSecrets of\nX\nY\nFacing (degrees)";
}

/*
 * The values, as JSON: one row for each name. Outside a level there is nothing to read, and
 * only the state is said.
 */
EXPORT(gd_values) const char *gd_values(void)
{
    static char out[768];
    player_t *p = &players[consoleplayer];
    monsters_t m;
    ammotype_t held;
    char level[16];
    const char *state;

    if (!started || gamestate != GS_LEVEL || p->mo == NULL) {
        snprintf(out, sizeof out, "[[\"%s\"]]", !started ? "Starting" : gamestate == GS_INTERMISSION ? "Between levels" : "Title");
        return out;
    }
    state = demoplayback ? "Demo" : paused ? "Paused" : menuactive ? "Menu" : p->playerstate == PST_DEAD ? "Dead" : "Playing";
    m = count_monsters(p->mo);
    held = p->readyweapon < NUMWEAPONS ? weaponinfo[p->readyweapon].ammo : am_noammo;
    if (gamemode == commercial)
        snprintf(level, sizeof level, "MAP%02d", gamemap);
    else
        snprintf(level, sizeof level, "E%dM%d", gameepisode, gamemap);
    snprintf(out, sizeof out,
        "[[\"%s\"],[\"%s\"],[%d],[%d],[%d],[\"%s\"],[%d],[%d],[%d],[%d],[%d],"
        "[%d],[%d],[%d],[%d],[%d],[%d],[%d],[%d],[%d],[%d],[%d],[%d]]",
        state, level, leveltime / TICRATE,
        p->health, p->armorpoints,
        p->readyweapon < NUMWEAPONS ? WEAPONS[p->readyweapon] : "",
        held < NUMAMMO ? p->ammo[held] : 0,
        p->ammo[am_clip], p->ammo[am_shell], p->ammo[am_misl], p->ammo[am_cell],
        m.nearby, m.in_sight, m.alive,
        p->killcount, totalkills, p->itemcount, totalitems, p->secretcount, totalsecret,
        p->mo->x >> FRACBITS, p->mo->y >> FRACBITS,
        (int) ((double) p->mo->angle * 360.0 / 4294967296.0));
    return out;
}

/* ---- where the game keeps some of that: for the page to read out of memory itself -------- */

/* One place in the game's memory, as a line: its name, how its four bytes are read, where it is. */
static int place(char *out, int size, int used, const char *name, const char *kind, const void *at)
{
    int wrote;
    if (used >= size)
        return used;
    wrote = snprintf(out + used, (size_t) (size - used), "%s\t%s\t%lu\n", name, kind, (unsigned long) (uintptr_t) at);
    return wrote < 0 ? used : used + wrote;
}

/*
 * A few places in the game's own memory, one a line: a name, how the four bytes there are
 * read, and the address. "int" is a whole number; "fixed" is Doom's fraction, 65536 to the
 * unit; "angle" is a turn, 2 to the 32nd to the circle.
 *
 * No value is handed over here. The page reads the bytes at each address out of this
 * program's memory itself, and the sheet works the number out of them: it is the game's
 * memory that is in the cells. The lines are always the same ones, in the same order. While
 * the player is not on a map, the places that are in the player's body have the address 0.
 */
EXPORT(gd_memory) const char *gd_memory(void)
{
    static char out[1024];
    player_t *p = &players[consoleplayer];
    mobj_t *body = started && gamestate == GS_LEVEL ? p->mo : NULL;
    int size = (int) sizeof out;
    int n = 0;

    out[0] = 0;
    n = place(out, size, n, "Game clock (ticks)", "int", &gametic);
    n = place(out, size, n, "Level clock (ticks)", "int", &leveltime);
    n = place(out, size, n, "Health", "int", &p->health);
    n = place(out, size, n, "Armor", "int", &p->armorpoints);
    n = place(out, size, n, "Bullets", "int", &p->ammo[am_clip]);
    n = place(out, size, n, "Shells", "int", &p->ammo[am_shell]);
    n = place(out, size, n, "Kills", "int", &p->killcount);
    n = place(out, size, n, "Hurt (the red flash)", "int", &p->damagecount);
    n = place(out, size, n, "X", "fixed", body ? (const void *) &body->x : NULL);
    n = place(out, size, n, "Y", "fixed", body ? (const void *) &body->y : NULL);
    n = place(out, size, n, "Speed east", "fixed", body ? (const void *) &body->momx : NULL);
    n = place(out, size, n, "Facing (degrees)", "angle", body ? (const void *) &body->angle : NULL);
    (void) n;
    return out;
}
"#),
    ("panel/index.html", r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Doom</title>
  <style>
    html, body { margin: 0; background: #16181c; color: #d8dbe0; font: 13px system-ui, sans-serif; }
    body { padding: 10px; }
    #screen {
      display: block; width: 100%; aspect-ratio: 4 / 3; background: #000;
      image-rendering: pixelated; outline: none; border: 2px solid #2a2d33; box-sizing: border-box;
    }
    #screen:focus { border-color: #c0392b; }
    p { margin: 8px 0 0; }
    .quiet { color: #9aa0a8; }
    button { font: inherit; padding: 5px 12px; margin-top: 8px; }
    details { margin-top: 10px; }
    pre { background: #0e0f12; padding: 8px; max-height: 12em; overflow: auto; font-size: 11px; white-space: pre-wrap; }
  </style>
</head>
<body>
  <canvas id="screen" tabindex="0" aria-label="Doom"></canvas>
  <p id="says" aria-live="polite"></p>
  <button id="link" hidden>Link to a sheet named "Doom"</button>
  <p id="linked" class="quiet"></p>
  <details>
    <summary class="quiet">What the game printed</summary>
    <pre id="lines"></pre>
  </details>

  <!-- GridCraft serves these two beside this page: spreadsheet.js gives the page its `spreadsheet`
       object, and program.js the loader (CProgram) that runs the program built from this project's C. -->
  <script src="spreadsheet.js"></script>
  <script src="program.js"></script>
  <script src="doom.js"></script>
</body>
</html>
"#),
    ("panel/doom.js", r#"// GridCraft Doom: what the panel's page does.
//
// Written for DevCraft's C Development Kit, which writes it into the project when the person sets
// Doom up, and under DevCraft's licence (MIT OR Apache-2.0). The game it runs is doomgeneric's,
// under the GNU General Public License, version 2: see the project's README.md.
//
// The game is the project's C, built by GridCraft's C Development Kit as one WebAssembly
// program. GridCraft serves it beside this page, with the loader that runs it (program.js):
// the game can do only what that loader hands it, which is a clock, a place to print and
// files kept in memory. It never sees the workbook. This page draws the picture, hands the
// keys over, reads the game's variables out of it and writes them into a sheet through
// GridCraft's `spreadsheet` object, where each call is checked against what you granted.

"use strict";

const GridCraftDoom = (() => {
  // Doom's own numbers for the keys a keyboard's own names don't give.
  const KEYS = {
    ArrowLeft: 0xac, ArrowUp: 0xad, ArrowRight: 0xae, ArrowDown: 0xaf,
    ControlLeft: 0xa3, ControlRight: 0xa3, Space: 0xa2,
    ShiftLeft: 0xb6, ShiftRight: 0xb6, AltLeft: 0xb8, AltRight: 0xb8,
    Enter: 13, NumpadEnter: 13, Escape: 27, Tab: 9, Backspace: 0x7f,
    Comma: 0xa0, Period: 0xa1, Minus: 0x2d, Equal: 0x3d,
  };

  /** Doom's number for a key, by where it is on the keyboard. 0 for one Doom has no use for. */
  function keyCode(code) {
    if (KEYS[code]) return KEYS[code];
    if (/^Key[A-Z]$/.test(code)) return code.charCodeAt(3) + 32;
    if (/^Digit[0-9]$/.test(code)) return code.charCodeAt(5);
    return 0;
  }

  return { keyCode };
})();

// --- the page ----------------------------------------------------------------------------------

if (typeof document !== "undefined") {
  const SHEET = "Doom";
  /** How often the variables are written (milliseconds), and how many seconds the chart looks back. */
  const EVERY = 250;
  const KEPT = 60;
  /** Where the last minute is kept, out of the way of the table and the charts: its first column, and its headings. */
  const LOG = "N";
  const LOGGED = ["Second", "Health", "Armor", null, "Second", "Monsters nearby", "Monsters in sight"];

  const screen = document.getElementById("screen");
  const says = document.getElementById("says");
  const link = document.getElementById("link");
  const linked = document.getElementById("linked");
  const lines = document.getElementById("lines");
  const printed = [];

  const tell = (text) => {
    says.textContent = text;
  };

  /** The row the table of the game's memory begins at, under the table of its variables. */
  const memoryTop = (names) => names.length + 4;

  /** An address as it is written: eight digits of sixteen. */
  const address = (at) => "0x" + at.toString(16).toUpperCase().padStart(8, "0");

  /** Four bytes as they lie in memory, lowest first: "64 00 00 00" is 100. */
  const lying = (bytes) => Array.from(bytes, (byte) => byte.toString(16).toUpperCase().padStart(2, "0")).join(" ");

  /**
   * The formula that works a number out of four bytes written so in the cell `c`. The sheet does
   * this, not the page: the bytes are turned round (the lowest lies first), read as one number,
   * and taken as the kind of number the game keeps there.
   */
  function workedOut(kind, c) {
    const digits = `MID(${c},10,2)&MID(${c},7,2)&MID(${c},4,2)&MID(${c},1,2)`;
    const whole = `IF(HEX2DEC(${digits})>=2147483648,HEX2DEC(${digits})-4294967296,HEX2DEC(${digits}))`;
    const number = kind === "fixed" ? `ROUND((${whole})/65536,1)` : kind === "angle" ? `ROUND(HEX2DEC(${digits})*360/4294967296,0)` : whole;
    return `=IF(${c}="","",${number})`;
  }

  /**
   * Makes the sheet the variables go to: a table of their names, a few cells worked out from
   * them, the last minute second by second, and a chart over that; and under the table, a few
   * places of the game's memory: where each is, the bytes there, and the number the sheet
   * works out of them. One step of undo.
   */
  async function makeSheet(names, places) {
    const last = names.length + 1;
    const row = (name) => names.indexOf(name) + 2;
    const cell = (name) => `B${row(name)}`;
    const table = [["Variable", "Value"], ...names.map((name) => [name, null])];
    const top = memoryTop(names);
    const memory = [
      ["In the game's memory", "Address", "Bytes there", "Worked out in the sheet"],
      ...places.map((place, i) => [place.name, null, null, workedOut(place.kind, `C${top + 1 + i}`)]),
    ];
    const worked = [
      ["Worked out in the sheet", null],
      ["Health", `=REPT("█",MAX(0,ROUND(${cell("Health")}/5,0)))`],
      ["Kills", `=TEXT(IF(${cell("Kills of")}>0,${cell("Kills")}/${cell("Kills of")},0),"0%")`],
      ["Danger", `=IF(${cell("Monsters in sight")}>0,"IN SIGHT: "&${cell("Monsters in sight")},IF(${cell("Monsters nearby")}>0,"Nearby: "&${cell("Monsters nearby")},"Clear"))`],
      ["Ammo left", `=IF(${cell("Ammo")}<10,"LOW","OK")`],
    ];
    await spreadsheet.batch(
      [
        // The new sheet is the one being worked on: the widths that follow are its columns'.
        ["home.insertSheet", { name: SHEET }],
        ["home.columnWidth", { cols: "A:A", width: 140 }],
        ["home.columnWidth", { cols: "B:B", width: 90 }],
        ["home.columnWidth", { cols: "C:C", width: 90 }],
        ["home.columnWidth", { cols: "D:D", width: 160 }],
        ["range.setValues", { range: `${SHEET}!A1`, values: table }],
        ["range.setValues", { range: `${SHEET}!D1`, values: worked }],
        ["range.setValues", { range: `${SHEET}!${LOG}1`, values: [LOGGED] }],
        ["range.setValues", { range: `${SHEET}!A${top}`, values: memory }],
        ["home.bold", { range: `${SHEET}!A1:T1` }],
        ["home.bold", { range: `${SHEET}!A2:A${last}` }],
        ["home.bold", { range: `${SHEET}!A${top}:D${top}` }],
        ["home.bold", { range: `${SHEET}!A${top + 1}:A${top + places.length}` }],
      ],
      { label: "Make the Doom sheet" },
    );
    // Two charts over the last minute, each with the seconds along the bottom. A workbook can do
    // without them: if one can't be made, the table still fills.
    const charts = [
      { range: `${SHEET}!R1:T${KEPT + 1}`, title: "Monsters, the last minute", at: "F1" },
      { range: `${SHEET}!N1:P${KEPT + 1}`, title: "Health and armor, the last minute", at: "F17" },
    ];
    for (const chart of charts) {
      try {
        await spreadsheet.run("insert.chart", { ...chart, type: "line", width: 500, height: 300 });
      } catch (error) {
        await spreadsheet.log("A chart could not be made:", String(error));
      }
    }
  }

  async function page() {
    tell("Loading the game...");
    const data = await fetch("doom1.wad");
    if (!data.ok) {
      tell("There is no game data: put a Doom WAD in this project's panel folder as doom1.wad, and trust the project again.");
      return;
    }
    // The program GridCraft built from this project's C, run by GridCraft's loader.
    const program = await CProgram.load({
      files: { "doom1.wad": new Uint8Array(await data.arrayBuffer()) },
      print: (line) => {
        printed.push(line);
        lines.textContent = printed.slice(-200).join("\n");
      },
    });
    const doom = program.exports;
    doom.gd_start();
    if (program.ended()) {
      tell(program.ended());
      return;
    }
    const [width, height] = [doom.gd_width(), doom.gd_height()];
    // The game, as the rest of this page uses it.
    const game = {
      width,
      height,
      names: program.string(doom.gd_names()).split("\n"),
      ended: () => program.ended(),
      due: () => doom.gd_due() === 1,
      step: () => doom.gd_step(),
      key: (pressed, code) => doom.gd_key(pressed ? 1 : 0, code),
      picture: () => (program.ended() ? null : program.bytes(doom.gd_picture(), width * height * 4)),
      values: () => (program.ended() ? [["Ended"]] : JSON.parse(program.string(doom.gd_values()))),
      // A few places in the game's memory: what each is, how its four bytes are read, and
      // where it is (0: nowhere, for now). The game says where; the page reads what is there.
      places: () =>
        program.ended() || typeof doom.gd_memory !== "function"
          ? []
          : program
              .string(doom.gd_memory())
              .split("\n")
              .filter(Boolean)
              .map((line) => {
                const [name, kind, at] = line.split("\t");
                return { name, kind, at: Number(at) || 0 };
              }),
      // The four bytes at an address of the game's memory, read by this page.
      peek: (at) => program.bytes(at, 4),
    };
    document.title = program.string(doom.gd_title()) || "Doom";

    // The picture.
    screen.width = game.width;
    screen.height = game.height;
    const context = screen.getContext("2d");
    const image = context.createImageData(game.width, game.height);
    const draw = () => {
      const picture = game.picture();
      if (picture) image.data.set(picture);
      context.putImageData(image, 0, 0);
    };

    // The keys, while the picture has them. When it loses them, every key is let go.
    const down = new Set();
    const key = (pressed) => (event) => {
      const code = GridCraftDoom.keyCode(event.code);
      if (!code) return;
      event.preventDefault();
      if (pressed === down.has(code)) return;
      pressed ? down.add(code) : down.delete(code);
      game.key(pressed, code);
    };
    screen.addEventListener("keydown", key(true));
    screen.addEventListener("keyup", key(false));
    screen.addEventListener("blur", () => {
      for (const code of down) game.key(false, code);
      down.clear();
      tell("Click the picture to play.");
    });
    screen.addEventListener("focus", () => tell("Arrows move, Ctrl fires, Space opens, Shift runs, Esc is the menu. Click the sheet to stop."));
    tell("Click the picture to play.");

    // The sheet. Linked by the person's click, or by itself to a sheet that is there already.
    let to = null; // the sheet's name while linked
    let busy = false;
    let before = "";
    let beforeMemory = "";
    let history = [];
    let second = -1;
    const show = () => {
      link.hidden = !!to;
      linked.textContent = to
        ? `Writing the game's variables, and the bytes at a few places of its memory, to the sheet "${to}", four times a second. Each write is a step of undo.`
        : "";
    };
    const unlink = (why) => {
      to = null;
      show();
      tell(why);
    };
    link.addEventListener("click", async () => {
      link.disabled = true;
      try {
        const book = await spreadsheet.workbook();
        if (!book.sheets.some((sheet) => sheet.name === SHEET)) await makeSheet(game.names, game.places());
        to = SHEET;
        before = "";
        beforeMemory = "";
        show();
      } catch (error) {
        tell(`The sheet could not be linked: ${error.message || error}`);
      }
      link.disabled = false;
    });
    try {
      const book = await spreadsheet.workbook();
      if (book.sheets.some((sheet) => sheet.name === SHEET)) to = SHEET;
    } catch (error) {
      // Without leave to read, the person links it by hand.
    }
    show();

    async function write() {
      if (!to || busy) return;
      const values = game.values();
      const full = values.length === game.names.length;
      const steps = [];
      const now = JSON.stringify(values);
      if (now !== before) steps.push(["range.setValues", { range: `${to}!B2`, values }]);
      // The game's memory itself: where each place is, and the four bytes that lie there now.
      // The sheet's own formulas work the numbers out of them.
      const memory = game.places().map((place) => (place.at ? [address(place.at), lying(game.peek(place.at))] : [null, null]));
      const lies = JSON.stringify(memory);
      if (memory.length && lies !== beforeMemory) {
        steps.push(["range.setValues", { range: `${to}!B${memoryTop(game.names) + 1}`, values: memory }]);
      }
      // Once a second of the level's own clock, a line of the last minute.
      if (full) {
        const at = (name) => values[game.names.indexOf(name)][0];
        const clock = at("Time (s)");
        if (clock !== second) {
          if (clock < second) history = [];
          second = clock;
          history.push([clock, at("Health"), at("Armor"), null, clock, at("Monsters nearby"), at("Monsters in sight")]);
          history = history.slice(-KEPT);
          const rows = history.concat(Array.from({ length: KEPT - history.length }, () => LOGGED.map(() => null)));
          steps.push(["range.setValues", { range: `${to}!${LOG}2`, values: rows }]);
        }
      }
      if (!steps.length) return;
      busy = true;
      try {
        await spreadsheet.batch(steps, { label: "Doom" });
        before = now;
        beforeMemory = lies;
      } catch (error) {
        unlink(`Writing to the sheet stopped: ${error.message || error}`);
      }
      busy = false;
    }
    setInterval(write, EVERY);

    // The game: a step whenever its clock has moved on, and the picture after it. A page is
    // called for each picture the screen shows; a timer beside that keeps the game going where
    // a page that is not being looked at is called less often.
    let timer = 0;
    const tick = () => {
      if (game.ended()) {
        tell(game.ended());
        clearInterval(timer);
        return false;
      }
      if (game.due()) {
        game.step();
        draw();
      }
      return true;
    };
    const frame = () => {
      if (tick()) requestAnimationFrame(frame);
    };
    requestAnimationFrame(frame);
    timer = setInterval(tick, 1000 / 35);
    spreadsheet.log(`${document.title} started.`).catch(() => {});
  }

  page().catch((error) => tell(error.message || String(error)));
}
"#),
];

/// What is kept of the doom project's code: 178 files, 1,553,565 bytes.
#[rustfmt::skip]
pub(crate) const DOOM_CODE: &[Keep] = &[
    Keep { from: "doomgeneric/am_map.c", to: "doom/am_map.c", bytes: 27807, sha256: "4d62b1c824f161f84efdc6fb7dfd3c02123a532f28f5296e0eef9c79778aee65" },
    Keep { from: "doomgeneric/am_map.h", to: "doom/am_map.h", bytes: 1243, sha256: "1755affedbe06d36901c5d4754f931ddd214f28ac6625706eb56533925f4651d" },
    Keep { from: "doomgeneric/config.h", to: "doom/config.h", bytes: 2801, sha256: "3fcfe210bf24702f25de2e72ca2bd0c7bdb20b23cb412ffddad15fcacf2031d7" },
    Keep { from: "doomgeneric/d_englsh.h", to: "doom/d_englsh.h", bytes: 21755, sha256: "a87f7d2d4693bdae0e0f8bf81918fe19bd78742d28ee4bfc3c7a92759d73c06a" },
    Keep { from: "doomgeneric/d_event.c", to: "doom/d_event.c", bytes: 1427, sha256: "f06d11316e62d26f8d4687103d77e88ec92e2273b5dedcd798136496ba8e9004" },
    Keep { from: "doomgeneric/d_event.h", to: "doom/d_event.h", bytes: 3186, sha256: "e42a92f9e8bbd27c3f810acc5d50929d9329019240b41af13fc886815ead2a64" },
    Keep { from: "doomgeneric/d_items.c", to: "doom/d_items.c", bytes: 1868, sha256: "57796c266d224c2cab69f9c8c9fae564d4161f04e2f859a80062675ada61e23b" },
    Keep { from: "doomgeneric/d_items.h", to: "doom/d_items.h", bytes: 989, sha256: "f251ed3bcdd5b15250db5c38c6b52996bc684c88802ff87dbfe5b4859d59dab4" },
    Keep { from: "doomgeneric/d_iwad.c", to: "doom/d_iwad.c", bytes: 18889, sha256: "9723efdc547f3739e26b72232b80e36c37c9f685795c3990cb696a7103daf88a" },
    Keep { from: "doomgeneric/d_iwad.h", to: "doom/d_iwad.h", bytes: 1636, sha256: "b9fd8c8bd82d8e2f6952ee7645e221c42ba7c3f1793939a7596067ec1bc68396" },
    Keep { from: "doomgeneric/d_loop.c", to: "doom/d_loop.c", bytes: 16671, sha256: "e93f84c7794f2fce9da0c4c96173fa2eec9d48e6a0c31daade9c48a1d2af35ea" },
    Keep { from: "doomgeneric/d_loop.h", to: "doom/d_loop.h", bytes: 2365, sha256: "27cc9f6611fe6273b39e3e780d44f9f25bc61937e5f1f62711229568af311a8c" },
    Keep { from: "doomgeneric/d_main.c", to: "doom/d_main.c", bytes: 44883, sha256: "9cb01798f600879ce8142a1acdc00828cbea48e1d01371f03b166bffdc72c0ee" },
    Keep { from: "doomgeneric/d_main.h", to: "doom/d_main.h", bytes: 1015, sha256: "93d04e541a63430da34396b546b4561867b60fe72a21ee7cf382159c8dbac421" },
    Keep { from: "doomgeneric/d_mode.c", to: "doom/d_mode.c", bytes: 4865, sha256: "4db5e599badeb809b8f1bcdf0ce483dcd704cdbc1abdb3fbdba447cda150ea0d" },
    Keep { from: "doomgeneric/d_mode.h", to: "doom/d_mode.h", bytes: 2942, sha256: "809224d5ab46b4a29be0ea2d82f3cbbd9eea6e476a38441f3aa42bb0f99b08e4" },
    Keep { from: "doomgeneric/d_net.c", to: "doom/d_net.c", bytes: 6704, sha256: "dd6cbd6e3ebc379bcdfca2e02e2bf0a5a3bfe8c1945a920f8c8dacac9fc45890" },
    Keep { from: "doomgeneric/d_player.h", to: "doom/d_player.h", bytes: 4457, sha256: "349de25b7ba47532507556f8ac4f2f218891f36b1c4647defaee03ae4b85d6fc" },
    Keep { from: "doomgeneric/d_textur.h", to: "doom/d_textur.h", bytes: 928, sha256: "7500fb01de1fe637e05110a0aa189425979bddf0de327a31527434418f4d3efc" },
    Keep { from: "doomgeneric/d_think.h", to: "doom/d_think.h", bytes: 1481, sha256: "a500e07ef47fd0e1ab24d34f5df24e7a59c7ad1b657f67c9c977704b72cef08b" },
    Keep { from: "doomgeneric/d_ticcmd.h", to: "doom/d_ticcmd.h", bytes: 1544, sha256: "96bebe68c88b81a8fb35bfb6e8d74870d792107e1ceabc1c69069ed257aff851" },
    Keep { from: "doomgeneric/deh_main.h", to: "doom/deh_main.h", bytes: 1493, sha256: "de68692339596b59f8903e41ce88842d79bef3c92d3995f4d68f9056f90ecb9e" },
    Keep { from: "doomgeneric/deh_misc.h", to: "doom/deh_misc.h", bytes: 3071, sha256: "cf07e29421704d2491f53cc3e055e367c6aaecce2baa9759e9bcace21e657cb2" },
    Keep { from: "doomgeneric/deh_str.h", to: "doom/deh_str.h", bytes: 1195, sha256: "2387262e1fd9751575be33659145dfc900d7525587e267b3dab5c63c7daa5ac0" },
    Keep { from: "doomgeneric/doom.h", to: "doom/doom.h", bytes: 1738, sha256: "09f5e81390611c078348327489b08e4eda85306c83dc2c2a764406b2e0abf51d" },
    Keep { from: "doomgeneric/doomdata.h", to: "doom/doomdata.h", bytes: 4679, sha256: "a258ffd069833b0c81a132c4db74fd9121e1815267b309c2b3b5dc501971a3eb" },
    Keep { from: "doomgeneric/doomdef.c", to: "doom/doomdef.c", bytes: 803, sha256: "56e23ec1235d7478f64577ac349f9fad92a9fe6f77a0197630d6319f0838b02a" },
    Keep { from: "doomgeneric/doomdef.h", to: "doom/doomdef.h", bytes: 3114, sha256: "87eaafe4d3366fe89924c8649c45b91fe89b6780a17f655e90c8a1d77b318a77" },
    Keep { from: "doomgeneric/doomfeatures.h", to: "doom/doomfeatures.h", bytes: 1020, sha256: "01fd54d8060c650933f1d65c90fbd04355637b9ec66108e65cda0ef1bba4b929" },
    Keep { from: "doomgeneric/doomgeneric.c", to: "doom/doomgeneric.c", bytes: 390, sha256: "5d1cc5ea8b4daf4bb470f681b365f393d11fe077ecb36ea3230865a757472656" },
    Keep { from: "doomgeneric/doomgeneric.h", to: "doom/doomgeneric.h", bytes: 793, sha256: "d24861dd7aa75d283226724f710ae4226839898bc61b986a0602df7df19df148" },
    Keep { from: "doomgeneric/doomkeys.h", to: "doom/doomkeys.h", bytes: 2604, sha256: "9bacfdc85b2003913b8a571c0762226d07e5e303dbdaa6c64887a194a4279ba2" },
    Keep { from: "doomgeneric/doomstat.c", to: "doom/doomstat.c", bytes: 959, sha256: "fa9332d64d0278e8eaa1481e18175b5b06d012af9e3f56a72edf823401a473bc" },
    Keep { from: "doomgeneric/doomstat.h", to: "doom/doomstat.h", bytes: 6786, sha256: "a4c71dc810f4ac7b23a881a03566b4427356f7d59de37d5ddd38b43069c1f1a5" },
    Keep { from: "doomgeneric/doomtype.h", to: "doom/doomtype.h", bytes: 2544, sha256: "17467a61fe3bb8b3c40a9173430213deea2d69a4f2ddbf0c7fc89bd180008827" },
    Keep { from: "doomgeneric/dstrings.c", to: "doom/dstrings.c", bytes: 2303, sha256: "9804ceaafe342d4a12d0ce52a1f882f84e1d8d62482bd8edf914bec8f4d6fb84" },
    Keep { from: "doomgeneric/dstrings.h", to: "doom/dstrings.h", bytes: 948, sha256: "81e8527b9264b1271fe7ab0737a359d24fb1f4052994ac3aea9a98c7a3a651c7" },
    Keep { from: "doomgeneric/dummy.c", to: "doom/dummy.c", bytes: 1995, sha256: "fe10b9869685fd586d5641883405531b3694d4ff6f9e6b800c4d3a43710d65fd" },
    Keep { from: "doomgeneric/f_finale.c", to: "doom/f_finale.c", bytes: 14691, sha256: "ac5ea9aff7dc47a8621e014b9f65ff91ac5cf8cdc1dcdaa855bbd5e3acc00861" },
    Keep { from: "doomgeneric/f_finale.h", to: "doom/f_finale.h", bytes: 918, sha256: "8b1d4777e91f0f8be1c97475698701e0f17013d00c394cb6a2cfbb87f46b9d62" },
    Keep { from: "doomgeneric/f_wipe.c", to: "doom/f_wipe.c", bytes: 5076, sha256: "60969b7843a0eb0483f549abff2c4fcd20ccfeafc5ace8817836d1e895819fa0" },
    Keep { from: "doomgeneric/f_wipe.h", to: "doom/f_wipe.h", bytes: 1163, sha256: "e3e769803e2998aab83a6f98634654ce76f9c51aa6c7c551c4489deac5a2e041" },
    Keep { from: "doomgeneric/g_game.c", to: "doom/g_game.c", bytes: 53214, sha256: "88d13781006089e80f8d1e4c1bea9fe6934f1314c290a1f379fc8155c3268123" },
    Keep { from: "doomgeneric/g_game.h", to: "doom/g_game.h", bytes: 1916, sha256: "b808b2ca10e734e22bef8591b0124d9ff4a4ef808ac12a6170189b4ef96c0c90" },
    Keep { from: "doomgeneric/gusconf.h", to: "doom/gusconf.h", bytes: 800, sha256: "beec7b7cceabc46ecf932ed584c85cbb68acb6dff7776f4df8586b35054afa58" },
    Keep { from: "doomgeneric/hu_lib.c", to: "doom/hu_lib.c", bytes: 6244, sha256: "fa6fa03cab0645248aeb16400ced25f9b8d52cadb9bf988ec9a6e2dcac228018" },
    Keep { from: "doomgeneric/hu_lib.h", to: "doom/hu_lib.h", bytes: 3630, sha256: "6ce06bc30e0275d8c77ebbf1b70d581f93594ead3efd2a0d84dcc8d00394fea5" },
    Keep { from: "doomgeneric/hu_stuff.c", to: "doom/hu_stuff.c", bytes: 12234, sha256: "726c8fa2a23a4da67b87b312156d9b04472c9cf966f210031e6924d46865ce0f" },
    Keep { from: "doomgeneric/hu_stuff.h", to: "doom/hu_stuff.h", bytes: 1342, sha256: "b7c9078a1eac5ccfc0e70ca691490ece0dae6171c28e315f4ece12ccff60cee5" },
    Keep { from: "doomgeneric/i_cdmus.c", to: "doom/i_cdmus.c", bytes: 4282, sha256: "6573cbb850e268671c21f9278fd0f3a5bb3c4bd31801e2231c649a334145902a" },
    Keep { from: "doomgeneric/i_cdmus.h", to: "doom/i_cdmus.h", bytes: 1351, sha256: "e0024e794a5e507c476b011a23770726c940e211c7c9603452d3bd156db47c30" },
    Keep { from: "doomgeneric/i_endoom.c", to: "doom/i_endoom.c", bytes: 2097, sha256: "4fd3d8b7b8959512b6742cb922b3a637f137ab64dd00c1811f146a18db80b590" },
    Keep { from: "doomgeneric/i_endoom.h", to: "doom/i_endoom.h", bytes: 811, sha256: "54834211849d212b3aec65b03ff8a8906f0b1f1985d334188eaad8990af73e4c" },
    Keep { from: "doomgeneric/i_input.c", to: "doom/i_input.c", bytes: 7975, sha256: "c7cfc91d1edaefd66395f9293fbef5050ea2ad367fd9acc2afd4b4cbf213daa4" },
    Keep { from: "doomgeneric/i_joystick.c", to: "doom/i_joystick.c", bytes: 8015, sha256: "01f52e0e986dfe3114e97a388f977a01d74e9bd16a454a57df58d6ecaeb7d519" },
    Keep { from: "doomgeneric/i_joystick.h", to: "doom/i_joystick.h", bytes: 2511, sha256: "1980f8481b417416d048ad03968058f0ff0cce9700e10f1bafd8e965190a80d1" },
    Keep { from: "doomgeneric/i_scale.c", to: "doom/i_scale.c", bytes: 32663, sha256: "f9dad27d7cae5ca6a63c87570c359d40e35002fb499c7d8a2e58ebb291a93e92" },
    Keep { from: "doomgeneric/i_scale.h", to: "doom/i_scale.h", bytes: 1606, sha256: "23d43b89fea35c33ee14959643441dafe2ba478b66f0d9c25af170abf27d203f" },
    Keep { from: "doomgeneric/i_sound.c", to: "doom/i_sound.c", bytes: 8351, sha256: "715918bc37b927c9426f37c3565d58d0f9f7bb517791b88b673820cc768572ee" },
    Keep { from: "doomgeneric/i_sound.h", to: "doom/i_sound.h", bytes: 5614, sha256: "b0a63a0fb482e767aa7812f5f7db3c66883feda084a30ec59abda10abc314a5e" },
    Keep { from: "doomgeneric/i_swap.h", to: "doom/i_swap.h", bytes: 1820, sha256: "b8e773577839ccb51c0c9bf7232e9c174ec32e44c9573b1b38cc3337e2d0ad45" },
    Keep { from: "doomgeneric/i_system.c", to: "doom/i_system.c", bytes: 12664, sha256: "e547a31d179f5aeb3a377f4d4fe22d46d212e737d2dedde8b64b0d253a4d71f9" },
    Keep { from: "doomgeneric/i_system.h", to: "doom/i_system.h", bytes: 2140, sha256: "19084cfd6ffb7b69516cb62916c2a510ace67545f17dd6fb673be848f1d81d9e" },
    Keep { from: "doomgeneric/i_timer.c", to: "doom/i_timer.c", bytes: 1604, sha256: "cf59e0db8d0ce9cefeb22643141a8983cea3668f16bae2bf009c58979c3b5cb3" },
    Keep { from: "doomgeneric/i_timer.h", to: "doom/i_timer.h", bytes: 1038, sha256: "39449001def04dbd4984be2ea42ed48e3a4ba1e0b10fdb43e43d9e18b23f5081" },
    Keep { from: "doomgeneric/i_video.c", to: "doom/i_video.c", bytes: 11235, sha256: "8b9e1998f344f8af78d904a3e806817733fd0defcf6d54368b03f5c1f61a2eed" },
    Keep { from: "doomgeneric/i_video.h", to: "doom/i_video.h", bytes: 4803, sha256: "c3321fc943e421a9a889509b7a2a7ee5b6a848c8fee4ecaab746c66588c15a3d" },
    Keep { from: "doomgeneric/info.c", to: "doom/info.c", bytes: 139548, sha256: "43ed8b32ecd053e26b485a5443d8c11d4cf302377d5f62655d302964c90e2a0f" },
    Keep { from: "doomgeneric/info.h", to: "doom/info.h", bytes: 21728, sha256: "7ddb348b1f09c500446fb7e8c1e6dd3eec715f3a01e1fdee211ad3b0ec9516df" },
    Keep { from: "doomgeneric/m_argv.c", to: "doom/m_argv.c", bytes: 5223, sha256: "7fb6bde5640bcce06a92cfc96bfab58c39a172d55823c7f7fae62864996f8da6" },
    Keep { from: "doomgeneric/m_argv.h", to: "doom/m_argv.h", bytes: 1222, sha256: "f6e755d39f0edb6654625e11f328ae1cfa66f0898da57202eeebeda49bf8dd85" },
    Keep { from: "doomgeneric/m_bbox.c", to: "doom/m_bbox.c", bytes: 1108, sha256: "89b9abb5d7b112e1d49052a7936020503d884e7368729dbf079439d2efd2b900" },
    Keep { from: "doomgeneric/m_bbox.h", to: "doom/m_bbox.h", bytes: 961, sha256: "6290e396400b65a87c9c14eddb04c455c64381f116b9784870e5162cfb6d1857" },
    Keep { from: "doomgeneric/m_cheat.c", to: "doom/m_cheat.c", bytes: 2171, sha256: "4d68a9eaa79bfdf4efad2f92246aaff7d067c3b5aa4aecc06902ea98ae76cdb7" },
    Keep { from: "doomgeneric/m_cheat.h", to: "doom/m_cheat.h", bytes: 1277, sha256: "30ae07736b39f44b57edbcd9c4b861dcd6d81cede100aa7c3c6c85648af05ab2" },
    Keep { from: "doomgeneric/m_config.c", to: "doom/m_config.c", bytes: 46812, sha256: "9ee5543597647fdbd182019769d15e12ca54d930595885a520344adfd041c6d6" },
    Keep { from: "doomgeneric/m_config.h", to: "doom/m_config.h", bytes: 1213, sha256: "002679e7cd0e9a5aeeede9e57fa9052c8f18e34e8273bd47b34ef878c95714eb" },
    Keep { from: "doomgeneric/m_controls.c", to: "doom/m_controls.c", bytes: 12542, sha256: "8b85c59a9af0cf2d698e6ed1fa0796e4759ca94fa7bae94c853b775cb9584718" },
    Keep { from: "doomgeneric/m_controls.h", to: "doom/m_controls.h", bytes: 3925, sha256: "089b36825ad0f49957990f52fc10fd3c4b2404b7822bf3051b0a69b297ea8f7b" },
    Keep { from: "doomgeneric/m_fixed.c", to: "doom/m_fixed.c", bytes: 1142, sha256: "d5e13eb7d84416055a5981b8c9bf6495821795626114de40d9b1d4ca36bdc544" },
    Keep { from: "doomgeneric/m_fixed.h", to: "doom/m_fixed.h", bytes: 908, sha256: "0928f51e77d3d9dadcd8593821e5c3788a82144e3d74ad40414e87d82cd28c70" },
    Keep { from: "doomgeneric/m_menu.c", to: "doom/m_menu.c", bytes: 39303, sha256: "d8cf0f46692ce17301511bd5b4163d3985c309afd34265f9cedf17c13a08b9d2" },
    Keep { from: "doomgeneric/m_menu.h", to: "doom/m_menu.h", bytes: 1473, sha256: "bd8d8c0c6d3f759a5bada0b2a8210c4242b5a46fa0f85439eefa0c425ae31d23" },
    Keep { from: "doomgeneric/m_misc.c", to: "doom/m_misc.c", bytes: 10678, sha256: "fbd40d5fb82ed3a315841d7bb4a2fea60971993597fb9908ff7c69d576028cb3" },
    Keep { from: "doomgeneric/m_misc.h", to: "doom/m_misc.h", bytes: 1796, sha256: "5e35de7b90304c2ef4a88dd4f4f672addb9062ced9b6f98566d253177b8cab7c" },
    Keep { from: "doomgeneric/m_random.c", to: "doom/m_random.c", bytes: 2402, sha256: "5be0ddef0e84bd3e4789e4d6311d979263b8da43a32f30403b9e6d08bf070fea" },
    Keep { from: "doomgeneric/m_random.h", to: "doom/m_random.h", bytes: 909, sha256: "f9d5a08dc6dcab13c22c8bb20a65e70ff42b35623dee81c0d2f363b595c2b612" },
    Keep { from: "doomgeneric/memio.c", to: "doom/memio.c", bytes: 3617, sha256: "29619ef12147ce4f66ac54ef05c59c7e2c0e792d851f9d1921837c1dc7a165b1" },
    Keep { from: "doomgeneric/memio.h", to: "doom/memio.h", bytes: 1206, sha256: "3dff35a40b042f831d0b2505b6238c8ff96663ea527aae3614d23ee1870fcc92" },
    Keep { from: "doomgeneric/mus2mid.h", to: "doom/mus2mid.h", bytes: 167, sha256: "5c0543485225e396f77e1707b033d4e679e950b1af1f3ad8f05d4bc274ee75ac" },
    Keep { from: "doomgeneric/net_client.h", to: "doom/net_client.h", bytes: 1619, sha256: "76d382f79cd596e99cedfc1e8ea50d472aa3ed03cc5ac734ef3e5effc9a8dfff" },
    Keep { from: "doomgeneric/net_dedicated.h", to: "doom/net_dedicated.h", bytes: 704, sha256: "d842253025919a48f3c314f354450dd64ef31448912e6ed86ab2a75f33add287" },
    Keep { from: "doomgeneric/net_defs.h", to: "doom/net_defs.h", bytes: 5916, sha256: "a35b30a1e248fc28f190113794f9c11c3d991a196ad11a31cd2dd9717160247a" },
    Keep { from: "doomgeneric/net_gui.h", to: "doom/net_gui.h", bytes: 830, sha256: "da25eac4d1e2e6e18d05039f3830dda814e81cbad4bf1af4f8fd9c1d5e7297c3" },
    Keep { from: "doomgeneric/net_io.h", to: "doom/net_io.h", bytes: 1251, sha256: "50d07149d4603b20b4477443e88adac8ba72303a190253d7d1fede4901db64bf" },
    Keep { from: "doomgeneric/net_loop.h", to: "doom/net_loop.h", bytes: 821, sha256: "6b2517390fda6c5a71b4db7cb8de885de685960d8d469e750712712434c0a998" },
    Keep { from: "doomgeneric/net_packet.h", to: "doom/net_packet.h", bytes: 1527, sha256: "3386dce3169cb8a5ad37cccc010b64bf8a66eb36d847eb55d38c5554af10ffec" },
    Keep { from: "doomgeneric/net_query.h", to: "doom/net_query.h", bytes: 1515, sha256: "2c1a1be246d9b560683b2ef56719b6b6b3958f62819a033bb3a35dedf724d60f" },
    Keep { from: "doomgeneric/net_sdl.h", to: "doom/net_sdl.h", bytes: 742, sha256: "73c88c6ec8ffad3a30583a48e1ace2ff186d4b5bb08061776c0fd01ec81ca51a" },
    Keep { from: "doomgeneric/net_server.h", to: "doom/net_server.h", bytes: 1108, sha256: "33437f02e5ce1b5cb2cc3fef790512f75b3afb1b14a90bf92378738c54af3cef" },
    Keep { from: "doomgeneric/p_ceilng.c", to: "doom/p_ceilng.c", bytes: 6053, sha256: "20c58a9336c44f61d42f5b57bcab08f866889776083365850b34cc24a90f7017" },
    Keep { from: "doomgeneric/p_doors.c", to: "doom/p_doors.c", bytes: 16417, sha256: "429bc5c7446caf4c95bfc0ac8a6c4f9892f88e9f4615fa02103f180a3dbb4178" },
    Keep { from: "doomgeneric/p_enemy.c", to: "doom/p_enemy.c", bytes: 37354, sha256: "0043c0fa6107ddd001737e5da25d80c3d8c606769710d25a9fa064661cd6a7d3" },
    Keep { from: "doomgeneric/p_floor.c", to: "doom/p_floor.c", bytes: 11290, sha256: "8bc68d1f1bea828aa72ef09213addebd946483e07886ca7e51457c95e87a4ec8" },
    Keep { from: "doomgeneric/p_inter.c", to: "doom/p_inter.c", bytes: 18939, sha256: "2b8f7c1c38feefd296406438820ddb7fb13c09c2b3cb5a78ee030b7838f318ba" },
    Keep { from: "doomgeneric/p_inter.h", to: "doom/p_inter.h", bytes: 708, sha256: "83b5c68164306ca2b51675d9753e118ca854a2eadc56c2c9e5bdf4b9bdd83cb4" },
    Keep { from: "doomgeneric/p_lights.c", to: "doom/p_lights.c", bytes: 6512, sha256: "f0aebf42466c7995f7d84faac28bee1abd25cb83e76f668edb7a24b77311ed91" },
    Keep { from: "doomgeneric/p_local.h", to: "doom/p_local.h", bytes: 6408, sha256: "f9c74f3ec882f4f759653fd96bdece15c83b4bbafa9baf47a3d86ae56d72ce1d" },
    Keep { from: "doomgeneric/p_map.c", to: "doom/p_map.c", bytes: 31710, sha256: "7e39f8a82c7a7b3de34845a28ac14ea8252509ed6af2ba280f1ba5d9d050c3e4" },
    Keep { from: "doomgeneric/p_maputl.c", to: "doom/p_maputl.c", bytes: 20059, sha256: "46b752624ae6686fffab2493c121f42eaa29816eb9bb487689ca6dd79a9373bc" },
    Keep { from: "doomgeneric/p_mobj.c", to: "doom/p_mobj.c", bytes: 21322, sha256: "c80a7063eb28fe1461a2b40845d48405fddfeea06fbaf2fd30cec88e689f070d" },
    Keep { from: "doomgeneric/p_mobj.h", to: "doom/p_mobj.h", bytes: 8672, sha256: "b54b339339a442f52412a6828b3749306c7841550849299f1f7d272ea323e168" },
    Keep { from: "doomgeneric/p_plats.c", to: "doom/p_plats.c", bytes: 6375, sha256: "9fd0e8bfdeced8ef16a46694b3d3159cf110e499c3a4ef7ce68e9b8c12f7f2dc" },
    Keep { from: "doomgeneric/p_pspr.c", to: "doom/p_pspr.c", bytes: 16908, sha256: "0d8b0f8a285d6533e81bc4fbaeeb18a6ab4bb305d547e81ae3853a462e5c9a1e" },
    Keep { from: "doomgeneric/p_pspr.h", to: "doom/p_pspr.h", bytes: 1548, sha256: "eee47f10c539d4c27b4b72bddfe5d83b2f317c14aa3d92eb5f9d1f993838ef6d" },
    Keep { from: "doomgeneric/p_saveg.c", to: "doom/p_saveg.c", bytes: 36266, sha256: "00618c964bea7664fef9f6b15e76f5b6d8766d7bebb4e0e979f0c2e4d8289cb5" },
    Keep { from: "doomgeneric/p_saveg.h", to: "doom/p_saveg.h", bytes: 1569, sha256: "1fb50d63262bd64046c3553d4a2f3e5913878c792338d0ae77934ce59bcd1ee4" },
    Keep { from: "doomgeneric/p_setup.c", to: "doom/p_setup.c", bytes: 19002, sha256: "e6735d36ac04a58c77467776d96ab2fa0f8234fd00c522cf7484ed1510abe3c0" },
    Keep { from: "doomgeneric/p_setup.h", to: "doom/p_setup.h", bytes: 865, sha256: "b694051e5a6981cfd5800c4f6a64484553172c4587148b5354c5db082af46dc9" },
    Keep { from: "doomgeneric/p_sight.c", to: "doom/p_sight.c", bytes: 7178, sha256: "9435efa2f2d2755c64155b3a490ead7587838bc32a8d081d45b64c3d45836271" },
    Keep { from: "doomgeneric/p_spec.c", to: "doom/p_spec.c", bytes: 29251, sha256: "7b79a0d9972f0866ab0f1a0cdcbf377ca28851d1d2139e8651037808eaca276e" },
    Keep { from: "doomgeneric/p_spec.h", to: "doom/p_spec.h", bytes: 9364, sha256: "0215f64ac43834fd292bc32003f84e6b195f9fb0ab30d0b6cb7fad04fdf78109" },
    Keep { from: "doomgeneric/p_switch.c", to: "doom/p_switch.c", bytes: 13307, sha256: "e7621d563f384a2e228644612a59702c7cd0f5c7ed88fa8f69a09098c062c236" },
    Keep { from: "doomgeneric/p_telept.c", to: "doom/p_telept.c", bytes: 2868, sha256: "5d0eb86e2620c7f58d9ce106ba85ffe331e7fe0886f7e3955d7e321a452551ee" },
    Keep { from: "doomgeneric/p_tick.c", to: "doom/p_tick.c", bytes: 2799, sha256: "824179765df5de9c3c93a420b606cad7aa48ccbd393bd4ce22f7fba9d9d54a19" },
    Keep { from: "doomgeneric/p_tick.h", to: "doom/p_tick.h", bytes: 797, sha256: "558c6879d9fb461ae1bf0f3e42f492fcdf4c7291e1795f9a283add8f78149567" },
    Keep { from: "doomgeneric/p_user.c", to: "doom/p_user.c", bytes: 8151, sha256: "617d5ffdd3e67770415d26d370e24bde8afcf525d46510c63753de3c4e648fe2" },
    Keep { from: "doomgeneric/r_bsp.c", to: "doom/r_bsp.c", bytes: 11028, sha256: "08197bf2685b9c9467af4483517e4842396889504a7311506ff0c73b8f56f371" },
    Keep { from: "doomgeneric/r_bsp.h", to: "doom/r_bsp.h", bytes: 1369, sha256: "fc1adc08614fd463384c8af565e82bca4dd396316d18302cd2baa3a9250a303f" },
    Keep { from: "doomgeneric/r_data.c", to: "doom/r_data.c", bytes: 20090, sha256: "6806c473319095e24fd422a9156be7a19eea01c9f11ade5543782a25813c6e57" },
    Keep { from: "doomgeneric/r_data.h", to: "doom/r_data.h", bytes: 1260, sha256: "ad200859ba61da621a82182bac9835dcc79f03e35d1ac91064c791676e1954c4" },
    Keep { from: "doomgeneric/r_defs.h", to: "doom/r_defs.h", bytes: 8139, sha256: "ab5e553358945af3cc800eb05d5275eed5e678e7452b0d872b35d837c866d272" },
    Keep { from: "doomgeneric/r_draw.c", to: "doom/r_draw.c", bytes: 21724, sha256: "b13e6ac18a4e911875601c15ccc02b7f1958712f13e5edf85cf0134372623bce" },
    Keep { from: "doomgeneric/r_draw.h", to: "doom/r_draw.h", bytes: 2178, sha256: "9e8d667a90785e93b44547a94cd7c0f317e6fce199ab3d551f9346d6ef9465ee" },
    Keep { from: "doomgeneric/r_local.h", to: "doom/r_local.h", bytes: 1140, sha256: "f0b87b3534d532b37b58386519de6b1d06328a580fe2af62f3de340aea95faeb" },
    Keep { from: "doomgeneric/r_main.c", to: "doom/r_main.c", bytes: 15531, sha256: "f4b0a5f14ac46fe90f537553a9dce558d8cc2d0aa6aab6c53a8080598fda3ab9" },
    Keep { from: "doomgeneric/r_main.h", to: "doom/r_main.h", bytes: 2928, sha256: "bfafe1bb3bfab0de24e14f463ed1162fdb28ce4a78f30d7a514e86ebfba359a6" },
    Keep { from: "doomgeneric/r_plane.c", to: "doom/r_plane.c", bytes: 8160, sha256: "d16c45af2658781620254b4ce2bc6985980912a208b3cc7037986a142ee3c7a8" },
    Keep { from: "doomgeneric/r_plane.h", to: "doom/r_plane.h", bytes: 1431, sha256: "e792ae34f6440363cc1e7ed68a9d4880b8a1057fc5293aebfbcf4466c889d773" },
    Keep { from: "doomgeneric/r_segs.c", to: "doom/r_segs.c", bytes: 16890, sha256: "3c473ad15d4830bfe24be959cd3e2ce72e7a6bbb2a41bef93495a7f11ffd53b4" },
    Keep { from: "doomgeneric/r_segs.h", to: "doom/r_segs.h", bytes: 780, sha256: "4ed0b751b8a0db73b01cdd0f6ec24dafef61c1b403de2be83094cf84e93c26ae" },
    Keep { from: "doomgeneric/r_sky.c", to: "doom/r_sky.c", bytes: 1198, sha256: "5e7ea2b14c227ff7df297741e1be674e6a7865ce2eeab29c99ca12d1f3209ade" },
    Keep { from: "doomgeneric/r_sky.h", to: "doom/r_sky.h", bytes: 931, sha256: "cc53c08d1cc7ef042944a2f9d1924ebe27022b6ef172bee3e492d4cfd6e9240a" },
    Keep { from: "doomgeneric/r_state.h", to: "doom/r_state.h", bytes: 2348, sha256: "7ec040a913b05ec2c57ea33720fd600b702186701076faad5a4f135e65a562f6" },
    Keep { from: "doomgeneric/r_things.c", to: "doom/r_things.c", bytes: 20586, sha256: "584277754102354ebd9e9a9dfc804e99466adafff394444a272dd241e9c64024" },
    Keep { from: "doomgeneric/r_things.h", to: "doom/r_things.h", bytes: 1551, sha256: "664ba8ac66b160e321dce8a00a9d29eea4f39643f20f7711f6d439286e58ef60" },
    Keep { from: "doomgeneric/s_sound.c", to: "doom/s_sound.c", bytes: 14465, sha256: "b11d6990c9308edf686a1c77bcca1eded7e3b5c0e233a918cb195861ca412101" },
    Keep { from: "doomgeneric/s_sound.h", to: "doom/s_sound.h", bytes: 1909, sha256: "08c6e17c15a86927f86e9e68fd16b8b7c7b50f7a11abd313e9905703969cd4d1" },
    Keep { from: "doomgeneric/sha1.c", to: "doom/sha1.c", bytes: 8641, sha256: "1920c8f3c669f86678c250d997d2159c6d562f41414891e34544c331e45aef22" },
    Keep { from: "doomgeneric/sha1.h", to: "doom/sha1.h", bytes: 1174, sha256: "cda0dd9326ef9bcf5622076fab20e7e420c8dfe2d173a9859cc7cddefb78804b" },
    Keep { from: "doomgeneric/sounds.c", to: "doom/sounds.c", bytes: 5113, sha256: "28d358a0b76ec79208773ce4f9644ac2624a3950a518a3dad1decc8697d57d71" },
    Keep { from: "doomgeneric/sounds.h", to: "doom/sounds.h", bytes: 3803, sha256: "8a360f02c2bcb17c0fae62bf41317f91b785cd63ef7c9f35bc0cbbf708e9aa7a" },
    Keep { from: "doomgeneric/st_lib.c", to: "doom/st_lib.c", bytes: 4691, sha256: "f30d72536657f369a4fea6c03e60122ee7d385d1062615c7e5a917c8c086c63c" },
    Keep { from: "doomgeneric/st_lib.h", to: "doom/st_lib.h", bytes: 3320, sha256: "e0486603bc4154e87fb6c90cb4e669365f9c2e66a29e6c8296e8534d7e6223eb" },
    Keep { from: "doomgeneric/st_stuff.c", to: "doom/st_stuff.c", bytes: 29750, sha256: "da145ef218df8f46cbfa23854dc6a993f80c53b15a9bbee628cebd3fb78d024b" },
    Keep { from: "doomgeneric/st_stuff.h", to: "doom/st_stuff.h", bytes: 1957, sha256: "7315a6067fe21c6b0b88be1c2eaca4f2e3623c7c24f4949bc30bcf0d71216772" },
    Keep { from: "doomgeneric/statdump.c", to: "doom/statdump.c", bytes: 8416, sha256: "79ab0c3af80f05e01df1f80f37d4e3902387f9a69acf50118e763d3a46302761" },
    Keep { from: "doomgeneric/statdump.h", to: "doom/statdump.h", bytes: 679, sha256: "15b7f0891cc6848d4f17bbb75508cb0f7ab1eee56067e7a8e556d0d151bf849b" },
    Keep { from: "doomgeneric/tables.c", to: "doom/tables.c", bytes: 130242, sha256: "b20ffe17b3b32e6eb689199db0b314deea234091301f2c34dd82b3512b42c411" },
    Keep { from: "doomgeneric/tables.h", to: "doom/tables.h", bytes: 2423, sha256: "f37583bc957fb456c4229fb85a6cb57fece3d24c069df622c8c9690dd66d094a" },
    Keep { from: "doomgeneric/v_patch.h", to: "doom/v_patch.h", bytes: 1470, sha256: "e5300f9c7b7c762131569f0e9d906b30fabe1d90eec040ad5808561fe7ff7da0" },
    Keep { from: "doomgeneric/v_video.c", to: "doom/v_video.c", bytes: 20873, sha256: "19d99cc57a49fb83a7357332a2a2338970633b477a883ffe61f9ee876ac13fe6" },
    Keep { from: "doomgeneric/v_video.h", to: "doom/v_video.h", bytes: 2998, sha256: "42f84f7588c2c33d6b7ba4297dedc83927063a39fb40f391df7399c8b05ad0e5" },
    Keep { from: "doomgeneric/w_checksum.c", to: "doom/w_checksum.c", bytes: 2219, sha256: "95a62cd90d615d67cb4326aea446e516df68a2384827236c55b01e483afdc571" },
    Keep { from: "doomgeneric/w_checksum.h", to: "doom/w_checksum.h", bytes: 812, sha256: "3ce1d2fcaf078d69d38bf7ae11e85c195463cf7da728c0546296eb3959b8b16c" },
    Keep { from: "doomgeneric/w_file.c", to: "doom/w_file.c", bytes: 1886, sha256: "c1da5ccf0ac64cbb3023730ffb79569d424b443de2e6064d081b0dea9566b6e6" },
    Keep { from: "doomgeneric/w_file.h", to: "doom/w_file.h", bytes: 2031, sha256: "3d2fcf565ae7ed32f2000bf3517ae55d6311a2e71069c80d3aa54d6c267e2c3c" },
    Keep { from: "doomgeneric/w_file_stdc.c", to: "doom/w_file_stdc.c", bytes: 2148, sha256: "515a68720dc0f4b3d66be0fd7831e5dc15d57d2015a2ff60613d26d120332237" },
    Keep { from: "doomgeneric/w_main.c", to: "doom/w_main.c", bytes: 4607, sha256: "fc9768762a4f5d3ee487f08ac483ad7088e443c9291fd7d4696b5177b7770b10" },
    Keep { from: "doomgeneric/w_main.h", to: "doom/w_main.h", bytes: 743, sha256: "9c8fc5515ef36b90e1ffd66a7934b94ac581ba7119f675037992f4ba9a6c4628" },
    Keep { from: "doomgeneric/w_merge.h", to: "doom/w_merge.h", bytes: 1220, sha256: "e8c2ba4f7f357fe265acde37122e42d6d4fd73bf135efcdb5b9af3043409ac34" },
    Keep { from: "doomgeneric/w_wad.c", to: "doom/w_wad.c", bytes: 12774, sha256: "5e3a6c495c5fc839f816ab6fb856272cd887ee4e9db67fbd6e0d339dd7847027" },
    Keep { from: "doomgeneric/w_wad.h", to: "doom/w_wad.h", bytes: 1606, sha256: "5fb042b3db5b20a09f2dd70134e734c26dd42230925c1dc9068274885bcc7d5e" },
    Keep { from: "doomgeneric/wi_stuff.c", to: "doom/wi_stuff.c", bytes: 33830, sha256: "881c934b7100507ce4141c74255c03ba381434632e233a34f7b98ff1c7ed4e03" },
    Keep { from: "doomgeneric/wi_stuff.h", to: "doom/wi_stuff.h", bytes: 1164, sha256: "82013f95fa7d59e1006f4f080d8bfedf69919571f57714415cf3413e049f273b" },
    Keep { from: "doomgeneric/z_zone.c", to: "doom/z_zone.c", bytes: 10396, sha256: "37b296f9ad28decc44b5bca9d38c6ea81ddbd8ab977c1d06f03fc7360e316423" },
    Keep { from: "doomgeneric/z_zone.h", to: "doom/z_zone.h", bytes: 2096, sha256: "d392cb68bf7c05b5950db5556757fb9271e8be903674afa01ab4b6d9d767871d" },
    Keep { from: "LICENSE", to: "LICENSE", bytes: 18092, sha256: "8177f97513213526df2cf6184d8ff986c675afb514d4e68a404010521b880643" },
];

/// What DevCraft writes into the quake project, by path in it.
#[rustfmt::skip]
pub(crate) const QUAKE_FILES: &[(&str, &str)] = &[
    ("project.toml", r#"# GridCraft Quake: Quake in a panel beside the sheet, with the game's variables in cells.
# A demonstration of the C Development Kit in DevCraft, which set it up. Not part of GridCraft or DevCraft.
id = "gridcraft-quake"
name = "GridCraft Quake"
version = "0.4.0"
author = ""
description = "Quake, docked beside the sheet. Its health, ammunition and monsters, and the bytes of its memory, update in a sheet as it is played."

# The C Development Kit: GridCraft builds the C below as one WebAssembly program, and the
# panel runs it. It can reach nothing but what its page hands it.
kit = "c"

# What this project asks to be allowed.
#   read   to see whether the workbook already has a sheet named "Quake"
#   write  to make that sheet, and to write the game's variables and the bytes of its memory into it
permissions = ["read", "write"]

# What is built: quakegeneric's Quake (the folder quake), and the one file written for this
# project. Quake leaves a frame half way by a jump, on an error and at the end of each demo:
# setjmp says so, and the kit builds it with what a jump takes.
[c]
sources = ["quake", "src"]
include = ["quake", "src"]
optimize = "speed"
entry = "none"
setjmp = true

[panel]
entry = "panel/index.html"
"#),
    ("README.md", r#"# GridCraft Quake

Quake in a panel docked beside a GridCraft sheet, with the game's own variables (health,
ammunition, the level, what has been killed and found) written into cells as it plays, formulas
and charts that follow them, and a table of the game's memory itself: where a few of those
variables lie, the bytes there, and the numbers the sheet works out of them.

It is a demonstration of the C Development Kit in DevCraft, a fork of GridCraft
(https://github.com/arcanii/DevCraft). **It is not our Quake**: Quake is id Software's game, and
its name and its artwork are theirs. This project is not part of GridCraft or DevCraft.

## How it was made

DevCraft's C Development Kit set it up when Set Up was pressed in the kit's part of Developer ▸
Development. Every file below that DevCraft did not write was downloaded from the place named
and checked against the SHA-256 DevCraft holds for it; nothing that did not match was kept.

- **Quake's code** (`quake/`, 112 files, and `LICENSE`) is quakegeneric
  (https://github.com/erysdren/quakegeneric), id Software's WinQuake source with the interface
  of a port, at commit `13052102577c629650cf07a46151a4b6e1b19c3c`. Its own example ports are
  left out.
- **The game's data** (`panel/id1/pak0.pak`, and its licence `SLICNSE.TXT`) is the shareware
  version, 1.06, unmodified, out of the shareware release as the Internet Archive holds it
  (`msdos_Quake106_shareware.zip`). Nothing else of that release was kept.
- **What DevCraft wrote for it**: `project.toml`, `src/gridcraft_quake.c` (what quakegeneric
  asks of a port, the picture, the keys, the game's variables as the rows of a table, and where
  a few of them lie in memory), `panel/index.html` and `panel/quake.js` (the panel: it draws
  the game, hands it the keys, writes the variables to the sheet, and reads the bytes of the
  game's memory for it), this README and `.gitignore`.
- **What DevCraft changed of Quake's code**: two lines, each marked "GridCraft Quake". Quake
  prints the date and time it was built (`quake/host.c`, `quake/host_cmd.c`), and the kit's
  compiler refuses them, since a build must come out the same each time: both lines say the
  kit built it instead. Everything else is as quakegeneric has it.

Quake leaves a frame half way by a jump (`setjmp` and `longjmp`): on an error, and also, with
nothing wrong, each time a demo ends. In WebAssembly a jump has to be asked for: the recipe
says `setjmp = true`, and the kit builds it with what a jump takes.

## Using it

1. In Developer ▸ Development, name your compiler (zig) in the C Development Kit's part.
2. Review the project and trust it in its own part. Build it. Dock its panel.
3. "Link to a sheet named Quake" makes the sheet and starts the writing.

Click the picture to give it the keys: arrows move, Ctrl fires, Space jumps, Esc is the menu.
Click it again to give the game the mouse: moving it looks about, its buttons are Quake's. Esc
gives the mouse back (the browser's own way out), and so does Ctrl+Alt+M. When the game is left
through its own Quit, the page says so and offers to start it again.

Every change to the folder ends your trust in it until you look again: that is GridCraft's
rule for every project.

## Licence

- **What DevCraft wrote** (listed above) is DevCraft's, under the MIT licence or the Apache
  License, version 2.0, at your choice.
- **Quake's code** (`quake/`) is id Software's, as released under the GNU General Public
  License, version 2 or later: `LICENSE` is that licence. A program built from this project
  holds both, and is under that licence as a whole.
- **Quake's data** (`panel/id1/pak0.pak`) is id Software's, under their shareware licence,
  `SLICNSE.TXT`: among other things it forbids any commercial use of the game "or any portion
  thereof, such as a screen display or a screenshot". Keep it out of any repository
  (`.gitignore` does).
"#),
    (".gitignore", r#"# The game's data is id Software's, not for a repository: Set Up put it here.
panel/id1/pak0.pak
"#),
    ("src/gridcraft_quake.c", r#"/*
 * GridCraft Quake: the one file that carries Quake into a GridCraft project's panel.
 *
 * Written for DevCraft's C Development Kit, which writes it into the project when the person
 * sets Quake up, and under DevCraft's licence (MIT OR Apache-2.0). Quake itself is quakegeneric
 * (https://github.com/erysdren/quakegeneric, in quake/), id Software's released code with a
 * port's interface, under the GNU General Public License, version 2 or later: a program built
 * from this project holds both, and is under that licence as a whole.
 *
 * This file gives quakegeneric the functions a port writes, and gives the panel's page what it
 * needs: a step of the game, the picture, the keys, and the game's own variables as the rows
 * of a table.
 *
 * Quake leaves a frame half way by a jump (setjmp and longjmp), on an error and at the end of
 * each demo. The project's recipe says so (setjmp = true), and GridCraft's kit builds it with
 * what a jump takes in WebAssembly.
 *
 * Built as WebAssembly: it can do nothing but what the page that runs it hands it.
 */
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "quakedef.h"
#include "quakegeneric.h"

#define EXPORT(name) __attribute__((export_name(#name)))

#define WIDTH QUAKEGENERIC_RES_X
#define HEIGHT QUAKEGENERIC_RES_Y

/* ---- what quakegeneric asks of a port ---------------------------------------------------- */

#define KEYS 64
static unsigned short key_queue[KEYS];
static unsigned key_put, key_take;

/* The colours Quake draws with, three bytes each, and the picture as a page's canvas takes it. */
static uint8_t colours[768];
static uint8_t picture[WIDTH * HEIGHT * 4];
static int pictures;

void QG_Init(void)
{
}

void QG_Quit(void)
{
}

void QG_SetPalette(unsigned char palette[768])
{
    memcpy(colours, palette, sizeof colours);
}

/* Quake hands over a picture of one byte a dot: each is one of its 256 colours. */
void QG_DrawFrame(void *pixels)
{
    const uint8_t *dots = pixels;
    int i;

    for (i = 0; i < WIDTH * HEIGHT; i++) {
        const uint8_t *colour = &colours[dots[i] * 3];
        picture[i * 4 + 0] = colour[0];
        picture[i * 4 + 1] = colour[1];
        picture[i * 4 + 2] = colour[2];
        picture[i * 4 + 3] = 255;
    }
    pictures++;
}

int QG_GetKey(int *down, int *key)
{
    unsigned short held;

    if (key_take == key_put)
        return 0;
    held = key_queue[key_take];
    key_take = (key_take + 1) % KEYS;
    *down = held >> 8;
    *key = held & 0xff;
    return 1;
}

/* How far the mouse has moved since Quake last asked: the page says, while it holds the mouse. */
static int mouse_x, mouse_y;

void QG_GetMouseMove(int *x, int *y)
{
    *x = mouse_x;
    *y = mouse_y;
    mouse_x = 0;
    mouse_y = 0;
}

void QG_GetJoyAxes(float *axes)
{
    int i;

    for (i = 0; i < QUAKEGENERIC_JOY_MAX_AXES; i++)
        axes[i] = 0;
}

/* ---- what the page asks ------------------------------------------------------------------ */

static int started;

/* Starts the game on the data the page put at id1/pak0.pak. */
EXPORT(gq_start) void gq_start(void)
{
    static char *args[] = {"quake", NULL};

    QG_Create(1, args);
    started = 1;
}

/* A step of the game: `seconds` is how long it is since the one before. */
EXPORT(gq_tick) void gq_tick(double seconds)
{
    if (started)
        QG_Tick(seconds);
}

/* A key went down (1) or came up (0): `key` is one of Quake's own codes. */
EXPORT(gq_key) void gq_key(int down, int key)
{
    unsigned next = (key_put + 1) % KEYS;

    if (next == key_take)
        return;
    key_queue[key_put] = (unsigned short) (((down ? 1 : 0) << 8) | (key & 0xff));
    key_put = next;
}

/* The mouse moved, while the page holds it for the game: so many dots across and down. */
EXPORT(gq_mouse) void gq_mouse(int across, int down)
{
    mouse_x += across;
    mouse_y += down;
}

/*
 * The page took the mouse for the game (1) or gave it back (0). While the game has it, moving
 * it looks about, up and down too: that is Quake's own "+mlook", which a player holds a key for.
 */
EXPORT(gq_mouse_held) void gq_mouse_held(int held)
{
    mouse_x = 0;
    mouse_y = 0;
    if (started)
        Cbuf_AddText(held ? "+mlook\n" : "-mlook\n");
}

EXPORT(gq_width) int gq_width(void)
{
    return WIDTH;
}

EXPORT(gq_height) int gq_height(void)
{
    return HEIGHT;
}

/* The picture: red, green, blue, and all of it opaque. */
EXPORT(gq_picture) const uint8_t *gq_picture(void)
{
    return picture;
}

/* How many pictures Quake has drawn. */
EXPORT(gq_pictures) int gq_pictures(void)
{
    return pictures;
}

/* ---- the game's variables, as the rows of a table ---------------------------------------- */

EXPORT(gq_names) const char *gq_names(void)
{
    return "State\nLevel\nMap\nTime (s)\nHealth\nArmor\nWeapon\nAmmo\nShells\nNails\nRockets\nCells\n"
           "Monsters killed\nMonsters of\nSecrets\nSecrets of\nThings in view\nX\nY\nZ\nFacing (degrees)";
}

static const char *state(void)
{
    if (cls.state != ca_connected || cls.signon != SIGNONS)
        return key_dest == key_menu ? "Menu" : "Console";
    if (cl.intermission)
        return "Intermission";
    if (cls.demoplayback)
        return "Demo";
    if (key_dest == key_menu)
        return "Menu";
    return cl.paused ? "Paused" : "Playing";
}

static const char *weapon(int active)
{
    switch (active) {
    case IT_AXE: return "Axe";
    case IT_SHOTGUN: return "Shotgun";
    case IT_SUPER_SHOTGUN: return "Double-barrelled shotgun";
    case IT_NAILGUN: return "Nailgun";
    case IT_SUPER_NAILGUN: return "Perforator";
    case IT_GRENADE_LAUNCHER: return "Grenade launcher";
    case IT_ROCKET_LAUNCHER: return "Rocket launcher";
    case IT_LIGHTNING: return "Thunderbolt";
    default: return "";
    }
}

/* Text as one cell of a table written in JSON: nothing in it but letters a sheet can show. */
static int text(char *to, int room, const char *from)
{
    int used = 0;

    if (room < 8)
        return 0;
    to[used++] = '[';
    to[used++] = '"';
    for (; *from && used < room - 6; from++) {
        unsigned char c = (unsigned char) *from & 0x7f; /* Quake's own letters have a coloured twin above 127 */
        if (c == '"' || c == '\\')
            to[used++] = '\\';
        to[used++] = c < 32 ? ' ' : (char) c;
    }
    to[used++] = '"';
    to[used++] = ']';
    to[used] = 0;
    return used;
}

/* The values of the variables gq_names lists, in that order: a table one column wide, as JSON. */
EXPORT(gq_values) const char *gq_values(void)
{
    static char said[1024];
    int used = 0;
    int in_game = cls.state == ca_connected && cls.signon == SIGNONS;
    const float *at = in_game && cl.viewentity > 0 && cl.viewentity < MAX_EDICTS ? cl_entities[cl.viewentity].origin : NULL;
    float facing = cl.viewangles[1];

#define ROOM ((int) sizeof said - used)
#define COMMA said[used++] = ','
#define NUMBER(value) used += snprintf(said + used, ROOM, "[%d]", (int) (value))
    while (facing < 0)
        facing += 360;
    said[used++] = '[';
    used += text(said + used, ROOM, state());
    COMMA;
    used += text(said + used, ROOM, in_game ? cl.levelname : "");
    COMMA;
    used += text(said + used, ROOM, in_game && cl.worldmodel ? cl.worldmodel->name : "");
    COMMA;
    NUMBER(in_game ? cl.time : 0);
    COMMA;
    NUMBER(cl.stats[STAT_HEALTH]);
    COMMA;
    NUMBER(cl.stats[STAT_ARMOR]);
    COMMA;
    used += text(said + used, ROOM, in_game ? weapon(cl.stats[STAT_ACTIVEWEAPON]) : "");
    COMMA;
    NUMBER(cl.stats[STAT_AMMO]);
    COMMA;
    NUMBER(cl.stats[STAT_SHELLS]);
    COMMA;
    NUMBER(cl.stats[STAT_NAILS]);
    COMMA;
    NUMBER(cl.stats[STAT_ROCKETS]);
    COMMA;
    NUMBER(cl.stats[STAT_CELLS]);
    COMMA;
    NUMBER(cl.stats[STAT_MONSTERS]);
    COMMA;
    NUMBER(cl.stats[STAT_TOTALMONSTERS]);
    COMMA;
    NUMBER(cl.stats[STAT_SECRETS]);
    COMMA;
    NUMBER(cl.stats[STAT_TOTALSECRETS]);
    COMMA;
    NUMBER(in_game ? cl_numvisedicts : 0);
    COMMA;
    NUMBER(at ? at[0] : 0);
    COMMA;
    NUMBER(at ? at[1] : 0);
    COMMA;
    NUMBER(at ? at[2] : 0);
    COMMA;
    NUMBER(in_game ? facing : 0);
    said[used++] = ']';
    said[used] = 0;
#undef NUMBER
#undef COMMA
#undef ROOM
    return said;
}

/*
 * A few places in the game's memory: what each is, how its four bytes are read, and where it
 * is. The game says only where; the page reads what is there, and the sheet works it out.
 */
EXPORT(gq_memory) const char *gq_memory(void)
{
    static char said[640];
    static const struct {
        const char *name;
        int stat;
    } PLACES[] = {
        {"Health", STAT_HEALTH},   {"Armor", STAT_ARMOR},     {"Ammo", STAT_AMMO},
        {"Shells", STAT_SHELLS},   {"Nails", STAT_NAILS},     {"Rockets", STAT_ROCKETS},
        {"Cells", STAT_CELLS},     {"Monsters killed", STAT_MONSTERS},
        {"Secrets", STAT_SECRETS},
    };
    int used = 0;
    unsigned i;

    for (i = 0; i < sizeof PLACES / sizeof PLACES[0]; i++)
        used += snprintf(said + used, sizeof said - (size_t) used, "%s\twhole\t%u\n", PLACES[i].name,
                         (unsigned) (uintptr_t) &cl.stats[PLACES[i].stat]);
    return said;
}
"#),
    ("panel/index.html", r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Quake</title>
  <style>
    html, body { margin: 0; background: #16181c; color: #d8dbe0; font: 13px system-ui, sans-serif; }
    body { padding: 10px; }
    #screen {
      display: block; width: 100%; aspect-ratio: 4 / 3; background: #000;
      image-rendering: pixelated; outline: none; border: 2px solid #2a2d33; box-sizing: border-box;
    }
    #screen:focus { border-color: #8a5a2b; }
    p { margin: 8px 0 0; }
    .quiet { color: #9aa0a8; }
    button { font: inherit; padding: 5px 12px; margin-top: 8px; }
    details { margin-top: 10px; }
    pre { background: #0e0f12; padding: 8px; max-height: 12em; overflow: auto; font-size: 11px; white-space: pre-wrap; }
  </style>
</head>
<body>
  <canvas id="screen" tabindex="0" aria-label="Quake"></canvas>
  <p id="says" aria-live="polite"></p>
  <button id="again" hidden>Start Quake again</button>
  <button id="link" hidden>Link to a sheet named "Quake"</button>
  <p id="linked" class="quiet"></p>
  <details>
    <summary class="quiet">What the game printed</summary>
    <pre id="lines"></pre>
  </details>

  <!-- GridCraft serves these two beside this page: spreadsheet.js gives the page its `spreadsheet`
       object, and program.js the loader (CProgram) that runs the program built from this project's C. -->
  <script src="spreadsheet.js"></script>
  <script src="program.js"></script>
  <script src="quake.js"></script>
</body>
</html>
"#),
    ("panel/quake.js", r#"// GridCraft Quake: what the panel's page does.
//
// Written for DevCraft's C Development Kit, which writes it into the project when the person sets
// Quake up, and under DevCraft's licence (MIT OR Apache-2.0). The game it runs is quakegeneric's,
// under the GNU General Public License, version 2: see the project's README.md.
//
// The game is the project's C, built by GridCraft's C Development Kit as one WebAssembly
// program. GridCraft serves it beside this page, with the loader that runs it (program.js):
// the game can do only what that loader hands it, which is a clock, a place to print and
// files kept in memory. It never sees the workbook. This page draws the picture, hands the
// keys over, reads the game's variables out of it and writes them into a sheet through
// GridCraft's `spreadsheet` object, where each call is checked against what you granted.
"use strict";

const GridCraftQuake = (() => {
  // Quake's own numbers for the keys a keyboard's own names don't give.
  const KEYS = {
    ArrowUp: 128, ArrowDown: 129, ArrowLeft: 130, ArrowRight: 131,
    AltLeft: 132, AltRight: 132, ControlLeft: 133, ControlRight: 133, ShiftLeft: 134, ShiftRight: 134,
    Enter: 13, NumpadEnter: 13, Escape: 27, Tab: 9, Space: 32, Backspace: 127,
    F1: 135, F2: 136, F3: 137, F4: 138, F5: 139, F6: 140, F7: 141, F8: 142, F9: 143, F10: 144, F11: 145, F12: 146,
    Insert: 147, Delete: 148, PageDown: 149, PageUp: 150, Home: 151, End: 152, Pause: 255,
    Comma: 44, Period: 46, Slash: 47, Minus: 45, Equal: 61, Semicolon: 59, Quote: 39,
    BracketLeft: 91, BracketRight: 93, Backslash: 92, Backquote: 96,
  };

  /** Quake's number for a key, by where it is on the keyboard. 0 for one Quake has no use for. */
  function keyCode(code) {
    if (KEYS[code]) return KEYS[code];
    if (/^Key[A-Z]$/.test(code)) return code.charCodeAt(3) + 32;
    if (/^Digit[0-9]$/.test(code)) return code.charCodeAt(5);
    return 0;
  }

  return { keyCode };
})();

// --- the page ----------------------------------------------------------------------------------

if (typeof document !== "undefined") {
  const SHEET = "Quake";
  /** Where the game's data is, in this project's panel folder and as the game opens it. */
  const DATA = "id1/pak0.pak";
  /** How often the variables are written (milliseconds), and how many seconds the chart looks back. */
  const EVERY = 250;
  const KEPT = 60;
  /** Where the last minute is kept, out of the way of the table and the charts: its first column, and its headings. */
  const LOG = "N";
  const LOGGED = ["Second", "Health", "Armor", null, "Second", "Monsters killed", "Things in view"];

  /** Quake's numbers for the mouse's buttons, by the page's: left, middle, right. */
  const BUTTONS = [200, 202, 201];
  const PLAY = "Arrows move, Ctrl fires, Space jumps, Esc is the menu. Click the picture again to look about with the mouse.";
  const MOUSE = "The game has the mouse: move it to look, its buttons fire. Esc gives it back (so does Ctrl+Alt+M).";

  const screen = document.getElementById("screen");
  const again = document.getElementById("again");
  const says = document.getElementById("says");
  const link = document.getElementById("link");
  const linked = document.getElementById("linked");
  const lines = document.getElementById("lines");
  const printed = [];
  const tell = (text) => {
    says.textContent = text;
  };

  /** The row the table of the game's memory begins at, under the table of its variables. */
  const memoryTop = (names) => names.length + 4;
  /** An address as it is written: eight digits of sixteen. */
  const address = (at) => "0x" + at.toString(16).toUpperCase().padStart(8, "0");
  /** Four bytes as they lie in memory, lowest first: "64 00 00 00" is 100. */
  const lying = (bytes) => Array.from(bytes, (byte) => byte.toString(16).toUpperCase().padStart(2, "0")).join(" ");

  /**
   * The formula that works a whole number out of four bytes written so in the cell `c`. The
   * sheet does this, not the page: the bytes are turned round (the lowest lies first) and read
   * as one number, below nothing when its top bit is set.
   */
  function workedOut(c) {
    const digits = `MID(${c},10,2)&MID(${c},7,2)&MID(${c},4,2)&MID(${c},1,2)`;
    return `=IF(${c}="","",IF(HEX2DEC(${digits})>=2147483648,HEX2DEC(${digits})-4294967296,HEX2DEC(${digits})))`;
  }

  /**
   * Makes the sheet the variables go to: a table of their names, a few cells worked out from
   * them, the last minute second by second, and two charts over that; and under the table, a
   * few places of the game's memory: where each is, the bytes there, and the number the sheet
   * works out of them. One step of undo.
   */
  async function makeSheet(names, places) {
    const last = names.length + 1;
    const row = (name) => names.indexOf(name) + 2;
    const cell = (name) => `B${row(name)}`;
    const table = [["Variable", "Value"], ...names.map((name) => [name, null])];
    const top = memoryTop(names);
    const memory = [
      ["In the game's memory", "Address", "Bytes there", "Worked out in the sheet"],
      ...places.map((place, i) => [place.name, null, null, workedOut(`C${top + 1 + i}`)]),
    ];
    const worked = [
      ["Worked out in the sheet", null],
      ["Health", `=REPT("█",MAX(0,ROUND(${cell("Health")}/5,0)))`],
      ["Kills", `=TEXT(IF(${cell("Monsters of")}>0,${cell("Monsters killed")}/${cell("Monsters of")},0),"0%")`],
      ["Secrets", `=${cell("Secrets")}&" of "&${cell("Secrets of")}`],
      ["Ammo left", `=IF(${cell("Ammo")}<10,"LOW","OK")`],
    ];
    await spreadsheet.batch(
      [
        // The new sheet is the one being worked on: the widths that follow are its columns'.
        ["home.insertSheet", { name: SHEET }],
        ["home.columnWidth", { cols: "A:A", width: 140 }],
        ["home.columnWidth", { cols: "B:B", width: 150 }],
        ["home.columnWidth", { cols: "C:C", width: 90 }],
        ["home.columnWidth", { cols: "D:D", width: 160 }],
        ["range.setValues", { range: `${SHEET}!A1`, values: table }],
        ["range.setValues", { range: `${SHEET}!D1`, values: worked }],
        ["range.setValues", { range: `${SHEET}!${LOG}1`, values: [LOGGED] }],
        ["range.setValues", { range: `${SHEET}!A${top}`, values: memory }],
        ["home.bold", { range: `${SHEET}!A1:T1` }],
        ["home.bold", { range: `${SHEET}!A2:A${last}` }],
        ["home.bold", { range: `${SHEET}!A${top}:D${top}` }],
        ["home.bold", { range: `${SHEET}!A${top + 1}:A${top + places.length}` }],
      ],
      { label: "Make the Quake sheet" },
    );
    // Two charts over the last minute, each with the seconds along the bottom. A workbook can do
    // without them: if one can't be made, the table still fills.
    const charts = [
      { range: `${SHEET}!R1:T${KEPT + 1}`, title: "Monsters killed and things in view, the last minute", at: "F1" },
      { range: `${SHEET}!N1:P${KEPT + 1}`, title: "Health and armor, the last minute", at: "F17" },
    ];
    for (const chart of charts) {
      try {
        await spreadsheet.run("insert.chart", { ...chart, type: "line", width: 500, height: 300 });
      } catch (error) {
        await spreadsheet.log("A chart could not be made:", String(error));
      }
    }
  }

  async function page() {
    tell("Loading the game...");
    const data = await fetch(DATA);
    if (!data.ok) {
      tell(`There is no game data: put Quake's pak0.pak in this project's panel folder as ${DATA}, and trust the project again.`);
      return;
    }
    // The program GridCraft built from this project's C, run by GridCraft's loader.
    const program = await CProgram.load({
      files: { [DATA]: new Uint8Array(await data.arrayBuffer()) },
      print: (line) => {
        printed.push(line);
        lines.textContent = printed.slice(-200).join("\n");
      },
    });
    const quake = program.exports;
    quake.gq_start();
    if (program.ended()) {
      tell(program.ended());
      return;
    }
    const [width, height] = [quake.gq_width(), quake.gq_height()];

    // The game, as the rest of this page uses it.
    const game = {
      width,
      height,
      names: program.string(quake.gq_names()).split("\n"),
      ended: () => program.ended(),
      tick: (seconds) => quake.gq_tick(seconds),
      key: (down, code) => quake.gq_key(down ? 1 : 0, code),
      mouse: (across, down) => quake.gq_mouse(across, down),
      mouseHeld: (held) => quake.gq_mouse_held(held ? 1 : 0),
      pictures: () => (program.ended() ? 0 : quake.gq_pictures()),
      picture: () => (program.ended() ? null : program.bytes(quake.gq_picture(), width * height * 4)),
      values: () => (program.ended() ? [["Ended"]] : JSON.parse(program.string(quake.gq_values()))),
      // A few places in the game's memory: what each is and where it is. The game says where;
      // the page reads what is there.
      places: () =>
        program.ended()
          ? []
          : program
              .string(quake.gq_memory())
              .split("\n")
              .filter(Boolean)
              .map((line) => {
                const [name, kind, at] = line.split("\t");
                return { name, kind, at: Number(at) || 0 };
              }),
      // The four bytes at an address of the game's memory, read by this page.
      peek: (at) => program.bytes(at, 4),
    };
    document.title = "Quake";

    // The picture.
    screen.width = game.width;
    screen.height = game.height;
    const context = screen.getContext("2d");
    const image = context.createImageData(game.width, game.height);
    let drawn = -1;
    const draw = () => {
      const now = game.pictures();
      if (now === drawn) return;
      drawn = now;
      const picture = game.picture();
      if (picture) image.data.set(picture);
      context.putImageData(image, 0, 0);
    };

    // The keys, while the picture has them. When it loses them, every key is let go.
    const down = new Set();
    const key = (pressed) => (event) => {
      const code = GridCraftQuake.keyCode(event.code);
      if (!code) return;
      event.preventDefault();
      if (pressed === down.has(code)) return;
      pressed ? down.add(code) : down.delete(code);
      game.key(pressed, code);
    };
    screen.addEventListener("keydown", key(true));
    screen.addEventListener("keyup", key(false));
    // The mouse. A click on the picture gives it the keys; a click when it has them gives the
    // game the mouse as well, until Esc (the browser's own way out, which no page can take
    // away) or Ctrl+Alt+M. While the game has it, its buttons are Quake's.
    const held = () => document.pointerLockElement === screen;
    const letGo = () => {
      for (const code of down) game.key(false, code);
      down.clear();
    };
    screen.addEventListener("mousedown", (event) => {
      if (game.ended()) return;
      if (!held()) {
        if (document.activeElement === screen && screen.requestPointerLock) screen.requestPointerLock();
        return;
      }
      const code = BUTTONS[event.button];
      if (!code) return;
      event.preventDefault();
      down.add(code);
      game.key(true, code);
    });
    screen.addEventListener("mouseup", (event) => {
      const code = BUTTONS[event.button];
      if (!code || !down.has(code)) return;
      down.delete(code);
      game.key(false, code);
    });
    screen.addEventListener("mousemove", (event) => {
      if (held() && !game.ended()) game.mouse(event.movementX, event.movementY);
    });
    screen.addEventListener("contextmenu", (event) => event.preventDefault());
    screen.addEventListener("keydown", (event) => {
      if (held() && event.ctrlKey && event.altKey && event.code === "KeyM") document.exitPointerLock();
    });
    document.addEventListener("pointerlockchange", () => {
      if (game.ended()) return;
      game.mouseHeld(held());
      if (!held()) letGo();
      tell(held() ? MOUSE : document.activeElement === screen ? PLAY : "Click the picture to play.");
    });
    screen.addEventListener("blur", () => {
      letGo();
      if (!game.ended()) tell("Click the picture to play.");
    });
    screen.addEventListener("focus", () => {
      if (!game.ended()) tell(PLAY);
    });
    tell("Click the picture to play.");

    // The game was left (its own Quit), or stopped on an error: it runs no more, and the
    // page says so and offers to start it again, which loads the page anew.
    again.addEventListener("click", () => location.reload());
    const over = () => {
      if (held()) document.exitPointerLock();
      letGo();
      const why = game.ended();
      tell(/\(0\)/.test(why) ? "You left Quake. Its last picture is above." : `Quake stopped. ${why}`);
      again.hidden = false;
      // The picture is dimmed, so that it is plain nothing moves any more.
      context.fillStyle = "rgba(0, 0, 0, 0.6)";
      context.fillRect(0, 0, game.width, game.height);
    };

    // The sheet. Linked by the person's click, or by itself to a sheet that is there already.
    let to = null; // the sheet's name while linked
    let busy = false;
    let before = "";
    let beforeMemory = "";
    let history = [];
    let second = -1;
    const show = () => {
      link.hidden = !!to;
      linked.textContent = to
        ? `Writing the game's variables, and the bytes at a few places of its memory, to the sheet "${to}", four times a second. Each write is a step of undo.`
        : "";
    };
    const unlink = (why) => {
      to = null;
      show();
      tell(why);
    };
    link.addEventListener("click", async () => {
      link.disabled = true;
      try {
        const book = await spreadsheet.workbook();
        if (!book.sheets.some((sheet) => sheet.name === SHEET)) await makeSheet(game.names, game.places());
        to = SHEET;
        before = "";
        beforeMemory = "";
        show();
      } catch (error) {
        tell(`The sheet could not be linked: ${error.message || error}`);
      }
      link.disabled = false;
    });
    try {
      const book = await spreadsheet.workbook();
      if (book.sheets.some((sheet) => sheet.name === SHEET)) to = SHEET;
    } catch (error) {
      // Without leave to read, the person links it by hand.
    }
    show();

    async function write() {
      if (!to || busy) return;
      const values = game.values();
      const full = values.length === game.names.length;
      const steps = [];
      const now = JSON.stringify(values);
      if (now !== before) steps.push(["range.setValues", { range: `${to}!B2`, values }]);
      // The game's memory itself: where each place is, and the four bytes that lie there now.
      // The sheet's own formulas work the numbers out of them.
      const memory = game.places().map((place) => (place.at ? [address(place.at), lying(game.peek(place.at))] : [null, null]));
      const lies = JSON.stringify(memory);
      if (memory.length && lies !== beforeMemory) {
        steps.push(["range.setValues", { range: `${to}!B${memoryTop(game.names) + 1}`, values: memory }]);
      }
      // Once a second of the level's own clock, a line of the last minute.
      if (full) {
        const at = (name) => values[game.names.indexOf(name)][0];
        const clock = at("Time (s)");
        if (clock !== second) {
          if (clock < second) history = [];
          second = clock;
          history.push([clock, at("Health"), at("Armor"), null, clock, at("Monsters killed"), at("Things in view")]);
          history = history.slice(-KEPT);
          const rows = history.concat(Array.from({ length: KEPT - history.length }, () => LOGGED.map(() => null)));
          steps.push(["range.setValues", { range: `${to}!${LOG}2`, values: rows }]);
        }
      }
      if (!steps.length) return;
      busy = true;
      try {
        await spreadsheet.batch(steps, { label: "Quake" });
        before = now;
        beforeMemory = lies;
      } catch (error) {
        unlink(`Writing to the sheet stopped: ${error.message || error}`);
      }
      busy = false;
    }
    setInterval(write, EVERY);

    // The game: a step for the time that has gone by, and the picture after it. A page is
    // called for each picture the screen shows; a timer beside that keeps the game going where
    // a page that is not being looked at is called less often. A long gap is not made up for.
    let timer = 0;
    let last = performance.now();
    const tick = () => {
      if (game.ended()) {
        if (again.hidden) over();
        clearInterval(timer);
        return false;
      }
      const now = performance.now();
      const gone = Math.min(0.1, (now - last) / 1000);
      if (gone >= 1 / 72) {
        last = now;
        game.tick(gone);
        draw();
      }
      return true;
    };
    const frame = () => {
      if (tick()) requestAnimationFrame(frame);
    };
    requestAnimationFrame(frame);
    timer = setInterval(tick, 1000 / 30);
    spreadsheet.log("Quake started.").catch(() => {});
  }

  page().catch((error) => tell(`The game could not start: ${error.message || error}`));
}
"#),
];

/// What is kept of the quake project's code: 113 files, 1,051,844 bytes.
#[rustfmt::skip]
pub(crate) const QUAKE_CODE: &[Keep] = &[
    Keep { from: "source/adivtab.h", to: "quake/adivtab.h", bytes: 10524, sha256: "06a327cee06d95748f772f48424dc8b48949778050cb452d17a03e33c9100e6f" },
    Keep { from: "source/anorms.h", to: "quake/anorms.h", bytes: 6295, sha256: "1d21f2e9c1ff6cb704ade4f04206cbc1aaadb6930370bb7d54a63c469eb9ee65" },
    Keep { from: "source/bspfile.h", to: "quake/bspfile.h", bytes: 7099, sha256: "7765b458a744879b253020f5039d7ed950fe52b2a0945ebfdd2f1027f53db700" },
    Keep { from: "source/cd_null.c", to: "quake/cd_null.c", bytes: 1004, sha256: "eb41e859a476a2f49824d425ab55d0cff65ae91819efa51a72db344c4b5be5cf" },
    Keep { from: "source/cdaudio.h", to: "quake/cdaudio.h", bytes: 938, sha256: "8bb410e536d6ce0e077cf229bc5fb77cf7ae7501d2e71179ee4211b5cd7c0207" },
    Keep { from: "source/chase.c", to: "quake/chase.c", bytes: 2395, sha256: "468d930ea3b2f3cf2b6a33921b636ef04ac1a978045128d816881364471b86a5" },
    Keep { from: "source/cl_demo.c", to: "quake/cl_demo.c", bytes: 7316, sha256: "91e69fe8be5cc8c7c26fc483815dc6be0035d7a59a51b63946b4ea53fa1aa40a" },
    Keep { from: "source/cl_input.c", to: "quake/cl_input.c", bytes: 11071, sha256: "0abece7eb8863e53858b263c2552fc34209ab884b7f353602d5d41eebbe2fd99" },
    Keep { from: "source/cl_main.c", to: "quake/cl_main.c", bytes: 14837, sha256: "71675b9966bee4111dc570771d8242a1b272818a33c42af7541b5ddeb9752ed1" },
    Keep { from: "source/cl_parse.c", to: "quake/cl_parse.c", bytes: 20320, sha256: "6d61a517b3511739074920c4c01296f5a0327fb1d1458adc9737af0205cd5aa0" },
    Keep { from: "source/cl_tent.c", to: "quake/cl_tent.c", bytes: 7924, sha256: "1b1df7061d3332908afe9c7bf3ab4b509b2c4ef688ac2f37c12202ae3b50250e" },
    Keep { from: "source/client.h", to: "quake/client.h", bytes: 8656, sha256: "4bdf3a952ce2a0886816dfc552b031a39cc7b9b98ffec8ee600f2d21c4059394" },
    Keep { from: "source/cmd.c", to: "quake/cmd.c", bytes: 12134, sha256: "d61003668f8586aaa629c13ed8d979d692606230eb30c59f80d1c3024dd8edb9" },
    Keep { from: "source/cmd.h", to: "quake/cmd.h", bytes: 4336, sha256: "99ca99629e32c7711da8fe15d775257db3c2e82ed002527700fc7d87caf2d1a3" },
    Keep { from: "source/common.c", to: "quake/common.c", bytes: 34746, sha256: "c12057b1c7203b36eb160946e9e410b19cd7a90ea9a745e1f675d98cf31643eb" },
    Keep { from: "source/common.h", to: "quake/common.h", bytes: 5451, sha256: "58380f5ce4de4cbbc63605de4fb4924e149f1d6923727a7e499b3035b31863b9" },
    Keep { from: "source/console.c", to: "quake/console.c", bytes: 12083, sha256: "4fc680a883e0e092724a23711149be137b7095fba647c57c0951daa59a860052" },
    Keep { from: "source/console.h", to: "quake/console.h", bytes: 1478, sha256: "6a3165abff55e700b336231482d0f91807ecaa5b92f43be9619d3ad700ef36cd" },
    Keep { from: "source/crc.c", to: "quake/crc.c", bytes: 3439, sha256: "ace65596615c564a62cf998d925d9457cb20dbf412c26eed4f1aedf3d77dacd1" },
    Keep { from: "source/crc.h", to: "quake/crc.h", bytes: 894, sha256: "cc7b9d14b3d7cbaf5c59226243d9f882a3396440a9ed905ccee6d84256c55798" },
    Keep { from: "source/cvar.c", to: "quake/cvar.c", bytes: 4195, sha256: "cd381320c10e85a4a0a4b3ec5c254326a77172bf58c1dfb92354cce8000b39cd" },
    Keep { from: "source/cvar.h", to: "quake/cvar.h", bytes: 3388, sha256: "a7c534e9aba9e9f618f09a5c901b815e6a5ff4d582fdeb4046f71c17005d5efa" },
    Keep { from: "source/d_edge.c", to: "quake/d_edge.c", bytes: 7573, sha256: "7b0071fbb15b335b3250760eae3f301b7017ef4787b9ec0aa5119591c64089c9" },
    Keep { from: "source/d_fill.c", to: "quake/d_fill.c", bytes: 1904, sha256: "c9e57957f56cd95601e5b5db48f3d85ae79f4a831ce5b3883e4b02603b3a40fc" },
    Keep { from: "source/d_iface.h", to: "quake/d_iface.h", bytes: 6487, sha256: "85036544d61c013f514964e34d0045fdb3b75bcfd706d0135b0136515f3b00e8" },
    Keep { from: "source/d_ifacea.h", to: "quake/d_ifacea.h", bytes: 2828, sha256: "d6093ccdd43de7397c9eec38bcc6e87004a3cc1c5088e78d91786706022234d3" },
    Keep { from: "source/d_init.c", to: "quake/d_init.c", bytes: 3135, sha256: "953713481fcaa7e1a4b94867df12ad12ccf92f117bf90ba8979feaa429f30a1e" },
    Keep { from: "source/d_local.h", to: "quake/d_local.h", bytes: 2949, sha256: "f0c2276074891ecaf0aa83e3bab0283aacbc28dc05908a7e55f7523bee83fbef" },
    Keep { from: "source/d_modech.c", to: "quake/d_modech.c", bytes: 1991, sha256: "5e470f3ec68f39daa2ab35a8a01d964ad886a7fc21d8c42be0156984f8a78ecb" },
    Keep { from: "source/d_part.c", to: "quake/d_part.c", bytes: 3655, sha256: "827236cbcaa039652a9a25d0f2d7dd4abe5765a3cec239124b41076fb3d9a7ec" },
    Keep { from: "source/d_polyse.c", to: "quake/d_polyse.c", bytes: 23903, sha256: "7bd31cf589e5ea9b1b2a4e68f5f11a94334009dc9adb2eb09a7852163c03f40c" },
    Keep { from: "source/d_scan.c", to: "quake/d_scan.c", bytes: 10774, sha256: "2057698b6930ad2280ba040a9390e37bdb75fbd08a8cb5289fe8cf9080fe65da" },
    Keep { from: "source/d_sky.c", to: "quake/d_sky.c", bytes: 3187, sha256: "7267d7454e04b999e15c56e7b802c8721ec28daa07f247427d79cc169fe19b3e" },
    Keep { from: "source/d_sprite.c", to: "quake/d_sprite.c", bytes: 10082, sha256: "7159be593ff80ac29f52a2004b9b3d1a6bc5b052b5b7261791180f0239cb334a" },
    Keep { from: "source/d_surf.c", to: "quake/d_surf.c", bytes: 7179, sha256: "b7d4f8607bfffc61b4127a10a6dd279a0f6f9a651eeede40cebe153137583914" },
    Keep { from: "source/d_vars.c", to: "quake/d_vars.c", bytes: 1481, sha256: "365d11dfa2ec73cff7d65e9adb63a5af679f792e4a82b4a63458db378735255a" },
    Keep { from: "source/d_zpoint.c", to: "quake/d_zpoint.c", bytes: 1219, sha256: "40a258b1afd20ac280f56e61fd6f946bf8d864d57c3142a2e59e485170720848" },
    Keep { from: "source/draw.c", to: "quake/draw.c", bytes: 12958, sha256: "6d5dbbb1ed6da6b5e41a460b1f5ee2bb66aa5125f34848f5308c424bb068c9a4" },
    Keep { from: "source/draw.h", to: "quake/draw.h", bytes: 1489, sha256: "5a661fd88e0224ad59353dd9024f135089337ca7fcea0188159cc37442d3e674" },
    Keep { from: "source/host.c", to: "quake/host.c", bytes: 18433, sha256: "96dc6b4b2be2efed6f93f3d344f2dcdead46a5d5575ba97e3fcc38d975e0938e" },
    Keep { from: "source/host_cmd.c", to: "quake/host_cmd.c", bytes: 33378, sha256: "51d821b2f48c23c9acab6cc513cf662458a96ec2428cb34641894079c09fe861" },
    Keep { from: "source/in_null.c", to: "quake/in_null.c", bytes: 9318, sha256: "eec222d31601d1172f46bb693f059164e7271ff9f91571f434c2543dec506556" },
    Keep { from: "source/input.h", to: "quake/input.h", bytes: 1097, sha256: "21f3c6ff84db0edd35ecfc446891d6e0003ffe34d487c94e498a9bf58da5a39e" },
    Keep { from: "source/keys.c", to: "quake/keys.c", bytes: 14162, sha256: "eb7582d1a5777fc8ffb02a4039b3e390e471575111c4969ed73fbbd67f9ca647" },
    Keep { from: "source/keys.h", to: "quake/keys.h", bytes: 1176, sha256: "5596150585d8614139f2b7187902e2f61dfe5a00358413adc5406374e86c1017" },
    Keep { from: "source/mathlib.c", to: "quake/mathlib.c", bytes: 12102, sha256: "8b909842358cac82824802278c7d23b3d39f8feccf8621c615b63438d6537c8f" },
    Keep { from: "source/mathlib.h", to: "quake/mathlib.h", bytes: 2742, sha256: "d738ca0d4a2c0179b552854481afb324004d95e3fe578ce9fd7c2eed0a5afc87" },
    Keep { from: "source/menu.c", to: "quake/menu.c", bytes: 64500, sha256: "5e33fc7aeaeff4124d6c90a9906d29031737ae92fff4e3edcc1cdb5f21f17a6f" },
    Keep { from: "source/menu.h", to: "quake/menu.h", bytes: 1055, sha256: "404ce3e8d39bc91a04754faff7866cbf48c7416612a1dd19fdea4d894f04074d" },
    Keep { from: "source/model.c", to: "quake/model.c", bytes: 40175, sha256: "e7a9cc2b2af8d3062b8a26e0c3a535649165381c0047a60551e0bfb2df8d1b08" },
    Keep { from: "source/model.h", to: "quake/model.h", bytes: 7819, sha256: "61048ae2f29de24f09618b6b1a0552d078a0b1799a7b6caa684dddd8d72186a3" },
    Keep { from: "source/modelgen.h", to: "quake/modelgen.h", bytes: 3012, sha256: "1af78ac12387e348df2da0ddcc85c8ccbe3870da159bb86f74a60d69cca04fda" },
    Keep { from: "source/net.h", to: "quake/net.h", bytes: 9323, sha256: "37e9efcf383df3afe8589bcfca1055ac5110745471bdce41e96bcae7136d2289" },
    Keep { from: "source/net_loop.c", to: "quake/net_loop.c", bytes: 5254, sha256: "174bb5c02a8cc07b239cbd7376a3253d91229d44b5acdd81c68f9c6f019889b8" },
    Keep { from: "source/net_loop.h", to: "quake/net_loop.h", bytes: 1271, sha256: "b10db09189ec85b36be6b782022b641363678167e8a1cb0b7d6bac98b9090f45" },
    Keep { from: "source/net_main.c", to: "quake/net_main.c", bytes: 19723, sha256: "c2dc539c93765e50a2de49b1bfd6fa32cea5dea1e99f1a9026df0599b9a7383c" },
    Keep { from: "source/net_none.c", to: "quake/net_none.c", bytes: 1191, sha256: "2c19f3b601c0fd8e3bde4ac50f51ac204713272f11e9fe3b068c8218bdc29bfb" },
    Keep { from: "source/net_vcr.c", to: "quake/net_vcr.c", bytes: 3522, sha256: "68928690ebdfcdd6289f3855d5abe04de4690834e7832c6b977be0234e6e758d" },
    Keep { from: "source/net_vcr.h", to: "quake/net_vcr.h", bytes: 1290, sha256: "7ca130f0ab94e56db4b41108559f8742a27441a574b2e1cc7954b448ca98eac7" },
    Keep { from: "source/nonintel.c", to: "quake/nonintel.c", bytes: 1196, sha256: "52e50ffe3659e1fbfc43d4e2fa7b57b758ecb6018f07b26a6458533e7d5f251a" },
    Keep { from: "source/pr_cmds.c", to: "quake/pr_cmds.c", bytes: 30333, sha256: "878d7644dba2f2f80e27ca4622306691785fec2dac8025044ede74b2e60db645" },
    Keep { from: "source/pr_comp.h", to: "quake/pr_comp.h", bytes: 3067, sha256: "30c8f32cb84bdb6ab31b79873390526e4a7bcc56ee14ea2b6b98dc274f5639ff" },
    Keep { from: "source/pr_edict.c", to: "quake/pr_edict.c", bytes: 22020, sha256: "97da745b0f498446c84b2b8fbf19385b412d1b58290c38c87e03e7bb334729e8" },
    Keep { from: "source/pr_exec.c", to: "quake/pr_exec.c", bytes: 12300, sha256: "6cfc54e30fc99e63b6c77e705419a7fe23020b959d79396134628952c4d28b21" },
    Keep { from: "source/progdefs.h", to: "quake/progdefs.h", bytes: 3185, sha256: "16622583aa637bc12e998b59302e7653669c77742a1e7b2318221e0485da8fa0" },
    Keep { from: "source/progs.h", to: "quake/progs.h", bytes: 3821, sha256: "2ba22bcae0bf4915970876e902064448c4191653968b3a2a2c5bdfade8005596" },
    Keep { from: "source/protocol.h", to: "quake/protocol.h", bytes: 4717, sha256: "000665c156ca5da011d87f5b941ed7331cc1726e46589e20995953cb372acae3" },
    Keep { from: "source/quakedef.h", to: "quake/quakedef.h", bytes: 7838, sha256: "b34d5db366b63c19f78fe3be22a9398526d515d0d9551b5643e9e292488b2517" },
    Keep { from: "source/quakegeneric.c", to: "quake/quakegeneric.c", bytes: 1105, sha256: "4a5ddd8101df58aa10d776808222fb7c430de4510c376e0c4e5c6aba8db444ad" },
    Keep { from: "source/quakegeneric.h", to: "quake/quakegeneric.h", bytes: 1559, sha256: "11bb53d63b94023d3c6970035ee06e465b491654242906f8955640462dfcdf54" },
    Keep { from: "source/quakekeys.h", to: "quake/quakekeys.h", bytes: 2699, sha256: "b8c17830d6d388980164c0d0557c0dc2494d23cd9cc9326f1e70ae9c230e464c" },
    Keep { from: "source/r_aclip.c", to: "quake/r_aclip.c", bytes: 8349, sha256: "0cdfbffcf208f28254fa2c30ba6554622de373414999f3fbf7a159377d14730b" },
    Keep { from: "source/r_alias.c", to: "quake/r_alias.c", bytes: 18552, sha256: "b0ce281038b4ce97d52095283ae1702f2e38ff7c01a7865ef96f6f3b2a9ed7a6" },
    Keep { from: "source/r_bsp.c", to: "quake/r_bsp.c", bytes: 14865, sha256: "f0c527a136c8e8e1ff373f0dc2e1980a5a2694c83e12c9e92be39f36fa3489a9" },
    Keep { from: "source/r_draw.c", to: "quake/r_draw.c", bytes: 20045, sha256: "3c1e36548656907c188be0f3e2d8175df919029cb5752e2620aa767a2d51de2f" },
    Keep { from: "source/r_edge.c", to: "quake/r_edge.c", bytes: 15668, sha256: "097ad38e85fb8677b4ab147c83a2b2854b63ea53426e58e36a16223fd2f352a1" },
    Keep { from: "source/r_efrag.c", to: "quake/r_efrag.c", bytes: 5039, sha256: "7922ef83807289740a4d957d81fbba08813e8aa29828122e996ed42a38427613" },
    Keep { from: "source/r_light.c", to: "quake/r_light.c", bytes: 5320, sha256: "f71db4a482b7c7f8ad37b02e12b2cb5eba856d026425de43b69c686dacf452d2" },
    Keep { from: "source/r_local.h", to: "quake/r_local.h", bytes: 8182, sha256: "67ac515132ec6ab4484a54e480cb8493a1adebc2e15981c183b91bdfdca62268" },
    Keep { from: "source/r_main.c", to: "quake/r_main.c", bytes: 24468, sha256: "ee94a451516d63f0cc964a1456116842ec95b2a8cb866e14bf4f62f66da8469a" },
    Keep { from: "source/r_misc.c", to: "quake/r_misc.c", bytes: 9588, sha256: "a69e002cebe72259530e38d836935ae952314063e42379e4c1f3900d79421661" },
    Keep { from: "source/r_part.c", to: "quake/r_part.c", bytes: 13253, sha256: "d201d20e6235f0397265644ec336a79c60ac764f1ae3c56342d936a81e7ab423" },
    Keep { from: "source/r_shared.h", to: "quake/r_shared.h", bytes: 4889, sha256: "43f2394879fcd368967dfe243ac1c916799d826a590cf26f2d4f68d0a3dfc61f" },
    Keep { from: "source/r_sky.c", to: "quake/r_sky.c", bytes: 3737, sha256: "1b2a68a0c6fa32c4748d3b90552a67432eb85dbe6caef1a9092c2319883f8138" },
    Keep { from: "source/r_sprite.c", to: "quake/r_sprite.c", bytes: 10776, sha256: "f16660ed5d69398b3972f0f75aec9974508aa8997df7f09d65233ae23f1fe625" },
    Keep { from: "source/r_surf.c", to: "quake/r_surf.c", bytes: 11947, sha256: "c7a6df6ca89b19d18e588eb813f748ce5f61d1a710d6095c596548462bc8c830" },
    Keep { from: "source/r_vars.c", to: "quake/r_vars.c", bytes: 1174, sha256: "1f0e8ccbd6d0f97a5343b004d040bb60d65b0932ed11947c0e58325c407f76e3" },
    Keep { from: "source/render.h", to: "quake/render.h", bytes: 4802, sha256: "d8a8ad4a9e8f314091eeca10dc04942e70ed598e2a644d9f0e54085b1d6a043b" },
    Keep { from: "source/sbar.c", to: "quake/sbar.c", bytes: 29282, sha256: "dbe1b3393b33a3d86d25d96ba30a0b192cd935af11ded8844fd7bd0aa06b73a4" },
    Keep { from: "source/sbar.h", to: "quake/sbar.h", bytes: 1264, sha256: "76eb2b6a3cc8f49812b86d758f7790701ab724b3d85a3649c8654f575d32d321" },
    Keep { from: "source/screen.c", to: "quake/screen.c", bytes: 18983, sha256: "92d102119e83f62691d49094243598ef9c373bf1e422ce752a094bd736308a8f" },
    Keep { from: "source/screen.h", to: "quake/screen.h", bytes: 1611, sha256: "5b889200a07e3288487b63244a9d04b04d700f0fb13868a0a4821e848bc89997" },
    Keep { from: "source/server.h", to: "quake/server.h", bytes: 6422, sha256: "ddf9041336d52b78527f1e44b49209e39e2be60473ea768167a22ca322320073" },
    Keep { from: "source/snd_null.c", to: "quake/snd_null.c", bytes: 1718, sha256: "3a41baecc2c08ae9478ce18edcf59f582e4d6f05cc280c0b42c458cb7437be02" },
    Keep { from: "source/sound.h", to: "quake/sound.h", bytes: 4842, sha256: "c3d18c65f381b7afdececb6cbed96c14a405516c18e0a0457830c7ba7259ec2b" },
    Keep { from: "source/spritegn.h", to: "quake/spritegn.h", bytes: 2808, sha256: "f3f707d9c4ccb8963e3aa4c014afa46ea7efc2f7724e768cc41838c18fde52ab" },
    Keep { from: "source/sv_main.c", to: "quake/sv_main.c", bytes: 26833, sha256: "7f078154cbcfa7cee6c715910dc517ffe21e83eac82619ecfa4a164738928c72" },
    Keep { from: "source/sv_move.c", to: "quake/sv_move.c", bytes: 9314, sha256: "dce60d060bb9ede188a8d257765304f22ef3ee0bb406b77a9512d3f04d1fece1" },
    Keep { from: "source/sv_phys.c", to: "quake/sv_phys.c", bytes: 26779, sha256: "d547d6afb612088af5ab46968ee2590a47620667ee6825c1777f1b6ce0dc6355" },
    Keep { from: "source/sv_user.c", to: "quake/sv_user.c", bytes: 12936, sha256: "6c7290b3d869938167cb52de931b999f56303cb08f926a572a881e9a39345bfd" },
    Keep { from: "source/sys.h", to: "quake/sys.h", bytes: 1939, sha256: "1dbd5e878eb5e350010e6955f5e5b7208de887f35216c91ae74f4afb751bb4ae" },
    Keep { from: "source/sys_null.c", to: "quake/sys_null.c", bytes: 3835, sha256: "b43b9446ccf3ab3cf1e67dff15044a42401832dacf73a877e075f12315e90a7a" },
    Keep { from: "source/vid.h", to: "quake/vid.h", bytes: 2672, sha256: "db6920fff853fdc0ee08b28f50c13cf929273283cbbe1a77fb690a467cfa19fb" },
    Keep { from: "source/vid_null.c", to: "quake/vid_null.c", bytes: 2232, sha256: "0ab2d64ab1a9210fc57bbb5251607f43a63fb298141a40f6cb567295de9980d2" },
    Keep { from: "source/view.c", to: "quake/view.c", bytes: 21502, sha256: "e492810287a2eeda58b7375400214b01a703538129aeae752d3c0f0ea3c8eaa7" },
    Keep { from: "source/view.h", to: "quake/view.h", bytes: 1032, sha256: "5534d00f1491479f715969fec20b91e32e41a3af53dbb80ab6208c364b07c803" },
    Keep { from: "source/wad.c", to: "quake/wad.c", bytes: 3417, sha256: "29447665d1253e1fe73907ba8d3f0c26e825902b05a7c0207829e2e313f9ef29" },
    Keep { from: "source/wad.h", to: "quake/wad.h", bytes: 1737, sha256: "2791cfc7619069afcfc7f82098b8d2bcbd93216619535bb962a881a528487c4d" },
    Keep { from: "source/world.c", to: "quake/world.c", bytes: 19548, sha256: "f50fd2aaec220de8382fd72e7ab8cb1360986f730a5905d565d0f76f116ce89a" },
    Keep { from: "source/world.h", to: "quake/world.h", bytes: 2688, sha256: "15272f34c7032fbc3e558587336edc28f15dca2041e388b3041185081970110c" },
    Keep { from: "source/zone.c", to: "quake/zone.c", bytes: 18822, sha256: "67cb8b207c7b2e3b128d4f79f2b5efad7b9bfe95d45da15704974eef42a4032b" },
    Keep { from: "source/zone.h", to: "quake/zone.h", bytes: 3182, sha256: "aec01740b4937228223d3487d8f5fc5e30216055738375c7b605f882fbd6e938" },
    Keep { from: "LICENSE", to: "LICENSE", bytes: 18092, sha256: "8177f97513213526df2cf6184d8ff986c675afb514d4e68a404010521b880643" },
];

/// What DevCraft writes into the glquake project, by path in it.
#[rustfmt::skip]
pub(crate) const GLQUAKE_FILES: &[(&str, &str)] = &[
    ("project.toml", r#"# GridCraft GLQuake: GLQuake in a panel beside the sheet, drawn with OpenGL, with the game's variables in cells.
# A demonstration of the C Development Kit in DevCraft, which set it up. Not part of GridCraft or DevCraft.
id = "gridcraft-glquake"
name = "GridCraft GLQuake"
version = "0.4.0"
author = ""
description = "GLQuake, docked beside the sheet and drawn with OpenGL. Its health, ammunition and monsters, and the bytes of its memory, update in a sheet as it is played."

# The C Development Kit: GridCraft builds the C below as one WebAssembly program, and the
# panel runs it, drawing on a canvas with the OpenGL ES the kit hands it. It can reach nothing
# but what its page hands it.
kit = "c"

# What this project asks to be allowed.
#   read     to see whether the workbook already has a sheet named "Quake"
#   write    to make that sheet, and to write the game's variables and the bytes of its memory into it
#   network  to play another GridCraft over the network: GridCraft carries its packets, only to
#            the addresses below, and only once you turn its network on (Developer > Development)
permissions = ["read", "write", "network"]

# What is built: id Software's GLQuake (the folder quake) and the file written for this project
# (the folder src). It draws with OpenGL 1 (<GL/gl.h>), which the kit supplies and builds in.
# GLQUAKE is what id Software's code is told to be built as GLQuake by. Quake leaves a frame half
# way by a jump, on an error and at the end of each demo: setjmp says so, and the kit builds it
# with what a jump takes.
[c]
sources = ["quake", "src"]
include = ["quake", "src"]
define = ["GLQUAKE"]
optimize = "speed"
entry = "none"
setjmp = true

[panel]
entry = "panel/index.html"

# Where it may play. Each is the computer of another GridCraft and Quake's port: as written, the
# other GridCraft is on this computer. To play one on your network, put its number here (like
# "192.168.1.20:26000"), trust the project again and turn its network on again.
# listen: the port it hosts a game on, for those computers. Without it, it can only join one.
[network]
allow = ["127.0.0.1:26000"]
listen = 26000
"#),
    ("README.md", r#"# GridCraft GLQuake

GLQuake, id Software's Quake as it draws with OpenGL, in a panel docked beside a GridCraft sheet:
drawn by the graphics card, at the size the panel is, with the game's own variables (health,
ammunition, the level, what has been killed and found) written into cells as it plays, and a
table of the bytes of its memory that the sheet works numbers out of. And it plays another
GridCraft over the network: one hosts a deathmatch, the other joins it.

It is a demonstration of the C Development Kit in DevCraft, a fork of GridCraft
(https://github.com/arcanii/DevCraft). **It is not our Quake**: Quake is id Software's game, and
its name and its artwork are theirs. This project is not part of GridCraft or DevCraft.

## How it was made

DevCraft's C Development Kit set it up when Set Up was pressed in the kit's part of Developer ▸
Development. Every file below that DevCraft did not write was downloaded from the place named
and checked against the SHA-256 DevCraft holds for it; nothing that did not match was kept.

- **GLQuake's code** (`quake/`, 88 files: the 47 C files GLQuake is built from, the 40 headers
  they read, and `progdefs.q1`; and `LICENSE`, which the release calls `gnu.txt`) is id
  Software's Quake source release (https://github.com/id-Software/Quake), at commit
  `bf4ac424ce754894ac8f1dae6a3981954bc9852d`, its `WinQuake` folder. The parts that talk to a
  machine are id Software's "null" ones; the machine itself is the project's (`src/`).
- **The game's data** (`panel/id1/pak0.pak`, and its licence `SLICNSE.TXT`) is the shareware
  version, 1.06, unmodified, out of the shareware release as the Internet Archive holds it
  (`msdos_Quake106_shareware.zip`). Nothing else of that release was kept.
- **What DevCraft wrote for it**:
  - `project.toml`: what the project is (`kit = "c"`), what it asks to be allowed (`read`,
    `write`, `network`), where it may play (`[network]`), and its recipe, which tells GLQuake's
    code it is GLQuake (`define = ["GLQUAKE"]`);
  - `src/gridcraft_glquake.c`: what a port of GLQuake writes (files, the clock, errors, the
    picture's size and colours, the keys and the mouse), and what the panel asks of the game: a
    step, the keys, the mouse, the game's variables as the rows of a table, and where a few of
    them lie in memory;
  - `src/net_gridcraft.c`: Quake's network, carried by GridCraft: the "LAN driver" Quake's own
    network part (`quake/net_dgrm.c`) speaks through, written on the kit's network calls; and
    `connect`, which asks the server a frame before joining;
  - `src/gridcraft_glquake.h`: the functions GLQuake's files call before anything declares
    them;
  - `panel/index.html`, `panel/glquake.js`: the panel. It hands the game a canvas to draw on, the
    keys and the mouse, writes the variables to the sheet, and reads the bytes of the game's
    memory for it;
  - this README and `.gitignore`.

None of what DevCraft wrote is id Software's code. Where GLQuake needs something done as its
own machine parts did it (its files, its picture's colours, the state OpenGL starts in, the
mouse), DevCraft's C says what, in its own way.

## How it draws

The kit hands a program OpenGL ES 3, which is what a page has (WebGL 2), on the canvas the page
names. GLQuake was written for OpenGL 1: it draws a shape a corner at a time (`glBegin`,
`glVertex`, `glEnd`), keeps stacks of matrices, and colours what it draws by a fixed recipe
(`glTexEnv`, `glAlphaFunc`). **The kit supplies that too**: `<GL/gl.h>`, which GLQuake includes,
is the kit's, and the kit builds its own OpenGL 1, made of OpenGL ES 3, into a program that
includes it. Nothing of it is in this project. It does this:

- the corners of each shape are turned into triangles as they come, each put through the
  modelview matrix there, with its place in the texture and its colour;
- the triangles are gathered and drawn many at a time: whenever something that changes how they
  are drawn changes (a texture, blending, the projection), what was gathered is drawn first.
  GLQuake draws its world a texture at a time, so a picture is a few dozen draws;
- one shader does the fixed recipe: the projection, the texture environment for each kind of
  texture, and the alpha test;
- OpenGL 1 lets a program name its textures itself, OpenGL ES 3 names them: a table holds both.

What it does not have: lighting, fog, arrays of corners, display lists, more than one texture
at a time.

## What it took for the kit to build it

The kit lets a project name its files and a few settings, and never the compiler's command line.

1. **GLQuake's code is told it is GLQuake** by the name `GLQUAKE`: the recipe's `define`.
2. **Quake leaves a frame half way by a jump** (`setjmp` and `longjmp`), on an error and at the
   end of each demo: the recipe says `setjmp = true`.
3. **Quake prints the date and time it was built**, in `quake/host.c` and `quake/host_cmd.c`,
   which the kit's compiler refuses: both lines say "built by the C Development Kit" instead.
4. **Two variables are defined in two files each** (`skytexturenum` in `gl_rsurf.c` and
   `gl_warp.c`, `sb_lines` in `sbar.c` and `gl_screen.c`). The compilers of 1996 made one of
   each; today's refuse. In `gl_warp.c` and `gl_screen.c` each is now `extern`.
5. **GLQuake's files call functions nothing has declared**, which the C of 1996 allowed and the
   kit's compiler refuses. `src/gridcraft_glquake.h` declares them by their types, and one line
   at the end of `quake/quakedef.h` reads it.

6. **Quake calls each of its "poll procedures" with an argument none of them takes**
   (`quake/net_main.c`). The x86 of 1996 let that pass; WebAssembly stops the program for it
   ("function signature mismatch"), the moment a game is joined. They are called as they are
   written.
7. **Quake's connect waits two and a half seconds for the server's answer** (`quake/net_dgrm.c`),
   and a page can take in nothing while it waits: it waits no time, and `connect`
   (`quake/host_cmd.c` names the project's in place of Quake's own) asks the server a frame
   before, so that the answer is already there.

**These eight places are all of id Software's code that DevCraft changes**, each marked
"GridCraft GLQuake", in lines DevCraft wrote. The rest of what GLQuake needs of a machine is in
`src/`:

- **The status bar is drawn on every picture.** GLQuake drew it again only on the two or three
  pictures a card of 1997 swapped between; a page's canvas begins each picture empty.
- **The "z trick" is off.** GLQuake left the depth buffer as it was and drew each picture into the
  other half of it; a canvas begins each picture afresh.

## Playing another GridCraft

GridCraft carries the game's packets, only to the computers `project.toml` names, and only once
you turn the project's network on (Developer ▸ Development, the project's part). As it comes,
it names this computer (`127.0.0.1:26000`): two GridCrafts on the one computer, each with this
project, can play each other. To play a GridCraft on another computer of your network, put that
computer's number in `allow` (like `"192.168.1.20:26000"`, one line each), trust the project
again and turn its network on again; Windows may ask whether GridCraft may use the network.

- **Host a game** starts a deathmatch on E1M1 here (`maxplayers 4`, `deathmatch 1`), on the
  port `listen` names, which is Quake's own, 26000. GridCraft takes in the players of the
  computers the project names, and no others.
- **Join** joins the game hosted at the number in the box. The console's `connect 192.168.1.20`
  does the same, and so does the menu's Multiplayer ▸ Join a Game. It takes a computer's number,
  not a name, and the screen stops for a second and a half as it joins (Quake looks for games on
  the network first, which here finds none).

## Using it

1. In Developer ▸ Development, name your compiler (zig) in the C Development Kit's part.
2. Review the project and trust it in its own part. Build it. Dock its panel.
3. "Link to a sheet named Quake" makes the sheet and starts the writing.
4. To play another GridCraft: turn the project's network on, in its part of Developer ▸
   Development, then Host a game, or Join one.

Click the picture to give it the keys: arrows move, Ctrl fires, Space jumps, Esc is the menu.
Click it again to give the game the mouse: moving it looks about, its buttons are Quake's. Esc
gives the mouse back (the browser's own way out), and so does Ctrl+Alt+M.

Every change to the folder ends your trust in it until you look again: that is GridCraft's
rule for every project.

## Licence

- **What DevCraft wrote** (listed above) is DevCraft's, under the MIT licence or the Apache
  License, version 2.0, at your choice.
- **GLQuake's code** (`quake/`) is id Software's, as released under the GNU General Public
  License, version 2 or later: `LICENSE` is that licence. A program built from this project
  holds both, and is under that licence as a whole.
- **Quake's data** (`panel/id1/pak0.pak`) is id Software's, under their shareware licence,
  `SLICNSE.TXT`: among other things it forbids any commercial use of the game "or any portion
  thereof, such as a screen display or a screenshot". Keep it out of any repository
  (`.gitignore` does).
"#),
    (".gitignore", r#"# The game's data is id Software's, not for a repository: Set Up put it here.
panel/id1/pak0.pak
"#),
    ("src/gridcraft_glquake.c", r#"/*
 * GridCraft GLQuake: the C that carries GLQuake into a GridCraft project's panel.
 *
 * Written for DevCraft's C Development Kit, which writes it into the project when the person
 * sets GLQuake up, and under DevCraft's licence (MIT OR Apache-2.0). GLQuake itself (quake/) is
 * id Software's, under the GNU General Public License, version 2 or later: a program built from
 * this project holds both, and is under that licence as a whole.
 *
 * This file is what a port of GLQuake writes: the system (files, the clock, errors), the video
 * (the size of the picture, its colours, the start and end of each picture) and the input (keys
 * and the mouse); and what the panel's page asks of the game: a step, the keys, the mouse, and
 * the game's own variables as the rows of a table. None of it is id Software's code: where
 * GLQuake needs something done as its own machine parts did it, this says what, in its own way.
 *
 * GLQuake draws with OpenGL 1, which the kit supplies (<GL/gl.h>, made of the OpenGL ES 3 its
 * loader hands a program), on the canvas the page names. Quake leaves a frame half way by a
 * jump: the project's recipe says setjmp, and the kit builds it with what a jump takes.
 *
 * Built as WebAssembly: it can do nothing but what the page that runs it hands it.
 */
#include <errno.h>
#include <math.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "quakedef.h"

#define EXPORT(name) __attribute__((export_name(#name)))

/* This is never a dedicated server: it draws, and a person plays it. */
qboolean isDedicated;

/* ---- the system: files ------------------------------------------------------------------- */

/*
 * Quake names an open file by a small number it is handed. The page's files are kept in memory
 * by the kit's loader, and read and written as the C library's streams; this table ties each
 * number to its stream. Number 0 is never handed out.
 */
enum { OPEN_FILES = 16 };
static FILE *open_files[OPEN_FILES];

/* Gives a stream a number; one too many open at once is an error Quake can't go on from. */
static int number_for(FILE *stream)
{
    int n;

    for (n = 1; n < OPEN_FILES; n++) {
        if (open_files[n] == NULL) {
            open_files[n] = stream;
            return n;
        }
    }
    fclose(stream);
    Sys_Error("GridCraft GLQuake: more than %d files open at once", OPEN_FILES - 1);
    return -1;
}

/* The stream a number stands for, or NULL for a number that stands for none. */
static FILE *stream_for(int handle)
{
    return handle > 0 && handle < OPEN_FILES ? open_files[handle] : NULL;
}

/* Opens a file to read from its start: its number goes in *handle, and its length is returned. -1 for none. */
int Sys_FileOpenRead(char *path, int *handle)
{
    FILE *stream = fopen(path, "rb");
    long length = -1;

    *handle = -1;
    if (stream == NULL)
        return -1;
    if (fseek(stream, 0, SEEK_END) == 0)
        length = ftell(stream);
    if (length < 0 || fseek(stream, 0, SEEK_SET) != 0) {
        fclose(stream);
        return -1;
    }
    *handle = number_for(stream);
    return (int) length;
}

/* Opens a file to write, emptied: a file that can't be is an error Quake can't go on from. */
int Sys_FileOpenWrite(char *path)
{
    FILE *stream = fopen(path, "wb");

    if (stream == NULL)
        Sys_Error("GridCraft GLQuake: %s could not be written (%s)", path, strerror(errno));
    return number_for(stream);
}

void Sys_FileClose(int handle)
{
    FILE *stream = stream_for(handle);

    if (stream != NULL) {
        fclose(stream);
        open_files[handle] = NULL;
    }
}

void Sys_FileSeek(int handle, int position)
{
    FILE *stream = stream_for(handle);

    if (stream != NULL)
        fseek(stream, position, SEEK_SET);
}

int Sys_FileRead(int handle, void *into, int count)
{
    FILE *stream = stream_for(handle);

    return stream != NULL && count > 0 ? (int) fread(into, 1, (size_t) count, stream) : 0;
}

int Sys_FileWrite(int handle, void *from, int count)
{
    FILE *stream = stream_for(handle);

    return stream != NULL && count > 0 ? (int) fwrite(from, 1, (size_t) count, stream) : 0;
}

/* Quake asks only whether a file is there: any time at all says it is, and -1 that it is not. */
int Sys_FileTime(char *path)
{
    FILE *stream = fopen(path, "rb");
    int there = stream != NULL;

    if (there)
        fclose(stream);
    return there ? 1 : -1;
}

void Sys_mkdir(char *path)
{
    (void) path; /* the page's files are kept in memory, and a file may be made in any folder */
}

/* ---- the system: the rest ---------------------------------------------------------------- */

void Sys_MakeCodeWriteable(unsigned long startaddr, unsigned long length)
{
    (void) startaddr;
    (void) length;
}

/* An error the game can't go on from: said, and the program ends (the page says so). */
void Sys_Error(char *error, ...)
{
    char why[1024];
    va_list rest;

    va_start(rest, error);
    vsnprintf(why, sizeof why, error, rest);
    va_end(rest);
    printf("Quake stopped on an error: %s\n", why);
    exit(1);
}

/* What Quake prints for its machine, as opposed to its console: the page's place to print. */
void Sys_Printf(char *fmt, ...)
{
    char line[2048];
    va_list rest;

    va_start(rest, fmt);
    vsnprintf(line, sizeof line, fmt, rest);
    va_end(rest);
    fputs(line, stdout);
}

/* Seconds, by the page's clock, since the game started. */
double Sys_FloatTime(void)
{
    static double first = -1;
    struct timespec now;
    double seconds;

    clock_gettime(CLOCK_MONOTONIC, &now);
    seconds = (double) now.tv_sec + (double) now.tv_nsec / 1e9;
    if (first < 0)
        first = seconds;
    return seconds - first;
}

/* A page never sleeps, and WebAssembly has one way of doing sums: these ask nothing of it. */
void Sys_HighFPPrecision(void) {}
void Sys_LowFPPrecision(void) {}
void Sys_Sleep(void) {}

/* Nothing is typed at a terminal: the console is the game's own. */
char *Sys_ConsoleInput(void)
{
    return NULL;
}

/* The game's own Quit: the program ends with nothing wrong, and the page offers to start it again. */
void Sys_Quit(void)
{
    exit(EXIT_SUCCESS);
}

/* The keys the page handed over since the game last asked, each down (1) or up (0). */
#define KEYS 64
static unsigned short key_queue[KEYS];
static unsigned key_put, key_take;

void Sys_SendKeyEvents(void)
{
    while (key_take != key_put) {
        unsigned short held = key_queue[key_take];
        key_take = (key_take + 1) % KEYS;
        Key_Event(held & 0xff, held >> 8);
    }
}

/* ---- the video --------------------------------------------------------------------------- */

/*
 * What GLQuake's drawing reads of its video part: the palette as four bytes a colour (and as
 * two, and the way back from fifteen bits to eight, for cards this is not), the number the next
 * texture is given, the filter a texture starts with, the depth range a picture is drawn in,
 * what OpenGL says it is, and what the card can do (one texture at a time, no 8-bit textures).
 */
unsigned d_8to24table[256];
unsigned short d_8to16table[256];
unsigned char d_15to8table[65536];
int texture_extension_number = 1;
int texture_mode = GL_LINEAR;
float gldepthmin, gldepthmax;
const char *gl_vendor, *gl_renderer, *gl_version, *gl_extensions;
qboolean gl_mtexable = false, is8bit = false, isPermedia = false;

/*
 * GLQuake's "z trick" leaves the depth buffer as it was and draws each picture into the other
 * half of it. A page's canvas starts each picture afresh, so here it is off from the start.
 */
cvar_t gl_ztrick = {"gl_ztrick", "0"};

/* The size of the picture in dots: the page's canvas. */
static int scr_width = 640, scr_height = 480;
static int pictures;

/*
 * The 256 colours as GLQuake's textures take them: four bytes each, red, green, blue and how
 * opaque, in the order they lie in memory. Colour 255 is the one GLQuake leaves see-through.
 */
void VID_SetPalette(unsigned char *palette)
{
    int colour;

    for (colour = 0; colour < 256; colour++) {
        const unsigned char *rgb = palette + 3 * colour;
        unsigned char *bytes = (unsigned char *) &d_8to24table[colour];

        bytes[0] = rgb[0];
        bytes[1] = rgb[1];
        bytes[2] = rgb[2];
        bytes[3] = colour == 255 ? 0 : 255;
    }
}

/* A flash of colour (a hit, a pick-up) is drawn over the picture: the palette stays as it is. */
void VID_ShiftPalette(unsigned char *palette)
{
    (void) palette;
}

void VID_Shutdown(void)
{
}

qboolean VID_Is8bit(void)
{
    return false;
}

/* The little picture a disc's turning was shown with, drawn straight to the screen: not here. */
void D_BeginDirectRect(int x, int y, byte *pbitmap, int width, int height)
{
    (void) x, (void) y, (void) pbitmap, (void) width, (void) height;
}

void D_EndDirectRect(int x, int y, int width, int height)
{
    (void) x, (void) y, (void) width, (void) height;
}

/*
 * GLQuake brightens its colours on any card but 3Dfx's, which a page's canvas is not: each of
 * the 768 bytes, taken as a share of 256 counting from one, is raised to the power 0.7, then
 * made a byte again, to the nearest.
 */
static void brighten(unsigned char *palette)
{
    int n;

    for (n = 0; n < 768; n++) {
        float raised = (float) pow((palette[n] + 1) / 256.0, (double) 0.7f);
        float level = raised * 255 + 0.5f;

        palette[n] = (unsigned char) (level >= 255 ? 255 : level <= 0 ? 0 : level);
    }
}

/*
 * How GLQuake expects to find OpenGL when it begins: textures on, the faces it does not draw
 * being its front ones (its triangles go round the other way), the alpha test passing a texel
 * more than two thirds opaque, one colour a triangle, the nearest texel when a texture is drawn
 * larger or smaller, textures repeating, blending by the source's opacity, and a texture's own
 * colours drawn as they are.
 */
static void begin_drawing(void)
{
    static const GLenum ON[] = {GL_TEXTURE_2D, GL_ALPHA_TEST};
    static const struct {
        GLenum what;
        GLenum how;
    } TEXTURES[] = {
        {GL_TEXTURE_MIN_FILTER, GL_NEAREST},
        {GL_TEXTURE_MAG_FILTER, GL_NEAREST},
        {GL_TEXTURE_WRAP_S, GL_REPEAT},
        {GL_TEXTURE_WRAP_T, GL_REPEAT},
    };
    unsigned i;

    gl_vendor = (const char *) glGetString(GL_VENDOR);
    gl_renderer = (const char *) glGetString(GL_RENDERER);
    gl_version = (const char *) glGetString(GL_VERSION);
    gl_extensions = (const char *) glGetString(GL_EXTENSIONS);
    Con_Printf("OpenGL: %s, %s, %s\n", gl_vendor, gl_renderer, gl_version);

    for (i = 0; i < sizeof ON / sizeof ON[0]; i++)
        glEnable(ON[i]);
    for (i = 0; i < sizeof TEXTURES / sizeof TEXTURES[0]; i++)
        glTexParameterf(GL_TEXTURE_2D, TEXTURES[i].what, (GLfloat) TEXTURES[i].how);
    glAlphaFunc(GL_GREATER, 0.666f);
    glCullFace(GL_FRONT);
    glShadeModel(GL_FLAT);
    glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);
    glTexEnvf(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_REPLACE);
    glClearColor(0, 0, 0, 0);
}

/*
 * The console and the menus are laid out on 640 by 480 dots, or on the canvas where it is
 * smaller (across, to a multiple of eight), and never on less than 320 by 200.
 */
static void lay_out(void)
{
    int across = scr_width < 640 ? scr_width & ~7 : 640;
    int down = scr_height < 480 ? scr_height : 480;

    vid.conwidth = vid.width = across < 320 ? 320 : across;
    vid.conheight = vid.height = down < 200 ? 200 : down;
    vid.aspect = ((float) vid.height / (float) vid.width) * (4.0f / 3.0f);
    vid.recalc_refdef = 1;
}

void VID_Init(unsigned char *palette)
{
    Cvar_RegisterVariable(&gl_ztrick);
    vid.colormap = host_colormap;
    vid.numpages = 2;
    lay_out();
    begin_drawing();
    brighten(palette);
    VID_SetPalette(palette);
    Con_SafePrintf("Video mode %dx%d initialized.\n", scr_width, scr_height);
}

/*
 * A picture begins. GLQuake draws the status bar again only on the two or three pictures a card
 * of 1997 swapped between; a page's canvas begins each picture empty, so it is asked for on each.
 */
void GL_BeginRendering(int *x, int *y, int *width, int *height)
{
    Sbar_Changed();
    *x = *y = 0;
    *width = scr_width;
    *height = scr_height;
}

/* A picture is done: what is gathered is drawn, and the page shows it when the step returns. */
void GL_EndRendering(void)
{
    glFlush();
    pictures++;
}

/* ---- the input --------------------------------------------------------------------------- */

/* How far the mouse has moved since Quake last asked: the page says, while it holds the mouse. */
static int mouse_x, mouse_y;

/* The page hands the keys and the mouse over as they come: there is nothing to start, ask or stop. */
void IN_Commands(void) {}
void IN_Init(void) {}
void IN_Shutdown(void) {}

/*
 * The mouse, as Quake's players know it. Across, it turns the player; or, with the strafe key
 * held (or with lookstrafe set while looking about), it steps them sideways. Up and down, while
 * looking about and not strafing, it tilts the view, to no more than 80 degrees down and 70 up;
 * otherwise it walks them forward and back, or, strafing with noclip's angle hack, up and down.
 */
void IN_Move(usercmd_t *cmd)
{
    float across = mouse_x * sensitivity.value;
    float down = mouse_y * sensitivity.value;
    int strafing = (in_strafe.state & 1) != 0;
    int looking = (in_mlook.state & 1) != 0;

    mouse_x = mouse_y = 0;
    if (strafing || (looking && lookstrafe.value))
        cmd->sidemove += across * m_side.value;
    else
        cl.viewangles[YAW] -= across * m_yaw.value;

    if (looking)
        V_StopPitchDrift();
    if (looking && !strafing) {
        float pitch = cl.viewangles[PITCH] + down * m_pitch.value;
        cl.viewangles[PITCH] = pitch > 80 ? 80 : pitch < -70 ? -70 : pitch;
    } else if (strafing && noclip_anglehack) {
        cmd->upmove -= down * m_forward.value;
    } else {
        cmd->forwardmove -= down * m_forward.value;
    }
}

/* ---- what the page asks ------------------------------------------------------------------ */

static int started;

/* The memory the game keeps everything it loads in: one block, handed to it as it starts. */
#define GAME_MEMORY (16 * 1024 * 1024)

/* Starts the game on the data the page put at id1/pak0.pak, drawing on a canvas so many dots across and down. */
EXPORT(gq_start) void gq_start(int width, int height)
{
    static char *command_line[] = {"glquake", NULL};
    static quakeparms_t start;

    if (width >= 320 && height >= 200) {
        scr_width = width;
        scr_height = height;
    }
    COM_InitArgv(1, command_line);
    start = (quakeparms_t){
        .basedir = ".",
        .argc = com_argc,
        .argv = com_argv,
        .memsize = GAME_MEMORY,
        .membase = malloc(GAME_MEMORY),
    };
    if (start.membase == NULL)
        Sys_Error("GridCraft GLQuake: no room for the game's %d MB", GAME_MEMORY >> 20);
    Host_Init(&start);
    started = 1;
}

/* The canvas changed size: the next picture is drawn to the new one. */
EXPORT(gq_resize) void gq_resize(int width, int height)
{
    if (width < 320 || height < 200)
        return;
    scr_width = width;
    scr_height = height;
    if (started)
        lay_out();
}

/* A step of the game: `seconds` is how long it is since the one before. */
EXPORT(gq_tick) void gq_tick(double seconds)
{
    if (!started)
        return;
    GridCraft_Net_Frame();
    Host_Frame((float) seconds);
}

/* Room for a line of the console's commands, which the page writes and then hands over. */
static char command[256];

EXPORT(gq_command_room) char *gq_command_room(void)
{
    return command;
}

EXPORT(gq_command_size) int gq_command_size(void)
{
    return (int) sizeof command;
}

/* Runs what the page wrote into the room, as if typed at the console: hosting a game, or joining one. */
EXPORT(gq_command) void gq_command(void)
{
    command[sizeof command - 1] = 0;
    if (started && command[0]) {
        Cbuf_AddText(command);
        Cbuf_AddText("\n");
    }
    command[0] = 0;
}

/* Whether this game is hosting (1) or joined to one (2) over the network, or neither (0). */
EXPORT(gq_network_state) int gq_network_state(void)
{
    if (sv.active && svs.maxclients > 1)
        return 1;
    if (cls.state == ca_connected && cls.netcon && cls.netcon->driver != 0)
        return 2;
    return 0;
}

/* A key went down (1) or came up (0): `key` is one of Quake's own codes. */
EXPORT(gq_key) void gq_key(int down, int key)
{
    unsigned next = (key_put + 1) % KEYS;

    if (next == key_take)
        return;
    key_queue[key_put] = (unsigned short) (((down ? 1 : 0) << 8) | (key & 0xff));
    key_put = next;
}

/* The mouse moved, while the page holds it for the game: so many dots across and down. */
EXPORT(gq_mouse) void gq_mouse(int across, int down)
{
    mouse_x += across;
    mouse_y += down;
}

/*
 * The page took the mouse for the game (1) or gave it back (0). While the game has it, moving
 * it looks about, up and down too: that is Quake's own "+mlook", which a player holds a key for.
 */
EXPORT(gq_mouse_held) void gq_mouse_held(int held)
{
    mouse_x = 0;
    mouse_y = 0;
    if (started)
        Cbuf_AddText(held ? "+mlook\n" : "-mlook\n");
}

/* How many pictures the game has drawn. */
EXPORT(gq_pictures) int gq_pictures(void)
{
    return pictures;
}

/* ---- the game's variables, as the rows of a table ---------------------------------------- */

EXPORT(gq_names) const char *gq_names(void)
{
    return "State\nLevel\nMap\nTime (s)\nHealth\nArmor\nWeapon\nAmmo\nShells\nNails\nRockets\nCells\n"
           "Monsters killed\nMonsters of\nSecrets\nSecrets of\nThings in view\nX\nY\nZ\nFacing (degrees)";
}

static const char *state(void)
{
    if (cls.state != ca_connected || cls.signon != SIGNONS)
        return key_dest == key_menu ? "Menu" : "Console";
    if (cl.intermission)
        return "Intermission";
    if (cls.demoplayback)
        return "Demo";
    if (key_dest == key_menu)
        return "Menu";
    return cl.paused ? "Paused" : "Playing";
}

static const char *weapon(int active)
{
    switch (active) {
    case IT_AXE: return "Axe";
    case IT_SHOTGUN: return "Shotgun";
    case IT_SUPER_SHOTGUN: return "Double-barrelled shotgun";
    case IT_NAILGUN: return "Nailgun";
    case IT_SUPER_NAILGUN: return "Perforator";
    case IT_GRENADE_LAUNCHER: return "Grenade launcher";
    case IT_ROCKET_LAUNCHER: return "Rocket launcher";
    case IT_LIGHTNING: return "Thunderbolt";
    default: return "";
    }
}

/* Text as one cell of a table written in JSON: nothing in it but letters a sheet can show. */
static int text(char *to, int room, const char *from)
{
    int used = 0;

    if (room < 8)
        return 0;
    to[used++] = '[';
    to[used++] = '"';
    for (; *from && used < room - 6; from++) {
        unsigned char c = (unsigned char) *from & 0x7f; /* Quake's own letters have a coloured twin above 127 */
        if (c == '"' || c == '\\')
            to[used++] = '\\';
        to[used++] = c < 32 ? ' ' : (char) c;
    }
    to[used++] = '"';
    to[used++] = ']';
    to[used] = 0;
    return used;
}

/* The values of the variables gq_names lists, in that order: a table one column wide, as JSON. */
EXPORT(gq_values) const char *gq_values(void)
{
    static char said[1024];
    int used = 0;
    int in_game = cls.state == ca_connected && cls.signon == SIGNONS;
    const float *at = in_game && cl.viewentity > 0 && cl.viewentity < MAX_EDICTS ? cl_entities[cl.viewentity].origin : NULL;
    float facing = cl.viewangles[1];

#define ROOM ((int) sizeof said - used)
#define COMMA said[used++] = ','
#define NUMBER(value) used += snprintf(said + used, ROOM, "[%d]", (int) (value))
    while (facing < 0)
        facing += 360;
    said[used++] = '[';
    used += text(said + used, ROOM, state());
    COMMA;
    used += text(said + used, ROOM, in_game ? cl.levelname : "");
    COMMA;
    used += text(said + used, ROOM, in_game && cl.worldmodel ? cl.worldmodel->name : "");
    COMMA;
    NUMBER(in_game ? cl.time : 0);
    COMMA;
    NUMBER(cl.stats[STAT_HEALTH]);
    COMMA;
    NUMBER(cl.stats[STAT_ARMOR]);
    COMMA;
    used += text(said + used, ROOM, in_game ? weapon(cl.stats[STAT_ACTIVEWEAPON]) : "");
    COMMA;
    NUMBER(cl.stats[STAT_AMMO]);
    COMMA;
    NUMBER(cl.stats[STAT_SHELLS]);
    COMMA;
    NUMBER(cl.stats[STAT_NAILS]);
    COMMA;
    NUMBER(cl.stats[STAT_ROCKETS]);
    COMMA;
    NUMBER(cl.stats[STAT_CELLS]);
    COMMA;
    NUMBER(cl.stats[STAT_MONSTERS]);
    COMMA;
    NUMBER(cl.stats[STAT_TOTALMONSTERS]);
    COMMA;
    NUMBER(cl.stats[STAT_SECRETS]);
    COMMA;
    NUMBER(cl.stats[STAT_TOTALSECRETS]);
    COMMA;
    NUMBER(in_game ? cl_numvisedicts : 0);
    COMMA;
    NUMBER(at ? at[0] : 0);
    COMMA;
    NUMBER(at ? at[1] : 0);
    COMMA;
    NUMBER(at ? at[2] : 0);
    COMMA;
    NUMBER(in_game ? facing : 0);
    said[used++] = ']';
    said[used] = 0;
#undef NUMBER
#undef COMMA
#undef ROOM
    return said;
}

/*
 * A few places in the game's memory: what each is, how its four bytes are read, and where it
 * is. The game says only where; the page reads what is there, and the sheet works it out.
 */
EXPORT(gq_memory) const char *gq_memory(void)
{
    static char said[640];
    static const struct {
        const char *name;
        int stat;
    } PLACES[] = {
        {"Health", STAT_HEALTH},   {"Armor", STAT_ARMOR},     {"Ammo", STAT_AMMO},
        {"Shells", STAT_SHELLS},   {"Nails", STAT_NAILS},     {"Rockets", STAT_ROCKETS},
        {"Cells", STAT_CELLS},     {"Monsters killed", STAT_MONSTERS},
        {"Secrets", STAT_SECRETS},
    };
    int used = 0;
    unsigned i;

    for (i = 0; i < sizeof PLACES / sizeof PLACES[0]; i++)
        used += snprintf(said + used, sizeof said - (size_t) used, "%s\twhole\t%u\n", PLACES[i].name,
                         (unsigned) (uintptr_t) &cl.stats[PLACES[i].stat]);
    return said;
}
"#),
    ("src/gridcraft_glquake.h", r#"/*
 * GridCraft GLQuake: what GLQuake's files call before anything has declared it.
 *
 * The C of 1996 let a file call a function it had not been told of; the compiler of the C
 * Development Kit refuses that, and a project can't tell it otherwise. quakedef.h, which every
 * file of the game reads, reads this at its end: the one line DevCraft adds to it. Each is
 * declared by its types alone, as GLQuake's own files define it.
 *
 * Written for DevCraft's C Development Kit, under DevCraft's licence (MIT OR Apache-2.0).
 */
#ifndef GRIDCRAFT_GLQUAKE_H
#define GRIDCRAFT_GLQUAKE_H

/* Starting and ending a picture, and the 2D drawn over it. */
void GL_Set2D(void);
void V_CalcBlend(void);
qboolean VID_Is8bit(void);

/* Textures. */
int GL_LoadPicTexture(qpic_t *);
void GL_Upload8_EXT(byte *, int, int, qboolean, qboolean);
void GL_BuildLightmaps(void);

/* Models, the world and what is in it. */
void GL_MakeAliasModelDisplayLists(model_t *, aliashdr_t *);
void R_StoreEfrags(efrag_t **);
qboolean R_CullBox(vec3_t, vec3_t);
void R_RotateForEntity(entity_t *);
void R_DrawWorld(void);
void R_DrawBrushModel(entity_t *);
void R_RenderBrushPoly(msurface_t *);

/* Light. */
void R_AnimateLight(void);
void R_RenderDlights(void);
void R_MarkLights(dlight_t *, int, mnode_t *);
int R_LightPoint(vec3_t);

/* Water and the sky. */
void GL_SubdivideSurface(msurface_t *);
void R_DrawWaterSurfaces(void);
void EmitWaterPolys(msurface_t *);
void EmitSkyPolys(msurface_t *);
void EmitBothSkyLayers(msurface_t *);
void R_DrawSkyChain(msurface_t *);

/* Particles. */
void R_ClearParticles(void);
void R_DrawParticles(void);
void R_InitParticles(void);

/* Sums, and what moves through the world. */
void RotatePointAroundVector(vec3_t, const vec3_t, const vec3_t, float);
qboolean SV_RecursiveHullCheck(hull_t *, int, float, float, vec3_t, vec3_t, trace_t *);

/* The console's connect, Quake's own, and the project's (src/net_gridcraft.c), which stands in for it. */
void Host_Connect_f(void);
void GridCraft_Connect_f(void);
void GridCraft_Net_Frame(void);

#endif
"#),
    ("src/net_gridcraft.c", r#"/*
 * GridCraft GLQuake: Quake's network, carried by GridCraft.
 *
 * Quake's own network part (net_dgrm.c, id Software's) speaks its protocol over a "LAN driver":
 * a set of functions that open sockets, read and write packets, and name addresses. This file
 * is that driver, written for this project on the calls GridCraft's C Development Kit hands a
 * program (<gridcraft/net.h>): GridCraft sends a packet only where the project names
 * ([network] allow in project.toml), once the person has turned its network on, and takes in
 * only from those hosts. It also says which drivers Quake has: the loopback (a game on this
 * page) and the datagram one, on this driver.
 *
 * GridCraft gives a program one port, where Quake opens a socket for each connection. So the
 * sockets here are Quake's names for who it talks to: a packet that comes is handed to the
 * socket connected to whoever sent it, or else to the one that listens. A game that hosts
 * listens on the port the project names ([network] listen), and answers every player from it.
 *
 * Quake's connect waits for the server's answer, and nothing can come while a page waits: so
 * `connect` asks the server first, a frame at a time, and makes the connection when the
 * answer is there (GridCraft_Connect_f, GridCraft_Net_Frame).
 *
 * Written for DevCraft's C Development Kit, which writes it into the project when the person
 * sets GLQuake up, and under DevCraft's licence (MIT OR Apache-2.0). It speaks to id Software's
 * net_dgrm.c through the driver Quake defines, and nothing of theirs is copied into it.
 */
#include <stdio.h>
#include <string.h>

#include <gridcraft/net.h>

#include "quakedef.h"
#include "net_dgrm.h"
#include "net_loop.h"

/* ---- Quake's drivers ---------------------------------------------------------------------- */

static int GC_Init(void);
static void GC_Shutdown(void);
static void GC_Listen(qboolean state);
static int GC_OpenSocket(int port);
static int GC_CloseSocket(int socket);
static int GC_Connect(int socket, struct qsockaddr *addr);
static int GC_CheckNewConnections(void);
static int GC_Read(int socket, byte *buf, int len, struct qsockaddr *addr);
static int GC_Write(int socket, byte *buf, int len, struct qsockaddr *addr);
static int GC_Broadcast(int socket, byte *buf, int len);
static char *GC_AddrToString(struct qsockaddr *addr);
static int GC_StringToAddr(char *string, struct qsockaddr *addr);
static int GC_GetSocketAddr(int socket, struct qsockaddr *addr);
static int GC_GetNameFromAddr(struct qsockaddr *addr, char *name);
static int GC_GetAddrFromName(char *name, struct qsockaddr *addr);
static int GC_AddrCompare(struct qsockaddr *addr1, struct qsockaddr *addr2);
static int GC_GetSocketPort(struct qsockaddr *addr);
static int GC_SetSocketPort(struct qsockaddr *addr, int port);

/*
 * The two ways Quake reaches a game: on this page (the loopback, which a game played alone goes
 * by), and over the network (net_dgrm.c, Quake's protocol), whose packets go by this driver.
 */
net_driver_t net_drivers[MAX_NET_DRIVERS] = {
    [0] = {
        .name = "Loopback",
        .Init = Loop_Init,
        .Listen = Loop_Listen,
        .SearchForHosts = Loop_SearchForHosts,
        .Connect = Loop_Connect,
        .CheckNewConnections = Loop_CheckNewConnections,
        .QGetMessage = Loop_GetMessage,
        .QSendMessage = Loop_SendMessage,
        .SendUnreliableMessage = Loop_SendUnreliableMessage,
        .CanSendMessage = Loop_CanSendMessage,
        .CanSendUnreliableMessage = Loop_CanSendUnreliableMessage,
        .Close = Loop_Close,
        .Shutdown = Loop_Shutdown,
    },
    [1] = {
        .name = "Datagram",
        .Init = Datagram_Init,
        .Listen = Datagram_Listen,
        .SearchForHosts = Datagram_SearchForHosts,
        .Connect = Datagram_Connect,
        .CheckNewConnections = Datagram_CheckNewConnections,
        .QGetMessage = Datagram_GetMessage,
        .QSendMessage = Datagram_SendMessage,
        .SendUnreliableMessage = Datagram_SendUnreliableMessage,
        .CanSendMessage = Datagram_CanSendMessage,
        .CanSendUnreliableMessage = Datagram_CanSendUnreliableMessage,
        .Close = Datagram_Close,
        .Shutdown = Datagram_Shutdown,
    },
};
int net_numdrivers = 2;

/* The one way packets go: GridCraft's, below. */
net_landriver_t net_landrivers[MAX_NET_DRIVERS] = {
    [0] = {
        .name = "GridCraft",
        .Init = GC_Init,
        .Shutdown = GC_Shutdown,
        .Listen = GC_Listen,
        .OpenSocket = GC_OpenSocket,
        .CloseSocket = GC_CloseSocket,
        .Connect = GC_Connect,
        .CheckNewConnections = GC_CheckNewConnections,
        .Read = GC_Read,
        .Write = GC_Write,
        .Broadcast = GC_Broadcast,
        .AddrToString = GC_AddrToString,
        .StringToAddr = GC_StringToAddr,
        .GetSocketAddr = GC_GetSocketAddr,
        .GetNameFromAddr = GC_GetNameFromAddr,
        .GetAddrFromName = GC_GetAddrFromName,
        .AddrCompare = GC_AddrCompare,
        .GetSocketPort = GC_GetSocketPort,
        .SetSocketPort = GC_SetSocketPort,
    },
};
int net_numlandrivers = 1;

/* ---- addresses: Quake keeps them as the system's sockaddr_in, which net_dgrm reads ---------- */

#define FAMILY 2 /* AF_INET */

static void make(struct qsockaddr *addr, const byte ip[4], int port)
{
    memset(addr, 0, sizeof *addr);
    addr->sa_family = FAMILY;
    addr->sa_data[0] = (byte) (port >> 8);
    addr->sa_data[1] = (byte) port;
    memcpy(&addr->sa_data[2], ip, 4);
}

/* "a.b.c.d" or "a.b.c.d:port"; a port not given is Quake's own. -1 when it is not that. */
static int read_address(const char *text, struct qsockaddr *addr)
{
    unsigned a, b, c, d, port = (unsigned) net_hostport;
    char end;
    byte ip[4];
    int got = sscanf(text, "%u.%u.%u.%u:%u%c", &a, &b, &c, &d, &port, &end);

    if ((got != 4 && got != 5) || a > 255 || b > 255 || c > 255 || d > 255 || port == 0 || port > 65535)
        return -1;
    ip[0] = (byte) a, ip[1] = (byte) b, ip[2] = (byte) c, ip[3] = (byte) d;
    make(addr, ip, (int) port);
    return 0;
}

static char *GC_AddrToString(struct qsockaddr *addr)
{
    static char text[32];
    const byte *at = addr->sa_data;

    snprintf(text, sizeof text, "%d.%d.%d.%d:%d", at[2], at[3], at[4], at[5], (at[0] << 8) | at[1]);
    return text;
}

static int GC_StringToAddr(char *string, struct qsockaddr *addr)
{
    return read_address(string, addr);
}

static int GC_GetNameFromAddr(struct qsockaddr *addr, char *name)
{
    Q_strcpy(name, GC_AddrToString(addr));
    return 0;
}

/* Only numbers: what the project names is what GridCraft sends to, and Quake asks no one for names. */
static int GC_GetAddrFromName(char *name, struct qsockaddr *addr)
{
    return read_address(name, addr);
}

static int GC_AddrCompare(struct qsockaddr *addr1, struct qsockaddr *addr2)
{
    if (addr1->sa_family != addr2->sa_family || memcmp(&addr1->sa_data[2], &addr2->sa_data[2], 4))
        return -1;
    if (addr1->sa_data[0] != addr2->sa_data[0] || addr1->sa_data[1] != addr2->sa_data[1])
        return 1;
    return 0;
}

static int GC_GetSocketPort(struct qsockaddr *addr)
{
    return (addr->sa_data[0] << 8) | addr->sa_data[1];
}

static int GC_SetSocketPort(struct qsockaddr *addr, int port)
{
    addr->sa_data[0] = (byte) (port >> 8);
    addr->sa_data[1] = (byte) port;
    return 0;
}

/* Where this program is: GridCraft does not say, and every socket of it is the one port Quake's own. */
static int GC_GetSocketAddr(int socket, struct qsockaddr *addr)
{
    static const byte none[4] = {0, 0, 0, 0};

    (void) socket;
    make(addr, none, net_hostport);
    return 0;
}

/* net_dgrm.c's ban (the console's `ban`) reads addresses so, as the systems of 1996 did. */
struct in_addr {
    union {
        struct {
            unsigned char s_b1, s_b2, s_b3, s_b4;
        } S_un_b;
        struct {
            unsigned short s_w1, s_w2;
        } S_un_w;
        unsigned long S_addr;
    } S_un;
};

char *inet_ntoa(struct in_addr in)
{
    static char text[16];

    snprintf(text, sizeof text, "%d.%d.%d.%d", in.S_un.S_un_b.s_b1, in.S_un.S_un_b.s_b2, in.S_un.S_un_b.s_b3, in.S_un.S_un_b.s_b4);
    return text;
}

unsigned long inet_addr(const char *cp)
{
    unsigned a, b, c, d;

    if (sscanf(cp, "%u.%u.%u.%u", &a, &b, &c, &d) != 4 || a > 255 || b > 255 || c > 255 || d > 255)
        return 0xffffffffUL;
    return (unsigned long) a | ((unsigned long) b << 8) | ((unsigned long) c << 16) | ((unsigned long) d << 24);
}

/* ---- sockets: Quake's names for who it talks to -------------------------------------------- */

#define SOCKETS 32
#define LISTENING 0 /* the socket that listens: the control socket */
static struct {
    qboolean open;
    qboolean connected;
    struct qsockaddr peer;
} sockets[SOCKETS];

/* What came and is not read yet: whose it is (-1 nobody's yet), when it came, from where. */
#define QUEUED 96
#define UNCLAIMED -1
static struct {
    int socket;
    double came;
    struct qsockaddr from;
    int length;
    byte data[NET_PACKET];
} queued[QUEUED];
static int queued_count;

static qboolean listen_wanted;
static int listen_said; /* what GridCraft last said of listening: 1 yes, -1 no, 0 not yet */

static void unqueue(int i)
{
    if (i < 0 || i >= queued_count)
        return;
    memmove(&queued[i], &queued[i + 1], (size_t) (queued_count - i - 1) * sizeof queued[0]);
    queued_count--;
}

/* The socket a packet from `from` is for: the one connected to it, else the one that listens. */
static int owner(const struct qsockaddr *from)
{
    int s;

    for (s = 1; s < SOCKETS; s++)
        if (sockets[s].open && sockets[s].connected && !GC_AddrCompare(&sockets[s].peer, (struct qsockaddr *) from))
            return s;
    return listen_wanted && listen_said > 0 ? LISTENING : UNCLAIMED;
}

/* Takes what GridCraft has brought, a socket for each; what nobody claims for a while is dropped. */
static void take_in(void)
{
    static byte data[NET_PACKET];
    char from[64];
    double now = Sys_FloatTime();
    int i, length;

    for (i = 0; i < queued_count;) {
        if (queued[i].socket == UNCLAIMED && now - queued[i].came > 5.0)
            unqueue(i);
        else
            i++;
    }
    while ((length = net_receive(from, sizeof from, data, sizeof data)) != -1) {
        struct qsockaddr at;
        if (length < 0 || read_address(from, &at) == -1)
            continue;
        if (queued_count == QUEUED)
            unqueue(0); /* as a network drops a packet when too many wait */
        queued[queued_count].socket = owner(&at);
        queued[queued_count].came = now;
        queued[queued_count].from = at;
        queued[queued_count].length = length;
        memcpy(queued[queued_count].data, data, (size_t) length);
        queued_count++;
    }
}

static int GC_Init(void)
{
    memset(sockets, 0, sizeof sockets);
    sockets[LISTENING].open = true;
    queued_count = 0;
    /* The menu offers TCP/IP when a driver says it has it. */
    tcpipAvailable = true;
    Q_strcpy(my_tcpip_address, "GridCraft");
    Cmd_AddCommand("gridcraft_connect", Host_Connect_f);
    return LISTENING;
}

static void GC_Shutdown(void)
{
    listen_wanted = false;
}

/* Quake hosts a game (maxplayers more than 1): GridCraft is asked, a frame at a time, until it says. */
static void GC_Listen(qboolean state)
{
    listen_wanted = state;
    listen_said = 0;
    if (state)
        GridCraft_Net_Frame();
}

static int GC_OpenSocket(int port)
{
    int s;

    (void) port;
    for (s = 1; s < SOCKETS; s++) {
        if (!sockets[s].open) {
            sockets[s].open = true;
            sockets[s].connected = false;
            return s;
        }
    }
    return -1;
}

/* Closed: what came for it waits a little for a socket connected to the same place to claim it. */
static int GC_CloseSocket(int socket)
{
    int i;

    if (socket <= LISTENING || socket >= SOCKETS)
        return 0;
    sockets[socket].open = false;
    sockets[socket].connected = false;
    for (i = 0; i < queued_count; i++)
        if (queued[i].socket == socket)
            queued[i].socket = UNCLAIMED;
    return 0;
}

static int GC_Connect(int socket, struct qsockaddr *addr)
{
    int i;

    if (socket <= LISTENING || socket >= SOCKETS)
        return -1;
    sockets[socket].connected = true;
    sockets[socket].peer = *addr;
    for (i = 0; i < queued_count; i++)
        if (queued[i].socket == UNCLAIMED && !GC_AddrCompare(&queued[i].from, addr))
            queued[i].socket = socket;
    return 0;
}

static int GC_CheckNewConnections(void)
{
    int i;

    if (!listen_wanted)
        return -1;
    take_in();
    for (i = 0; i < queued_count; i++)
        if (queued[i].socket == LISTENING)
            return LISTENING;
    return -1;
}

static int GC_Read(int socket, byte *buf, int len, struct qsockaddr *addr)
{
    int i, length;

    take_in();
    for (i = 0; i < queued_count; i++) {
        if (queued[i].socket != socket)
            continue;
        length = queued[i].length < len ? queued[i].length : len;
        memcpy(buf, queued[i].data, (size_t) length);
        *addr = queued[i].from;
        unqueue(i);
        return length;
    }
    return 0;
}

static int GC_Write(int socket, byte *buf, int len, struct qsockaddr *addr)
{
    int sent;

    (void) socket;
    sent = net_send(GC_AddrToString(addr), buf, len);
    if (sent == -1)
        return -1; /* the network is off */
    return len;    /* sent, or dropped as a network drops a packet */
}

/* No calling out to everyone: GridCraft sends only where the project names. */
static int GC_Broadcast(int socket, byte *buf, int len)
{
    (void) socket, (void) buf, (void) len;
    return -1;
}

/* ---- joining a game, a frame at a time ------------------------------------------------------ */

static char joining[64];
static int joining_socket = -1;
static struct qsockaddr joining_at;
static double joining_asked;
static int joining_tries;

/* The first four bytes of a packet of Quake's protocol, high byte first: its kind and its length. */
static unsigned packet_header(const byte *data)
{
    return ((unsigned) data[0] << 24) | ((unsigned) data[1] << 16) | ((unsigned) data[2] << 8) | (unsigned) data[3];
}

/*
 * A request to join, in Quake's protocol: a control packet saying its length, then what it asks
 * (to connect), the game's name with its ending zero, and the version of the protocol.
 */
static void ask_to_join(void)
{
    static const char GAME[] = "QUAKE";
    byte question[4 + 1 + sizeof GAME + 1];
    unsigned header = NETFLAG_CTL | (unsigned) sizeof question;

    question[0] = (byte) (header >> 24);
    question[1] = (byte) (header >> 16);
    question[2] = (byte) (header >> 8);
    question[3] = (byte) header;
    question[4] = CCREQ_CONNECT;
    memcpy(&question[5], GAME, sizeof GAME);
    question[sizeof question - 1] = NET_PROTOCOL_VERSION;
    GC_Write(joining_socket, question, (int) sizeof question, &joining_at);
    joining_asked = Sys_FloatTime();
    joining_tries++;
}

/* The console's `connect` (and the menu's Join): a game on this page as Quake makes it; one over
   the network asked first, and made a frame later, when the server has answered. */
void GridCraft_Connect_f(void)
{
    char *host;
    int on;

    if (Cmd_Argc() < 2) {
        Con_Printf("connect <address> : join the game at that address, like 192.168.1.20\n");
        return;
    }
    host = Cmd_Argv(1);
    if (!Q_strcasecmp(host, "local")) {
        Host_Connect_f();
        return;
    }
    if (GC_GetAddrFromName(host, &joining_at) == -1) {
        Con_Printf("GridCraft GLQuake joins a game by the number of its computer, like 192.168.1.20 or 127.0.0.1:26000\n");
        return;
    }
    on = net_status();
    if (on < 0) {
        Con_Printf("The network is off. Turn it on in Developer > Development, in this project's part.\n");
        return;
    }
    if (joining_socket > 0)
        GC_CloseSocket(joining_socket);
    joining_socket = GC_OpenSocket(0);
    if (joining_socket < 0)
        return;
    GC_Connect(joining_socket, &joining_at);
    Q_strncpy(joining, host, sizeof joining - 1);
    joining[sizeof joining - 1] = 0;
    joining_tries = 0;
    ask_to_join();
    Con_Printf("Asking %s to let you join...\n", GC_AddrToString(&joining_at));
}

/* Each frame, before Quake's: whether GridCraft listens yet, and whether a server answered. */
void GridCraft_Net_Frame(void)
{
    int i;

    if (listen_wanted && listen_said == 0) {
        listen_said = net_listen();
        if (listen_said > 0)
            Con_Printf("Hosting: GridCraft takes in players on port %d, from the hosts this project names.\n", net_hostport);
        else if (listen_said < 0)
            Con_Printf("GridCraft does not take in players here: the Developer console says why.\n");
    }
    if (joining_socket < 0)
        return;
    take_in();
    for (i = 0; i < queued_count; i++) {
        if (queued[i].socket == joining_socket && queued[i].length > 4 && (packet_header(queued[i].data) & NETFLAG_CTL)) {
            /* An answer: Quake makes the connection now, and finds it waiting. */
            GC_CloseSocket(joining_socket);
            joining_socket = -1;
            Cbuf_AddText(va("gridcraft_connect \"%s\"\n", joining));
            return;
        }
    }
    if (Sys_FloatTime() - joining_asked > 1.5) {
        if (joining_tries < 5) {
            ask_to_join();
        } else {
            Con_Printf("No answer from %s.\n", GC_AddrToString(&joining_at));
            GC_CloseSocket(joining_socket);
            joining_socket = -1;
        }
    }
}
"#),
    ("panel/index.html", r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>GLQuake</title>
  <style>
    html, body { margin: 0; background: #16181c; color: #d8dbe0; font: 13px system-ui, sans-serif; }
    body { padding: 10px; }
    #screen {
      display: block; width: 100%; aspect-ratio: 4 / 3; background: #000;
      outline: none; border: 2px solid #2a2d33; box-sizing: border-box;
    }
    #screen:focus { border-color: #8a5a2b; }
    #screen.over { filter: brightness(0.4); }
    p { margin: 8px 0 0; }
    .quiet { color: #9aa0a8; }
    button { font: inherit; padding: 5px 12px; margin-top: 8px; }
    input { font: inherit; padding: 4px 6px; margin-top: 8px; background: #0e0f12; color: inherit; border: 1px solid #2a2d33; }
    #together { margin-top: 6px; }
    details { margin-top: 10px; }
    pre { background: #0e0f12; padding: 8px; max-height: 12em; overflow: auto; font-size: 11px; white-space: pre-wrap; }
  </style>
</head>
<body>
  <canvas id="screen" tabindex="0" aria-label="GLQuake"></canvas>
  <p id="says" aria-live="polite"></p>
  <button id="again" hidden>Start GLQuake again</button>
  <div id="together">
    <button id="host" title="Starts a deathmatch here that another GridCraft can join">Host a game</button>
    <input id="address" value="127.0.0.1" size="18" aria-label="The number of the computer whose game to join">
    <button id="join" title="Joins the game hosted at that number">Join</button>
    <p id="netsays" class="quiet"></p>
  </div>
  <button id="link" hidden>Link to a sheet named "Quake"</button>
  <p id="linked" class="quiet"></p>
  <details>
    <summary class="quiet">What the game printed</summary>
    <pre id="lines"></pre>
  </details>

  <!-- GridCraft serves these two beside this page: spreadsheet.js gives the page its `spreadsheet`
       object, and program.js the loader (CProgram) that runs the program built from this project's C,
       and hands it OpenGL ES on the canvas above. -->
  <script src="spreadsheet.js"></script>
  <script src="program.js"></script>
  <script src="glquake.js"></script>
</body>
</html>
"#),
    ("panel/glquake.js", r#"// GridCraft GLQuake: what the panel's page does.
//
// Written for DevCraft's C Development Kit, which writes it into the project when the person sets
// GLQuake up, and under DevCraft's licence (MIT OR Apache-2.0). The game it runs is id Software's
// GLQuake, under the GNU General Public License, version 2 or later: see the project's README.md.
//
// The game is the project's C, built by GridCraft's C Development Kit as one WebAssembly
// program. GridCraft serves it beside this page, with the loader that runs it (program.js):
// the game can do only what that loader hands it, which is a clock, a place to print, files
// kept in memory, and OpenGL ES on the one canvas this page names. It never sees the workbook.
// This page sizes the canvas, hands the keys and the mouse over, reads the game's variables out
// of it and writes them into a sheet through GridCraft's `spreadsheet` object, where each call
// is checked against what you granted.
"use strict";

const GridCraftGLQuake = (() => {
  // Quake's own numbers for the keys a keyboard's own names don't give.
  const KEYS = {
    ArrowUp: 128, ArrowDown: 129, ArrowLeft: 130, ArrowRight: 131,
    AltLeft: 132, AltRight: 132, ControlLeft: 133, ControlRight: 133, ShiftLeft: 134, ShiftRight: 134,
    Enter: 13, NumpadEnter: 13, Escape: 27, Tab: 9, Space: 32, Backspace: 127,
    F1: 135, F2: 136, F3: 137, F4: 138, F5: 139, F6: 140, F7: 141, F8: 142, F9: 143, F10: 144, F11: 145, F12: 146,
    Insert: 147, Delete: 148, PageDown: 149, PageUp: 150, Home: 151, End: 152, Pause: 255,
    Comma: 44, Period: 46, Slash: 47, Minus: 45, Equal: 61, Semicolon: 59, Quote: 39,
    BracketLeft: 91, BracketRight: 93, Backslash: 92, Backquote: 96,
  };

  /** Quake's number for a key, by where it is on the keyboard. 0 for one Quake has no use for. */
  function keyCode(code) {
    if (KEYS[code]) return KEYS[code];
    if (/^Key[A-Z]$/.test(code)) return code.charCodeAt(3) + 32;
    if (/^Digit[0-9]$/.test(code)) return code.charCodeAt(5);
    return 0;
  }

  /**
   * The canvas's size in dots: as wide as it is shown, at the screen's own fineness, and four
   * by three. Never smaller than Quake's least, nor larger than is worth drawing.
   */
  function dots(shownWidth, ratio) {
    const width = Math.max(320, Math.min(1600, Math.round(shownWidth * (ratio || 1))));
    return [width, Math.round((width * 3) / 4)];
  }

  return { keyCode, dots };
})();

// --- the page ----------------------------------------------------------------------------------

if (typeof document !== "undefined") {
  const SHEET = "Quake";
  /** Where the game's data is, in this project's panel folder and as the game opens it. */
  const DATA = "id1/pak0.pak";
  /** How often the variables are written (milliseconds), and how many seconds the chart looks back. */
  const EVERY = 250;
  const KEPT = 60;
  /** Where the last minute is kept, out of the way of the table and the charts: its first column, and its headings. */
  const LOG = "N";
  const LOGGED = ["Second", "Health", "Armor", null, "Second", "Monsters killed", "Things in view"];

  /** Quake's numbers for the mouse's buttons, by the page's: left, middle, right. */
  const BUTTONS = [200, 202, 201];
  const PLAY = "Arrows move, Ctrl fires, Space jumps, Esc is the menu. Click the picture again to look about with the mouse.";
  const MOUSE = "The game has the mouse: move it to look, its buttons fire. Esc gives it back (so does Ctrl+Alt+M).";

  const screen = document.getElementById("screen");
  const again = document.getElementById("again");
  const says = document.getElementById("says");
  const link = document.getElementById("link");
  const linked = document.getElementById("linked");
  const lines = document.getElementById("lines");
  const printed = [];
  const tell = (text) => {
    says.textContent = text;
  };

  /** The row the table of the game's memory begins at, under the table of its variables. */
  const memoryTop = (names) => names.length + 4;
  /** An address as it is written: eight digits of sixteen. */
  const address = (at) => "0x" + at.toString(16).toUpperCase().padStart(8, "0");
  /** Four bytes as they lie in memory, lowest first: "64 00 00 00" is 100. */
  const lying = (bytes) => Array.from(bytes, (byte) => byte.toString(16).toUpperCase().padStart(2, "0")).join(" ");

  /**
   * The formula that works a whole number out of four bytes written so in the cell `c`. The
   * sheet does this, not the page: the bytes are turned round (the lowest lies first) and read
   * as one number, below nothing when its top bit is set.
   */
  function workedOut(c) {
    const digits = `MID(${c},10,2)&MID(${c},7,2)&MID(${c},4,2)&MID(${c},1,2)`;
    return `=IF(${c}="","",IF(HEX2DEC(${digits})>=2147483648,HEX2DEC(${digits})-4294967296,HEX2DEC(${digits})))`;
  }

  /**
   * Makes the sheet the variables go to: a table of their names, a few cells worked out from
   * them, the last minute second by second, and two charts over that; and under the table, a
   * few places of the game's memory: where each is, the bytes there, and the number the sheet
   * works out of them. One step of undo.
   */
  async function makeSheet(names, places) {
    const last = names.length + 1;
    const row = (name) => names.indexOf(name) + 2;
    const cell = (name) => `B${row(name)}`;
    const table = [["Variable", "Value"], ...names.map((name) => [name, null])];
    const top = memoryTop(names);
    const memory = [
      ["In the game's memory", "Address", "Bytes there", "Worked out in the sheet"],
      ...places.map((place, i) => [place.name, null, null, workedOut(`C${top + 1 + i}`)]),
    ];
    const worked = [
      ["Worked out in the sheet", null],
      ["Health", `=REPT("█",MAX(0,ROUND(${cell("Health")}/5,0)))`],
      ["Kills", `=TEXT(IF(${cell("Monsters of")}>0,${cell("Monsters killed")}/${cell("Monsters of")},0),"0%")`],
      ["Secrets", `=${cell("Secrets")}&" of "&${cell("Secrets of")}`],
      ["Ammo left", `=IF(${cell("Ammo")}<10,"LOW","OK")`],
    ];
    await spreadsheet.batch(
      [
        // The new sheet is the one being worked on: the widths that follow are its columns'.
        ["home.insertSheet", { name: SHEET }],
        ["home.columnWidth", { cols: "A:A", width: 140 }],
        ["home.columnWidth", { cols: "B:B", width: 150 }],
        ["home.columnWidth", { cols: "C:C", width: 90 }],
        ["home.columnWidth", { cols: "D:D", width: 160 }],
        ["range.setValues", { range: `${SHEET}!A1`, values: table }],
        ["range.setValues", { range: `${SHEET}!D1`, values: worked }],
        ["range.setValues", { range: `${SHEET}!${LOG}1`, values: [LOGGED] }],
        ["range.setValues", { range: `${SHEET}!A${top}`, values: memory }],
        ["home.bold", { range: `${SHEET}!A1:T1` }],
        ["home.bold", { range: `${SHEET}!A2:A${last}` }],
        ["home.bold", { range: `${SHEET}!A${top}:D${top}` }],
        ["home.bold", { range: `${SHEET}!A${top + 1}:A${top + places.length}` }],
      ],
      { label: "Make the Quake sheet" },
    );
    // Two charts over the last minute, each with the seconds along the bottom. A workbook can do
    // without them: if one can't be made, the table still fills.
    const charts = [
      { range: `${SHEET}!R1:T${KEPT + 1}`, title: "Monsters killed and things in view, the last minute", at: "F1" },
      { range: `${SHEET}!N1:P${KEPT + 1}`, title: "Health and armor, the last minute", at: "F17" },
    ];
    for (const chart of charts) {
      try {
        await spreadsheet.run("insert.chart", { ...chart, type: "line", width: 500, height: 300 });
      } catch (error) {
        await spreadsheet.log("A chart could not be made:", String(error));
      }
    }
  }

  async function page() {
    tell("Loading the game...");
    const data = await fetch(DATA);
    if (!data.ok) {
      tell(`There is no game data: put Quake's pak0.pak in this project's panel folder as ${DATA}, and trust the project again.`);
      return;
    }
    // The canvas the game draws on, as many dots as it is shown with.
    const size = () => GridCraftGLQuake.dots(screen.clientWidth, window.devicePixelRatio);
    [screen.width, screen.height] = size();
    // The program GridCraft built from this project's C, run by GridCraft's loader, drawing on the canvas.
    const program = await CProgram.load({
      files: { [DATA]: new Uint8Array(await data.arrayBuffer()) },
      canvas: screen,
      print: (line) => {
        printed.push(line);
        lines.textContent = printed.slice(-200).join("\n");
      },
    });
    if (!program.draws) {
      tell("This copy of GridCraft can't draw with OpenGL ES here: GLQuake needs a page that has WebGL 2.");
      return;
    }
    const quake = program.exports;
    quake.gq_start(screen.width, screen.height);
    if (program.ended()) {
      tell(program.ended());
      return;
    }

    // The game, as the rest of this page uses it.
    const game = {
      names: program.string(quake.gq_names()).split("\n"),
      ended: () => program.ended(),
      tick: (seconds) => quake.gq_tick(seconds),
      resize: (width, height) => quake.gq_resize(width, height),
      key: (down, code) => quake.gq_key(down ? 1 : 0, code),
      mouse: (across, down) => quake.gq_mouse(across, down),
      mouseHeld: (held) => quake.gq_mouse_held(held ? 1 : 0),
      values: () => (program.ended() ? [["Ended"]] : JSON.parse(program.string(quake.gq_values()))),
      // A few places in the game's memory: what each is and where it is. The game says where;
      // the page reads what is there.
      places: () =>
        program.ended()
          ? []
          : program
              .string(quake.gq_memory())
              .split("\n")
              .filter(Boolean)
              .map((line) => {
                const [name, kind, at] = line.split("\t");
                return { name, kind, at: Number(at) || 0 };
              }),
      // The four bytes at an address of the game's memory, read by this page.
      peek: (at) => program.bytes(at, 4),
    };
    document.title = "GLQuake";

    // When the panel changes size, so does the canvas, and the game draws to the new size.
    new ResizeObserver(() => {
      const [width, height] = size();
      if (game.ended() || (width === screen.width && height === screen.height)) return;
      screen.width = width;
      screen.height = height;
      game.resize(width, height);
    }).observe(screen);

    // The keys, while the picture has them. When it loses them, every key is let go.
    const down = new Set();
    const key = (pressed) => (event) => {
      const code = GridCraftGLQuake.keyCode(event.code);
      if (!code) return;
      event.preventDefault();
      if (pressed === down.has(code)) return;
      pressed ? down.add(code) : down.delete(code);
      game.key(pressed, code);
    };
    screen.addEventListener("keydown", key(true));
    screen.addEventListener("keyup", key(false));
    // The mouse. A click on the picture gives it the keys; a click when it has them gives the
    // game the mouse as well, until Esc (the browser's own way out, which no page can take
    // away) or Ctrl+Alt+M. While the game has it, its buttons are Quake's.
    const held = () => document.pointerLockElement === screen;
    const letGo = () => {
      for (const code of down) game.key(false, code);
      down.clear();
    };
    screen.addEventListener("mousedown", (event) => {
      if (game.ended()) return;
      if (!held()) {
        if (document.activeElement === screen && screen.requestPointerLock) screen.requestPointerLock();
        return;
      }
      const code = BUTTONS[event.button];
      if (!code) return;
      event.preventDefault();
      down.add(code);
      game.key(true, code);
    });
    screen.addEventListener("mouseup", (event) => {
      const code = BUTTONS[event.button];
      if (!code || !down.has(code)) return;
      down.delete(code);
      game.key(false, code);
    });
    screen.addEventListener("mousemove", (event) => {
      if (held() && !game.ended()) game.mouse(event.movementX, event.movementY);
    });
    screen.addEventListener("contextmenu", (event) => event.preventDefault());
    screen.addEventListener("keydown", (event) => {
      if (held() && event.ctrlKey && event.altKey && event.code === "KeyM") document.exitPointerLock();
    });
    document.addEventListener("pointerlockchange", () => {
      if (game.ended()) return;
      game.mouseHeld(held());
      if (!held()) letGo();
      tell(held() ? MOUSE : document.activeElement === screen ? PLAY : "Click the picture to play.");
    });
    screen.addEventListener("blur", () => {
      letGo();
      if (!game.ended()) tell("Click the picture to play.");
    });
    screen.addEventListener("focus", () => {
      if (!game.ended()) tell(PLAY);
    });
    tell("Click the picture to play.");

    // Playing another GridCraft. GridCraft carries the game's packets, only to the computers this
    // project names, and only once you have turned its network on.
    const command = (text) => {
      const at = quake.gq_command_room();
      const room = quake.gq_command_size();
      const written = new TextEncoder().encode(text).subarray(0, room - 1);
      program.bytes(at, room).set(written);
      program.bytes(at + written.length, 1)[0] = 0;
      quake.gq_command();
    };
    const netsays = document.getElementById("netsays");
    const address = document.getElementById("address");
    document.getElementById("host").addEventListener("click", () => {
      command("disconnect; maxplayers 4; deathmatch 1; map e1m1");
      screen.focus();
    });
    document.getElementById("join").addEventListener("click", () => {
      // A computer's number and perhaps a port: nothing else reaches the console.
      const where = address.value.trim();
      if (!/^[0-9.]{7,15}(:[0-9]{1,5})?$/.test(where)) {
        netsays.textContent = "Join takes a computer's number, like 192.168.1.20 or 127.0.0.1:26000.";
        return;
      }
      command(`connect ${where}`);
      screen.focus();
    });
    setInterval(() => {
      if (game.ended()) return;
      const on = program.network();
      const state = quake.gq_network_state();
      netsays.textContent =
        on < 0
          ? "The network is off: turn it on in Developer > Development, in this project's part, to play another GridCraft."
          : state === 1
            ? "Hosting: another GridCraft joins with this computer's number."
            : state === 2
              ? "Joined a game over the network."
              : "Host a game here, or join another GridCraft's by its computer's number.";
    }, 1000);

    // The game was left (its own Quit), or stopped on an error: it runs no more, and the
    // page says so and offers to start it again, which loads the page anew.
    again.addEventListener("click", () => location.reload());
    const over = () => {
      if (held()) document.exitPointerLock();
      letGo();
      const why = game.ended();
      tell(/\(0\)/.test(why) ? "You left GLQuake. Its last picture is above." : `GLQuake stopped. ${why}`);
      again.hidden = false;
      // The picture is dimmed, so that it is plain nothing moves any more.
      screen.classList.add("over");
    };

    // The sheet. Linked by the person's click, or by itself to a sheet that is there already.
    let to = null; // the sheet's name while linked
    let busy = false;
    let before = "";
    let beforeMemory = "";
    let history = [];
    let second = -1;
    const show = () => {
      link.hidden = !!to;
      linked.textContent = to
        ? `Writing the game's variables, and the bytes at a few places of its memory, to the sheet "${to}", four times a second. Each write is a step of undo.`
        : "";
    };
    const unlink = (why) => {
      to = null;
      show();
      tell(why);
    };
    link.addEventListener("click", async () => {
      link.disabled = true;
      try {
        const book = await spreadsheet.workbook();
        if (!book.sheets.some((sheet) => sheet.name === SHEET)) await makeSheet(game.names, game.places());
        to = SHEET;
        before = "";
        beforeMemory = "";
        show();
      } catch (error) {
        tell(`The sheet could not be linked: ${error.message || error}`);
      }
      link.disabled = false;
    });
    try {
      const book = await spreadsheet.workbook();
      if (book.sheets.some((sheet) => sheet.name === SHEET)) to = SHEET;
    } catch (error) {
      // Without leave to read, the person links it by hand.
    }
    show();

    async function write() {
      if (!to || busy) return;
      const values = game.values();
      const full = values.length === game.names.length;
      const steps = [];
      const now = JSON.stringify(values);
      if (now !== before) steps.push(["range.setValues", { range: `${to}!B2`, values }]);
      // The game's memory itself: where each place is, and the four bytes that lie there now.
      // The sheet's own formulas work the numbers out of them.
      const memory = game.places().map((place) => (place.at ? [address(place.at), lying(game.peek(place.at))] : [null, null]));
      const lies = JSON.stringify(memory);
      if (memory.length && lies !== beforeMemory) {
        steps.push(["range.setValues", { range: `${to}!B${memoryTop(game.names) + 1}`, values: memory }]);
      }
      // Once a second of the level's own clock, a line of the last minute.
      if (full) {
        const at = (name) => values[game.names.indexOf(name)][0];
        const clock = at("Time (s)");
        if (clock !== second) {
          if (clock < second) history = [];
          second = clock;
          history.push([clock, at("Health"), at("Armor"), null, clock, at("Monsters killed"), at("Things in view")]);
          history = history.slice(-KEPT);
          const rows = history.concat(Array.from({ length: KEPT - history.length }, () => LOGGED.map(() => null)));
          steps.push(["range.setValues", { range: `${to}!${LOG}2`, values: rows }]);
        }
      }
      if (!steps.length) return;
      busy = true;
      try {
        await spreadsheet.batch(steps, { label: "Quake" });
        before = now;
        beforeMemory = lies;
      } catch (error) {
        unlink(`Writing to the sheet stopped: ${error.message || error}`);
      }
      busy = false;
    }
    setInterval(write, EVERY);

    // The game: a step for the time that has gone by, which draws a picture on the canvas; the
    // page shows it when the step returns. A page is called for each picture the screen shows;
    // a timer beside that keeps the game going where a page that is not being looked at is
    // called less often. A long gap is not made up for.
    let timer = 0;
    let last = performance.now();
    const tick = () => {
      if (game.ended()) {
        if (again.hidden) over();
        clearInterval(timer);
        return false;
      }
      const now = performance.now();
      const gone = Math.min(0.1, (now - last) / 1000);
      if (gone >= 1 / 72) {
        last = now;
        game.tick(gone);
      }
      return true;
    };
    const frame = () => {
      if (tick()) requestAnimationFrame(frame);
    };
    requestAnimationFrame(frame);
    timer = setInterval(tick, 1000 / 30);
    spreadsheet.log("GLQuake started.").catch(() => {});
  }

  page().catch((error) => tell(`The game could not start: ${error.message || error}`));
}
"#),
];

/// What is kept of the glquake project's code: 89 files, 1,007,106 bytes.
#[rustfmt::skip]
pub(crate) const GLQUAKE_CODE: &[Keep] = &[
    Keep { from: "WinQuake/anorm_dots.h", to: "quake/anorm_dots.h", bytes: 21261, sha256: "d9156f000b9df16f85d231e082733809a52f6b2e46fd8be0f7515a8aa2be398a" },
    Keep { from: "WinQuake/anorms.h", to: "quake/anorms.h", bytes: 6295, sha256: "1d21f2e9c1ff6cb704ade4f04206cbc1aaadb6930370bb7d54a63c469eb9ee65" },
    Keep { from: "WinQuake/bspfile.h", to: "quake/bspfile.h", bytes: 7099, sha256: "7765b458a744879b253020f5039d7ed950fe52b2a0945ebfdd2f1027f53db700" },
    Keep { from: "WinQuake/cd_null.c", to: "quake/cd_null.c", bytes: 1003, sha256: "e4629e97ae81bc32f75be7ab6b9b26c9ce0bed8d778d6bc541589b4c757e890f" },
    Keep { from: "WinQuake/cdaudio.h", to: "quake/cdaudio.h", bytes: 938, sha256: "8bb410e536d6ce0e077cf229bc5fb77cf7ae7501d2e71179ee4211b5cd7c0207" },
    Keep { from: "WinQuake/chase.c", to: "quake/chase.c", bytes: 2395, sha256: "468d930ea3b2f3cf2b6a33921b636ef04ac1a978045128d816881364471b86a5" },
    Keep { from: "WinQuake/cl_demo.c", to: "quake/cl_demo.c", bytes: 7316, sha256: "91e69fe8be5cc8c7c26fc483815dc6be0035d7a59a51b63946b4ea53fa1aa40a" },
    Keep { from: "WinQuake/cl_input.c", to: "quake/cl_input.c", bytes: 11211, sha256: "51e25a1eeaf5ffe004c2e060f4de856c158e44325e54a1d6965c0f414d9dca9b" },
    Keep { from: "WinQuake/cl_main.c", to: "quake/cl_main.c", bytes: 15381, sha256: "f75a64e11578539b49e4dd0a18d2939ef71a1fb869c34351a86ccc7c7dc4cbf6" },
    Keep { from: "WinQuake/cl_parse.c", to: "quake/cl_parse.c", bytes: 20811, sha256: "9c968eef63776a3f89f2f5b9b3e0bd96dab550d094f87df61fe485be4603f065" },
    Keep { from: "WinQuake/cl_tent.c", to: "quake/cl_tent.c", bytes: 8901, sha256: "f2a2fca42dd9a01b8a7a9b87d7fbefaa79de35b6fdda27460cfccf9db620f70e" },
    Keep { from: "WinQuake/client.h", to: "quake/client.h", bytes: 8949, sha256: "884dd5388eb45d28ba1a2c749b6379f638a1b4adea8d3f683764ed4f24e77fd0" },
    Keep { from: "WinQuake/cmd.c", to: "quake/cmd.c", bytes: 12134, sha256: "d61003668f8586aaa629c13ed8d979d692606230eb30c59f80d1c3024dd8edb9" },
    Keep { from: "WinQuake/cmd.h", to: "quake/cmd.h", bytes: 4336, sha256: "99ca99629e32c7711da8fe15d775257db3c2e82ed002527700fc7d87caf2d1a3" },
    Keep { from: "WinQuake/common.c", to: "quake/common.c", bytes: 34881, sha256: "0c0d2cb048f2a088ff8e478d0642d594c0cdb27cf222eeacd68fcdf79e9631e0" },
    Keep { from: "WinQuake/common.h", to: "quake/common.h", bytes: 5430, sha256: "1fffb6991393af49b0291e7f2bb609df49a180e2c4bd4d786b97d9ea7d71369f" },
    Keep { from: "WinQuake/console.c", to: "quake/console.c", bytes: 12187, sha256: "800aa09cc2c331e58948ebb710a450b048eb2e4a6e6c37828e24ef8a711dfa55" },
    Keep { from: "WinQuake/console.h", to: "quake/console.h", bytes: 1478, sha256: "6a3165abff55e700b336231482d0f91807ecaa5b92f43be9619d3ad700ef36cd" },
    Keep { from: "WinQuake/crc.c", to: "quake/crc.c", bytes: 3438, sha256: "549ea3a1ca39a60e7fe82eaa55510970082ab89364ad9904be8b32f2aaecfb54" },
    Keep { from: "WinQuake/crc.h", to: "quake/crc.h", bytes: 894, sha256: "cc7b9d14b3d7cbaf5c59226243d9f882a3396440a9ed905ccee6d84256c55798" },
    Keep { from: "WinQuake/cvar.c", to: "quake/cvar.c", bytes: 4195, sha256: "cd381320c10e85a4a0a4b3ec5c254326a77172bf58c1dfb92354cce8000b39cd" },
    Keep { from: "WinQuake/cvar.h", to: "quake/cvar.h", bytes: 3388, sha256: "a7c534e9aba9e9f618f09a5c901b815e6a5ff4d582fdeb4046f71c17005d5efa" },
    Keep { from: "WinQuake/draw.h", to: "quake/draw.h", bytes: 1489, sha256: "5a661fd88e0224ad59353dd9024f135089337ca7fcea0188159cc37442d3e674" },
    Keep { from: "WinQuake/gl_draw.c", to: "quake/gl_draw.c", bytes: 27776, sha256: "fa8cc2ebf30117aaba6b7992ee144423f00a71466c9234644c401f8eea8bba6e" },
    Keep { from: "WinQuake/gl_mesh.c", to: "quake/gl_mesh.c", bytes: 7694, sha256: "c4804d983ee1ec41d15dbcc99870ef0dffced406039f78f4bbee161fc88c35c9" },
    Keep { from: "WinQuake/gl_model.c", to: "quake/gl_model.c", bytes: 40353, sha256: "66c8a7353344c25b2fab8d8ebc310f137f19530c3573b7fa813a1a14eef41cf5" },
    Keep { from: "WinQuake/gl_model.h", to: "quake/gl_model.h", bytes: 9113, sha256: "1937bdb212f76167015b3a7e1661bcacc21e3f36cd200648f886aee810593c61" },
    Keep { from: "WinQuake/gl_refrag.c", to: "quake/gl_refrag.c", bytes: 4224, sha256: "dbffc12bf33acfe51344504fa46a933da1048eba172a4448b5203b1e2ba3850f" },
    Keep { from: "WinQuake/gl_rlight.c", to: "quake/gl_rlight.c", bytes: 7106, sha256: "b4338bd6b9bb1c4be4a722f4c3f1d49a32a1355b3808c65404aef68063eef818" },
    Keep { from: "WinQuake/gl_rmain.c", to: "quake/gl_rmain.c", bytes: 24449, sha256: "3334706f49183729d50b9b76e2e55e4702d82b169c893674174e9109d6369712" },
    Keep { from: "WinQuake/gl_rmisc.c", to: "quake/gl_rmisc.c", bytes: 11434, sha256: "ab7cb26cbb791cccb81399cab1ccea4324e4a199f531d67ccce1d419d8d0962e" },
    Keep { from: "WinQuake/gl_rsurf.c", to: "quake/gl_rsurf.c", bytes: 35201, sha256: "afee08bc6b8b32ddbc8000b0cba3f2ad67cf3e590af989281ba73bbb6e239902" },
    Keep { from: "WinQuake/gl_screen.c", to: "quake/gl_screen.c", bytes: 17524, sha256: "9635bf984a2daa7feeb5c27fa24843eaa073f182bcbc5250f06a4775ecffe253" },
    Keep { from: "WinQuake/gl_test.c", to: "quake/gl_test.c", bytes: 3362, sha256: "e860f7ea4baffce17e99b9ea42fac3dce537f5025c3109c67d81741c6747dfe2" },
    Keep { from: "WinQuake/gl_warp.c", to: "quake/gl_warp.c", bytes: 21013, sha256: "224631788933f8ba5a47fcf19793557460e440dd5c535a109ba68e2ee15c8401" },
    Keep { from: "WinQuake/gl_warp_sin.h", to: "quake/gl_warp_sin.h", bytes: 3183, sha256: "4c2805707a29602a3ed3a895f89c104f16086624efa82ec9a8aadca191e1d32f" },
    Keep { from: "WinQuake/glquake.h", to: "quake/glquake.h", bytes: 6895, sha256: "c619baedb9270f3385683c95542850c8612aa783c1c0e79209f944a563e7e145" },
    Keep { from: "WinQuake/host.c", to: "quake/host.c", bytes: 20002, sha256: "5f16220fdd2b34e4d6c1d60a9542a951c9a379b53e030af4a19811533498b5f1" },
    Keep { from: "WinQuake/host_cmd.c", to: "quake/host_cmd.c", bytes: 38132, sha256: "39fd5b8bc3f08d90954217e4000253208d6e2b4053d29ec42dc94e5da0d174e8" },
    Keep { from: "WinQuake/input.h", to: "quake/input.h", bytes: 1097, sha256: "21f3c6ff84db0edd35ecfc446891d6e0003ffe34d487c94e498a9bf58da5a39e" },
    Keep { from: "WinQuake/keys.c", to: "quake/keys.c", bytes: 14162, sha256: "eb7582d1a5777fc8ffb02a4039b3e390e471575111c4969ed73fbbd67f9ca647" },
    Keep { from: "WinQuake/keys.h", to: "quake/keys.h", bytes: 3047, sha256: "5e2511a5f5319441de1d98ce748dd32de88b088397d54be481b308d6e3498b55" },
    Keep { from: "WinQuake/mathlib.c", to: "quake/mathlib.c", bytes: 12246, sha256: "68bbe9d6f50c1dcb0ce4adde75dd97a162e8d957ef9e65506b93880cbe6f4115" },
    Keep { from: "WinQuake/mathlib.h", to: "quake/mathlib.h", bytes: 2742, sha256: "d738ca0d4a2c0179b552854481afb324004d95e3fe578ce9fd7c2eed0a5afc87" },
    Keep { from: "WinQuake/menu.c", to: "quake/menu.c", bytes: 66592, sha256: "d872a2178d3df70523bcac5f6df65654bfc156cbdd9d4ea06b04ba84220b901d" },
    Keep { from: "WinQuake/menu.h", to: "quake/menu.h", bytes: 1055, sha256: "404ce3e8d39bc91a04754faff7866cbf48c7416612a1dd19fdea4d894f04074d" },
    Keep { from: "WinQuake/modelgen.h", to: "quake/modelgen.h", bytes: 3012, sha256: "1af78ac12387e348df2da0ddcc85c8ccbe3870da159bb86f74a60d69cca04fda" },
    Keep { from: "WinQuake/net.h", to: "quake/net.h", bytes: 9699, sha256: "5a7dbb64e051ae636176e1d2d206326cbd5f7a4e67ab3e04431551ab8ce53e0b" },
    Keep { from: "WinQuake/net_dgrm.c", to: "quake/net_dgrm.c", bytes: 33161, sha256: "1907ac9a2e611f472c267ba24b183264bff1a833137a2ed201cd5cdbaab442a4" },
    Keep { from: "WinQuake/net_dgrm.h", to: "quake/net_dgrm.h", bytes: 1319, sha256: "535ee1229f68dab1dd80c5bbdc2a472717632fb823530242ca4a4eb4ff1e6df9" },
    Keep { from: "WinQuake/net_loop.c", to: "quake/net_loop.c", bytes: 5254, sha256: "174bb5c02a8cc07b239cbd7376a3253d91229d44b5acdd81c68f9c6f019889b8" },
    Keep { from: "WinQuake/net_loop.h", to: "quake/net_loop.h", bytes: 1271, sha256: "b10db09189ec85b36be6b782022b641363678167e8a1cb0b7d6bac98b9090f45" },
    Keep { from: "WinQuake/net_main.c", to: "quake/net_main.c", bytes: 19678, sha256: "6b0006a11d39e3114cdfd00024bcd869198aeeb28255c7f395be07a84fd68443" },
    Keep { from: "WinQuake/net_vcr.c", to: "quake/net_vcr.c", bytes: 3500, sha256: "a78094df2f8047569bc98d45b62644b52bb0a37ec042ac248b14906746d1e1c7" },
    Keep { from: "WinQuake/net_vcr.h", to: "quake/net_vcr.h", bytes: 1290, sha256: "7ca130f0ab94e56db4b41108559f8742a27441a574b2e1cc7954b448ca98eac7" },
    Keep { from: "WinQuake/pr_cmds.c", to: "quake/pr_cmds.c", bytes: 37281, sha256: "24be89ddb3a27ba2bcc90868f6bb884b069490d97d2c84c631844d63bd38b075" },
    Keep { from: "WinQuake/pr_comp.h", to: "quake/pr_comp.h", bytes: 3067, sha256: "30c8f32cb84bdb6ab31b79873390526e4a7bcc56ee14ea2b6b98dc274f5639ff" },
    Keep { from: "WinQuake/pr_edict.c", to: "quake/pr_edict.c", bytes: 22014, sha256: "90467fdbc99ffb557afae0a828e82c6d2e3d38e7509707fe4a6842578798f28c" },
    Keep { from: "WinQuake/pr_exec.c", to: "quake/pr_exec.c", bytes: 12378, sha256: "43cc7181ee0a74f43e32045fb77f05767041fa1d94ef69f4cc8532d933340b8b" },
    Keep { from: "WinQuake/progdefs.h", to: "quake/progdefs.h", bytes: 803, sha256: "19f6787c93bde2fddacb956b8130eb14f4f891516f846166608fab15522e0f36" },
    Keep { from: "WinQuake/progdefs.q1", to: "quake/progdefs.q1", bytes: 2455, sha256: "e81793ac2f78dc2a461d5de78c65937060ebb272c876bdf12250aec2b673d30c" },
    Keep { from: "WinQuake/progs.h", to: "quake/progs.h", bytes: 3821, sha256: "2ba22bcae0bf4915970876e902064448c4191653968b3a2a2c5bdfade8005596" },
    Keep { from: "WinQuake/protocol.h", to: "quake/protocol.h", bytes: 4789, sha256: "f48a144e3a9c523fb9e9432851fa84b8e9326a0e964b3a346693e1154821c381" },
    Keep { from: "WinQuake/quakedef.h", to: "quake/quakedef.h", bytes: 8474, sha256: "323355bb0413fc2f19d1bfeb41536edf732451e427203b085758318dc4ec23f4" },
    Keep { from: "WinQuake/r_local.h", to: "quake/r_local.h", bytes: 8377, sha256: "070a95a64ff72f4f4f60e01b37bb6271c9e19b53fb208ef53f46beb85b9cbe29" },
    Keep { from: "WinQuake/r_part.c", to: "quake/r_part.c", bytes: 15160, sha256: "6ac4a788d8d447112eada36bd0c6c540432bb646ab05dac73460e1ca4600ab57" },
    Keep { from: "WinQuake/render.h", to: "quake/render.h", bytes: 4866, sha256: "c21a651dec5719f72241ca43dc0f9f4d34fbc7b77d7bbb2857ed86275eea4011" },
    Keep { from: "WinQuake/sbar.c", to: "quake/sbar.c", bytes: 29282, sha256: "dbe1b3393b33a3d86d25d96ba30a0b192cd935af11ded8844fd7bd0aa06b73a4" },
    Keep { from: "WinQuake/sbar.h", to: "quake/sbar.h", bytes: 1264, sha256: "76eb2b6a3cc8f49812b86d758f7790701ab724b3d85a3649c8654f575d32d321" },
    Keep { from: "WinQuake/screen.h", to: "quake/screen.h", bytes: 1611, sha256: "5b889200a07e3288487b63244a9d04b04d700f0fb13868a0a4821e848bc89997" },
    Keep { from: "WinQuake/server.h", to: "quake/server.h", bytes: 7113, sha256: "a408184fe7a6261953d3b40f53df7020e43702abece1746206d736bc1ca59a5d" },
    Keep { from: "WinQuake/snd_null.c", to: "quake/snd_null.c", bytes: 1718, sha256: "3a41baecc2c08ae9478ce18edcf59f582e4d6f05cc280c0b42c458cb7437be02" },
    Keep { from: "WinQuake/sound.h", to: "quake/sound.h", bytes: 4842, sha256: "c3d18c65f381b7afdececb6cbed96c14a405516c18e0a0457830c7ba7259ec2b" },
    Keep { from: "WinQuake/spritegn.h", to: "quake/spritegn.h", bytes: 2808, sha256: "f3f707d9c4ccb8963e3aa4c014afa46ea7efc2f7724e768cc41838c18fde52ab" },
    Keep { from: "WinQuake/sv_main.c", to: "quake/sv_main.c", bytes: 27372, sha256: "8cd794da3c9173888a2b72f60b1ba0d244c32a6fddc0e326ba03f5f7ae17714b" },
    Keep { from: "WinQuake/sv_move.c", to: "quake/sv_move.c", bytes: 9482, sha256: "aea150d48835fb7c6ea2c24d01495622598510c39d08074856d67ffa07eeb92a" },
    Keep { from: "WinQuake/sv_phys.c", to: "quake/sv_phys.c", bytes: 37342, sha256: "4c65e087e2a86fa6fb1883e0ec050ce0469cb31242f4c2243ac28d974826952e" },
    Keep { from: "WinQuake/sv_user.c", to: "quake/sv_user.c", bytes: 13032, sha256: "536f6227f23083b6a59e70976eef9445aa991ba8b1ec6efaa37c758270cdd8d9" },
    Keep { from: "WinQuake/sys.h", to: "quake/sys.h", bytes: 1939, sha256: "1dbd5e878eb5e350010e6955f5e5b7208de887f35216c91ae74f4afb751bb4ae" },
    Keep { from: "WinQuake/vid.h", to: "quake/vid.h", bytes: 2748, sha256: "6dc94194b318f5fd4ebc7adda4da49486c11450ff34f43f6bb0d14b514c15894" },
    Keep { from: "WinQuake/view.c", to: "quake/view.c", bytes: 24042, sha256: "02f1224bdbdb5ec6b64ed76fe2d0bdaa016b03c9e78f681cfbb4dd99a11b4606" },
    Keep { from: "WinQuake/view.h", to: "quake/view.h", bytes: 1032, sha256: "5534d00f1491479f715969fec20b91e32e41a3af53dbb80ab6208c364b07c803" },
    Keep { from: "WinQuake/wad.c", to: "quake/wad.c", bytes: 3417, sha256: "29447665d1253e1fe73907ba8d3f0c26e825902b05a7c0207829e2e313f9ef29" },
    Keep { from: "WinQuake/wad.h", to: "quake/wad.h", bytes: 1737, sha256: "2791cfc7619069afcfc7f82098b8d2bcbd93216619535bb962a881a528487c4d" },
    Keep { from: "WinQuake/world.c", to: "quake/world.c", bytes: 21387, sha256: "258b9a79a35fd40bc6e06cf9d1b426818c407e1f3fd05820bfb8d215654a354d" },
    Keep { from: "WinQuake/world.h", to: "quake/world.h", bytes: 2571, sha256: "2afb7c444c80c276537710c92bb5e0c5e1ccee61a6bdd94d4ca4cbd0776772e1" },
    Keep { from: "WinQuake/zone.c", to: "quake/zone.c", bytes: 18822, sha256: "67cb8b207c7b2e3b128d4f79f2b5efad7b9bfe95d45da15704974eef42a4032b" },
    Keep { from: "WinQuake/zone.h", to: "quake/zone.h", bytes: 3182, sha256: "aec01740b4937228223d3487d8f5fc5e30216055738375c7b605f882fbd6e938" },
    Keep { from: "gnu.txt", to: "LICENSE", bytes: 14882, sha256: "70402dc11b4b91b2e2b4821592ff9cb31ff2f4c6f3dedae15efde5395c7ee170" },
];
