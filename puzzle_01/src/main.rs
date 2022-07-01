use std::env;
use std::fs;

fn solve_puzzle_01(filename: String) -> i64 {
    let mut increased = 0;

    let input = fs::read_to_string(filename).expect("error opening file");

    let mut numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    for (pos, _) in numbers.iter().enumerate() {
        if pos > 0 && numbers[pos - 1] < numbers[pos] {
                increased += 1;
        }
    }

    increased
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = solve_puzzle_01(filename.to_string());

    println!("Solution for puzzle 01: {}", result);
}