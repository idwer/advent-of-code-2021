use std::env;

mod part_01;
mod part_02;

fn main() {
    let args:Vec<String> = env::args().collect();
    let filename = &args[1];

    let result_part_1 = part_01::solve_part_1(filename.to_string());
    let result_part_2 = part_02::solve_part_2(filename.to_string());

    println!("Solution for part 1: {}", result_part_1);
    println!("Solution for part 2: {}", result_part_2);
}