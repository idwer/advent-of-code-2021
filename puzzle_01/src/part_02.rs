use std::fs;

pub fn solve_part_2(filename: String) -> i64 {
    let mut increased = 0;
    let input = fs::read_to_string(filename).expect("error opening file");

    let mut numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    for (pos, _) in numbers.iter().enumerate() {
        if pos + 2 < numbers.len() {
            let first = numbers[pos] + numbers[pos + 1] + numbers[pos + 2];
            let mut second = numbers[pos + 1] + numbers[pos + 1 + 1];

            if pos + 3 < numbers.len() {
                second += numbers[pos + 1 + 2];
            }
            if first < second {
                increased += 1;
            }
        }
    }

    increased
}