use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use entity::player::{Player, PlayerRegistry};
use protocol::clientbound::ClientboundPacket;
use world::{Position, World};

pub enum ConnectionAction {
    Continue,
    Disconnect
}

pub struct ConnectionContext<'a> {
    pub stream: &'a mut TcpStream,
    pub player: &'a Arc<Player>,
    pub world: &'a Arc<Mutex<World>>,
    pub players: &'a PlayerRegistry
}

impl<'a> ConnectionContext<'a> {
    pub fn update_position(&self, position: Position) { // TODO
        self.player.set_position(position);

        self.players.broadcast_except(self.player.id(), |w| {
            ClientboundPacket::EntityTeleport { entity_id: self.player.id(), position }.write(w)
        });
    }
}