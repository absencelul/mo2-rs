pub mod test_data_provider;

pub trait DataProvider<T> {
    fn condition(&self, object: &T) -> bool;
    fn after_execute(&self);
    fn execute(&self, object: &T);
}
