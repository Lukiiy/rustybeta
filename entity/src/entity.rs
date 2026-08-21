use world::Position;
use super::next_id;

pub struct Entity {
    pub id: i32,
    pub position: Position,
}

impl Entity {
    pub fn new(position: Position) -> Self {
        Self { id: next_id(), position }
    }
}