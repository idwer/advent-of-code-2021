use std::fs;

pub fn solve_part_2(filename: String) -> i64 {
    let mut increased = 0;
    let mut prev = 0;

    let input = fs::read_to_string(filename).expect("error opening file");

    let mut numbers: Vec<i32> = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<i32>().unwrap());
    }

    for window in numbers.windows(3) {
        let cur = window[0] + window[1] + window[2];

        if prev == 0 {
            prev = cur;
            continue;
        }

        if prev < cur {
            increased += 1;
        }

        prev = cur;
    }

    increased
}

#[cfg(test)]
mod tests_p01p2 {
    use super::*;

    #[test]
    fn test_p01p2() {
        assert_eq!(solve_part_2("input".to_string()), 1781);
        assert_eq!(solve_part_2("test_input".to_string()), 5);
    }
}