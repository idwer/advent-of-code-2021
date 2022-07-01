use std::env;

mod puzzle_01;

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let result = puzzle_01::solve_puzzle_01(filename.to_string());

    println!("Solution for puzzle 01: {}", result);
}