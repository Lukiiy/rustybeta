use std::io::{Write, Result};
use flate2::{write::ZlibEncoder, Compression};

pub const SIZE_X: usize = 16;
pub const SIZE_Y: usize = 128;
pub const SIZE_Z: usize = 16;
const BLOCKS: usize = SIZE_X * SIZE_Y * SIZE_Z;

pub struct Chunk {
    pub x: i32,
    pub z: i32,

    types: Box<[u8; BLOCKS]>,
    metadata: Box<[u8; BLOCKS / 2]>,
    block_light: Box<[u8; BLOCKS / 2]>,
    sky_light: Box<[u8; BLOCKS / 2]>
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Self {
        Self {
            x, z,
            types: Box::new([0; BLOCKS]),
            metadata: Box::new([0; BLOCKS / 2]),
            block_light: Box::new([0; BLOCKS / 2]),
            sky_light: Box::new([0xFF; BLOCKS / 2]) // TODO lightning engine?
        }
    }

    fn index(x: usize, y: usize, z: usize) -> usize {
        y + z * SIZE_Y + x * SIZE_Y * SIZE_Z
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, id: u8) {
        self.types[Self::index(x, y, z)] = id;
    }

    pub fn compressed(&self) -> Result<Vec<u8>> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

        encoder.write_all(&*self.types)?;
        encoder.write_all(&*self.metadata)?;
        encoder.write_all(&*self.block_light)?;
        encoder.write_all(&*self.sky_light)?;

        Ok(encoder.finish()?)
    }
}