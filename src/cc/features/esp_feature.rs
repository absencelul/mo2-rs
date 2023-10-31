use crate::cc::features::Feature;
use crate::sdk::engine::AActor;
use std::any::Any;

pub struct PlayerFeature;

impl Feature for PlayerFeature {
    fn condition(&self, object: &dyn Any) -> bool {
        object.downcast_ref::<AActor>().is_some()
    }

    fn execute(&self, _object: &dyn Any) {
        println!("PlayerFeature::execute")
    }
}
