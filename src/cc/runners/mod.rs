use std::any::Any;
use crate::cc::features::Feature;

pub mod level_objects_runner;

pub trait Runner {
    fn condition(&self) -> bool;
    fn on_execute(&self, features: &[Box<dyn Feature>]);
}
