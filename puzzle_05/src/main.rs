mod solution;

fn main() {
    let result_part_1_sample = solution::solve_puzzle_05_sample(false);
    println!(
        "Solution for part 1: {} - using test_input",
        result_part_1_sample
    );

    let result_part_1 = solution::solve_puzzle_05(false);
    println!(
        "Solution for part 1: {} - using input",
        result_part_1
    );

    let result_part_2_sample = solution::solve_puzzle_05_sample(true);
    println!(
        "Solution for part 2: {} - using test_input",
        result_part_2_sample
    );

    let result_part_2 = solution::solve_puzzle_05(true);
    println!(
        "Solution for part 2: {} - using input",
        result_part_2
    );
}
