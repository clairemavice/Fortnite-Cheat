use egui::{Color32, Context, LayerId, Pos2, Rect, Sense, Shape, Vec2};
use parking_lot::Mutex;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref GUI_STATE: Arc<Mutex<GuiState>> = Arc::new(Mutex::new(GuiState::new()));
}

pub struct Overlay {
    context: Context,
}

impl Overlay {
    pub fn new() -> Self {
        Self {
            context: Context::default(),
        }
    }

    pub fn run(&mut self, config: super::utils::config::Config) {
        let mut run_ctx = egui::RunContext::new();
        loop {
            self.context.begin_frame(run_ctx.take());
            
            egui::Area::new("main")
                .fixed_pos(Pos2::new(50.0, 50.0))
                .show(&self.context, |ui| {
                    ui.heading("Game Assistant");
                    ui.separator();
                    self.build_esp_settings(ui, &config);
                    self.build_aim_settings(ui, &config);
                });

            self.context.end_frame();
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }

    fn build_esp_settings(&self, ui: &mut egui::Ui, config: &super::utils::config::Config) {
        ui.collapsing("Visuals", |ui| {
            ui.checkbox(&mut config.esp_enabled, "Player ESP");
            ui.checkbox(&mut config.item_esp, "Item ESP");
            ui.add(egui::Slider::new(&mut config.max_distance, 0.0..=500.0).text("Max Distance"));
        });
    }

    fn build_aim_settings(&self, ui: &mut egui::Ui, config: &super::utils::config::Config) {
        ui.collapsing("Combat", |ui| {
            ui.checkbox(&mut config.aimbot_enabled, "Aimbot");
            ui.checkbox(&mut config.triggerbot, "Triggerbot");
            ui.add(egui::Slider::new(&mut config.fov, 1.0..=180.0).text("Aim FOV"));
        });
    }
}

struct GuiState {
    visible: bool,
    keybind: u32,
}

impl GuiState {
    fn new() -> Self {
        Self {
            visible: true,
            keybind: 0x23,
        }
    }
}