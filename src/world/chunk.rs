/* NOTE(Able): Each string id will be converted into a lisr of values and matched to that string id
these should be at the top of the save file so that the game won't mix up newly added/changed string ids
 EXAMPLE(Able): 0 <-> Air, 1 <-> Stone
*/
struct Block {
    id: String,
}
pub struct Chunk {
    pub data: [[[i32; 32]; 32]; 32],
}

impl Chunk {
    pub fn fetch(self, x: usize, y: usize, z: usize) {
        println!("{}", self.data[x][y][z]);
    }
}