use std::any::Any;

use sdk::engine::classes::AActor;
use sdk::get_g_world;

use crate::cc::features::Feature;

pub struct EspFeature;

impl EspFeature {
    fn is_valid_actor(&self, actor: &AActor) -> bool {
        let name = actor.object_.name.get_name();
        println!("Name: {}", name);

        if name.is_empty() || !name.contains("BP_PlayerCharacter_C") {
            return false;
        }

        let g_world = get_g_world();
        if g_world.is_null() {
            return false;
        }

        let game = unsafe { (*g_world).owning_game_instance };
        if game.is_null() {
            return false;
        }

        let is_valid = unsafe {
            let player = (*game).local_players.get(0);
            if player.is_null() {
                return false;
            }

            let player_controller = (*player).player_.player_controller;
            if player_controller.is_null() {
                return false;
            }

            let acknowledged_pawn = (*player_controller).acknowledged_pawn;
            if acknowledged_pawn.is_null() {
                return false;
            }

            let my_actor = &(*acknowledged_pawn).actor_;
            let distance = my_actor.get_distance_to(actor);
            println!("Distance: {}", distance);

            distance < 250f32
        };

        is_valid
    }
}

impl Feature for EspFeature {
    fn before_execute(&self) {}

    fn execute(&self, object: &dyn Any) {
        let actor = object.downcast_ref::<AActor>();
        if let Some(actor) = actor {
            if !self.is_valid_actor(actor) {
                return;
            }
        }
    }

    fn after_execute(&self) {}
}
