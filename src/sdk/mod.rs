use crate::sdk::basic::{FNamePool, TUObjectArray};
use crate::sdk::engine::UWorld;
use crate::utils::get_base_address;

pub(crate) mod basic;
pub(crate) mod core;
pub(crate) mod engine;

pub const PROCESS_EVENTS_INDEX: usize = 0x44;
pub const POST_RENDER_INDEX: usize = 0x63;
const GLOBAL_OBJECTS_OFFSET: u64 = 0x0CC44C20;
const GLOBAL_WORLD_OFFSET: u64 = 0x0CD8CAA0;
const GLOBAL_NAMES_OFFSET: u64 = 0x0CC088C0;

pub static mut G_OBJECTS: Option<*const TUObjectArray> = None;
pub static mut G_WORLD: Option<*const *const UWorld> = None;
pub static mut G_NAMES: Option<*const FNamePool> = None;

pub fn get_g_objects() -> *const TUObjectArray {
    let base_address = get_base_address();
    unsafe { std::mem::transmute(base_address + GLOBAL_OBJECTS_OFFSET) }
}

pub fn get_g_names() -> *const FNamePool {
    let base_address = get_base_address();
    unsafe { std::mem::transmute(base_address + GLOBAL_NAMES_OFFSET) }
}

pub fn get_g_world() -> *const *const UWorld {
    let base_address = get_base_address();
    unsafe { std::mem::transmute(base_address + GLOBAL_WORLD_OFFSET) }
}

pub fn initialize_globals() {
    unsafe {
        G_OBJECTS = Some(get_g_objects());
        G_NAMES = Some(get_g_names());
        G_WORLD = Some(get_g_world());
    }

    println!("[-] Base Address: 0x{:X}", get_base_address());
}
