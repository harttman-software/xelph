use eframe::egui;

#[derive(PartialEq)]
pub enum Tabs {
    Home,
    Settings
}

pub struct App {
    dark_mode: bool,
    current_tab: Tabs
}

impl App {
    pub fn new() -> Self {
        Self {
            dark_mode: true,
            current_tab: Tabs::Home
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // ==============
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            crate::ui::menu::menu(ui);
        });
        // ==============
        egui::SidePanel::right("sidebar").show(ctx, |ui| {
            crate::ui::sidebar::sidebar(ui, &mut self.current_tab);
        });
        // =============
        egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::tabs::home(crate::ui::home::Home {
                ui,
                current_tab: &mut self.current_tab,
                dark_mode: &mut self.dark_mode,
                ctx
            });
        });
    }
}