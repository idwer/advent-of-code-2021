mod solution;

fn main() {
    let result_part_1_sample = solution::solve_puzzle_06_sample(80);
    println!(
        "Solution for part 1: {} - using test_input\n",
        result_part_1_sample
    );

    let result_part_1 = solution::solve_puzzle_06(80);
    println!(
        "Solution for part 1: {} - using input\n",
        result_part_1
    );

    let result_part_2_sample = solution::solve_puzzle_06_sample(256);
    println!(
        "Solution for part 2: {} - using test_input\n",
        result_part_2_sample
    );

    let result_part_2 = solution::solve_puzzle_06(256);
    println!(
        "Solution for part 2: {} - using input\n",
        result_part_2
    );
}
