mod solution;

fn main() {
    println!("Hello, world!");

    let result_part_1_sample = solution::solve_puzzle_07_sample(Vec::from([1, 2, 3, 10]));
    println!(
        "Solution for part 1: {} - using test_input\n",
        result_part_1_sample
    );

    let result_part_1 = solution::solve_puzzle_07(Vec::from([1, 2, 3, 10]));
    println!(
        "Solution for part 1: {} - using input\n",
        result_part_1
    );

    let result_part_2_sample = solution::solve_puzzle_07_sample(Vec::from([1, 2, 3, 10]));
    println!(
        "Solution for part 2: {} - using test_input\n",
        result_part_2_sample
    );

    let result_part_2 = solution::solve_puzzle_07(Vec::from([1, 2, 3, 10]));
    println!(
        "Solution for part 2: {} - using input\n",
        result_part_2
    );
}
