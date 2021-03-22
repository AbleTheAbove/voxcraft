pub struct Chunk {
    data: [[[String; 32]; 32]; 32],
}

impl Chunk {
    pub fn fetch(self, x: usize, y: usize, z: usize) {
        println!("{}", self.data[x][y][z]);
    }
}
