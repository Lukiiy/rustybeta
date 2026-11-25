use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use entity::player::{Player, PlayerRegistry};
use protocol::clientbound::Clientbound;
use protocol::clientbound::entity::{EntityMove, EntityTeleport};
use utils::math::{angle_byte, fixed_point};
use world::{Position, World};

pub enum ConnectionAction {
    Continue,
    Disconnect
}

pub struct ConnectionContext<'a> {
    pub player: &'a Arc<Player>,
    pub world: &'a Arc<Mutex<World>>,
    pub players: &'a PlayerRegistry
}

impl<'a> ConnectionContext<'a> {
    pub fn update_position(&self, new: Position) {
        let old = self.player.position();

        self.player.set_position(new);

        let id = self.player.id();
        let dx = fixed_point(new.x) - fixed_point(old.x);
        let dy = fixed_point(new.y) - fixed_point(old.y);
        let dz = fixed_point(new.z) - fixed_point(old.z);
        let yaw = angle_byte(new.yaw);
        let pitch = angle_byte(new.pitch);

        if (dx, dy, dz) == (0, 0, 0) && yaw == angle_byte(old.yaw) && pitch == angle_byte(old.pitch) {
            return;
        }

        let packet: Clientbound = match (i8::try_from(dx), i8::try_from(dy), i8::try_from(dz)) {
            (Ok(dx), Ok(dy), Ok(dz)) => EntityMove {
                id,
                dx,
                dy,
                dz,
                yaw,
                pitch
            }.into(),

            _ => EntityTeleport {
                id,
                position: new
            }.into()
        };

        self.players.broadcast_packet_except(id, packet);
    }
}