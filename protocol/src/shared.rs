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