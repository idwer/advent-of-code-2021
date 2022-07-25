// for future use

enum Direction {
    Forward,
    Down,
    Up,
}

struct Instruction {
    pub direction: Direction,
    pub steps: i32,
}