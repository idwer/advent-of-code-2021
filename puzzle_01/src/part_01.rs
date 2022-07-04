use std::fs;

pub fn solve_part_1(filename: String) -> i64 {
    let mut increased = 0;

    let input = fs::read_to_string(filename).expect("error opening file");

    let mut numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    for window in numbers.windows(2) {
        if window[0] < window[1] {
            increased += 1;
        }
    }

    increased
}