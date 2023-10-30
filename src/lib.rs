use std::ffi::c_void;

use retour::static_detour;
use windows::Win32::Foundation::{BOOL, HINSTANCE};
use windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,
};
use windows::Win32::UI::Input::KeyboardAndMouse::VK_END;

use crate::gui::colors;
use crate::sdk::core::{FVector2D, UObject};
use crate::sdk::engine::{UCanvas, UFont, UGameViewportClient};
use crate::sdk::{G_NAMES, G_OBJECTS, G_WORLD, POST_RENDER_INDEX, PROCESS_EVENTS_INDEX};

pub mod gui;
pub mod sdk;
pub mod utils;

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

type FnProcessEvent =
    unsafe extern "fastcall" fn(a1: *const UObject, a2: *const UObject, params: *mut usize);
static_detour! {
    static ProcessEvent: unsafe extern "fastcall" fn(
        *const UObject,
        *const UObject,
        *mut usize);
}
fn hk_process_event(a1: *const UObject, a2: *const UObject, params: *mut usize) {
    unsafe {
        let name = (*a2).get_full_name();
        println!("ProcessEvent: {}", name);
        if name != "Function Engine.HUD.ReceiveDrawHUD" {
            ProcessEvent.call(a1, a2, params);
            return;
        }

        ProcessEvent.call(a1, a2, params);
    }
}

fn hook_process_event(object: &UObject) -> bool {
    let vf_table = object.vf_table;
    unsafe {
        let address = *vf_table.add(PROCESS_EVENTS_INDEX);
        let fn_process_event: FnProcessEvent = std::mem::transmute(address as *const usize);

        ProcessEvent
            .initialize(fn_process_event, hk_process_event)
            .unwrap()
            .enable()
            .unwrap();

        fn_process_event as u64 > 0
    }
}

type FnPostRender =
    unsafe extern "fastcall" fn(viewport: *const UGameViewportClient, canvas: *const UCanvas);
static_detour! {
    static PostRender: unsafe extern "fastcall" fn(
        *const UGameViewportClient,
        *const UCanvas);
}
fn hk_post_render(viewport: *const UGameViewportClient, canvas: *const UCanvas) {
    unsafe {
        let font = UFont::get_font();
        if !font.is_null() {
            (*canvas).k2_draw_text(
                font,
                FVector2D { x: 0f32, y: 100f32 },
                "From PostRender!",
                colors::TEXT,
            );
        }

        PostRender.call(viewport, canvas);
    }
}

fn hook_post_render(object: &UObject) -> bool {
    let vf_table = object.vf_table;
    unsafe {
        let address = *vf_table.add(POST_RENDER_INDEX);

        let fn_post_render: FnPostRender = std::mem::transmute(address as *const usize);
        PostRender
            .initialize(fn_post_render, hk_post_render)
            .unwrap()
            .enable()
            .unwrap();
        fn_post_render as u64 > 0
    }
}

fn unhook_all() {
    if ProcessEvent.is_enabled() {
        unsafe {
            ProcessEvent.disable().unwrap();
        }
    }

    if PostRender.is_enabled() {
        unsafe {
            PostRender.disable().unwrap();
        }
    }
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

    println!("[-] Base Address: 0x{:X}", utils::get_base_address());

    unsafe {
        G_OBJECTS = Some(sdk::get_g_objects());
        G_NAMES = Some(sdk::get_g_names());
        G_WORLD = Some(sdk::get_g_world());

        let world = &*G_WORLD.unwrap();
        let obj = &*G_OBJECTS.unwrap();

        while *world as u64 == 0x0 || (*obj).len() < 1usize {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        println!("[-] GWorld: 0x{:X}", *world as u64);

        let game = (*(*world)).owning_game_instance;
        if game.is_null() {
            println!("[-] UGameInstance is null");
            return;
        }

        let players = &(*game).local_players;
        if players.len() == 0 {
            println!("[-] TArray<*const ULocalPlayer> is empty");
            return;
        }

        let local_player = players.get(0);
        if local_player.is_null() {
            println!("[-] ULocalPlayer is null");
            return;
        }

        let viewport = (*local_player).viewport_client as *mut UGameViewportClient;
        if viewport.is_null() {
            println!("[-] UGameViewportClient is null");
            return;
        }

        println!("[-] Hooking PostRender");
        hook_post_render(&(*viewport).script_viewport_client_.object_);

        let controller = (*local_player).player_.player_controller;
        if controller.is_null() {
            println!("[-] APlayerController is null");
            return;
        }

        // let mut console_class: *const UClass = std::ptr::null_mut();
        // let mut gameplay_statics: *const UGameplayStatics = std::ptr::null_mut();
        // loop {
        //     console_class = UObject::find_class("Class Engine.Console");
        //     gameplay_statics =
        //         UObject::find_class("Class Engine.GameplayStatics") as *const UGameplayStatics;
        //     if !console_class.is_null() && !gameplay_statics.is_null() {
        //         println!("[-] Console Class: 0x{:X}", console_class as u64);
        //         println!("[-] GameplayStatics: 0x{:X}", gameplay_statics as u64);
        //         break;
        //     }
        //
        //     std::thread::sleep(std::time::Duration::from_millis(100));
        // }

        // let console = (*gameplay_statics).spawn_object(
        //     console_class,
        //     &(*viewport).script_viewport_client_.object_ as *const _,
        // );
        // if console.is_null() {
        //     println!("[-] UConsole is null");
        //     return;
        // } else {
        //     println!("[-] UConsole: 0x{:X}", console as u64);
        //     (*viewport).viewport_console = console as *mut UConsole;
        // }

        let hud = (*controller).my_hud;
        while hud.is_null() {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        println!("[-] AHUD: 0x{:X}", hud as u64);
        println!("[-] Hooking ProcessEvent");
        hook_process_event(&(*hud).actor_.object_);
    }

    println!("[-] successfully loaded mo2_rs.dll");

    unsafe {
        #[allow(clippy::empty_loop)]
        while !EXITING {
            on_loop();
        }
    }
}
