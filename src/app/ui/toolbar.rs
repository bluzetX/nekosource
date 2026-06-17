use eframe::egui;
use super::super::{language::Language, state::NekoApp};

pub struct ToolbarActions {
    pub refresh:       bool,
    pub trigger_clone: bool,
    pub open_settings: bool,
    pub open_about:    bool,
    pub lang_changed:  Option<Language>,
}

pub fn show_toolbar(ui: &mut egui::Ui, app: &mut NekoApp) -> ToolbarActions {
    let mut actions = ToolbarActions {
        refresh:       false,
        trigger_clone: false,
        open_settings: false,
        open_about:    false,
        lang_changed:  None,
    };

    let available_width = ui.available_width();
    let compact = available_width < 640.0;

    if compact {
        ui.horizontal_wrapped(|ui| {
            if ui.button(app.t("btn-refresh")).clicked() { actions.refresh = true; }
            ui.separator();
            if ui.button(app.t("btn-settings")).clicked() { actions.open_settings = true; }
            if ui.button(app.t("btn-about")).clicked()    { actions.open_about    = true; }
            ui.separator();
            lang_switcher(ui, app, &mut actions);
        });

        ui.horizontal(|ui| {
            ui.label(app.t("label-url"));

            let url_w = (ui.available_width() - 184.0).max(60.0);
            ui.add_sized(
                [url_w, 20.0],
                egui::TextEdit::singleline(&mut app.clone_url_input),
            );

            let hint = app.t("hint-branch");
            ui.add_sized(
                [90.0, 20.0],
                egui::TextEdit::singleline(&mut app.clone_branch_input)
                    .hint_text(hint),
            );

            if ui.button(app.t("btn-clone")).clicked() { actions.trigger_clone = true; }
        });
    } else {
        ui.horizontal_wrapped(|ui| {
            if ui.button(app.t("btn-refresh")).clicked() { actions.refresh = true; }
            ui.separator();

            ui.label(app.t("label-url"));
            let url_width = (available_width - 660.0).clamp(80.0, 260.0);
            ui.add_sized(
                [url_width, 20.0],
                egui::TextEdit::singleline(&mut app.clone_url_input),
            );

            let hint = app.t("hint-branch");
            ui.add_sized(
                [90.0, 20.0],
                egui::TextEdit::singleline(&mut app.clone_branch_input)
                    .hint_text(hint),
            );

            if ui.button(app.t("btn-clone")).clicked() { actions.trigger_clone = true; }
            ui.separator();
            if ui.button(app.t("btn-settings")).clicked() { actions.open_settings = true; }
            if ui.button(app.t("btn-about")).clicked()    { actions.open_about    = true; }
            ui.separator();
            lang_switcher(ui, app, &mut actions);
        });
    }

    actions
}

fn lang_switcher(ui: &mut egui::Ui, app: &mut NekoApp, actions: &mut ToolbarActions) {
    const LANGUAGES: &[(Language, &str, &str)] = &[
        (Language::Ru, "🇷🇺 RU", "🇷🇺 Русский"),
        (Language::En, "🇬🇧 EN", "🇬🇧 English"),
        (Language::Uk, "🇺🇦 UA", "🇺🇦 Українська"),
        (Language::Be, "🇧🇾 BY", "🇧🇾 Беларуская"),
    ];

    let selected_label = LANGUAGES
        .iter()
        .find(|(lang, _, _)| *lang == app.current_lang)
        .map(|(_, short, _)| *short)
        .unwrap_or("🌐 LA");

    egui::ComboBox::from_id_salt("lang_switcher")
        .width(85.0)
        .selected_text(selected_label)
        .show_ui(ui, |ui| {
            LANGUAGES.iter().for_each(|&(lang, _, long_label)| {
                let clicked = ui
                    .selectable_value(&mut app.current_lang, lang, long_label)
                    .clicked();

                clicked
                    .then_some(lang)
                    .map(|changed_lang| actions.lang_changed = Some(changed_lang));
            });
        });
}