use crate::sdk::engine::UWorld;
use std::sync::Mutex;
#[macro_use]
use lazy_static::lazy_static;

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

lazy_static! {
    pub static ref SHARED_DATA: Mutex<SharedData> = Mutex::new(SharedData::default());
}
