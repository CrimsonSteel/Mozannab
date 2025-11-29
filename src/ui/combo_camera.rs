use eframe::egui;
use crate::config;
use super::styles::ThemeColors;

pub fn show(
    ui: &mut egui::Ui,
    ui_state: &mut crate::camera_app::UiState,
    app_state: &mut crate::camera_app::AppState,
    colors: &ThemeColors,
    config: &config::AppConfig,
) {

    ui_state.combo_enabled = !app_state.is_monitoring && !app_state.is_calibrating;

    let width = 170.0;
    let cameras = &app_state.detected_cameras;

     // just for safety
    if cameras.is_empty() {
        ui.horizontal(|ui| {
            ui.label("📷 No cameras");
        });

        return;
    }

    // just for safety, clamp selected camera
    app_state.selected_camera = app_state.selected_camera.clamp(0, cameras.len() as i32 - 1);

    // Save previous
    // let old_selected = app_state.selected_camera;

    ui.add_enabled_ui(ui_state.combo_enabled, |ui| {
        // let widget_id = ui.make_persistent_id("camera_select");
        let combo_box = egui::ComboBox::from_id_source("camera_select")
            .selected_text(
                {
                    egui::RichText::new(format!("📷 {}",cameras[app_state.selected_camera as usize].1))
                    .size(14.0)
                    .color(colors.text_primary)
                }
            )
            .width(width)
            .show_ui(ui, |ui| {
                ui.style_mut().spacing.item_spacing.y = 4.0;
                ui.style_mut().spacing.button_padding = egui::vec2(12.0, 10.0);
                
                for (i, cam) in cameras.iter().enumerate() {
                    let is_selected = app_state.selected_camera == i as i32;

                    let text = egui::RichText::new(cam.1.clone())
                        .size(14.0)
                        .color(if is_selected {
                            colors.text_on_primary
                        } else {
                            colors.text_primary
                        });
                        
                    let resp = ui.selectable_label(is_selected, text);

                    if resp.clicked() {
                        app_state.selected_camera = i as i32;
                    }
                }

            });
        
        ui_state.combo_open = combo_box.inner.is_some();
    });

    // Resize when opening
    if ui_state.combo_open && !ui_state.prev_combo_open {
        let items = cameras.len() as f32;
        let item_h = 24.0;
        let padding = items * item_h + 10.0;
        let new_h = 24.0*config.compact_size[1] + padding;

        crate::request_native_window_resize(&ui.ctx(), config.compact_size[0], new_h);
    }
    // Resize back when closed
    else if !ui_state.combo_open && ui_state.prev_combo_open {
        crate::request_native_window_resize(
            &ui.ctx(),
            config.compact_size[0],
            config.compact_size[1],
        );
    }

    ui_state.prev_combo_open = ui_state.combo_open;

}
