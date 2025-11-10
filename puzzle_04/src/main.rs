mod board;
mod puzzle_04;

fn main() {
    let result_part_1_sample = puzzle_04::solve_puzzle_04_sample(false);
    println!(
        "Solution for part 1: {} - using test_input",
        result_part_1_sample
    );

    let result_part_1 = puzzle_04::solve_puzzle_04(false);
    println!(
        "Solution for part 1: {} - using input",
        result_part_1
    );

    let result_part_2_sample = puzzle_04::solve_puzzle_04_sample(true);
    println!(
        "Solution for part 2: {} - using test_input (squid_must_win)",
        result_part_2_sample
    );

    let result_part_2 = puzzle_04::solve_puzzle_04(true);
    println!(
        "Solution for part 2: {} - using input (squid_must_win)",
        result_part_2
    );
}
