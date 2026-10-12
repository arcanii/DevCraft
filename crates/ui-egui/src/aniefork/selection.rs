//! How the selection is drawn, where the fork's options and fixes differ from upstream.
//!
//! - **Its outline can be thinner** (File ▸ Options ▸ Selection). Upstream draws it two points
//!   wide; Excel's reads as lighter.
//! - **It can be hidden while another window is the active one**, as Excel does: the outline,
//!   the shading of a range and the fill handle go, and come back when GridCraft is active
//!   again. The marching ants around a copied range stay, as they do in Excel.
//! - **The highlight of a cell being typed into stays where the selection's outline is.** Upstream
//!   draws the editor's border half a point further out on every side, makes the editor a few
//!   points taller than a row of the default height, and stretches the border to go round the
//!   text as it grows. Here the highlight neither moves nor grows: text that runs past the cell
//!   is laid over the cells beside it, a whole cell at a time, and clears the lines it runs
//!   over, as Excel does. This one is a fix, not an option.

use egui::{Color32, Rect, Stroke, StrokeKind, Vec2, pos2, vec2};
use gridcraft_engine::core::{CellRef, MAX_COLS, MAX_ROWS, RangeRef};
use gridcraft_engine::model::Sheet;

use crate::SheetApp;
use crate::grid::Geo;
use crate::theme::Tokens;

/// How far outside the cell's own edge the selection's outline is centred: as upstream's
/// `paint_selection` draws it.
const OUTSIDE: f32 = 0.5;
/// The room upstream's editor keeps beside its text: what the text field leaves clear at its
/// edges, so the editor grows just before its text would be cut off.
const ROOM: f32 = 14.0;
/// Where the text starts inside the editor, and the width of the caret after it.
const TEXT_AT: f32 = 2.0;
const CARET: f32 = 2.0;
/// Painting a line out takes this much beside it too: the soft edge it was drawn with.
const FRINGE: f32 = 0.5;
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

/// What the editor paints besides its own cell, in the grid's logical coordinates.
#[derive(Clone, Debug, Default, PartialEq)]
struct Cover {
    /// The cells it has grown over, inside the gridlines that bound them. Painted under the
    /// highlight.
    cells: Vec<Rect>,
    /// The sides of the highlight it runs over. Painted out.
    lines: Vec<Rect>,
}

/// A rectangle with something in it, or nothing.
fn solid(min: egui::Pos2, max: egui::Pos2) -> Option<Rect> {
    let rect = Rect::from_min_max(min, max);
    (rect.is_finite() && rect.is_positive()).then_some(rect)
}

/// What an editor of the rectangle `editor` covers around its cell at `cell`: the cells beside
/// and below, and the sides of a highlight `width` points wide.
///
/// The highlight's right side goes only once the text (`text_w` points wide) has reached it,
/// and its bottom side whenever the editor is taller than the cell. The top and left sides
/// always stay. On a right-to-left sheet the text starts at the editor's far edge, so the line
/// it has to reach is the width of the cells it has grown over away.
fn cover(cell: Rect, editor: Rect, width: f32, text_w: f32, rtl: bool) -> Cover {
    // The highlight is drawn centred `OUTSIDE` the cell's edge: this far inside it, and this far out.
    let inner = (width / 2.0 - OUTSIDE).max(0.0);
    let outer = OUTSIDE + width / 2.0;
    let wide = editor.right() > cell.right() + 0.5;
    let tall = editor.bottom() > cell.bottom() + 0.5;
    // A gridline is a point wide and ends on the whole point nearest the edge it marks: the
    // editor stops short of the ones that bound it, and they stay.
    let (right, bottom) = (editor.right().round() - 1.0, editor.bottom().round() - 1.0);
    let (beside, below) = (cell.right().round(), cell.bottom().round());
    let mut cover = Cover::default();
    if wide {
        cover.cells.extend(solid(pos2(beside, cell.top().round()), pos2(right, bottom)));
    }
    if tall {
        let to = if wide { beside } else { beside - 1.0 };
        cover.cells.extend(solid(pos2(cell.left().round(), below), pos2(to, bottom)));
    }
    let to_the_line = if rtl { editor.right() - cell.right() } else { cell.width() };
    let over = wide && TEXT_AT + text_w + CARET + inner >= to_the_line;
    if over {
        let to = if tall { cell.bottom() + outer + FRINGE } else { cell.bottom() - inner };
        cover.lines.extend(solid(pos2(cell.right() - inner - FRINGE, cell.top() + inner), pos2(cell.right() + outer + FRINGE, to)));
    }
    if tall {
        let to = if over { cell.right() + outer + FRINGE } else { cell.right() - inner };
        cover.lines.extend(solid(pos2(cell.left() + inner, cell.bottom() - inner - FRINGE), pos2(to, cell.bottom() + outer + FRINGE)));
    }
    cover
}

/// Hook of `grid::in_cell_editor`: paints the editor's background and the highlight of its cell.
///
/// `cell` is the cell's rectangle and `size` the editor's size (`editor_size`), in the grid's
/// logical coordinates: what is painted is mirrored for a right-to-left sheet here. The
/// highlight is the selection's own outline, exactly: the same place and the same weight.
pub fn paint_editor(app: &SheetApp, painter: &egui::Painter, geo: &Geo, cell: Rect, size: Vec2, text_w: f32, fill: Color32) {
    let width = app.ui.aniefork.selection_outline.width();
    let cover = cover(cell, Rect::from_min_size(cell.min, size), width, text_w, geo.rtl);
    painter.rect_filled(geo.mirror_rect(cell), 0.0, fill);
    for rect in &cover.cells {
        painter.rect_filled(geo.mirror_rect(*rect), 0.0, fill);
    }
    let stroke = Stroke::new(width, Tokens::get(painter.ctx()).sel_border);
    painter.rect_stroke(geo.mirror_rect(cell.expand(OUTSIDE)), 0.0, stroke, StrokeKind::Middle);
    for rect in &cover.lines {
        painter.rect_filled(geo.mirror_rect(*rect), 0.0, fill);
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

    fn app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        session.run("selection.set", json!({"range": "C4"})).unwrap();
        SheetApp::new(session, Default::default())
    }

    fn harness(app: SheetApp) -> Harness {
        let mut h = egui_kittest::Harness::builder().with_size(vec2(900.0, 600.0)).with_pixels_per_point(2.0).wgpu().build_ui_state(
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

    fn colour(rgba: [u8; 4]) -> Color32 {
        Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3])
    }

    /// The pixel at a point given in points, at two pixels a point.
    fn pixel(h: &mut Harness, x: f32, y: f32) -> Color32 {
        let image = h.render().unwrap();
        colour(image.get_pixel((x * 2.0) as u32, (y * 2.0) as u32).0)
    }

    /// Whether a pixel is the selection's green rather than a gridline's grey or the sheet's white.
    fn is_outline(c: Color32) -> bool {
        c.g() > c.r().saturating_add(40) && c.g() > c.b().saturating_add(25)
    }

    /// Where the outline's pixels lie along a horizontal line through the middle of the cell,
    /// near the edge at `x` points: (first, last) offsets from the edge in pixels.
    fn across(h: &mut Harness, x: f32) -> Option<(i32, i32)> {
        let at = cell(h);
        let image = h.render().unwrap();
        let y = (at.center().y * 2.0) as u32;
        let edge = (x * 2.0).round() as i32;
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
        let x = (at.center().x * 2.0) as u32;
        let edge = (at.bottom() * 2.0).round() as i32;
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
        assert_eq!(cover(cell, cell, 2.0, 30.0, false), Cover::default(), "an editor the size of its cell covers nothing else");
        // Grown over the next column, with the text still short of the line between them.
        let grown = Rect::from_min_max(cell.min, pos2(220.0, 66.0));
        let short = cover(cell, grown, 2.0, 50.0, false);
        assert_eq!(short.cells, [Rect::from_min_max(pos2(160.0, 50.0), pos2(219.0, 65.0))], "the next cell, inside its gridlines");
        assert!(short.lines.is_empty(), "the line stays until the text reaches it");
        // The text reaches the line: it goes, between the top and bottom lines, which stay.
        let over = cover(cell, grown, 2.0, 56.0, false);
        assert_eq!(over.cells, short.cells);
        let [line] = over.lines[..] else { panic!("{:?}", over.lines) };
        assert!(line.left() <= 159.5 && line.right() >= 161.5, "the whole width of the line: {line:?}");
        assert_eq!((line.top(), line.bottom()), (50.5, 65.5));
        // A thinner highlight is a thinner line to paint out, and reaches less far into the cell.
        let [hair] = cover(cell, grown, 1.0, 56.0, false).lines[..] else { panic!() };
        assert!(hair.left() <= 160.0 && hair.right() >= 161.0 && hair.width() < line.width());
        assert_eq!((hair.top(), hair.bottom()), (50.0, 66.0));
        // Taller than the cell: the bottom line goes, and the cells below are covered up to the
        // gridline at the cell's right, which stays.
        let taller = Rect::from_min_max(cell.min, pos2(160.0, 98.0));
        let down = cover(cell, taller, 2.0, 10.0, false);
        assert_eq!(down.cells, [Rect::from_min_max(pos2(100.0, 66.0), pos2(159.0, 97.0))]);
        let [bottom] = down.lines[..] else { panic!("{:?}", down.lines) };
        assert!(bottom.top() <= 65.5 && bottom.bottom() >= 67.5, "{bottom:?}");
        assert_eq!((bottom.left(), bottom.right()), (100.5, 159.5), "between the left and right lines, which stay");
        // Both ways: both lines, and the corner between them.
        let both = cover(cell, Rect::from_min_max(cell.min, pos2(220.0, 98.0)), 2.0, 200.0, false);
        assert_eq!(both.cells.len(), 2);
        let [right, bottom] = both.lines[..] else { panic!("{:?}", both.lines) };
        assert!(right.bottom() >= 67.5 && bottom.right() >= 161.5);
        // Right to left, the text starts at the editor's far edge: the line is the grown part away.
        assert!(cover(cell, grown, 2.0, 50.0, true).lines.is_empty());
        assert_eq!(cover(cell, grown, 2.0, 56.0, true).lines.len(), 1);
        let far = Rect::from_min_max(cell.min, pos2(400.0, 66.0));
        assert!(cover(cell, far, 2.0, 56.0, true).lines.is_empty(), "text that starts far off has further to come");
        assert_eq!(cover(cell, far, 2.0, 56.0, false).lines.len(), 1);
        // Numbers that make no sense paint nothing.
        for (editor, width, text_w) in [
            (Rect::from_min_max(cell.min, pos2(f32::NAN, f32::NAN)), 2.0, 60.0),
            (Rect::from_min_max(cell.min, pos2(f32::INFINITY, f32::INFINITY)), 2.0, 60.0),
            (grown, f32::NAN, f32::NAN),
            (Rect::from_min_max(cell.min, pos2(160.6, 66.0)), 2.0, 600.0),
        ] {
            let cover = cover(cell, editor, width, text_w, false);
            assert!(cover.cells.iter().chain(&cover.lines).all(|r| r.is_finite() && r.is_positive()), "{editor:?}: {cover:?}");
        }
    }

    #[test]
    fn the_selection_is_shown_at_upstreams_width_unless_the_options_say_otherwise() {
        let mut a = app();
        let ctx = egui::Context::default();
        assert_eq!(look(&a, &ctx), Look { shown: true, width: 2.0 });
        a.ui.aniefork.selection_outline = Outline::Hairline;
        assert_eq!(look(&a, &ctx), Look { shown: true, width: 1.0 });
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
        // Two pixels a point: 2, 1.5 and 1 points are 4, 3 and 2 pixels (an edge may blend).
        assert!(widths[0] >= 4 && widths[0] > widths[1] && widths[1] > widths[2] && widths[2] >= 2, "{widths:?}");
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
        let inside = pixel(&mut h, at.right() + 30.0, at.bottom() + 30.0);
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
            let x = (at.left() * 2.0 + 20.0) as u32;
            let edge = (at.top() * 2.0).round() as i32;
            let above = (-12..-1).filter(|dy| is_outline(colour(image.get_pixel(x, (edge + dy) as u32).0))).count() as f32;
            assert!(above <= outline.width() * 2.0, "{outline:?}: nothing is drawn further out than the outline ({above} px)");
        }
    }

    #[test]
    fn text_that_runs_past_the_cell_clears_the_lines_it_crosses_and_the_highlight_does_not_grow() {
        for outline in Outline::ALL {
            let mut h = harness(app());
            h.state_mut().ui.aniefork.selection_outline = outline;
            h.run_steps(2);
            let (at, d4) = (cell(&h), rect_of(&h, CellRef::new(3, 3)));
            let selected = (left_edge(&mut h).unwrap(), bottom_edge(&mut h).unwrap());
            // The gridline between D4 and E4, near the top of the row, where no letter reaches.
            let gridline = (d4.right().round() - 0.5, at.top() + 1.5);
            assert_ne!(pixel(&mut h, gridline.0, gridline.1), Color32::WHITE, "{outline:?}: a gridline to begin with");
            typed(&mut h, LONG);
            // The highlight is where it was, on the left and at the bottom…
            assert_eq!((left_edge(&mut h).unwrap(), bottom_edge(&mut h).unwrap()), selected, "{outline:?}");
            // …its right side is gone, where the text runs over it…
            assert_eq!(right_edge(&mut h), None, "{outline:?}: the line the text crosses is cleared");
            // …and it has not grown: its top line ends at the cell, and nothing is drawn where
            // upstream's border went, round the far end of the text.
            let on_top = at.top() - 0.25;
            assert!(is_outline(pixel(&mut h, at.right() - 6.0, on_top)), "{outline:?}: the top line, over the cell");
            for x in [at.right() + 6.0, d4.center().x, d4.right() + 6.0] {
                assert!(!is_outline(pixel(&mut h, x, on_top)), "{outline:?}: no top line past the cell, at {x}");
                assert!(!is_outline(pixel(&mut h, x, at.bottom() + 0.25)), "{outline:?}: no bottom line past the cell, at {x}");
            }
            let image = h.render().unwrap();
            let y = (at.center().y * 2.0) as u32;
            let past = ((at.right() + 4.0) * 2.0) as u32..(at.right() * 2.0) as u32 + 700;
            assert!(past.clone().all(|x| !is_outline(colour(image.get_pixel(x, y).0))), "{outline:?}: no border beyond the text");
            // The gridline the text passed is cleared, and the one above the row is not.
            assert_eq!(pixel(&mut h, gridline.0, gridline.1), Color32::WHITE, "{outline:?}: the gridline under the text");
            assert_ne!(pixel(&mut h, gridline.0, at.top() - 6.0), Color32::WHITE, "{outline:?}: the row above keeps its own");
            assert_ne!(pixel(&mut h, d4.center().x, at.top().round() - 0.5), Color32::WHITE, "{outline:?}: the line over D4 stays");
            assert_ne!(pixel(&mut h, d4.center().x, at.bottom().round() - 0.5), Color32::WHITE, "{outline:?}: and the one under it");
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
            let (x0, x1) = ((d4.center().x * 2.0) as u32, (d4.right() * 2.0) as u32 - 6);
            let (y0, y1) = ((d4.top() * 2.0) as u32 + 4, (d4.bottom() * 2.0) as u32 - 6);
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
