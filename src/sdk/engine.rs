use once_cell::sync::OnceCell;

use crate::sdk::basic::{FName, FString, TArray};
use crate::sdk::core::{FLinearColor, FVector2D, UClass, UFunction, UObject};

/**
 * Class Engine.Actor
 * Size -> 0x01F8 (FullSize[0x0220] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct AActor {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x1f8], // 0x28(0x1f8)
}

/**
 * Class Engine.Canvas
 * Size -> 0x02A8 (FullSize[0x02D0] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct UCanvas {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x2a8], // 0x28(0x2a8)
}

impl UCanvas {
    pub fn k2_draw_text(
        &self,
        font: *const UFont,
        screen: FVector2D,
        text: &str,
        color: FLinearColor,
    ) {
        #[repr(C)]
        pub struct Params {
            render_font: *const UFont,
            render_text: FString,
            screen_position: FVector2D,
            scale: FVector2D,
            render_color: FLinearColor,
            kerning: f64,
            shadow_color: FLinearColor,
            shadow_offset: FVector2D,
            b_centre_x: bool,
            b_centre_y: bool,
            b_outlined: bool,
            outline_color: FLinearColor,
        }

        let mut params = Params {
            render_font: font,
            render_text: FString::new(text),
            screen_position: screen,
            scale: FVector2D { x: 1f32, y: 1f32 },
            render_color: color,
            kerning: 0f64,
            shadow_color: FLinearColor {
                r: 0f32,
                g: 0f32,
                b: 0f32,
                a: 0f32,
            },
            shadow_offset: FVector2D { x: 1f32, y: 1f32 },
            b_centre_x: true,
            b_centre_y: true,
            b_outlined: true,
            outline_color: FLinearColor {
                r: 0f32,
                g: 0f32,
                b: 0f32,
                a: 1f32,
            },
        };

        static DRAW_TEXT: OnceCell<usize> = OnceCell::new();
        DRAW_TEXT.get_or_init(|| {
            UObject::find_object::<usize>("Function Engine.Canvas.K2_DrawText") as usize
        });
        let func = *DRAW_TEXT.get().unwrap() as *const UFunction;
        self.object_
            .process_event(func, &mut params as *mut _ as *mut usize);
    }
}

/**
 * Class Engine.HUD
 * Size -> 0x00F0 (FullSize[0x0310] - InheritedSize[0x0220])
 */
#[repr(C)]
pub struct AHUD {
    pub actor_: AActor,
    // 0x00(0x220)
    pub player_owner: *const APlayerController,
    // 0x220(0x8)
    pad_228: [u8; 0x1],
    // 0x228(0x1)
    pad_229: [u8; 0x3],
    // 0x229(0x3)
    pub current_target_index: i32,
    // 0x22c(0x4)
    pad_230: [u8; 0x1],
    // 0x230(0x1)
    pad_231: [u8; 0x7],
    // 0x231(0x7)
    post_rendered_actors: TArray<*const AActor>,
    // 0x238(0x10)
    pad_248: [u8; 0x8],
    // 0x248(0x8)
    debug_display: TArray<FName>,
    // 0x250(0x10)
    toggled_debug_categories: TArray<FName>,
    // 0x260(0x10)
    pub canvas: *const UCanvas,
    // 0x270(0x8)
    debug_canvas: *const UCanvas,
    // 0x278(0x8)
    debug_text_list: [u8; 0x10],
    // 0x280(0x10) TArray<FDebugTextInfo> debug_text_list
    show_debug_target_desired_class: *const AActor,
    // 0x290(0x8)
    show_debug_target_actor: *const AActor,
    // 0x298(0x8)
    pad_2a0: [u8; 0x70], // 0x2a0(0x70)
}

/**
 * Class Engine.Player
 * Size -> 0x0020 (FullSize[0x0048] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct UPlayer {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x8],
    // 0x28(0x8)
    pub player_controller: *const APlayerController,
    // 0x30(0x8)
    current_net_speed: i32,
    // 0x38(0x4)
    configured_internet_speed: i32,
    // 0x3C(0x4)
    configured_lan_speed: i32,
    // 0x40(0x4)
    pad_44: [u8; 0x4], // 0x44(0x4)
}

/**
 * Class Engine.Controller
 * Size -> 0x0078 (FullSize[0x0298] - InheritedSize[0x0220])
 */
#[repr(C)]
pub struct AController {
    pub actor_: AActor,
    // 0x00(0x220)
    pad_220: [u8; 0x78], // 0x220(0x78)
}

/**
 * Class Engine.Pawn
 * Size -> 0x0060 (FullSize[0x0280] - InheritedSize[0x0220])
 */
#[repr(C)]
pub struct APawn {
    pub actor_: AActor,
    // 0x00(0x220)
    pad_220: [u8; 0x60], // 0x220(0x60)
}

/**
 * Class Engine.PlayerController
 * Size -> 0x02D8 (FullSize[0x0570] - InheritedSize[0x0298])
 */
#[repr(C)]
pub struct APlayerController {
    pub controller_: AController,
    // 0x00(0x298)
    pub player: *const UPlayer,
    // 0x298(0x8)
    pub acknowledged_pawn: *const APawn,
    // 0x2A0(0x8)
    controlling_dir_track_inst: *const usize,
    // 0x2A8(0x8) UInterpTrackInstDirector* controlling_dir_track_inst
    pub my_hud: *const AHUD,
    // 0x2B0(0x8)
    pub player_camera_manager: *const usize,
    // 0x2B8(0x8) APlayerCameraManager* player_camera_manager
    player_camera_manager_class: *const UClass,
    // 0x2C0(0x8)
    pad_2c8: [u8; 0x2a8], // 0x2c8(0x2a8)
}

/**
 * Class Engine.LocalPlayer
 * Size -> 0x0210 (FullSize[0x0258] - InheritedSize[0x0048])
 */
#[repr(C)]
pub struct ULocalPlayer {
    pub player_: UPlayer,
    // 0x00(0x48)
    pad_48: [u8; 0x28],
    // 0x48(0x28)
    pub viewport_client: *const UGameViewportClient,
    // 0x70(0x8)
    pad_78: [u8; 0x1C],
    // 0x78(0x1C)
    aspect_ratio_axis_constraint: [u8; 0x1],
    // 0x94(0x1)
    pad_95: [u8; 0x3],
    // 0x95(0x3)
    pending_level_player_controller_class: *const UClass,
    // 0x98(0x8)
    sent_split_join: [u8; 0x1],
    // 0xA0(0x1)
    pad_a1: [u8; 0x17],
    // 0xA1(0x17)
    controller_id: i32,
    // 0xB8(0x4)
    pad_bc: [u8; 0x19C], // 0xBC(0x19C)
}

/**
 * Class Engine.GameInstance
 * Size -> 0x0180 (FullSize[0x01A8] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct UGameInstance {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x10],
    // 0x28(0x10)
    pub local_players: TArray<*const ULocalPlayer>,
    // 0x38(0x10)
    online_session: *const usize,
    // 0x48(0x8) UOnlineSession* online_session
    referenced_objects: TArray<*const UObject>,
    // 0x50(0x10)
    pad_60: [u8; 0x18],
    // 0x60(0x18)
    on_pawn_controller_changed_delegates: [u8; 0x10],
    // 0x78(0x10) FScriptMulticastDelegate on_pawn_controller_changed_delegates
    pad_88: [u8; 0x120], // 0x88(0x120)
}

/**
 * Class Engine.BlueprintFunctionLibrary
 * Size -> 0x0000 (FullSize[0x0028] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct UBlueprintFunctionLibrary {
    pub object_: UObject, // 0x00(0x28)
}

/**
 * Class Engine.GameplayStatics
 * Size -> 0x0000 (FullSize[0x0028] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct UGameplayStatics {
    pub blueprint_function_library_: UBlueprintFunctionLibrary, // 0x00(0x28)
}

impl UGameplayStatics {
    pub fn spawn_object(&self, class: *const UClass, outer: *const UObject) -> *const UObject {
        static mut FUNC: *mut UFunction = std::ptr::null_mut();
        struct SpawnObjectParams {
            class: *const UClass,
            outer: *const UObject,
            return_val: *const UObject,
        }

        unsafe {
            if FUNC.is_null() {
                FUNC = UObject::find_object::<UFunction>(
                    "Function Engine.GameplayStatics.SpawnObject",
                ) as *mut UFunction;
                if FUNC.is_null() {
                    println!("[-] Failed to find SpawnObject function");
                    return std::ptr::null();
                }
            }
            let mut params = SpawnObjectParams {
                class,
                outer,
                return_val: std::ptr::null_mut(),
            };
            let flags = (*FUNC).function_flags;
            self.blueprint_function_library_
                .object_
                .process_event(FUNC, &mut params as *mut _ as *mut usize);
            (*FUNC).function_flags = flags;
            params.return_val
        }
    }
}

/**
 * Class Engine.Console
 * Size -> 0x0108 (FullSize[0x0130] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct UConsole {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x10],
    // 0x28(0x10)
    pub console_target_player: *const ULocalPlayer,
    // 0x38(0x8)
    default_texture_black: *const u64,
    // 0x40(0x8) UTexture2D* default_texture_black
    default_texture_white: *const u64,
    // 0x48(0x8) UTexture2D* default_texture_white
    pad_50: [u8; 0x18],
    // 0x50(0x18)
    history_buffer: TArray<FString>,
    // 0x68(0x10) TArray<FString> history_buffer
    pad_78: [u8; 0xb8], // 0x78(0xb8)
}

#[repr(C)]
pub struct UScriptViewportClient {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x10], // 0x28(0x10)
}

#[repr(C)]
pub struct UGameViewportClient {
    pub script_viewport_client_: UScriptViewportClient,
    // 0x00(0x38)
    pad_38: [u8; 0x8],
    // 0x38(0x8)
    pub viewport_console: *const UConsole,
    // 0x40(0x8)
    pad_48: [u8; 0x10],
    // 0x48(0x10) TArray<FDebugDisplayProperty> debug_properties
    pad_58: [u8; 0x10],
    // 0x58(0x10)
    max_split_screen_players: i32,
    // 0x68(0x4)
    pad_6c: [u8; 0xC],
    // 0x6c(0xC)
    pub world: *const UWorld,
    // 0x78(0x8)
    pub game_instance: *const UGameInstance,
    // 0x80(0x8)
    pad_88: [u8; 0x2D8], // 0x88(0x2D8)
}

/**
 * Class Engine.Level
 * Size -> 0x0270 (FullSize[0x0298] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct ULevel {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x70],
    // 0x28(0x70)
    pub actors: TArray<*const AActor>,
    // 0x98(0x10)
    pub garbage_actors: TArray<*const AActor>,
    // 0xa8(0x10)
    pub owning_world: *const UWorld,
    // 0xb8(0x8)
    model: *const usize,
    // 0xc0(0x8) UModel* model
    model_components: TArray<*const usize>,
    // 0xc8(0x10) TArray<UModelComponent> model_components
    actor_cluster: *const usize,
    // 0xd8(0x8) ULevelActorContainer* actor_cluster
    num_texture_streaming_un_built_components: i32,
    // 0xe0(0x4)
    num_texture_streaming_dirty_resources: i32,
    // 0xe4(0x4)
    level_script_actor: *const usize,
    // 0xe8(0x8) ALevelScriptActor* level_script_actor
    nav_list_start: *const usize,
    // 0xf0(0x8) ANavigationObjectBase* nav_list_start
    nav_list_end: *const usize,
    // 0xf8(0x8) ANavigationObjectBase* nav_list_end
    nav_data_chunks: TArray<*const usize>,
    // 0x100(0x10) TArray<UNavigationDataChunk*> nav_data_chunks
    light_map_total_size: f32,
    // 0x110(0x4)
    shadow_map_total_size: f32,
    // 0x114(0x4)
    static_navigable_geometry: [u8; 0x10],
    // 0x118(0x10) TArray<FVector> static_navigable_geometry
    streaming_texture_guids: [u8; 0x10],
    // 0x128(0x10) TArray<FGuid> streaming_texture_guids
    pad_138: [u8; 0x98],
    // 0x138(0x98)
    level_build_data_id: [u8; 0x10],
    // 0x1d0(0x10) FGuid level_build_data_id
    map_build_data: *const usize,
    // 0x1e0(0x8) UMapBuildDataRegistry* map_build_data
    light_build_level_offset: [u8; 0xC],
    // 0x1e8(0xC) FIntVector light_build_level_offset
    pad_1f4: [u8; 0x1],
    // 0x1f4(0x1)
    pad_1f5: [u8; 0x63],
    // 0x1f5(0x63)
    world_settings: *const usize,
    // 0x258(0x8) AWorldSettings* world_settings
    pad_260: [u8; 0x8],
    // 0x260(0x8)
    asset_user_data: TArray<*const usize>,
    // 0x268(0x10) TArray<UAssetUserData*> asset_user_data
    pad_278: [u8; 0x10],
    // 0x278(0x10)
    destroyed_replicated_static_actors: [u8; 0x10], // 0x288(0x10) TArray<FReplicatedStaticActorDestructionInfo> destroyed_replicated_static_actors
}

/**
 * Class Engine.World
 * Size -> 0x0770 (FullSize[0x0798] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct UWorld {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x8],
    // 0x28(0x8)
    pub persistent_level: *const ULevel,
    // 0x30(0x8)
    net_driver: *const usize,
    // 0x38(0x8) UNetDriver* net_driver
    line_batcher: *const usize,
    // 0x40(0x8) ULineBatchComponent* line_batcher
    persistent_level_batcher: *const usize,
    // 0x48(0x8) ULineBatchComponent* persistent_level_batcher
    foreground_line_batcher: *const usize,
    // 0x50(0x8) ULineBatchComponent* foreground_line_batcher
    network_manager: *const usize,
    // 0x58(0x8) AGameNetworkManager* network_manager
    physics_collision_handler: *const usize,
    // 0x60(0x8) UPhysicsCollisionHandler* physics_collision_handler
    extra_referenced_objects: TArray<*const UObject>,
    // 0x68(0x10)
    per_module_data_objects: TArray<*const UObject>,
    // 0x78(0x10)
    streaming_levels: TArray<*const usize>,
    // 0x88(0x10) TArray<ULevelStreaming> streaming_levels
    streaming_levels_to_consider: [u8; 0x28],
    // 0x98(0x28) FStreamingLevelsToConsider streaming_levels_to_consider
    streaming_levels_prefix: FString,
    // 0xC0(0x10)
    current_level_pending_visibility: *const ULevel,
    // 0xD0(0x8)
    current_level_pending_invisibility: *const ULevel,
    // 0xD8(0x8)
    demo_net_driver: *const usize,
    // 0xE0(0x8) UDemoNetDriver* demo_net_driver
    my_particle_event_manager: *const usize,
    // 0xE8(0x8) AParticleEventManager* my_particle_event_manager
    default_physics_volume: *const usize,
    // 0xF0(0x8) APhysicsVolume* default_physics_volume
    pad_f8: [u8; 0x16],
    // 0xF8(0x16)
    pad_10e: [u8; 0x1],
    // 0x10e(0x1)
    pad_10f: [u8; 0x1],
    // 0x10f(0x1)
    navigation_system: *const usize,
    // 0x110(0x8) UNavigationSystemBase* navigation_system
    authority_game_mode: *const usize,
    // 0x118(0x8) AGameModeBase* authority_game_mode
    game_state: *const usize,
    // 0x120(0x8) AGameStateBase* game_state
    ai_system: *const usize,
    // 0x128(0x8) UAISystemBase* ai_system
    avoidance_manager: *const usize,
    // 0x130(0x8) UAvoidanceManager* avoidance_manager
    pub levels: TArray<*const ULevel>,
    // 0x138(0x10)
    level_collections: [u8; 10],
    // 0x148(0x10) TArray<FLevelCollection> level_collections
    pad_158: [u8; 0x28],
    // 0x158(0x28)
    pub owning_game_instance: *const UGameInstance,
    // 0x180(0x8)
    parameter_collection_instances: [u8; 0x10],
    // 0x188(0x10)
    canvas_for_rendering_to_target: *const UCanvas,
    // 0x198(0x8)
    canvas_for_draw_material_to_render_target: *const UCanvas,
    // 0x1a0(0x8)
    pad_1a8: [u8; 0x50],
    // 0x1a8(0x50)
    physics_field: *const usize,
    // 0x1f8(0x8) UPhysicsFieldComponent* physics_field
    components_that_need_pre_end_of_frame_sync: [u8; 0x50],
    // 0x200(0x50)
    components_that_need_end_of_frame_update: TArray<*const usize>,
    // 0x250(0x10)
    components_that_need_end_of_frame_update_on_game_thread: TArray<*const usize>,
    // 0x260(0x10)
    pad_270: [u8; 0x370],
    // 0x270(0x370)
    world_composition: *const usize,
    // 0x5e0(0x8) UWorldComposition* world_composition
    pad_5e8: [u8; 0x90],
    // 0x5e8(0x90)
    psc_pool: [u8; 0x58],
    // 0x678(0x58) FWorldPSCPool
    pad_6d0: [u8; 0xc8], // 0x6d0(0xc8)
}

/**
 * Class Engine.Font
 * Size -> 0x01A8 (FullSize[0x01D0] - InheritedSize[0x0028])
 */
#[repr(C)]
pub struct UFont {
    pub object_: UObject,
    // 0x00(0x28)
    pad_28: [u8; 0x180], // 0x28(0x180)
}

impl UFont {
    pub fn get_font() -> *const Self {
        unsafe { UObject::find_object::<UFont>("Font Roboto.Roboto") }
    }
}
