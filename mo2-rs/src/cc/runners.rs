pub mod actor_objects;

pub trait Runner {
    fn condition(&self) -> bool;
    fn on_execute(&self);
}
