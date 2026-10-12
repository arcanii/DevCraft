//! The fork's own preferences.
//!
//! They are saved with upstream's, in `ui.json`, through one field of `UiState` that is flattened
//! into it: the keys sit beside upstream's (`developerTab`, `selectionOutline`…), and upstream's
//! struct gains one line however many the fork adds here.
//!
//! A saved value that makes no sense reads as the default. It must not fail the whole of
//! `UiState`, which would throw away every preference the user has.

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value as Json;

/// How heavily the selection's outline is drawn. It is always a whole number of device pixels
/// wide (`aniefork::selection::Bars`), so a weight is the nearest such to its points.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Outline {
    /// Upstream's: two points. Three pixels on a display at 150%.
    #[default]
    Standard,
    /// One and a half points: two pixels on a display at 100% to 150%, which is what Excel was
    /// measured to draw at 150%.
    Thin,
    /// One point: the gridline itself, in the selection's colour.
    Hairline,
}

impl Outline {
    pub const ALL: [Outline; 3] = [Outline::Standard, Outline::Thin, Outline::Hairline];

    /// The word a command and `ui.json` use.
    pub fn code(self) -> &'static str {
        match self {
            Outline::Standard => "standard",
            Outline::Thin => "thin",
            Outline::Hairline => "hairline",
        }
    }

    /// What the Options page calls it.
    pub fn label(self) -> &'static str {
        match self {
            Outline::Standard => "Standard",
            Outline::Thin => "Thin",
            Outline::Hairline => "Hairline",
        }
    }

    pub fn parse(code: &str) -> Option<Outline> {
        Outline::ALL.into_iter().find(|o| o.code() == code)
    }

    /// The outline's width, in points.
    pub fn width(self) -> f32 {
        match self {
            Outline::Standard => 2.0,
            Outline::Thin => 1.5,
            Outline::Hairline => 1.0,
        }
    }
}

fn lenient_flag<'de, D: Deserializer<'de>>(d: D) -> Result<bool, D::Error> {
    Ok(Json::deserialize(d)?.as_bool().unwrap_or(false))
}

fn lenient_outline<'de, D: Deserializer<'de>>(d: D) -> Result<Outline, D::Error> {
    Ok(Json::deserialize(d)?.as_str().and_then(Outline::parse).unwrap_or_default())
}

/// The fork's preferences, as `UiState::aniefork`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Prefs {
    /// The ribbon shows the Developer tab (File ▸ Options).
    #[serde(deserialize_with = "lenient_flag")]
    pub developer_tab: bool,
    /// How heavily the selection's outline is drawn (File ▸ Options ▸ Selection).
    #[serde(deserialize_with = "lenient_outline")]
    pub selection_outline: Outline,
    /// The selection isn't drawn while another window is the active one, as in Excel.
    #[serde(deserialize_with = "lenient_flag")]
    pub hide_selection_when_inactive: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UiState;
    use serde_json::json;

    #[test]
    fn the_forks_preferences_are_saved_beside_upstreams() {
        let mut ui = UiState::default();
        assert_eq!(ui.aniefork, Prefs::default());
        assert_eq!(
            (ui.aniefork.developer_tab, ui.aniefork.selection_outline, ui.aniefork.hide_selection_when_inactive),
            (false, Outline::Standard, false)
        );
        ui.aniefork = Prefs { developer_tab: true, selection_outline: Outline::Thin, hide_selection_when_inactive: true };
        ui.dark = true;
        let saved = serde_json::to_value(&ui).unwrap();
        assert_eq!(
            (&saved["developerTab"], &saved["selectionOutline"], &saved["hideSelectionWhenInactive"], &saved["dark"]),
            (&json!(true), &json!("thin"), &json!(true), &json!(true)),
            "{saved}"
        );
        assert!(saved.get("aniefork").is_none(), "the keys sit beside upstream's, not under one of their own");
        let back: UiState = serde_json::from_value(saved).unwrap();
        assert_eq!(back.aniefork, ui.aniefork);
        assert!(back.dark && back.animations);
    }

    #[test]
    fn a_file_from_before_a_setting_existed_or_from_upstream_reads_with_the_defaults() {
        // Written by an earlier fork build: the Developer tab's key was already at the top.
        let old: UiState = serde_json::from_str(r#"{"dark": true, "ribbonTab": "View", "developerTab": true}"#).unwrap();
        assert!(old.dark && old.aniefork.developer_tab);
        assert_eq!((old.aniefork.selection_outline, old.aniefork.hide_selection_when_inactive), (Outline::Standard, false));
        // Written by upstream: none of the fork's keys.
        let upstream: UiState = serde_json::from_str(r#"{"dark": false, "animations": false, "interfaceScale": 125}"#).unwrap();
        assert_eq!(upstream.aniefork, Prefs::default());
        assert!(!upstream.animations && upstream.interface_scale == 125);
        assert_eq!(serde_json::from_str::<UiState>("{}").unwrap().aniefork, Prefs::default());
    }

    #[test]
    fn a_saved_value_that_makes_no_sense_costs_only_itself() {
        let junk = json!({
            "dark": true,
            "ribbonTab": "Data",
            "developerTab": "yes",
            "selectionOutline": "enormous",
            "hideSelectionWhenInactive": 7,
        });
        let ui: UiState = serde_json::from_value(junk).unwrap();
        assert_eq!(ui.aniefork, Prefs::default(), "each reads as its default");
        assert!(ui.dark && ui.ribbon_tab == "Data", "and upstream's preferences are kept");
        for outline in [json!(null), json!(3), json!(["thin"]), json!({"weight": "thin"}), json!("THIN")] {
            let ui: UiState = serde_json::from_value(json!({"selectionOutline": outline, "developerTab": true})).unwrap();
            assert_eq!((ui.aniefork.selection_outline, ui.aniefork.developer_tab), (Outline::Standard, true), "{outline}");
        }
    }

    #[test]
    fn the_outlines_get_thinner_and_their_codes_round_trip() {
        let widths: Vec<f32> = Outline::ALL.iter().map(|o| o.width()).collect();
        assert_eq!(widths, [2.0, 1.5, 1.0]);
        for outline in Outline::ALL {
            assert_eq!(Outline::parse(outline.code()), Some(outline));
            assert_eq!(serde_json::to_value(outline).unwrap(), json!(outline.code()));
            assert!(!outline.label().is_empty());
        }
        assert_eq!(Outline::parse("Thin"), None, "codes are exact");
        assert_eq!(Outline::parse(""), None);
    }
}
