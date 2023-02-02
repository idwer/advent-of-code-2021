mod solution;
mod board;

fn main() {
    let result_part_1_sample = solution::solve_puzzle_04_sample(false);
    // let result_part_1 = solution::solve_puzzle_04(false);

    // let result_part_2_sample = solution::solve_puzzle_04_sample(true);
    // let result_part_2 = solution::solve_puzzle_04(true);

    // println!("Solution for part 1: {}", result_part_1);
    println!("Solution for part 1: {} - using test_input", result_part_1_sample);

    // println!("Solution for part 2: {}", result_part_2);
    // println!("Solution for part 2: {} - using test_input", result_part_2_sample);


  // google: rust range steps
  // so: 27893223


    // for n in 0..5_u8.pow(2) {
    for n in (0.. 5_u8.pow(2) + 1).step_by(5) {

        // println!("{}", n);
    }

//    for n in (0..778).step_by(7) {
    for n in (0..21).step_by(5) {
    	// println!("{}", n);
    }
}
