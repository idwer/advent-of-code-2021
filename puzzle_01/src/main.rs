mod part_01;
mod part_02;

fn main() {
    let result_part_1 = part_01::solve_part_1("input".to_string());
    let result_part_2 = part_02::solve_part_2("input".to_string());

    let result_part_1_sample = part_01::solve_part_1("test_input".to_string());
    let result_part_2_sample = part_02::solve_part_2("test_input".to_string());

    println!("Solution for part 1: {}", result_part_1);
    println!("Solution for part 2: {}", result_part_2);

    println!("Solution for part 1: {} - using test_input", result_part_1_sample);
    println!("Solution for part 2: {} - using test_input", result_part_2_sample);
}