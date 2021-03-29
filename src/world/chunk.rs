use crate::Block;

pub const CHUNKSIZE: usize = 16;

pub type ChunkData = [[[Block; CHUNKSIZE]; CHUNKSIZE]; CHUNKSIZE];

#[derive(Debug, Clone, Copy)]
pub struct ChunkOffset {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Chunk {
    pub location: ChunkOffset,
    pub data: ChunkData,
}
