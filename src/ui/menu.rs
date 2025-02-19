use eframe::egui;

pub fn menu(ui: &mut egui::Ui) {
    egui::menu::bar(ui, |ui| {
        ui.menu_button("App", |ui| {
            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
        });
    });
}