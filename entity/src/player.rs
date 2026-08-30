use std::net::TcpStream;
use std::sync::{Mutex};
use std::io::{Result};

use super::entity::Entity;
use super::registry::{Identified, EntityRegistry};
use protocol::PacketWriter;
use world::Position;

pub struct Player {
    pub entity: Entity,
    pub username: String,

    position: Mutex<Position>,
    write: Mutex<TcpStream>
}

impl Player {
    pub fn new(entity: Entity, username: String, stream: TcpStream) -> Self {
        let position = entity.position;

        Self {
            entity,
            username,

            position: Mutex::new(position),
            write: Mutex::new(stream)
        }
    }

    pub fn id(&self) -> i32 {
        self.entity.id
    }

    pub fn position(&self) -> Position {
        *self.position.lock().unwrap()
    }

    pub fn set_position(&self, position: Position) {
        *self.position.lock().unwrap() = position;
    }

    pub fn send(&self, f: impl FnOnce(&mut PacketWriter<TcpStream>) -> Result<()>) -> Result<()> {
        let mut stream = self.write.lock().unwrap();

        f(&mut PacketWriter::new(&mut *stream))
    }
}

impl Identified for Player {
    fn id(&self) -> i32 {
        self.entity.id
    }
}

pub type PlayerRegistry = EntityRegistry<Player>;

impl PlayerRegistry {
    pub fn broadcast(&self, f: impl Fn(&mut PacketWriter<TcpStream>) -> Result<()>) {
        self.for_each(|player| { let _ = player.send(&f); });
    }

    pub fn broadcast_except(&self, except_id: i32, f: impl Fn(&mut PacketWriter<TcpStream>) -> Result<()>) {
        self.for_each(|player| {
            if player.id() != except_id {
                let _ = player.send(&f);
            }
        });
    }
}