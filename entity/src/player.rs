use std::net::TcpStream;
use std::sync::{Mutex};
use std::io::{Result, Write};

use super::entity::Entity;
use super::registry::{Identified, EntityRegistry};
use protocol::PacketWriter;
use protocol::clientbound::Clientbound;
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

    pub fn send(&self, packet: impl Into<Clientbound>) -> Result<()> {
        self.send_bytes(&packet.into().to_bytes()?)
    }

    pub fn send_bytes(&self, bytes: &[u8]) -> Result<()> {
        self.write.lock().unwrap().write_all(bytes)
    }
}

impl Identified for Player {
    fn id(&self) -> i32 {
        self.entity.id
    }
}

pub type PlayerRegistry = EntityRegistry<Player>;

impl PlayerRegistry {
    pub fn broadcast(&self, packet: impl Into<Clientbound>) {
        let Ok(bytes) = packet.into().to_bytes() else { return };

        self.for_each(|p| {
            let _ = p.send_bytes(&bytes);
        });
    }

    pub fn broadcast_except(&self, except: i32, packet: impl Into<Clientbound>) {
        let Ok(bytes) = packet.into().to_bytes() else { return };

        self.for_each(|p| if p.id() != except {
            let _ = p.send_bytes(&bytes);
        });
    }
}