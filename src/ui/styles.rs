use egui;

/* Centralized theme colors for the app. */
pub struct ThemeColors {
    pub window_fill: egui::Color32,

    // Panel
    pub panel_bg: egui::Color32,
    pub panel_stroke: egui::Color32,
    pub panel_stroke_light: egui::Color32,
    pub shadow: egui::Color32,

    // Text
    pub text_primary: egui::Color32,
    pub text_secondary: egui::Color32,
    pub text_on_primary: egui::Color32,

    // Buttons
    pub button_primary: egui::Color32,
    pub button_primary_hover: egui::Color32,
    pub button_recording: egui::Color32,
    pub button_recording_hover: egui::Color32,
    pub button_secondary_fill: egui::Color32,
    pub button_secondary_stroke: egui::Color32,

    // Selection
    pub selection_bg: egui::Color32,
    pub selection_stroke: egui::Color32,
}

impl ThemeColors {
    pub fn loom() -> ThemeColors {
        ThemeColors {
            window_fill: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0),
            // TODO: remove this after debugging
            // window_fill: egui::Color32::from_rgba_unmultiplied(128, 0, 0, 128),

            // Panel
            panel_bg: egui::Color32::from_rgb(255, 255, 255),
            panel_stroke: egui::Color32::from_rgb(235, 235, 240),
            panel_stroke_light: egui::Color32::from_rgb(230, 230, 235),
            shadow: egui::Color32::from_black_alpha(20),

            // Text
            text_primary: egui::Color32::from_rgb(50, 50, 60),
            text_secondary: egui::Color32::from_rgb(100, 100, 120),
            text_on_primary: egui::Color32::WHITE,

            // Buttons
            button_primary: egui::Color32::from_rgb(99, 102, 241),
            button_primary_hover: egui::Color32::from_rgb(79, 82, 221),
            button_recording: egui::Color32::from_rgb(255, 90, 95),
            button_recording_hover: egui::Color32::from_rgb(240, 75, 80),
            button_secondary_fill: egui::Color32::WHITE,
            button_secondary_stroke: egui::Color32::from_rgb(230, 230, 235),

            // Selection
            selection_bg: egui::Color32::from_rgb(99, 102, 241),
            selection_stroke: egui::Color32::from_rgb(99, 102, 241),
        }
    }
}
