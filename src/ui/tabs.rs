use eframe::{self, egui};

pub fn home(ui: &mut eframe::egui::Ui, current_tab: &mut crate::app::Tabs, dark_mode: &mut bool, ctx: &egui::Context) {
    match current_tab {
        crate::app::Tabs::Home => {
            ui.heading("Home");
        },
        crate::app::Tabs::Settings => {
            ui.heading("Settings");
            if ui.button("toggle theme").clicked() {
                *dark_mode = !*dark_mode;
                ctx.set_visuals(if *dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                });        
            }
        }
    }
}