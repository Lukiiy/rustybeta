use std::io::{Result, Write};

use utils::math::{angle_byte, fixed_point};
use world::Position;

use crate::PacketWriter;
use crate::codec::{Encode, Packet};

#[derive(Debug, Clone)]
pub struct PlayerPositionLook {
    pub position: Position,
    pub stance: f64
}

impl PlayerPositionLook {
    pub fn at(position: Position) -> Self {
        Self { stance: position.y + 1.62, position }
    }
}

impl Packet for PlayerPositionLook {
    const ID: u8 = 0x0D;
}

impl Encode for PlayerPositionLook {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        let p = &self.position;

        w.write_f64(p.x)?;
        w.write_f64(self.stance)?;
        w.write_f64(p.y)?;
        w.write_f64(p.z)?;
        w.write_f32(p.yaw)?;
        w.write_f32(p.pitch)?;
        w.write_bool(p.on_ground)
    }
}


#[derive(Debug, Clone)]
pub struct SpawnPlayer {
    pub id: i32,
    pub username: String,
    pub position: Position,
    pub current_item: i16
}

impl Packet for SpawnPlayer {
    const ID: u8 = 0x14;
}

impl Encode for SpawnPlayer {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        let p = &self.position;

        w.write_i32(self.id)?;
        w.write_string(&self.username)?;
        w.write_i32(fixed_point(p.x))?;
        w.write_i32(fixed_point(p.y))?;
        w.write_i32(fixed_point(p.z))?;
        w.write_u8(angle_byte(p.yaw))?;
        w.write_u8(angle_byte(p.pitch))?;
        w.write_i16(self.current_item)
    }
}


#[derive(Debug, Clone)]
pub struct DestroyEntity {
    pub id: i32
}

impl Packet for DestroyEntity {
    const ID: u8 = 0x1D;
}

impl Encode for DestroyEntity {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> { w.write_i32(self.id) }
}


#[derive(Debug, Clone)]
pub struct EntityTeleport {
    pub id: i32,
    pub position: Position
}

impl Packet for EntityTeleport {
    const ID: u8 = 0x22;
}

impl Encode for EntityTeleport {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        let pos = &self.position;

        w.write_i32(self.id)?;
        w.write_i32(fixed_point(pos.x))?;
        w.write_i32(fixed_point(pos.y))?;
        w.write_i32(fixed_point(pos.z))?;
        w.write_u8(angle_byte(pos.yaw))?;
        w.write_u8(angle_byte(pos.pitch))
    }
}


#[derive(Debug, Clone)]
pub struct EntityCrouch {
    pub id: i32,
    pub sneaking: bool
}

impl Packet for EntityCrouch {
    const ID: u8 = 0x28;
}

impl Encode for EntityCrouch {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        w.write_i32(self.id)?;
        w.write_u8(0x00)?; // metadata idx 0
        w.write_u8(if self.sneaking { 0x02 } else { 0x00 })?;
        w.write_u8(0x7F) // end
    }
}


#[derive(Debug, Clone)]
pub struct EntityMove {
    pub id: i32,
    pub dx: i8,
    pub dy: i8,
    pub dz: i8,
    pub yaw: u8,
    pub pitch: u8
}

impl Packet for EntityMove {
    const ID: u8 = 0x21;
}

impl Encode for EntityMove {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        w.write_i32(self.id)?;
        w.write_i8(self.dx)?;
        w.write_i8(self.dy)?;
        w.write_i8(self.dz)?;
        w.write_u8(self.yaw)?;
        w.write_u8(self.pitch)
    }
}