use egui;
mod ui;
mod config;
mod camera_app;


fn config_style(ctx: &egui::Context) {

    let config = config::AppConfig::default();
    let colors = ui::styles::ThemeColors::loom();
    let mut style = (*ctx.style()).clone();
    // Global UI scale 
    // TODO: expose config to settings pane when made
    ctx.set_pixels_per_point(config.ui_scale);

    // Window rounding
    style.visuals.window_rounding = egui::Rounding::same(14.0);

    // Button / widget styling
    style.spacing.button_padding = egui::vec2(12.0, 8.0);
    style.visuals.widgets.inactive.bg_fill = colors.panel_bg;
    style.visuals.widgets.inactive.weak_bg_fill = colors.panel_bg;
    style.visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, colors.panel_stroke_light);
    style.visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, colors.text_primary);

    
    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(248, 248, 250);
    style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 220, 230));

    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(240, 240, 245);
    style.visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 200, 220));

    // Selection styling
    style.visuals.selection.bg_fill = colors.selection_bg;
    style.visuals.selection.stroke = egui::Stroke::new(1.0, colors.selection_stroke);

    // Rounding
    style.visuals.widgets.noninteractive.rounding = egui::Rounding::same(10.0);
    style.visuals.widgets.inactive.rounding = egui::Rounding::same(10.0);
    style.visuals.widgets.hovered.rounding = egui::Rounding::same(10.0);
    style.visuals.widgets.active.rounding = egui::Rounding::same(10.0);

    ctx.set_style(style);
}

fn run_ui() -> Result<(), Box<dyn std::error::Error>> {

    // Initialize egui 
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 250.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_window_level(egui::viewport::WindowLevel::AlwaysOnTop)
            .with_position([0.0, 0.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Mozannab",
        options,
        Box::new(|_cc| {
            // Config the style 
            config_style(&_cc.egui_ctx);

            Ok(Box::new(camera_app::CameraApp::new(_cc)))

        }),
    );

    Ok(())
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    return run_ui()
}
