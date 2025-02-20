use eframe::egui;
use crate::app::Tabs;
use crate::ui::home::Home;

pub fn home(args: Home) {
    let Home {
        ui,
        current_tab,
        dark_mode,
        ctx
    } = args;

    match current_tab {
        Tabs::Home => {
            ui.heading("Home");
        },
        Tabs::Settings => {
            if ui.button("Toggle theme").clicked() {
                *dark_mode = !*dark_mode;
                ctx.set_visuals(if *dark_mode {
                    egui::Visuals::light()
                } else {
                    egui::Visuals::dark()
                });
            }
        }
    }
}