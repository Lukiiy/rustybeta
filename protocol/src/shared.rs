use std::io::{Read, Result, Write};

use crate::{PacketReader, PacketWriter, codec::{Decode, Encode, Packet}};

#[derive(Debug, Clone)]
pub struct KeepAlive;

impl Packet for KeepAlive {
    const ID: u8 = 0x00;
}

impl Encode for KeepAlive {
    fn encode<W: Write>(&self, _: &mut PacketWriter<W>) -> Result<()> { Ok(()) }
}

impl Decode for KeepAlive {
    fn decode<R: Read>(_: &mut PacketReader<R>) -> Result<Self> { Ok(Self) }
}


#[derive(Debug, Clone)]
pub struct Chat {
    pub message: String
}

impl Packet for Chat {
    const ID: u8 = 0x03;
}

impl Encode for Chat {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        w.write_string(&self.message)
    }
}

impl Decode for Chat {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self { message: r.read_string()? })
    }
}


#[derive(Debug, Clone)]
pub struct Kick {
    pub reason: String
}

impl Packet for Kick {
    const ID: u8 = 0xFF;
}

impl Encode for Kick {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> { w.write_string(&self.reason) }
}

impl Decode for Kick {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self { reason: r.read_string()? })
    }
}


#[derive(Debug, Clone)]
pub struct Animation {
    pub id: i32,
    pub animate: u8  // 1 = arm swing
}

impl Packet for Animation {
    const ID: u8 = 0x12;
}

impl Encode for Animation {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        w.write_i32(self.id)?;
        w.write_u8(self.animate)
    }
}

impl Decode for Animation {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            id: r.read_i32()?,
            animate: r.read_u8()?
        })
    }
}


#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: u8,
    pub action: i16,
    pub accepted: bool
}

impl Packet for Transaction {
    const ID: u8 = 0x6A;
}

impl Decode for Transaction {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            id: r.read_u8()?,
            action: r.read_i16()?,
            accepted: r.read_bool()?
        })
    }
}


#[derive(Debug, Clone)]
pub struct CloseWindow {
    pub id: u8
}

impl Packet for CloseWindow {
    const ID: u8 = 0x65;
}

impl Decode for CloseWindow {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self> {
        Ok(Self {
            id: r.read_u8()?
        })
    }
}