mod app;

use app::NekoApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 650.0])
            .with_min_inner_size([1000.0, 650.0])
            .with_max_inner_size([1000.0, 650.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "NekoSource",
        options,
        Box::new(|_| Ok(Box::new(NekoApp::new()))),
    )
}