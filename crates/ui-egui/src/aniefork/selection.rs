//! How the selection is drawn, where the fork's options and fixes differ from upstream.
//!
//! - **Its outline lies on the gridlines**, a whole number of device pixels wide. Upstream draws
//!   it centred half a point outside the cells, which puts its bottom and right sides a point
//!   past the gridlines they should cover and, on a display scaled 125% or 150%, leaves a grey
//!   line showing inside them. Excel's outline covers the gridline and reaches outward from it,
//!   on every side (measured: see `Bars`). This one is a fix, not an option.
//! - **Its outline is thinner, and its weight a choice** (File ▸ Options ▸ Selection). Upstream
//!   draws it two points wide: three pixels on a display at 150%, where Excel draws two. The
//!   fork's default, Thin, is Excel's two; Standard is upstream's.
//! - **It can be hidden while another window is the active one**, as Excel does: the outline,
//!   the shading of a range and the fill handle go, and come back when GridCraft is active
//!   again. The marching ants around a copied range stay, as they do in Excel.
//! - **The highlight of a cell being typed into is that same outline, and stays where it is.**
//!   Upstream draws the editor's border half a point further out on every side, makes the editor
//!   a few points taller than a row of the default height, and stretches the border to go round
//!   the text as it grows. Here the highlight neither moves nor grows: text that runs past the
//!   cell is laid over the cells beside it, a whole cell at a time, and clears the lines it runs
//!   over, as Excel does. A fix as well.

use egui::{Color32, Pos2, Rect, Vec2, pos2, vec2};
use gridcraft_engine::core::{CellRef, MAX_COLS, MAX_ROWS, RangeRef};
use gridcraft_engine::model::Sheet;

use crate::SheetApp;
use crate::grid::Geo;
use crate::theme::Tokens;

/// The room upstream's editor keeps beside its text: what the text field leaves clear at its
/// edges, so the editor grows just before its text would be cut off.
const ROOM: f32 = 14.0;
/// Where the text starts inside the editor, and the width of the caret after it.
const TEXT_AT: f32 = 2.0;
const CARET: f32 = 2.0;
/// The most columns or rows the editor looks along for a gridline to stop at.
const REACH: u32 = 2048;

/// What `paint_selection` asks about the selection it is about to draw.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Look {
    /// Whether the selection is drawn at all.
    pub shown: bool,
    /// The width of its outline, in points.
    pub width: f32,
}

/// Hook of `grid::paint_selection`: how the selection is drawn this frame.
pub fn look(app: &SheetApp, ctx: &egui::Context) -> Look {
    let prefs = &app.ui.aniefork;
    let active = ctx.input(|i| i.focused);
    Look { shown: active || !prefs.hide_selection_when_inactive, width: prefs.selection_outline.width() }
}

/// A scale the arithmetic can use: the window's pixels to a point, or 1 if that makes no sense.
fn scale(ppp: f32) -> f32 {
    if ppp.is_finite() && ppp > 0.0 { ppp } else { 1.0 }
}

/// A rectangle with something in it, or nothing.
fn solid(min: Pos2, max: Pos2) -> Option<Rect> {
    let rect = Rect::from_min_max(min, max);
    (rect.is_finite() && rect.is_positive()).then_some(rect)
}

/// Where the grid draws the gridline that marks the cell edge at `at`: from the near side of
/// its first device pixel to the far side of its last, in points.
///
/// The grid draws it a point wide, centred half a point before the edge rounded to a whole
/// point, and the renderer then moves it onto device pixels: a line an odd number of pixels
/// wide is centred on a pixel, an even one between two (epaint's
/// `Stroke::round_center_to_pixel`). This repeats that rule, so that the outline can be laid
/// exactly on the line. A test renders both at several scales and compares them, so a renderer
/// that changes its rule is noticed.
fn gridline(at: f32, ppp: f32) -> (f32, f32) {
    let ppp = scale(ppp);
    let width = ppp;
    let centre = (at.round() - 0.5) * ppp;
    let odd = width <= 1.0 || (width * 0.5 + 0.25).fract() > 0.5;
    let centre = if odd { (centre - 0.5).round() + 0.5 } else { centre.round() };
    // The pixels it covers half of or more: a fainter fringe beside them doesn't count.
    let first = (centre - width / 2.0).round();
    let past = (centre + width / 2.0).round().max(first + 1.0);
    (first / ppp, past / ppp)
}

/// The four sides of the outline round some cells, in points.
///
/// Each side lies on the gridline that bounds the cells and reaches outward from it, a whole
/// number of device pixels wide, so the inside of the outline is exactly the inside of the
/// cells and every edge is sharp at any display scale.
///
/// That is where Excel puts it. Measured on a display at 150% (2026-10-12, a cell being
/// edited): the gridline is one device pixel and the outline two, the gridline's own pixel and
/// the one outside it, on each side. Upstream's outline is centred half a point outside the
/// cells instead, which at the bottom and right is a point past the gridline.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bars {
    pub left: Rect,
    pub top: Rect,
    pub right: Rect,
    pub bottom: Rect,
}

impl Bars {
    /// The outline `width` points wide round `cells`, on a display of `ppp` pixels to a point.
    pub fn new(cells: Rect, width: f32, ppp: f32) -> Bars {
        let ppp = scale(ppp);
        let (left, top) = (gridline(cells.left(), ppp), gridline(cells.top(), ppp));
        let (right, bottom) = (gridline(cells.right(), ppp), gridline(cells.bottom(), ppp));
        // A whole number of pixels, and never thinner than the gridline it lies on.
        let w = ((width * ppp).round().max(1.0) / ppp).max(left.1 - left.0);
        let inner = Rect::from_min_max(pos2(left.1, top.1), pos2(right.0, bottom.0));
        let outer = inner.expand(w);
        Bars {
            left: Rect::from_min_max(outer.min, pos2(inner.left(), outer.bottom())),
            top: Rect::from_min_max(outer.min, pos2(outer.right(), inner.top())),
            right: Rect::from_min_max(pos2(inner.right(), outer.top()), outer.max),
            bottom: Rect::from_min_max(pos2(outer.left(), inner.bottom()), outer.max),
        }
    }

    /// What the outline goes round: the inside of the cells, between their gridlines.
    pub fn inner(&self) -> Rect {
        Rect::from_min_max(pos2(self.left.right(), self.top.bottom()), pos2(self.right.left(), self.bottom.top()))
    }

    /// The far corner of the outline: the outside of its right and bottom sides.
    fn far_corner(&self) -> Pos2 {
        pos2(self.right.right(), self.bottom.bottom())
    }
}

/// Hook of `grid::paint_selection`: paints the selection's outline round `cells`, `width` points
/// wide, on the gridlines.
pub fn paint_outline(p: &egui::Painter, cells: Rect, width: f32, colour: Color32) {
    let bars = Bars::new(cells, width, p.ctx().pixels_per_point());
    for bar in [bars.left, bars.top, bars.right, bars.bottom] {
        if bar.is_finite() && bar.is_positive() {
            p.rect_filled(bar, 0.0, colour);
        }
    }
}

/// How tall the editor needs to be, for text of `lines` lines in a font of `font` points, in a
/// cell `cell` points tall.
///
/// Upstream gives every line 1.3 times its font and four points over, which is more than a row
/// of the default height has: the editor then hangs below the cell's bottom gridline. A cell
/// that shows a line of its font has room to edit one, so there the editor is exactly as tall as
/// the cell. More lines, or a row squeezed shorter than its letters, need what the lines
/// themselves take and no more: two lines of the default font then fit two rows of the default
/// height, where the four points over would take a third.
pub fn editor_height(font: f32, lines: f32, cell: f32) -> f32 {
    let one_line_fits = lines <= 1.0 && cell >= font + 2.0;
    if one_line_fits { cell } else { (font * 1.3 * lines).max(cell) }
}

/// Hook of `grid::in_cell_editor`: the size of the editor for the cell at `rect`, whose text is
/// `text_w` points wide and `lines` lines long in a font of `font` points.
///
/// While the text fits, the editor is the cell. Text that needs more is given the cells after
/// it, a whole column or row at a time, so the editor always ends on a gridline: a cell it
/// covers is covered entirely, and its contents never show beside the text being typed.
pub fn editor_size(geo: &Geo, sh: &Sheet, cell: CellRef, rect: Rect, font: f32, lines: f32, text_w: f32) -> Vec2 {
    let area = sh.merge_at(cell).unwrap_or_else(|| RangeRef::cell(cell));
    // A frozen pane cuts the editor off at its edge: there is nothing past it to grow over.
    let last_col = if area.end.col < geo.fc { geo.fc.saturating_sub(1) } else { MAX_COLS - 1 };
    let last_row = if area.end.row < geo.fr { geo.fr.saturating_sub(1) } else { MAX_ROWS - 1 };
    let in_view = vec2(geo.cells.right() - rect.left(), geo.cells.bottom() - rect.top());
    let w = grown(text_w + ROOM, rect.width(), in_view.x, |n| {
        let col = area.end.col.checked_add(n).filter(|col| *col <= last_col)?;
        Some(geo.cell_rect(sh, CellRef::new(cell.row, col)).right() - rect.left())
    });
    let h = grown(editor_height(font, lines, rect.height()), rect.height(), in_view.y, |n| {
        let row = area.end.row.checked_add(n).filter(|row| *row <= last_row)?;
        Some(geo.cell_rect(sh, CellRef::new(row, cell.col)).bottom() - rect.top())
    });
    vec2(w.min(in_view.x), h)
}

/// The editor's size along one axis: the cell's `own` while `need` fits in it, and otherwise out
/// to the far edge of the first following column or row that gives it room. `far_edge(n)` is the
/// far edge of the `n`th one after the cell, `None` when there is no such column or row. Past
/// `in_view` nothing shows, so the search stops there.
fn grown(need: f32, own: f32, in_view: f32, far_edge: impl Fn(u32) -> Option<f32>) -> f32 {
    if !need.is_finite() || need <= own {
        return own;
    }
    (1..=REACH).map_while(far_edge).find(|edge| *edge >= need || *edge >= in_view).unwrap_or(need)
}

/// What the editor does to what is round its cell, in the grid's logical coordinates.
#[derive(Clone, Debug, Default, PartialEq)]
struct Cover {
    /// The cells it has grown over, inside the gridlines that bound them.
    cells: Vec<Rect>,
    /// Its text has reached the highlight's far side, which is cleared.
    over: bool,
    /// It is taller than its cell: the highlight's bottom side is cleared.
    tall: bool,
}

/// What an editor of the rectangle `editor` covers round its cell at `cell`, whose highlight is
/// `bars`.
///
/// The highlight's far side goes only once the text (`text_w` points wide) has reached it, and
/// its bottom side whenever the editor is taller than the cell. The top and near sides always
/// stay. On a right-to-left sheet the text starts at the editor's far edge, so the line it has
/// to reach is the width of the cells it has grown over away.
fn cover(cell: Rect, editor: Rect, bars: &Bars, text_w: f32, rtl: bool, ppp: f32) -> Cover {
    let wide = editor.right() > cell.right() + 0.5;
    let tall = editor.bottom() > cell.bottom() + 0.5;
    let inner = bars.inner();
    // The gridlines that bound the cells it has grown over stay: it stops short of them.
    let (right, bottom) = (gridline(editor.right(), ppp).0, gridline(editor.bottom(), ppp).0);
    let mut cells = Vec::new();
    if wide {
        cells.extend(solid(pos2(inner.right(), inner.top()), pos2(right, bottom)));
    }
    if tall {
        cells.extend(solid(pos2(inner.left(), inner.bottom()), pos2(inner.right(), bottom)));
    }
    let to_the_line = if rtl { editor.right() - inner.right() } else { inner.right() - cell.left() };
    Cover { cells, over: wide && TEXT_AT + text_w + CARET >= to_the_line, tall }
}

/// Hook of `grid::in_cell_editor`: paints the editor's background and the highlight of its cell.
///
/// `cell` is the cell's rectangle and `size` the editor's size (`editor_size`), in the grid's
/// logical coordinates: what is painted is mirrored for a right-to-left sheet here. The
/// highlight is the selection's own outline, exactly: the same place and the same weight.
pub fn paint_editor(app: &SheetApp, painter: &egui::Painter, geo: &Geo, cell: Rect, size: Vec2, text_w: f32, fill: Color32) {
    let ppp = scale(painter.ctx().pixels_per_point());
    let bars = Bars::new(cell, app.ui.aniefork.selection_outline.width(), ppp);
    let cover = cover(cell, Rect::from_min_size(cell.min, size), &bars, text_w, geo.rtl, ppp);
    let green = Tokens::get(painter.ctx()).sel_border;
    let paint = |rect: Rect, colour: Color32| {
        if rect.is_finite() && rect.is_positive() {
            painter.rect_filled(geo.mirror_rect(rect), 0.0, colour);
        }
    };
    // Everything inside the highlight's top and near sides: the cell itself, and the selection's
    // own right and bottom sides, which are drawn again below unless the editor runs over them.
    paint(Rect::from_min_max(bars.inner().min, bars.far_corner()), fill);
    for cells in &cover.cells {
        paint(*cells, fill);
    }
    paint(bars.left, green);
    paint(bars.top, green);
    if !cover.over {
        paint(bars.right, green);
    }
    if !cover.tall {
        paint(bars.bottom, green);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aniefork::prefs::Outline;
    use gridcraft_engine::model::LineInfo;
    use serde_json::json;

    type Harness = egui_kittest::Harness<'static, SheetApp>;

    const C4: CellRef = CellRef { row: 3, col: 2 };
    const LONG: &str = "Text that is wider than a cell";
    /// The pixels to a point most of these tests render at.
    const PPP: f32 = 2.0;

    fn app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        session.run("selection.set", json!({"range": "C4"})).unwrap();
        SheetApp::new(session, Default::default())
    }

    fn harness_at(app: SheetApp, ppp: f32) -> Harness {
        let mut h = egui_kittest::Harness::builder().with_size(vec2(900.0, 600.0)).with_pixels_per_point(ppp).wgpu().build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            app,
        );
        h.run_steps(4);
        h
    }

    fn harness(app: SheetApp) -> Harness {
        harness_at(app, PPP)
    }

    /// A cell's rectangle on screen, in points.
    fn rect_of(h: &Harness, cell: CellRef) -> Rect {
        let a = h.state();
        let doc = a.session.active().unwrap();
        let sheet = doc.wb.active().unwrap();
        let geo = Geo::new(sheet, a.grid.rect.unwrap(), a.view().scroll);
        geo.cell_rect(sheet, cell)
    }

    /// The selected cell's rectangle, in points.
    fn cell(h: &Harness) -> Rect {
        rect_of(h, h.state().session.active().unwrap().selection.active)
    }

    /// The sides of the selected cell's outline, as the options have it.
    fn bars(h: &Harness) -> Bars {
        Bars::new(cell(h), h.state().ui.aniefork.selection_outline.width(), PPP)
    }

    fn colour(rgba: [u8; 4]) -> Color32 {
        Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3])
    }

    /// The pixel at a point given in points.
    fn pixel(h: &mut Harness, at: Pos2) -> Color32 {
        let image = h.render().unwrap();
        colour(image.get_pixel((at.x * PPP) as u32, (at.y * PPP) as u32).0)
    }

    /// Whether a pixel is the selection's green rather than a gridline's grey or the sheet's white.
    fn is_outline(c: Color32) -> bool {
        c.g() > c.r().saturating_add(40) && c.g() > c.b().saturating_add(25)
    }

    /// Whether a pixel shows a gridline: grey, and at least half as dark as one drawn in full.
    fn is_gridline(c: Color32) -> bool {
        !is_outline(c) && c.r() <= 240
    }

    /// Where the outline's pixels lie along a horizontal line through the middle of the cell,
    /// near the edge at `x` points: (first, last) offsets from the edge in pixels.
    fn across(h: &mut Harness, x: f32) -> Option<(i32, i32)> {
        let at = cell(h);
        let image = h.render().unwrap();
        let y = (at.center().y * PPP) as u32;
        let edge = (x * PPP).round() as i32;
        let hits: Vec<i32> = (-12..12).filter(|dx| is_outline(colour(image.get_pixel((edge + dx) as u32, y).0))).collect();
        Some((*hits.first()?, *hits.last()?))
    }

    fn left_edge(h: &mut Harness) -> Option<(i32, i32)> {
        let x = cell(h).left();
        across(h, x)
    }

    fn right_edge(h: &mut Harness) -> Option<(i32, i32)> {
        let x = cell(h).right();
        across(h, x)
    }

    /// The same along a vertical line through the middle of the cell, near its bottom edge.
    fn bottom_edge(h: &mut Harness) -> Option<(i32, i32)> {
        let at = cell(h);
        let image = h.render().unwrap();
        let x = (at.center().x * PPP) as u32;
        let edge = (at.bottom() * PPP).round() as i32;
        let hits: Vec<i32> = (-12..12).filter(|dy| is_outline(colour(image.get_pixel(x, (edge + dy) as u32).0))).collect();
        Some((*hits.first()?, *hits.last()?))
    }

    fn typed(h: &mut Harness, text: &str) {
        h.input_mut().events.push(egui::Event::Text(text.into()));
        h.run_steps(3);
        assert!(h.state().editor.is_some());
    }

    /// A sheet and its geometry, with nothing scrolled.
    fn sheet(change: impl FnOnce(&mut Sheet)) -> (Sheet, Geo) {
        let a = app();
        let mut sh = a.session.active().unwrap().wb.active().unwrap().clone();
        change(&mut sh);
        let geo = Geo::new(&sh, Rect::from_min_size(pos2(0.0, 0.0), vec2(900.0, 600.0)), Vec2::ZERO);
        (sh, geo)
    }

    #[test]
    fn a_gridline_is_found_where_the_renderer_puts_it() {
        // A pixel to a point: the point before the edge. Two: the same point, two pixels.
        assert_eq!(gridline(100.0, 1.0), (99.0, 100.0));
        assert_eq!(gridline(100.3, 1.0), (99.0, 100.0), "the grid rounds the edge to a whole point first");
        assert_eq!(gridline(100.0, 2.0), (99.0, 100.0));
        // One and a half: two pixels, wherever the nearest pixel boundary falls.
        for edge in [100.0, 101.0, 102.0, 103.0] {
            let (first, past) = gridline(edge, 1.5);
            assert_eq!(((past - first) * 1.5).round(), 2.0, "{edge}");
            assert!(first <= edge - 0.5 && edge - 0.5 <= past, "{edge}: round the line's own centre");
            assert_eq!((first * 1.5).fract(), 0.0, "{edge}: on whole pixels");
        }
        // One and a quarter: one pixel, the one its centre falls in.
        let (first, past) = gridline(100.0, 1.25);
        assert_eq!(((past - first) * 1.25).round(), 1.0);
        // A scale that makes no sense is read as one.
        for ppp in [0.0, -2.0, f32::NAN, f32::INFINITY] {
            assert_eq!(gridline(100.0, ppp), (99.0, 100.0), "{ppp}");
        }
        assert!(gridline(f32::NAN, 1.0).0.is_nan(), "and an edge that makes none gives nothing to paint");
    }

    #[test]
    fn the_outline_covers_each_gridline_and_reaches_outward() {
        let cells = Rect::from_min_max(pos2(100.0, 50.0), pos2(160.0, 66.0));
        // Two points at a pixel a point: each gridline's own pixel and the one outside it.
        let b = Bars::new(cells, 2.0, 1.0);
        assert_eq!(b.inner(), Rect::from_min_max(pos2(100.0, 50.0), pos2(159.0, 65.0)), "the inside of the cell, between its gridlines");
        assert_eq!((b.left.left(), b.left.right()), (98.0, 100.0));
        assert_eq!((b.top.top(), b.top.bottom()), (48.0, 50.0));
        assert_eq!((b.right.left(), b.right.right()), (159.0, 161.0));
        assert_eq!((b.bottom.top(), b.bottom.bottom()), (65.0, 67.0));
        // The sides meet at the corners.
        assert_eq!((b.top.left(), b.top.right()), (98.0, 161.0));
        assert_eq!((b.left.top(), b.left.bottom()), (48.0, 67.0));
        // A point wide, it is the gridlines themselves.
        let hair = Bars::new(cells, 1.0, 1.0);
        assert_eq!((hair.left.left(), hair.left.right(), hair.right.left(), hair.right.right()), (99.0, 100.0, 159.0, 160.0));
        assert_eq!(hair.inner(), b.inner(), "whatever its weight, it goes round the same inside");
        // A whole number of pixels at every scale, and never thinner than the gridline.
        for ppp in [1.0_f32, 1.25, 1.5, 1.75, 2.0, 3.0] {
            for width in [2.0_f32, 1.5, 1.0] {
                let b = Bars::new(cells, width, ppp);
                let (px, grid) = (b.left.width() * ppp, gridline(cells.left(), ppp));
                assert!((px - px.round()).abs() < 1e-3 && px >= 1.0, "{width} at {ppp}: {px} pixels");
                assert!(px + 1e-3 >= (grid.1 - grid.0) * ppp, "{width} at {ppp}: covers its gridline");
                assert_eq!(b.left.right(), grid.1, "{width} at {ppp}: its inner edge is the gridline's");
                assert_eq!(b.right.left(), gridline(cells.right(), ppp).0);
                for side in [b.left, b.top, b.right, b.bottom] {
                    let weight = side.width().min(side.height());
                    assert!((weight - b.left.width()).abs() < 1e-3, "{width} at {ppp}: every side the same weight ({weight})");
                }
            }
        }
        // What Excel draws at 150%, two pixels, is the fork's Thin there; upstream's weight is three.
        assert_eq!((Bars::new(cells, 1.5, 1.5).left.width() * 1.5).round(), 2.0);
        assert_eq!((Bars::new(cells, 2.0, 1.5).left.width() * 1.5).round(), 3.0);
        // Numbers that make no sense give nothing that would be painted.
        for (cells, width, ppp) in [
            (Rect::from_min_max(pos2(f32::NAN, 0.0), pos2(10.0, 10.0)), 2.0, 1.0),
            (cells, f32::NAN, 1.0),
            (cells, 2.0, f32::NAN),
            (Rect::from_min_max(pos2(0.0, 0.0), pos2(f32::INFINITY, 10.0)), 2.0, 1.5),
        ] {
            let b = Bars::new(cells, width, ppp);
            let _ = [b.left, b.top, b.right, b.bottom].map(|side| side.is_finite() && side.is_positive());
        }
    }

    #[test]
    fn a_one_line_editor_is_as_tall_as_its_cell() {
        // The default: an 11-point font (14.67 on screen) in a 20-point row.
        assert_eq!(editor_height(14.67, 1.0, 20.0), 20.0);
        assert_eq!(editor_height(14.67, 1.0, 64.0), 64.0, "a tall cell keeps its height");
        // More lines than one, or a row too short for its font, need what the lines take.
        assert!(editor_height(14.67, 2.0, 20.0) > 38.0);
        assert!(editor_height(24.0, 1.0, 20.0) > 30.0);
        // On screen the default is an 11.73-point font in a 16-point row: a row a line.
        for lines in 2..12 {
            let need = editor_height(11.73, lines as f32, 16.0);
            assert!(need > 16.0 * (lines - 1) as f32 && need <= 16.0 * lines as f32, "{lines} lines need {need}");
        }
        assert_eq!(editor_height(15.0, 1.0, 19.5), 19.5, "a row a shade under the default still holds its line");
        assert_eq!(editor_height(14.67, 3.0, 200.0), 200.0);
        // Numbers that make no sense give a number.
        for (font, lines, cell) in
            [(f32::NAN, 1.0, 20.0), (14.0, f32::NAN, 20.0), (14.0, 1.0, f32::NAN), (-5.0, -1.0, -1.0), (f32::INFINITY, 1.0, 20.0)]
        {
            let _ = editor_height(font, lines, cell);
        }
    }

    #[test]
    fn the_editor_is_its_cell_until_the_text_needs_more_and_then_grows_a_whole_cell_at_a_time() {
        let (sh, geo) = sheet(|_| {});
        let at = geo.cell_rect(&sh, C4);
        let far = |col: u32| geo.cell_rect(&sh, CellRef::new(3, col)).right() - at.left();
        let size = |text_w: f32, lines: f32| editor_size(&geo, &sh, C4, at, 12.0, lines, text_w);
        assert_eq!(size(0.0, 1.0), at.size());
        assert_eq!(size(at.width() - ROOM, 1.0), at.size(), "text that just fits");
        assert_eq!(size(at.width() - ROOM + 1.0, 1.0), vec2(far(3), at.height()), "a point more takes the whole of D4");
        assert_eq!(size(far(3) - ROOM, 1.0).x, far(3));
        assert_eq!(size(far(3), 1.0).x, far(4), "and then the whole of E4");
        assert_eq!(size(far(5) - 20.0, 1.0).x, far(5));
        // Downwards too: three lines need more than the row, and get whole rows.
        let below = |row: u32| geo.cell_rect(&sh, CellRef::new(row, 2)).bottom() - at.top();
        let tall = size(0.0, 3.0);
        let need = editor_height(12.0, 3.0, at.height());
        assert_eq!(tall.x, at.width());
        let rows = (4..40).find(|row| below(*row) >= need).unwrap();
        assert_eq!(tall.y, below(rows), "the first gridline that gives the lines room");
        assert!(below(rows - 1) < need);
        assert_eq!(rows, 5, "three lines take C4, C5 and C6");
        // It stops where the grid does.
        let most = geo.cells.right() - at.left();
        assert_eq!(size(1.0e9, 1.0).x, most);
        // Numbers that make no sense leave the editor the size of its cell.
        for (font, lines, text_w) in [(f32::NAN, 1.0, 10.0), (12.0, f32::NAN, 10.0), (12.0, 1.0, f32::NAN), (12.0, 1.0, f32::INFINITY)] {
            let size = editor_size(&geo, &sh, C4, at, font, lines, text_w);
            assert!(size.x.is_finite() && size.y.is_finite(), "{font} {lines} {text_w}: {size:?}");
        }
        assert_eq!(size(-50.0, -3.0), at.size());
    }

    #[test]
    fn the_editor_grows_past_a_hidden_column_over_a_merged_cell_and_not_out_of_a_frozen_pane() {
        // D is hidden: the next gridline is E's.
        let (sh, geo) = sheet(|sh| {
            sh.cols.insert(3, LineInfo { hidden: true, ..Default::default() });
        });
        let at = geo.cell_rect(&sh, C4);
        let e4 = geo.cell_rect(&sh, CellRef::new(3, 4));
        assert_eq!(geo.cell_rect(&sh, CellRef::new(3, 3)).width(), 0.0);
        assert_eq!(editor_size(&geo, &sh, C4, at, 12.0, 1.0, at.width()).x, e4.right() - at.left());
        // C4:D5 is merged: the editor starts as all of it, and grows from its far edges.
        let merged = RangeRef::new(C4, CellRef::new(4, 3));
        let (sh, geo) = sheet(|sh| sh.merges.push(merged));
        let at = geo.range_rect(&sh, merged);
        assert_eq!(editor_size(&geo, &sh, C4, at, 12.0, 1.0, 10.0), at.size());
        let e4 = geo.cell_rect(&sh, CellRef::new(3, 4));
        assert_eq!(editor_size(&geo, &sh, C4, at, 12.0, 1.0, at.width()).x, e4.right() - at.left());
        // Columns A to C are frozen: C4 has no cell in its pane to grow over, so its editor is
        // only as wide as its text needs, and the pane's edge cuts it off.
        let (sh, geo) = sheet(|sh| sh.freeze = Some((0, 3)));
        let at = geo.cell_rect(&sh, C4);
        assert_eq!(editor_size(&geo, &sh, C4, at, 12.0, 1.0, at.width()).x, at.width() + ROOM);
        // B4, in the same pane, still has C4.
        let b4 = geo.cell_rect(&sh, CellRef::new(3, 1));
        assert_eq!(editor_size(&geo, &sh, CellRef::new(3, 1), b4, 12.0, 1.0, b4.width()).x, at.right() - b4.left());
    }

    #[test]
    fn the_highlight_loses_only_the_sides_the_editor_runs_over() {
        let cell = Rect::from_min_max(pos2(100.0, 50.0), pos2(160.0, 66.0));
        let bars = Bars::new(cell, 2.0, 1.0);
        let cover = |editor: Rect, text_w: f32, rtl: bool| cover(cell, editor, &bars, text_w, rtl, 1.0);
        assert_eq!(cover(cell, 30.0, false), Cover::default(), "an editor the size of its cell covers nothing else");
        // Grown over the next column, with the text still short of the line between them.
        let grown = Rect::from_min_max(cell.min, pos2(220.0, 66.0));
        let short = cover(grown, 50.0, false);
        assert_eq!(short.cells, [Rect::from_min_max(pos2(159.0, 50.0), pos2(219.0, 65.0))], "the gridline between them and the next cell");
        assert!(!short.over && !short.tall, "the line stays until the text reaches it");
        // The text reaches the line: it goes. The bottom line stays.
        let over = cover(grown, 56.0, false);
        assert_eq!(over.cells, short.cells);
        assert!(over.over && !over.tall);
        // Taller than the cell: the bottom line goes, and the cells below are covered up to the
        // gridline at the cell's right, which stays.
        let down = cover(Rect::from_min_max(cell.min, pos2(160.0, 98.0)), 10.0, false);
        assert_eq!(down.cells, [Rect::from_min_max(pos2(100.0, 65.0), pos2(159.0, 97.0))]);
        assert!(down.tall && !down.over);
        // Both ways: both lines.
        let both = cover(Rect::from_min_max(cell.min, pos2(220.0, 98.0)), 200.0, false);
        assert_eq!(both.cells.len(), 2);
        assert!(both.over && both.tall);
        // Right to left, the text starts at the editor's far edge: the line is the grown part away.
        assert!(!cover(grown, 50.0, true).over);
        assert!(cover(grown, 57.0, true).over);
        let far = Rect::from_min_max(cell.min, pos2(400.0, 66.0));
        assert!(!cover(far, 57.0, true).over, "text that starts far off has further to come");
        assert!(cover(far, 57.0, false).over);
        // Numbers that make no sense paint nothing.
        for (editor, text_w) in [
            (Rect::from_min_max(cell.min, pos2(f32::NAN, f32::NAN)), 60.0),
            (Rect::from_min_max(cell.min, pos2(f32::INFINITY, f32::INFINITY)), 60.0),
            (grown, f32::NAN),
            (Rect::from_min_max(cell.min, pos2(160.6, 66.0)), 600.0),
        ] {
            let cover = cover(editor, text_w, false);
            assert!(cover.cells.iter().all(|r| r.is_finite() && r.is_positive()), "{editor:?}: {cover:?}");
        }
    }

    #[test]
    fn the_selection_is_shown_and_thin_unless_the_options_say_otherwise() {
        let mut a = app();
        let ctx = egui::Context::default();
        assert_eq!(look(&a, &ctx), Look { shown: true, width: 1.5 });
        a.ui.aniefork.selection_outline = Outline::Standard;
        assert_eq!(look(&a, &ctx), Look { shown: true, width: 2.0 }, "upstream's weight is there for the choosing");
        a.ui.aniefork.selection_outline = Outline::Hairline;
        assert_eq!(look(&a, &ctx), Look { shown: true, width: 1.0 });
    }

    /// The pixels of one line of a rendering that are the outline's, and those that show a
    /// gridline: positions along the line, from `from` to `to`.
    fn line(at: impl Fn(i32) -> Color32, from: i32, to: i32) -> (Vec<i32>, Vec<i32>) {
        ((from..=to).filter(|i| is_outline(at(*i))).collect(), (from..=to).filter(|i| is_gridline(at(*i))).collect())
    }

    #[test]
    fn the_outline_lies_on_the_gridlines_at_every_display_scale() {
        for ppp in [1.0_f32, 1.25, 1.5, 2.0] {
            for outline in Outline::ALL {
                let mut h = harness_at(app(), ppp);
                h.state_mut().ui.aniefork.selection_outline = outline;
                h.run_steps(2);
                let (at, above, before) = (cell(&h), rect_of(&h, CellRef::new(2, 2)), rect_of(&h, CellRef::new(3, 1)));
                let image = h.render().unwrap();
                let px = |v: f32| (v * ppp).round() as i32;
                let weight = ((outline.width() * ppp).round() as usize).max(1);
                let what = format!("{outline:?} at {ppp}");
                // Across the cell, with the row above for where the gridlines are; then down it,
                // with the column before.
                for (horizontal, own, other, from, to, middle) in [
                    (true, px(at.center().y), px(above.center().y), px(at.left()) - 9, px(at.right()) + 9, px(at.center().x)),
                    (false, px(at.center().x), px(before.center().x), px(at.top()) - 9, px(at.bottom()) + 9, px(at.center().y)),
                ] {
                    let at = |fixed: i32| {
                        let image = &image;
                        move |i: i32| {
                            let (x, y) = if horizontal { (i, fixed) } else { (fixed, i) };
                            colour(image.get_pixel(x as u32, y as u32).0)
                        }
                    };
                    let (green, inside) = line(at(own), from, to);
                    let (_, grid) = line(at(other), from, to);
                    let (near, far): (Vec<i32>, Vec<i32>) = green.iter().partition(|i| **i < middle);
                    let (grid_near, grid_far): (Vec<i32>, Vec<i32>) = grid.iter().partition(|i| **i < middle);
                    assert!(!grid_near.is_empty() && !grid_far.is_empty(), "{what}: the gridlines beside the cell are found");
                    // Each side is one run of whole pixels, as heavy as asked (or as its gridline).
                    for side in [&near, &far] {
                        assert_eq!(side.len(), weight.max(grid_near.len()), "{what} ({horizontal}): {side:?}");
                        assert!(side.windows(2).all(|w| w[1] == w[0] + 1), "{what}: {side:?}");
                    }
                    // It covers its gridline and reaches outward from it, never into the cell.
                    assert_eq!(near.last(), grid_near.last(), "{what} ({horizontal}): near side {near:?} on gridline {grid_near:?}");
                    assert_eq!(far.first(), grid_far.first(), "{what} ({horizontal}): far side {far:?} on gridline {grid_far:?}");
                    // No gridline shows inside it.
                    assert!(inside.is_empty(), "{what} ({horizontal}): a gridline inside the outline at {inside:?}");
                }
            }
        }
    }

    #[test]
    fn the_outline_is_drawn_thinner_when_asked() {
        let mut h = harness(app());
        let mut widths = Vec::new();
        for outline in Outline::ALL {
            h.state_mut().ui.aniefork.selection_outline = outline;
            h.run_steps(2);
            let (first, last) = left_edge(&mut h).unwrap();
            widths.push(last - first + 1);
        }
        assert_eq!(widths, [4, 3, 2], "two pixels a point: 2, 1.5 and 1 points");
    }

    #[test]
    fn the_selection_hides_while_another_window_is_active_only_when_asked() {
        let mut a = app();
        a.session.run("selection.set", json!({"range": "C4:D6"})).unwrap();
        let mut h = harness(a);
        assert!(left_edge(&mut h).is_some());
        // Another window becomes the active one: by default nothing changes.
        h.input_mut().focused = false;
        h.run_steps(2);
        assert!(left_edge(&mut h).is_some(), "upstream's behaviour unless the option is on");
        h.state_mut().ui.aniefork.hide_selection_when_inactive = true;
        h.run_steps(2);
        assert!(left_edge(&mut h).is_none(), "the outline is gone");
        let at = cell(&h);
        let inside = pixel(&mut h, pos2(at.right() + 30.0, at.bottom() + 30.0));
        assert_eq!(inside, Color32::WHITE, "and so is the shading of the range");
        // GridCraft is active again: it is back.
        h.input_mut().focused = true;
        h.run_steps(2);
        assert!(left_edge(&mut h).is_some());
    }

    #[test]
    fn typing_into_a_cell_leaves_the_highlight_where_it_was() {
        for outline in Outline::ALL {
            let mut h = harness(app());
            h.state_mut().ui.aniefork.selection_outline = outline;
            h.run_steps(2);
            let selected = (left_edge(&mut h).unwrap(), bottom_edge(&mut h).unwrap(), right_edge(&mut h).unwrap());
            typed(&mut h, "Test");
            let typing = (left_edge(&mut h).unwrap(), bottom_edge(&mut h).unwrap(), right_edge(&mut h).unwrap());
            assert_eq!(typing, selected, "{outline:?}: the highlight stays on the gridlines on every side when typing begins");
            // And the same above the cell.
            let at = cell(&h);
            let image = h.render().unwrap();
            let x = (at.left() * PPP + 20.0) as u32;
            let edge = (at.top() * PPP).round() as i32;
            let above = (-12..12).filter(|dy| is_outline(colour(image.get_pixel(x, (edge + dy) as u32).0))).count() as f32;
            assert_eq!(above, outline.width() * PPP, "{outline:?}: the top side is as heavy as the others");
        }
    }

    #[test]
    fn text_that_runs_past_the_cell_clears_the_lines_it_crosses_and_the_highlight_does_not_grow() {
        for outline in Outline::ALL {
            let mut h = harness(app());
            h.state_mut().ui.aniefork.selection_outline = outline;
            h.run_steps(2);
            let (at, d4, sides) = (cell(&h), rect_of(&h, CellRef::new(3, 3)), bars(&h));
            let selected = (left_edge(&mut h).unwrap(), bottom_edge(&mut h).unwrap());
            // The gridline between D4 and E4, near the top of the row, where no letter reaches.
            let gridline = pos2(d4.right().round() - 0.5, at.top() + 1.5);
            assert!(is_gridline(pixel(&mut h, gridline)), "{outline:?}: a gridline to begin with");
            assert!(is_outline(pixel(&mut h, sides.right.center())), "{outline:?}: and a right side");
            typed(&mut h, LONG);
            // The highlight is where it was, on the left and at the bottom…
            assert_eq!((left_edge(&mut h).unwrap(), bottom_edge(&mut h).unwrap()), selected, "{outline:?}");
            // …its right side is gone, where the text runs over it…
            assert_eq!(right_edge(&mut h), None, "{outline:?}: the line the text crosses is cleared");
            // …and it has not grown: its top and bottom sides end at the cell, and nothing is
            // drawn where upstream's border went, round the far end of the text.
            let (top, bottom) = (sides.top.center().y, sides.bottom.center().y);
            assert!(is_outline(pixel(&mut h, pos2(at.right() - 6.0, top))), "{outline:?}: the top side, over the cell");
            assert!(is_outline(pixel(&mut h, pos2(at.right() - 6.0, bottom))), "{outline:?}: the bottom side, under it");
            for x in [sides.right.right() + 3.0, d4.center().x, d4.right() + 6.0] {
                assert!(!is_outline(pixel(&mut h, pos2(x, top))), "{outline:?}: no top side past the cell, at {x}");
                assert!(!is_outline(pixel(&mut h, pos2(x, bottom))), "{outline:?}: no bottom side past the cell, at {x}");
            }
            let image = h.render().unwrap();
            let y = (at.center().y * PPP) as u32;
            let past = ((at.right() + 4.0) * PPP) as u32..(at.right() * PPP) as u32 + 700;
            assert!(past.clone().all(|x| !is_outline(colour(image.get_pixel(x, y).0))), "{outline:?}: no border beyond the text");
            // The gridline the text passed is cleared, and the one above the row is not.
            assert_eq!(pixel(&mut h, gridline), Color32::WHITE, "{outline:?}: the gridline under the text");
            assert!(is_gridline(pixel(&mut h, pos2(gridline.x, at.top() - 6.0))), "{outline:?}: the row above keeps its own");
            assert!(is_gridline(pixel(&mut h, pos2(d4.center().x, at.top().round() - 0.5))), "{outline:?}: the line over D4 stays");
            assert!(is_gridline(pixel(&mut h, pos2(d4.center().x, at.bottom().round() - 0.5))), "{outline:?}: and the one under it");
        }
    }

    #[test]
    fn a_neighbour_the_text_runs_over_is_covered_whole() {
        let mut a = app();
        a.session.run("cell.set", json!({"cell": "D4", "input": "WWWWWW"})).unwrap();
        let mut h = harness(a);
        let d4 = rect_of(&h, CellRef::new(3, 3));
        // What is drawn in the far half of D4, inside its gridlines.
        let ink = |h: &mut Harness| {
            let image = h.render().unwrap();
            let (x0, x1) = ((d4.center().x * PPP) as u32, (d4.right() * PPP) as u32 - 6);
            let (y0, y1) = ((d4.top() * PPP) as u32 + 4, (d4.bottom() * PPP) as u32 - 6);
            (x0..x1).flat_map(|x| (y0..y1).map(move |y| (x, y))).filter(|(x, y)| colour(image.get_pixel(*x, *y).0) != Color32::WHITE).count()
        };
        assert!(ink(&mut h) > 20, "D4 shows its own text");
        // Text in C4 that fits leaves D4 alone.
        typed(&mut h, "Hi");
        assert!(ink(&mut h) > 20);
        // A little more than fits, and nowhere near the far half of D4: D4 is covered whole all
        // the same, so none of it shows beside what is being typed.
        typed(&mut h, "llllllllllllll");
        assert_eq!(ink(&mut h), 0);
        // Leaving the editor gives D4 back.
        h.key_press(egui::Key::Escape);
        h.run_steps(3);
        assert!(h.state().editor.is_none());
        assert!(ink(&mut h) > 20);
    }
}
