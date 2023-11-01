use crate::cc::features::Feature;
use crate::cc::runners::Runner;
use crate::shared::SHARED_DATA;

pub struct ActorObjectsRunner {
    features: Vec<Box<dyn Feature>>,
}

impl ActorObjectsRunner {
    pub fn new(features: Vec<Box<dyn Feature>>) -> Self {
        Self { features }
    }
}

impl Runner for ActorObjectsRunner {
    fn condition(&self) -> bool {
        if SHARED_DATA.g.is_none() {
            println!("LevelObjectsRunner::condition: g_world is None");
            return false;
        }

        if SHARED_DATA.g.unwrap().is_null() {
            println!("LevelObjectsRunner::condition: g_world is null");
            return false;
        }

        true
    }

    fn on_execute(&self) {
        println!("LevelObjectsRunner::on_execute");

        let g_world = unsafe { &*(SHARED_DATA.g.unwrap()) };
        let level = (*g_world).persistent_level;
        if level.is_null() {
            println!("LevelObjectsRunner::on_execute: level is null");
            return;
        }

        let level = unsafe { &*level };
        let actors = &(*level).actors;
        if actors.is_empty() || actors.data.is_null() {
            println!("LevelObjectsRunner::on_execute: actors is null or empty");
            return;
        }

        actors.iter().for_each(|actor| {
            if actor.is_null() {
                return;
            }

            for feature in &self.features {
                feature.execute(&actor);
            }
        });
    }
}
