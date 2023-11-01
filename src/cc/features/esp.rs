use crate::cc::features::Feature;
use crate::sdk::engine::AActor;
use std::any::Any;

pub struct EspFeature;

impl EspFeature {
    fn is_valid_player_state(&self, actor: &AActor) -> bool {
        true
    }
}

impl Feature for EspFeature {
    fn before_execute(&self) {}

    fn execute(&self, object: &dyn Any) {
        let actor = object.downcast_ref::<AActor>();
        if let Some(actor) = actor {
            if !self.is_valid_player_state(actor) {
                return;
            }
        }
    }

    fn after_execute(&self) {}
}
