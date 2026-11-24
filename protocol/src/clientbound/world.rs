use std::io::{Result, Write};

use world::{Chunk, SIZE_X, SIZE_Y, SIZE_Z};

use crate::PacketWriter;
use crate::codec::{Encode, Packet};

#[derive(Debug, Clone)]
pub struct SetSpawnPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Packet for SetSpawnPosition {
    const ID: u8 = 0x06;
}

impl Encode for SetSpawnPosition {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        w.write_i32(self.x)?;
        w.write_i32(self.y)?;
        w.write_i32(self.z)
    }
}


#[derive(Debug, Clone)]
pub struct PreChunk {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub mode: bool
}

impl Packet for PreChunk {
    const ID: u8 = 0x32;
}

impl Encode for PreChunk {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        w.write_i32(self.chunk_x)?;
        w.write_i32(self.chunk_z)?;
        w.write_bool(self.mode)
    }
}


#[derive(Debug, Clone)]
pub struct ChunkData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub data: Vec<u8>
}

impl ChunkData {
    pub fn from_chunk(chunk: &Chunk) -> Result<Self> {
        Ok(Self { chunk_x: chunk.x, chunk_z: chunk.z, data: chunk.compressed()? })
    }
}

impl Packet for ChunkData {
    const ID: u8 = 0x33;
}

impl Encode for ChunkData {
    fn encode<W: Write>(&self, w: &mut PacketWriter<W>) -> Result<()> {
        w.write_i32(self.chunk_x * 16)?;
        w.write_i16(0)?;
        w.write_i32(self.chunk_z * 16)?;
        w.write_u8((SIZE_X - 1) as u8)?;
        w.write_u8((SIZE_Y - 1) as u8)?;
        w.write_u8((SIZE_Z - 1) as u8)?;
        w.write_i32(self.data.len() as i32)?;
        w.write_bytes(&self.data)
    }
}