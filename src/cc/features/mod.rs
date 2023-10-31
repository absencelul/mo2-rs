use std::any::Any;

pub mod esp_feature;

pub trait Feature: Send + Sync {
    fn condition(&self, object: &dyn Any) -> bool;
    fn execute(&self, object: &dyn Any);
}
