#![feature(lazy_cell)]

use std::sync::{LazyLock, Mutex};

use crate::basic::classes::{FNamePool, TUObjectArray};
use crate::engine::classes::UWorld;

pub mod basic;
pub mod core;
pub mod engine;

const GLOBAL_OBJECTS_OFFSET: u64 = 0x0CC44C20;
const GLOBAL_WORLD_OFFSET: u64 = 0x0CD8CAA0;
const GLOBAL_NAMES_OFFSET: u64 = 0x0CC088C0;

static mut G_WORLD: LazyLock<Mutex<Option<*const *const UWorld>>> =
    LazyLock::new(|| Mutex::new(None));
static mut G_NAMES: LazyLock<Mutex<Option<*const FNamePool>>> = LazyLock::new(|| Mutex::new(None));
static mut G_OBJECTS: LazyLock<Mutex<Option<*const TUObjectArray>>> =
    LazyLock::new(|| Mutex::new(None));

pub fn init_sdk() {
    get_g_world();
    get_g_names();
    get_g_objects();
}

pub fn get_g_world() -> *const UWorld {
    let mut g_world_guard = match unsafe { &*G_WORLD }.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    unsafe {
        **g_world_guard.get_or_insert_with(|| {
            std::mem::transmute::<u64, *const *const UWorld>(
                memory::get_base_address() + GLOBAL_WORLD_OFFSET,
            )
        })
    }
}

pub fn get_g_names() -> *const FNamePool {
    let mut g_names_guard = match unsafe { &*G_NAMES }.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *g_names_guard.get_or_insert_with(|| unsafe {
        std::mem::transmute::<u64, *const FNamePool>(
            memory::get_base_address() + GLOBAL_NAMES_OFFSET,
        )
    })
}

pub fn get_g_objects() -> *const TUObjectArray {
    let mut g_objects_guard = match unsafe { &*G_OBJECTS }.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    *g_objects_guard.get_or_insert_with(|| unsafe {
        std::mem::transmute::<u64, *const TUObjectArray>(
            memory::get_base_address() + GLOBAL_OBJECTS_OFFSET,
        )
    })
}
