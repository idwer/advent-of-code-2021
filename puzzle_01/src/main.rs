use std::fs;
use std::path::Path;

fn solve_puzzle_01_input() -> i64 {
    let input = fs::read_to_string(Path::new("input"));

    0
}

fn solve_puzzle_01_test_input() -> i64 {
    let mut increased = 0;

    let input = fs::read_to_string("test_input").expect("error opening file");

    let mut numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    for (pos, e) in numbers.iter().enumerate() {
        if pos > 0 && numbers[pos - 1] < numbers[pos] {
                increased += 1;
        }
    }

    increased
}

fn solve_puzzle_02_input() -> i64 {
    let input = fs::read_to_string(Path::new("input"));

    0
}

fn solve_puzzle_02_test_input() -> i64 {
    let input = fs::read_to_string(Path::new("test_input"));

    0
}

fn main() {
    let mut result = solve_puzzle_01_input();

    assert_eq!(result, 1752);
    println!("Solution for puzzle 01: {}", result);

    result = solve_puzzle_01_test_input();

    assert_eq!(result, 7);
    println!("Solution for puzzle 01, test input: {}", result);

    result = solve_puzzle_02_input();

    assert_eq!(result, 1781);
    println!("Solution for puzzle 02: {}", result);

    result = solve_puzzle_02_test_input();

    assert_eq!(result, 5);
    println!("Solution for puzzle 02, test input: {}", result);
}