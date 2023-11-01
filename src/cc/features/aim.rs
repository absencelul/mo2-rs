use crate::cc::features::Feature;
use crate::sdk::engine::AActor;
use std::any::Any;

pub struct AimFeature;

impl Feature for AimFeature {
    fn before_execute(&self) {}

    fn execute(&self, object: &dyn Any) {
        if object.downcast_ref::<AActor>().is_none() {
            return;
        }
        println!("AimFeature::execute");
    }

    fn after_execute(&self) {}
}
