use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Anything storable just needs a stable id
pub trait Identified {
    fn id(&self) -> i32;
}

/// Generic id to Arc<T> registry.
#[derive(Clone)]
pub struct EntityRegistry<T> {
    entries: Arc<Mutex<HashMap<i32, Arc<T>>>>
}

impl<T> Default for EntityRegistry<T> {
    fn default() -> Self {
        Self {
            entries: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl<T: Identified> EntityRegistry<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, entry: Arc<T>) {
        self.entries.lock().unwrap().insert(entry.id(), entry);
    }

    pub fn unregister(&self, id: i32) {
        self.entries.lock().unwrap().remove(&id);
    }

    pub fn get(&self, id: i32) -> Option<Arc<T>> {
        self.entries.lock().unwrap().get(&id).cloned()
    }

    pub fn for_each(&self, mut f: impl FnMut(&Arc<T>)) {
        for entry in self.entries.lock().unwrap().values() {
            f(entry);
        }
    }
}