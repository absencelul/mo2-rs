mod post_render;
mod process_event;

use crate::hooks::post_render::{hook_post_render, unhook_post_render};
use crate::hooks::process_event::{hook_process_event, unhook_process_event};
use crate::sdk::engine::UGameViewportClient;
use crate::sdk::{G_OBJECTS, G_WORLD};

pub fn unhook_all() {
    unhook_process_event();
    unhook_post_render();
}

pub fn initialize_hooks() -> Result<(), &'static str> {
    let g_world = unsafe { G_WORLD };
    let g_objects = unsafe { G_OBJECTS };

    if let Some((g_world, g_objects)) = g_world.zip(g_objects) {
        let g_world = unsafe { &*g_world };
        let g_objects = unsafe { &*g_objects };

        while *g_world as u64 == 0x0 || (*g_objects).len() < 1usize {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        println!("[-] GWorld: 0x{:X}", *g_world as u64);

        let game = unsafe { (*(*g_world)).owning_game_instance };
        if game.is_null() {
            return Err("[-] UGameInstance is null");
        }

        let players = unsafe { &(*game).local_players };
        if players.len() == 0 {
            return Err("[-] TArray<*const ULocalPlayer> is empty");
        }

        let local_player = players.get(0);
        if local_player.is_null() {
            return Err("[-] ULocalPlayer is null");
        }

        let viewport = unsafe { (*local_player).viewport_client } as *mut UGameViewportClient;
        if viewport.is_null() {
            return Err("[-] UGameViewportClient is null");
        }

        println!("[-] Hooking PostRender");
        if !hook_post_render(unsafe { &(*viewport).script_viewport_client_.object_ }) {
            return Err("[-] Failed to hook PostRender");
        }

        let controller = unsafe { (*local_player).player_.player_controller };
        if controller.is_null() {
            return Err("[-] APlayerController is null");
        }

        let hud = unsafe { (*controller).my_hud };
        while hud.is_null() {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        println!("[-] Hooking ProcessEvent");
        if !hook_process_event(unsafe { &(*hud).actor_.object_ }) {
            return Err("[-] Failed to hook ProcessEvent");
        }

        return Ok(());
    }

    Err("[-] Failed to initialize hooks")
}
