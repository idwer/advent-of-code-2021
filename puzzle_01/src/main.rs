use std::fs;
use std::path::Path;

fn solve_puzzle_01_input() -> i64 {
    let input = fs::read_to_string(Path::new("input"));

    0
}

fn solve_puzzle_01_test_input() -> i64 {
    let input = fs::read_to_string(Path::new("test_input"));

    0
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