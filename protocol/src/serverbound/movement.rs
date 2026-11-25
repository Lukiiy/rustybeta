use std::io::{Read, Result};

use crate::{PacketReader, codec::{Decode, Packet}};

#[derive(Debug, Clone)]
pub struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
    pub on_ground: bool
}

impl Packet for PlayerPosition {
    const ID: u8 = 0x0B;
}

impl Decode for PlayerPosition {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            x: r.read_f64()?,
            y: r.read_f64()?,
            stance: r.read_f64()?,
            z: r.read_f64()?,
            on_ground: r.read_bool()?
        })
    }
}


#[derive(Debug, Clone)]
pub struct PlayerFlying {
    pub on_ground: bool
}

impl Packet for PlayerFlying {
    const ID: u8 = 0x0A;
}

impl Decode for PlayerFlying {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self { on_ground: r.read_bool()? })
    }
}


#[derive(Debug, Clone)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool
}

impl Packet for PlayerLook {
    const ID: u8 = 0x0C;
}

impl Decode for PlayerLook {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            yaw: r.read_f32()?,
            pitch: r.read_f32()?,
            on_ground: r.read_bool()?
        })
    }
}


#[derive(Debug, Clone)]
pub struct PlayerPositionLook {
    pub x: f64,
    pub y: f64,
    pub stance: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool
}

impl Packet for PlayerPositionLook {
    const ID: u8 = 0x0D;
}

impl Decode for PlayerPositionLook {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            x: r.read_f64()?,
            y: r.read_f64()?,
            stance: r.read_f64()?,
            z: r.read_f64()?,
            yaw: r.read_f32()?,
            pitch: r.read_f32()?,
            on_ground: r.read_bool()?
        })
    }
}