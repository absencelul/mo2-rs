use std::ffi::CString;

use windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState;
use windows::{
    core::PCSTR,
    Win32::System::{
        Console::{AllocConsole, FreeConsole},
        LibraryLoader::{FreeLibraryAndExitThread, GetModuleHandleA},
    },
};

pub fn key_released(key: u16) -> bool {
    static mut PRESSED_KEYS: [bool; 255] = [false; 255];

    unsafe {
        let result = GetAsyncKeyState(key as i32);

        let is_currently_pressed = (result >> 15) & 1 != 0;
        let was_previously_pressed = PRESSED_KEYS[key as usize];

        PRESSED_KEYS[key as usize] = is_currently_pressed;

        !is_currently_pressed && was_previously_pressed
    }
}

pub fn get_base_address() -> u64 {
    unsafe { GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0 as u64 }
}

pub fn alloc_console() {
    unsafe {
        AllocConsole().expect("TODO: panic message");
    }
}

pub fn free_console() {
    unsafe {
        FreeConsole().expect("TODO: panic message");
    }
}

pub fn unload() {
    let module_name = CString::new("mo2_rs.dll").unwrap();
    println!("[-] unloading {:?}", module_name);

    unsafe {
        let module_handle =
            GetModuleHandleA(PCSTR::from_raw(module_name.as_ptr() as *const u8)).unwrap();
        FreeLibraryAndExitThread(module_handle, 0);
    }
}
