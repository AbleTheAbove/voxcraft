/* NOTE(Able): Each string id will be converted into a lisr of values and matched to that string id
these should be at the top of the save file so that the game won't mix up newly added/changed string ids
 EXAMPLE(Able): 0 <-> Air, 1 <-> Stone
*/
pub const CHUNKSIZE: usize = 16;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up, // NOTE(Able): +Y The default direction ^-^
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub id: u32,
    pub facing: Direction,
}
pub type ChunkData = [[[Block; CHUNKSIZE]; CHUNKSIZE]; CHUNKSIZE];

#[derive(Debug, Clone, Copy)]
pub struct ChunkOffset {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Chunk {
    pub location: ChunkOffset,
    pub data: ChunkData,
}
