use eframe::egui;
use super::styles::ThemeColors;

pub fn show(
    ui: &mut egui::Ui,
    ui_state: &mut crate::camera_app::UiState,
    app_state: &mut crate::camera_app::AppState,
    colors: &ThemeColors,
    config: &crate::config::AppConfig,
) {

    let is_calibrating = app_state.is_calibrating;
    let button_text = if is_calibrating { "Calibrating.." } else { "Calibrate" };

    let btn = egui::Button::new(
        egui::RichText::new(button_text)
            .size(14.0)
            .color(colors.text_secondary)
    )
    .fill(colors.button_secondary_fill)
    .rounding(10.0)
    .stroke(egui::Stroke::new(1.0, colors.button_secondary_stroke));


    let btn_size = [config.button_width, config.button_height];
    let response = ui.add_sized(btn_size, btn);

    if response.clicked() {
        exec_clicked(ui, ui_state, app_state, colors, config);
    }
}

pub fn exec_clicked(
    ui: &mut egui::Ui,
    ui_state: &mut crate::camera_app::UiState,
    app_state: &mut crate::camera_app::AppState,
    colors: &ThemeColors,
    config: &crate::config::AppConfig,
) {
    app_state.is_calibrating = !app_state.is_calibrating;

    ui_state.combo_enabled = !app_state.is_calibrating;


    ui_state.calibration_popup_step1_opened = app_state.is_calibrating;
}