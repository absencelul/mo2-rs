use std::ffi::c_void;
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_END;

use crate::hooks::{initialize_hooks, unhook_all};

mod constants;
mod cc;
mod gui;
mod hooks;
mod sdk;
mod shared;
mod utils;

static mut EXITING: bool = false;

#[no_mangle]
extern "stdcall" fn DllMain(hinstance: HINSTANCE, reason: u32, _reserved: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(move || main_thread(hinstance));
        }
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        DLL_PROCESS_DETACH => {
            unhook_all();
            utils::free_console();
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
        _ => {}
    };

    BOOL::from(true)
}

fn on_loop() {
    if utils::key_released(VK_END.0) {
        unsafe {
            EXITING = true;
        };
        utils::unload();
    }
}

fn main_thread(_hinstance: HINSTANCE) {
    utils::alloc_console();
    sdk::initialize_globals();

    if let Err(e) = initialize_hooks() {
        println!("{}", e);
        return;
    }

    println!("[-] successfully loaded mo2_rs.dll");

    unsafe {
        #[allow(clippy::empty_loop)]
        while !EXITING {
            on_loop();
        }
    }
}
