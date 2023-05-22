use std::collections::HashMap;

fn input_to_list(input: Vec<&str>) -> Vec<u64> {
    let mut ret: Vec<_> = Vec::new();

    for line in &input {
        for l in line.split(",") {
            ret.push(l.parse::<u64>().unwrap())
        }
    }

    ret
}

fn list_to_hashmap(input: Vec<u64>) -> HashMap<u64, u64> {
    let mut ret = HashMap::new();

    // upsert values in the hashmap
    for e in input.clone() {
        match ret.contains_key(&e) {
            true => {
                let value = ret.get(&e).unwrap();
                // update
                ret.insert(e, value + 1)
            }
            // insert
            false => ret.insert(e, 1)
        };
    }

    ret
}

fn amount_of_elements_in_hashmap(input: HashMap<u64, u64>) -> u64 {
    let mut ret = 0;

    for e in input {
        ret += e.1 as u64
    }

    ret
}

fn process_list(input: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut rethm = HashMap::new();

    for key in input.clone().keys() {
        match key {
            0 => {
                let mut amount_of_zeroes_in_input = 0;

                let result = input.get(&0);

                match result {
                    None => (),
                    Some(s) => {
                        amount_of_zeroes_in_input = *s;
                    }
                }

                match rethm.contains_key(&6) {
                    true => {
                        let result = rethm.get(&6);

                        match result {
                            None => (),
                            Some(s) => {
                                // update
                                rethm.insert(6, s + amount_of_zeroes_in_input);
                            }
                        }
                    }
                    false => {
                        // insert
                        rethm.insert(6, amount_of_zeroes_in_input);
                    }
                }

                match rethm.contains_key(&8) {
                    true => {
                        let result = rethm.get(&8);

                        match result {
                            None => (),
                            Some(s) => {
                                // update
                                rethm.insert(8, s + amount_of_zeroes_in_input);
                            }
                        }
                    }
                    false => {
                        // insert
                        rethm.insert(8, amount_of_zeroes_in_input);
                    }
                }
            }
            1.. => {
                let result = input.get(key);

                match result {
                    None => (),
                    Some(s_outer) => {
                        match rethm.contains_key(&(key - 1)) {
                            true => {
                                let result = rethm.get(&(key - 1));

                                match result {
                                    None => (),
                                    Some(s) => {
                                        // update
                                        rethm.insert(key.clone() - 1, s + s_outer);
                                    }
                                }
                            }
                            false => {
                                rethm.insert(key.clone() - 1, *s_outer);
                            }
                        }
                    }
                }
            }
        }
    }
    
    rethm
}

fn solution(rows: Vec<&str>, days: u16) -> u64 {
    let mut _rethm = HashMap::new();
    let initial_state = input_to_list(rows.clone());

    _rethm = list_to_hashmap(initial_state.clone());

    for n in 1..=days {
        _rethm = process_list(_rethm);

        if n == days {
            return amount_of_elements_in_hashmap(_rethm);
        }
    }

    0
}

pub fn solve_puzzle_06_sample(days: u16) -> u64 {
    let instructions: Vec<_> = include_str!("../test_input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(instructions, days)
}

pub fn solve_puzzle_06(days: u16) -> u64 {
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
    fn p06_test_amount_of_elements_in_hashmap_without_zero_in_input() {
        let hm = HashMap::from([
           (3, 2),
           (4, 1),
           (2, 1),
           (1, 1),
        ]);
        
        assert_eq!(amount_of_elements_in_hashmap(hm), 5);
    }

    #[test]
    fn p06_test_amount_of_elements_in_hashmap_with_zero_in_input() {
        let hm = HashMap::from([
            (0, 2),
            (1, 1),
            (5, 1),
            (6, 1),
            (7, 1),
            (8, 1),
        ]);

        assert_eq!(amount_of_elements_in_hashmap(hm), 7);
    }

    #[test]
    fn p06_test_list_to_hashmap() {
        let initial_state = Vec::from([3, 4, 3, 1, 2]);
        let hm = list_to_hashmap(initial_state);

        assert_eq!(hm.len(), 4);
    }

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
        assert_eq!(solve_puzzle_06(256), 1702631502303);
    }
}
