use std::io::{Read, Result};

use world::ItemStack;

use crate::{PacketReader, codec::{Decode, Packet}};

#[derive(Debug, Clone)]
pub struct PlayerDigging {
    pub status: u8,
    pub x: i32,
    pub y: u8,
    pub z: i32,
    pub face: u8
}

impl Packet for PlayerDigging {
    const ID: u8 = 0x0E;
}

impl Decode for PlayerDigging {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            status: r.read_u8()?,
            x: r.read_i32()?,
            y: r.read_u8()?,
            z: r.read_i32()?,
            face: r.read_u8()?
        })
    }
}


#[derive(Debug, Clone)]
pub struct BlockPlacement {
    pub x: i32,
    pub y: u8,
    pub z: i32,
    pub direction: u8,
    pub item: Option<ItemStack>
}

impl Packet for BlockPlacement {
    const ID: u8 = 0x0F;
}

impl Decode for BlockPlacement {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            x: r.read_i32()?,
            y: r.read_u8()?,
            z: r.read_i32()?,
            direction: r.read_u8()?,
            item: r.read_itemstack()?
        })
    }
}


#[derive(Debug, Clone)]
pub struct EntityAction {
    pub id: i32,
    pub action: u8
}

impl Packet for EntityAction {
    const ID: u8 = 0x13;
}

impl Decode for EntityAction {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            id: r.read_i32()?,
            action: r.read_u8()?
        })
    }
}


#[derive(Debug, Clone)]
pub struct UseEntity {
    pub user_id: i32,
    pub target_id: i32,
    pub left_click: bool
}

impl Packet for UseEntity {
    const ID: u8 = 0x07;
}

impl Decode for UseEntity {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            user_id: r.read_i32()?,
            target_id: r.read_i32()?,
            left_click: r.read_bool()?
        })
    }
}


#[derive(Debug, Clone)]
pub struct HeldItemChange {
    pub slot: i16
}

impl Packet for HeldItemChange {
    const ID: u8 = 0x10;
}

impl Decode for HeldItemChange {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self { slot: r.read_i16()? })
    }
}