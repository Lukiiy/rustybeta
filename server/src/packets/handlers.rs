use std::io::Result;
use std::net::TcpStream;
use protocol::{PacketReader, clientbound};

use super::context::{ConnectionAction, ConnectionContext};
use super::registry::ServerboundPacket;

use world::{Position, ItemStack};

// so...

pub struct KeepAlivePacket;

impl ServerboundPacket for KeepAlivePacket {
    const ID: u8 = 0x00;

    fn read(_reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self)
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct PlayerFlyingPacket {
    pub on_ground: bool
}

impl ServerboundPacket for PlayerFlyingPacket {
    const ID: u8 = 0x0A;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            on_ground: reader.read_bool()?
        })
    }

    fn handle(self, ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        let position = ctx.player.position().with_ground(self.on_ground);

        ctx.player.set_position(position);

        Ok(ConnectionAction::Continue)
    }
}


pub struct PlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
    pub on_ground: bool
}

impl ServerboundPacket for PlayerPositionPacket {
    const ID: u8 = 0x0B;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            x: reader.read_f64()?,
            y: reader.read_f64()?,
            stance: reader.read_f64()?,
            z: reader.read_f64()?,
            on_ground: reader.read_bool()?
        })
    }

    fn handle(self, ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        let position = ctx.player.position().with_ground(self.on_ground);

        ctx.update_position(Position {
            x: self.x,
            y: self.y,
            z: self.z,
            ..position
        });

        Ok(ConnectionAction::Continue)
    }
}


pub struct PlayerLookPacket {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool
}

impl ServerboundPacket for PlayerLookPacket {
    const ID: u8 = 0x0C;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            yaw: reader.read_f32()?,
            pitch: reader.read_f32()?,
            on_ground: reader.read_bool()?
        })
    }

    fn handle(self, ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        let position = ctx.player.position();

        ctx.update_position(position.with_look(self.yaw, self.pitch).with_ground(self.on_ground));

        Ok(ConnectionAction::Continue)
    }
}


pub struct PlayerPosLookPacket {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool
}

impl ServerboundPacket for PlayerPosLookPacket {
    const ID: u8 = 0x0D;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            x: reader.read_f64()?,
            y: reader.read_f64()?,
            stance: reader.read_f64()?,
            z: reader.read_f64()?,
            yaw: reader.read_f32()?,
            pitch: reader.read_f32()?,
            on_ground: reader.read_bool()?
        })
    }

    fn handle(self, ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        ctx.update_position(Position {
            x: self.x,
            y: self.y,
            z: self.z,
            yaw: self.yaw,
            pitch: self.pitch,
            on_ground: self.on_ground
        });

        Ok(ConnectionAction::Continue)
    }
}


pub struct ChatMessagePacket {
    pub message: String
}

impl ServerboundPacket for ChatMessagePacket {
    const ID: u8 = 0x03;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            message: reader.read_string()?
        })
    }

    fn handle(self, ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        if let Some(command) = self.message.strip_prefix('/') {
            println!("{} issued command: {command}", ctx.player.username);

            return Ok(ConnectionAction::Continue);
        }

        println!("<{}> {}", ctx.player.username, self.message);
        ctx.players.broadcast(|w| clientbound::write_chatmsg(w, &format!("<{}> {}", ctx.player.username, self.message)));

        Ok(ConnectionAction::Continue)
    }
}


pub struct ArmAnimationPacket {
    pub entity_id: i32,
    pub animate: u8
}

impl ServerboundPacket for ArmAnimationPacket {
    const ID: u8 = 0x12;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            entity_id: reader.read_i32()?,
            animate: reader.read_u8()?
        })
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct CloseWindowPacket {
    pub window_id: u8
}

impl ServerboundPacket for CloseWindowPacket {
    const ID: u8 = 0x65;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            window_id: reader.read_u8()?
        })
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct PlayerDiggingPacket {
    pub status: u8,
    pub x: i32,
    pub y: u8,
    pub z: i32,
    pub face: u8
}

impl ServerboundPacket for PlayerDiggingPacket {
    const ID: u8 = 0x0E;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            status: reader.read_u8()?,
            x: reader.read_i32()?,
            y: reader.read_u8()?,
            z: reader.read_i32()?,
            face: reader.read_u8()?
        })
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct BlockPlacementPacket {
    pub x: i32,
    pub y: u8,
    pub z: i32,
    pub direction: u8,
    pub item: Option<ItemStack>
}

impl ServerboundPacket for BlockPlacementPacket {
    const ID: u8 = 0x0F;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        let x = reader.read_i32()?;
        let y = reader.read_u8()?;
        let z = reader.read_i32()?;
        let direction = reader.read_u8()?;
        let id = reader.read_i16()?;

        let item = if id >= 0 {
            let count = reader.read_u8()?;
            let damage = reader.read_i16()?;

            Some(ItemStack { id, count, damage })
        } else {
            None
        };

        Ok(Self { x, y, z, direction, item })
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct EntityActionPacket {
    pub entity_id: i32,
    pub action: u8
}

impl ServerboundPacket for EntityActionPacket {
    const ID: u8 = 0x13;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            entity_id: reader.read_i32()?,
            action: reader.read_u8()?
        })
    }

    fn handle(self, ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        match self.action {
            1 => { // yes
                ctx.players.broadcast_except(ctx.player.id(), |w| {
                    clientbound::write_player_crouch_lol(w, ctx.player.id(), true)
                });
            }

            2 => { // no
                ctx.players.broadcast_except(ctx.player.id(), |w| {
                    clientbound::write_player_crouch_lol(w, ctx.player.id(), false)
                });
            }

            _ => {}
        }

        Ok(ConnectionAction::Continue)
    }
}


pub struct UseEntityPacket {
    pub user_id: i32,
    pub target_id: i32,
    pub left_click: bool
}

impl ServerboundPacket for UseEntityPacket {
    const ID: u8 = 0x07;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            user_id: reader.read_i32()?,
            target_id: reader.read_i32()?,
            left_click: reader.read_bool()?
        })
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct HeldItemChangePacket {
    pub slot: i16
}

impl ServerboundPacket for HeldItemChangePacket {
    const ID: u8 = 0x10;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            slot: reader.read_i16()?
        })
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct WindowClickPacket {
    pub window_id: u8,
    pub slot: i16,
    pub rightclick: u8,
    pub action: i16,
    pub shift: bool,
    pub item: Option<ItemStack>
}

impl ServerboundPacket for WindowClickPacket {
    const ID: u8 = 0x66;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        let window_id = reader.read_u8()?;
        let slot = reader.read_i16()?;
        let click = reader.read_u8()?;
        let act = reader.read_i16()?;
        let shift = reader.read_bool()?;
        let id = reader.read_i16()?;

        let item = if id != -1 {
            let count = reader.read_u8()?;
            let damage = reader.read_i16()?;

            Some(ItemStack { id, count, damage })
        } else {
            None
        };

        Ok(Self { window_id, slot, rightclick: click, action: act, shift, item })
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct TransactionPacket {
    pub window_id: u8,
    pub action: i16,
    pub accepted: bool
}

impl ServerboundPacket for TransactionPacket {
    const ID: u8 = 0x6A;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            window_id: reader.read_u8()?,
            action: reader.read_i16()?,
            accepted: reader.read_bool()?
        })
    }

    fn handle(self, _ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        Ok(ConnectionAction::Continue) // TODO
    }
}


pub struct DisconnectPacket {
    pub reason: String
}

impl ServerboundPacket for DisconnectPacket {
    const ID: u8 = 0xFF;

    fn read(reader: &mut PacketReader<&mut TcpStream>) -> Result<Self> {
        Ok(Self {
            reason: reader.read_string()?
        })
    }

    fn handle(self, ctx: &mut ConnectionContext) -> Result<ConnectionAction> {
        println!("{} disconnected: {}", ctx.player.username, self.reason);

        Ok(ConnectionAction::Disconnect)
    }
}