use eframe::egui;
use super::super::super::state::NekoApp;

pub fn show_passphrase_dialog(ctx: &egui::Context, app: &mut NekoApp) {
    if !app.show_passphrase_dialog { return; }

    let mut submitted = false;
    let mut cancelled = false;

    egui::Window::new(app.t("passphrase-title"))
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .min_width(360.0)
        .show(ctx, |ui| {
            ui.add_space(4.0);
            ui.label(app.t("passphrase-prompt"));
            ui.add_space(4.0);

            if !app.passphrase_key_path.is_empty() {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("🔑")
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(&app.passphrase_key_path)
                                .monospace()
                                .color(egui::Color32::from_rgb(130, 170, 230)),
                        ).truncate()
                    );
                });
                ui.add_space(2.0);
            }

            ui.label(
                egui::RichText::new(app.t("passphrase-hint"))
                    .small()
                    .color(egui::Color32::GRAY),
            );
            ui.add_space(8.0);

            let response = ui.add(
                egui::TextEdit::singleline(&mut app.passphrase_input)
                    .password(true)
                    .desired_width(f32::INFINITY)
                    .hint_text("••••••••"),
            );
            response.request_focus();

            if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                submitted = true;
            }

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button(app.t("btn-ok")).clicked()     { submitted = true; }
                if ui.button(app.t("btn-cancel")).clicked() { cancelled = true; }
            });
            ui.add_space(4.0);
        });

    if submitted {
        if let Some(sender) = app.passphrase_sender.take() {
            sender.send(Some(app.passphrase_input.clone())).ok();
        }
        app.passphrase_input.clear();
        app.passphrase_key_path.clear();
        app.show_passphrase_dialog = false;
    }
    if cancelled {
        if let Some(sender) = app.passphrase_sender.take() {
            sender.send(None).ok();
        }
        app.passphrase_input.clear();
        app.passphrase_key_path.clear();
        app.show_passphrase_dialog = false;
    }
}