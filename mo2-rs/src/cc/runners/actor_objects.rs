use sdk::get_g_world;

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
        if SHARED_DATA.g_world.is_none() {
            println!("LevelObjectsRunner::condition: g_world is None");
            return false;
        }

        if SHARED_DATA.g_world.unwrap().is_null() {
            println!("LevelObjectsRunner::condition: g_world is null");
            return false;
        }

        true
    }

    fn on_execute(&self) {
        let g_world = get_g_world();
        if g_world.is_null() {
            return;
        }

        let level = unsafe { (*g_world).persistent_level };
        if level.is_null() {
            return;
        }

        let actors = unsafe { &(*level).actors };
        if actors.is_empty() || actors.data.is_null() {
            return;
        }

        actors.iter().for_each(|actor| {
            if actor.is_null() {
                return;
            }

            for feature in &self.features {
                feature.before_execute();
                feature.execute(&actor);
                feature.after_execute();
            }
        });
    }
}
