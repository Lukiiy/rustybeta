use std::{any::{Any, TypeId}, collections::HashMap, sync::{Arc, RwLock}};

type Listener = Box<dyn Fn(&mut dyn Any) + Send + Sync>;

pub trait Cancellable {
    fn is_cancelled(&self) -> bool;
    fn set_cancelled(&mut self, cancelled: bool);
}

#[derive(Default)]
pub struct EventNode {
    listeners: RwLock<HashMap<TypeId, Vec<Listener>>>,
    children: RwLock<Vec<Arc<EventNode>>>
}

impl EventNode {
    pub fn new() -> Arc<Self> {
        Arc::default()
    }

    pub fn add_listener<E: Any>(&self, f: impl Fn(&mut E) + Send + Sync + 'static) -> &Self {
        let listener: Listener = Box::new(move |event| {
            if let Some(event) = event.downcast_mut::<E>() {
                f(event);
            }
        });

        self.listeners.write().unwrap().entry(TypeId::of::<E>()).or_default().push(listener);

        self
    }

    pub fn add_child(&self, child: Arc<EventNode>) {
        self.children.write().unwrap().push(child);
    }

    pub fn call<E: Any>(&self, event: &mut E) {
        self.dispatch(TypeId::of::<E>(), event);
    }

    fn dispatch(&self, id: TypeId, event: &mut dyn Any) {
        if let Some(listeners) = self.listeners.read().unwrap().get(&id) {
            listeners.iter().for_each(|l| l(event));
        }

        self.children.read().unwrap().iter().for_each(|c| c.dispatch(id, event));
    }
}