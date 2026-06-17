mod commit_panel;
mod dialogs;
mod repo_list;
mod toolbar;
mod workers;

use eframe::egui;
use super::state::{NekoApp, Status};

pub fn show_ui(app: &mut NekoApp, ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();

    workers::poll_worker(app);

    if app.selected_idx != app.log_loaded_for {
        app.refresh_selected_log();
    }

    dialogs::show_passphrase_dialog(&ctx, app);

    if dialogs::show_status_dialog(&ctx, app) {
        app.status = Status::Idle;
    }

    dialogs::show_settings_dialog(&ctx, app);
    dialogs::show_about_dialog(&ctx, app);

    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.heading(app.t("title-mock"));
        ui.add_space(8.0);

        let actions = toolbar::show_toolbar(ui, app);
        if let Some(lang) = actions.lang_changed {
            app.current_lang = lang;
            app.locale = super::language::Locale::new(lang);
        }
        if actions.refresh {
            app.selected_idx = None;
            app.refresh_all_branches();
            app.log_loaded_for = None;
            app.selected_commits.clear();
        }
        if actions.open_settings { app.show_settings = true; }
        if actions.open_about    { app.show_about    = true; }
        if actions.trigger_clone && !app.is_busy() {
            workers::spawn_clone(app, ctx.clone());
        }

        ui.add_space(10.0);

        let available_width = ui.available_width();

        if available_width >= 650.0 {
            let left_width = (available_width * 0.4).clamp(250.0, 450.0);
            let panel_height = 580.0;
            ui.horizontal(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(left_width, panel_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| repo_list::show_repo_list(ui, app),
                );
                ui.separator();
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), panel_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        if commit_panel::show_commit_panel(ui, app) && !app.is_busy() {
                            workers::spawn_commit(app, ctx.clone());
                        }
                    },
                );
            });
        } else {
            let top_height = (ui.available_height() * 0.45).clamp(150.0, 300.0);
            ui.vertical(|ui| {
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), top_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| repo_list::show_repo_list(ui, app),
                );
                ui.separator();
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), ui.available_height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        if commit_panel::show_commit_panel(ui, app) && !app.is_busy() {
                            workers::spawn_commit(app, ctx.clone());
                        }
                    },
                );
            });
        }
    });
}
