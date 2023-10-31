use crate::cc::features::Feature;
use crate::cc::runners::Runner;
use crate::shared::SHARED_DATA;

pub struct LevelObjectsRunner;

impl Runner for LevelObjectsRunner {
    fn condition(&self) -> bool {
        let data = SHARED_DATA.lock().unwrap();
        if data.g.is_none() {
            println!("LevelObjectsRunner::condition: g_world is None");
            return false;
        }

        if data.g.unwrap().is_null() {
            println!("LevelObjectsRunner::condition: g_world is null");
            return false;
        }

        true
    }

    fn on_execute(&self, features: &[Box<dyn Feature>]) {
        println!("LevelObjectsRunner::on_execute");

        let data = SHARED_DATA.lock().unwrap();
        let g_world = data.g.unwrap();
        let g_world = unsafe { &*g_world };
        let level = (*g_world).persistent_level;
        if level.is_null() {
            println!("LevelObjectsRunner::on_execute: level is null");
            return;
        }

        let level = unsafe { &*level };
        let actors = &(*level).actors;
        if actors.is_empty() {
            println!("LevelObjectsRunner::on_execute: actors is null or empty");
            return;
        }

        actors.iter().for_each(|actor| {
            if !actor.is_null() {
                for feature in features {
                    if feature.condition(&actor) {
                        feature.execute(&actor);
                    }
                }
            }
        });
    }
}
