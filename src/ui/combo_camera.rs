use eframe::egui;
use crate::config;

use super::styles::ThemeColors;

type CameraList = Vec<(i32, String)>;

pub fn show(
    ui: &mut egui::Ui,
    ui_state: &mut crate::camera_app::UiState,
    app_state: &mut crate::camera_app::AppState,
    colors: &ThemeColors,
    config: &config::AppConfig,
) {

    // reset
    ui_state.combo_open = false;
    let width = 170.0;
    
    let cameras = &app_state.detected_cameras;
    let selected = &mut app_state.selected_camera;

    egui::ComboBox::from_id_source("camera_select")
    .selected_text(
        egui::RichText::new(format!("📷 {}", cameras[*selected].1.clone()))
            .size(14.0)
            .color(colors.text_primary)
    )
    .width(width)
    .show_ui(ui, |ui| {

        ui_state.combo_open = true;
        ui.style_mut().spacing.item_spacing.y = 4.0;
        ui.style_mut().spacing.button_padding = egui::vec2(12.0, 10.0);

        for (i, camera) in cameras.iter().enumerate() {
                let is_selected = *selected == i;
                let response = ui.selectable_label(
                    is_selected,
                    egui::RichText::new(camera.1.clone())
                        .size(14.0)
                        .color(if is_selected {
                            colors.text_on_primary
                        } else {
                            colors.text_primary
                        })
                );
                
                if response.clicked() {
                    *selected = i;
                }
            }
    });


}

