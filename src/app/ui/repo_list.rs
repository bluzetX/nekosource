use eframe::egui;
use super::super::state::NekoApp;

pub fn show_repo_list(ui: &mut egui::Ui, app: &mut NekoApp) {
    ui.label(
        egui::RichText::new(app.t("label-local-repos"))
            .heading(),
    );
    ui.add_space(8.0);

    egui::ScrollArea::vertical()
        .id_salt("repo_list_scroll")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            egui::Grid::new("repos_table")
                .striped(true)
                .num_columns(3)
                .min_col_width(60.0)
                .show(ui, |ui| {
                    for (idx, (path, url)) in app.repos.iter().enumerate() {
                        let selected = Some(idx) == app.selected_idx;

                        if ui.selectable_label(selected, path).clicked() {
                            app.selected_idx = Some(idx);
                        }

                        let branch = app.repo_branches
                            .get(idx)
                            .map(|s| s.as_str())
                            .unwrap_or("?");

                        ui.label(
                            egui::RichText::new(format!("🔀  {}", branch))
                                .monospace()
                                .color(egui::Color32::from_rgb(100, 210, 130)),
                        );

                        ui.add(egui::Label::new(
                            egui::RichText::new(url).color(egui::Color32::GRAY),
                        ).truncate());

                        ui.end_row();
                    }
                });
        });

    ui.add_space(8.0);

    if let Some(idx) = app.selected_idx {
        if ui.button(app.t("btn-delete")).clicked() {
            app.repos.remove(idx);
            if idx < app.repo_branches.len() {
                app.repo_branches.remove(idx);
            }
            app.selected_idx     = None;
            app.log_loaded_for   = None;
            app.selected_commits.clear();
            app.save_repos();
        }
    }
}