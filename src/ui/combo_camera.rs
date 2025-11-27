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

    /* 
        if combo opened, we need to main window size (which is transparent) to be bigger (so the full combo have space to show),
        if closed, we need to get it back to its combact size

        This is to not have a transparent dead zone around the app where used cannot click (although it is transparent)
    */

    //opened
    if ui_state.combo_open && !ui_state.prev_combo_open {
        let items = cameras.len() as f32;
        let item_h = 24.0;
        let padding = items * item_h + 10.0;
        let new_h = config.compact_size[1] + padding;

        // dbg!("Here");
        crate::request_native_window_resize(&ui.ctx(), config.compact_size[0], new_h);
    }
    // closed
    else if !ui_state.combo_open && ui_state.prev_combo_open {
        crate::request_native_window_resize(&ui.ctx(), config.compact_size[0], config.compact_size[1]);
    }

    ui_state.prev_combo_open = ui_state.combo_open;

}

