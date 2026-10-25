//! Programmatic control of the running app (agents, tests, MCP).
//!
//! Methods (JSON lines over the host's transport, see `docs/control-protocol.md`):
//! - `engine.execute {command, params}` — run any engine or UI command
//! - `engine.commands {search?}` — every command with enablement and params docs
//! - `document.inspect`, `sheet.read {range?}`, `cell.get {cell?}` — read state
//! - `ui.inspect` — UI state (ribbon tab, editor, dialog, grid geometry, perf)
//! - `ui.click {x, y, button?, count?, shift?, cmd?, alt?}`, `ui.drag {x, y, toX, toY}`,
//!   `ui.move {x, y}` — real pointer input in window points
//! - `ui.cell.click {cell}`, `ui.cell.drag {from, to}` — pointer input on a cell's centre
//! - `ui.key {key, shift?, cmd?, alt?, ctrl?}`, `ui.text {text}` — keyboard input
//! - `ui.ribbon {tab}`, `ui.dialog {name}`, `ui.dialog.set {field, value}`,
//!   `ui.dialog.confirm`, `ui.dialog.cancel`, `ui.message.dismiss`
//! - `ui.edit.begin {text?}`, `ui.edit.commit {move?}`, `ui.edit.cancel`
//! - `ui.screenshot {path?}` — PNG of the window (base64 when no path); animations are settled
//! - `ui.resize {width, height}`, `ui.set {dark?, formulaBar?, ribbonCollapsed?, animations?}`
//! - `app.open {path, password?}`, `app.save {path?}`, `app.quit`

use std::sync::mpsc::Sender;

use craft_ui::capture::CaptureSlot;
use gridcraft_engine::core::CellRef;
use serde_json::{Value as Json, json};

use crate::SheetApp;

pub type ControlResponse = Json;

pub struct ControlRequest {
    pub method: String,
    pub params: Json,
    pub reply: Sender<ControlResponse>,
    /// Who has the request: [`PENDING`], [`TAKEN`] (the app runs it) or [`ABANDONED`] (the sender
    /// gave up waiting, so the app must not run it late: a client told its request timed out had
    /// it applied anyway on the next UI pass, and an agent that retried ran it twice (#1914).
    pub state: std::sync::Arc<std::sync::atomic::AtomicU8>,
}

/// Queued, not yet run.
pub const PENDING: u8 = 0;
/// The app has started it: it will answer.
pub const TAKEN: u8 = 1;
/// Its sender gave up waiting (the reply timeout expired): it is dropped unrun.
pub const ABANDONED: u8 = 2;

impl ControlRequest {
    pub fn new(method: impl Into<String>, params: Json) -> (Self, std::sync::mpsc::Receiver<ControlResponse>) {
        let (tx, rx) = std::sync::mpsc::channel();
        (Self { method: method.into(), params, reply: tx, state: Default::default() }, rx)
    }

    /// Claim the request for running; false when its sender already gave up on it.
    pub fn take(&self) -> bool {
        use std::sync::atomic::Ordering;
        self.state.compare_exchange(PENDING, TAKEN, Ordering::SeqCst, Ordering::SeqCst).is_ok() || self.state.load(Ordering::SeqCst) == TAKEN
    }

    /// Give up on the request; false when the app has already started it (it will answer).
    pub fn abandon(state: &std::sync::atomic::AtomicU8) -> bool {
        use std::sync::atomic::Ordering;
        state.compare_exchange(PENDING, ABANDONED, Ordering::SeqCst, Ordering::SeqCst).is_ok() || state.load(Ordering::SeqCst) == ABANDONED
    }
}

pub enum Outcome {
    Done(Json),
    Screenshot { path: Option<String> },
}

/// Most screenshots one client may have in flight or waiting for its reply at once.
pub const SCREENSHOT_MAX_REQUESTS: usize = 4;
/// Retained bytes for one screenshot image. The renderer's readback buffer is a separate
/// transient allocation (recorded in peak RSS), not part of this reservation.
pub const SCREENSHOT_MAX_IMAGE_BYTES: usize = 128 * 1024 * 1024;
/// Aggregate bytes reserved by every outstanding screenshot request.
pub const SCREENSHOT_MAX_AGGREGATE_BYTES: usize = 256 * 1024 * 1024;
/// Largest transient working set allowed while one capture is encoded. It covers the RGBA copy
/// made from the retained `ColorImage`, the PNG stream and (for base64 replies) the encoded text.
/// Admission checks this before a request is issued, so a large capture cannot make the process
/// peak several times above its retained-image budget.
pub const SCREENSHOT_MAX_ENCODING_BYTES: usize = 256 * 1024 * 1024;
/// Largest base64 reply this client produces. The encoded length is checked before the string is
/// allocated; at most [`SCREENSHOT_MAX_REQUESTS`] such replies can therefore wait in the channel,
/// keeping queued base64 text bounded too.
pub const SCREENSHOT_MAX_BASE64_BYTES: usize = 64 * 1024 * 1024;
/// Hard bound on retained request images plus one capture's encoding working set. Admission,
/// issuance and every retry reject a request that would push the process over this, so the
/// aggregate reservation and the transient encoder peak together stay bounded.
pub const SCREENSHOT_MAX_PEAK_BYTES: usize = 512 * 1024 * 1024;
/// Capture retries allowed inside one request's deadline after a resize/DPI/zoom/surface change.
pub const SCREENSHOT_MAX_RETRIES: u32 = 2;
/// Reported once when the active document changes before a queued capture is issued or completed.
pub const SCREENSHOT_DOCUMENT_CHANGED: &str = "screenshot failed: the document changed while the capture was queued";
/// Reported once when a capture request or completion is not for the active viewport.
pub const SCREENSHOT_VIEWPORT_CHANGED: &str = "screenshot failed: the capture belongs to another viewport";

/// Reserved while the viewport has no capturable size yet (minimized, or not laid out at all):
/// the largest single image the per-image limit allows. The real size replaces it at issuance.
pub const UNKNOWN_CAPTURE_BYTES: usize = SCREENSHOT_MAX_IMAGE_BYTES;

/// The viewport geometry a capture is admitted, issued and completed against.
///
/// `SheetApp` keeps a monotonic epoch (see [`crate::SheetApp::logic`]) that increments on any
/// change of these values, so a resize back to the original size is still a mismatch: an image
/// taken in another layout must never be combined with the current geometry. The viewport id,
/// minimized/occluded state and visibility are part of the identity too: a minimize/restore or
/// an occlusion transition at the same pixel size is a new surface generation, not a no-op.
///
/// The identity deliberately keeps `zoom`, `visible` and `occluded` even though interface zoom
/// does not change the expected physical surface size. A capture is bound to one layout generation,
/// so a zoom or occlusion transition during it is retried within [`SCREENSHOT_MAX_RETRIES`] and
/// then reported as a bounded failure instead of risking pixels from another generation. The common
/// wheel-zoom path is frozen while a capture is in flight; a non-glide zoom or a compositor
/// occlusion flap can still exhaust the retries, which is accepted as a rare, explicit error.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CaptureGeometry {
    /// The viewport this capture belongs to.
    pub viewport_id: egui::ViewportId,
    /// Logical size of the content that is captured, or `None` when the window currently has no
    /// capturable surface (minimized, or not laid out yet). A hidden-but-occluded window still
    /// reports a size.
    pub viewport: Option<egui::Vec2>,
    pub pixels_per_point: f32,
    pub zoom: f32,
    /// Maximum physical width/height the surface/renderer can capture.
    pub max_texture_side: usize,
    pub minimized: Option<bool>,
    pub occluded: Option<bool>,
    pub visible: Option<bool>,
}

impl CaptureGeometry {
    /// The geometry described by a native frame's fresh [`egui::RawInput`].
    ///
    /// `raw.screen_rect` is the authoritative content size when `ViewportInfo::inner_rect` is
    /// absent (Wayland always omits it, and a minimized window can omit both). The fallback to the
    /// last UI pass's rect is deliberately *not* used here: a logic-only tick after a resize or
    /// minimize must record the new state (or the explicit unknown state), never the old layout.
    pub fn from_raw_input(raw: &egui::RawInput, ctx: &egui::Context) -> Self {
        let info = raw.viewports.get(&raw.viewport_id).cloned().unwrap_or_default();
        Self::build(raw.viewport_id, &info, raw.screen_rect, raw.max_texture_side, ctx.zoom_factor(), ctx.pixels_per_point())
    }

    /// Builds a `CaptureGeometry` from fresh viewport information. A minimized window has no size; `inner_rect` falls back to the raw screen rect; native scale is multiplied by interface zoom, and a zero surface limit means uncapped.
    fn build(
        viewport_id: egui::ViewportId,
        info: &egui::ViewportInfo,
        screen_rect: Option<egui::Rect>,
        max_texture_side: Option<usize>,
        zoom: f32,
        fallback_pixels_per_point: f32,
    ) -> Self {
        let minimized = info.minimized;
        let visible = info.visible();
        // A minimized window has no surface to capture even if the platform still reports the
        // last inner rectangle. Both rectangles missing likewise means "unknown size", not a
        // reason to re-use an earlier layout.
        let viewport = if minimized == Some(true) {
            None
        } else {
            info.inner_rect.map(|rect| rect.size()).filter(positive_size).or_else(|| screen_rect.map(|rect| rect.size()).filter(positive_size))
        };
        let pixels_per_point =
            info.native_pixels_per_point.filter(|p| p.is_finite() && *p > 0.0).map_or(fallback_pixels_per_point, |native| native * zoom);
        Self {
            viewport_id,
            viewport,
            pixels_per_point,
            zoom,
            // Zero means "unreported" (a bare test context); `expected_size` treats it as no cap.
            max_texture_side: max_texture_side.unwrap_or(0),
            minimized,
            occluded: info.occluded,
            visible,
        }
    }

    /// The geometry of egui's current input. This is a fallback for callers that do not run
    /// [`crate::SheetApp::raw_input_hook`] (tests and embedders); native eframe always records the
    /// fresh [`Self::from_raw_input`] value first, so issuance and drain never fall back here.
    pub fn current(ctx: &egui::Context) -> Self {
        let (viewport_id, info, screen_rect, max_texture_side, fallback_ppp) = ctx.input(|i| {
            let viewport_id = i.raw.viewport_id;
            let info = i.raw.viewports.get(&viewport_id).cloned().unwrap_or_default();
            // `screen_rect` from the pass if one supplied it; otherwise egui's retained content
            // rect (which is only fresh when this is called from a UI pass, never a logic tick).
            let screen_rect = i.raw.screen_rect.or_else(|| Some(i.viewport_rect()));
            (viewport_id, info, screen_rect, i.max_texture_side, i.pixels_per_point())
        });
        Self::build(viewport_id, &info, screen_rect, Some(max_texture_side), ctx.zoom_factor(), fallback_ppp)
    }

    /// Physical pixel size the screenshot should have, rounded and capped by the surface limit.
    /// `None` when the viewport has no positive, finite size (minimized or not laid out yet).
    pub fn expected_size(&self) -> Option<[usize; 2]> {
        let viewport = self.viewport?;
        let ppp = self.pixels_per_point;
        if !viewport.x.is_finite() || !viewport.y.is_finite() || viewport.x <= 0.0 || viewport.y <= 0.0 {
            return None;
        }
        if !ppp.is_finite() || ppp <= 0.0 {
            return None;
        }
        // An unreported surface limit (0) must not cap every capture to one pixel.
        let max = if self.max_texture_side == 0 { usize::MAX } else { self.max_texture_side };
        let axis = |points: f32| -> Option<usize> {
            let px = (f64::from(points) * f64::from(ppp)).round();
            if !px.is_finite() || px <= 0.0 || px > usize::MAX as f64 {
                return None;
            }
            Some((px as usize).min(max))
        };
        Some([axis(viewport.x)?, axis(viewport.y)?])
    }

    /// Checked `width * height * 4` bytes for the expected image, capped by the surface limit.
    pub fn expected_bytes(&self) -> Option<usize> {
        let [w, h] = self.expected_size()?;
        w.checked_mul(h)?.checked_mul(4)
    }

    /// Checked bytes of the largest image [`validate_image`] accepts for this geometry: one
    /// physical pixel of rounding per axis. Reservations and the slot's per-image bound both use
    /// this, so the aggregate budget is strict for the bytes actually retained.
    pub fn retained_bytes(&self) -> Option<usize> {
        let [w, h] = self.expected_size()?;
        let w = w.checked_add(1)?;
        let h = h.checked_add(1)?;
        w.checked_mul(h)?.checked_mul(4)
    }
}

/// True only when both width and height are finite and positive.
fn positive_size(size: &egui::Vec2) -> bool {
    size.x.is_finite() && size.y.is_finite() && size.x > 0.0 && size.y > 0.0
}

/// Conservative worst-case size of the PNG stream for a `bytes`-sized RGBA image with `height`
/// rows. PNG adds a filter byte per row and zlib framing; uncompressed blocks add a few bytes per
/// 64 KiB. 1% + 64 KiB plus the row bytes is safely above the real encoder output.
fn png_worst_bytes(bytes: usize, height: usize) -> Option<usize> {
    bytes.checked_add(bytes / 100)?.checked_add(height.checked_mul(2)?)?.checked_add(64 * 1024)
}

/// Base64 length (with padding) for `len` bytes.
fn base64_len(len: usize) -> Option<usize> {
    len.checked_add(2)?.checked_div(3)?.checked_mul(4)
}

/// Worst transient bytes for encoding one expected capture: the retained `ColorImage` plus the
/// RGBA copy made from it, then (for base64) the PNG stream and the encoded text. `None` on
/// overflow. Admission rejects a request whose working set exceeds
/// [`SCREENSHOT_MAX_ENCODING_BYTES`] before any slot or command exists.
pub fn encoding_working_bytes(expected: [usize; 2], base64: bool) -> Option<usize> {
    let [w, h] = expected;
    let bytes = w.checked_mul(h)?.checked_mul(4)?;
    let retained_and_copy = bytes.checked_mul(2)?;
    if !base64 {
        return Some(retained_and_copy);
    }
    let png = png_worst_bytes(bytes, h)?;
    let encoded = base64_len(png)?;
    Some(retained_and_copy.max(bytes.checked_add(png)?.checked_add(encoded)?))
}

/// Reject an expected capture whose encoding working set or base64 reply would exceed the
/// explicit encoding limits. Checked before admission and again immediately before issuance.
fn check_encoding_budget(expected: [usize; 2], base64: bool) -> Result<(), String> {
    let [w, h] = expected;
    let Some(bytes) = w.checked_mul(h).and_then(|p| p.checked_mul(4)) else {
        return Err("screenshot failed: the window size overflows the screenshot budget".to_string());
    };
    let Some(working) = encoding_working_bytes(expected, base64) else {
        return Err("screenshot failed: the window size overflows the screenshot encoding budget".to_string());
    };
    if working > SCREENSHOT_MAX_ENCODING_BYTES {
        return Err(format!(
            "screenshot encoding budget exceeded: {working} bytes of transient working memory for one {w}×{h} capture, over the {SCREENSHOT_MAX_ENCODING_BYTES}-byte encoding limit"
        ));
    }
    if base64 {
        let Some(png) = png_worst_bytes(bytes, h) else {
            return Err("screenshot failed: the window size overflows the screenshot encoding budget".to_string());
        };
        let Some(encoded) = base64_len(png) else {
            return Err("screenshot failed: the window size overflows the screenshot encoding budget".to_string());
        };
        if encoded > SCREENSHOT_MAX_BASE64_BYTES {
            return Err(format!(
                "screenshot base64 budget exceeded: the reply for one {w}×{h} capture could be {encoded} bytes, over the {SCREENSHOT_MAX_BASE64_BYTES}-byte base64 limit; request a path instead"
            ));
        }
    }
    Ok(())
}

/// Reject a request whose reserved retained bytes plus its encoding working set exceed
/// [`SCREENSHOT_MAX_PEAK_BYTES`]. `reserved` already includes this request's own reservation.
fn check_peak_budget(reserved: usize, expected: [usize; 2], base64: bool) -> Result<(), String> {
    let Some(working) = encoding_working_bytes(expected, base64) else {
        return Err("screenshot failed: the window size overflows the screenshot encoding budget".to_string());
    };
    let peak = reserved.saturating_add(working);
    if peak > SCREENSHOT_MAX_PEAK_BYTES {
        return Err(format!(
            "screenshot peak-memory budget exceeded: {reserved} bytes retained plus {working} bytes encoding for one capture is over the {SCREENSHOT_MAX_PEAK_BYTES}-byte peak limit"
        ));
    }
    Ok(())
}

/// One admitted `ui.screenshot` request. The slot owns the image and its viewport waker; the
/// callback holds only a weak reference to it and never writes a file or sends a reply.
struct Shot {
    token: u64,
    path: Option<String>,
    reply: Sender<ControlResponse>,
    /// Monotonic deadline for the whole request, including retries.
    deadline: f64,
    /// Settle time before the first issuance (the UI must be still).
    ready_at: f64,
    /// Real (non-discarded) UI passes counted towards settling.
    frames: u32,
    /// `cumulative_pass_nr` of the last pass counted in `frames`, so repeated `ui` calls in one
    /// pass count once and a settled frame is never counted twice.
    settle_pass: Option<u64>,
    issued: bool,
    retries: u32,
    /// Geometry at admission, updated immediately before issuance.
    geometry: CaptureGeometry,
    /// The viewport this request was admitted for. A callback or slot from another viewport must
    /// never fill this request.
    viewport_id: egui::ViewportId,
    /// The active document (`DocState::uid`) at admission. A queued capture is never rebound to a
    /// document that became active later.
    document: Option<u64>,
    /// Monotonic geometry epoch at issuance; any later change rejects the image.
    epoch: u64,
    /// Physical size expected at issuance; `None` while the window has no capturable size.
    expected: Option<[usize; 2]>,
    /// Bytes reserved from the aggregate budget until the request completes or is dropped. This
    /// is the largest image the request may retain (one pixel of rounding per axis included) and
    /// is also the slot's per-image bound.
    bytes: usize,
    /// Created at issuance (and anew for every retry), never at admission: the bound must match
    /// the geometry and reservation the command is issued against.
    slot: Option<CaptureSlot>,
}

#[derive(Default)]
pub struct Shots {
    token: u64,
    requests: Vec<Shot>,
}

impl Shots {
    /// A screenshot was asked for and hasn't been delivered yet: animations hold still, so the
    /// picture shows the settled state.
    pub(crate) fn in_flight(&self) -> bool {
        !self.requests.is_empty()
    }

    /// Whether a hidden window still needs `RequestPaintWhileHidden` to reach issuance. Only a
    /// request that is admitted or settling with a capturable size needs a first `ui` pass; once
    /// `issue_screenshots` has sent the `Screenshot` command, that command's own `wants_paint`
    /// paints the hidden window. A minimized request has no capturable size, so requesting hidden
    /// full frames for its whole wait cannot make progress (the restore pass issues it).
    pub(crate) fn needs_hidden_paint(&self) -> bool {
        self.requests.iter().any(|r| !r.issued && r.expected.is_some())
    }

    /// Bytes currently reserved by admitted requests.
    fn reserved(&self) -> usize {
        self.requests.iter().fold(0usize, |sum, r| sum.saturating_add(r.bytes))
    }

    /// Drop every outstanding request without a reply (app shutdown): callbacks become inert.
    pub fn cancel_all(&mut self) {
        for request in self.requests.drain(..) {
            if let Some(slot) = request.slot {
                slot.cancel();
            }
        }
    }
}

fn ok(v: Json) -> Outcome {
    Outcome::Done(json!({"ok": true, "result": v}))
}
fn err(e: impl std::fmt::Display) -> Outcome {
    Outcome::Done(json!({"ok": false, "error": e.to_string()}))
}
fn wrap(r: Result<Json, String>) -> Outcome {
    match r {
        Ok(v) => ok(v),
        Err(e) => err(e),
    }
}

/// Runs a command for an agent. Commands that move the selection scroll the active cell into
/// view, as the Go To dialog, the Name Box and the arrow keys do.
fn run(app: &mut SheetApp, id: &str, params: Json) -> Result<Json, String> {
    let r = app.run(id, params);
    if r.is_ok() && matches!(id, "edit.goTo" | "edit.goToSpecial" | "selection.set" | "selection.move") {
        app.grid.ensure_visible = true;
    }
    r
}

pub fn inspect(app: &SheetApp, ctx: &egui::Context) -> Json {
    let r = ctx.content_rect();
    let v = app.view();
    json!({
        "ui": serde_json::to_value(&app.ui).unwrap_or_default(),
        "window": [r.width(), r.height()],
        "grid": app.grid.cells_rect.map(|c| json!([c.left(), c.top(), c.width(), c.height()])),
        "scroll": [v.scroll.x, v.scroll.y],
        "editor": app.editor.as_ref().map(|e| json!({"cell": e.cell.a1(), "text": e.text, "formulaBar": e.from_formula_bar, "point": e.can_point()})),
        "dialog": app.dialog.as_ref().map(|d| json!({"name": d.name, "title": d.title, "values": d.values, "tab": d.tab})),
        "backstage": app.backstage.as_ref().map(|b| b.page.id()),
        "message": app.message.as_ref().map(|(t, m)| json!({"title": t, "text": m})),
        "selectedChart": app.selected_chart,
        "mode": format!("{:?}", app.session.mode),
        "documents": app.session.documents().iter().map(|d| json!({"title": d.display_title(), "dirty": d.is_dirty()})).collect::<Vec<_>>(),
        "activeDocument": app.session.active_index(),
        "perf": {"frameMs": app.perf.frame_ms, "gridMs": app.perf.grid_ms, "fps": app.perf.fps},
    })
}

fn key_from(name: &str) -> Option<egui::Key> {
    egui::Key::from_name(name).or(match name.to_ascii_lowercase().as_str() {
        "enter" | "return" => Some(egui::Key::Enter),
        "esc" | "escape" => Some(egui::Key::Escape),
        "delete" | "del" => Some(egui::Key::Delete),
        "backspace" => Some(egui::Key::Backspace),
        "left" => Some(egui::Key::ArrowLeft),
        "right" => Some(egui::Key::ArrowRight),
        "up" => Some(egui::Key::ArrowUp),
        "down" => Some(egui::Key::ArrowDown),
        "space" => Some(egui::Key::Space),
        "tab" => Some(egui::Key::Tab),
        "pageup" => Some(egui::Key::PageUp),
        "pagedown" => Some(egui::Key::PageDown),
        "home" => Some(egui::Key::Home),
        "end" => Some(egui::Key::End),
        "f2" => Some(egui::Key::F2),
        "f4" => Some(egui::Key::F4),
        "f9" => Some(egui::Key::F9),
        _ => None,
    })
}

fn modifiers(p: &Json) -> egui::Modifiers {
    let b = |n: &str| p.get(n).and_then(Json::as_bool).unwrap_or(false);
    egui::Modifiers {
        alt: b("alt"),
        ctrl: b("ctrl"),
        shift: b("shift"),
        mac_cmd: b("cmd") && cfg!(target_os = "macos"),
        command: b("cmd") || (b("ctrl") && !cfg!(target_os = "macos")),
    }
}

fn cell_center(app: &SheetApp, c: CellRef) -> Option<egui::Pos2> {
    let d = app.session.active()?;
    let sh = d.wb.active()?;
    let rect = app.grid.rect?;
    let geo = crate::grid::Geo::new(sh, rect, app.view().scroll);
    Some(geo.screen_cell_rect(sh, c).center())
}

/// Most synthetic events (pointer, key, text and move alike) a control client may keep queued at
/// once. The queue offers one batch per UI pass, and a hidden logic-only frame keeps the same
/// batch unapplied, so every producer must honour this budget.
pub const MAX_QUEUED_SYNTHETIC_EVENTS: usize = 10_000;
/// Most bytes of synthetic input queued at once: the `Event` storage plus every text payload.
pub const MAX_QUEUED_SYNTHETIC_BYTES: usize = 8 * 1024 * 1024;
/// Largest single `ui.text` string accepted (well below the control line limit; one paste-sized
/// string is plenty for a spreadsheet cell and keeps a burst from exhausting memory).
pub const MAX_SYNTHETIC_TEXT_BYTES: usize = 1024 * 1024;

/// Bytes charged to one synthetic event: `size_of::<Event>()` plus the payload of a `Text` event.
fn event_bytes(event: &egui::Event) -> usize {
    std::mem::size_of::<egui::Event>()
        + match event {
            egui::Event::Text(text) => text.len(),
            _ => 0,
        }
}

/// Saturating totals of queued synthetic events and their charged bytes across every pending batch.
fn synthetic_usage(app: &SheetApp) -> (usize, usize) {
    app.synthetic
        .iter()
        .flat_map(|batch| batch.iter())
        .fold((0usize, 0usize), |(events, bytes), event| (events.saturating_add(1), bytes.saturating_add(event_bytes(event))))
}

/// Refuses a request that would push the queued synthetic input past either limit; the queue is
/// left unchanged.
fn synthetic_budget(app: &SheetApp, extra_events: usize, extra_bytes: usize) -> Result<(), String> {
    let (events, bytes) = synthetic_usage(app);
    if events.saturating_add(extra_events) > MAX_QUEUED_SYNTHETIC_EVENTS {
        return Err(format!("too many synthetic input events (limit {MAX_QUEUED_SYNTHETIC_EVENTS} queued at once)"));
    }
    if bytes.saturating_add(extra_bytes) > MAX_QUEUED_SYNTHETIC_BYTES {
        return Err(format!("synthetic input exceeds the {MAX_QUEUED_SYNTHETIC_BYTES}-byte queue limit"));
    }
    Ok(())
}

/// Checks the synthetic-event budget, then queues one pointer move followed by `count` press/release pairs (`count` is at least one).
fn push_click(app: &mut SheetApp, pos: egui::Pos2, button: egui::PointerButton, count: u32, m: egui::Modifiers) -> Result<(), String> {
    let count = count.max(1) as usize;
    let events = count.saturating_mul(2).saturating_add(1);
    synthetic_budget(app, events, events.saturating_mul(std::mem::size_of::<egui::Event>()))?;
    app.synthetic.push_back(vec![egui::Event::PointerMoved(pos)]);
    let mut batch = Vec::with_capacity(count * 2);
    for _ in 0..count {
        batch.push(egui::Event::PointerButton { pos, button, pressed: true, modifiers: m });
        batch.push(egui::Event::PointerButton { pos, button, pressed: false, modifiers: m });
    }
    app.synthetic.push_back(batch);
    Ok(())
}

/// Checks the synthetic-event budget, then queues a move, a press, `steps` interpolated moves and a release.
fn push_drag(app: &mut SheetApp, a: egui::Pos2, b: egui::Pos2, steps: usize, m: egui::Modifiers) -> Result<(), String> {
    let steps = steps.max(1);
    let events = steps.saturating_add(3);
    synthetic_budget(app, events, events.saturating_mul(std::mem::size_of::<egui::Event>()))?;
    app.synthetic.push_back(vec![egui::Event::PointerMoved(a)]);
    app.synthetic.push_back(vec![egui::Event::PointerButton { pos: a, button: egui::PointerButton::Primary, pressed: true, modifiers: m }]);
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        app.synthetic.push_back(vec![egui::Event::PointerMoved(a + (b - a) * t)]);
    }
    app.synthetic.push_back(vec![egui::Event::PointerButton { pos: b, button: egui::PointerButton::Primary, pressed: false, modifiers: m }]);
    Ok(())
}

/// Checks the synthetic-event budget and queues one key press/release pair.
fn push_key(app: &mut SheetApp, key: egui::Key, m: egui::Modifiers) -> Result<(), String> {
    let events = 2;
    synthetic_budget(app, events, events * std::mem::size_of::<egui::Event>())?;
    app.synthetic.push_back(vec![
        egui::Event::Key { key, physical_key: None, pressed: true, repeat: false, modifiers: m },
        egui::Event::Key { key, physical_key: None, pressed: false, repeat: false, modifiers: m },
    ]);
    Ok(())
}

/// Rejects text longer than `MAX_SYNTHETIC_TEXT_BYTES`, then queues one `Text` event under the shared queue budget.
fn push_text(app: &mut SheetApp, text: String) -> Result<(), String> {
    if text.len() > MAX_SYNTHETIC_TEXT_BYTES {
        return Err(format!("text input is {} bytes, over the {MAX_SYNTHETIC_TEXT_BYTES}-byte limit for one event", text.len()));
    }
    let event = egui::Event::Text(text);
    synthetic_budget(app, 1, event_bytes(&event))?;
    app.synthetic.push_back(vec![event]);
    Ok(())
}

/// Queues one pointer-move event under the shared synthetic queue budget.
fn push_move(app: &mut SheetApp, pos: egui::Pos2) -> Result<(), String> {
    let event = egui::Event::PointerMoved(pos);
    synthetic_budget(app, 1, event_bytes(&event))?;
    app.synthetic.push_back(vec![event]);
    Ok(())
}

/// Keeps the commands whose id, label or ribbon path contains `search` (case-insensitive); `None` keeps all.
fn filter_commands(commands: Vec<Json>, search: Option<&str>) -> Vec<Json> {
    let Some(q) = search.map(str::to_ascii_lowercase) else { return commands };
    let has = |c: &Json, k: &str| c.get(k).and_then(Json::as_str).is_some_and(|v| v.to_ascii_lowercase().contains(&q));
    let in_menu = |c: &Json| {
        let menu = c.get("menu").and_then(Json::as_array);
        menu.is_some_and(|m| m.iter().filter_map(Json::as_str).any(|v| v.to_ascii_lowercase().contains(&q)))
    };
    commands.into_iter().filter(|c| has(c, "id") || has(c, "label") || in_menu(c)).collect()
}

/// Dispatches one control request: commands, inspection, UI state, dialogs, editing, synthetic input, open/save/quit and screenshots. Unknown methods (and unknown command ids) return an error. Immediate answers request a repaint; `ui.screenshot` is admitted asynchronously instead of being answered here.
pub fn handle(app: &mut SheetApp, ctx: &egui::Context, method: &str, p: &Json) -> Outcome {
    let s = |k: &str| p.get(k).and_then(Json::as_str);
    let f = |k: &str| p.get(k).and_then(Json::as_f64).unwrap_or(0.0) as f32;
    let out = match method {
        "engine.execute" | "command" | "ui.menu.invoke" => {
            let Some(id) = s("command").or(s("id")) else { return err("missing `command`") };
            let params = p.get("params").cloned().unwrap_or(json!({}));
            wrap(run(app, id, if params.is_null() { json!({}) } else { params }))
        }
        "engine.commands" => {
            let mut v: Vec<Json> = app.session.commands().into_iter().map(|c| serde_json::to_value(c).unwrap_or_default()).collect();
            let language_params = format!("{{language: {}}} (alias: code)", crate::i18n::Language::code_list("|"));
            for (id, label, params) in [
                ("ui.ribbonTab", "Ribbon Tab", ""),
                ("app.checkForUpdates", "Check for Updates…", ""),
                ("view.collapseRibbon", "Collapse Ribbon", ""),
                (
                    "ui.customizeRibbon",
                    "Customize the Ribbon",
                    "{layout?: {order?: [tab], hidden?: [tab], tabs?: [{key: \"customN\", label}], groups?: [{tab, label, commands: [id]}], qat?: [id]}, reset?: bool} → {layout, tabs}; {} reads it. Unknown tabs and commands are left out",
                ),
                ("ui.quickAccessToolbar", "Quick Access Toolbar", "{commands?: [id], add?: id, remove?: id, reset?: bool} → {commands}; {} reads it"),
                ("view.expandFormulaBar", "Expand Formula Bar", "{on?: bool} (toggles without it) → {on}"),
                ("view.darkMode", "Dark Mode", ""),
                ("view.theme", "Display Theme", ""),
                ("view.animations", "Animations", "{on?: bool} (toggles without it) → {on}"),
                ("options.autoComplete", "Enable AutoComplete for cell values", "{on?: bool} (toggles without it) → {on}"),
                ("view.interfaceScale", "Interface Size", "{percent: 75–200 | \"auto\"} (100 = auto; clamped)"),
                ("ui.dialog", "Open Dialog", ""),
                ("ui.backstage", "File (Backstage view)", "{page?: home|new|open|info|saveAs|print|export|options|account|close}"),
                (
                    "ui.printSettings",
                    "Print Settings",
                    "{printer?: discovered name, copies?: 1..999, duplex?: printerDefault|off|longEdge|shortEdge|null, grayscale?: bool, refresh?: bool} → {printer,copies,duplex,grayscale,printers,ready,busy,discovering,status,detail}; {} inspects without submitting",
                ),
                (
                    "ui.printSubmit",
                    "Submit Print Job",
                    "existing file.print sheet/pagination params → {printing,bytes,pending}; submits without opening a dialog",
                ),
                ("app.language.set", "Interface Language", language_params.as_str()),
                ("app.language.english", "Interface Language: English", ""),
                ("app.language.japanese", "Interface Language: Japanese", ""),
            ] {
                v.push(json!({"id": id, "label": label, "params": params, "enabled": true, "ui": true}));
            }
            ok(Json::Array(filter_commands(v, s("search"))))
        }
        "engine.journal" => ok(json!(app.session.journal.iter().map(|(id, p)| json!({"command": id, "params": p})).collect::<Vec<_>>())),
        "document.inspect" | "sheet.read" | "cell.get" | "selection.stats" => wrap(app.session.run(method, p.clone())),
        "ui.inspect" => ok(inspect(app, ctx)),
        "ui.ribbon" => {
            app.ui.ribbon_tab = s("tab").unwrap_or("Home").to_string();
            app.ui.ribbon_collapsed = false;
            ok(Json::Null)
        }
        "ui.set" => {
            if let Some(v) = p.get("dark").and_then(Json::as_bool) {
                app.ui.dark = v;
                app.ui.system_theme = false;
            }
            if let Some(v) = p.get("formulaBar").and_then(Json::as_bool) {
                let _ = app.run("view.formulaBar", json!({"on": v}));
            }
            if let Some(v) = p.get("ribbonCollapsed").and_then(Json::as_bool) {
                app.ui.ribbon_collapsed = v;
            }
            if let Some(v) = p.get("animations").and_then(Json::as_bool) {
                app.ui.animations = v;
            }
            let mut out = serde_json::to_value(&app.ui).unwrap_or_default();
            if let Some(o) = out.as_object_mut() {
                o.insert("formulaBar".into(), json!(app.session.prefs.formula_bar));
            }
            ok(out)
        }
        "ui.dialog" => {
            app.open_dialog(s("name").unwrap_or(""), p.clone());
            ok(json!({"dialog": app.dialog.as_ref().map(|d| d.name.clone())}))
        }
        "ui.dialog.set" => match app.dialog.as_mut() {
            Some(d) => {
                let Some(field) = s("field") else { return err("missing `field`") };
                if field == "tab" {
                    d.tab = s("value").unwrap_or("").to_string();
                } else if field == "search" {
                    d.search = s("value").unwrap_or("").to_string();
                } else {
                    d.set_value(field, p.get("value").cloned().unwrap_or(Json::Null));
                }
                ok(json!(d.values))
            }
            None => err("no dialog is open"),
        },
        "ui.dialog.confirm" => match app.dialog.take() {
            Some(d) => match d.command {
                Some(cmd) => {
                    let params = crate::dialogs::build_params(&d);
                    let r = app.run(cmd, params);
                    if r.is_err() {
                        app.dialog = Some(d);
                    }
                    wrap(r)
                }
                None => {
                    app.dialog = Some(d);
                    err("this dialog has no OK command; drive it with its own commands")
                }
            },
            None => err("no dialog is open"),
        },
        "ui.dialog.cancel" => {
            app.dialog = None;
            ok(Json::Null)
        }
        "ui.message.dismiss" => {
            app.message = None;
            ok(Json::Null)
        }
        "ui.edit.begin" => {
            app.begin_edit(s("text").map(str::to_string), false);
            ok(Json::Null)
        }
        "ui.edit.text" => match app.editor.as_mut() {
            Some(e) => {
                e.text = s("text").unwrap_or("").to_string();
                e.caret = e.text.chars().count();
                ok(Json::Null)
            }
            None => err("not editing"),
        },
        "ui.edit.commit" => {
            let (dr, dc) = match s("move") {
                Some("right") => (0, 1),
                Some("up") => (-1, 0),
                Some("left") => (0, -1),
                Some("none") => (0, 0),
                _ => (1, 0),
            };
            if app.commit_edit(dr, dc, false, false) {
                ok(Json::Null)
            } else {
                err(app.message.clone().map(|m| m.1).unwrap_or_else(|| "could not commit".into()))
            }
        }
        "ui.edit.cancel" => {
            app.cancel_edit();
            ok(Json::Null)
        }
        "ui.key" => {
            let Some(k) = s("key").and_then(key_from) else { return err("unknown or missing `key`") };
            match push_key(app, k, modifiers(p)) {
                Ok(()) => ok(Json::Null),
                Err(e) => err(e),
            }
        }
        "ui.text" => {
            let text = s("text").unwrap_or("");
            // Check the borrowed payload first: do not make a second, potentially large copy
            // before the per-event bound is known to hold.
            if text.len() > MAX_SYNTHETIC_TEXT_BYTES {
                return err(format!("text input is {} bytes, over the {MAX_SYNTHETIC_TEXT_BYTES}-byte limit for one event", text.len()));
            }
            match push_text(app, text.to_string()) {
                Ok(()) => ok(Json::Null),
                Err(e) => err(e),
            }
        }
        "ui.move" => match push_move(app, egui::pos2(f("x"), f("y"))) {
            Ok(()) => ok(Json::Null),
            Err(e) => err(e),
        },
        "ui.click" => {
            let button = if s("button") == Some("right") { egui::PointerButton::Secondary } else { egui::PointerButton::Primary };
            let count = p.get("count").and_then(Json::as_u64).unwrap_or(1).min(u64::from(u32::MAX)) as u32;
            match push_click(app, egui::pos2(f("x"), f("y")), button, count, modifiers(p)) {
                Ok(()) => ok(Json::Null),
                Err(e) => err(e),
            }
        }
        "ui.drag" => {
            let steps = p.get("steps").and_then(Json::as_u64).unwrap_or(8).min(u64::from(u32::MAX)) as usize;
            match push_drag(app, egui::pos2(f("x"), f("y")), egui::pos2(f("toX"), f("toY")), steps, modifiers(p)) {
                Ok(()) => ok(Json::Null),
                Err(e) => err(e),
            }
        }
        "ui.cell.click" => {
            let Some(c) = s("cell").and_then(CellRef::parse) else { return err("missing `cell`") };
            let Some(pos) = cell_center(app, c) else { return err("no grid yet") };
            let button = if s("button") == Some("right") { egui::PointerButton::Secondary } else { egui::PointerButton::Primary };
            let count = p.get("count").and_then(Json::as_u64).unwrap_or(1).min(u64::from(u32::MAX)) as u32;
            match push_click(app, pos, button, count, modifiers(p)) {
                Ok(()) => ok(json!({"x": pos.x, "y": pos.y})),
                Err(e) => err(e),
            }
        }
        "ui.cell.drag" => {
            let (Some(a), Some(b)) = (s("from").and_then(CellRef::parse), s("to").and_then(CellRef::parse)) else {
                return err("missing `from`/`to`");
            };
            let (Some(pa), Some(pb)) = (cell_center(app, a), cell_center(app, b)) else { return err("no grid yet") };
            match push_drag(app, pa, pb, 8, modifiers(p)) {
                Ok(()) => ok(Json::Null),
                Err(e) => err(e),
            }
        }
        "ui.resize" => {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(f("width").max(400.0), f("height").max(300.0))));
            ok(Json::Null)
        }
        "ui.focus" => {
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            ok(Json::Null)
        }
        "ui.screenshot" => return Outcome::Screenshot { path: s("path").map(str::to_string) },
        "app.open" => {
            let Some(path) = s("path") else { return err("missing `path`") };
            // Not `open_path`: that asks for a missing password in a dialog, and programmatic
            // calls never open one. A password-protected workbook takes `password` instead.
            let r = app.run("file.open", json!({"path": path, "password": s("password")}));
            if let Ok(r) = &r {
                app.after_open(Some(path), r);
            }
            app.after_engine();
            match (r, app.message.take()) {
                (Err(e), _) => err(crate::clean_error(&e)),
                (Ok(_), Some((_, m))) => err(m),
                (Ok(_), None) => ok(json!({"title": app.session.active().map(|d| d.display_title())})),
            }
        }
        "app.save" => wrap(app.run("file.save", json!({"path": s("path")}))),
        "app.quit" => {
            // Programmatic calls never open dialogs: quitting from here doesn't ask about unsaved work.
            app.allow_close = true;
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            ok(Json::Null)
        }
        // Anything else that names a command runs it.
        other if gridcraft_engine::find_command(other).is_some() => wrap(run(app, other, p.clone())),
        other => err(format!("unknown method `{other}`")),
    };
    ctx.request_repaint();
    out
}

/// Drains queued control requests, drops requests whose client stopped waiting, handles immediate outcomes and admits screenshots, then requests a repaint while synthetic input remains.
pub fn poll(app: &mut SheetApp, ctx: &egui::Context) {
    let Some(rx) = app.control_rx.take() else { return };
    while let Ok(req) = rx.try_recv() {
        // A request whose sender stopped waiting is dropped unrun: running it would apply a
        // command the client was told had failed, and its reply would go nowhere (#1914).
        if !req.take() {
            continue;
        }
        let outcome = handle(app, ctx, &req.method, &req.params);
        match outcome {
            Outcome::Done(v) => {
                let _ = req.reply.send(v);
            }
            Outcome::Screenshot { path } => admit(app, ctx, path, req.reply),
        }
    }
    app.control_rx = Some(rx);
    if !app.synthetic.is_empty() {
        ctx.request_repaint();
    }
}

/// Sends one screenshot rejection carrying the request token.
fn reject_shot(reply: Sender<ControlResponse>, token: u64, message: impl std::fmt::Display) {
    let _ = reply.send(json!({"ok": false, "error": message.to_string(), "request": token}));
}

/// Put the request token on every screenshot reply (success, rejection, timeout), so a client can
/// correlate an asynchronous capture even when the reply order differs from the request order.
fn with_token(mut reply: Json, token: u64) -> Json {
    if let Some(object) = reply.as_object_mut() {
        object.insert("request".into(), json!(token));
    }
    reply
}

/// The geometry to use for admission, issuance and drain.
///
/// Native eframe calls [`crate::SheetApp::raw_input_hook`] before every frame (including a
/// logic-only tick), so the recorded value carries fresh `RawInput` geometry even when egui's
/// last UI pass is stale. Tests and embedders that never call the hook fall back to egui's current
/// input; that fallback is only used when no raw frame was seen.
pub(crate) fn current_capture_geometry(app: &SheetApp, ctx: &egui::Context) -> CaptureGeometry {
    if app.capture_geometry_from_raw { app.capture_geometry.unwrap_or_else(|| CaptureGeometry::current(ctx)) } else { CaptureGeometry::current(ctx) }
}

/// Record `geometry` as the layout captures are bound to and bump the epoch when it differs.
///
/// Issuance and drain use this instead of writing the fields separately: during a hidden
/// logic-only tick the viewport info can be newer than the last UI pass, and the epoch must
/// change together with the geometry or a valid image would be retried against its own epoch.
pub(crate) fn note_capture_geometry(app: &mut SheetApp, geometry: CaptureGeometry) -> u64 {
    if app.capture_geometry != Some(geometry) {
        app.capture_geometry = Some(geometry);
        app.capture_epoch = app.capture_epoch.wrapping_add(1);
    }
    app.capture_epoch
}

/// Returns the next screenshot token, wrapping and skipping zero.
fn next_token(app: &mut SheetApp) -> u64 {
    let mut token = app.shots.token.wrapping_add(1);
    if token == 0 {
        token = 1;
    }
    app.shots.token = token;
    token
}

/// Admit one screenshot request against the count and byte budgets **before** any capture is
/// issued. A rejected request gets one error reply and never evicts an earlier request.
fn admit(app: &mut SheetApp, ctx: &egui::Context, path: Option<String>, reply: Sender<ControlResponse>) {
    let token = next_token(app);
    let geometry = current_capture_geometry(app, ctx);
    let expected = geometry.expected_size();
    let base64 = path.is_none();
    // A minimized or not-yet-laid-out window has no size: reserve the largest image the
    // per-image limit allows and keep the request queued. Issuance waits for a real size, and the
    // deadline reports a timeout if none arrives.
    let bytes = match expected {
        Some(size) => {
            let Some(bytes) = geometry.retained_bytes() else {
                reject_shot(reply, token, "screenshot failed: the window size overflows the screenshot budget");
                return;
            };
            if bytes > SCREENSHOT_MAX_IMAGE_BYTES {
                reject_shot(
                    reply,
                    token,
                    format!(
                        "screenshot budget exceeded: {bytes} bytes for one {}×{} image, over the {SCREENSHOT_MAX_IMAGE_BYTES}-byte per-image limit",
                        size[0], size[1]
                    ),
                );
                return;
            }
            if let Err(message) = check_encoding_budget(size, base64) {
                reject_shot(reply, token, message);
                return;
            }
            bytes
        }
        None => UNKNOWN_CAPTURE_BYTES,
    };
    if app.shots.requests.len() >= SCREENSHOT_MAX_REQUESTS {
        reject_shot(
            reply,
            token,
            format!("screenshot budget exceeded: {} requests are already outstanding (limit {SCREENSHOT_MAX_REQUESTS})", app.shots.requests.len()),
        );
        return;
    }
    let reserved = app.shots.reserved().saturating_add(bytes);
    if reserved > SCREENSHOT_MAX_AGGREGATE_BYTES {
        reject_shot(
            reply,
            token,
            format!(
                "screenshot budget exceeded: {reserved} bytes reserved by outstanding requests, over the {SCREENSHOT_MAX_AGGREGATE_BYTES}-byte aggregate limit"
            ),
        );
        return;
    }
    if let Some(size) = expected
        && let Err(message) = check_peak_budget(reserved, size, base64)
    {
        reject_shot(reply, token, message);
        return;
    }
    let now = crate::monotonic_ms();
    let document = app.session.active().map(|d| d.uid);
    let epoch = note_capture_geometry(app, geometry);
    app.shots.requests.push(Shot {
        token,
        path,
        reply,
        deadline: now + 8000.0,
        ready_at: now + 100.0,
        frames: 0,
        settle_pass: None,
        issued: false,
        retries: 0,
        geometry,
        viewport_id: geometry.viewport_id,
        document,
        epoch,
        expected,
        bytes,
        // Issued only from `ui`, so the slot is created there against the geometry in force.
        slot: None,
    });
    // A queued request with a capturable size has not issued a paint-requesting command yet;
    // `logic` keeps asking for hidden paints until `issue_screenshots` runs. A size-less request
    // asks for none (see `Shots::needs_hidden_paint`).
    ctx.request_repaint();
}

/// Builds the standard token-carrying timeout reply for a screenshot request.
fn timeout_reply(message: &str, token: u64) -> Json {
    json!({"ok": false, "error": message, "request": token})
}

/// The bytes to reserve for one capture of `expected` against `geometry`, after checking the
/// applicable limits. Issuance and retry both use this (admission keeps its own inline checks) so a
/// retry cannot drift from the limits the first issuance applied. `expected` is `None` when the
/// viewport has no capturable size yet: that reserves [`UNKNOWN_CAPTURE_BYTES`] and still checks
/// the aggregate limit, while the per-image, encoding and peak checks only apply once a size is
/// known.
fn capture_bytes(app: &SheetApp, i: usize, geometry: CaptureGeometry, expected: Option<[usize; 2]>, base64: bool) -> Result<usize, String> {
    let bytes = match expected {
        Some(expected) => {
            let bytes = geometry.retained_bytes().ok_or_else(|| "screenshot failed: the window size overflows the screenshot budget".to_string())?;
            if bytes > SCREENSHOT_MAX_IMAGE_BYTES {
                return Err(format!(
                    "screenshot budget exceeded: {bytes} bytes for one {}×{} image, over the {SCREENSHOT_MAX_IMAGE_BYTES}-byte per-image limit",
                    expected[0], expected[1]
                ));
            }
            check_encoding_budget(expected, base64)?;
            bytes
        }
        None => UNKNOWN_CAPTURE_BYTES,
    };
    let others = app.shots.requests.iter().enumerate().filter(|(j, _)| *j != i).fold(0usize, |sum, (_, r)| sum.saturating_add(r.bytes));
    if others.saturating_add(bytes) > SCREENSHOT_MAX_AGGREGATE_BYTES {
        return Err(format!(
            "screenshot budget exceeded: {bytes} bytes for this image would exceed the {SCREENSHOT_MAX_AGGREGATE_BYTES}-byte aggregate limit"
        ));
    }
    if let Some(expected) = expected {
        check_peak_budget(others.saturating_add(bytes), expected, base64)?;
    }
    Ok(bytes)
}

/// Issued from `ui`: counts one real (non-discarded) settling pass per request, revalidates the
/// geometry and reserves the exact retained bytes, then sends one
/// `ViewportCommand::Screenshot(callback)` per request.
///
/// The slot is created here, not at admission. Its per-image bound is the same reservation the
/// aggregate budget holds, so an image that would exceed the budget cannot be retained; a retry
/// after a resize creates a fresh slot for the new geometry.
pub fn issue_screenshots(app: &mut SheetApp, ctx: &egui::Context) {
    let now = crate::monotonic_ms();
    let pass = ctx.cumulative_pass_nr();
    // A pass whose output is already going to be discarded is not a settling frame and its
    // layout is not the one to capture. GridCraft requests no discard after this call.
    let discard = ctx.will_discard();
    let active_document = app.session.active().map(|d| d.uid);
    let current_viewport = ctx.viewport_id();
    let len = app.shots.requests.len();
    let mut failures: Vec<(usize, String)> = Vec::new();
    for i in 0..len {
        let (issued, ready_at, settle_pass) = {
            let r = &app.shots.requests[i];
            (r.issued, r.ready_at, r.settle_pass)
        };
        if issued || discard || settle_pass == Some(pass) {
            continue;
        }
        // One count per egui pass, however many times this `ui` runs in it.
        let frames = {
            let r = &mut app.shots.requests[i];
            r.settle_pass = Some(pass);
            r.frames = r.frames.saturating_add(1);
            r.frames
        };
        if now < ready_at || frames < 3 {
            continue;
        }
        // A queued capture belongs to the document and viewport it was admitted for. A switch
        // before issuance is reported once and never rebound to the new document.
        let (viewport_id, document, base64) = {
            let r = &app.shots.requests[i];
            (r.viewport_id, r.document, r.path.is_none())
        };
        if viewport_id != current_viewport {
            failures.push((i, SCREENSHOT_VIEWPORT_CHANGED.into()));
            continue;
        }
        if document != active_document {
            failures.push((i, SCREENSHOT_DOCUMENT_CHANGED.into()));
            continue;
        }
        let current = current_capture_geometry(app, ctx);
        let Some(expected) = current.expected_size() else {
            // Still minimized or not laid out: keep the request queued and try again on a later
            // real pass; `drain_screenshots` reports the size-specific timeout if none arrives.
            app.shots.requests[i].expected = None;
            continue;
        };
        let bytes = match capture_bytes(app, i, current, Some(expected), base64) {
            Ok(bytes) => bytes,
            Err(message) => {
                failures.push((i, message));
                continue;
            }
        };
        // Update the geometry and its epoch together, so the drain accepts exactly the layout the
        // capture is issued against even when a hidden logic tick already saw the new viewport.
        let epoch = note_capture_geometry(app, current);
        let r = &mut app.shots.requests[i];
        r.geometry = current;
        r.expected = Some(expected);
        r.bytes = bytes;
        r.epoch = epoch;
        if let Some(old) = r.slot.take() {
            old.cancel();
        }
        let (slot, callback) = CaptureSlot::new(ctx, viewport_id, bytes);
        r.slot = Some(slot);
        ctx.send_viewport_cmd_to(viewport_id, egui::ViewportCommand::Screenshot(callback));
        r.issued = true;
    }
    for (i, message) in failures.into_iter().rev() {
        reject_shot_at(app, i, message);
    }
    if app.shots.in_flight() {
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}

/// Checks that the image has the pixel count its dimensions imply, is within one pixel of the expected size on each axis, and keeps the expected aspect ratio within that rounding.
fn validate_image(image: &egui::ColorImage, expected: [usize; 2]) -> Result<(), String> {
    let [w, h] = image.size;
    let [ew, eh] = expected;
    let pixels = w.checked_mul(h).ok_or_else(|| "screenshot dimensions overflow".to_string())?;
    if image.pixels.len() != pixels {
        return Err(format!("screenshot has {} pixels for its {w}×{h} dimensions", image.pixels.len()));
    }
    // Allow one physical pixel of rounding per axis, but not independent distortion: an image
    // that is one pixel wider and one pixel shorter has a different aspect and cannot be the
    // recorded viewport. Rejecting it forces the retry/timeout path instead of saving skewed pixels.
    if w.abs_diff(ew) > 1 || h.abs_diff(eh) > 1 {
        return Err(format!("screenshot is {w}×{h} pixels, expected {ew}×{eh}: the window changed size while it was captured"));
    }
    // Keep the returned scale's aspect within the rounding error of the recorded logical viewport.
    let cross = w.checked_mul(eh).and_then(|a| h.checked_mul(ew).map(|b| a.abs_diff(b)));
    let Some(cross) = cross else {
        return Err("screenshot dimensions overflow the aspect check".to_string());
    };
    if cross > ew.max(eh) {
        return Err(format!("screenshot is {w}×{h} pixels, expected {ew}×{eh}: the aspect ratio changed while it was captured"));
    }
    Ok(())
}

/// What [`drain_screenshots`] must do with one request after inspecting it.
enum DrainAction {
    /// Still waiting; leave the request at the same index.
    Keep,
    /// Answer the request with this reply and remove it.
    Remove(Json),
    /// Re-arm the request for a fresh capture within its retry budget.
    Restart,
}

/// Called from `logic`: takes each completed slot once, validates identity, size, geometry/epoch
/// and **deadline** and only then encodes, writes and replies outside the slot. A stale, oversized
/// or late image is dropped without writing a file.
pub fn drain_screenshots(app: &mut SheetApp, ctx: &egui::Context) {
    let now = crate::monotonic_ms();
    let current_geometry = current_capture_geometry(app, ctx);
    let epoch = note_capture_geometry(app, current_geometry);
    let active_document = app.session.active().map(|d| d.uid);
    let current_viewport = ctx.viewport_id();
    let mut i = 0;
    while i < app.shots.requests.len() {
        match drain_action(app, i, now, current_geometry, epoch, active_document, current_viewport) {
            DrainAction::Keep => i += 1,
            DrainAction::Remove(reply) => finish_shot(app, i, reply),
            DrainAction::Restart => match retry_capture(app, i, current_geometry, epoch, now) {
                Ok(()) => i += 1,
                Err(message) => reject_shot_at(app, i, message),
            },
        }
    }
    if app.shots.in_flight() {
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
    }
}

/// Validates one request and decides whether it stays queued, is answered or is re-armed.
fn drain_action(
    app: &mut SheetApp,
    i: usize,
    now: f64,
    current_geometry: CaptureGeometry,
    epoch: u64,
    active_document: Option<u64>,
    current_viewport: egui::ViewportId,
) -> DrainAction {
    let r = &mut app.shots.requests[i];
    // A document or viewport change is a definitive mismatch: cancel once, never retry and
    // never write the pixels against the new identity.
    if r.viewport_id != current_viewport {
        cancel_shot(r);
        return DrainAction::Remove(json!({"ok": false, "error": SCREENSHOT_VIEWPORT_CHANGED}));
    }
    if r.document != active_document {
        cancel_shot(r);
        return DrainAction::Remove(json!({"ok": false, "error": SCREENSHOT_DOCUMENT_CHANGED}));
    }
    if !r.issued {
        return drain_unissued(r, now);
    }
    drain_completion(r, now, current_geometry, epoch)
}

/// A request that never issued a capture: keep waiting, or report the timeout that fits whether
/// the window ever had a capturable size.
fn drain_unissued(r: &mut Shot, now: f64) -> DrainAction {
    if now < r.deadline {
        return DrainAction::Keep;
    }
    cancel_shot(r);
    let message = if r.expected.is_none() {
        "screenshot timeout: the window never reported a capturable size (it may be minimized or not laid out)"
    } else {
        "no frame was presented before the screenshot timeout"
    };
    DrainAction::Remove(timeout_reply(message, r.token))
}

/// A request that issued a capture: inspect its slot once, then apply the deadline, validation
/// and geometry checks in the same order as issuance.
fn drain_completion(r: &mut Shot, now: f64, current_geometry: CaptureGeometry, epoch: u64) -> DrainAction {
    match r.slot.as_ref().and_then(|slot| slot.take()) {
        // A completion that arrives past the deadline is late even if the pixels are valid: the
        // client was promised one answer within the deadline, so nothing is written for it.
        Some(result) if now >= r.deadline => {
            drop(result);
            DrainAction::Remove(timeout_reply("screenshot timeout: the image completed after its deadline; no file was written", r.token))
        }
        Some(Ok(image)) => drain_image(r, image, now, current_geometry, epoch),
        // The slot refused the image before `validate_image` saw it: an image larger than the
        // reservation is as much evidence of a resize as a smaller one. Retry within the same
        // bounded budget instead of failing only when the window grew. A non-resize fault still
        // exhausts the retries and reports the slot's error, so this stays fail-closed.
        Some(Err(_)) if retry_allowed(r, now) => DrainAction::Restart,
        Some(Err(e)) => DrainAction::Remove(json!({"ok": false, "error": e.to_string()})),
        None if now >= r.deadline => {
            cancel_shot(r);
            DrainAction::Remove(timeout_reply("no frame was presented before the screenshot timeout", r.token))
        }
        None => DrainAction::Keep,
    }
}

/// Validates a completed image against the size recorded at issuance and the current
/// geometry/epoch; only a valid image is saved, and only a retry that is still inside the
/// request's deadline and retry budget gets a fresh attempt.
fn drain_image(r: &mut Shot, image: std::sync::Arc<egui::ColorImage>, now: f64, current_geometry: CaptureGeometry, epoch: u64) -> DrainAction {
    let validated = match r.expected {
        Some(expected) => validate_image(&image, expected),
        None => Err("screenshot has no expected size".to_string()),
    };
    match validated {
        Err(_) if retry_allowed(r, now) => {
            drop(image);
            DrainAction::Restart
        }
        Err(message) => DrainAction::Remove(json!({"ok": false, "error": message})),
        Ok(()) if epoch != r.epoch || current_geometry != r.geometry => {
            if !retry_allowed(r, now) {
                return DrainAction::Remove(json!({
                    "ok": false,
                    "error": "screenshot geometry changed while it was captured; no image was written"
                }));
            }
            drop(image);
            DrainAction::Restart
        }
        Ok(()) => DrainAction::Remove(save_image(image, r.path.as_deref())),
    }
}

/// Whether this request may take another capture: both limits are fixed for the whole request.
fn retry_allowed(r: &Shot, now: f64) -> bool {
    r.retries < SCREENSHOT_MAX_RETRIES && now < r.deadline
}

/// Takes and cancels the request's capture slot, if it has one.
fn cancel_shot(r: &mut Shot) {
    if let Some(slot) = r.slot.take() {
        slot.cancel();
    }
}

/// Remove request `i`, cancel its slot and answer it once with a token-carrying reply.
fn finish_shot(app: &mut SheetApp, i: usize, reply: Json) {
    let request = app.shots.requests.remove(i);
    if let Some(slot) = request.slot {
        slot.cancel();
    }
    let _ = request.reply.send(with_token(reply, request.token));
}

/// Remove request `i`, cancel its slot and reject it with `message`.
fn reject_shot_at(app: &mut SheetApp, i: usize, message: impl std::fmt::Display) {
    let request = app.shots.requests.remove(i);
    if let Some(slot) = request.slot {
        slot.cancel();
    }
    reject_shot(request.reply, request.token, message);
}

/// Re-arm one request for a fresh issuance against `geometry`: a new reservation (the maximum
/// retained image, or [`UNKNOWN_CAPTURE_BYTES`] while there is no size), a new epoch and no slot.
/// A fresh slot and callback are created by [`issue_screenshots`]; the old image was dropped by
/// the caller.
fn retry_capture(app: &mut SheetApp, i: usize, geometry: CaptureGeometry, epoch: u64, now: f64) -> Result<(), String> {
    let expected = geometry.expected_size();
    let base64 = app.shots.requests.get(i).is_some_and(|r| r.path.is_none());
    let bytes = capture_bytes(app, i, geometry, expected, base64)?;
    let r = &mut app.shots.requests[i];
    r.retries = r.retries.saturating_add(1);
    r.issued = false;
    r.frames = 3;
    r.settle_pass = None;
    r.ready_at = now;
    r.expected = expected;
    r.bytes = bytes;
    r.geometry = geometry;
    r.epoch = epoch;
    if let Some(slot) = r.slot.take() {
        slot.cancel();
    }
    Ok(())
}

/// Encodes a screenshot as PNG to `path`, or returns it as base64.
///
/// The retained `ColorImage` is released before encoding, and a path capture streams the PNG
/// straight to the file. A base64 reply is length-checked against
/// [`SCREENSHOT_MAX_BASE64_BYTES`] *before* the string is allocated, so one admitted capture cannot
/// retain an unbounded reply alongside its image.
pub fn save_image(image: std::sync::Arc<egui::ColorImage>, path: Option<&str>) -> Json {
    use image::ImageEncoder as _;
    let [w, h] = image.size;
    let Some(capacity) = w.checked_mul(h).and_then(|p| p.checked_mul(4)) else { return json!({"ok": false, "error": "bad image"}) };
    if image.pixels.len().checked_mul(4) != Some(capacity) {
        return json!({"ok": false, "error": "bad image"});
    }
    let (Ok(width), Ok(height)) = (u32::try_from(w), u32::try_from(h)) else { return json!({"ok": false, "error": "bad image"}) };
    // One RGBA copy is unavoidable with the `image` encoder API. Drop the retained image before
    // encoding so the peak is the copy plus encoder buffers, not both images plus the PNG stream.
    let mut rgba = Vec::with_capacity(capacity);
    for p in &image.pixels {
        rgba.extend_from_slice(&p.to_array());
    }
    drop(image);
    match path {
        Some(p) => match std::fs::File::create(p) {
            Ok(file) => {
                let encoder = image::codecs::png::PngEncoder::new(file);
                match encoder.write_image(&rgba, width, height, image::ExtendedColorType::Rgba8) {
                    Ok(()) => json!({"ok": true, "result": {"path": p, "width": w, "height": h}}),
                    Err(e) => {
                        // Do not leave a truncated PNG behind.
                        let _ = std::fs::remove_file(p);
                        json!({"ok": false, "error": e.to_string()})
                    }
                }
            }
            Err(e) => json!({"ok": false, "error": e.to_string()}),
        },
        None => {
            let mut png = Vec::new();
            let encoder = image::codecs::png::PngEncoder::new(&mut png);
            if let Err(e) = encoder.write_image(&rgba, width, height, image::ExtendedColorType::Rgba8) {
                return json!({"ok": false, "error": e.to_string()});
            }
            let Some(encoded_len) = base64_len(png.len()) else {
                return json!({"ok": false, "error": "screenshot base64 length overflow"});
            };
            if encoded_len > SCREENSHOT_MAX_BASE64_BYTES {
                return json!({
                    "ok": false,
                    "error": format!(
                        "screenshot base64 budget exceeded: the {w}×{h} reply would be {encoded_len} bytes, over the {SCREENSHOT_MAX_BASE64_BYTES}-byte base64 limit; request a path instead"
                    )
                });
            }
            json!({"ok": true, "result": {"base64": gridcraft_engine::io::base64_encode(&png), "width": w, "height": h}})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Json> {
        vec![
            json!({"id": "home.borders", "label": "Borders", "menu": ["Home", "Font"]}),
            json!({"id": "home.bold", "label": "Bold", "menu": ["Home", "Font"]}),
            json!({"id": "ui.ribbonTab", "label": "Ribbon Tab", "enabled": true, "ui": true}),
        ]
    }

    fn ids(v: &[Json]) -> Vec<&str> {
        v.iter().filter_map(|c| c.get("id").and_then(Json::as_str)).collect()
    }

    #[test]
    fn search_filters_case_insensitively() {
        assert_eq!(ids(&filter_commands(sample(), Some("border"))), ["home.borders"]);
        assert_eq!(filter_commands(sample(), Some("BORDER")), filter_commands(sample(), Some("border")));
        assert_eq!(ids(&filter_commands(sample(), Some("ribbon"))), ["ui.ribbonTab"]);
        assert_eq!(ids(&filter_commands(sample(), Some("font"))), ["home.borders", "home.bold"]);
        assert!(filter_commands(sample(), Some("catalog_no_match_7391")).is_empty());
        assert_eq!(filter_commands(sample(), None).len(), 3);
    }

    fn test_app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        SheetApp::new(session, crate::Services::default())
    }

    #[test]
    /// Checks that oversized click/drag counts are refused without changing already queued batches, that queued input counts toward the budget, and that normal gestures still queue fully.
    fn oversized_pointer_requests_are_refused_and_leave_the_queue_alone() {
        let mut app = test_app();
        let p = egui::pos2(20.0, 20.0);
        let m = egui::Modifiers::NONE;
        let btn = egui::PointerButton::Primary;
        assert!(push_click(&mut app, p, btn, 2, m).is_ok());
        assert_eq!(app.synthetic.iter().map(Vec::len).sum::<usize>(), 5);
        assert!(push_click(&mut app, p, btn, u32::MAX, m).is_err());
        assert!(push_drag(&mut app, p, p, usize::MAX, m).is_err());
        assert!(push_drag(&mut app, p, p, MAX_QUEUED_SYNTHETIC_EVENTS, m).is_err());
        assert_eq!(app.synthetic.len(), 2);
        assert_eq!(app.synthetic.iter().map(Vec::len).sum::<usize>(), 5);
        // Already queued input counts toward the budget.
        assert!(push_click(&mut app, p, btn, (MAX_QUEUED_SYNTHETIC_EVENTS / 2) as u32, m).is_err());
        assert_eq!(app.synthetic.len(), 2);
        // Normal requests still queue in full.
        assert!(push_drag(&mut app, p, p + egui::vec2(40.0, 40.0), 3, m).is_ok());
        assert_eq!(app.synthetic.len(), 8);
    }

    #[test]
    fn print_command_discovery_documents_settings_and_submission() {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        let mut app = SheetApp::new(session, crate::Services::default());
        let Outcome::Done(reply) = handle(&mut app, &egui::Context::default(), "engine.commands", &json!({"search":"print"})) else {
            panic!("command discovery must return immediately");
        };
        assert_eq!(reply["ok"], true);
        let commands = reply["result"].as_array().unwrap();
        for (id, expected_docs) in [
            ("ui.printSettings", &["copies", "duplex", "refresh", "discovering", "status", "detail"][..]),
            ("ui.printSubmit", &["file.print", "pagination", "pending", "without opening a dialog"][..]),
        ] {
            let command = commands.iter().find(|command| command["id"] == id).unwrap();
            assert_eq!(command["ui"], true);
            let docs = command["params"].as_str().unwrap();
            for expected in expected_docs {
                assert!(docs.contains(expected), "{id}: missing {expected}");
            }
        }
        assert!(!app.printing.busy(), "discovery must not submit a job");
        assert!(app.backstage.is_none(), "discovery must not open settings");
    }

    #[test]
    fn language_command_docs_list_every_language() {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        let mut app = SheetApp::new(session, crate::Services::default());
        let Outcome::Done(reply) = handle(&mut app, &egui::Context::default(), "engine.commands", &json!({"search":"language"})) else {
            panic!("command discovery must return immediately");
        };
        let commands = reply["result"].as_array().unwrap();
        let docs = commands.iter().find(|c| c["id"] == "app.language.set").unwrap()["params"].as_str().unwrap();
        for l in crate::i18n::Language::ALL {
            assert!(docs.contains(&format!("\"{}\"", l.code())), "app.language.set docs miss {:?}: {docs}", l.code());
        }
    }
    /// One headless UI pass. A test has no renderer to apply the font-atlas delta, so dropping
    /// it is intentional (0.37 panics when a `TexturesDelta` is dropped unapplied).
    fn ui_pass(ctx: &egui::Context, raw: egui::RawInput, f: impl FnMut(&mut egui::Ui)) -> egui::FullOutput {
        let mut out = ctx.run_ui(raw, f);
        out.textures_delta.clear();
        out
    }

    /// Returns an egui context initialized by one blank pass at `size` with an 8192-pixel texture limit.
    fn sized_context(size: egui::Vec2) -> egui::Context {
        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            max_texture_side: Some(8192),
            ..Default::default()
        };
        let _ = ui_pass(&ctx, raw, |_| {});
        ctx
    }

    /// Returns blank raw input with a screen rect at `size` and an 8192-pixel texture limit.
    fn raw_of(size: egui::Vec2) -> egui::RawInput {
        egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)), max_texture_side: Some(8192), ..Default::default() }
    }

    /// Extracts cloned `ViewportCommand::Screenshot` callbacks from a `FullOutput` for the context's viewport.
    fn screenshot_callbacks(out: &egui::FullOutput, ctx: &egui::Context) -> Vec<egui::ScreenshotCallback> {
        out.viewport_output
            .get(&ctx.viewport_id())
            .map(|output| {
                output
                    .commands
                    .iter()
                    .filter_map(|command| match command {
                        egui::ViewportCommand::Screenshot(callback) => Some(callback.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default()
    }

    /// Run three settled passes with `raw` and return the callbacks issued in them.
    fn issue_with(ctx: &egui::Context, app: &mut SheetApp, raw: egui::RawInput) -> Vec<egui::ScreenshotCallback> {
        let mut callbacks = Vec::new();
        for _ in 0..3 {
            let out = ui_pass(ctx, raw.clone(), |ui| issue_screenshots(app, ui.ctx()));
            callbacks.extend(screenshot_callbacks(&out, ctx));
        }
        callbacks
    }

    /// Returns a per-process temporary PNG path after removing any stale file at that path.
    fn temp_png(name: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("gridcraft-shot-{name}-{}.png", std::process::id()));
        let _ = std::fs::remove_file(&path);
        path
    }

    #[test]
    /// Checks rounded physical pixels, expected/retained byte counts, the texture-axis cap, the no-size case and overflow refusal.
    fn capture_geometry_reports_checked_capped_pixels() {
        let geometry = CaptureGeometry {
            viewport_id: egui::ViewportId::ROOT,
            viewport: Some(egui::vec2(800.0, 600.0)),
            pixels_per_point: 2.0,
            zoom: 1.0,
            max_texture_side: 4096,
            minimized: None,
            occluded: None,
            visible: None,
        };
        assert_eq!(geometry.expected_size(), Some([1600, 1200]));
        assert_eq!(geometry.expected_bytes(), Some(1600 * 1200 * 4));
        assert_eq!(geometry.retained_bytes(), Some(1601 * 1201 * 4), "one pixel of rounding per axis is reserved too");
        let capped = CaptureGeometry { max_texture_side: 1024, ..geometry };
        assert_eq!(capped.expected_size(), Some([1024, 1024]), "the surface limit caps both axes");
        let empty = CaptureGeometry { viewport: Some(egui::Vec2::ZERO), ..geometry };
        assert_eq!(empty.expected_size(), None, "a minimized window has no capturable size");
        assert_eq!(empty.retained_bytes(), None);
        let huge = CaptureGeometry { viewport: Some(egui::vec2(f32::MAX, f32::MAX)), ..geometry };
        assert_eq!(huge.expected_size(), None, "overflow is refused, not truncated");
    }

    #[test]
    /// Checks that one request beyond `SCREENSHOT_MAX_REQUESTS` is rejected with its token while earlier requests stay queued and do not reply early.
    fn screenshot_admission_stops_on_the_count_budget_without_evicting() {
        let ctx = sized_context(egui::vec2(800.0, 600.0));
        let mut app = test_app();
        let mut replies = Vec::new();
        for _ in 0..SCREENSHOT_MAX_REQUESTS {
            let (tx, rx) = std::sync::mpsc::channel();
            admit(&mut app, &ctx, None, tx);
            replies.push(rx);
        }
        assert_eq!(app.shots.requests.len(), SCREENSHOT_MAX_REQUESTS);
        assert!(replies.iter_mut().all(|r| r.try_recv().is_err()), "admitted requests do not reply early");
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        let reply = rx.try_recv().expect("the over-count request is rejected");
        assert_eq!(reply["ok"], false);
        assert!(reply["error"].as_str().is_some_and(|e| e.contains("limit 4")), "{reply}");
        assert!(reply["request"].as_u64().is_some(), "a rejection carries its request token: {reply}");
        assert_eq!(app.shots.requests.len(), SCREENSHOT_MAX_REQUESTS, "an earlier request is never evicted");
    }

    #[test]
    /// Three roughly 100 MB path captures hit the aggregate byte limit; the third is rejected and only two requests remain.
    fn screenshot_admission_stops_on_aggregate_bytes_before_the_count() {
        let ctx = sized_context(egui::vec2(5000.0, 5000.0));
        let mut app = test_app();
        let path = temp_png("aggregate").to_string_lossy().into_owned();
        let mut rejected = None;
        for _ in 0..3 {
            let (tx, rx) = std::sync::mpsc::channel();
            // A path capture (not base64) isolates the retained-image aggregate budget.
            admit(&mut app, &ctx, Some(path.clone()), tx);
            if let Ok(reply) = rx.try_recv() {
                rejected = Some(reply);
            }
        }
        let reply = rejected.expect("three 100 MB captures exceed the 256 MiB aggregate budget");
        assert!(reply["error"].as_str().is_some_and(|e| e.contains("aggregate limit")), "{reply}");
        assert_eq!(app.shots.requests.len(), 2);
    }

    #[test]
    /// An unknown-size request reserves the placeholder, issues nothing while settling, then replies once on timeout and releases the reservation.
    fn requests_without_a_size_stay_queued_and_time_out() {
        let ctx = sized_context(egui::Vec2::ZERO);
        let mut app = test_app();
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        assert!(rx.try_recv().is_err(), "an unknown size is queued, not refused with an overflow error");
        assert_eq!(app.shots.requests[0].bytes, UNKNOWN_CAPTURE_BYTES, "the largest possible image is reserved meanwhile");
        // Settling passes without a size neither issue nor fail the request.
        let callbacks = issue_with(&ctx, &mut app, raw_of(egui::Vec2::ZERO));
        assert!(callbacks.is_empty());
        assert!(!app.shots.requests[0].issued);
        assert!(rx.try_recv().is_err());
        app.shots.requests[0].deadline = 0.0;
        let _ = ui_pass(&ctx, raw_of(egui::Vec2::ZERO), |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the size-less request times out once");
        assert!(reply["error"].as_str().is_some_and(|e| e.contains("capturable size")), "{reply}");
        assert_eq!(app.shots.reserved(), 0, "the placeholder reservation is released");
    }

    /// Counts `RequestPaintWhileHidden` commands in a logic output.
    fn hidden_paints(out: &egui::LogicOutput) -> usize {
        out.viewport_commands.values().flatten().filter(|c| matches!(c, egui::ViewportCommand::RequestPaintWhileHidden)).count()
    }

    #[test]
    /// A queued capture requests a hidden paint; once issued, the screenshot command supplies the paint itself.
    fn a_queued_capture_requests_hidden_paint_until_it_is_issued() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let (tx, _rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        let queued = ctx.run_logic(&raw_of(egui::vec2(64.0, 48.0)), |ctx| app.logic(ctx));
        assert_eq!(hidden_paints(&queued), 1, "a queued capture needs a real `ui` pass");

        app.shots.requests[0].ready_at = 0.0;
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 1);
        let issued = ctx.run_logic(&raw_of(egui::vec2(64.0, 48.0)), |ctx| app.logic(ctx));
        assert_eq!(hidden_paints(&issued), 0, "the issued Screenshot command paints the hidden window by itself");
    }

    #[test]
    /// A minimized request stays queued without requesting hidden paints, because it cannot issue until restored.
    fn a_minimized_capture_does_not_force_hidden_frames_while_it_waits() {
        let ctx = egui::Context::default();
        let mut app = test_app();
        let mut raw = raw_of(egui::vec2(800.0, 600.0));
        if let Some(v) = raw.viewports.get_mut(&egui::ViewportId::ROOT) {
            v.inner_rect = None;
            v.minimized = Some(true);
        }
        app.raw_input_hook(&ctx, &mut raw);
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        assert!(rx.try_recv().is_err());
        let waited = ctx.run_logic(&raw, |ctx| app.logic(ctx));
        assert_eq!(hidden_paints(&waited), 0, "a minimized request cannot issue until it is restored");
        app.shots.cancel_all();
    }

    #[test]
    /// The real app harness issues exactly one callback, replies once with the token and dimensions, and releases the reservation.
    fn one_admitted_capture_issues_one_callback_and_replies_once() {
        use egui_kittest::kittest::Queryable as _;

        let mut app = test_app();
        app.use_prepared_fonts();
        let mut h = egui_kittest::Harness::builder()
            .with_size(egui::vec2(900.0, 700.0))
            .with_context_setup(|ctx| {
                ctx.set_fonts(crate::theme::pinned_font_definitions());
                crate::theme::apply(ctx, false);
            })
            // This test completes the callback itself. Keep the real app/AccessKit tree, but
            // do not let kittest's automatic screenshot handling initialize a GPU renderer.
            .renderer(egui_kittest::LazyRenderer::Uninitialized { textures_delta: Default::default(), builder: None })
            .build_ui_state(
                |ui, app: &mut SheetApp| {
                    app.logic(ui.ctx());
                    app.ui(ui);
                },
                app,
            );
        h.run_steps(4);
        assert!(h.query_by_label("Name Box").is_some(), "the capture fixture draws the real app, not an empty tree");
        h.check_accessibility();

        let ctx = h.ctx.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        admit(h.state_mut(), &ctx, None, tx);
        let token = h.state().shots.requests[0].token;
        h.state_mut().shots.requests[0].ready_at = 0.0;
        let mut callbacks = Vec::new();
        for _ in 0..3 {
            h.step();
            callbacks.extend(screenshot_callbacks(h.output(), &ctx));
        }
        assert_eq!(callbacks.len(), 1, "the settled request is issued once");
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([900, 700], egui::Color32::RED)));
        h.step(); // the real logic drains the callback and the UI repaints
        h.check_accessibility();
        assert!(h.query_by_label("Name Box").is_some(), "the app tree survives callback drain and repaint");
        let reply = rx.try_recv().expect("the drain replies once");
        assert_eq!(reply["ok"], true, "{reply}");
        assert_eq!(reply["result"]["width"], 900);
        assert_eq!(reply["result"]["height"], 700);
        assert_eq!(reply["request"].as_u64(), Some(token), "the reply names its request");
        assert!(rx.try_recv().is_err(), "a completed slot is taken exactly once");
        assert_eq!(h.state().shots.reserved(), 0, "the reservation is released after the reply");
    }

    #[test]
    /// Completing callbacks in reverse order replies on each matching token/path and writes each request's own pixels.
    fn out_of_order_completions_reply_on_their_own_request() {
        let ctx = sized_context(egui::vec2(8.0, 8.0));
        let mut app = test_app();
        let p1 = temp_png("order-1");
        let p2 = temp_png("order-2");
        let (tx1, rx1) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(p1.to_string_lossy().into_owned()), tx1);
        let token1 = app.shots.requests[0].token;
        let (tx2, rx2) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(p2.to_string_lossy().into_owned()), tx2);
        let token2 = app.shots.requests[1].token;
        for request in &mut app.shots.requests {
            request.ready_at = 0.0;
        }
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 2, "one callback per admitted request");
        // Complete the second request first: slots, not callback order, carry the result.
        callbacks[1].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([8, 8], egui::Color32::BLUE)));
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        let second = rx2.try_recv().expect("the second request gets its own reply");
        assert_eq!(second["ok"], true, "{second}");
        assert_eq!(second["request"].as_u64(), Some(token2));
        assert_eq!(second["result"]["path"].as_str(), Some(p2.to_string_lossy().as_ref()));
        assert!(!p1.exists(), "the unfinished request has not written anything");
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([8, 8], egui::Color32::RED)));
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        let first = rx1.try_recv().expect("the first request gets its own reply");
        assert_eq!(first["request"].as_u64(), Some(token1));
        assert_eq!(first["result"]["path"].as_str(), Some(p1.to_string_lossy().as_ref()));
        assert_ne!(std::fs::read(&p1).unwrap(), std::fs::read(&p2).unwrap(), "each reply carries its own pixels");
        let _ = std::fs::remove_file(&p1);
        let _ = std::fs::remove_file(&p2);
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// An expired request replies once with a timeout, writes no file and releases its reservation.
    fn a_timed_out_capture_replies_once_and_never_writes() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let path = temp_png("timeout");
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx);
        app.shots.requests[0].deadline = 0.0;
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the timeout replies");
        assert_eq!(reply["ok"], false);
        assert!(reply["error"].as_str().is_some_and(|e| e.contains("timeout")), "{reply}");
        assert!(!path.exists(), "a timed-out capture never writes a file");
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        assert!(rx.try_recv().is_err(), "a timeout is reported once");
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// A valid image that completes after its deadline gets one timeout reply and is not written.
    fn a_capture_completed_after_its_deadline_writes_nothing() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let path = temp_png("late");
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx);
        let token = app.shots.requests[0].token;
        app.shots.requests[0].ready_at = 0.0;
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 1);
        // The deadline passes while the callback is outstanding; the image arrives afterwards.
        app.shots.requests[0].deadline = 0.0;
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([64, 48], egui::Color32::RED)));
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the late completion gets one timeout reply");
        assert_eq!(reply["ok"], false, "{reply}");
        assert!(reply["error"].as_str().is_some_and(|e| e.contains("timeout")), "{reply}");
        assert_eq!(reply["request"].as_u64(), Some(token));
        assert!(!path.exists(), "a capture that completed after its deadline never writes a file");
        assert!(rx.try_recv().is_err(), "the late image is reported once");
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// Cancelling a request does not reply; a later callback for it is ignored and writes nothing.
    fn a_callback_after_the_request_is_removed_is_inert() {
        let ctx = sized_context(egui::vec2(32.0, 24.0));
        let mut app = test_app();
        let path = temp_png("removed");
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx);
        app.shots.requests[0].ready_at = 0.0;
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 1);
        app.shots.cancel_all();
        assert!(rx.try_recv().is_err(), "cancelling a request does not reply");
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([32, 24], egui::Color32::RED)));
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        assert!(rx.try_recv().is_err(), "a callback for a removed request is inert");
        assert!(!path.exists());
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// The first issuance adopts a new geometry, updates the epoch and reservation, spends no retry, and the capture then succeeds.
    fn a_resize_between_admission_and_issuance_does_not_cost_a_retry() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        app.shots.requests[0].ready_at = 0.0;
        let admitted_epoch = app.shots.requests[0].epoch;
        let resized = raw_of(egui::vec2(32.0, 24.0));
        let callbacks = issue_with(&ctx, &mut app, resized.clone());
        assert_eq!(callbacks.len(), 1);
        let request = &app.shots.requests[0];
        assert_eq!(request.retries, 0, "the first issuance adopts the current geometry instead of spending a retry");
        assert_eq!(request.geometry.viewport, Some(egui::vec2(32.0, 24.0)));
        assert_eq!(request.expected, Some([32, 24]));
        assert_eq!(request.bytes, 33 * 25 * 4);
        assert!(request.epoch > admitted_epoch, "issuance records the new epoch together with the geometry");
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([32, 24], egui::Color32::RED)));
        let _ = ui_pass(&ctx, resized, |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the once-issued capture replies");
        assert_eq!(reply["ok"], true, "{reply}");
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// The reservation is the largest accepted rounded image (11×9×4 bytes for a 10×8 viewport), and that image is accepted.
    fn the_retained_reservation_is_the_slot_bound() {
        let ctx = sized_context(egui::vec2(10.0, 8.0));
        let mut app = test_app();
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        // validate_image accepts one pixel of rounding per axis: an 11x9 image is the largest.
        assert_eq!(app.shots.requests[0].bytes, 11 * 9 * 4);
        app.shots.requests[0].ready_at = 0.0;
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 1);
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([11, 9], egui::Color32::RED)));
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the rounded image is accepted");
        assert_eq!(reply["ok"], true, "{reply}");
        assert_eq!(reply["result"]["width"], 11);
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// Oversized images retry within the bounded budget, never write, then report the per-image limit, while the valid peer is written.
    fn an_image_beyond_the_reservation_is_released_without_writing() {
        let ctx = sized_context(egui::vec2(10.0, 8.0));
        let mut app = test_app();
        let p1 = temp_png("over-1");
        let p2 = temp_png("over-2");
        let (tx1, rx1) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(p1.to_string_lossy().into_owned()), tx1);
        let (tx2, rx2) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(p2.to_string_lossy().into_owned()), tx2);
        for request in &mut app.shots.requests {
            request.ready_at = 0.0;
        }
        assert_eq!(app.shots.reserved(), 2 * 11 * 9 * 4);
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 2);
        // The slot refuses an image over the reservation instead of retaining bytes beyond it.
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([12, 9], egui::Color32::RED)));
        // The other request completes with valid pixels.
        callbacks[1].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([10, 8], egui::Color32::BLUE)));
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        // A larger image can be a genuine resize, so it receives the same bounded retries as
        // a smaller one. Each oversized callback is released and never written while retrying.
        for attempt in 1..=SCREENSHOT_MAX_RETRIES {
            assert_eq!(app.shots.requests[0].retries, attempt);
            assert!(rx1.try_recv().is_err(), "retry {attempt} has not replied early");
            assert!(!p1.exists(), "oversized pixels are never written on retry {attempt}");
            let callbacks = issue_with(&ctx, &mut app, raw_of(egui::vec2(10.0, 8.0)));
            assert_eq!(callbacks.len(), 1, "the other valid request is already complete");
            callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([12, 9], egui::Color32::RED)));
            let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        }
        let refused = rx1.try_recv().expect("the oversized image is reported after bounded retries");
        assert_eq!(refused["ok"], false, "{refused}");
        assert!(refused["error"].as_str().is_some_and(|e| e.contains("per-image limit")), "{refused}");
        assert!(!p1.exists());
        let accepted = rx2.try_recv().expect("the valid image is written");
        assert_eq!(accepted["ok"], true, "{accepted}");
        let _ = std::fs::remove_file(&p2);
        assert_eq!(app.shots.reserved(), 0, "both reservations are released");
    }

    #[test]
    /// Two `ui` calls in one pass count as one settling frame, and a pass that requests a discard is not counted; only its rerun counts.
    fn settling_counts_one_real_pass_and_skips_a_discarded_one() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let (tx, _rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        // Two `ui` calls in one pass are one settling frame.
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| {
            issue_screenshots(&mut app, ui.ctx());
            issue_screenshots(&mut app, ui.ctx());
        });
        assert_eq!(app.shots.requests[0].frames, 1);
        // A pass that already asked to be discarded does not count, even though `run_ui` repeats it.
        let mut asked = false;
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| {
            if !asked {
                asked = true;
                ui.ctx().request_discard("settling test");
            }
            issue_screenshots(&mut app, ui.ctx());
        });
        assert_eq!(app.shots.requests[0].frames, 2, "only the rerun (real) pass counted");
    }

    #[test]
    /// Issuance survives a discard requested after it, the image is written once, and a duplicate callback completion is inert.
    fn a_discard_requested_after_issuance_still_issues_and_writes_once() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let path = temp_png("discard-after-issue");
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx);
        app.shots.requests[0].ready_at = 0.0;
        let mut asked = false;
        let mut callbacks = Vec::new();
        for _ in 0..3 {
            let out = ui_pass(&ctx, egui::RawInput::default(), |ui| {
                issue_screenshots(&mut app, ui.ctx());
                if !asked && app.shots.requests.first().is_some_and(|r| r.issued) {
                    asked = true;
                    ui.ctx().request_discard("discard after screenshot issuance");
                }
            });
            callbacks.extend(screenshot_callbacks(&out, &ctx));
        }
        assert!(asked, "the test requested a discard after issuance");
        assert_eq!(callbacks.len(), 1, "issuance happens exactly once across the discard repeat");
        let image = std::sync::Arc::new(egui::ColorImage::filled([64, 48], egui::Color32::RED));
        callbacks[0].clone().complete(image.clone());
        callbacks[0].clone().complete(image);
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the discarded pass still delivers one reply");
        assert_eq!(reply["ok"], true, "{reply}");
        assert!(path.exists(), "the image is written exactly once");
        assert!(rx.try_recv().is_err(), "a second completion of the same callback is inert");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    /// A grow-resize spends one retry, reissues at the new size and then accepts the new image.
    fn an_image_larger_than_the_reservation_retries_like_a_smaller_one() {
        let ctx = sized_context(egui::vec2(10.0, 8.0));
        let mut app = test_app();
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        app.shots.requests[0].ready_at = 0.0;
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 1);

        // The surface grew by two pixels while the image was outstanding. The slot's reservation
        // refuses the larger image before `validate_image`; drain must retry at the new geometry
        // instead of failing because the resize went in this direction.
        let mut grown = raw_of(egui::vec2(20.0, 16.0));
        app.raw_input_hook(&ctx, &mut grown);
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([12, 9], egui::Color32::RED)));
        let _ = ctx.run_logic(&grown, |ctx| app.logic(ctx));
        let request = &app.shots.requests[0];
        assert_eq!(request.retries, 1, "the slot's per-image refusal spends one retry, not a failure");
        assert!(!request.issued);
        assert_eq!(request.geometry.viewport, Some(egui::vec2(20.0, 16.0)));
        assert!(rx.try_recv().is_err(), "a retried capture does not reply yet");

        let callbacks = issue_with(&ctx, &mut app, raw_of(egui::vec2(20.0, 16.0)));
        assert_eq!(callbacks.len(), 1);
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([20, 16], egui::Color32::BLUE)));
        let _ = ui_pass(&ctx, raw_of(egui::vec2(20.0, 16.0)), |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the retried capture replies");
        assert_eq!(reply["ok"], true, "{reply}");
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// Minimize retries reserve unknown-size bytes; only two fit the aggregate budget and stay queued (the rest are reported), and the survivors write after restore.
    fn a_minimize_retry_without_a_size_still_checks_the_aggregate_budget() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let mut paths = Vec::new();
        let mut receivers = Vec::new();
        for n in 0..SCREENSHOT_MAX_REQUESTS {
            let path = temp_png(&format!("minimize-retry-{n}"));
            let (tx, rx) = std::sync::mpsc::channel();
            // A path capture isolates the retained-image aggregate budget.
            admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx);
            paths.push(path);
            receivers.push(rx);
        }
        for request in &mut app.shots.requests {
            request.ready_at = 0.0;
        }
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), SCREENSHOT_MAX_REQUESTS, "four path captures fit every budget");
        assert_eq!(app.shots.reserved(), SCREENSHOT_MAX_REQUESTS * 65 * 49 * 4);

        // The window is minimized with every capture outstanding. The drain retries each one
        // against the new surface generation, which has no capturable size, so every retry
        // reserves UNKNOWN_CAPTURE_BYTES. Only two unknown-size reservations fit the aggregate
        // limit; the rest must be reported instead of inflating `reserved()` over the budget.
        let mut minimized = raw_of(egui::vec2(64.0, 48.0));
        if let Some(viewport) = minimized.viewports.get_mut(&egui::ViewportId::ROOT) {
            viewport.inner_rect = None;
            viewport.minimized = Some(true);
        }
        app.raw_input_hook(&ctx, &mut minimized);
        let image = std::sync::Arc::new(egui::ColorImage::filled([64, 48], egui::Color32::RED));
        for callback in &callbacks {
            callback.clone().complete(image.clone());
        }
        let _ = ctx.run_logic(&minimized, |ctx| app.logic(ctx));

        assert!(
            app.shots.reserved() <= SCREENSHOT_MAX_AGGREGATE_BYTES,
            "size-less retries reserved {} bytes, over the {SCREENSHOT_MAX_AGGREGATE_BYTES}-byte aggregate limit",
            app.shots.reserved()
        );
        let rejected: Vec<Json> = receivers.iter_mut().filter_map(|rx| rx.try_recv().ok()).collect();
        assert_eq!(rejected.len(), 2, "two unknown-size retries no longer fit the aggregate budget");
        assert!(rejected.iter().all(|reply| reply["error"].as_str().is_some_and(|e| e.contains("aggregate limit"))), "{rejected:?}");
        assert_eq!(app.shots.requests.len(), 2, "the two fitting retries stay queued");
        assert!(app.shots.requests.iter().all(|r| !r.issued && r.expected.is_none() && r.bytes == UNKNOWN_CAPTURE_BYTES));
        assert!(receivers.iter_mut().all(|rx| rx.try_recv().is_err()), "the surviving retries wait for the restore");

        // Restoring re-issues only the surviving requests at the real size; both fit because the
        // over-reserved retries were rejected rather than kept.
        let mut restored = raw_of(egui::vec2(64.0, 48.0));
        if let Some(viewport) = restored.viewports.get_mut(&egui::ViewportId::ROOT) {
            viewport.inner_rect = Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(64.0, 48.0)));
            viewport.minimized = Some(false);
            viewport.occluded = Some(false);
        }
        app.raw_input_hook(&ctx, &mut restored);
        let callbacks = issue_with(&ctx, &mut app, restored.clone());
        assert_eq!(callbacks.len(), 2, "the surviving requests issue on restore");
        for callback in &callbacks {
            callback.clone().complete(image.clone());
        }
        let _ = ctx.run_logic(&restored, |ctx| app.logic(ctx));

        let mut delivered = 0;
        for rx in &mut receivers {
            if let Ok(reply) = rx.try_recv() {
                assert_eq!(reply["ok"], true, "{reply}");
                delivered += 1;
            }
        }
        assert_eq!(delivered, 2, "the two surviving captures reply once each");
        let written = paths.iter().filter(|path| path.exists()).count();
        assert_eq!(written, 2, "only the surviving captures write their image");
        for path in &paths {
            let _ = std::fs::remove_file(path);
        }
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// Fresh hidden viewport geometry invalidates the old image, spends one retry, and the reissued image succeeds.
    fn a_hidden_resize_rejects_an_image_issued_before_it_and_reissues() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        app.shots.requests[0].ready_at = 0.0;
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 1);
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([64, 48], egui::Color32::RED)));

        // A hidden tick carries the resize in `RawInput::viewports`; the last UI pass still says
        // 64x48, so only the fresh viewport info lets the drain see the change.
        let hidden = || {
            let mut raw = raw_of(egui::vec2(64.0, 48.0));
            if let Some(viewport) = raw.viewports.get_mut(&egui::ViewportId::ROOT) {
                viewport.inner_rect = Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(32.0, 400.0)));
                viewport.native_pixels_per_point = Some(1.0);
            }
            raw
        };
        let _ = ctx.run_logic(&hidden(), |ctx| app.logic(ctx));
        let request = &app.shots.requests[0];
        assert!(!request.issued, "the image captured before the hidden resize is not accepted");
        assert_eq!(request.retries, 1);
        assert_eq!(request.geometry.viewport, Some(egui::vec2(32.0, 400.0)));
        assert!(rx.try_recv().is_err(), "a retried capture does not reply yet");

        // The next real pass issues against the new size and its image is accepted.
        let resized = raw_of(egui::vec2(32.0, 24.0));
        let callbacks = issue_with(&ctx, &mut app, resized.clone());
        assert_eq!(callbacks.len(), 1, "the retry is issued once against the current geometry");
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([32, 24], egui::Color32::BLUE)));
        let _ = ui_pass(&ctx, resized, |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the retried capture replies");
        assert_eq!(reply["ok"], true, "{reply}");
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// Draining four completed captures replies to all of them and permits a new admission.
    fn a_drained_capture_frees_its_slot_for_the_next_admission() {
        let ctx = sized_context(egui::vec2(8.0, 8.0));
        let mut app = test_app();
        let mut receivers = Vec::new();
        for _ in 0..SCREENSHOT_MAX_REQUESTS {
            let (tx, rx) = std::sync::mpsc::channel();
            admit(&mut app, &ctx, None, tx);
            receivers.push(rx);
        }
        for request in &mut app.shots.requests {
            request.ready_at = 0.0;
        }
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        for callback in &callbacks {
            callback.clone().complete(std::sync::Arc::new(egui::ColorImage::filled([8, 8], egui::Color32::GREEN)));
        }
        assert_eq!(app.shots.requests.len(), SCREENSHOT_MAX_REQUESTS);
        // Draining before admitting is what `logic` does: completed requests release first.
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        assert_eq!(app.shots.requests.len(), 0);
        assert!(receivers.iter_mut().all(|rx| rx.try_recv().is_ok()));
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        assert!(rx.try_recv().is_err(), "a new request is admitted once the drained slots are free");
        assert_eq!(app.shots.requests.len(), 1);
    }

    #[test]
    /// Uses `screen_rect` when `inner_rect` is absent, applies native scale times zoom, and advances the epoch on a hidden resize.
    fn raw_input_geometry_uses_screen_rect_when_inner_rect_is_absent() {
        // Wayland always omits `inner_rect`; the fresh `RawInput::screen_rect` is the only size.
        let ctx = sized_context(egui::vec2(800.0, 600.0));
        let mut app = test_app();
        let mut raw = raw_of(egui::vec2(1280.0, 720.0));
        if let Some(v) = raw.viewports.get_mut(&egui::ViewportId::ROOT) {
            v.inner_rect = None;
            v.native_pixels_per_point = Some(2.0);
            v.minimized = Some(false);
            v.occluded = Some(false);
        }
        app.raw_input_hook(&ctx, &mut raw);
        let geo = app.capture_geometry.expect("the raw frame records its geometry");
        assert_eq!(geo.viewport, Some(egui::vec2(1280.0, 720.0)));
        assert_eq!(geo.pixels_per_point, 2.0, "native scale times zoom");
        assert_eq!(geo.expected_size(), Some([2560, 1440]));

        let epoch = app.capture_epoch;
        // A hidden resize lands on a logic-only tick: no UI pass ran, but the raw frame is fresh.
        let mut hidden = raw_of(egui::vec2(640.0, 480.0));
        if let Some(v) = hidden.viewports.get_mut(&egui::ViewportId::ROOT) {
            v.inner_rect = None;
            v.native_pixels_per_point = Some(2.0);
            v.minimized = Some(false);
            v.occluded = Some(true);
        }
        app.raw_input_hook(&ctx, &mut hidden);
        let _ = ctx.run_logic(&hidden, |ctx| app.logic(ctx));
        assert!(app.capture_epoch > epoch, "a hidden resize is a new epoch");
        assert_eq!(app.capture_geometry.expect("geometry").viewport, Some(egui::vec2(640.0, 480.0)));
    }

    #[test]
    /// Minimized geometry has no size and reserves unknown bytes; restoring at the same size still changes the epoch.
    fn minimized_geometry_has_no_size_and_restore_bumps_the_epoch() {
        let ctx = egui::Context::default();
        let mut app = test_app();
        let mut minimized = raw_of(egui::vec2(800.0, 600.0));
        if let Some(v) = minimized.viewports.get_mut(&egui::ViewportId::ROOT) {
            v.inner_rect = None;
            v.minimized = Some(true);
        }
        app.raw_input_hook(&ctx, &mut minimized);
        let geo = app.capture_geometry.expect("recorded");
        assert_eq!(geo.viewport, None, "a minimized window has no capturable size");
        assert_eq!(geo.visible, Some(false));

        // Admission while minimized reserves the unknown-size maximum and waits.
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        assert_eq!(app.shots.requests[0].bytes, UNKNOWN_CAPTURE_BYTES);
        assert!(rx.try_recv().is_err());
        let epoch = app.capture_epoch;

        // Restoring at the same logical size is still a new surface generation.
        let mut restored = raw_of(egui::vec2(800.0, 600.0));
        if let Some(v) = restored.viewports.get_mut(&egui::ViewportId::ROOT) {
            v.inner_rect = Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0)));
            v.minimized = Some(false);
            v.occluded = Some(false);
        }
        app.raw_input_hook(&ctx, &mut restored);
        assert!(app.capture_epoch > epoch, "minimize/restore at the same size changes the epoch");
        assert_eq!(app.capture_geometry.expect("geometry").viewport, Some(egui::vec2(800.0, 600.0)));
        app.shots.cancel_all();
    }

    #[test]
    /// Switching documents before issuance cancels the request once with a document-changed error and writes nothing.
    fn a_document_switch_before_issuance_cancels_without_writing() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let path = temp_png("doc-queued");
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx);
        app.shots.requests[0].ready_at = 0.0;
        let admitted = app.shots.requests[0].document;
        assert!(app.run("file.new", json!({})).is_ok());
        assert_ne!(app.session.active().map(|d| d.uid), admitted, "the test switched documents");

        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert!(callbacks.is_empty(), "a capture for another document is never issued");
        let reply = rx.try_recv().expect("the queued request is cancelled once");
        assert!(reply["error"].as_str().is_some_and(|e| e.contains("document changed")), "{reply}");
        assert!(!path.exists(), "no pixels for the old document are written to its path");
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// Switching documents after issuance rejects the completed image once and writes nothing.
    fn a_document_switch_after_issuance_rejects_the_completed_image() {
        let ctx = sized_context(egui::vec2(64.0, 48.0));
        let mut app = test_app();
        let path = temp_png("doc-issued");
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx);
        app.shots.requests[0].ready_at = 0.0;
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 1);
        assert!(app.run("file.new", json!({})).is_ok());
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([64, 48], egui::Color32::RED)));
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the switched document is reported once");
        assert!(reply["error"].as_str().is_some_and(|e| e.contains("document changed")), "{reply}");
        assert!(!path.exists(), "a completion after a document switch never writes");
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// A request bound to another viewport is rejected once with a viewport error and writes nothing.
    fn a_completion_for_another_viewport_is_rejected() {
        let ctx = sized_context(egui::vec2(32.0, 24.0));
        let mut app = test_app();
        let path = temp_png("viewport");
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx);
        app.shots.requests[0].ready_at = 0.0;
        let callbacks = issue_with(&ctx, &mut app, egui::RawInput::default());
        assert_eq!(callbacks.len(), 1);
        // A slot/request bound to a different viewport must not fill this one.
        app.shots.requests[0].viewport_id = egui::ViewportId::from_hash_of("other-window");
        callbacks[0].clone().complete(std::sync::Arc::new(egui::ColorImage::filled([32, 24], egui::Color32::RED)));
        let _ = ui_pass(&ctx, egui::RawInput::default(), |ui| drain_screenshots(&mut app, ui.ctx()));
        let reply = rx.try_recv().expect("the viewport mismatch is reported once");
        assert!(reply["error"].as_str().is_some_and(|e| e.contains("another viewport")), "{reply}");
        assert!(!path.exists());
        assert_eq!(app.shots.reserved(), 0);
    }

    #[test]
    /// Accepts one-pixel rounding on a single axis, but rejects opposite-axis deviation, aspect distortion and more than one pixel of error.
    fn validate_image_rejects_distortion_but_allows_one_pixel_rounding() {
        let image = |w: usize, h: usize| egui::ColorImage::filled([w, h], egui::Color32::RED);
        assert!(validate_image(&image(100, 100), [100, 100]).is_ok());
        assert!(validate_image(&image(101, 100), [100, 100]).is_ok(), "one pixel of rounding on one axis is legitimate");
        assert!(validate_image(&image(100, 101), [100, 100]).is_ok());
        assert!(validate_image(&image(101, 99), [100, 100]).is_err(), "independent per-axis tolerance must not accept a distorted image");
        assert!(validate_image(&image(99, 101), [100, 100]).is_err());
        assert!(validate_image(&image(102, 100), [100, 100]).is_err(), "more than one pixel per axis is a different surface");
    }

    #[test]
    /// Checks the encoding working-set threshold, the base64 reply cap, and the retained-plus-encoding peak budget.
    fn encoding_working_set_is_bounded_for_admitted_captures() {
        assert!(encoding_working_bytes([8191, 2048], false).expect("checked") <= SCREENSHOT_MAX_ENCODING_BYTES);
        assert!(encoding_working_bytes([8192, 8192], false).expect("checked") > SCREENSHOT_MAX_ENCODING_BYTES);
        assert!(encoding_working_bytes([4000, 3000], true).expect("checked") <= SCREENSHOT_MAX_ENCODING_BYTES);
        assert!(encoding_working_bytes([5000, 4000], true).expect("checked") > SCREENSHOT_MAX_ENCODING_BYTES);
        // The base64 reply itself is capped too, before the string exists.
        let png = png_worst_bytes(5000 * 4000 * 4, 4000).expect("checked");
        assert!(base64_len(png).expect("checked") > SCREENSHOT_MAX_BASE64_BYTES);
        // Retained requests plus one encoding working set are held to a hard peak.
        assert!(check_peak_budget(0, [64, 48], false).is_ok());
        assert!(check_peak_budget(SCREENSHOT_MAX_AGGREGATE_BYTES, [9000, 4096], false).is_err());
    }

    #[test]
    /// An oversized base64 capture is refused with nothing reserved, while the same capture as a path is admitted.
    fn base64_capture_over_the_encoding_budget_is_rejected_before_admission() {
        let ctx = sized_context(egui::vec2(5000.0, 4000.0));
        let mut app = test_app();
        let (tx, rx) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, None, tx);
        let reply = rx.try_recv().expect("an oversized base64 capture is refused at admission");
        assert_eq!(reply["ok"], false, "{reply}");
        assert!(
            reply["error"].as_str().is_some_and(|e| e.contains("encoding") || e.contains("base64")),
            "the error names the encoding budget: {reply}"
        );
        assert!(app.shots.requests.is_empty(), "a refused request reserves nothing");

        // The same capture as a file is admitted within the per-image and encoding budgets.
        let path = temp_png("encoding-path");
        let (tx2, rx2) = std::sync::mpsc::channel();
        admit(&mut app, &ctx, Some(path.to_string_lossy().into_owned()), tx2);
        assert!(rx2.try_recv().is_err(), "the path form is admitted");
        assert_eq!(app.shots.requests.len(), 1);
        app.shots.cancel_all();
    }

    #[test]
    /// Oversized text, repeated maximum texts, keys and moves are refused without changing the queue, and the byte/event limits are both enforced.
    fn synthetic_text_key_and_move_producers_honour_the_queue_budget() {
        let ctx = egui::Context::default();
        let mut app = test_app();
        // One oversized text event is refused before it is queued.
        let huge = "x".repeat(MAX_SYNTHETIC_TEXT_BYTES + 1);
        let Outcome::Done(reply) = handle(&mut app, &ctx, "ui.text", &json!({"text": huge})) else { panic!("ui.text returns immediately") };
        assert_eq!(reply["ok"], false, "{reply}");
        assert!(app.synthetic.is_empty());

        // The byte budget stops repeated maximum-size texts; the queue is left unchanged.
        let max = "x".repeat(MAX_SYNTHETIC_TEXT_BYTES);
        let mut refused_at = None;
        for i in 0..64 {
            let Outcome::Done(reply) = handle(&mut app, &ctx, "ui.text", &json!({"text": max.clone()})) else {
                panic!("ui.text returns immediately")
            };
            if reply["ok"] == false {
                refused_at = Some((i, app.synthetic.len()));
                break;
            }
        }
        let (accepted, queued) = refused_at.expect("the synthetic byte budget is enforced");
        assert!(accepted > 0 && accepted < 64, "{accepted} texts fit");
        let Outcome::Done(reply) = handle(&mut app, &ctx, "ui.text", &json!({"text": max})) else { panic!("ui.text returns immediately") };
        assert_eq!(reply["ok"], false);
        assert_eq!(app.synthetic.len(), queued, "a refused text leaves the queue alone");

        // Event count: keys and moves use the same overall budget as pointer gestures.
        app.synthetic.clear();
        let pos = egui::pos2(1.0, 1.0);
        app.synthetic.push_back(vec![egui::Event::PointerMoved(pos); MAX_QUEUED_SYNTHETIC_EVENTS - 1]);
        let Outcome::Done(reply) = handle(&mut app, &ctx, "ui.key", &json!({"key":"enter"})) else { panic!("ui.key returns immediately") };
        assert_eq!(reply["ok"], false, "two key events do not fit in the last slot: {reply}");
        assert_eq!(app.synthetic.iter().map(Vec::len).sum::<usize>(), MAX_QUEUED_SYNTHETIC_EVENTS - 1);

        app.synthetic.clear();
        app.synthetic.push_back(vec![egui::Event::PointerMoved(pos); MAX_QUEUED_SYNTHETIC_EVENTS]);
        let Outcome::Done(reply) = handle(&mut app, &ctx, "ui.move", &json!({"x":2.0,"y":2.0})) else { panic!("ui.move returns immediately") };
        assert_eq!(reply["ok"], false, "{reply}");
        assert_eq!(app.synthetic.iter().map(Vec::len).sum::<usize>(), MAX_QUEUED_SYNTHETIC_EVENTS);
    }
}
