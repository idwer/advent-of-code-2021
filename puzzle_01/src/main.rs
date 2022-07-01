use std::fs;
use std::path::Path;

fn solve_puzzle_01() -> i64 {
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

fn main() {
    let result = solve_puzzle_01();

    println!("Solution for puzzle 01: {}", result);
}