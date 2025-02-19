use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Xelph",
        native_options, 
        Box::new(|_cc| Ok(Box::new(App::new())))
    );
}

#[derive(PartialEq)]
enum Tab {
    Home,
    Settings
}

struct App {
    current_tab: Tab,
    dark_mode: bool
}

impl App {
    fn new() -> Self {
        Self { 
            current_tab: Tab::Home,
            dark_mode: true
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        });

        egui::SidePanel::left("tabs").show(ctx, |ui| {
            if ui.selectable_label(self.current_tab == Tab::Home, "Home").clicked() {
                self.current_tab = Tab::Home;
            }
            if ui.selectable_label(self.current_tab == Tab::Settings, "Settings").clicked() {
                self.current_tab = Tab::Settings;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Home => {
                    ui.heading("Home tab");
                    if ui.button("Toggle theme").clicked() {
                        self.dark_mode = !self.dark_mode;
                        ctx.set_visuals(if self.dark_mode {
                            egui::Visuals::dark()
                        } else {
                            egui::Visuals::light()
                        });
                    }
                },
                Tab::Settings => {
                    ui.heading("Settings");
                }
            }
        });
    }
}
