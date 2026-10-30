use super::Dialog;

/// Help ▸ About GridCraft: the app, its contributors and the AI models that helped. The credits
/// are compiled in (`crate::credits`, docs/contributors.md).
pub(super) fn about(ui: &mut egui::Ui, d: &mut Dialog) {
    ui.horizontal(|ui| {
        for tab in ["About", "Contributors", "Models"] {
            if ui.selectable_label(d.tab == tab, tab).clicked() {
                d.tab = tab.to_string();
            }
        }
    });
    ui.separator();
    match d.tab.as_str() {
        "Contributors" => crate::credits::contributors_ui(ui),
        "Models" => crate::credits::models_ui(ui),
        _ => {
            // The fork's name, and DevCraft's version beside GridCraft's (aniefork).
            crate::aniefork::about::heading(ui);
            ui.add_space(4.0);
            // What the fork says of itself, in place of upstream's three lines (aniefork).
            crate::aniefork::about::credits(ui);
        }
    }
}
