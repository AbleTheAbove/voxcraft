#[derive(Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up, // NOTE(Able): +Y The default direction ^-^
    Down,
}
