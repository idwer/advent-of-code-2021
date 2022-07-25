mod part_01;

fn main() {
    let result_part_1 = part_01::solve_part_1();

    let result_part_1_sample = part_01::solve_part_1_sample();

    println!("Solution for part 1: {}", result_part_1);

    println!("Solution for part 1: {} - using test_input", result_part_1_sample);
 }