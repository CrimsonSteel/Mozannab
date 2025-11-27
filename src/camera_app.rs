use egui;
use eframe;

use crate::config::AppConfig;
use crate::ui;
use crate::ui::styles::ThemeColors;



pub struct UiState {
    pub combo_open: bool,
    pub prev_combo_open: bool,
    pub combo_enabled: bool,

    pub calibration_popup_step1_opened: bool,
    pub calibration_popup_step2_opened: bool,

}
impl UiState {
    pub fn new() -> Self {
        Self {
            combo_open: false,
            prev_combo_open: false,
            combo_enabled: false,
            calibration_popup_step1_opened: false,
            calibration_popup_step2_opened: false,
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
            detected_cameras: vec![(0, "Integrated Camera".to_string()), (1, "USB Camera".to_string()), (2, "Some strange camera fr".to_string())],
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

    fn render_immediate_calibration_step1(&mut self, ctx: &egui::Context, position: [f32; 2], viewport_size: [f32; 2]) {
        self.render_immediate_calibration_step(ctx, position, viewport_size, true);
    }

    fn render_immediate_calibration_step2(&mut self, ctx: &egui::Context, position: [f32; 2], viewport_size: [f32; 2]) {
        self.render_immediate_calibration_step(ctx, position, viewport_size, false);
    }

    fn render_immediate_calibration_step(&mut self, ctx: &egui::Context, position: [f32; 2], viewport_size: [f32; 2], good_bad: bool) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("immediate_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Immediate Viewport")
                .with_inner_size(viewport_size)
                .with_decorations(false)
                .with_transparent(true)
                .with_window_level(egui::viewport::WindowLevel::AlwaysOnTop)
                .with_position(position)
                ,
                |ctx, _class| {
                    egui::CentralPanel::default()
                        .frame(
                            egui::Frame::none()
                                .fill(self.theme.panel_bg)
                                .inner_margin(egui::Margin::same(14.0)),
                        )
                        .show(ctx, |ui| {

                            ui.disable();

                            // Center everything vertically and horizontally
                            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.spacing_mut().item_spacing.y = 18.0; 
                                    ui.add_space(60.0);
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new(
                                                format!("Please sit up in {} posture. When you're ready, press:", if good_bad { "good" } else { "bad" })
                                            )
                                            .size(20.0)
                                            .strong() 
                                        )
                                        .wrap()
                                    );
                                    
                                    ui.add(
                                        egui::Label::new(
                                            egui::RichText::new("Ctrl + Q")
                                                .size(40.0)
                                                .strong() 
                                                .color(self.theme.button_recording),
                                        )
                                    );
                                });
                            });
                        });
                },
        );
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


                if self.ui_state.calibration_popup_step1_opened || self.ui_state.calibration_popup_step2_opened {
  
                    let viewport_w = 340.0;
                    let viewport_h = 250.0;
                    
                    let viewport_info = ctx.input(|i| i.viewport().clone());
                    let screen_rect = viewport_info.monitor_size.unwrap_or_default();

                    let pos_x = screen_rect.x - viewport_w;
                    let pos_y = screen_rect.y - viewport_h;
                    let position = [pos_x/2.0, pos_y/2.0];

                    if self.ui_state.calibration_popup_step1_opened {
                        self.render_immediate_calibration_step1(ctx, position, [viewport_w, viewport_h]);
                    }
                    else {
                        self.render_immediate_calibration_step2(ctx, position, [viewport_w, viewport_h]);
                    }
                }
                

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

            ui::button_calibrate::show(ui, &mut self.ui_state, &mut self.state, &self.theme, &self.config);
            
            ui::button_control::show(ui, &mut self.ui_state, &mut self.state, &self.theme, &self.config);
            
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