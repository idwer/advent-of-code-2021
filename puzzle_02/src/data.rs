pub enum Direction {
    Forward,
    Down,
    Up,
}

pub struct Instruction {
    pub direction: Direction,
    pub steps: u32,
}
