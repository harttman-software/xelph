use eframe::egui;

use crate::app::Tabs;

pub fn sidebar(ui: &mut egui::Ui, current_tab: &mut Tabs) {
    if ui.selectable_label(*current_tab == Tabs::Home, "Home").clicked() {
        *current_tab = Tabs::Home;
    }

    if ui.selectable_label(current_tab == &mut Tabs::Settings, "Settings").clicked() {
        *current_tab = Tabs::Settings;
    }
}