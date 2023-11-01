use retour::static_detour;
use std::sync::LazyLock;

use crate::cc::features::{aim, esp};
use crate::cc::runners::{actor_objects::ActorObjectsRunner, Runner};
use crate::gui::colors;
use crate::sdk::core::{FVector2D, UObject};
use crate::sdk::engine::{UCanvas, UFont, UGameViewportClient};
use crate::sdk::POST_RENDER_INDEX;

type FnPostRender =
    unsafe extern "fastcall" fn(viewport: *const UGameViewportClient, canvas: *const UCanvas);

static_detour! {
    static PostRender: unsafe extern "fastcall" fn(
        *const UGameViewportClient,
        *const UCanvas);
}

pub static ACTOR_OBJECTS_RUNNER: LazyLock<ActorObjectsRunner> = LazyLock::new(|| {
    ActorObjectsRunner::new(vec![
        Box::new(esp::EspFeature),
        Box::new(aim::AimFeature),
    ])
});

fn hk_post_render(viewport: *const UGameViewportClient, canvas: *const UCanvas) {
    unsafe {
        if ACTOR_OBJECTS_RUNNER.condition() {
            ACTOR_OBJECTS_RUNNER.on_execute();
        }

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

pub fn hook_post_render(object: &UObject) -> bool {
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

pub fn unhook_post_render() {
    if PostRender.is_enabled() {
        unsafe {
            PostRender.disable().unwrap();
        }
    }
}
