use eframe::egui;
use super::styles::ThemeColors;
use std::sync::mpsc::{Sender, Receiver};

pub fn show(
    ui: &mut egui::Ui,
    ui_state: &mut crate::camera_app::UiState,
    app_state: &mut crate::camera_app::AppState,
    colors: &ThemeColors,
    config: &crate::config::AppConfig,
) -> bool {
    let is_monitoring = app_state.is_monitoring;
    let btn_size = [config.button_width, config.button_height];

    let button_text = if is_monitoring { "⏸ Pause" } else { "▶ Start" };
    let (button_color, button_hover) = if is_monitoring {
        (colors.button_recording, colors.button_recording_hover)
    } else {
        (colors.button_primary, colors.button_primary_hover)
    };

    let enabled = !app_state.is_calibrating && app_state.calibration_result.is_some();
    
    let mut clicked = false;

    ui.add_enabled_ui(enabled, |ui| {
        
        let btn = egui::Button::new(
            egui::RichText::new(button_text)
                .size(14.0)
                .color(colors.text_on_primary)
        )
        .fill(button_color)
        .rounding(10.0)
        .stroke(egui::Stroke::NONE);

        let response = ui.add_sized(btn_size, btn);

        if response.hovered() {
            ui.painter().rect_filled(response.rect, 10.0, button_hover);
            ui.painter().text(
                response.rect.center(),
                egui::Align2::CENTER_CENTER,
                button_text,
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );
        }

        if response.clicked() {
            exec_clicked(ui_state, app_state);

            clicked = true;
        }        
    });

    return clicked;
}


pub fn exec_clicked(
    ui_state: &mut crate::camera_app::UiState,
    app_state: &mut crate::camera_app::AppState
) {
    app_state.is_monitoring = !app_state.is_monitoring;

    ui_state.combo_enabled = !app_state.is_monitoring;
}