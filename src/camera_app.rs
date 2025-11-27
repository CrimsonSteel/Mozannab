use egui;
use eframe;

use crate::config::AppConfig;
use crate::ui;
use crate::ui::styles::ThemeColors;



pub struct UiState {
    pub combo_open: bool,
}
impl UiState {
    pub fn new() -> Self {
        Self {
            combo_open: false,
        }
    }
    
}

pub struct AppState {
    // opencv ID, camera name
    pub detected_cameras: Vec<(i32, String)>,
    pub selected_camera: usize,
    pub is_monitoring: bool,
    pub is_calibrating: bool,

}

impl AppState { 
    pub fn new() -> Self {
        Self {
            // placeholders for now for now
            detected_cameras: vec![(0, "Integrated Camera".to_string()), (1, "USB Camera".to_string())],
            selected_camera: 0,
            is_monitoring: false,
            is_calibrating: false,
        }
    }   
}

pub struct CameraApp {
    state: AppState,
    ui_state: UiState,
    config: AppConfig,
    theme: ThemeColors,
}

impl CameraApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::new(),
            ui_state: UiState::new(),
            config: AppConfig::default(),
            theme: ThemeColors::loom(),
        }
    }

    // Rendering main UI
    fn render_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            // Transparent background
            .frame(egui::Frame::none().fill(self.theme.window_fill))
            .show(ctx, |ui| {
                // Main Area
                egui::Area::new(egui::Id::new("main_ui"))
                    .fixed_pos(egui::pos2(0.0, 0.0))    
                    .show(ctx, |ui| {
                        self.render_main_panel(ui);
                    });
            });
    }

    // Render main panel UI
    fn render_main_panel(&mut self, ui: &mut egui::Ui) {
        egui::Frame::none()
            .fill(self.theme.panel_bg)
            .rounding(14.0)
            .stroke(egui::Stroke::new(1.0, self.theme.panel_stroke))
            .shadow(egui::epaint::Shadow {
                offset: egui::vec2(0.0, 4.0),
                blur: 16.0,
                spread: 0.0,
                color: self.theme.shadow,
            })
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                self.render_controls(ui);
            });
    }

    // Render cameras combo box and buttons (start/stop and calibrate)
    fn render_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {

            ui.spacing_mut().item_spacing.x = 10.0;

            // Camera combobox
            ui::combo_camera::show(ui, &mut self.ui_state, &mut self.state, &self.theme, &self.config);

            ui::button_control::show(ui, &mut self.ui_state, &mut self.state, &self.theme, &self.config);
            
            ui::button_calibrate::show(ui, &mut self.ui_state, &mut self.state, &self.theme, &self.config);
            
        });

    }


}

impl eframe::App for CameraApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        self.render_ui(ctx);
    }
}