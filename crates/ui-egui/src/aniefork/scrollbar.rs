//! The column the sheet's vertical scrollbar stands in, above where its track begins.
//!
//! Upstream's grid paints the scrollbar's track from the first row that scrolls, and nothing
//! above it: beside the column headings (and beside frozen rows) the sheet's own white showed
//! through, a white square at the top of the scrollbar, plain to see in the dark theme (the
//! owner, 2026-10-19: "a graphical oddity the white square at the top of the scrollbar"). Here
//! that part of the column is painted: beside the headings as the headings are, beside frozen
//! rows as the track is.

use egui::{Painter, Rect, Stroke, pos2};

use crate::theme::Tokens;

/// Hook of the grid (`grid::show`), after the scrollbar: the scrollbar's column, `x` its left
/// and right edges, from `top` (the top of the sheet's area) to `track_top`, where the track
/// begins. The headings end at `cells_top`.
pub(crate) fn above_the_track(painter: &Painter, x: (f32, f32), top: f32, cells_top: f32, track_top: f32, t: &Tokens) {
    if x.1 <= x.0 {
        return;
    }
    let headings = Rect::from_min_max(pos2(x.0, top), pos2(x.1, cells_top.max(top)));
    if headings.height() > 0.0 {
        painter.rect_filled(headings, 0.0, t.header_bg);
        painter.line_segment([pos2(x.0, cells_top - 0.5), pos2(x.1, cells_top - 0.5)], Stroke::new(1.0, t.header_line));
    }
    let frozen = Rect::from_min_max(pos2(x.0, cells_top), pos2(x.1, track_top.max(cells_top)));
    if frozen.height() > 0.0 {
        painter.rect_filled(frozen, 0.0, t.tab_bar);
    }
}

#[cfg(test)]
mod tests {
    use egui::{Color32, vec2};

    use crate::SheetApp;
    use crate::theme::Tokens;

    /// The pixels to a point the test renders at.
    const PPP: f32 = 2.0;

    #[test]
    fn beside_the_headings_the_scrollbars_column_is_the_headings_colour_not_the_sheets_white() {
        for dark in [false, true] {
            let mut session = gridcraft_engine::Session::new();
            session.new_workbook();
            let mut app = SheetApp::new(session, Default::default());
            app.ui.dark = dark;
            app.ui.system_theme = false;
            let mut h = egui_kittest::Harness::builder().with_size(vec2(900.0, 600.0)).with_pixels_per_point(PPP).wgpu().build_ui_state(
                |ui, app: &mut SheetApp| {
                    let ctx = ui.ctx().clone();
                    app.logic(&ctx);
                    app.ui(ui);
                },
                app,
            );
            crate::theme::apply(&h.ctx, dark);
            h.run_steps(4);
            let grid = h.state().grid.rect.unwrap();
            let t = Tokens::get(&h.ctx);
            // The scrollbar's column is right of the grid; the headings are its first row.
            let doc = h.state().session.active().unwrap();
            let geo = crate::grid::Geo::new(doc.wb.active().unwrap(), grid, h.state().view().scroll);
            let at = (grid.right() + 7.0, (geo.cells.top() - 6.0).max(grid.top() + 2.0));
            let image = h.render().unwrap();
            let rgba = image.get_pixel((at.0 * PPP) as u32, (at.1 * PPP) as u32).0;
            let seen = Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
            assert_eq!(seen, t.header_bg, "dark: {dark}; at {at:?}");
            assert_ne!(seen, Color32::WHITE, "dark: {dark}");
            assert_eq!(t.dark, dark);
        }
    }
}
