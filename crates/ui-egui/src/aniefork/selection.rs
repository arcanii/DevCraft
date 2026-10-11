//! The selection outline glides to a new selection instead of jumping there, like the cell
//! cursor in Excel for Windows: press an arrow key or click a cell and the border slides over in
//! about a tenth of a second; extend the selection and it grows edge by edge.
//!
//! The behaviour comes from using Excel as a user; the timing and the easing curve are our own
//! estimate ([`GLIDE_SECS`]). `GRIDCRAFT_SELECTION_GLIDE_MS` overrides the duration (0 turns the
//! glide off; a long one makes it easy to look at).
//!
//! What glides and what doesn't:
//! - the selection changed (arrows, Enter, Tab, a click, a drag, Shift+arrows, the Name Box, any
//!   command): the outline of the current area, its fill and its fill handle glide from wherever
//!   they are drawn right now, so a held arrow key reads as one continuous motion;
//! - the same cells moved on screen (scrolling, zoom, a resized column or window): the outline
//!   stays glued to its cells;
//! - another sheet or workbook, an outline that wasn't drawn the frame before, a move that starts
//!   or ends off screen: it appears in place;
//! - a control-channel screenshot is on its way, or egui's animations are off (the headless
//!   harness turns them off): it appears in place, so scripted pictures never catch it in flight.
//!
//! Only painting is delayed. The selection itself, hit testing (the fill handle, the border you
//! drag) and the cell editor use the real cells at once.
//!
//! The glide is kept as each edge's distance from its target, decaying to zero, so the outline is
//! always "the cells' rectangle plus what is left of the offset", in screen points. Scrolling
//! under a gliding outline therefore carries it along, and an arrow key held at the edge of the
//! window (the sheet scrolls, the cell stays where it is on screen) doesn't make it wobble.

use egui::{Rect, pos2};
use gridcraft_engine::Selection;
use gridcraft_engine::core::RangeRef;
use gridcraft_engine::model::Sheet;

use crate::SheetApp;
use crate::grid::Geo;

/// How long a glide takes, in seconds.
pub const GLIDE_SECS: f64 = 0.12;
/// The longest glide `GRIDCRAFT_SELECTION_GLIDE_MS` can ask for.
const MAX_GLIDE_SECS: f64 = 10.0;
/// How far outside the cell area a glide may start or end (screen points).
const OFFSCREEN_SLACK: f32 = 48.0;

/// What the grid knows about the selection outline in one frame.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    /// The workbook (its uid) and sheet on screen: the outline never glides between two of them.
    pub place: (u64, usize),
    /// The current area of the selection.
    pub range: RangeRef,
    /// Where that area's cells are on screen.
    pub target: Rect,
    /// The grid's cell area on screen.
    pub view: Rect,
    /// egui's clock, in seconds.
    pub now: f64,
    /// egui's frame counter: an outline that skipped a frame doesn't glide from its stale place.
    pub frame_nr: u64,
    /// Glides may run this frame; otherwise the outline is on its cells.
    pub animate: bool,
}

/// The outline's motion between frames. Time-based, so it looks the same at any frame rate.
#[derive(Clone, Debug)]
pub struct SelectionGlide {
    secs: f64,
    /// What the outline was last drawn for.
    last: Option<((u64, usize), RangeRef)>,
    /// Where it was drawn (screen points).
    drawn: Rect,
    /// Each edge's distance from the target when the glide set off: left, top, right, bottom.
    offset: [f32; 4],
    started: f64,
    gliding: bool,
    frame_nr: u64,
}

impl Default for SelectionGlide {
    fn default() -> Self {
        SelectionGlide::with_secs(glide_secs())
    }
}

impl SelectionGlide {
    /// An outline whose glides take `secs` seconds (0 or less: it always jumps).
    pub fn with_secs(secs: f64) -> Self {
        let secs = if secs.is_finite() { secs.clamp(0.0, MAX_GLIDE_SECS) } else { GLIDE_SECS };
        SelectionGlide { secs, last: None, drawn: Rect::ZERO, offset: [0.0; 4], started: 0.0, gliding: false, frame_nr: 0 }
    }

    /// Advance to this frame; returns where to draw the outline. Asking again in the same frame
    /// (once per pane of a frozen sheet) gives the same rectangle.
    pub fn frame(&mut self, f: Frame) -> Rect {
        let fresh = f.frame_nr.wrapping_sub(self.frame_nr) <= 1;
        self.frame_nr = f.frame_nr;
        let key = Some((f.place, f.range));
        if self.last != key {
            // A new destination: set off from where the outline is drawn now (mid-glide or at rest).
            let same_sheet = self.last.is_some_and(|(place, _)| place == f.place);
            self.offset = if same_sheet && fresh { offset_between(self.drawn, f.target, f.view) } else { [0.0; 4] };
            self.started = f.now;
            self.last = key;
        }
        if !f.animate {
            self.offset = [0.0; 4];
        }
        // Arrived when the time is up (or the clock went backwards).
        let progress = (f.now - self.started) / self.secs;
        self.gliding = self.offset != [0.0; 4] && (0.0..1.0).contains(&progress);
        let left = if self.gliding { 1.0 - ease_out(progress as f32) } else { 0.0 };
        let [l, t, r, b] = self.offset.map(|o| o * left);
        self.drawn = Rect::from_min_max(pos2(f.target.min.x + l, f.target.min.y + t), pos2(f.target.max.x + r, f.target.max.y + b));
        self.drawn
    }

    /// The outline is on its way somewhere: keep repainting.
    pub fn is_gliding(&self) -> bool {
        self.gliding
    }

    /// Where the outline was drawn last frame (screen points).
    pub fn drawn(&self) -> Rect {
        self.drawn
    }
}

/// Decelerating curve: quick off the mark, soft landing.
fn ease_out(t: f32) -> f32 {
    let r = 1.0 - t.clamp(0.0, 1.0);
    1.0 - r * r * r
}

/// Each edge's distance from `to` back to `from`. No glide (all zero) when either end is off
/// screen or the numbers are not sane. An edge sets off at most one view away, so the far edge of
/// a whole column or row slides in from the border of the window instead of popping in at the end.
fn offset_between(from: Rect, to: Rect, view: Rect) -> [f32; 4] {
    let seen = view.expand(OFFSCREEN_SLACK);
    if !(seen.intersects(from) && seen.intersects(to)) {
        return [0.0; 4];
    }
    let limit = |v: f32| if v.is_finite() { v.abs() } else { 0.0 };
    let (lx, ly) = (limit(view.width()), limit(view.height()));
    let offset = [
        (from.min.x - to.min.x).clamp(-lx, lx),
        (from.min.y - to.min.y).clamp(-ly, ly),
        (from.max.x - to.max.x).clamp(-lx, lx),
        (from.max.y - to.max.y).clamp(-ly, ly),
    ];
    if offset.iter().all(|o| o.is_finite()) { offset } else { [0.0; 4] }
}

/// The glide's duration: [`GLIDE_SECS`], or `GRIDCRAFT_SELECTION_GLIDE_MS` when set (tests ignore it).
fn glide_secs() -> f64 {
    if cfg!(test) {
        return GLIDE_SECS;
    }
    static SECS: std::sync::OnceLock<f64> = std::sync::OnceLock::new();
    *SECS.get_or_init(|| parse_ms(std::env::var("GRIDCRAFT_SELECTION_GLIDE_MS").ok().as_deref()))
}

fn parse_ms(value: Option<&str>) -> f64 {
    match value.and_then(|s| s.trim().parse::<f64>().ok()) {
        Some(ms) if ms.is_finite() => (ms / 1000.0).clamp(0.0, MAX_GLIDE_SECS),
        _ => GLIDE_SECS,
    }
}

fn state_id() -> egui::Id {
    egui::Id::new("aniefork.selection_glide")
}

/// The grid's hook: where to draw the outline of the selection's current area this frame (its
/// cells' rectangle at rest). `sh` is the sheet on screen.
pub fn glide(ctx: &egui::Context, app: &SheetApp, geo: &Geo, sh: &Sheet, sel: &Selection) -> Rect {
    let target = geo.range_rect(sh, sel.current());
    let Some(place) = app.view_key() else { return target };
    let frame = Frame {
        place,
        range: sel.current(),
        target,
        view: geo.cells,
        now: ctx.input(|i| i.time),
        frame_nr: ctx.cumulative_frame_nr(),
        // egui's own switch for "no animations"; and a screenshot shows the outline on its cells.
        animate: ctx.global_style().animation_time > 0.0 && !app.shots.in_flight(),
    };
    let (outline, gliding) = ctx.data_mut(|d| {
        let state = d.get_temp_mut_or_default::<SelectionGlide>(state_id());
        (state.frame(frame), state.is_gliding())
    });
    if gliding {
        ctx.request_repaint();
    }
    outline
}

/// The outline's glide state this frame (for tests and tools).
pub fn state(ctx: &egui::Context) -> SelectionGlide {
    ctx.data(|d| d.get_temp::<SelectionGlide>(state_id())).unwrap_or_default()
}

/// The grid's hook for the marching ants around a copied range: their dash offset by egui's
/// clock. Upstream steps it once per painted frame, which is steady only while nothing else
/// repaints; a gliding outline repaints every frame and would make the ants lurch forward.
pub fn ants_phase(ctx: &egui::Context) -> f32 {
    ants_phase_at(ctx.input(|i| i.time))
}

/// Half a point every 60 ms (upstream's pace at rest), wrapping with the 8-point dash pattern.
fn ants_phase_at(now: f64) -> f32 {
    let phase = (now / 0.12).rem_euclid(8.0) as f32;
    if phase.is_finite() { phase } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Vec2, vec2};
    use gridcraft_engine::core::CellRef;
    use serde_json::json;

    const PLACE: (u64, usize) = (1, 0);
    const VIEW: Rect = Rect::from_min_max(pos2(0.0, 0.0), pos2(1000.0, 600.0));
    const DT: f64 = 1.0 / 240.0;

    fn cell(row: u32, col: u32) -> RangeRef {
        RangeRef::cell(CellRef::new(row, col))
    }

    /// A cell-sized rectangle at a place on screen.
    fn at(x: f32, y: f32) -> Rect {
        Rect::from_min_size(pos2(x, y), vec2(64.0, 16.0))
    }

    fn near(a: Rect, b: Rect) -> bool {
        (a.min - b.min).length() < 1e-3 && (a.max - b.max).length() < 1e-3
    }

    /// Drives a glide the way the grid does: one frame per step, a clock, a frame counter.
    struct Rig {
        glide: SelectionGlide,
        f: Frame,
    }

    impl Rig {
        fn new(range: RangeRef, target: Rect) -> Self {
            let f = Frame { place: PLACE, range, target, view: VIEW, now: 10.0, frame_nr: 100, animate: true };
            let mut rig = Rig { glide: SelectionGlide::with_secs(GLIDE_SECS), f };
            rig.step(0.0);
            rig
        }

        /// The next frame, `dt` seconds later.
        fn step(&mut self, dt: f64) -> Rect {
            self.f.now += dt;
            self.f.frame_nr += 1;
            self.glide.frame(self.f)
        }

        /// The selection changes; the next frame shows it.
        fn select(&mut self, range: RangeRef, target: Rect) {
            self.f.range = range;
            self.f.target = target;
        }

        /// Steps until the outline rests, returning every rectangle drawn on the way.
        fn settle(&mut self, dt: f64) -> Vec<Rect> {
            let mut drawn = vec![self.step(dt)];
            while self.glide.is_gliding() && drawn.len() < 10_000 {
                drawn.push(self.step(dt));
            }
            drawn
        }
    }

    #[test]
    fn the_first_outline_is_on_its_cells() {
        let r = Rig::new(cell(0, 0), at(10.0, 10.0));
        assert_eq!(r.glide.drawn(), at(10.0, 10.0));
        assert!(!r.glide.is_gliding());
    }

    #[test]
    fn a_move_glides_and_lands_exactly() {
        let (a, b) = (at(0.0, 0.0), at(64.0, 32.0));
        let mut r = Rig::new(cell(0, 0), a);
        r.select(cell(2, 1), b);
        let drawn = r.settle(DT);
        assert_eq!(drawn.first(), Some(&a), "it sets off from where it was");
        assert_eq!(drawn.last(), Some(&b), "and lands exactly on the cells");
        assert!(drawn.len() > 20, "over about {GLIDE_SECS} s, not in one frame: {} frames", drawn.len());
        for w in drawn.windows(2) {
            assert!(w[0].min.x <= w[1].min.x && w[0].min.y <= w[1].min.y, "never backwards or past the cells: {w:?}");
            assert!(w[1].min.x <= b.min.x && w[1].min.y <= b.min.y);
            assert!((w[1].width() - 64.0).abs() < 1e-3 && (w[1].height() - 16.0).abs() < 1e-3, "it keeps its shape on the way");
        }
        // Decelerating: more than half of the way in the first third of the time.
        let third = drawn[drawn.len() / 3];
        assert!(third.min.x > 32.0, "{third:?}");
    }

    #[test]
    fn the_glide_is_the_same_at_any_frame_rate() {
        let at_time = |dt: f64, frames: usize| {
            let mut r = Rig::new(cell(0, 0), at(0.0, 0.0));
            r.select(cell(0, 5), at(320.0, 0.0));
            r.step(0.0);
            (0..frames).map(|_| r.step(dt)).last().unwrap()
        };
        // 60 ms into the glide at 50, 100 and 200 frames a second.
        let (slow, mid, fast) = (at_time(0.02, 3), at_time(0.01, 6), at_time(0.005, 12));
        assert!(near(slow, mid) && near(mid, fast), "{slow:?} {mid:?} {fast:?}");
        assert!(slow.min.x > 0.0 && slow.min.x < 320.0);
    }

    #[test]
    fn extending_the_selection_grows_it_edge_by_edge() {
        let one = at(0.0, 0.0);
        let two = Rect::from_min_size(pos2(0.0, 0.0), vec2(128.0, 16.0));
        let mut r = Rig::new(cell(0, 0), one);
        r.select(RangeRef::new(CellRef::new(0, 0), CellRef::new(0, 1)), two);
        r.step(0.0);
        let mid = r.step(GLIDE_SECS / 2.0);
        assert_eq!(mid.min, one.min, "the anchored corner stays put");
        assert!(mid.max.x > one.max.x && mid.max.x < two.max.x && mid.max.y == two.max.y, "{mid:?}");
    }

    #[test]
    fn a_new_move_takes_over_from_where_the_outline_is() {
        let mut r = Rig::new(cell(0, 0), at(0.0, 0.0));
        r.select(cell(0, 1), at(64.0, 0.0));
        r.step(0.0);
        let mid = r.step(GLIDE_SECS / 3.0);
        assert!(mid.min.x > 0.0 && mid.min.x < 64.0);
        // The next arrow key lands before the first glide is over.
        r.select(cell(0, 2), at(128.0, 0.0));
        let next = r.step(0.0);
        assert!(near(next, mid), "no jump when the destination changes: {next:?} vs {mid:?}");
        assert_eq!(r.settle(DT).last(), Some(&at(128.0, 0.0)));
    }

    #[test]
    fn the_same_cells_moving_on_screen_never_glide() {
        let mut r = Rig::new(cell(4, 0), at(0.0, 64.0));
        // Scrolled, zoomed, a column resized: same range, another rectangle.
        r.f.target = at(0.0, 16.0);
        assert_eq!(r.step(DT), at(0.0, 16.0));
        r.f.target = Rect::from_min_size(pos2(8.0, 24.0), vec2(96.0, 24.0));
        assert_eq!(r.step(DT), r.f.target);
        assert!(!r.glide.is_gliding());
        // Scrolling under a gliding outline carries it along with the sheet.
        r.select(cell(5, 0), at(0.0, 200.0));
        r.step(0.0);
        let before = r.step(GLIDE_SECS / 4.0);
        r.f.target = at(0.0, 300.0);
        let after = r.step(0.0);
        assert!(near(after, before.translate(vec2(0.0, 100.0))), "{after:?} vs {before:?}");
        // An arrow key held at the window's edge: the sheet scrolls and the next cell is drawn
        // where the last one was. Nothing to glide.
        let mut r = Rig::new(cell(30, 0), at(0.0, 580.0));
        r.select(cell(31, 0), at(0.0, 580.0));
        assert_eq!(r.step(DT), at(0.0, 580.0));
        assert!(!r.glide.is_gliding());
    }

    #[test]
    fn another_sheet_a_stale_or_off_screen_outline_appears_in_place() {
        // Another sheet, another workbook.
        let mut r = Rig::new(cell(0, 0), at(0.0, 0.0));
        r.f.place = (1, 1);
        r.select(cell(9, 9), at(500.0, 300.0));
        assert_eq!(r.step(DT), at(500.0, 300.0));
        r.f.place = (2, 1);
        r.select(cell(0, 0), at(0.0, 0.0));
        assert_eq!(r.step(DT), at(0.0, 0.0));
        assert!(!r.glide.is_gliding());
        // The outline wasn't drawn for a few frames: its old place means nothing now.
        r.f.frame_nr += 5;
        r.select(cell(0, 3), at(192.0, 0.0));
        assert_eq!(r.step(DT), at(192.0, 0.0));
        // It was scrolled out of sight, and the move brings the view back.
        let mut r = Rig::new(cell(500, 0), at(0.0, -8000.0));
        r.select(cell(501, 0), at(0.0, 300.0));
        assert_eq!(r.step(DT), at(0.0, 300.0));
        // The move goes somewhere off screen.
        r.select(cell(900, 0), at(0.0, 9000.0));
        assert_eq!(r.step(DT), at(0.0, 9000.0));
        assert!(!r.glide.is_gliding());
    }

    #[test]
    fn far_edges_set_off_no_further_than_one_view_away() {
        // A whole column: its bottom edge is millions of points below the window.
        let column = Rect::from_min_max(pos2(0.0, 0.0), pos2(64.0, 16_000_000.0));
        let mut r = Rig::new(RangeRef::cols(0, 0), column);
        r.select(cell(3, 0), at(0.0, 48.0));
        let first = r.step(DT);
        assert_eq!(first.min, column.min);
        assert_eq!(first.max.y, 64.0 + VIEW.height(), "from the window's height below, not from row 1,048,576");
        assert_eq!(r.settle(DT).last(), Some(&at(0.0, 48.0)));
        // And back: the far edge leaves the same way.
        r.select(RangeRef::cols(0, 0), column);
        assert_eq!(r.step(DT).max.y, 16_000_000.0 - VIEW.height());
        assert_eq!(r.settle(DT).last(), Some(&column));
    }

    #[test]
    fn turned_off_or_hostile_numbers_never_glide_or_panic() {
        // No animations this frame: on the cells, and a glide under way stops.
        let mut r = Rig::new(cell(0, 0), at(0.0, 0.0));
        r.f.animate = false;
        r.select(cell(0, 1), at(64.0, 0.0));
        assert_eq!(r.step(DT), at(64.0, 0.0));
        r.f.animate = true;
        r.select(cell(0, 2), at(128.0, 0.0));
        r.step(0.0);
        r.step(DT);
        assert!(r.glide.is_gliding());
        r.f.animate = false;
        assert_eq!(r.step(DT), at(128.0, 0.0));
        assert!(!r.glide.is_gliding());
        // A duration of zero (GRIDCRAFT_SELECTION_GLIDE_MS=0) jumps; nonsense means the usual one.
        for secs in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut r = Rig::new(cell(0, 0), at(0.0, 0.0));
            r.glide = SelectionGlide::with_secs(secs);
            r.step(DT);
            r.select(cell(0, 1), at(64.0, 0.0));
            let drawn = r.settle(DT);
            assert_eq!(drawn.last(), Some(&at(64.0, 0.0)), "{secs}");
            assert_eq!(drawn.len() == 1, secs.is_finite(), "{secs}: {} frames", drawn.len());
        }
        assert_eq!((parse_ms(None), parse_ms(Some("abc")), parse_ms(Some("NaN"))), (GLIDE_SECS, GLIDE_SECS, GLIDE_SECS));
        assert_eq!((parse_ms(Some("0")), parse_ms(Some(" 250 ")), parse_ms(Some("-5")), parse_ms(Some("1e12"))), (0.0, 0.25, 0.0, MAX_GLIDE_SECS));
        // Geometry and clocks that make no sense: whatever is drawn meanwhile, nothing panics,
        // and a sane selection afterwards is drawn on its cells once its glide is over.
        for v in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::MAX, f32::MIN, -1.0, 0.0] {
            for now in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0e300, 0.0] {
                let mut r = Rig::new(cell(0, 0), at(0.0, 0.0));
                r.f.view = Rect::from_min_max(pos2(v, 0.0), pos2(1000.0, v));
                r.select(cell(0, 1), Rect::from_min_max(pos2(v, v), pos2(64.0, 16.0)));
                r.step(DT);
                r.f.now = now;
                r.select(cell(0, 2), Rect::from_min_size(pos2(64.0, 0.0), Vec2::splat(v)));
                r.step(DT);
                r.step(DT);
                r.f.view = VIEW;
                r.f.now = 50.0;
                r.select(cell(0, 3), at(192.0, 0.0));
                r.step(DT);
                assert_eq!(r.step(1.0), at(192.0, 0.0), "view {v}, clock {now}");
            }
        }
        assert_eq!((ease_out(-3.0), ease_out(0.0), ease_out(1.0), ease_out(7.0)), (0.0, 0.0, 1.0, 1.0));
    }

    #[test]
    fn the_ants_march_by_the_clock() {
        assert_eq!(ants_phase_at(0.0), 0.0);
        assert!((ants_phase_at(0.06) - 0.5).abs() < 1e-4, "half a point every 60 ms");
        assert!((ants_phase_at(0.96 + 0.06) - 0.5).abs() < 1e-4, "wrapping with the dash pattern");
        for now in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -12.3, 1.0e300, 7.77] {
            let p = ants_phase_at(now);
            assert!((0.0..=8.0).contains(&p), "{now}: {p}");
        }
    }

    fn harness(dt: f64) -> egui_kittest::Harness<'static, SheetApp> {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        egui_kittest::Harness::builder().with_size(vec2(1200.0, 800.0)).with_step_dt(dt as f32).build_ui_state(
            |ui, app: &mut SheetApp| {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            },
            SheetApp::new(session, Default::default()),
        )
    }

    /// Where a cell is on screen right now.
    fn cell_rect(h: &egui_kittest::Harness<'static, SheetApp>, a1: &str) -> Rect {
        let app = h.state();
        let d = app.session.active().unwrap();
        let sh = d.wb.active().unwrap();
        Geo::new(sh, app.grid.rect.unwrap(), app.view().scroll).cell_rect(sh, CellRef::parse(a1).unwrap())
    }

    fn press(h: &mut egui_kittest::Harness<'static, SheetApp>, key: egui::Key) {
        for pressed in [true, false] {
            h.input_mut().events.push(egui::Event::Key { key, physical_key: None, pressed, repeat: false, modifiers: egui::Modifiers::NONE });
        }
    }

    /// The whole path: an arrow key in the real app glides the drawn outline across the grid.
    #[test]
    fn the_grid_outline_glides_after_a_move() {
        let mut h = harness(DT);
        h.run_steps(4);
        let (a1, b1, d6) = (cell_rect(&h, "A1"), cell_rect(&h, "B1"), cell_rect(&h, "D6"));
        assert_eq!(state(&h.ctx).drawn(), a1, "at rest the outline is on its cells");
        // The test harness turns egui's animations off, and the glide follows that switch.
        h.state_mut().run("selection.set", json!({"cell": "D6"})).unwrap();
        for _ in 0..3 {
            h.step();
            assert_eq!(state(&h.ctx).drawn(), d6, "no animations: the outline is on its cells");
        }
        h.state_mut().run("selection.set", json!({"cell": "A1"})).unwrap();
        h.step();
        h.ctx.all_styles_mut(|s| s.animation_time = 0.1);
        h.step();
        press(&mut h, egui::Key::ArrowRight);
        h.step();
        // The selection itself moves at once; only its outline takes its time.
        assert_eq!(h.state().session.active().map(|d| d.selection.active.a1()).as_deref(), Some("B1"));
        h.step();
        h.step();
        let mid = state(&h.ctx);
        assert!(mid.is_gliding(), "the outline is on its way");
        let x = mid.drawn().min.x;
        assert!(x > a1.min.x && x < b1.min.x, "drawn in between: {x}");
        for _ in 0..40 {
            h.step();
        }
        assert!(!state(&h.ctx).is_gliding(), "and comes to rest");
        assert_eq!(state(&h.ctx).drawn(), b1, "exactly on the cell");
        // Zooming moves the cell on screen but doesn't glide the outline.
        h.state_mut().run("view.zoom", json!({"percent": 150})).unwrap();
        for _ in 0..3 {
            h.step();
            assert!(!state(&h.ctx).is_gliding());
        }
        assert_ne!(state(&h.ctx).drawn(), b1);
        assert_eq!(state(&h.ctx).drawn(), cell_rect(&h, "B1"));
    }

    /// A screenshot asked for over the control channel shows the outline on its cells.
    #[test]
    fn a_screenshot_never_catches_the_outline_in_flight() {
        let mut h = harness(DT);
        h.run_steps(4);
        h.ctx.all_styles_mut(|s| s.animation_time = 0.1);
        h.step();
        let (a1, b1) = (cell_rect(&h, "A1"), cell_rect(&h, "B1"));
        let (tx, rx) = std::sync::mpsc::channel();
        h.state_mut().control_rx = Some(rx);
        let (shot, _reply) = crate::ControlRequest::new("ui.screenshot", json!({}));
        tx.send(shot).unwrap();
        press(&mut h, egui::Key::ArrowRight);
        for _ in 0..20 {
            h.step();
            let drawn = state(&h.ctx).drawn();
            assert!(drawn == a1 || drawn == b1, "{drawn:?}");
        }
        assert_eq!(state(&h.ctx).drawn(), b1);
    }
}
