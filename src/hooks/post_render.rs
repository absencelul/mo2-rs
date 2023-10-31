use lazy_static::lazy_static;
use retour::static_detour;

use crate::gui::colors;
use crate::sdk::core::{FVector2D, UObject};
use crate::sdk::engine::{UCanvas, UFont, UGameViewportClient};
use crate::sdk::POST_RENDER_INDEX;
use crate::cc::features;
use crate::cc::features::Feature;
use crate::cc::runners::level_objects_runner::LevelObjectsRunner;
use crate::cc::runners::Runner;

type FnPostRender =
    unsafe extern "fastcall" fn(viewport: *const UGameViewportClient, canvas: *const UCanvas);

static_detour! {
    static PostRender: unsafe extern "fastcall" fn(
        *const UGameViewportClient,
        *const UCanvas);
}

lazy_static! {
    pub static ref FEATURES: Vec<Box<dyn Feature>> =
        vec![Box::new(features::esp_feature::PlayerFeature)];
    pub static ref LEVEL_RUNNER: LevelObjectsRunner = LevelObjectsRunner;
}

fn hk_post_render(viewport: *const UGameViewportClient, canvas: *const UCanvas) {
    unsafe {
        if LEVEL_RUNNER.condition() {
            LEVEL_RUNNER.on_execute(&FEATURES);
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
