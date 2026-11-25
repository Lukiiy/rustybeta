use std::io::{Read, Result};

use world::ItemStack;

use crate::{PacketReader, codec::{Decode, Packet}};

#[derive(Debug, Clone)]
pub struct WindowClick {
    pub id: u8,
    pub slot: i16,
    pub right_click: u8,
    pub action: i16,
    pub shift: bool,
    pub item: Option<ItemStack>
}

impl Packet for WindowClick {
    const ID: u8 = 0x66;
}

impl Decode for WindowClick {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            id: r.read_u8()?,
            slot: r.read_i16()?,
            right_click: r.read_u8()?,
            action: r.read_i16()?,
            shift: r.read_bool()?,
            item: r.read_itemstack()?
        })
    }
}