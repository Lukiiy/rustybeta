mod chunk;
mod generator;

use std::collections::HashMap;

pub use chunk::Chunk;
pub use generator::{FlatGenerator, Generator};

pub struct World {
    chunks: HashMap<(i32, i32), Chunk>,
    generator: Box<dyn Generator>
}

impl World {
    pub fn new(generator: impl Generator + 'static) -> Self {
        Self {
            chunks: HashMap::new(),
            generator: Box::new(generator)
        }
    }

    pub fn chunk(&mut self, x: i32, z: i32) -> &Chunk {
        let generator = &self.generator;

        self.chunks.entry((x, z)).or_insert_with(|| {
            let mut chunk = Chunk::new(x, z);

            generator.generate(&mut chunk);
            chunk
        })
    }
}