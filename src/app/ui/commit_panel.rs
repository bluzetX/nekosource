use eframe::egui;
use super::super::state::NekoApp;

pub fn show_commit_panel(ui: &mut egui::Ui, app: &mut NekoApp) -> bool {
    if let Some(idx) = app.selected_idx {
        let path   = app.repos[idx].0.clone();
        let branch = app.repo_branches.get(idx).cloned().unwrap_or_else(|| "?".into());

        ui.horizontal(|ui| {
            ui.colored_label(egui::Color32::LIGHT_BLUE, &path);
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(format!("🔀  {}", branch))
                    .color(egui::Color32::from_rgb(100, 210, 130))
                    .monospace(),
            );
        });
        ui.add_space(8.0);

        ui.label(app.t("label-commit-msg"));
        ui.add_space(4.0);
        ui.add_sized(
            [ui.available_width(), 56.0],
            egui::TextEdit::multiline(&mut app.commit_msg_input),
        );
        ui.add_space(4.0);
        let clicked = ui.add_sized(
            [ui.available_width(), 26.0],
            egui::Button::new(app.t("btn-commit")),
        ).clicked();

        ui.add_space(6.0);
        ui.separator();
        ui.add_space(4.0);

        ui.label(
            egui::RichText::new(app.t("label-recent-commits"))
                .small()
                .color(egui::Color32::GRAY),
        );

        ui.add_space(4.0);

        let remaining_height = ui.available_height().max(0.0);

        ui.allocate_ui_with_layout(
            egui::vec2(ui.available_width(), remaining_height),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.set_min_height(remaining_height);

                egui::ScrollArea::vertical()
                    .id_salt("commit_log_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.set_min_height(remaining_height);

                        if app.selected_commits.is_empty() {
                            ui.weak(app.t("placeholder-no-commits"));
                            return;
                        }

                        for c in &app.selected_commits {
                            let row = ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(&c.short_hash)
                                        .monospace()
                                        .color(egui::Color32::from_rgb(130, 170, 230)),
                                );
                                ui.add_space(2.0);
                                ui.label(
                                    egui::RichText::new(&c.title)
                                        .color(egui::Color32::WHITE),
                                );
                            });

                            row.response.on_hover_text(&c.datetime);

                            ui.horizontal(|ui| {
                                ui.add_space(6.0);
                                ui.label(
                                    egui::RichText::new(format!("{} · {}", c.author, c.time_ago))
                                        .small()
                                        .color(egui::Color32::GRAY),
                                );
                            });

                            ui.add_space(3.0);
                        }
                    });
            },
        );
        clicked
    } else {
        ui.add_space(ui.available_height() / 2.0 - 20.0);
        ui.vertical_centered(|ui| {
            ui.add(egui::Label::new(app.t("placeholder-select")).wrap());
        });
        false
    }
}
