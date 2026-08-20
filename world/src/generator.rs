use crate::chunk::*;
use crate::chunk::Chunk;

pub trait Generator: Send + Sync {
    fn generate(&self, chunk: &mut Chunk);
}

/// Flat world, single block type up to `height`.
pub struct FlatGenerator {
    pub height: usize,
    pub block: u8
}

impl Generator for FlatGenerator {
    fn generate(&self, chunk: &mut Chunk) {
        for x in 0..SIZE_X {
            for z in 0..SIZE_Z {
                for y in 0..self.height {
                    chunk.set_block(x, y, z, self.block);
                }
            }
        }
    }
}