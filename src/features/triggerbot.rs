use super::GameState;
use crate::gui::Overlay;

pub struct TriggerBotSystem {
    enabled: bool,
}

impl TriggerBotSystem {
    pub fn new() -> Self {
        Self { enabled: false }
    }

    pub fn update(&mut self, game_state: &GameState, overlay: &mut Overlay) {
        if !self.enabled {
            return;
        }

        for player in &game_state.players {
            if player.team != 0 || player.health <= 0.0 {
                continue;
            }

            let screen_pos = self.world_to_screen(player.position, &game_state.view_matrix);
            self.draw_box(screen_pos, overlay);
        }
    }

    fn world_to_screen(&self, pos: [f32; 3], view_matrix: &[f32; 16]) -> [f32; 2] {
        let clip_coords = [
            pos[0] * view_matrix[0] + pos[1] * view_matrix[4] + pos[2] * view_matrix[8] + view_matrix[12],
            pos[0] * view_matrix[1] + pos[1] * view_matrix[5] + pos[2] * view_matrix[9] + view_matrix[13],
            pos[0] * view_matrix[2] + pos[1] * view_matrix[6] + pos[2] * view_matrix[10] + view_matrix[14],
            pos[0] * view_matrix[3] + pos[1] * view_matrix[7] + pos[2] * view_matrix[11] + view_matrix[15],
        ];

        let ndc = [
            clip_coords[0] / clip_coords[3],
            clip_coords[1] / clip_coords[3],
        ];

        [ (ndc[0] + 1.0) * 960.0, (1.0 - ndc[1]) * 540.0 ]
    }

    fn draw_box(&self, screen_pos: [f32; 2], overlay: &mut Overlay) {
        let rect = egui::Rect::from_center_size(
            egui::Pos2::new(screen_pos[0], screen_pos[1]),
            egui::Vec2::new(50.0, 100.0),
        );
        overlay.context().layer_painter(LayerId::background()).rect(
            rect,
            2.0,
            egui::Color32::RED,
            egui::Stroke::new(1.0, Color32::WHITE),
        );
    }
}