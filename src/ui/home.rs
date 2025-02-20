use eframe::egui;

pub struct Home<'a> {
    pub ui: &'a mut egui::Ui, 
    pub current_tab: &'a mut crate::app::Tabs,
    pub dark_mode: &'a mut bool,
    pub ctx: &'a egui::Context
}