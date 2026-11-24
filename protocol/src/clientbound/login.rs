use std::io::{Result, Write};

use crate::PacketWriter;
use crate::codec::{Encode, Packet};

#[derive(Debug, Clone)]
pub struct Handshake;

impl Packet for Handshake {
    const ID: u8 = 0x02;
}

impl Encode for Handshake {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> { w.write_string("-") }
}


#[derive(Debug, Clone)]
pub struct Login {
    pub id: i32,
    pub seed: i64,
    pub dimension: i8
}

impl Packet for Login {
    const ID: u8 = 0x01;
}

impl Encode for Login {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        w.write_i32(self.id)?; // entity id
        w.write_string("")?;
        w.write_i64(self.seed)?;
        w.write_u8(self.dimension as u8)
    }
}