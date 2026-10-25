//! Design tokens and fonts.
//!
//! Colours are our own, tuned to read like a modern desktop spreadsheet. Fonts come from egui's
//! bundled faces and the operating system at runtime: the UI uses the platform UI font, cells use
//! the best available match for the workbook font (Calibri/Carlito/Aptos/Arial…), and egui's
//! bundled Ubuntu/Hack/egui-icons and monochrome emoji faces are the last resort. A build with
//! `CRAFT_FONTS_DIR` also embeds the permitted OFL craft-fonts faces (barred Adobe/Source/Noto CJK
//! families never enter it, and a web build keeps only its explicit allowlist). Native builds opt
//! into craft-fonts' OFL LXGW Chinese face, so a host without a Han system face still draws Chinese
//! interface text; no font file is stored in this repo.
//!
//! Chrome colours belong in [`Tokens`], each with a light and a dark value; colours
//! drawn on the sheet keep their light value in both themes, because the sheet stays white. The
//! foreground/background pairs are listed in `contrast_tests`, which holds them to WCAG 2 (4.5:1
//! for text, 3:1 for glyphs) in both themes: a new token goes there too.

use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use egui::epaint::text::Blob;
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, Visuals};

/// A font from the optional craft-fonts build input (`CRAFT_FONTS_DIR` at build time; see
/// <https://github.com/storytold/craft-fonts> and its `docs/integration.md`). The slice is empty
/// when the app is built without the checkout; nothing is downloaded and no font is in this repo.
#[derive(Debug)]
pub struct CraftFont {
    pub family: &'static str,
    pub style: &'static str,
    /// ISO 15924 scripts the family is intended for, from the craft-fonts manifest.
    pub scripts: &'static [&'static str],
    pub bytes: &'static [u8],
}

mod craft_fonts {
    use super::CraftFont;
    include!(concat!(env!("OUT_DIR"), "/craft_fonts.rs"));
}

/// The light or dark value of a colour, by mode.
fn pick<T>(dark: bool, light: T, dark_value: T) -> T {
    if dark { dark_value } else { light }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub dark: bool,
    pub window: Color32,
    pub ribbon: Color32,
    pub ribbon_border: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_disabled: Color32,
    pub accent: Color32,
    pub accent_dark: Color32,
    pub accent_soft: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub separator: Color32,
    pub grid_bg: Color32,
    /// Behind the pages in Page Layout view.
    pub desk: Color32,
    pub gridline: Color32,
    pub header_bg: Color32,
    pub header_text: Color32,
    pub header_line: Color32,
    pub header_sel_bg: Color32,
    pub header_sel_text: Color32,
    pub header_all_bg: Color32,
    pub sel_fill: Color32,
    pub sel_border: Color32,
    pub input_bg: Color32,
    pub input_border: Color32,
    pub tab_bar: Color32,
    pub tab_active: Color32,
    pub status_bar: Color32,
    pub cell_text: Color32,
    pub menu_bg: Color32,
    pub shadow: Color32,
    pub danger: Color32,
    /// Background error checking: the corner triangle, and the warning button's diamond and mark.
    pub error_check_green: Color32,
    pub warning_fill: Color32,
    pub warning_ink: Color32,
    /// Info-bar (notice strip) fills, by severity.
    pub notice_warn_bg: Color32,
    pub notice_warn_border: Color32,
    pub notice_info_bg: Color32,
    pub notice_info_border: Color32,
    /// Text and glyphs drawn on an `accent` fill (primary buttons, switched-on toggles).
    pub on_accent: Color32,
    /// The category palette, indexed by [`Cat`]: the coloured parts of icons, gallery highlights.
    pub cat: [Color32; 8],
    /// The page colour inside icons (documents, cells, clipboards).
    pub paper: Color32,
    /// Reference colours for formulas edited in the window-coloured formula bar.
    pub ref_colors: [Color32; 8],
    /// Reference colours on the sheet (outlines and the in-cell editor). The sheet stays white.
    pub sheet_ref_colors: [Color32; 8],
    /// Code colours in the code editor (and the window-coloured fields it sits in).
    pub syntax: Syntax,
    /// The message box warning disc and its glyph.
    pub warning: Color32,
    pub on_warning: Color32,
    /// Controls drawn on the sheet (filter and validation buttons, object handles). They follow
    /// the sheet, so they are light in both themes.
    pub sheet_control_bg: Color32,
    pub sheet_control_border: Color32,
    pub sheet_control_text: Color32,
    /// A placeholder fill on the sheet (a picture that has not loaded).
    pub sheet_track: Color32,
}

/// The eight category colours, in the order of [`Tokens::cat`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Cat {
    Blue,
    Green,
    Orange,
    Red,
    Purple,
    Teal,
    Amber,
    Slate,
}

impl Cat {
    pub const ALL: [Cat; 8] = [Cat::Blue, Cat::Green, Cat::Orange, Cat::Red, Cat::Purple, Cat::Teal, Cat::Amber, Cat::Slate];
}

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

/// Reference colours on the white sheet, the same in both themes (the sheet does not follow the
/// theme). The light theme's formula bar is white too, so it uses them as well.
const SHEET_REF_COLORS: [Color32; 8] =
    [rgb(0x2E6FD8), rgb(0xD03C3C), rgb(0x7A4FC9), rgb(0x23803D), rgb(0xB5531A), rgb(0x16798A), rgb(0xB83B8E), rgb(0x77681A)];
/// Code colours for one background; plain text uses the surface's own text colour.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Syntax {
    pub keyword: Color32,
    pub string: Color32,
    pub number: Color32,
    pub comment: Color32,
    pub builtin: Color32,
}

const LIGHT_SYNTAX: Syntax =
    Syntax { keyword: rgb(0x0B57B3), string: rgb(0xA3361B), number: rgb(0x157A5A), comment: rgb(0x6B7280), builtin: rgb(0x7B3FB5) };
const DARK_SYNTAX: Syntax =
    Syntax { keyword: rgb(0x6FAEFF), string: rgb(0xF09A7A), number: rgb(0x7FD1A8), comment: rgb(0x9AA3AE), builtin: rgb(0xC69BF2) };
const SHEET_CONTROL_BG: Color32 = rgb(0xF3F3F3);
const SHEET_CONTROL_BORDER: Color32 = rgb(0x8A8A8A);
const SHEET_CONTROL_TEXT: Color32 = rgb(0x1F1F1F);
const SHEET_TRACK: Color32 = rgb(0xD6D6D6);

impl Tokens {
    pub fn light() -> Tokens {
        Tokens::palette(false)
    }

    pub fn dark() -> Tokens {
        Tokens::palette(true)
    }

    /// One palette for both modes: each colour is written once, with its light and dark values.
    fn palette(dark: bool) -> Tokens {
        Tokens {
            dark,
            window: pick(dark, Color32::from_rgb(0xF3, 0xF3, 0xF3), Color32::from_rgb(0x29, 0x29, 0x29)),
            ribbon: pick(dark, Color32::from_rgb(0xFF, 0xFF, 0xFF), Color32::from_rgb(0x33, 0x33, 0x33)),
            ribbon_border: pick(dark, Color32::from_rgb(0xE3, 0xE3, 0xE3), Color32::from_rgb(0x40, 0x40, 0x40)),
            text: pick(dark, Color32::from_rgb(0x24, 0x24, 0x24), Color32::from_rgb(0xEE, 0xEE, 0xEE)),
            text_dim: pick(dark, Color32::from_rgb(0x61, 0x61, 0x61), Color32::from_rgb(0xB0, 0xB0, 0xB0)),
            text_disabled: pick(dark, Color32::from_rgb(0xB0, 0xB0, 0xB0), Color32::from_rgb(0x70, 0x70, 0x70)),
            accent: pick(dark, Color32::from_rgb(0x10, 0x7C, 0x41), Color32::from_rgb(0x5C, 0xC5, 0x87)),
            accent_dark: pick(dark, Color32::from_rgb(0x0B, 0x5E, 0x31), Color32::from_rgb(0x3F, 0xA8, 0x6C)),
            accent_soft: pick(dark, Color32::from_rgb(0xDF, 0xF1, 0xE6), Color32::from_rgb(0x23, 0x45, 0x31)),
            hover: pick(dark, Color32::from_rgb(0xEB, 0xEB, 0xEB), Color32::from_rgb(0x42, 0x42, 0x42)),
            pressed: pick(dark, Color32::from_rgb(0xDD, 0xDD, 0xDD), Color32::from_rgb(0x4D, 0x4D, 0x4D)),
            separator: pick(dark, Color32::from_rgb(0xE0, 0xE0, 0xE0), Color32::from_rgb(0x48, 0x48, 0x48)),
            grid_bg: Color32::WHITE,
            desk: pick(dark, Color32::from_rgb(0xDE, 0xDE, 0xDE), Color32::from_rgb(0x3A, 0x3A, 0x3A)),
            gridline: Color32::from_rgb(0xE1, 0xE1, 0xE1),
            header_bg: pick(dark, Color32::from_rgb(0xF8, 0xF8, 0xF8), Color32::from_rgb(0x30, 0x30, 0x30)),
            header_text: pick(dark, Color32::from_rgb(0x42, 0x42, 0x42), Color32::from_rgb(0xCC, 0xCC, 0xCC)),
            header_line: pick(dark, Color32::from_rgb(0xD6, 0xD6, 0xD6), Color32::from_rgb(0x45, 0x45, 0x45)),
            header_sel_bg: pick(dark, Color32::from_rgb(0xE1, 0xE1, 0xE1), Color32::from_rgb(0x45, 0x45, 0x45)),
            header_sel_text: pick(dark, Color32::from_rgb(0x0B, 0x5E, 0x31), Color32::from_rgb(0x6F, 0xD3, 0x9A)),
            header_all_bg: pick(dark, Color32::from_rgb(0xC8, 0xE6, 0xD3), Color32::from_rgb(0x2E, 0x55, 0x3E)),
            sel_fill: pick(dark, Color32::from_rgba_unmultiplied(0x10, 0x7C, 0x41, 0x1C), Color32::from_rgba_unmultiplied(0x10, 0x7C, 0x41, 0x24)),
            sel_border: Color32::from_rgb(0x10, 0x7C, 0x41),
            input_bg: pick(dark, Color32::WHITE, Color32::from_rgb(0x1F, 0x1F, 0x1F)),
            input_border: pick(dark, Color32::from_rgb(0xD1, 0xD1, 0xD1), Color32::from_rgb(0x50, 0x50, 0x50)),
            tab_bar: pick(dark, Color32::from_rgb(0xF3, 0xF3, 0xF3), Color32::from_rgb(0x29, 0x29, 0x29)),
            tab_active: pick(dark, Color32::WHITE, Color32::from_rgb(0x3A, 0x3A, 0x3A)),
            status_bar: pick(dark, Color32::from_rgb(0xF3, 0xF3, 0xF3), Color32::from_rgb(0x29, 0x29, 0x29)),
            cell_text: Color32::BLACK,
            menu_bg: pick(dark, Color32::WHITE, Color32::from_rgb(0x30, 0x30, 0x30)),
            shadow: pick(dark, Color32::from_black_alpha(40), Color32::from_black_alpha(90)),
            danger: pick(dark, Color32::from_rgb(0xC4, 0x2B, 0x1C), Color32::from_rgb(0xFF, 0x95, 0x85)),
            error_check_green: pick(dark, Color32::from_rgb(0x1E, 0x8E, 0x3E), Color32::from_rgb(0x4C, 0xC2, 0x6B)),
            warning_fill: Color32::from_rgb(0xF2, 0xB7, 0x05),
            warning_ink: Color32::BLACK,
            notice_warn_bg: pick(dark, Color32::from_rgb(0xFF, 0xF4, 0xCE), Color32::from_rgb(0x4A, 0x3F, 0x1E)),
            notice_warn_border: pick(dark, Color32::from_rgb(0xF0, 0xC3, 0x6E), Color32::from_rgb(0x7A, 0x67, 0x30)),
            notice_info_bg: pick(dark, Color32::from_rgb(0xEA, 0xF2, 0xFB), Color32::from_rgb(0x24, 0x33, 0x44)),
            notice_info_border: pick(dark, Color32::from_rgb(0xB9, 0xD2, 0xEC), Color32::from_rgb(0x3B, 0x55, 0x72)),
            on_accent: pick(dark, Color32::WHITE, rgb(0x0F2A1A)),
            cat: pick(
                dark,
                [rgb(0x2F6FCF), rgb(0x1E8A4C), rgb(0xC8601A), rgb(0xC8372D), rgb(0x8350C8), rgb(0x13808A), rgb(0x9C6C00), rgb(0x5A6B7D)],
                [rgb(0x6EA8F7), rgb(0x5CC587), rgb(0xF0965A), rgb(0xF07B6E), rgb(0xB394EE), rgb(0x4CC7CF), rgb(0xF2C14E), rgb(0xA9B6C4)],
            ),
            paper: pick(dark, Color32::WHITE, rgb(0x454545)),
            ref_colors: pick(
                dark,
                SHEET_REF_COLORS,
                [rgb(0x6FA3F5), rgb(0xF27C7C), rgb(0xB394EE), rgb(0x6CCB85), rgb(0xF0A061), rgb(0x5CC9DA), rgb(0xE77FC4), rgb(0xD4C25A)],
            ),
            sheet_ref_colors: SHEET_REF_COLORS,
            syntax: pick(dark, LIGHT_SYNTAX, DARK_SYNTAX),
            warning: rgb(0xF2C32E),
            on_warning: rgb(0x1F1F1F),
            sheet_control_bg: SHEET_CONTROL_BG,
            sheet_control_border: SHEET_CONTROL_BORDER,
            sheet_control_text: SHEET_CONTROL_TEXT,
            sheet_track: SHEET_TRACK,
        }
    }

    pub fn get(ctx: &egui::Context) -> Tokens {
        if ctx.global_style().visuals.dark_mode { Tokens::dark() } else { Tokens::light() }
    }

    /// One colour of the category palette.
    pub fn cat(&self, c: Cat) -> Color32 {
        self.cat.get(c as usize).copied().unwrap_or(self.text)
    }

    /// The reference colour for a formula's `index`-th reference, on the bar or on the sheet.
    pub fn ref_color(&self, index: usize, on_sheet: bool) -> Color32 {
        let set = if on_sheet { &self.sheet_ref_colors } else { &self.ref_colors };
        set.get(index % set.len()).copied().unwrap_or(self.text)
    }
}

/// WCAG 2 relative luminance of an opaque colour.
pub fn luminance(c: Color32) -> f32 {
    let lin = |v: u8| {
        let v = f32::from(v) / 255.0;
        if v <= 0.040_45 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
}

/// WCAG 2 contrast ratio between two opaque colours (1 to 21).
pub fn contrast(a: Color32, b: Color32) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

pub const UI: &str = "ui";
pub const UI_BOLD: &str = "ui-bold";
pub const CELL: &str = "cell";
pub const CELL_BOLD: &str = "cell-bold";
pub const CELL_ITALIC: &str = "cell-italic";
pub const CELL_BOLD_ITALIC: &str = "cell-bold-italic";
pub const SERIF: &str = "serif";
pub const MONO: &str = "mono";

pub fn ui_font(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(UI.into()))
}
pub fn ui_bold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(UI_BOLD.into()))
}

/// The font family to use for a cell font name and weight.
///
/// When the name is an installed font whose faces were bound by [`font_definitions`], that family
/// is used; otherwise the name is bucketed to one of the built-in roles (mono/serif/cell). egui
/// panics on a family that was never bound, so the installed branch is gated on
/// [`crate::fonts::is_bound`] and never names a family egui hasn't been given.
pub fn cell_family(name: &str, bold: bool, italic: bool) -> FontFamily {
    let role = crate::fonts::role_key(name, bold, italic);
    if crate::fonts::is_bound(&role) {
        return FontFamily::Name(role.into());
    }
    let lower = name.to_ascii_lowercase();
    if lower.contains("courier") || lower.contains("mono") || lower.contains("consolas") {
        return FontFamily::Name(MONO.into());
    }
    if lower.contains("times")
        || lower.contains("georgia")
        || lower.contains("cambria")
        || lower.contains("serif") && !lower.contains("sans")
        || lower.contains("garamond")
    {
        return FontFamily::Name(SERIF.into());
    }
    FontFamily::Name(
        match (bold, italic) {
            (true, true) => CELL_BOLD_ITALIC,
            (true, false) => CELL_BOLD,
            (false, true) => CELL_ITALIC,
            (false, false) => CELL,
        }
        .into(),
    )
}

/// Width in points, at 100% zoom, of one line of cell text drawn as the grid draws it: in the
/// face [`cell_family`] picks, which on a machine without the named font is a wider or narrower
/// substitute (#1316). A family the active font set doesn't have yet (bound fonts apply from the
/// next frame) falls back to the engine's estimate rather than letting egui panic on it.
///
/// `exact` lays the text out (a galley kept until the end of the frame); otherwise the glyph
/// advances are summed without kerning, which is cheap and never narrower. This takes the
/// Context lock (`fonts_mut`): never call it, or run a command, while an egui lock is held.
pub fn cell_text_width(ctx: &egui::Context, text: &str, font: &gridcraft_engine::model::Font, exact: bool) -> f32 {
    let family = cell_family(&font.name, font.bold, font.italic);
    let id = FontId::new(font.size * 96.0 / 72.0, family);
    ctx.fonts_mut(|f| {
        f.families().contains(&id.family).then(|| {
            if exact {
                f.layout_no_wrap(text.to_owned(), id, Color32::PLACEHOLDER).size().x
            } else {
                // Half a point covers the galley's rounding of its width up to the pixel grid.
                text.chars().map(|c| f.glyph_width(&id, c)).sum::<f32>() + 0.5
            }
        })
    })
    .unwrap_or_else(|| gridcraft_engine::cmd::format::approx_text_width(text, font.size, font.bold))
}

/// Whether epaint can open face `index` of this font file and it maps characters to glyphs.
/// epaint panics (in `Context::set_fonts`, i.e. on startup) on a face it can't open, so every font
/// file we did not ship is checked first with the same parser and the same call epaint makes
/// (`skrifa::FontRef::from_index`): a corrupt file, something that is not an sfnt at all, or a
/// collection index past the end is rejected instead of crashing the app (#226). A face whose
/// character map is unreadable (a truncated file) would open but draw nothing, so it is refused
/// too and the next candidate gets its turn.
pub(crate) fn font_parses(bytes: &[u8], index: u32) -> bool {
    use skrifa::MetadataProvider;
    skrifa::FontRef::from_index(bytes, index).is_ok_and(|font| font.charmap().has_map())
}

/// Loads the first candidate that exists and parses. A file that exists but can't be parsed is
/// skipped with a warning and the next candidate is tried.
#[cfg(not(target_arch = "wasm32"))]
fn try_load(paths: &[(&str, u32)]) -> Option<FontData> {
    for (p, index) in paths {
        if let Some(blob) = font_file(p, *index) {
            return Some(FontData::from_blob(blob, *index));
        }
    }
    None
}

/// The interface and script fallback font files, each read once per run and kept as a shared
/// `Blob` for the rest of it. egui shares the blob instead of copying the file into its text
/// engine, because every one of these files is loaded by every font rebuild for as long as the
/// app runs. Only a file that parses is kept: a missing or broken one is looked for again next
/// time.
#[cfg(not(target_arch = "wasm32"))]
fn font_file(path: &str, index: u32) -> Option<Blob> {
    static FILES: std::sync::Mutex<std::collections::BTreeMap<String, Blob>> = std::sync::Mutex::new(std::collections::BTreeMap::new());
    let mut files = FILES.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(blob) = files.get(path) {
        let bytes: &[u8] = (**blob).as_ref();
        return font_parses(bytes, index).then(|| std::sync::Arc::clone(blob));
    }
    let bytes = std::fs::read(path).ok()?;
    if !font_parses(&bytes, index) {
        log::warn!("skipping font {path} (face {index}): not a font file egui can read");
        return None;
    }
    let blob: Blob = std::sync::Arc::new(bytes);
    files.insert(path.to_string(), blob.clone());
    Some(blob)
}

/// Last line of defence before [`FontDefinitions`] reach egui: drops every face epaint could not
/// parse (and its key from every family), so a bad font costs a fallback, never a crash.
pub(crate) fn drop_unparseable_fonts(fonts: &mut FontDefinitions) {
    let bad: Vec<String> = fonts.font_data.iter().filter(|(_, fd)| !font_parses(fd.bytes(), fd.index)).map(|(k, _)| k.clone()).collect();
    for key in &bad {
        log::warn!("dropping font {key:?}: egui can't parse it");
        fonts.font_data.remove(key);
    }
    if !bad.is_empty() {
        for chain in fonts.families.values_mut() {
            chain.retain(|k| !bad.contains(k));
        }
    }
}

/// egui's bundled monospace face, used as the last-resort symbol fallback (see `Hack-Regular.ttf`
/// in `epaint_default_fonts`). It is present whenever egui is built with `default_fonts`.
const HACK: &str = "Hack";

/// The permitted craft-fonts faces with their `FontDefinitions` keys, in manifest order. Empty
/// without a `CRAFT_FONTS_DIR` checkout at build time. The filter is repeated here so a stale
/// generated table can never smuggle in a barred face or a web-build face outside the allowlist.
fn craft_font_faces() -> Vec<(String, &'static CraftFont)> {
    let wasm = cfg!(target_arch = "wasm32");
    craft_fonts::CRAFT_FONTS
        .iter()
        .enumerate()
        .filter(|(_, font)| !crate::craft_fonts_policy::barred(font.family))
        .filter(|(_, font)| !wasm || crate::craft_fonts_policy::web_allowed(font.family, font.style))
        .map(|(i, font)| (format!("craft-{i}-{}-{}", font.family, font.style), font))
        .collect()
}

/// Adds the optional craft-fonts faces to `fonts` and returns their keys.
fn add_craft_fonts(fonts: &mut FontDefinitions) -> Vec<String> {
    let mut keys = Vec::new();
    for (key, font) in craft_font_faces() {
        fonts.font_data.insert(key.clone(), Arc::new(FontData::from_static(font.bytes)));
        keys.push(key);
    }
    keys
}

/// Whether the optional craft-fonts input was embedded in this build.
pub fn craft_fonts_loaded() -> bool {
    !craft_fonts::CRAFT_FONTS.is_empty()
}

/// Whether the native-only Simplified Chinese craft face (LXGW WenKai Lite) was embedded. A web
/// build or an older craft-fonts checkout without `fonts/simplified-chinese-manifest.txt` has none.
pub fn chinese_font_loaded() -> bool {
    craft_font_faces().iter().any(|(_, f)| f.family == "LXGW WenKai Lite" && f.style == "Regular")
}

/// Whether this build was made with `CRAFT_FONTS_REQUIRED=1`. Acceptance tests must fail rather
/// than skip when this is true and the faces are absent.
pub fn craft_fonts_required() -> bool {
    craft_fonts::CRAFT_FONTS_REQUIRED
}

/// A bundled-face character with the same meaning as `c`, for interface chrome whose exact symbol
/// no bundled face has (`✕`, `✓`, `⌃`). Document text still needs a real glyph; this is only a
/// display fallback and never changes the characters a command inserts or stores.
pub fn symbol_fallback(c: char) -> Option<char> {
    match c {
        '✕' => Some('×'),
        '✓' => Some('✔'),
        '⌃' => Some('^'),
        _ => None,
    }
}

/// `text` with every character the current font set cannot draw replaced by its semantic fallback
/// (see [`symbol_fallback`]). Used by chrome tooltips so a host without symbol faces never paints
/// a tofu box; it is not applied to workbook text.
pub fn available_text(ui: &egui::Ui, text: &str) -> String {
    let font = ui_font(13.0);
    text.chars().map(|c| if ui.ctx().fonts_mut(|f| f.has_glyph(&font, c)) { c } else { symbol_fallback(c).unwrap_or(c) }).collect()
}

/// The family, style and scripts of every embedded craft-fonts face (empty without the input).
pub fn craft_font_scripts() -> impl Iterator<Item = (&'static str, &'static str, &'static [&'static str])> {
    craft_font_faces().into_iter().map(|(_, f)| (f.family, f.style, f.scripts))
}

/// Whether one embedded craft-fonts face maps every non-whitespace character in `text`. Tests use
/// this to tell "the pinned fixture covers this script" from "this host has no face for it",
/// without trusting a manifest script tag alone.
pub fn craft_fonts_cover(text: &str) -> bool {
    craft_font_faces().iter().any(|(_, f)| {
        use skrifa::MetadataProvider;
        let Ok(font) = skrifa::FontRef::from_index(f.bytes, 0) else {
            return false;
        };
        let map = font.charmap();
        text.chars().filter(|c| !c.is_whitespace()).all(|c| map.map(c).is_some())
    })
}

#[cfg(target_arch = "wasm32")]
fn try_load(_paths: &[(&str, u32)]) -> Option<FontData> {
    None
}

fn home_fonts(file: &str) -> String {
    let home = std::env::var("HOME").unwrap_or_default();
    format!("{home}/Library/Fonts/{file}")
}

/// Every file in `files` under every directory in `dirs`, directory first, in the same order a
/// nested loop over the directories would add them.
fn in_dirs(dirs: &[&str], files: &[&str]) -> Vec<String> {
    dirs.iter().flat_map(|d| files.iter().map(move |f| format!("{d}/{f}"))).collect()
}

/// Candidate font files per role, in preference order.
fn candidates(role: &str) -> Vec<(String, u32)> {
    let mac_sys = "/System/Library/Fonts";
    let mac_sup = "/System/Library/Fonts/Supplemental";
    let win = std::env::var("WINDIR").map(|w| format!("{w}\\Fonts")).unwrap_or_else(|_| "C:\\Windows\\Fonts".into());
    // ChromeOS's Linux container (Crostini) mounts the host's fonts under `chromeos/` (#1762).
    let lin = ["/usr/share/fonts/truetype", "/usr/share/fonts", "/usr/local/share/fonts", "/usr/share/fonts/TTF", "/usr/share/fonts/chromeos"];
    let mut v: Vec<(String, u32)> = Vec::new();
    let mut add = |s: String, i: u32| v.push((s, i));
    match role {
        "ui" => {
            add(format!("{mac_sys}/SFNS.ttf"), 0);
            add(format!("{mac_sys}/HelveticaNeue.ttc"), 0);
            add(format!("{win}\\segoeui.ttf"), 0);
            for p in in_dirs(
                &lin,
                &["cantarell/Cantarell-Regular.otf", "noto/NotoSans-Regular.ttf", "dejavu/DejaVuSans.ttf", "liberation/LiberationSans-Regular.ttf"],
            ) {
                add(p, 0);
            }
            // Fedora keeps each family in its own package folder (#54).
            add("/usr/share/fonts/abattis-cantarell-fonts/Cantarell-Regular.otf".into(), 0);
            add("/usr/share/fonts/google-noto/NotoSans-Regular.ttf".into(), 0);
            add("/usr/share/fonts/dejavu-sans-fonts/DejaVuSans.ttf".into(), 0);
            add("/usr/share/fonts/liberation-sans-fonts/LiberationSans-Regular.ttf".into(), 0);
        }
        "ui-bold" => {
            add(format!("{mac_sys}/HelveticaNeue.ttc"), 1);
            add(format!("{win}\\segoeuib.ttf"), 0);
            for p in in_dirs(&lin, &["noto/NotoSans-Bold.ttf", "dejavu/DejaVuSans-Bold.ttf", "liberation/LiberationSans-Bold.ttf"]) {
                add(p, 0);
            }
            add("/usr/share/fonts/google-noto/NotoSans-Bold.ttf".into(), 0);
            add("/usr/share/fonts/dejavu-sans-fonts/DejaVuSans-Bold.ttf".into(), 0);
            add("/usr/share/fonts/liberation-sans-fonts/LiberationSans-Bold.ttf".into(), 0);
        }
        "cell" | "cell-bold" | "cell-italic" | "cell-bold-italic" => {
            let (carlito, calibri_win, calibri_mac, arial_mac, arial_win, lib, dejavu) = match role {
                "cell" => {
                    ("Carlito-Regular.ttf", "calibri.ttf", "Calibri.ttf", "Arial.ttf", "arial.ttf", "LiberationSans-Regular.ttf", "DejaVuSans.ttf")
                }
                "cell-bold" => (
                    "Carlito-Bold.ttf",
                    "calibrib.ttf",
                    "Calibri Bold.ttf",
                    "Arial Bold.ttf",
                    "arialbd.ttf",
                    "LiberationSans-Bold.ttf",
                    "DejaVuSans-Bold.ttf",
                ),
                "cell-italic" => (
                    "Carlito-Italic.ttf",
                    "calibrii.ttf",
                    "Calibri Italic.ttf",
                    "Arial Italic.ttf",
                    "ariali.ttf",
                    "LiberationSans-Italic.ttf",
                    "DejaVuSans-Oblique.ttf",
                ),
                _ => (
                    "Carlito-BoldItalic.ttf",
                    "calibriz.ttf",
                    "Calibri Bold Italic.ttf",
                    "Arial Bold Italic.ttf",
                    "arialbi.ttf",
                    "LiberationSans-BoldItalic.ttf",
                    "DejaVuSans-BoldOblique.ttf",
                ),
            };
            // Calibri if the user has it installed system-wide (never from an Office bundle).
            add(format!("/Library/Fonts/{calibri_mac}"), 0);
            add(home_fonts(calibri_mac), 0);
            add(format!("{win}\\{calibri_win}"), 0);
            for p in in_dirs(&lin, &[&format!("crosextra/{carlito}"), &format!("carlito/{carlito}")]) {
                add(p, 0);
            }
            // Fedora keeps each family in its own package folder, as for the UI fonts (#54).
            add(format!("/usr/share/fonts/google-carlito-fonts/{carlito}"), 0);
            add(home_fonts(carlito), 0);
            add(format!("{mac_sup}/{arial_mac}"), 0);
            add(format!("{win}\\{arial_win}"), 0);
            for p in in_dirs(&lin, &[&format!("liberation/{lib}"), &format!("dejavu/{dejavu}")]) {
                add(p, 0);
            }
            add(format!("/usr/share/fonts/liberation-sans-fonts/{lib}"), 0);
            add(format!("/usr/share/fonts/dejavu-sans-fonts/{dejavu}"), 0);
        }
        "serif" => {
            add(format!("{mac_sup}/Times New Roman.ttf"), 0);
            add(format!("{win}\\times.ttf"), 0);
            for p in in_dirs(&lin, &["liberation/LiberationSerif-Regular.ttf", "dejavu/DejaVuSerif.ttf"]) {
                add(p, 0);
            }
            add("/usr/share/fonts/liberation-serif-fonts/LiberationSerif-Regular.ttf".into(), 0);
            add("/usr/share/fonts/dejavu-serif-fonts/DejaVuSerif.ttf".into(), 0);
        }
        "mono" => {
            add(format!("{mac_sys}/SFNSMono.ttf"), 0);
            add(format!("{mac_sys}/Menlo.ttc"), 0);
            add(format!("{win}\\consola.ttf"), 0);
            for p in in_dirs(&lin, &["dejavu/DejaVuSansMono.ttf"]) {
                add(p, 0);
            }
            add("/usr/share/fonts/dejavu-sans-mono-fonts/DejaVuSansMono.ttf".into(), 0);
            // Fedora installs DejaVu Sans Mono only on request. Liberation Mono comes with LibreOffice
            // (in Workstation and KDE by default), Noto Sans Mono (variable, Regular by default) always.
            for p in in_dirs(&lin, &["liberation/LiberationMono-Regular.ttf"]) {
                add(p, 0);
            }
            add("/usr/share/fonts/liberation-mono-fonts/LiberationMono-Regular.ttf".into(), 0);
            add("/usr/share/fonts/google-noto-vf/NotoSansMono[wght].ttf".into(), 0);
        }
        _ => {}
    }
    v
}

/// Builds the font set from system fonts (with egui's defaults as fallback), with the default
/// (non-Japanese) Han order. Never reads the host locale, so tests and snapshots are stable.
pub fn font_definitions() -> FontDefinitions {
    font_definitions_for_language(crate::i18n::Language::En)
}

/// Which Han face leads the fallback chain. Both Han faces cover kana and most kanji, so this only
/// decides glyph shapes (and which face catches a glyph the other lacks); switching between two
/// languages with the same order needs no font rebuild.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HanOrder {
    /// Japanese glyph shapes first (the interface is Japanese).
    Japanese,
    /// Simplified Chinese first: the widest Han coverage, for every other interface language.
    Chinese,
}

impl HanOrder {
    pub fn of(language: crate::i18n::Language) -> Self {
        if language == crate::i18n::Language::Ja { Self::Japanese } else { Self::Chinese }
    }
}

/// Builds the font set with the Han fallback faces ordered for the interface language.
pub fn font_definitions_for_language(language: crate::i18n::Language) -> FontDefinitions {
    font_definitions_with(language, &[])
}

/// The bundled faces plus the pinned craft-fonts faces, with no system-font probing or installed
/// workbook-font binding.
///
/// Integration tests use this through their shared harness, so a minimal CI image and a developer
/// machine see the same font set. Craft-fonts remains a pinned build input rather than a repository
/// asset; without it only egui's bundled faces are available, which is the documented no-font mode.
pub fn pinned_font_definitions() -> FontDefinitions {
    font_definitions_impl(crate::i18n::Language::En, &[], false)
}

/// [`font_definitions_for_language`] that also binds the workbook fonts in `used` that are
/// installed, so cells set in them render with them; the rest fall back to the roles. The bound
/// families are recorded in [`crate::fonts`] so [`cell_family`] only names a family egui has.
pub fn font_definitions_with(language: crate::i18n::Language, used: &[String]) -> FontDefinitions {
    font_definitions_impl(language, used, true)
}

/// Builds the role font chains, script fallbacks, installed workbook fonts and optional craft-font faces, drops unparseable data and records which families were bound.
fn font_definitions_impl(language: crate::i18n::Language, used: &[String], system: bool) -> FontDefinitions {
    let mut fonts = FontDefinitions::default();
    let base_prop: Vec<String> = fonts.families.get(&FontFamily::Proportional).cloned().unwrap_or_default();
    let base_mono: Vec<String> = fonts.families.get(&FontFamily::Monospace).cloned().unwrap_or_default();
    let mut bound: Vec<String> = Vec::new();
    // The non-Latin fallback faces have to sit at the end of each chain: a Latin font owns Latin,
    // and epaint picks the first face that has the glyph, so they only catch their own scripts.
    let fallback = if system { load_fallback(&mut fonts, HanOrder::of(language)) } else { Vec::new() };
    let hack = fonts.font_data.contains_key(HACK).then_some(HACK);
    for role in [UI, UI_BOLD, CELL, CELL_BOLD, CELL_ITALIC, CELL_BOLD_ITALIC, SERIF, MONO] {
        let chain = build_role_chain(&mut fonts, role, &base_prop, &base_mono, &fallback, hack, system);
        fonts.families.insert(FontFamily::Name(role.into()), chain);
        bound.push(role.to_string());
    }
    // Installed workbook fonts, so a cell's chosen font actually renders in it.
    if system {
        for family in crate::fonts::installed_among(used) {
            bound.extend(crate::fonts::bind_family(&mut fonts, &family));
        }
    }
    // Interface text (formula bar, name box, dialogs) falls back to the cell font before the
    // script faces: whatever a cell can draw, the formula bar can draw too, even where no script
    // fallback face was found (#349: macOS UI font without Hebrew, cells in Arial with it).
    bind_cell_fallback(&mut fonts, &fallback);
    // Default proportional text in widgets uses the UI font.
    if let Some(ui) = fonts.families.get(&FontFamily::Name(UI.into())).cloned() {
        fonts.families.insert(FontFamily::Proportional, ui);
    }
    // The optional craft-fonts faces go at the very end of every family: a system or bundled face
    // that has the script keeps its glyph shapes, while the permitted OFL faces cover scripts and
    // symbols the host image lacks (Japanese, Korean, Thai, Arabic, Georgian and the UI symbols).
    // Chinese comes from the native-only OFL LXGW face when embedded; a system Han face still
    // leads, and the barred Noto CJK collection never enters.
    append_craft_faces(&mut fonts);
    drop_unparseable_fonts(&mut fonts);
    // Only families egui still has data for count as bound (a dropped face can empty a chain).
    bound.retain(|f| fonts.families.get(&FontFamily::Name(f.as_str().into())).is_some_and(|c| !c.is_empty()));
    crate::fonts::set_bound(bound);
    fonts
}

/// Builds one role family's chain: the loaded system face (or its role fallback), egui's bundled
/// proportional/monospace base, Hack's symbols, then the script fallbacks. Keys without font data
/// are dropped, as before.
fn build_role_chain(
    fonts: &mut FontDefinitions,
    role: &str,
    base_prop: &[String],
    base_mono: &[String],
    fallback: &[String],
    hack: Option<&str>,
    system: bool,
) -> Vec<String> {
    let paths = candidates(role);
    let refs: Vec<(&str, u32)> = paths.iter().map(|(p, i)| (p.as_str(), *i)).collect();
    let mut chain: Vec<String> = Vec::new();
    let loaded = if system { try_load(&refs) } else { None };
    if let Some(mut fd) = loaded {
        // Helvetica Neue Bold's own TrueType hints close Cyrillic е into ө at 13 pt on a Retina
        // screen (#1184). Core Text never runs them, so this draws it the way macOS does.
        if cfg!(target_os = "macos") && role == UI_BOLD {
            fd.tweak.hinting = Some(false);
        }
        let key = format!("sys-{role}");
        fonts.font_data.insert(key.clone(), Arc::new(fd));
        chain.push(key);
    } else if role == UI_BOLD {
        chain.push(format!("sys-{UI}"));
    } else if role.starts_with("cell-") {
        chain.push(format!("sys-{CELL}"));
    }
    chain.extend(if role == MONO { base_mono.to_vec() } else { base_prop.to_vec() });
    // egui bundles Hack, a monospace face rich in arrows, triangles and keyboard symbols, but
    // 0.37 only puts it in the Monospace family. The interface draws ↑, ▲, ▼, ▸, ▾ and ↳ as
    // text (filter menus, ribbon, keytips), so add it behind the UI/cell and emoji faces; it
    // only catches characters those do not have.
    if let Some(hack) = hack
        && !chain.iter().any(|k| k.as_str() == hack)
    {
        chain.push(hack.to_string());
    }
    chain.extend(fallback.iter().cloned());
    chain.retain(|k| fonts.font_data.contains_key(k));
    chain
}

/// Inserts the loaded cell face before the script fallbacks of the interface families, so the
/// formula bar and dialogs can draw whatever a cell can (#349).
fn bind_cell_fallback(fonts: &mut FontDefinitions, fallback: &[String]) {
    let cell_key = format!("sys-{CELL}");
    if !fonts.font_data.contains_key(&cell_key) {
        return;
    }
    for role in [UI, UI_BOLD, SERIF, MONO] {
        if let Some(chain) = fonts.families.get_mut(&FontFamily::Name(role.into()))
            && !chain.contains(&cell_key)
        {
            let at = chain.iter().position(|k| fallback.contains(k)).unwrap_or(chain.len());
            chain.insert(at, cell_key.clone());
        }
    }
}

/// Appends the permitted craft-fonts faces to every family, preserving the leading system or
/// bundled face's glyph shapes.
fn append_craft_faces(fonts: &mut FontDefinitions) {
    let craft = add_craft_fonts(fonts);
    if craft.is_empty() {
        return;
    }
    for chain in fonts.families.values_mut() {
        for key in &craft {
            if !chain.contains(key) {
                chain.push(key.clone());
            }
        }
    }
}

/// Loads the system non-Latin fallback faces (one file per script family) and returns their keys so
/// every family can carry them as fallback. egui's bundled fonts have no coverage beyond Latin, so
/// without this a Thai, Hebrew, Japanese or Chinese workbook shows tofu boxes even though the text
/// was read correctly. With a face installed those scripts get real glyphs instead of tofu; nothing
/// is bundled, and a machine without the face keeps the old behaviour.
#[cfg(not(target_arch = "wasm32"))]
fn load_fallback(fonts: &mut FontDefinitions, han: HanOrder) -> Vec<String> {
    let mut keys = Vec::new();
    // Files already loaded as a fallback face. A file serves at most one key: the region-merged
    // Noto Sans CJK .ttc found for Han also carries Hangul (and Tahoma carries both Thai and
    // Hebrew), so loading it again for a later script would only duplicate tens of MB in memory.
    let mut loaded: Vec<String> = Vec::new();
    for (key, cands) in fallback_faces(han) {
        for (path, index) in &cands {
            if loaded.contains(path) {
                break; // a face loaded for an earlier script already covers this one
            }
            if let Some(fd) = try_load(&[(path.as_str(), *index)]) {
                fonts.font_data.insert(key.to_string(), Arc::new(fd));
                keys.push(key.to_string());
                loaded.push(path.clone());
                break;
            }
        }
    }
    keys
}

#[cfg(target_arch = "wasm32")]
fn load_fallback(_fonts: &mut FontDefinitions, _han: HanOrder) -> Vec<String> {
    Vec::new()
}

/// Every fallback face we look for, in chain order: `(key, candidate files)`. One file per script
/// family is enough — epaint picks the first face with the glyph, and a script's glyphs live in one
/// file. Order only matters between faces that both cover a glyph (Japanese vs Chinese kanji), so
/// the interface language reorders the two Han entries and nothing else.
#[cfg(not(target_arch = "wasm32"))]
fn fallback_faces(han: HanOrder) -> Vec<(&'static str, Vec<(String, u32)>)> {
    let ja = ("sys-cjk-han-ja", cjk_han_japanese());
    let zh = ("sys-cjk-han-zh", cjk_han_simplified());
    let (first, second) = match han {
        HanOrder::Japanese => (ja, zh),
        HanOrder::Chinese => (zh, ja),
    };
    vec![
        first,
        second,
        ("sys-cjk-hangul", cjk_hangul()),
        ("sys-thai", thai()),
        ("sys-lao", lao()),
        ("sys-hebrew", hebrew()),
        ("sys-symbols", symbols()),
        ("sys-symbols-2", symbols_more()),
    ]
}

/// Linux directories that hold CJK fonts across distributions: Debian/Ubuntu (`opentype/noto`,
/// `truetype/wqy`), Fedora (`google-noto-cjk`), Arch (`noto-cjk`, `wenquanyi`), ChromeOS's Linux
/// container (`chromeos/notocjk`, #1762), plus local and
/// flat layouts. Flatpak exposes the same host layouts through read-only font mounts.
#[cfg(not(target_arch = "wasm32"))]
const LINUX_CJK_DIRS: [&str; 22] = [
    // Flatpak mounts host fonts here without a broad filesystem permission. Keep the same
    // distribution layouts as below; /usr/share/fonts inside the sandbox belongs to the runtime.
    "/run/host/fonts/opentype/noto",
    "/run/host/fonts/google-noto-cjk",
    "/run/host/fonts/noto-cjk",
    "/run/host/fonts/truetype/noto",
    "/run/host/fonts/noto",
    "/run/host/fonts/truetype",
    "/run/host/fonts/TTF",
    "/run/host/fonts",
    "/run/host/local-fonts",
    "/run/host/local-fonts/noto-cjk",
    "/run/host/user-fonts",
    "/usr/share/fonts/opentype/noto",
    "/usr/share/fonts/google-noto-cjk",
    "/usr/share/fonts/noto-cjk",
    "/usr/share/fonts/truetype/noto",
    "/usr/share/fonts/noto",
    "/usr/share/fonts/truetype",
    "/usr/share/fonts/TTF",
    "/usr/share/fonts",
    "/usr/local/share/fonts",
    "/usr/local/share/fonts/noto-cjk",
    "/usr/share/fonts/chromeos/notocjk",
];

/// The Windows fonts folder (`%WINDIR%\Fonts`, or the default install location).
#[cfg(not(target_arch = "wasm32"))]
fn windows_fonts() -> String {
    std::env::var("WINDIR").map(|w| format!("{w}\\Fonts")).unwrap_or_else(|_| "C:\\Windows\\Fonts".into())
}

/// The Noto Sans CJK faces for one locale (`jp`, `sc`, `kr`): the per-locale OTF, then the
/// region-merged TTC at `face`, in every Linux font directory.
#[cfg(not(target_arch = "wasm32"))]
fn noto_cjk(locale: &str, face: u32) -> Vec<(String, u32)> {
    let mut v = Vec::new();
    for d in LINUX_CJK_DIRS {
        v.push((format!("{d}/NotoSansCJK{locale}-Regular.otf"), 0));
        v.push((format!("{d}/NotoSansCJK-Regular.ttc"), face));
    }
    v
}

/// Japanese Han + Kana faces (Japanese kanji glyph shapes).
///
/// The region-merged `NotoSansCJK-Regular.ttc` holds one face per locale (JP 0, KR 1, SC 2, TC 3,
/// HK 4) with the same glyph coverage, so the index only picks glyph shapes. Whichever Han list
/// comes first loads it; [`load_fallback`] does not load the same file again for the other.
#[cfg(not(target_arch = "wasm32"))]
fn cjk_han_japanese() -> Vec<(String, u32)> {
    let mac = "/System/Library/Fonts";
    let win = windows_fonts();
    let mut v: Vec<(String, u32)> = Vec::new();
    v.push((format!("{win}\\YuGothR.ttc"), 0)); // Yu Gothic
    v.push((format!("{win}\\msgothic.ttc"), 0)); // MS Gothic
    v.push((format!("{mac}/ヒラギノ角ゴシック W3.ttc"), 0)); // Hiragino Sans
    v.extend(noto_cjk("jp", 0));
    v
}

/// Simplified Chinese Han faces. Loaded apart from the Japanese ones so a glyph one family lacks
/// (simplified-only hanzi in a Japanese font) still falls through to a real glyph.
#[cfg(not(target_arch = "wasm32"))]
fn cjk_han_simplified() -> Vec<(String, u32)> {
    let mac = "/System/Library/Fonts";
    let win = windows_fonts();
    let mut v: Vec<(String, u32)> = vec![
        (format!("{win}\\msyh.ttc"), 0),            // Microsoft YaHei
        (format!("{win}\\simsun.ttc"), 0),          // SimSun
        (format!("{win}\\simhei.ttf"), 0),          // SimHei
        (format!("{mac}/Hiragino Sans GB.ttc"), 0), // ships on every recent macOS
        (format!("{mac}/PingFang.ttc"), 0),         // older macOS only; now a downloadable asset
    ];
    v.extend(noto_cjk("sc", 2));
    for d in LINUX_CJK_DIRS {
        v.push((format!("{d}/wqy/wqy-microhei.ttc"), 0));
        v.push((format!("{d}/wenquanyi/wqy-microhei.ttc"), 0));
        v.push((format!("{d}/wqy-microhei.ttc"), 0));
    }
    v
}

/// Hangul faces (Korean). The Han faces above that come from the region-merged Noto Sans CJK file
/// already carry Hangul; [`load_fallback`] then skips this list rather than load the file twice.
#[cfg(not(target_arch = "wasm32"))]
fn cjk_hangul() -> Vec<(String, u32)> {
    let mac = "/System/Library/Fonts";
    let win = windows_fonts();
    let mut v: Vec<(String, u32)> = Vec::new();
    v.push((format!("{win}\\malgun.ttf"), 0)); // Malgun Gothic
    v.push((format!("{mac}/AppleSDGothicNeo.ttc"), 0));
    v.extend(noto_cjk("kr", 1));
    v
}

/// Thai faces. Windows ships Leelawadee UI and Tahoma/Microsoft Sans Serif (both carry Thai);
/// Linux font directories searched by the Thai and Hebrew faces, in priority order.
#[cfg(not(target_arch = "wasm32"))]
fn linux_noto_dirs() -> [&'static str; 5] {
    [
        "/usr/share/fonts/truetype/noto",
        "/usr/share/fonts/opentype/noto",
        "/usr/share/fonts/truetype",
        "/usr/share/fonts",
        "/usr/share/fonts/chromeos/noto", // ChromeOS's Linux container (#1762)
    ]
}

/// Faces for one script: Windows files under `WINDIR\Fonts` first, then macOS paths, then the
/// Noto/Linux files in every [`linux_noto_dirs`] directory, then the Fedora variable font.
#[cfg(not(target_arch = "wasm32"))]
fn script_fonts(win: &[&str], mac: &[&str], lin_files: &[&str], variable: &str) -> Vec<(String, u32)> {
    let w = windows_fonts();
    let mut v: Vec<(String, u32)> = win.iter().map(|f| (format!("{w}\\{f}"), 0)).collect();
    v.extend(mac.iter().map(|p| (p.to_string(), 0)));
    v.extend(in_dirs(&linux_noto_dirs(), lin_files).into_iter().map(|p| (p, 0)));
    v.push((variable.into(), 0));
    v
}

/// macOS has Thonburi; Linux usually has Noto Sans Thai or Garuda.
#[cfg(not(target_arch = "wasm32"))]
fn thai() -> Vec<(String, u32)> {
    script_fonts(
        // Leelawadee UI, Tahoma, Microsoft Sans Serif
        &["LeelawUI.ttf", "tahoma.ttf", "micross.ttf"],
        &["/System/Library/Fonts/Supplemental/Thonburi.ttc", "/System/Library/Fonts/SukhumvitSet.ttc"],
        &["NotoSansThai-Regular.ttf", "tlwg/Garuda.ttf", "garuda/Garuda.ttf"],
        // Fedora ships the variable Noto Sans Thai; its default instance is Regular.
        "/usr/share/fonts/google-noto-vf/NotoSansThai[wght].ttf",
    )
}

/// Lao faces. Windows ships Leelawadee UI (and DokChampa on older versions); macOS has Lao Sangam
/// MN and Lao MN; Linux usually has Noto Sans Lao or Phetsarath OT.
#[cfg(not(target_arch = "wasm32"))]
fn lao() -> Vec<(String, u32)> {
    let mac_sup = "/System/Library/Fonts/Supplemental";
    let win = windows_fonts();
    let lin = ["/usr/share/fonts/truetype/noto", "/usr/share/fonts/opentype/noto", "/usr/share/fonts/truetype", "/usr/share/fonts"];
    let mut v: Vec<(String, u32)> = vec![
        (format!("{win}\\LeelawUI.ttf"), 0), // Leelawadee UI
        (format!("{win}\\dokchamp.ttf"), 0), // DokChampa
        (format!("{mac_sup}/Lao Sangam MN.ttf"), 0),
        (format!("{mac_sup}/Lao MN.ttc"), 0),
    ];
    for d in lin {
        v.push((format!("{d}/NotoSansLao-Regular.ttf"), 0));
        v.push((format!("{d}/lao/Phetsarath_OT.ttf"), 0));
        v.push((format!("{d}/Phetsarath_OT.ttf"), 0));
    }
    v
}

/// Hebrew faces. Windows has David and FrankRuehl (and Tahoma/Courier New carry Hebrew); macOS has
/// Arial Hebrew and David; Linux usually has Noto Sans Hebrew or DejaVu Sans.
#[cfg(not(target_arch = "wasm32"))]
fn hebrew() -> Vec<(String, u32)> {
    script_fonts(
        // David, FrankRuehl, Tahoma
        &["david.ttf", "frank.ttf", "tahoma.ttf"],
        &[
            "/System/Library/Fonts/ArialHB.ttc",
            "/System/Library/Fonts/SFHebrew.ttf",
            "/System/Library/Fonts/Supplemental/Arial Hebrew.ttf",
            "/System/Library/Fonts/Supplemental/David.ttf",
            "/System/Library/Fonts/LucidaGrande.ttc",
            "/System/Library/Fonts/Lucida Grande.ttc",
        ],
        &["NotoSansHebrew-Regular.ttf", "dejavu/DejaVuSans.ttf", "freefont/FreeSans.ttf"],
        "/usr/share/fonts/google-noto-vf/NotoSansHebrew[wght].ttf",
    )
}

/// Symbol faces behind the UI font. Interface fonts such as Noto Sans and Cantarell (and egui's
/// built-in Ubuntu) lack arrows, check marks, geometric shapes and keyboard symbols (▾ ✓ ✕ ▲ ↳ ⇧),
/// which then render as empty boxes (#197, #54). DejaVu Sans covers most of them; the paths cover
/// Debian/Ubuntu, Fedora and Arch layouts. [`load_fallback`] skips the file if Hebrew already
/// loaded it.
#[cfg(not(target_arch = "wasm32"))]
fn symbols() -> Vec<(String, u32)> {
    let win = std::env::var("WINDIR").map(|w| format!("{w}\\Fonts")).unwrap_or_else(|_| "C:\\Windows\\Fonts".into());
    let mut v: Vec<(String, u32)> = vec![(format!("{win}\\seguisym.ttf"), 0), ("/System/Library/Fonts/Apple Symbols.ttf".into(), 0)];
    for d in [
        "/usr/share/fonts/truetype/dejavu",
        "/usr/share/fonts/dejavu-sans-fonts",
        "/usr/share/fonts/TTF",
        "/usr/share/fonts/dejavu",
        "/usr/share/fonts/truetype/ttf-dejavu",
        "/usr/local/share/fonts/dejavu",
        "/usr/local/share/fonts",
    ] {
        v.push((format!("{d}/DejaVuSans.ttf"), 0));
    }
    v
}

/// A second symbol face for what the first lacks (⌘ and other technical symbols): Noto Sans
/// Symbols 2 where it is installed, or Segoe UI Symbol behind Segoe UI.
#[cfg(not(target_arch = "wasm32"))]
fn symbols_more() -> Vec<(String, u32)> {
    let mut v: Vec<(String, u32)> = Vec::new();
    for d in [
        "/usr/share/fonts/noto",
        "/usr/share/fonts/truetype/noto",
        "/usr/share/fonts/opentype/noto",
        "/usr/share/fonts/google-noto-sans-symbols-2-fonts",
        "/usr/share/fonts/google-noto",
        "/usr/local/share/fonts/noto",
        "/usr/share/fonts/chromeos/noto",
    ] {
        v.push((format!("{d}/NotoSansSymbols2-Regular.ttf"), 0));
    }
    v
}

/// Whether any symbol fallback face is installed (tests skip their glyph checks without one).
pub fn symbol_face_installed() -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        symbols().iter().chain(symbols_more().iter()).any(|(p, _)| std::path::Path::new(p).exists())
    }
    #[cfg(target_arch = "wasm32")]
    {
        false
    }
}

/// Installs light and dark token visuals in both theme slots, keeps the 2-3 px widget and 8-10 px window/menu corner radii from 0.36, and sets spacing, text styles and the active theme.
pub fn apply(ctx: &egui::Context, dark: bool) {
    // Install both palettes before System can switch between the style slots.
    for theme in [egui::Theme::Light, egui::Theme::Dark] {
        let dark = theme == egui::Theme::Dark;
        let t = if dark { Tokens::dark() } else { Tokens::light() };
        let mut v = if dark { Visuals::dark() } else { Visuals::light() };
        v.panel_fill = t.window;
        v.window_fill = t.menu_bg;
        v.extreme_bg_color = t.input_bg;
        v.selection.bg_fill = t.accent_soft;
        v.selection.stroke = Stroke::new(1.0, t.accent);
        v.hyperlink_color = t.accent;
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.text);
        v.widgets.inactive.fg_stroke = Stroke::new(1.0, t.text);
        v.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        v.widgets.hovered.weak_bg_fill = t.hover;
        v.widgets.active.weak_bg_fill = t.pressed;
        v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.input_border);
        // 0.37 raises the default widget corner radii; GridCraft keeps its 0.36 shape (#8617).
        v.widgets.noninteractive.corner_radius = egui::CornerRadius::same(2);
        v.widgets.inactive.corner_radius = egui::CornerRadius::same(2);
        v.widgets.hovered.corner_radius = egui::CornerRadius::same(3);
        v.widgets.active.corner_radius = egui::CornerRadius::same(2);
        v.widgets.open.corner_radius = egui::CornerRadius::same(2);
        v.window_corner_radius = egui::CornerRadius::same(10);
        v.menu_corner_radius = egui::CornerRadius::same(8);
        v.popup_shadow = egui::epaint::Shadow { offset: [0, 4], blur: 16, spread: 0, color: t.shadow };
        ctx.set_visuals_of(theme, v);
    }
    ctx.options_mut(|o| o.fallback_theme = egui::Theme::Light);
    ctx.set_theme(if dark { egui::Theme::Dark } else { egui::Theme::Light });
    ctx.all_styles_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(6.0, 3.0);
        s.spacing.interact_size.y = 22.0;
        s.text_styles.insert(egui::TextStyle::Body, ui_font(13.0));
        s.text_styles.insert(egui::TextStyle::Button, ui_font(13.0));
        s.text_styles.insert(egui::TextStyle::Small, ui_font(11.0));
        s.text_styles.insert(egui::TextStyle::Heading, ui_bold(18.0));
        s.text_styles.insert(egui::TextStyle::Monospace, FontId::new(12.0, FontFamily::Name(MONO.into())));
    });
}

/// Model colour → egui colour.
pub fn color32(rgb: [u8; 3]) -> Color32 {
    Color32::from_rgb(rgb[0], rgb[1], rgb[2])
}

#[cfg(test)]
mod font_binding_tests {
    use super::*;

    /// The families [`font_definitions`] always binds, whatever is installed.
    fn always_bound() -> Vec<String> {
        [UI, UI_BOLD, CELL, CELL_BOLD, CELL_ITALIC, CELL_BOLD_ITALIC, SERIF, MONO].iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_name_that_is_not_an_installed_font_gets_a_built_in_role() {
        let _g = crate::fonts::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Nothing installed: every name buckets to a role that font_definitions bound.
        crate::fonts::set_bound(always_bound());
        for name in ["", "Calibri", "Courier New", "Times New Roman", "Grossly Missing Font", "\u{1F600}"] {
            for (b, i) in [(false, false), (true, false), (false, true), (true, true)] {
                let FontFamily::Name(fam) = cell_family(name, b, i) else { panic!("{name:?} should name a family") };
                assert!(crate::fonts::is_bound(&fam), "{name:?} {b} {i} → unbound {fam:?}");
            }
        }
    }

    #[test]
    fn a_bound_installed_font_is_used_by_name() {
        let _g = crate::fonts::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let role = crate::fonts::role_key("Ubuntu", true, false);
        crate::fonts::set_bound(always_bound().into_iter().chain([role.clone()]));
        assert!(matches!(cell_family("Ubuntu", true, false), FontFamily::Name(f) if *f == *role));
        // The same family in a style that wasn't bound falls back to a built-in role that is.
        let FontFamily::Name(fallback) = cell_family("Ubuntu", false, true) else { panic!("should name a family") };
        assert!(crate::fonts::is_bound(&fallback), "the fallback is a bound role");
        crate::fonts::set_bound(always_bound());
    }

    #[test]
    fn arbitrary_names_never_yield_an_unbound_family() {
        let _g = crate::fonts::TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // A workbook can hold any font name; none may name a family egui has no data for.
        crate::fonts::set_bound(always_bound());
        for seed in 0..500u32 {
            let name: String =
                (0..(seed % 40)).map(|k| char::from_u32(0x20 + (seed.wrapping_mul(31).wrapping_add(k) % 0x5F)).unwrap_or('x')).collect();
            let FontFamily::Name(fam) = cell_family(&name, seed % 2 == 0, seed % 3 == 0) else { continue };
            assert!(crate::fonts::is_bound(&fam), "{name:?} → unbound {fam:?}");
        }
    }
}

#[cfg(test)]
mod contrast_tests {
    use super::*;

    /// Body text needs 4.5:1 (WCAG AA); glyphs, icons and other graphics need 3:1.
    const TEXT: f32 = 4.5;
    const GLYPH: f32 = 3.0;

    /// Every foreground/background pair the chrome draws: ribbon, popups, dialogs, panes, formula
    /// bar, status bar, plus the colours that sit on the white sheet.
    fn pairs(t: &Tokens) -> Vec<(String, Color32, Color32, f32)> {
        let mut v: Vec<(String, Color32, Color32, f32)> = Vec::new();
        let mut add = |name: &str, fg: Color32, bg: Color32, min: f32| v.push((name.to_string(), fg, bg, min));
        let surfaces = [
            ("window", t.window),
            ("ribbon", t.ribbon),
            ("menu", t.menu_bg),
            ("input", t.input_bg),
            ("status bar", t.status_bar),
            ("tab bar", t.tab_bar),
            ("active tab", t.tab_active),
            ("hover", t.hover),
        ];
        for (name, bg) in surfaces {
            add(&format!("text on {name}"), t.text, bg, TEXT);
            add(&format!("dim text on {name}"), t.text_dim, bg, TEXT);
            add(&format!("accent glyph on {name}"), t.accent, bg, GLYPH);
            add(&format!("danger on {name}"), t.danger, bg, TEXT);
        }
        add("text on pressed", t.text, t.pressed, TEXT);
        add("accent text (File, contextual tabs) on window", t.accent, t.window, TEXT);
        for (name, bg) in [("ribbon", t.ribbon), ("menu", t.menu_bg), ("hover", t.hover), ("pressed", t.pressed)] {
            for c in Cat::ALL {
                add(&format!("{c:?} icon colour on {name}"), t.cat(c), bg, GLYPH);
            }
        }
        add("icon stroke on paper", t.text, t.paper, GLYPH);
        add("on_accent on accent", t.on_accent, t.accent, TEXT);
        add("warning glyph", t.on_warning, t.warning, TEXT);
        add("notice text (warning)", t.text, t.notice_warn_bg, TEXT);
        add("notice text (info)", t.text, t.notice_info_bg, TEXT);
        add("header text", t.header_text, t.header_bg, TEXT);
        add("selected header text", t.header_sel_text, t.header_sel_bg, TEXT);
        for (i, c) in t.ref_colors.iter().enumerate() {
            add(&format!("formula bar reference {i}"), *c, t.input_bg, TEXT);
        }
        let s = t.syntax;
        for (name, c) in [("keyword", s.keyword), ("string", s.string), ("number", s.number), ("comment", s.comment), ("builtin", s.builtin)] {
            add(&format!("code {name} on input"), c, t.input_bg, TEXT);
        }
        // On the sheet.
        add("cell text", t.cell_text, t.grid_bg, TEXT);
        for (i, c) in t.sheet_ref_colors.iter().enumerate() {
            add(&format!("sheet reference {i}"), *c, t.grid_bg, TEXT);
        }
        add("selection border on sheet", t.sel_border, t.grid_bg, GLYPH);
        add("sheet control text", t.sheet_control_text, t.sheet_control_bg, TEXT);
        add("sheet control border on sheet", t.sheet_control_border, t.grid_bg, GLYPH);
        v
    }

    #[test]
    fn token_pairs_meet_wcag_contrast_in_both_themes() {
        let mut failures = Vec::new();
        for t in [Tokens::light(), Tokens::dark()] {
            for (name, fg, bg, min) in pairs(&t) {
                let ratio = contrast(fg, bg);
                if ratio < min {
                    failures.push(format!("{} theme: {name} is {ratio:.2}:1, needs {min}:1", if t.dark { "dark" } else { "light" }));
                }
            }
        }
        assert!(failures.is_empty(), "contrast failures:\n{}", failures.join("\n"));
    }

    #[test]
    fn contrast_matches_wcag_reference_values() {
        assert!((contrast(Color32::BLACK, Color32::WHITE) - 21.0).abs() < 0.01);
        assert!((contrast(Color32::WHITE, Color32::WHITE) - 1.0).abs() < 0.001);
        // #767676 on white is the classic 4.54:1 grey.
        assert!((contrast(rgb(0x767676), Color32::WHITE) - 4.54).abs() < 0.01);
    }

    #[test]
    fn sheet_tokens_follow_the_sheet_not_the_theme() {
        let (l, d) = (Tokens::light(), Tokens::dark());
        assert_eq!(l.grid_bg, d.grid_bg);
        assert_eq!(l.sheet_ref_colors, d.sheet_ref_colors);
        assert_eq!(
            (l.sheet_control_bg, l.sheet_control_border, l.sheet_control_text, l.sheet_track),
            (d.sheet_control_bg, d.sheet_control_border, d.sheet_control_text, d.sheet_track)
        );
        for c in Cat::ALL {
            assert_ne!(l.cat(c), d.cat(c), "{c:?} has a dark-mode value of its own");
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::i18n::Language;

    #[test]
    /// Loaded script-fallback faces end each role chain just before the craft fonts, in the language's order.
    fn fallback_faces_are_appended_to_every_family() {
        // Structural: whatever loaded, the non-Latin fallback keys must sit at the end of each chain
        // (after the Latin/default fonts) so Latin still wins for Latin text, in the language's order.
        for han in [HanOrder::Chinese, HanOrder::Japanese] {
            let language = if han == HanOrder::Japanese { Language::Ja } else { Language::Zh };
            let fonts = font_definitions_for_language(language);
            let fb_keys: Vec<String> =
                fallback_faces(han).into_iter().map(|(k, _)| k.to_string()).filter(|k| fonts.font_data.contains_key(k)).collect();
            if fb_keys.is_empty() {
                return; // no fallback face on this machine (minimal Linux)
            }
            for role in [UI, UI_BOLD, CELL, CELL_BOLD, CELL_ITALIC, CELL_BOLD_ITALIC, SERIF, MONO] {
                let chain = fonts.families.get(&FontFamily::Name(role.into())).map(Vec::as_slice).unwrap_or_default();
                // The optional craft-fonts faces follow the system fallbacks at the very end.
                let end = chain.iter().rposition(|k| !k.starts_with("craft-")).map_or(0, |i| i + 1);
                assert!(end >= fb_keys.len(), "{role} chain too short for its fallback");
                assert_eq!(&chain[end - fb_keys.len()..end], fb_keys.as_slice(), "{role} must end its non-craft chain with the fallback faces");
            }
        }
    }

    #[test]
    /// Every font family ends with exactly the permitted craft faces in manifest order.
    fn craft_fonts_are_the_last_fallback_in_every_family() {
        // Ordering: app/bundled faces, Hack/emoji, system script fallbacks, permitted craft faces.
        // A craft face must never displace a system face that already covers a glyph.
        let fonts = font_definitions_for_language(Language::En);
        let expected: Vec<String> = craft_font_faces().into_iter().map(|(key, _)| key).collect();
        if expected.is_empty() {
            return; // optional input absent
        }
        for (family, chain) in &fonts.families {
            let first = chain.iter().position(|k| k.starts_with("craft-")).unwrap_or(chain.len());
            assert_eq!(&chain[first..], expected.as_slice(), "{family:?} must end with exactly the permitted craft faces in manifest order");
        }
    }

    #[test]
    /// Skips on wasm or when required mode is off; a required native build must embed LXGW with `Hani`/`Hans`.
    fn a_required_native_build_embeds_the_chinese_face() {
        if cfg!(target_arch = "wasm32") || !craft_fonts_required() {
            return;
        }
        assert!(
            chinese_font_loaded(),
            "CRAFT_FONTS_REQUIRED=1 native builds must embed the LXGW Chinese face from fonts/simplified-chinese-manifest.txt"
        );
        let face = craft_font_scripts().find(|(f, s, _)| *f == "LXGW WenKai Lite" && *s == "Regular");
        assert_eq!(face.map(|(_, _, scripts)| scripts), Some(["Hani", "Hans"].as_slice()), "the Chinese manifest's script tags are asserted");
    }

    #[test]
    /// No embedded craft face belongs to the barred Noto CJK, Adobe or Source families.
    fn barred_faces_never_enter_the_font_set() {
        for (family, _, _) in craft_font_scripts() {
            assert!(!family.contains("CJK"), "the barred Noto CJK collection must never be embedded: {family}");
            assert!(!family.starts_with("Source ") && !family.starts_with("Adobe "), "a barred Adobe/Source face was embedded: {family}");
        }
    }

    #[test]
    /// Skips when craft fonts are absent unless required mode is set; otherwise the required faces and their exact script tags are embedded.
    fn the_required_craft_faces_keep_their_exact_scripts() {
        if !craft_fonts_loaded() {
            assert!(!craft_fonts_required(), "CRAFT_FONTS_REQUIRED=1 but no craft-fonts faces were embedded");
            return;
        }
        let faces: Vec<_> = craft_font_scripts().collect();
        for (family, style, scripts) in [
            ("BIZ UDPGothic", "Regular", ["Jpan", "Latn"].as_slice()),
            ("BIZ UDPGothic", "Bold", ["Jpan", "Latn"].as_slice()),
            ("NanumGothic", "Regular", ["Hang", "Kore"].as_slice()),
            ("Noto Sans Symbols", "Regular", ["Zsym"].as_slice()),
            ("Noto Sans Symbols 2", "Regular", ["Zsym"].as_slice()),
        ] {
            assert!(
                faces.iter().any(|(f, s, declared)| *f == family && *s == style && *declared == scripts),
                "{family} {style} with {scripts:?} must be embedded; got {faces:?}"
            );
        }
    }

    #[test]
    /// Interface text maps unbundled `⌃`, `✓` and `✕` to readable chrome fallbacks; document text is not rewritten.
    fn available_text_maps_unbundled_chrome_symbols() {
        // No bundled face has ⌃ ✓ ✕; chrome text must show ^ ✔ × instead of tofu. Document text is
        // never rewritten: this helper is only used for interface tooltips.
        let ctx = egui::Context::default();
        ctx.set_fonts(FontDefinitions::default());
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            assert_eq!(available_text(ui, "Show Formulas (⌃`) ✓ ✕"), "Show Formulas (^`) ✔ ×");
        });
        out.textures_delta.clear();
    }

    #[test]
    fn interface_families_fall_back_to_the_cell_font() {
        // Structural (#349): the formula bar, name box and dialogs draw every script the cells can,
        // so each interface chain carries the cell face, ahead of the script fallback faces.
        let fonts = font_definitions_for_language(Language::En);
        let cell = format!("sys-{CELL}");
        if !fonts.font_data.contains_key(&cell) {
            return; // no system cell font on this machine
        }
        let fb: Vec<String> = fallback_faces(HanOrder::Chinese).into_iter().map(|(k, _)| k.to_string()).collect();
        for family in [FontFamily::Proportional, FontFamily::Name(UI.into()), FontFamily::Name(UI_BOLD.into()), FontFamily::Name(MONO.into())] {
            let chain = fonts.families.get(&family).cloned().unwrap_or_default();
            let at = chain.iter().position(|k| *k == cell);
            assert!(at.is_some(), "{family:?} lacks the cell face: {chain:?}");
            let first_fb = chain.iter().position(|k| fb.contains(k)).unwrap_or(chain.len());
            assert!(at.unwrap_or(usize::MAX) < first_fb, "{family:?}: cell face must precede the script faces");
        }
    }

    #[test]
    fn language_reorders_only_the_han_faces() {
        let keys = |han| fallback_faces(han).into_iter().map(|(k, _)| k).collect::<Vec<_>>();
        assert_eq!(
            keys(HanOrder::Japanese),
            ["sys-cjk-han-ja", "sys-cjk-han-zh", "sys-cjk-hangul", "sys-thai", "sys-lao", "sys-hebrew", "sys-symbols", "sys-symbols-2"]
        );
        assert_eq!(
            keys(HanOrder::Chinese),
            ["sys-cjk-han-zh", "sys-cjk-han-ja", "sys-cjk-hangul", "sys-thai", "sys-lao", "sys-hebrew", "sys-symbols", "sys-symbols-2"]
        );
        assert_eq!(HanOrder::of(Language::Ja), HanOrder::Japanese);
        for l in [Language::En, Language::Zh, Language::Ko, Language::Ru, Language::PtBr, Language::De, Language::Cs, Language::Fr, Language::Tr] {
            assert_eq!(HanOrder::of(l), HanOrder::Chinese, "{l:?} shares the default order (no font rebuild)");
        }
    }

    #[test]
    fn cjk_fallback_probes_flatpak_fonts_with_the_correct_locale_faces() {
        // Exercise the candidates the fallback loader actually consumes, not just the directory
        // constant. This must work on CI without an installed CJK font or a Flatpak runtime.
        for (candidates, face) in [(cjk_han_japanese(), 0), (cjk_han_simplified(), 2), (cjk_hangul(), 1)] {
            for directory in [
                "/run/host/fonts/opentype/noto",
                "/run/host/fonts/google-noto-cjk",
                "/run/host/fonts/noto-cjk",
                "/run/host/fonts/truetype/noto",
                "/run/host/fonts/noto",
                "/run/host/fonts/truetype",
                "/run/host/fonts/TTF",
                "/run/host/fonts",
                "/run/host/local-fonts",
                "/run/host/local-fonts/noto-cjk",
                "/run/host/user-fonts",
            ] {
                let path = format!("{directory}/NotoSansCJK-Regular.ttc");
                assert!(candidates.contains(&(path.clone(), face)), "missing Flatpak CJK candidate {path}, face {face}");
            }
        }
        let chinese = cjk_han_simplified();
        assert!(chinese.contains(&("/run/host/fonts/truetype/wqy/wqy-microhei.ttc".into(), 0)));
        assert!(chinese.contains(&("/run/host/fonts/wenquanyi/wqy-microhei.ttc".into(), 0)));
    }

    #[test]
    /// A font file shared by several fallback scripts is loaded once, with one shared font blob rather than a copy per script.
    fn fallback_faces_load_each_file_once() {
        // The region-merged Noto Sans CJK .ttc serves Japanese, Chinese and Hangul; it must be loaded
        // once, not once per script (tens of MB each).
        for language in [Language::En, Language::Ja] {
            let fonts = font_definitions_for_language(language);
            let present: Vec<&Arc<FontData>> =
                fallback_faces(HanOrder::of(language)).into_iter().filter_map(|(k, _)| fonts.font_data.get(k)).collect();
            for (i, a) in present.iter().enumerate() {
                for b in present.iter().skip(i + 1) {
                    assert!(!std::sync::Arc::ptr_eq(&a.font, &b.font), "the same font file is loaded for two fallback scripts");
                }
            }
        }
    }

    #[test]
    /// Checks that the proportional and cell families have glyphs for every present script sample; samples with no system or craft face are printed as skipped.
    fn fallback_covers_its_script() {
        // Real glyph check, per script: the proportional family (what cells and the UI use) must
        // have a glyph for every non-space character in the sample. A script runs when a system
        // face or the optional craft-fonts input provides one; the printed skip is a recorded
        // gate, and a present face that is not wired in fails.
        let cases: [(&str, Vec<(String, u32)>, &str); 10] = [
            ("sys-cjk-han-ja", cjk_han_japanese(), "カテゴリ"),
            // Chinese uses a system Han face or the native-only permitted LXGW craft face; the
            // barred Noto CJK collection never supplies it.
            ("sys-cjk-han-zh", cjk_han_simplified(), "页面布局 客户端配置"),
            ("sys-cjk-hangul", cjk_hangul(), "한글"),
            ("sys-thai", thai(), "ทดสอบ"),
            ("sys-lao", lao(), "ພາສາລາວ"),
            ("sys-hebrew", hebrew(), "עברית"),
            ("craft-noto-sans-arabic", Vec::new(), "مرحبا"),
            ("craft-noto-sans-georgian", Vec::new(), "ქართული"),
            ("ui", candidates(UI), "Русский"),
            // Czech interface labels: every Czech accented letter, in both cases.
            ("ui", candidates(UI), "Příliš žluťoučký kůň úpěl ďábelské ódy ŘŠČŽÝÁÍÉÚŮĚŤĎŇÓ"),
        ];
        for language in [Language::En, Language::Ja] {
            let ctx = egui::Context::default();
            ctx.set_fonts(font_definitions_for_language(language));
            let mut out = ctx.run_ui(egui::RawInput::default(), |_ui| {});
            out.textures_delta.clear(); // nothing consumes the atlas in a headless test
            for family in [FontFamily::Proportional, FontFamily::Name(CELL.into())] {
                let font = FontId::new(14.0, family.clone());
                for (key, cands, sample) in &cases {
                    // A script runs when a system face or the optional craft-fonts input provides
                    // one. The skip is printed: a missing fixture is a recorded gate, not a pass.
                    let system = cands.iter().any(|(p, _)| std::path::Path::new(p).exists());
                    if !system && !craft_fonts_cover(sample) {
                        eprintln!("skipped {key}: no system face and no craft-fonts face for {sample:?}");
                        continue;
                    }
                    let missing: String =
                        sample.chars().filter(|c| !c.is_whitespace()).filter(|&c| !ctx.fonts_mut(|f| f.has_glyph(&font, c))).collect();
                    assert!(missing.is_empty(), "{key} is present but {sample:?} has no glyph for {missing:?} ({language:?}, {family:?})");
                }
            }
        }
    }

    #[test]
    fn chromeos_font_layout_is_probed() {
        // #1762: Crostini mounts the host's fonts under /usr/share/fonts/chromeos, one folder per
        // family set; a script whose list misses it renders as tofu with the font installed.
        let probes = |cands: Vec<(String, u32)>, path: &str| cands.iter().any(|(p, _)| p == path);
        let cros = "/usr/share/fonts/chromeos";
        assert!(probes(thai(), &format!("{cros}/noto/NotoSansThai-Regular.ttf")));
        assert!(probes(hebrew(), &format!("{cros}/noto/NotoSansHebrew-Regular.ttf")));
        assert!(probes(cjk_han_japanese(), &format!("{cros}/notocjk/NotoSansCJK-Regular.ttc")));
        assert!(probes(cjk_han_simplified(), &format!("{cros}/notocjk/NotoSansCJK-Regular.ttc")));
        assert!(probes(cjk_hangul(), &format!("{cros}/notocjk/NotoSansCJK-Regular.ttc")));
        assert!(probes(symbols_more(), &format!("{cros}/noto/NotoSansSymbols2-Regular.ttf")));
        assert!(probes(candidates(UI), &format!("{cros}/noto/NotoSans-Regular.ttf")));
        assert!(probes(candidates(CELL), &format!("{cros}/crosextra/Carlito-Regular.ttf")));
    }

    #[test]
    /// Empty, garbage, truncated and bad-index fonts are refused; a bad face is dropped from the definitions and `set_fonts` does not panic.
    fn unparseable_font_files_are_rejected_not_fatal() {
        // #226: epaint panics in `set_fonts` on a face it can't parse ("An offset was out of
        // bounds"). Garbage, a truncated real font and a bad collection index must all be refused.
        let real = FontDefinitions::default().font_data.values().next().map(|fd| fd.bytes().to_vec()).unwrap_or_default();
        assert!(font_parses(&real, 0), "egui's own font must parse");
        assert!(!font_parses(b"", 0));
        assert!(!font_parses(&[0xAB; 64], 0));
        assert!(!font_parses(real.get(..20).unwrap_or_default(), 0), "a truncated font must be refused");
        assert!(!font_parses(&real, 3), "a single-face file has no face 3");

        // A bad face that slipped into the definitions is dropped (key and family entries), and the
        // context builds its fonts without panicking.
        let mut fonts = font_definitions();
        fonts.font_data.insert("sys-hebrew".into(), Arc::new(FontData::from_owned(vec![0, 1, 0, 0, 0, 0xFF, 0, 0])));
        for chain in fonts.families.values_mut() {
            chain.push("sys-hebrew".into());
        }
        drop_unparseable_fonts(&mut fonts);
        assert!(!fonts.font_data.contains_key("sys-hebrew"));
        assert!(fonts.families.values().all(|c| !c.iter().any(|k| k == "sys-hebrew")));
        let ctx = egui::Context::default();
        ctx.set_fonts(fonts);
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            ui.label("Hebrew עברית");
        });
        out.textures_delta.clear(); // nothing consumes the atlas in a headless test
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    /// A broken existing font candidate is skipped in favor of a later readable candidate.
    fn try_load_skips_a_broken_candidate() {
        // A candidate that exists but is not a readable font must not win over a good one behind it.
        let dir = std::env::temp_dir().join(format!("gridcraft-font-test-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&dir);
        let bad = dir.join("broken.ttf");
        let good = dir.join("good.ttf");
        let real = FontDefinitions::default().font_data.values().next().map(|fd| fd.bytes().to_vec()).unwrap_or_default();
        let _ = std::fs::write(&bad, real.get(..40).unwrap_or_default());
        let _ = std::fs::write(&good, &real);
        let (bad_s, good_s) = (bad.to_string_lossy().into_owned(), good.to_string_lossy().into_owned());
        assert!(try_load(&[(bad_s.as_str(), 0)]).is_none());
        let loaded = try_load(&[(bad_s.as_str(), 0), (good_s.as_str(), 0)]);
        assert_eq!(loaded.map(|fd| fd.bytes().len()), Some(real.len()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    /// Repeated loads reuse one shared font blob, while a face the kept file does not contain is still refused.
    fn interface_fonts_are_read_once_and_shared_with_egui() {
        // egui copies owned font bytes into its text engine and keeps the original too; borrowed
        // bytes it shares. A rebuild of the font set reuses the file read the first time.
        let path = std::env::temp_dir().join(format!("gridcraft-font-shared-{}.ttf", std::process::id()));
        std::fs::write(&path, epaint_default_fonts::UBUNTU_LIGHT).unwrap();
        let p = path.to_string_lossy().into_owned();
        let first = try_load(&[(p.as_str(), 0)]).unwrap();
        std::fs::remove_file(&path).unwrap();
        let second = try_load(&[(p.as_str(), 0)]).expect("kept from the first read");
        assert!(
            std::sync::Arc::ptr_eq(&first.font, &second.font),
            "interface font bytes must be one shared blob, so egui shares them instead of copying"
        );
        assert_eq!(first.bytes(), epaint_default_fonts::UBUNTU_LIGHT);
        assert!(try_load(&[(p.as_str(), 3)]).is_none(), "a face the kept file doesn't have is still refused");
    }

    #[test]
    fn macos_bold_interface_face_is_drawn_unhinted() {
        // #1184: Helvetica Neue Bold's own TrueType hints close Cyrillic е into ө at 13 pt on a
        // Retina screen. Core Text never runs them, so on macOS the face goes unhinted like every
        // other app draws it; the regular face and the other platforms keep egui's default.
        let fonts = font_definitions_for_language(Language::En);
        let hinting = |key: &str| fonts.font_data.get(key).map(|fd| fd.tweak.hinting);
        if let Some(bold) = hinting(&format!("sys-{UI_BOLD}")) {
            assert_eq!(bold, if cfg!(target_os = "macos") { Some(false) } else { None });
        }
        if let Some(regular) = hinting(&format!("sys-{UI}")) {
            assert_eq!(regular, None, "only the bold face is affected");
        }
    }

    #[test]
    fn fedora_package_folders_are_searched_after_the_other_linux_layouts() {
        // #1694: Fedora keeps each font family in its own package folder, so without these entries
        // no cell face is found there and bold and italic cells draw as regular text. Each comes
        // after the existing Linux folders for the same font, so wherever one of those matched,
        // nothing changes.
        let paths = |cands: Vec<(String, u32)>| cands.into_iter().map(|(p, _)| p).collect::<Vec<_>>();
        let cases: [(&str, Vec<String>, &str, &str); 9] = [
            (CELL, paths(candidates(CELL)), "google-carlito-fonts/Carlito-Regular.ttf", "/carlito/Carlito-Regular.ttf"),
            (CELL_BOLD, paths(candidates(CELL_BOLD)), "google-carlito-fonts/Carlito-Bold.ttf", "/carlito/Carlito-Bold.ttf"),
            (CELL_ITALIC, paths(candidates(CELL_ITALIC)), "google-carlito-fonts/Carlito-Italic.ttf", "/carlito/Carlito-Italic.ttf"),
            (CELL_BOLD_ITALIC, paths(candidates(CELL_BOLD_ITALIC)), "google-carlito-fonts/Carlito-BoldItalic.ttf", "/carlito/Carlito-BoldItalic.ttf"),
            (CELL_BOLD, paths(candidates(CELL_BOLD)), "liberation-sans-fonts/LiberationSans-Bold.ttf", "/liberation/LiberationSans-Bold.ttf"),
            (SERIF, paths(candidates(SERIF)), "liberation-serif-fonts/LiberationSerif-Regular.ttf", "/liberation/LiberationSerif-Regular.ttf"),
            (MONO, paths(candidates(MONO)), "liberation-mono-fonts/LiberationMono-Regular.ttf", "/dejavu/DejaVuSansMono.ttf"),
            ("thai", paths(thai()), "google-noto-vf/NotoSansThai[wght].ttf", "/NotoSansThai-Regular.ttf"),
            ("hebrew", paths(hebrew()), "google-noto-vf/NotoSansHebrew[wght].ttf", "/NotoSansHebrew-Regular.ttf"),
        ];
        for (role, list, fedora_file, earlier) in &cases {
            let at = list.iter().position(|p| *p == format!("/usr/share/fonts/{fedora_file}"));
            let last_earlier = list.iter().rposition(|p| p.ends_with(earlier));
            assert!(at.is_some(), "{role}: Fedora's {fedora_file} is not a candidate");
            assert!(last_earlier.is_some(), "{role}: no existing {earlier} candidate to compare with");
            assert!(at > last_earlier, "{role}: {fedora_file} must come after every existing {earlier}");
        }
        // Liberation Mono only stands in for DejaVu Sans Mono, wherever that is installed.
        let mono = paths(candidates(MONO));
        let first_liberation = mono.iter().position(|p| p.ends_with("LiberationMono-Regular.ttf"));
        let last_dejavu = mono.iter().rposition(|p| p.ends_with("DejaVuSansMono.ttf"));
        assert!(first_liberation.is_some() && first_liberation > last_dejavu, "Liberation Mono must follow DejaVu Sans Mono: {mono:?}");
        let noto = mono.iter().position(|p| p == "/usr/share/fonts/google-noto-vf/NotoSansMono[wght].ttf");
        assert!(noto.is_some() && noto.unwrap_or(0) + 1 == mono.len(), "Fedora's default Noto Sans Mono must be the last resort: {mono:?}");
    }
}
