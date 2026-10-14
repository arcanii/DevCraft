//! The Development window's own icon: the toolbox, drawn here dot by dot.
//!
//! The window is a window of its own (`window.rs`), and a window with no icon is given the
//! operating system's blank one. This is the same toolbox the ribbon's Development button and
//! the page's heading show, from the same points and colours
//! (`crates/ui-egui/src/aniefork/icons.rs`, and `Icon.svelte` in the page): made in code, as
//! every icon here is, so the repository holds no picture file for it.

/// The side of the picture, in dots: enough for a title bar and for the taskbar.
pub const SIZE: u32 = 64;

/// The side of the square the toolbox is drawn in, as its points are given.
const UNITS: f32 = 16.0;
/// How finely a dot is looked at: this many places each way, for a soft edge.
const FINE: u32 = 4;

const BOX: [u8; 3] = [0xD1, 0x3B, 0x2F];
const LID: [u8; 3] = [0xA8, 0x2C, 0x23];
const LATCH: [u8; 3] = [0xF6, 0xE7, 0xB8];
/// Where the lid ends and the body begins, and half the handle's thickness.
const LID_ENDS: f32 = 8.8;
const HANDLE_HALF: f32 = 0.75;
/// The handle: up from the lid, across, and down again.
const HANDLE: [(f32, f32); 4] = [(5.5, 5.5), (5.5, 3.2), (10.5, 3.2), (10.5, 5.5)];

/// Whether a point is inside a rectangle whose corners are rounded.
fn in_round(x: f32, y: f32, (left, top, right, bottom): (f32, f32, f32, f32), radius: f32) -> bool {
    if x < left || x > right || y < top || y > bottom {
        return false;
    }
    let (near_x, near_y) = (x.clamp(left + radius, right - radius), y.clamp(top + radius, bottom - radius));
    (x - near_x).powi(2) + (y - near_y).powi(2) <= radius * radius
}

/// How far a point is from a stroke between two points.
fn from_stroke(x: f32, y: f32, (ax, ay): (f32, f32), (bx, by): (f32, f32)) -> f32 {
    let (dx, dy) = (bx - ax, by - ay);
    let length = dx * dx + dy * dy;
    let along = if length > 0.0 { (((x - ax) * dx + (y - ay) * dy) / length).clamp(0.0, 1.0) } else { 0.0 };
    ((x - (ax + along * dx)).powi(2) + (y - (ay + along * dy)).powi(2)).sqrt()
}

/// The colour at a point of the toolbox's square, or nothing where there is no toolbox: the
/// latch over the lid and the body, and those over the handle.
fn at(x: f32, y: f32) -> Option<[u8; 3]> {
    if in_round(x, y, (6.7, 7.5, 9.3, 10.3), 0.6) {
        return Some(LATCH);
    }
    if in_round(x, y, (1.5, 5.5, 14.5, 14.0), 1.5) {
        return Some(if y <= LID_ENDS { LID } else { BOX });
    }
    HANDLE.windows(2).any(|ends| matches!(ends, [from, to] if from_stroke(x, y, *from, *to) <= HANDLE_HALF)).then_some(LID)
}

/// The toolbox as a square picture `size` dots a side: red, green, blue and how solid, a byte
/// each, row by row. Clear where there is no toolbox.
pub fn toolbox(size: u32) -> Vec<u8> {
    let mut picture = Vec::with_capacity((size as usize).saturating_mul(size as usize).saturating_mul(4));
    let unit = UNITS / size.max(1) as f32;
    for row in 0..size {
        for column in 0..size {
            let (mut sum, mut hits) = ([0u32; 3], 0u32);
            for fine_y in 0..FINE {
                for fine_x in 0..FINE {
                    let x = (column as f32 + (fine_x as f32 + 0.5) / FINE as f32) * unit;
                    let y = (row as f32 + (fine_y as f32 + 0.5) / FINE as f32) * unit;
                    if let Some(colour) = at(x, y) {
                        for (total, part) in sum.iter_mut().zip(colour) {
                            *total += u32::from(part);
                        }
                        hits += 1;
                    }
                }
            }
            match hits {
                0 => picture.extend([0, 0, 0, 0]),
                hits => picture.extend([(sum[0] / hits) as u8, (sum[1] / hits) as u8, (sum[2] / hits) as u8, (hits * 255 / (FINE * FINE)) as u8]),
            }
        }
    }
    picture
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dot(picture: &[u8], size: u32, x: u32, y: u32) -> [u8; 4] {
        let at = ((y * size + x) * 4) as usize;
        [picture[at], picture[at + 1], picture[at + 2], picture[at + 3]]
    }

    #[test]
    fn the_toolbox_is_drawn_whole_and_clear_around_it() {
        for size in [16, 32, SIZE] {
            let picture = toolbox(size);
            assert_eq!(picture.len(), (size * size * 4) as usize);
            let unit = size / 16;
            // Clear at the corners; the body, the lid, the latch and the handle each in their colour.
            for (x, y) in [(0, 0), (size - 1, 0), (0, size - 1), (size - 1, size - 1)] {
                assert_eq!(dot(&picture, size, x, y)[3], 0, "{size}: the corner at {x}, {y}");
            }
            assert_eq!(dot(&picture, size, 4 * unit, 12 * unit), [BOX[0], BOX[1], BOX[2], 255], "{size}: the body");
            assert_eq!(dot(&picture, size, 4 * unit, 7 * unit), [LID[0], LID[1], LID[2], 255], "{size}: the lid");
            assert_eq!(dot(&picture, size, 8 * unit, 9 * unit), [LATCH[0], LATCH[1], LATCH[2], 255], "{size}: the latch");
            assert!(dot(&picture, size, 8 * unit, 3 * unit)[3] > 0, "{size}: the handle's top");
            assert_eq!(dot(&picture, size, 8 * unit, 4 * unit + unit / 2)[3], 0, "{size}: under the handle is clear");
            // The same on both sides.
            for y in 0..size {
                for x in 0..size / 2 {
                    assert_eq!(dot(&picture, size, x, y), dot(&picture, size, size - 1 - x, y), "{size}: {x}, {y}");
                }
            }
        }
        // No size at all is no picture, and nothing breaks.
        assert!(toolbox(0).is_empty());
    }
}
