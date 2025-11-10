pub fn solve_part_1(filename: String) -> u64 {
    let mut increased = 0;

    let input = std::fs::read_to_string(filename).expect("error opening file");

    let mut numbers: Vec<u32> = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<u32>().unwrap());
    }

    for window in numbers.windows(2) {
        if window[0] < window[1] {
            increased += 1;
        }
    }

    increased
}


pub fn solve_part_2(filename: String) -> u64 {
    let mut increased = 0;
    let mut prev = 0;

    let input = std::fs::read_to_string(filename).expect("error opening file");

    let mut numbers: Vec<u32> = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<u32>().unwrap());
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
mod tests_p01p1 {
    use super::*;

    #[test]
    fn test_p01p1() {
        assert_eq!(solve_part_1("input".to_string()), 1752);
        assert_eq!(solve_part_1("test_input".to_string()), 7);
    }

    #[test]
    fn test_p01p2() {
        assert_eq!(solve_part_2("input".to_string()), 1781);
        assert_eq!(solve_part_2("test_input".to_string()), 5);
    }
}
