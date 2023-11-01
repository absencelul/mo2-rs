use crate::sdk::engine::UWorld;
use std::sync::LazyLock;

use crate::sdk::G_WORLD;

pub struct SharedData {
    pub g: Option<*const UWorld>,
}

unsafe impl Sync for SharedData {}
unsafe impl Send for SharedData {}

impl Default for SharedData {
    fn default() -> Self {
        if let Some(g_world) = unsafe { G_WORLD } {
            let g_world = unsafe { &*g_world };
            return Self { g: Some(*g_world) };
        }

        Self { g: None }
    }
}

pub static SHARED_DATA: LazyLock<SharedData> = LazyLock::new(|| SharedData::default());
