use crate::events::event::Events;
use std::collections::HashMap;
use crate::assets::map::Object;
use linkme::distributed_slice;

pub type SpawnRegistrar = (&'static str, fn(Spawn, &mut Events));

/// Populated by `#[derive::spawn("Name")]` on spawn handlers; walked by `Spawner::discover`.
#[distributed_slice]
pub static SPAWNS: [SpawnRegistrar] = [..];

pub struct Spawner {
    spawns: HashMap<String, fn(Spawn, &mut Events)>
}

pub struct Spawn<'a> {
    pub x: f64,
    pub y: f64,
    pub object: &'a Object
}

impl Spawner {
    pub fn new() -> Self {
        Spawner { spawns: HashMap::new() }
    }

    pub fn discover() -> Self {
        let mut spawner = Spawner::new();
        for (name, handler) in SPAWNS.iter().copied() {
            spawner.register(name, handler);
        }
        spawner
    }

    pub fn spawn(&self, object: &Object, events: &mut Events) {
        let spawn = Spawn { x: object.x, y: object.y, object };
        self.spawns.get(&object.user_type).map(|f| f(spawn, events));
    }

    pub fn register(&mut self, name: &str, spawner: fn(Spawn, &mut Events)) {
        self.spawns.insert(name.to_string(), spawner);
    }
}