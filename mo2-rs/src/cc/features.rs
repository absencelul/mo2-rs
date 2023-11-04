use std::any::Any;

pub mod aim;
pub mod esp;

pub trait Feature: Send + Sync {
    fn before_execute(&self);
    fn execute(&self, object: &dyn Any);
    fn after_execute(&self);
}
