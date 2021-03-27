// Signed because stats can be negative
#[derive(Debug)]
pub struct Stats {
    pub speed: i8,
    pub strength: i8,
}
#[derive(Debug)]
pub struct Player {
    pub x_rot: f32,
    pub stats: Stats,
}

impl Player {
    fn walk(self) {}
}
