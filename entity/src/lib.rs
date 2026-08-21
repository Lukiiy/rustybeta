use std::sync::atomic::{AtomicI32, Ordering};

pub mod entity;
pub mod player;

static NEXT_ID: AtomicI32 = AtomicI32::new(0);

fn next_id() -> i32 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

pub mod registry;