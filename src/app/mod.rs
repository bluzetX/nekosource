mod language;
mod state;
mod ui;
pub mod git;
pub mod gpg;
pub mod error;

pub use state::NekoApp;

impl eframe::App for NekoApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        ui::show_ui(self, ui);
    }
}
