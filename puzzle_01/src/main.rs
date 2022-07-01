use std::env;

mod puzzle_01;
mod puzzle_02;

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename_part_1 = &args[1];
    let filename_part_2 = &args[2];

    let result_part_1 = puzzle_01::solve_puzzle_01(filename_part_1.to_string());
    let result_part_2 = puzzle_02::solve_puzzle_02(filename_part_2.to_string());

    println!("Solution for part 1: {}", result_part_1);
    println!("Solution for part 2: {}", result_part_2);
}