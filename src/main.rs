mod ui;
mod app;

use eframe;
pub fn main() {
    let native_options = crate::eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Xelph",
        native_options,
        Box::new(|_cc| Ok(Box::new(crate::app::App::new())))
    );
}