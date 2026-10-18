//! The C Development Kit's second showcase: the Mandelbrot set, worked out by a C program, with
//! its view kept in a sheet.
//!
//! The owner asked for it (2026-10-18: "Can we produce a mandelbrot demo", then "build it in C
//! with the workbook link"). It shows the two halves of a C project at work together: a program
//! that does the arithmetic, a picture's worth of it many times a second, and a page that hands
//! it what to work out and keeps the result in step with the workbook. The program reaches
//! nothing but its own memory; the page reads and writes the sheet through `spreadsheet`, with
//! the `read` and `write` the project asks for.
//!
//! The view lives in a sheet named Mandelbrot, made by the page when the person asks: the middle
//! of the picture, the zoom, the iterations and the colours, each a cell. Change a cell and the
//! picture follows; move the picture and the cells follow, one step of undo each time, so that
//! Undo takes the picture back too. The iterations are a formula of the zoom until the person
//! types a number of their own. Under the view are places to go, which the page lists as
//! buttons and to which it adds the place shown when asked.
//!
//! It is a project like any other: created untrusted, in the user's projects folder, and it does
//! nothing until the user trusts it and builds it.

use std::path::Path;

use crate::project::Project;
use crate::showcase::Showcase;

/// The showcase project's id, and its folder's name.
pub const ID: &str = "mandelbrot";
pub const NAME: &str = "Mandelbrot";

const MANIFEST: &str = r#"# The C Development Kit's second showcase: a C program that works out the Mandelbrot set,
# and a sheet that holds what it shows.
id = "mandelbrot"
name = "Mandelbrot"
version = "0.1.0"
author = ""
description = "A C program that works out the Mandelbrot set, linked to a sheet: change a cell and the picture follows; move the picture and the cells follow."

# The C Development Kit: GridCraft builds the C below as one WebAssembly program, and the
# panel runs it. The program reaches nothing but what its page hands it. The page keeps the
# view in a sheet named Mandelbrot, through `spreadsheet`.
kit = "c"

# The page reads the view from the sheet, and writes it there when the picture is moved.
permissions = ["read", "write"]

[panel]
entry = "panel/index.html"

# Its page calls the three functions it exports (spacing, view and rows): it has no main.
[c]
sources = ["src"]
entry = "none"
"#;

const SOURCE: &str = r##"/*
 * The Mandelbrot set, worked out by a C program in a GridCraft panel.
 *
 * Built by the C Development Kit as WebAssembly. The page says which part of the set to show
 * (view), and has the picture worked out a band of rows at a time (rows), so that it can show
 * it as it comes and keep answering the mouse. The program draws nothing itself: it fills a
 * block of its memory with each dot's colour, four bytes a dot (red, green, blue and how
 * opaque), which the page puts on its canvas.
 *
 * A point c is in the set when z, starting at 0 and taken again and again to z * z + c, never
 * goes further than 2 from the middle. Each dot is followed for at most `limit` steps. One that
 * has gone by then is coloured by how soon it went, smoothly between two steps; one that has
 * not is taken to be in the set, and drawn dark.
 */
#include <math.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define EXPORT(name) __attribute__((export_name(#name)))

/* The most dots a picture has along either side. */
#define MOST 4096
/* The most steps a dot is followed. */
#define LONGEST 100000
/* How far a point may go before it is taken to have gone for good: further than 2 (this is the
 * square of 16), so that the colours between two steps come out smooth. */
#define GONE 256.0
/* How many steps the colours take to go round once. */
#define ROUND 40.0

/* A colour along a scheme: where it stands (0 to 1), and its red, green and blue. Each scheme
 * ends where it starts, so that going round it again shows no seam. */
typedef struct {
    double at;
    uint8_t red, green, blue;
} stop;

static const stop EMBER[] = {
    {0.00, 10, 8, 30}, {0.18, 70, 20, 90}, {0.40, 190, 55, 45},
    {0.62, 245, 155, 45}, {0.82, 255, 238, 175}, {1.00, 10, 8, 30},
};
static const stop SEA[] = {
    {0.00, 4, 18, 40}, {0.25, 16, 92, 128}, {0.50, 110, 200, 205},
    {0.70, 240, 250, 245}, {0.88, 40, 120, 160}, {1.00, 4, 18, 40},
};
static const stop GREY[] = {
    {0.00, 12, 12, 12}, {0.50, 236, 236, 236}, {1.00, 12, 12, 12},
};

/* The schemes, in the order the page names them: Ember, Sea, Grey. */
static const struct {
    const stop *stops;
    int count;
} SCHEMES[] = {
    {EMBER, sizeof EMBER / sizeof *EMBER},
    {SEA, sizeof SEA / sizeof *SEA},
    {GREY, sizeof GREY / sizeof *GREY},
};
#define SCHEME_COUNT ((int) (sizeof SCHEMES / sizeof *SCHEMES))

/* What is shown: the middle of the picture, how far apart its dots are, and how many there are. */
static double middle_x, middle_y, apart;
static int wide, high, limit = 1, scheme;
static uint8_t *dots;

/* How far apart the dots are of a picture `width` by `height` at `zoom`. At 1 the picture holds
 * 3.5 across or 2.5 up, whichever leaves room for the other: the whole set, with a margin. */
EXPORT(spacing) double spacing(double zoom, int width, int height)
{
    double across = 3.5 / (width > 0 ? width : 1);
    double up = 2.5 / (height > 0 ? height : 1);

    return (across > up ? across : up) / (zoom > 0 ? zoom : 1);
}

/* Says what to show: the middle of the picture (x, y), how many times closer than the whole
 * set, the picture's size in dots, the most steps a dot is followed, and the colours. Gives
 * where the picture's dots are kept, or 0 when there is not room for them. */
EXPORT(view) uint8_t *view(double x, double y, double zoom, int width, int height, int steps, int colours)
{
    if (width < 1 || height < 1 || width > MOST || height > MOST)
        return 0;
    if (!isfinite(x) || !isfinite(y) || !isfinite(zoom) || zoom <= 0)
        return 0;
    if (!dots || width != wide || height != high) {
        uint8_t *room = realloc(dots, (size_t) width * (size_t) height * 4);
        if (!room)
            return 0;
        dots = room;
        wide = width;
        high = height;
    }
    middle_x = x;
    middle_y = y;
    apart = spacing(zoom, width, height);
    limit = steps < 1 ? 1 : steps > LONGEST ? LONGEST : steps;
    scheme = colours >= 0 && colours < SCHEME_COUNT ? colours : 0;
    return dots;
}

/* How many steps the point (x, y) took to go, smoothly between two; or -1 when it is in the set. */
static double follow(double x, double y)
{
    double zx = 0, zy = 0, xx = 0, yy = 0;
    /* A point it passed, to see whether it comes round to it again; taken anew ever further
     * apart, so that a long round is found too. */
    double seen_x = 0, seen_y = 0;
    int n, since = 0, round = 8;
    double q = (x - 0.25) * (x - 0.25) + y * y;

    /* The two largest parts of the set are known shapes: the heart, and the disc left of it. A
     * point in either never goes, and following one to the limit would be most of the work. */
    if (q * (q + (x - 0.25)) <= 0.25 * y * y || (x + 1) * (x + 1) + y * y <= 0.0625)
        return -1;
    for (n = 0; n < limit; n++) {
        zy = 2 * zx * zy + y;
        zx = xx - yy + x;
        xx = zx * zx;
        yy = zy * zy;
        if (xx + yy > GONE) {
            /* Each step squares how far it is: so how far it went past GONE says how much of a
             * step it took to get there. */
            double steps = n + 1 - log2(0.5 * log(xx + yy));
            return steps > 0 ? steps : 0;
        }
        /* Back where it was: it goes round for ever, and is in the set. */
        if (fabs(zx - seen_x) + fabs(zy - seen_y) < 1e-16)
            return -1;
        if (++since == round) {
            seen_x = zx;
            seen_y = zy;
            since = 0;
            round *= 2;
        }
    }
    return -1;
}

/* The colour of a dot that took `steps` to go, or of one in the set. */
static void paint(uint8_t dot[4], double steps)
{
    const stop *stops = SCHEMES[scheme].stops;
    int count = SCHEMES[scheme].count, i;
    double along, part;

    dot[3] = 255;
    if (steps < 0) {
        dot[0] = 6;
        dot[1] = 6;
        dot[2] = 10;
        return;
    }
    along = fmod(steps / ROUND, 1.0);
    for (i = 1; i < count - 1 && stops[i].at < along; i++)
        ;
    part = (along - stops[i - 1].at) / (stops[i].at - stops[i - 1].at);
    dot[0] = (uint8_t) (stops[i - 1].red + (stops[i].red - stops[i - 1].red) * part + 0.5);
    dot[1] = (uint8_t) (stops[i - 1].green + (stops[i].green - stops[i - 1].green) * part + 0.5);
    dot[2] = (uint8_t) (stops[i - 1].blue + (stops[i].blue - stops[i - 1].blue) * part + 0.5);
}

/* Works out `count` rows of the picture from row `from`: one dot in every `block` along and
 * down, each standing for the block of dots it is the middle of. A block of 8 is a rough
 * picture, quickly; a block of 1 is the whole one. Gives how many of the dots worked out are
 * in the set. */
EXPORT(rows) int rows(int from, int count, int block)
{
    int inside = 0, end, row, col, down, along;

    if (!dots || from < 0 || from >= high || count < 1 || block < 1 || block > 64)
        return 0;
    end = count > high - from ? high : from + count;
    for (row = from; row < end; row += block) {
        double y = middle_y + (high * 0.5 - row - block * 0.5) * apart;
        for (col = 0; col < wide; col += block) {
            double x = middle_x + (col + block * 0.5 - wide * 0.5) * apart;
            double steps = follow(x, y);
            uint8_t colour[4];

            inside += steps < 0;
            paint(colour, steps);
            for (down = row; down < row + block && down < end; down++)
                for (along = col; along < col + block && along < wide; along++)
                    memcpy(dots + ((size_t) down * (size_t) wide + (size_t) along) * 4, colour, 4);
        }
    }
    return inside;
}
"##;

const PANEL: &str = r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Mandelbrot</title>
  <style>
    html, body { margin: 0; background: #16181c; color: #d8dbe0; font: 13px system-ui, sans-serif; }
    body { padding: 10px; }
    /* As tall as leaves the buttons and what is shown in sight below it: the program works out any shape. */
    #picture { display: block; width: 100%; height: min(75vw, max(220px, calc(100vh - 230px))); background: #000; outline: 1px solid #2a2d33; cursor: grab; touch-action: none; }
    #picture.moving { cursor: grabbing; }
    .bar { display: flex; flex-wrap: wrap; gap: 6px; align-items: center; margin: 8px 0 0; }
    button, select { font: inherit; color: inherit; background: #23262c; border: 1px solid #3a3e46; border-radius: 4px; padding: 3px 9px; }
    button:hover { background: #2c3038; }
    button:disabled { opacity: 0.5; }
    p { margin: 8px 0 0; }
    .small { color: #9aa0a8; font-size: 12px; }
    h2 { font-size: 13px; margin: 14px 0 0; }
    pre { background: #0e0f12; padding: 8px; font-size: 11px; white-space: pre-wrap; margin: 8px 0 0; }
    pre:empty, p:empty { display: none; }
  </style>
</head>
<body>
  <canvas id="picture" aria-label="The Mandelbrot set"></canvas>
  <div class="bar">
    <button id="whole">Whole Set</button>
    <button id="closer">Zoom In</button>
    <button id="further">Zoom Out</button>
    <label>Colours <select id="colours"><option>Ember</option><option>Sea</option><option>Grey</option></select></label>
  </div>
  <p id="where" aria-live="polite"></p>
  <p id="says" aria-live="polite"></p>
  <p class="small">Drag to move. Turn the wheel to zoom where the pointer is; double-click to go four times closer, with Shift four times further.</p>

  <h2>The sheet</h2>
  <p id="linked"></p>
  <div class="bar">
    <button id="make" hidden>Make the Mandelbrot Sheet</button>
    <button id="keep" hidden>Keep This Place</button>
  </div>
  <div class="bar" id="places"></div>
  <pre id="lines"></pre>

  <!-- GridCraft serves these two beside this page: spreadsheet.js gives the page its `spreadsheet`,
       program.js runs the program built from this project's C. Neither is a file of the project. -->
  <script src="spreadsheet.js"></script>
  <script src="program.js"></script>
  <script>
    "use strict";
    const $ = (id) => document.getElementById(id);
    const picture = $("picture");
    const [where, says, linkedSays, lines, placesBar] = [$("where"), $("says"), $("linked"), $("lines"), $("places")];

    // The sheet the view is kept in, and where on it each thing is: B2:B6 hold the middle of the
    // picture (x, then y), the zoom, the iterations and the colours; the places to go are listed
    // from row 14 down.
    const SHEET = "Mandelbrot";
    const VIEW = `${SHEET}!B2:B6`;
    const PLACES_TOP = 14;
    const PLACES_MOST = 100;
    const PLACES = `${SHEET}!A${PLACES_TOP}:D${PLACES_TOP + PLACES_MOST - 1}`;
    const COLOURS = ["Ember", "Sea", "Grey"];
    const WHOLE = { x: -0.75, y: 0, zoom: 1 };
    // Closer than this the program's numbers (doubles) can no longer tell its dots apart.
    const DEEPEST = 1e14;
    const FURTHEST = 0.5;
    // The most dots worked out for one picture: past this the canvas is shown larger than it is.
    const MOST_DOTS = 2500000;

    // The places to go the sheet starts with. The person adds their own under them.
    const STARTING_PLACES = [
      ["The whole set", -0.75, 0, 1],
      ["Seahorse Valley", -0.745, 0.11, 60],
      ["A seahorse's tail", -0.7436, 0.1318, 400],
      ["Elephant Valley", 0.275, 0.006, 100],
      ["A small copy of the set", -1.7548776662466927, 0, 90],
      ["Deep in Seahorse Valley", -0.7436438870371587, 0.131825904205312, 200000],
    ];

    // The iterations follow the zoom, by this rule: in the sheet as a formula, which the person
    // can replace with a number of their own, and here while no sheet is linked.
    const STEPS_RULE = "=ROUND(200*(1+MAX(0,LOG10(B4))),0)";
    const stepsFor = (zoom) => Math.round(200 * (1 + Math.max(0, Math.log10(zoom))));

    const view = { x: WHOLE.x, y: WHOLE.y, zoom: WHOLE.zoom, steps: stepsFor(WHOLE.zoom), colours: "Ember" };
    let program = null;
    // Whether the workbook has the sheet; the view as the sheet holds it, last read or written
    // ([x, y, zoom, iterations, colours]); which of its cells hold formulas, which the page never
    // writes over; whether the iterations are a number the person typed; and the places to go.
    let linked = false;
    let held = null;
    let formulas = [false, false, false, false, false];
    let ownSteps = false;
    let places = [];
    let freeRow = 0;

    const tell = (text) => (says.textContent = text);
    const clampZoom = (zoom) => Math.min(DEEPEST, Math.max(FURTHEST, zoom));
    const number = (cell) => (typeof cell === "number" && Number.isFinite(cell) ? cell : null);

    // ---- The picture: the program works it out, a band of rows at a time, and the page shows it.

    /** The canvas, as many dots as it is shown with, up to MOST_DOTS. */
    function fit() {
      let [width, height] = [picture.clientWidth * devicePixelRatio, picture.clientHeight * devicePixelRatio];
      const over = Math.sqrt((width * height) / MOST_DOTS);
      if (over > 1) [width, height] = [width / over, height / over];
      [width, height] = [Math.max(1, Math.round(width)), Math.max(1, Math.round(height))];
      if (picture.width !== width || picture.height !== height) [picture.width, picture.height] = [width, height];
    }

    // A rough picture first, a dot in every eight along and down; then every dot.
    const BLOCKS = [8, 1];
    let job = null;
    let asked = false;

    /** Starts the picture of the view again, from the rough one. */
    function draw() {
      if (!program) return;
      fit();
      const at = program.exports.view(view.x, view.y, view.zoom, picture.width, picture.height, view.steps, COLOURS.indexOf(view.colours));
      if (!at) {
        job = null;
        tell(program.ended() || "The program has no room for a picture this size.");
        return;
      }
      job = { at, width: picture.width, height: picture.height, pass: 0, row: 0, inside: 0, began: performance.now() };
      showWhere(null);
      if (!asked) {
        asked = true;
        requestAnimationFrame(work);
      }
    }

    /** Works on the picture for a few milliseconds, shows what is done, and comes back for more. */
    function work() {
      asked = false;
      const pen = picture.getContext("2d");
      const until = performance.now() + 12;
      while (job && performance.now() < until) {
        const block = BLOCKS[job.pass];
        const band = Math.min(block === 1 ? 4 : block, job.height - job.row);
        const inside = program.exports.rows(job.row, band, block);
        if (program.ended()) {
          tell(program.ended());
          job = null;
          return;
        }
        // The program's memory is looked at afresh each time: it may have grown since.
        const bytes = new Uint8ClampedArray(program.memory.buffer, job.at + job.row * job.width * 4, band * job.width * 4);
        pen.putImageData(new ImageData(bytes, job.width, band), 0, job.row);
        if (block === 1) job.inside += inside;
        job.row += band;
        if (job.row >= job.height) {
          if (job.pass + 1 < BLOCKS.length) {
            job.pass += 1;
            job.row = 0;
          } else {
            showWhere(job);
            job = null;
          }
        }
      }
      if (job && !asked) {
        asked = true;
        requestAnimationFrame(work);
      }
    }

    /** Says what is shown, and, once the picture is whole, how long it took. */
    function showWhere(done) {
      const digits = Math.min(17, Math.max(3, Math.ceil(Math.log10(view.zoom)) + 4));
      const zoom = view.zoom < 1e5 ? String(Math.round(view.zoom * 100) / 100) : view.zoom.toExponential(2);
      let text = `${view.x.toFixed(digits)} ${view.y < 0 ? "-" : "+"} ${Math.abs(view.y).toFixed(digits)}i, ${zoom} times closer, ${view.steps} iterations`;
      if (done) {
        const share = (100 * done.inside) / (done.width * done.height);
        text += `: worked out in ${Math.round(performance.now() - done.began)} ms, ${share.toFixed(1)}% of it in the set.`;
      }
      if (view.zoom > 1e13) text += " This close, the program's numbers can no longer tell every dot apart.";
      where.textContent = text;
    }

    // ---- Moving the picture: by the mouse, the buttons and the places to go.

    const spacing = () => program.exports.spacing(view.zoom, picture.width, picture.height);
    /** Where on the canvas an event is, in the canvas's own dots. */
    function dotsOf(event) {
      const box = picture.getBoundingClientRect();
      return [((event.clientX - box.left) * picture.width) / box.width, ((event.clientY - box.top) * picture.height) / box.height];
    }
    /** The iterations follow the zoom, unless the sheet holds a number the person typed. */
    function stepsFollow() {
      if (!(linked && ownSteps)) view.steps = stepsFor(view.zoom);
    }
    /** Goes `by` times closer, keeping the point under (px, py) where it is. */
    function zoomAt(px, py, by) {
      if (!program) return;
      let apart = spacing();
      const [x, y] = [view.x + (px - picture.width / 2) * apart, view.y - (py - picture.height / 2) * apart];
      view.zoom = clampZoom(view.zoom * by);
      apart = spacing();
      [view.x, view.y] = [x - (px - picture.width / 2) * apart, y + (py - picture.height / 2) * apart];
      stepsFollow();
      draw();
    }
    function go(place) {
      [view.x, view.y, view.zoom] = [place.x, place.y, clampZoom(place.zoom)];
      stepsFollow();
      draw();
      moved();
    }

    let dragging = null;
    picture.addEventListener("pointerdown", (event) => {
      if (event.button !== 0 || !program) return;
      picture.setPointerCapture(event.pointerId);
      picture.classList.add("moving");
      dragging = dotsOf(event);
    });
    picture.addEventListener("pointermove", (event) => {
      if (!dragging) return;
      const [px, py] = dotsOf(event);
      const apart = spacing();
      view.x -= (px - dragging[0]) * apart;
      view.y += (py - dragging[1]) * apart;
      dragging = [px, py];
      draw();
    });
    const letGo = () => {
      if (!dragging) return;
      dragging = null;
      picture.classList.remove("moving");
      moved();
    };
    picture.addEventListener("pointerup", letGo);
    picture.addEventListener("pointercancel", letGo);
    picture.addEventListener(
      "wheel",
      (event) => {
        event.preventDefault();
        // A wheel says how far it turned in dots, lines or pages.
        const turned = event.deltaY * (event.deltaMode === 1 ? 33 : event.deltaMode === 2 ? 300 : 1);
        const [px, py] = dotsOf(event);
        zoomAt(px, py, Math.pow(2, -turned / 300));
        moved();
      },
      { passive: false }
    );
    picture.addEventListener("dblclick", (event) => {
      const [px, py] = dotsOf(event);
      zoomAt(px, py, event.shiftKey ? 0.25 : 4);
      moved();
    });
    $("whole").addEventListener("click", () => go(WHOLE));
    $("closer").addEventListener("click", () => (zoomAt(picture.width / 2, picture.height / 2, 2), moved()));
    $("further").addEventListener("click", () => (zoomAt(picture.width / 2, picture.height / 2, 0.5), moved()));
    $("colours").addEventListener("change", (event) => {
      view.colours = event.target.value;
      draw();
      writeColours();
    });
    new ResizeObserver(() => draw()).observe(picture);

    // ---- The sheet: the view is kept there, and each side follows the other.

    function showLinked() {
      $("make").hidden = linked;
      $("keep").hidden = !linked;
      if (!linked) {
        linkedSays.textContent = `This workbook has no sheet named ${SHEET}. Make it, and the view is kept there: change a cell and the picture follows.`;
      } else if (formulas.slice(0, 3).some(Boolean)) {
        linkedSays.textContent = `Linked to the sheet ${SHEET}. A cell for the middle or the zoom holds a formula: the picture follows the sheet, and moving it changes nothing there.`;
      } else {
        linkedSays.textContent = `Linked to the sheet ${SHEET}: change B2:B6 and the picture follows; move the picture and B2:B4 follow it, a step of undo each time.`;
      }
      placesBar.replaceChildren(
        ...places.map((place) => {
          const button = document.createElement("button");
          button.textContent = place.name;
          button.title = `${place.x}, ${place.y}, ${place.zoom} times closer`;
          button.addEventListener("click", () => go(place));
          return button;
        })
      );
    }

    // Reads the view and the places from the sheet. What changed there since it was last read or
    // written, and only that, is taken into the picture: what the page wrote itself comes back
    // unchanged, and the person may have moved the picture on since.
    let reading = false;
    let again = false;
    async function readSheet() {
      if (reading) {
        again = true;
        return;
      }
      reading = true;
      do {
        again = false;
        let found;
        try {
          found = await Promise.all([spreadsheet.read(VIEW), spreadsheet.read(VIEW, { formulas: true }), spreadsheet.read(PLACES)]);
        } catch (error) {
          // No such sheet in the workbook being worked on (or no workbook): nothing is linked.
          linked = false;
          held = null;
          places = [];
          showLinked();
          continue;
        }
        const [values, typed, rows] = found;
        const cell = (rows, i) => (rows[i] || [])[0];
        const now = [number(cell(values, 0)), number(cell(values, 1)), number(cell(values, 2)), number(cell(values, 3)), cell(values, 4)];
        formulas = [0, 1, 2, 3, 4].map((i) => typeof cell(typed, i) === "string" && cell(typed, i).startsWith("="));
        ownSteps = !formulas[3] && now[3] !== null;
        linked = true;
        const changed = (i) => !held || held[i] !== now[i];
        let redraw = false;
        if (changed(0) && now[0] !== null) [view.x, redraw] = [now[0], true];
        if (changed(1) && now[1] !== null) [view.y, redraw] = [now[1], true];
        if (changed(2) && now[2] !== null && now[2] > 0) [view.zoom, redraw] = [clampZoom(now[2]), true];
        if (changed(4) && COLOURS.includes(now[4])) {
          [view.colours, redraw] = [now[4], true];
          $("colours").value = now[4];
        }
        const steps = ownSteps || formulas[3] ? now[3] : null;
        const wanted = steps !== null && steps >= 1 ? Math.min(100000, Math.round(steps)) : stepsFor(view.zoom);
        if (wanted !== view.steps) [view.steps, redraw] = [wanted, true];
        held = now;
        places = [];
        freeRow = 0;
        rows.forEach((row, i) => {
          const [name, x, y, zoom] = row;
          if (typeof name === "string" && name.trim() && number(x) !== null && number(y) !== null && number(zoom) > 0) {
            places.push({ name: name.trim(), x, y, zoom });
          } else if (!freeRow && row.every((part) => part === null || part === "")) {
            freeRow = PLACES_TOP + i;
          }
        });
        showLinked();
        if (redraw) draw();
      } while (again);
      reading = false;
    }

    // Writes the view to the sheet once the picture has stopped moving for a moment: one step of
    // undo for one move. Cells that hold formulas are left as they are.
    let waiting = 0;
    function moved() {
      clearTimeout(waiting);
      waiting = setTimeout(writeView, 400);
    }
    async function writeView() {
      if (!linked || !held || formulas.slice(0, 3).some(Boolean)) return;
      const now = [view.x, view.y, view.zoom];
      if (now.every((value, i) => value === held[i])) return;
      // What is written is what the sheet holds, before the sheet says so.
      held = [...now, held[3], held[4]];
      try {
        await spreadsheet.batch([["range.setValues", { range: `${SHEET}!B2`, values: now.map((value) => [value]) }]], { label: "Move the Mandelbrot picture" });
      } catch (error) {
        tell(`The view could not be written to the sheet: ${error.message || error}`);
        readSheet();
      }
    }
    async function writeColours() {
      if (!linked || !held || formulas[4] || held[4] === view.colours) return;
      held = [...held.slice(0, 4), view.colours];
      try {
        await spreadsheet.batch([["range.setValues", { range: `${SHEET}!B6`, values: [[view.colours]] }]], { label: "Change the Mandelbrot colours" });
      } catch (error) {
        tell(`The colours could not be written to the sheet: ${error.message || error}`);
        readSheet();
      }
    }

    /** Makes the sheet, with the view shown now in it: one step of undo. */
    async function makeSheet() {
      const viewRows = [
        ["Mandelbrot", null, "Change a value in column B and the picture follows. Move the picture and B2:B4 follow it."],
        ["Centre X", view.x, "Where the middle of the picture is, along."],
        ["Centre Y", view.y, "And up."],
        ["Zoom", view.zoom, "How many times closer than the whole set."],
        ["Iterations", STEPS_RULE, "How many steps each dot is followed before it is taken to be in the set: more shows more of the edge, and takes longer. Worked out from the zoom: type a number to fix it."],
        ["Colours", view.colours, "Ember, Sea or Grey."],
      ];
      const worked = [
        ["Worked out in the sheet", null],
        ["Width shown, at least", "=3.5/B4"],
        ["Powers of ten", "=ROUND(LOG10(B4),1)"],
        ["Precision", '=IF(B4>1E13,"Past what the program\'s numbers hold","Within what the program\'s numbers hold")'],
      ];
      const placeRows = [["Places to go", "Centre X", "Centre Y", "Zoom"], ...STARTING_PLACES];
      await spreadsheet.batch(
        [
          // The new sheet is the one being worked on: the widths that follow are its columns'.
          ["home.insertSheet", { name: SHEET }],
          ["home.columnWidth", { cols: "A:A", width: 160 }],
          ["home.columnWidth", { cols: "B:B", width: 150 }],
          ["home.columnWidth", { cols: "C:C", width: 130 }],
          ["home.columnWidth", { cols: "D:D", width: 90 }],
          ["range.setValues", { range: `${SHEET}!A1`, values: viewRows }],
          ["range.setValues", { range: `${SHEET}!A8`, values: worked }],
          ["range.setValues", { range: `${SHEET}!A${PLACES_TOP - 1}`, values: placeRows }],
          // Bold, said each time: without `on`, Bold turns the other way from the cell that is selected.
          ["home.bold", { range: `${SHEET}!A1:A6`, on: true }],
          ["home.bold", { range: `${SHEET}!A8`, on: true }],
          ["home.bold", { range: `${SHEET}!A${PLACES_TOP - 1}:D${PLACES_TOP - 1}`, on: true }],
        ],
        { label: "Make the Mandelbrot sheet" }
      );
    }

    $("make").addEventListener("click", async () => {
      $("make").disabled = true;
      try {
        const book = await spreadsheet.workbook();
        if (!book.sheets.some((sheet) => sheet.name === SHEET)) await makeSheet();
        held = null;
        await readSheet();
        tell("");
      } catch (error) {
        tell(`The sheet could not be made: ${error.message || error}`);
      }
      $("make").disabled = false;
    });
    $("keep").addEventListener("click", async () => {
      if (!freeRow) {
        tell("The places to go are full: clear a row of them to keep another.");
        return;
      }
      try {
        const row = [[`Place ${places.length + 1}`, view.x, view.y, view.zoom]];
        await spreadsheet.batch([["range.setValues", { range: `${SHEET}!A${freeRow}`, values: row }]], { label: "Keep a Mandelbrot place" });
        tell(`Kept in row ${freeRow} of the sheet: give it a name of your own there.`);
      } catch (error) {
        tell(`The place could not be kept: ${error.message || error}`);
      }
    });

    async function page() {
      // The program GridCraft built from this project's C, run by GridCraft's loader. It is
      // handed nothing: no canvas, no files. It only works out what the page asks.
      program = await CProgram.load({ print: (line) => (lines.textContent += line + "\n") });
      draw();
      try {
        await spreadsheet.on("workbookChanged", () => readSheet());
        await spreadsheet.on("workbookActivated", () => {
          held = null;
          readSheet();
        });
      } catch (error) {
        tell(`GridCraft does not tell this page what changes: ${error.message || error}`);
      }
      await readSheet();
      spreadsheet.log("Mandelbrot started.").catch(() => {});
    }

    page().catch((error) => tell(String(error.message || error)));
  </script>
</body>
</html>
"##;

/// The showcase's files: where each goes in the project, and its text.
pub const FILES: [(&str, &str); 3] = [("project.toml", MANIFEST), ("src/mandelbrot.c", SOURCE), ("panel/index.html", PANEL)];

/// The showcase, as the C kit's part offers it.
pub const SHOWCASE: Showcase = Showcase {
    id: ID,
    name: NAME,
    kit: crate::kit::C.id,
    says: "A C program that works out the Mandelbrot set, with its view kept in a sheet: change a cell and the picture follows; move the picture and the cells follow.",
    files: &FILES,
};

/// Creates the showcase project in `projects_dir` and returns it as read back from disk. It
/// never touches a folder that already exists.
pub fn create(projects_dir: &Path) -> Result<Project, String> {
    SHOWCASE.create(projects_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::Permission;
    use crate::project::tests::TempDir;
    use crate::trust::{Status, TrustStore};

    #[test]
    fn the_mandelbrot_is_a_sound_c_project_that_asks_to_read_and_write_and_starts_untrusted() {
        let t = TempDir::new("mandelbrot");
        let made = create(&t.0).unwrap();
        assert!(made.is_sound(), "{:?}", made.problem);
        assert_eq!((made.id.as_str(), made.name()), (ID, NAME));
        let manifest = made.manifest.as_ref().unwrap();
        assert_eq!((manifest.kit.as_str(), manifest.network.as_ref()), ("c", None));
        assert_eq!(manifest.permissions, [Permission::Read, Permission::Write], "the sheet, and nothing else");
        let recipe = manifest.c.as_ref().unwrap();
        assert_eq!(
            (recipe.entry, recipe.optimize, recipe.setjmp, recipe.sources_among(&made.files).unwrap()),
            (crate::c::Entry::None, crate::c::Optimize::Size, false, vec!["src/mandelbrot.c".to_string()])
        );
        assert_eq!(made.files, ["panel/index.html", "project.toml", "src/mandelbrot.c"]);
        assert_eq!(TrustStore::load(&t.0.join("trusted.json")).status(&made), Status::Untrusted);
        // A second time, the folder is there: nothing is written over.
        std::fs::write(t.0.join(ID).join("src/mandelbrot.c"), "/* mine */").unwrap();
        assert!(create(&t.0).unwrap_err().contains("already a project folder"));
        assert_eq!(std::fs::read_to_string(t.0.join(ID).join("src/mandelbrot.c")).unwrap(), "/* mine */");
    }

    #[test]
    fn the_program_only_works_out_and_the_page_keeps_the_view_in_the_sheet() {
        // The program exports what the page calls, and asks for nothing to draw with: it fills
        // its own memory, and the page shows it.
        for export in ["EXPORT(spacing)", "EXPORT(view)", "EXPORT(rows)"] {
            assert!(SOURCE.contains(export), "{export}");
        }
        assert!(!SOURCE.contains("#include <GL") && !SOURCE.contains("gridcraft/net.h") && !SOURCE.contains("fopen"));
        for call in ["exports.spacing(", "exports.view(", "exports.rows("] {
            assert!(PANEL.contains(call), "{call}");
        }
        // The page loads GridCraft's two scripts and nothing from anywhere else, and hands the
        // program no canvas.
        assert!(PANEL.contains("<script src=\"spreadsheet.js\"></script>") && PANEL.contains("<script src=\"program.js\"></script>"));
        assert!(!PANEL.contains("http") && !PANEL.contains("fetch(") && !PANEL.contains("canvas:"));
        // The view is the sheet's: read on each change, written back in one step of undo, and
        // a formula there is never written over.
        assert!(PANEL.contains("spreadsheet.on(\"workbookChanged\"") && PANEL.contains("label: \"Move the Mandelbrot picture\""));
        assert!(PANEL.contains("formulas.slice(0, 3).some(Boolean)) return;"));
        // The commands it runs in its batches are these four, and no other: what makes the
        // sheet, and what writes values.
        let commands = ["home.insertSheet", "home.columnWidth", "range.setValues", "home.bold"];
        let mut run: Vec<&str> = PANEL
            .split("[\"")
            .skip(1)
            .filter_map(|after| after.split('"').next())
            .filter(|word| word.contains('.') && word.chars().all(|c| c.is_ascii_alphanumeric() || c == '.'))
            .collect();
        run.sort_unstable();
        run.dedup();
        let mut expected = commands.to_vec();
        expected.sort_unstable();
        assert_eq!(run, expected);
        // Bold is said each time: without it, Bold turns the other way from the selected cell.
        assert_eq!(PANEL.matches("[\"home.bold\"").count(), PANEL.matches(", on: true }]").count());
        // The rule for the iterations is the same in the sheet's formula and in the page.
        assert!(PANEL.contains("\"=ROUND(200*(1+MAX(0,LOG10(B4))),0)\"") && PANEL.contains("Math.round(200 * (1 + Math.max(0, Math.log10(zoom))))"));
        // Every file goes where a project may hold one.
        for (path, _) in FILES {
            assert_eq!(crate::manifest::inner_path(path).as_deref(), Ok(path));
        }
    }

    /// Opt-in: with a zig (`GRIDCRAFT_TEST_ZIG`), the kit builds the program, which exports
    /// the page's three functions and asks for nothing to draw with or to send.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn with_a_zig_the_kit_builds_it_with_the_three_functions_the_page_calls() {
        let zig = std::env::var("GRIDCRAFT_TEST_ZIG").unwrap_or_default();
        if crate::c::find(&zig).is_err() {
            return;
        }
        let t = TempDir::new("mandelbrot-build");
        let at =
            crate::bridge::Places { projects: t.0.join("projects"), trust: t.0.join("trusted-projects.json"), settings: t.0.join("developer.json") };
        create(&at.projects).unwrap();
        let found = crate::project::load(&at.projects.join(ID));
        TrustStore::load(&at.trust).trust(&found, false).unwrap();
        let built = crate::c::build(&at, ID, &zig).unwrap();
        let program = std::fs::read(&built.program).unwrap();
        let has = |name: &str| program.windows(name.len()).any(|part| part == name.as_bytes());
        assert!(has("spacing") && has("view") && has("rows") && has("memory"), "what it exports");
        assert!(!has("glDrawArrays") && !has("net_send"), "what it asks for");
        // Built for size, the kit's way: a few kilobytes (built for speed, zig keeps nearly a
        // megabyte of what a debugger reads beside them, for no faster a picture).
        assert!(built.bytes < 64 * 1024, "{} bytes", built.bytes);
    }
}
