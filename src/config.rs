pub struct AppConfig {
    pub ui_scale: f32,
    pub button_width: f32,
    pub button_height: f32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ui_scale: 0.75,
            button_width: 90.0,
            button_height: 32.0,
        }
    }
}