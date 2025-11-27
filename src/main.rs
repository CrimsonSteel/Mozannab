use egui;
mod ui;
mod config;
mod camera_app;
mod hotkey;
/* 
    Following functions are Windows only. They are used to handle dynamic window resize needed by combobox
*/
#[cfg(windows)]
fn windows_set_window_size_by_title(title: &str, width: i32, height: i32) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, SetWindowPos, SWP_NOZORDER, SWP_NOMOVE};

    let mut wide: Vec<u16> = OsStr::new(title).encode_wide().collect();
    wide.push(0);
    unsafe {
        let hwnd = FindWindowW(PCWSTR::null(), PCWSTR(wide.as_ptr()));
        if hwnd.0 != 0 {
            // SetWindowPos(hwnd, 0, x, y, cx, cy, flags)
            let _ = SetWindowPos(HWND(hwnd.0), HWND(0), 0, 0, width, height, SWP_NOZORDER | SWP_NOMOVE);
        }
    }
}

#[cfg(windows)]
fn request_native_window_resize(ctx: &egui::Context, width_points: f32, height_points: f32) {
    // convert logical points to physical pixels using egui's scale
    let ppp = ctx.pixels_per_point();
    let width_px = (width_points * ppp).round() as i32;
    let height_px = (height_points * ppp).round() as i32;
    let config = config::AppConfig::default();
    windows_set_window_size_by_title(&config.window_title, width_px, height_px);
}


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
