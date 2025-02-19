mod ui;
use eframe;
use ui::App;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Xelph",
        native_options, 
        Box::new(|_cc| Ok(Box::new(App::new())))
    );
}
