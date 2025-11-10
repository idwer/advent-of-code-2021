mod data;
mod puzzle_02;

fn main() {
    let result_part_1 = puzzle_02::solve_part_1();
    let result_part_2 = puzzle_02::solve_part_2();

    let result_part_1_sample = puzzle_02::solve_part_1_sample();
    let result_part_2_sample = puzzle_02::solve_part_2_sample();

    println!("Solution for part 1: {}", result_part_1);
    println!("Solution for part 2: {}", result_part_2);

    println!("Solution for part 1: {}", result_part_1_sample);
    println!("Solution for part 2: {}", result_part_2_sample);
}
