use crate::cc::data_providers::DataProvider;
use crate::sdk::engine::AActor;

struct TestDataProvider;

impl DataProvider<AActor> for TestDataProvider {
    fn condition(&self, _object: &AActor) -> bool {
        true
    }

    fn after_execute(&self) {
        println!("TestDataProvider::after_execute")
    }

    fn execute(&self, _object: &AActor) {
        println!("TestDataProvider::execute")
    }
}
