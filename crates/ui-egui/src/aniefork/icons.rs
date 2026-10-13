//! The icons the fork draws for itself: a toolbox for Development, a shield for a development
//! kit, and the badges that say what a kit is made of.
//!
//! Upstream's icons are one list in `crate::icons`, which the fork leaves as it is: a variant
//! added there would meet every icon upstream adds. These are drawn here instead, in code like
//! upstream's and on the same grid of sixteen units, and are original to the fork. The
//! Development window's page draws the same two shapes, from the same points
//! (`apps/gridcraft/src/aniefork/page.rs`, `Icon.svelte`).

use egui::epaint::{PathShape, PathStroke};
use egui::{Color32, Painter, Pos2, Rect, Response, Sense, Stroke, Ui, pos2, vec2};

use crate::theme::{self, Tokens};

/// What is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Glyph {
    /// A toolbox: the Development tools.
    Toolbox,
    /// A shield: a development kit.
    Shield,
}

/// The outline of the shield, on a grid of sixteen: from the top, down the right side.
pub(super) const SHIELD: [(f32, f32); 10] =
    [(8.0, 1.5), (13.5, 3.5), (13.5, 8.0), (12.6, 10.6), (10.8, 12.8), (8.0, 14.5), (5.2, 12.8), (3.4, 10.6), (2.5, 8.0), (2.5, 3.5)];

/// The toolbox's own colours: red, as a toolbox is, with a darker lid and a pale latch.
const BOX: Color32 = Color32::from_rgb(0xD1, 0x3B, 0x2F);
const LID: Color32 = Color32::from_rgb(0xA8, 0x2C, 0x23);
const LATCH: Color32 = Color32::from_rgb(0xF6, 0xE7, 0xB8);

/// Draws `glyph` into `rect` (square-ish), with its lines in `colour`.
pub(super) fn paint(painter: &Painter, rect: Rect, glyph: Glyph, colour: Color32) {
    let side = rect.width().min(rect.height());
    let square = Rect::from_center_size(rect.center(), vec2(side, side));
    let unit = side / 16.0;
    let at = |x: f32, y: f32| -> Pos2 { pos2(square.left() + x * unit, square.top() + y * unit) };
    let width = 1.3 * unit.clamp(1.0, 2.0);
    match glyph {
        Glyph::Toolbox => {
            // The handle, then the box under it: its lid, its body, and the latch across the two.
            let handle = vec![at(5.5, 5.5), at(5.5, 3.2), at(10.5, 3.2), at(10.5, 5.5)];
            painter.add(PathShape::line(handle, PathStroke::new(width, colour)));
            painter.rect_filled(Rect::from_min_max(at(1.5, 5.5), at(14.5, 14.0)), 1.5 * unit, BOX);
            painter.rect_filled(Rect::from_min_max(at(1.5, 5.5), at(14.5, 8.8)), 1.5 * unit, LID);
            painter.rect_filled(Rect::from_min_max(at(1.5, 7.6), at(14.5, 8.8)), 0.0, LID);
            painter.rect_filled(Rect::from_min_max(at(6.7, 7.5), at(9.3, 10.3)), 0.6 * unit, LATCH);
        }
        Glyph::Shield => {
            let outline: Vec<Pos2> = SHIELD.iter().map(|(x, y)| at(*x, *y)).collect();
            painter.add(PathShape::convex_polygon(outline, colour.gamma_multiply(0.18), Stroke::new(width, colour)));
        }
    }
}

/// A badge: the name of something a kit is made of, in white capitals on the colour that
/// thing is known by. It is the look of the badges many projects carry on their front page
/// (the owner pointed at a collection of them, 2026-10-13). Those are pictures fetched from a
/// server; these are drawn, and nothing is fetched.
///
/// **The page's badges carry their subject's mark too** (the owner: "we want the logos"):
/// shapes from Simple Icons, kept in the page's own source as the fork's exception to
/// upstream's rule on iconography (`ATTRIBUTION-aniefork.md`). GridCraft's own window draws
/// the name alone: it has nothing that fills such a shape.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Badge {
    /// The word both windows know it by: one of a fixed few.
    pub code: &'static str,
    pub fill: Color32,
    pub text: Color32,
}

/// The badges there are, by the name of what a kit is made of. The page has the same colours
/// in its stylesheet, by the same codes (a test of the shell's checks there is one for each).
const BADGES: [(&str, Badge); 3] = [
    ("Python", Badge { code: "python", fill: Color32::from_rgb(0x36, 0x70, 0xA0), text: Color32::WHITE }),
    ("Svelte", Badge { code: "svelte", fill: Color32::from_rgb(0xF1, 0x41, 0x3D), text: Color32::WHITE }),
    ("C", Badge { code: "c", fill: Color32::from_rgb(0x00, 0x59, 0x9C), text: Color32::WHITE }),
];

/// The badge of anything else: grey, with its name.
const PLAIN: Badge = Badge { code: "plain", fill: Color32::from_rgb(0x55, 0x55, 0x55), text: Color32::WHITE };

/// The badge for something of this name.
pub(super) fn badge_for(name: &str) -> Badge {
    BADGES.iter().find(|(of, _)| *of == name).map_or(PLAIN, |(_, badge)| *badge)
}

/// The codes of the badges that have a colour of their own.
pub(super) fn badge_codes() -> Vec<&'static str> {
    BADGES.iter().map(|(_, badge)| badge.code).collect()
}

/// Draws the badge of `name`.
pub(super) fn badge(ui: &mut Ui, name: &str) -> Response {
    let look = badge_for(name);
    let mut job = egui::text::LayoutJob::default();
    let font = egui::FontId::new(10.5, egui::FontFamily::Name(theme::CELL_BOLD.into()));
    job.append(&name.to_uppercase(), 0.0, egui::TextFormat { font_id: font, extra_letter_spacing: 0.8, color: look.text, ..Default::default() });
    let words = ui.painter().layout_job(job);
    let (rect, response) = ui.allocate_exact_size(words.size() + vec2(14.0, 7.0), Sense::hover());
    ui.painter().rect_filled(rect, 3.0, look.fill);
    ui.painter().galley(rect.center() - words.size() / 2.0, words, look.text);
    // It is read as its name, not as the capitals it is drawn in.
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, name));
    response
}

/// A large ribbon button with one of the fork's icons: laid out as upstream's `big_button` is,
/// so that it sits among them as one of them.
pub(super) fn big_button(ui: &mut Ui, glyph: Glyph, label: &str, tip: &str) -> Response {
    let t = Tokens::get(ui.ctx());
    let language = crate::i18n::current(ui.ctx());
    let font = theme::ui_font(11.5);
    let lines: Vec<_> = language.tr(label).split('\n').map(|line| ui.painter().layout_no_wrap(line.to_string(), font.clone(), t.text)).collect();
    let text_width = lines.iter().map(|line| line.size().x).fold(0.0, f32::max);
    let (rect, response) = ui.allocate_exact_size(vec2((text_width + 12.0).max(44.0), 72.0), Sense::click());
    if response.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, 5.0, t.pressed);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 5.0, t.hover);
    }
    paint(ui.painter(), Rect::from_center_size(pos2(rect.center().x, rect.top() + 21.0), vec2(30.0, 30.0)), glyph, t.text);
    for (index, line) in lines.iter().enumerate() {
        let y = rect.top() + 46.0 + index as f32 * 13.0;
        ui.painter().galley(pos2(rect.center().x - line.size().x / 2.0, y - line.size().y / 2.0), line.clone(), t.text);
    }
    // Screen readers and tests find a button by its name, as they find upstream's.
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label.replace('\n', " ")));
    response.on_hover_text(language.tr(tip))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shield_is_one_closed_shape_inside_its_square_and_the_same_on_both_sides() {
        for (x, y) in SHIELD {
            assert!((0.0..=16.0).contains(&x) && (0.0..=16.0).contains(&y), "({x}, {y})");
        }
        // Mirrored about the middle: each point on the right has its twin on the left.
        for (x, y) in SHIELD {
            assert!(SHIELD.iter().any(|(other, level)| (other - (16.0 - x)).abs() < 0.01 && (level - y).abs() < 0.01), "({x}, {y})");
        }
    }

    #[test]
    fn what_a_kit_is_made_of_has_a_badge_and_anything_else_a_plain_one() {
        // Every kit's parts have a badge of their own colour, and the codes are all different.
        for kit in gridcraft_aniefork_dev::kit::loaded() {
            for name in kit.made_of {
                assert_ne!(badge_for(name).code, PLAIN.code, "{name}");
            }
        }
        let codes = badge_codes();
        assert_eq!(codes, ["python", "svelte", "c"]);
        // A name nobody listed is drawn all the same, in grey: it is never a class or a colour
        // of the name's own making.
        for other in ["", "python", "Rust", "<b>", "C++"] {
            assert_eq!(badge_for(other), PLAIN, "{other:?}");
        }
        // White on each of them can be read: the fill is dark enough.
        for (name, look) in BADGES {
            let light = 0.299 * f32::from(look.fill.r()) + 0.587 * f32::from(look.fill.g()) + 0.114 * f32::from(look.fill.b());
            assert!(light < 150.0, "{name}: {light}");
            assert_eq!(look.text, Color32::WHITE);
        }
    }

    #[test]
    fn both_icons_draw_at_the_sizes_they_are_used_at() {
        let ctx = egui::Context::default();
        let mut output = ctx.run_ui(Default::default(), |ui| {
            for glyph in [Glyph::Toolbox, Glyph::Shield] {
                for side in [12.0, 16.0, 30.0] {
                    paint(ui.painter(), Rect::from_min_size(pos2(0.0, 0.0), vec2(side, side)), glyph, Color32::WHITE);
                }
            }
        });
        // Something was drawn for each, and nothing here uploads what a renderer would.
        assert!(output.shapes.len() >= 6, "{}", output.shapes.len());
        output.textures_delta.clear();
    }
}
