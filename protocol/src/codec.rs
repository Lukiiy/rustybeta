use std::io::{Read, Result, Write};

use crate::{PacketReader, PacketWriter};

pub trait Packet {
    const ID: u8;
}

pub trait Encode {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()>;
}

pub trait Decode: Sized {
    fn decode<R: Read>(r: &mut PacketReader<R>) -> Result<Self>;
}