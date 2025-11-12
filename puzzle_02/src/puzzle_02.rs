use crate::data::Direction;
use crate::data::Instruction;

fn parse_instruction(instruction: &str) -> Instruction {
    let (direction, steps) = instruction.split_once(' ').unwrap();

    let direction = match direction {
        "forward" => Direction::Forward,
        "down" => Direction::Down,
        "up" => Direction::Up,
        unhandled => panic!("unhandled direction: {}", unhandled),
    };

    let steps = steps.parse::<u32>().unwrap();

    Instruction { direction, steps }
}

fn solution(instructions: &Vec<Instruction>, part_one: bool) -> u32 {
    let mut aim = 0;
    let mut pos_vertical = 0;
    let mut pos_horizontal = 0;

    if part_one {
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

pub fn solve_part_1() -> u32 {
    let instructions: Vec<_> = include_str!("../input")
    .lines()
    .filter(|l| !l.is_empty())
    .map(parse_instruction)
    .collect();

    solution(&instructions, true)
}

pub fn solve_part_1_sample() -> u32 {
    let instructions: Vec<_> = include_str!("../test_input")
    .lines()
    .filter(|l| !l.is_empty())
    .map(parse_instruction)
    .collect();

    solution(&instructions, true)
}

pub fn solve_part_2() -> u32 {
    let instructions: Vec<_> = include_str!("../input")
    .lines()
    .filter(|l| !l.is_empty())
    .map(parse_instruction)
    .collect();

    solution(&instructions, false)
}

pub fn solve_part_2_sample() -> u32 {
    let instructions: Vec<_> = include_str!("../test_input")
    .lines()
    .filter(|l| !l.is_empty())
    .map(parse_instruction)
    .collect();

    solution(&instructions, false)
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

