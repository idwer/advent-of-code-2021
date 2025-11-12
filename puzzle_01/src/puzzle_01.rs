fn input_to_list(input: String) -> Vec<u32> {
    let mut numbers = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<u32>().unwrap());
    }

    numbers
}

pub fn solve_part_1(filename: String) -> u64 {
    let mut number_of_larger_sums = 0;

    let input = std::fs::read_to_string(filename).expect("error opening file");

    let numbers = input_to_list(input);

    for window in numbers.windows(2) {
        if window[0] < window[1] {
            number_of_larger_sums += 1;
        }
    }

    number_of_larger_sums
}

pub fn solve_part_2(filename: String) -> u64 {
    let mut number_of_larger_sums = 0;
    let mut previous_sum_of_sliding_window = 0;

    let input = std::fs::read_to_string(filename).expect("error opening file");

    let numbers = input_to_list(input);

    for window in numbers.windows(3) {
        let current_sum_of_sliding_window = window[0..=2].iter().sum();

        if previous_sum_of_sliding_window == 0 {
            previous_sum_of_sliding_window = current_sum_of_sliding_window;

            continue;
        }

        if previous_sum_of_sliding_window < current_sum_of_sliding_window {
            number_of_larger_sums += 1;
        }

        previous_sum_of_sliding_window = current_sum_of_sliding_window;
    }

    number_of_larger_sums
}

#[cfg(test)]
mod tests_p01 {
    use super::*;

    #[test]
    fn test_input_to_list() {
        let input = std::fs::read_to_string("test_input").unwrap();
        let numbers = input_to_list(input);

        let data = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263
        ];

        assert_eq!(numbers, data);
    }

    #[test]
    fn test_p01p1_test_input() {
        assert_eq!(solve_part_1("test_input".to_string()), 7);
    }

    #[test]
    fn test_p01p1_puzzle_input() {
        assert_eq!(solve_part_1("input".to_string()), 1752);
    }

    #[test]
    fn test_p01p2_test_input() {
        assert_eq!(solve_part_2("test_input".to_string()), 5);
    }

    #[test]
    fn test_p01p2_puzzle_input() {
        assert_eq!(solve_part_2("input".to_string()), 1781);
    }
}
