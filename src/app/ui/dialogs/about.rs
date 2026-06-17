use eframe::egui;
use super::super::super::state::NekoApp;

pub fn show_about_dialog(ctx: &egui::Context, app: &mut NekoApp) {
    if !app.show_about { return; }

    egui::Window::new(app.t("about-title"))
        .default_size([480.0, 260.0])
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("🐱 NekoSource");
                ui.label(app.t("about-description"));
                ui.vertical_centered(|ui| {
                    ui.weak(app.t("about-copyright"));
                });
            });

            ui.separator();
            ui.label(app.t("about-development"));

            contributor(ui, app, "OctoBanon",      "https://github.com/OctoBanon-Main", "contributor-octobanon");
            contributor(ui, app, "ShadowCj",       "https://github.com/bluzetX",        "contributor-shadowcj");
            contributor(ui, app, "xelframe",       "https://github.com/xelframe",       "contributor-xelframe");

            ui.separator();
            ui.add_space(5.0);
            if ui.button(app.t("btn-close")).clicked() {
                app.show_about = false;
            }
        });
}

fn contributor(ui: &mut egui::Ui, app: &NekoApp, name: &str, url: &str, role_key: &str) {
    ui.horizontal(|ui| {
        ui.label("•");
        ui.hyperlink_to(name, url);
        ui.label(app.t(role_key));
    });
}
