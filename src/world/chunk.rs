/* NOTE(Able): Each string id will be converted into a lisr of values and matched to that string id
these should be at the top of the save file so that the game won't mix up newly added/changed string ids
 EXAMPLE(Able): 0 <-> Air, 1 <-> Stone
*/
#[derive(Copy, Clone)]
pub struct Block {
    pub id: u32,
}

pub const CHUNK_SIZE: u8 = 8;

pub struct Chunk {
    pub data: [[[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
}

impl Chunk {
    pub fn fetch(self, x: usize, y: usize, z: usize) {
        self.data[x][y][z];
    }
}
