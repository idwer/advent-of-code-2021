mod board;
mod solution;

fn main() {
    let result_part_1_sample_smnw = solution::solve_puzzle_04_sample(false);
    let result_part_1_sample_smw = solution::solve_puzzle_04_sample(true);
    // let result_part_1 = solution::solve_puzzle_04(false);

    // let result_part_2_sample = solution::solve_puzzle_04_sample(true);
    // let result_part_2 = solution::solve_puzzle_04(true);

    // println!("Solution for part 1: {}", result_part_1);
    println!(
        "Solution for part 1: {} - using test_input",
        result_part_1_sample_smnw
    );
    println!(
        "Solution for part 1: {} - using test_input",
        result_part_1_sample_smw
    );

    // println!("Solution for part 2: {}", result_part_2);
    // println!("Solution for part 2: {} - using test_input", result_part_2_sample);
}
