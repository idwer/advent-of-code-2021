// todo: use enum and struct found in data.rs

enum Direction {
    Forward,
    Down,
    Up,
}

struct Instruction {
    pub direction: Direction,
    pub steps: i32,
}

fn parse_instruction(instruction: &str) -> Instruction {
    let (direction, steps) = instruction.split_once(' ').unwrap();

    let direction = match direction {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        unhandled => panic!("unhandled direction: {}", unhandled),
    };

    let steps = steps.parse::<i32>().unwrap();

    Instruction { direction, steps }
}

fn solution(instructions: &Vec<Instruction>, part_two: bool) -> i32 {
    let mut aim = 0;
    let mut pos_vertical = 0;
    let mut pos_horizontal = 0;

    if !part_two {
        for Instruction { direction, steps } in instructions {
            match direction {
                Direction::Forward => pos_horizontal += steps,
                Direction::Down => pos_vertical += steps,
                Direction::Up => pos_vertical -= steps,
            }
        }
    } else {
        for Instruction { direction, steps } in instructions {
            match direction {
                Direction::Forward => {
                    pos_vertical += aim * steps;
                    pos_horizontal += steps;
                }
                Direction::Down => aim += steps,
                Direction::Up => aim -= steps,
            }
        }
    }

    pos_horizontal * pos_vertical
}

pub fn solve_part_1() -> i32 {
    let instructions: Vec<_> = include_str!("../input")
    .lines()
    .filter(|l| !l.is_empty())
    .map(parse_instruction)
    .collect();

    solution(&instructions, false)
}

pub fn solve_part_1_sample() -> i32 {
    let instructions: Vec<_> = include_str!("../test_input")
    .lines()
    .filter(|l| !l.is_empty())
    .map(parse_instruction)
    .collect();

    solution(&instructions, false)
}

pub fn solve_part_2() -> i32 {
    let instructions: Vec<_> = include_str!("../input")
    .lines()
    .filter(|l| !l.is_empty())
    .map(parse_instruction)
    .collect();

    solution(&instructions, true)
}

pub fn solve_part_2_sample() -> i32 {
    let instructions: Vec<_> = include_str!("../test_input")
    .lines()
    .filter(|l| !l.is_empty())
    .map(parse_instruction)
    .collect();

    solution(&instructions, true)
}

#[cfg(test)]
mod tests_p02 {
    use super::*;

    #[test]
    fn test_p02p1() {
        assert_eq!(solve_part_1(), 1480518);
    }
    #[test]
    fn test_p02p1_sample() {
        assert_eq!(solve_part_1_sample(), 150);
    }

    #[test]
    fn test_p02p2() {
        assert_eq!(solve_part_2(), 1282809906);
    }

    #[test]
    fn test_p02p2_sample() {
        assert_eq!(solve_part_2_sample(), 900);
    }
}

