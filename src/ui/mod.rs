use eframe::egui;
use tabs::Tabs;

pub mod menu;
pub mod tabs;

pub struct App {
    current_tab: Tabs,
    dark_mode: bool
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tab: Tabs::Home,
            dark_mode: false
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            crate::ui::menu::menu(ui);
        });

        egui::SidePanel::right("tabs").show(ctx, |ui| {
            if ui.selectable_label(self.current_tab == Tabs::Home, "Home").clicked() {
                self.current_tab = Tabs::Home;
            }
            if ui.selectable_label(self.current_tab == Tabs::Settings, "Settings").clicked() {
                self.current_tab = Tabs::Settings;
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tabs::Home => {
                    ui.heading("Home tab");
                },
                Tabs::Settings => {
                    ui.heading("Settings tabs");
                }
            }
        });
    }
}