use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{LazyLock, Mutex, OnceLock};

use gui::components::label::{Alignment, Label};
use gui::components::window::Window;
use gui::{colors, Gui};
use retour::static_detour;
use sdk::core::classes::UObject;
use sdk::core::structs::FVector2D;
use sdk::engine::classes::{UCanvas, UGameViewportClient};

use crate::cc::features::{aim, esp};
use crate::cc::runners::actor_objects::ActorObjectsRunner;

type FnPostRender =
    unsafe extern "fastcall" fn(viewport: *const UGameViewportClient, canvas: *const UCanvas);

static_detour! {
    static PostRender: unsafe extern "fastcall" fn(
        *const UGameViewportClient,
        *const UCanvas);
}

pub static ACTOR_OBJECTS_RUNNER: LazyLock<ActorObjectsRunner> = LazyLock::new(|| {
    ActorObjectsRunner::new(vec![Box::new(esp::EspFeature), Box::new(aim::AimFeature)])
});

static mut GUI: OnceLock<Mutex<Gui>> = OnceLock::new();

static mut GUI_INITIALIZED: AtomicBool = AtomicBool::new(false);

fn setup_gui(canvas: &UCanvas) {
    unsafe {
        GUI.get_or_init(|| Mutex::new(Gui::new(&canvas)));

        if let Some(g) = GUI.get_mut() {
            let mut g = g.lock().unwrap();
            (*g).add_component(Label::new(
                &FVector2D { x: 5f32, y: 5f32 },
                Alignment::Left,
                true,
                &colors::TEXT,
                "Left Aligned from PostRender",
            ));

            (*g).add_component(Label::new(
                &FVector2D { x: 100f32, y: 100f32 },
                Alignment::Center,
                false,
                &colors::TEXT,
                "Center Aligned From PostRender!",
            ));

            // (*g).add_component(Rect::new(
            //     &FVector2D { x: 200f32, y: 200f32 },
            //     &FVector2D { x: 250f32, y: 150f32 },
            //     &colors::WINDOW_BACKGROUND,
            //     RectType::Filled,
            // ));

            (*g).add_component(Window::new(
                "Window Title",
                &FVector2D { x: 200f32, y: 200f32 },
                &FVector2D { x: 500f32, y: 400f32 },
                &colors::WINDOW_BACKGROUND,
                true,
            ));

            GUI_INITIALIZED.store(true, Relaxed);
        }
    }
}

fn hk_post_render(viewport: *const UGameViewportClient, canvas: *const UCanvas) {
    unsafe {
        // if ACTOR_OBJECTS_RUNNER.condition() {
        //     ACTOR_OBJECTS_RUNNER.on_execute();
        // }
        if !GUI_INITIALIZED.load(Relaxed) {
            setup_gui(&*canvas);
        }

        if let Some(g) = GUI.get() {
            let g = g.lock().unwrap();
            g.render();
        }

        // (*canvas).k2_draw_line(
        //     &FVector2D { x: 100f32, y: 100f32 },
        //     &FVector2D { x: 200f32, y: 200f32 },
        //     2f32,
        //     &colors::TEXT,
        // );

        PostRender.call(viewport, canvas);
    }
}

pub const POST_RENDER_INDEX: usize = 0x63;

pub fn hook_post_render(object: &UObject) -> bool {
    let vf_table = object.vf_table;
    unsafe {
        let address = *vf_table.add(POST_RENDER_INDEX);

        let fn_post_render: FnPostRender = std::mem::transmute(address as *const usize);
        PostRender.initialize(fn_post_render, hk_post_render).unwrap().enable().unwrap();
        fn_post_render as u64 > 0
    }
}

pub fn unhook_post_render() {
    if PostRender.is_enabled() {
        unsafe {
            PostRender.disable().unwrap();
        }
    }
}
