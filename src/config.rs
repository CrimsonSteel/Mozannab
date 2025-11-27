pub struct AppConfig {
    pub ui_scale: f32,
    pub button_width: f32,
    pub button_height: f32,
    pub compact_size: [f32; 2],
    pub window_title: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ui_scale: 0.75,
            button_width: 90.0,
            button_height: 32.0,
            compact_size: [450.0, 58.0],
            window_title: String::from("Mozannab"),
        }
    }
}