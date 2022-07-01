use std::fs;

pub fn solve_part_2(filename: String) -> i64 {
    let mut increased = 0;
    let mut index = 0;

    let input = fs::read_to_string(filename).expect("error opening file");

    let mut numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    for (pos, _) in numbers.iter().enumerate() {
        let sum_window_first = &numbers[index..index + 3];

        index += 1;
        let sum_window_second = &numbers[index..index + 3];

        if sum_window_first < sum_window_second {
            increased += 1;
        }

    }

    increased
}