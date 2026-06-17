use eframe::egui;
use super::super::super::state::{NekoApp, Status};

pub fn show_status_dialog(ctx: &egui::Context, app: &mut NekoApp) -> bool {
    if app.show_passphrase_dialog { return false; }

    let (title, msg, is_loading) = match &app.status {
        Status::Idle        => return false,
        Status::Loading(m)  => ("⏳ ...",  m.clone(), true),
        Status::Success(m)  => ("✅",       m.clone(), false),
        Status::Error(m)    => ("❌",       m.clone(), false),
    };

    let mut dismiss = false;

    egui::Window::new(title)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(320.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            if is_loading {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label(&msg);
                });
            } else {
                ui.label(&msg);
                ui.add_space(8.0);
                if ui.button(app.t("btn-close")).clicked() {
                    dismiss = true;
                }
            }
            ui.add_space(4.0);
        });

    dismiss
}
