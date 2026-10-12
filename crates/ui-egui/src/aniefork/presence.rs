//! Who is at the controls: the person at the screen, or the control channel.
//!
//! The control channel clicks and types by putting input into the window as if it had come from
//! the mouse and keyboard, so a button can't tell an agent's click from a person's. A few things
//! in the Development subsystem are for the person alone: trusting a project, running its code,
//! opening its panel, naming the Python to run. Before doing one of those, the code asks here.
//!
//! [`watch`] runs once a frame, right after the control channel is polled. Input the channel
//! queued then goes into the next frame, so that frame, and a short while after it, counts as
//! driven: a press and its release can be frames apart.

use crate::SheetApp;

/// How long after the control channel's last input the window still counts as driven.
const HOLD_MS: f64 = 1_000.0;

/// Said when something only the person at the screen may do was asked for while driven.
pub const BY_HAND: &str = "That is for the person at the screen: it can't be done through the control channel.";

#[derive(Clone, Copy, Default)]
struct Watch {
    /// The frame this was last brought up to date in.
    frame: u64,
    /// How much input the control channel had waiting then.
    queued: usize,
    /// Until when (in `now_ms`) the window counts as driven.
    until: f64,
}

fn id() -> egui::Id {
    egui::Id::new("aniefork.presence")
}

/// Notes whether the control channel is putting input into the window. Called once a frame,
/// after the control channel has been polled.
pub fn watch(app: &SheetApp, ctx: &egui::Context) {
    let mut watch: Watch = ctx.data(|d| d.get_temp(id())).unwrap_or_default();
    let now = crate::now_ms();
    let frame = ctx.cumulative_frame_nr();
    if watch.frame != frame {
        // What was waiting when the last frame was drawn has gone into this one's input.
        if watch.queued > 0 {
            watch.until = watch.until.max(now + HOLD_MS);
        }
        watch.frame = frame;
    }
    watch.queued = app.synthetic.len();
    if watch.queued > 0 {
        watch.until = watch.until.max(now + HOLD_MS);
    }
    ctx.data_mut(|d| d.insert_temp(id(), watch));
}

/// Whether what is clicked and typed now may be the control channel's doing.
pub fn driven(ctx: &egui::Context) -> bool {
    ctx.data(|d| d.get_temp::<Watch>(id())).is_some_and(|watch| crate::now_ms() < watch.until)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> SheetApp {
        let mut session = gridcraft_engine::Session::new();
        session.new_workbook();
        SheetApp::new(session, Default::default())
    }

    #[test]
    fn a_window_nobody_drives_is_the_persons() {
        let (a, ctx) = (app(), egui::Context::default());
        assert!(!driven(&ctx), "before anything was watched");
        for _ in 0..3 {
            watch(&a, &ctx);
            assert!(!driven(&ctx));
        }
    }

    #[test]
    fn input_from_the_control_channel_marks_the_window_as_driven_for_a_while() {
        let (mut a, ctx) = (app(), egui::Context::default());
        watch(&a, &ctx);
        // The channel queues a click: it will be in the next frame's input.
        let reply = crate::control::handle(&mut a, &ctx, "ui.click", &serde_json::json!({"x": 10, "y": 10}));
        assert!(matches!(reply, crate::control::Outcome::Done(_)));
        assert!(!a.synthetic.is_empty());
        watch(&a, &ctx);
        assert!(driven(&ctx));
        // The input has been used up; the window still counts as driven for a moment.
        a.synthetic.clear();
        watch(&a, &ctx);
        assert!(driven(&ctx), "a press and its release can be frames apart");
        std::thread::sleep(std::time::Duration::from_millis(HOLD_MS as u64 + 150));
        watch(&a, &ctx);
        assert!(!driven(&ctx), "and then it is the person's again");
    }

    #[test]
    fn typing_and_keys_from_the_channel_count_too() {
        for (method, params) in [("ui.key", serde_json::json!({"key": "Enter"})), ("ui.text", serde_json::json!({"text": "x"}))] {
            let (mut a, ctx) = (app(), egui::Context::default());
            let _ = crate::control::handle(&mut a, &ctx, method, &params);
            watch(&a, &ctx);
            assert!(driven(&ctx), "{method}");
        }
    }
}
