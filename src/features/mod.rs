mod esp;
mod aimbot;
mod triggerbot;

pub use esp::EspSystem;
pub use aimbot::Aimbot;
pub use triggerbot::Triggerbot;

pub struct GameState {
    local_player: usize,
    players: Vec<Player>,
    view_matrix: [f32; 16],
}

pub struct Player {
    position: [f32; 3],
    health: f32,
    team: i32,
    visible: bool,
}

impl GameState {
    pub fn update(&mut self, mem: &super::memory::MemoryManager) {
        self.local_player = mem.read(0x143FF0000).unwrap_or(0);
        self.view_matrix = mem.read(0x143AA0000).unwrap_or([0.0; 16]);
        self.players = (0..100)
            .filter_map(|i| mem.read::<Player>(0x144BB0000 + i * 0x200))
            .collect();
    }
}