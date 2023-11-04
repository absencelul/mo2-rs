use std::sync::LazyLock;

use sdk::engine::classes::{ULocalPlayer, UWorld};
use sdk::get_g_world;

pub struct SharedData {
    pub g_world: Option<*const UWorld>,
    pub local_player: Option<*const ULocalPlayer>,
}

unsafe impl Sync for SharedData {}

unsafe impl Send for SharedData {}

impl Default for SharedData {
    fn default() -> Self {
        let g_world = get_g_world();
        unsafe {
            if !g_world.is_null() {
                let game = (*g_world).owning_game_instance;
                let players = &(*game).local_players;
                let local_player = players.get(0);
                return Self { g_world: Some(g_world), local_player: Some(local_player) };
            }
        }

        Self { g_world: None, local_player: None }
    }
}

pub static SHARED_DATA: LazyLock<SharedData> = LazyLock::new(|| SharedData::default());
