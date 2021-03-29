use crate::world::directions::Direction;

/* NOTE(Able): Each string id will be converted into a lisr of values and matched to that string id
these should be at the top of the save file so that the game won't mifacing up newly added/changed string ids
 EfacingAMPLE(Able): 0 <-> Air, 1 <-> Stone
*/
#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub id: u32,
    pub facing: Direction,
}

impl Block {
    pub fn rotate_facing(mut self) {
        let mut facing: Direction;
        match self.facing {
            Direction::Up => {
                facing = Direction::Down;
            }
            Direction::Down => {
                facing = Direction::North;
            }
            Direction::North => {
                facing = Direction::East;
            }
            Direction::East => {
                facing = Direction::South;
            }
            Direction::South => {
                facing = Direction::West;
            }
            Direction::West => {
                facing = Direction::Up;
            }
        }
        println!("{:?}", facing);
        self.facing = facing
    }
    pub fn counter_rotate_facing(mut self) {
        let facing: Direction;
        match self.facing {
            Direction::Up => {
                facing = Direction::West;
            }
            Direction::Down => {
                facing = Direction::Up;
            }
            Direction::North => {
                facing = Direction::Down;
            }
            Direction::East => {
                facing = Direction::North;
            }
            Direction::South => {
                facing = Direction::East;
            }
            Direction::West => {
                facing = Direction::South;
            }
        }
        self.facing = facing;
    }
}
