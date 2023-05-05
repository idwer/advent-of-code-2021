fn input_to_list(input: Vec<&str>) -> Vec<u8> {
    let mut ret: Vec<_> = Vec::new();

    for line in input {
        for l in line.split(",") {
            ret.push(l.parse::<u8>().unwrap())
        }
    }

    ret
}

fn process_list(input: Vec<u8>) -> Vec<u8>  {
    let mut ret: Vec<_> = Vec::new();
    let mut amount_of_zeroes_in_input = 0;

    for e in &input {
        if e > &0 {
            ret.push(e - 1)
        }

        if e == &0 {
            ret.push(6);
            amount_of_zeroes_in_input += 1;
        }
    }

    for _ in 0..amount_of_zeroes_in_input {
        ret.push(8)
    }

    ret
}

fn solution(rows: Vec<&str>, days: u16) -> usize {
    let ret = 0;

    let mut list: Vec<_> = Vec::new();

    let initial_state = input_to_list(rows.clone());

    // println!("Initial state:\t{:?}", initial_state);

    list = process_list(initial_state);

    for n in 1..=days {
/*        if n == 1 {
            println!("After {:2} day:\t{:?}", n, list.clone());
        } else {
            println!("After {:2} days:\t{:?}", n, list.clone());
        }*/

        list = process_list(list);

        if n == days - 1 {
            return list.len()
        }
    }

    ret
}

pub fn solve_puzzle_06_sample(days: u16) -> usize {
    let instructions: Vec<_> = include_str!("../test_input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(instructions, days)
}

pub fn solve_puzzle_06(days: u16) -> usize {
    let instructions: Vec<_> = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(instructions, days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p06p1_sample() {
        assert_eq!(solve_puzzle_06_sample(80), 5934);
    }

    #[test]
    fn p06p1() {
        assert_eq!(solve_puzzle_06(80), 379114);
    }

    #[test]
    fn p06p2_sample() {
        assert_eq!(solve_puzzle_06_sample(256), 26984457539);
    }

    #[test]
    fn p06p2() {
        assert_eq!(solve_puzzle_06(256), 0);
    }
}
