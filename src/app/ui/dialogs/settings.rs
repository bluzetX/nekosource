use eframe::egui;
use super::super::super::state::{NekoApp, Status};

pub fn show_settings_dialog(ctx: &egui::Context, app: &mut NekoApp) {
    if !app.show_settings { return; }

    let mut do_save       = false;
    let mut do_close      = false;
    let mut do_reload_gpg = false;

    egui::Window::new(app.t("settings-title"))
        .min_width(360.0)
        .show(ctx, |ui| {
            egui::Grid::new("settings_grid")
                .min_col_width(110.0)
                .show(ui, |ui| {
                    ui.label(app.t("label-username"));
                    ui.add_sized([220.0, 20.0], egui::TextEdit::singleline(&mut app.username));
                    ui.end_row();

                    ui.label(app.t("label-email"));
                    ui.add_sized([220.0, 20.0], egui::TextEdit::singleline(&mut app.email));
                    ui.end_row();
                });

            ui.separator();

            let label_gpg = app.t("label-gpg");
            ui.checkbox(&mut app.enable_gpg, label_gpg);

            if app.enable_gpg {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(app.t("label-gpg-id"));

                    let selected_label: String = app.gpg_keys
                        .iter()
                        .find(|(id, _)| id == &app.gpg_key_id)
                        .map(|(_, label)| label.clone())
                        .unwrap_or_else(|| {
                            if app.gpg_key_id.is_empty() {
                                app.t("gpg-no-keys")
                            } else {
                                app.gpg_key_id.clone()
                            }
                        });

                    egui::ComboBox::from_id_salt("gpg_key_selector")
                        .width(260.0)
                        .selected_text(&selected_label)
                        .show_ui(ui, |ui| {
                            if app.gpg_keys.is_empty() {
                                ui.weak(app.t("gpg-no-keys"));
                            } else {
                                let keys = app.gpg_keys.clone();
                                for (key_id, label) in &keys {
                                    ui.selectable_value(&mut app.gpg_key_id, key_id.clone(), label);
                                }
                            }
                        });

                    if ui.button("🔄").on_hover_text(app.t("gpg-refresh-keys")).clicked() {
                        do_reload_gpg = true;
                    }
                });
            }

            ui.add_space(12.0);
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button(app.t("btn-save")).clicked()  { do_save = true; do_close = true; }
                if ui.button(app.t("btn-close")).clicked() { do_close = true; }
            });
        });

    if do_reload_gpg {
        app.gpg_keys = crate::app::gpg::list_keys();
        let still_valid = app.gpg_keys.iter().any(|(id, _)| id == &app.gpg_key_id);
        if !still_valid {
            app.gpg_key_id = app.gpg_keys.first().map(|(id, _)| id.clone()).unwrap_or_default();
        }
    }

    if do_save {
        if let Err(e) = crate::app::git::write_user_config(&app.username, &app.email) {
            app.status = Status::Error(e.localize(&app.locale));
        }
    }

    if do_close {
        app.show_settings = false;
    }
}
