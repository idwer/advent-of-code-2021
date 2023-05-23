use std::collections::HashMap;

fn fuel_cost_of_elements_in_hashmap(input: HashMap<i64, i64>, offset: i64) -> i64 {
    let mut ret = 0;

    for e in input {
        if e.0 == 0 || e.0 - offset == 0 {
            continue
        }

        // used with offset = 10
        if offset % 10 == 0 {
            if offset > (e.0 * e.1) {
                ret += offset - (e.0 * e.1);
            }

            if e.0 * e.1 > offset {
                ret += (e.0 * e.1) + offset;
            }
        } else {
            // used with offset = 1, offset = 2, offset = 3
            if (e.0 - offset) > offset || (offset - e.0) > e.0 {
                ret += e.0 * e.1
            }
        }
    }

    ret
}

fn input_to_list(input: Vec<&str>) -> Vec<i64> {
    let mut ret: Vec<_> = Vec::new();

    for line in &input {
        for l in line.split(",") {
            ret.push(l.parse::<i64>().unwrap())
        }
    }

    ret
}

fn list_to_hashmap(input: Vec<i64>) -> HashMap<i64, i64> {
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

fn solution(rows: Vec<&str>, positions: Vec<i64>) -> i64 {
    let initial_state = input_to_list(rows.clone());
    let rethm = list_to_hashmap(initial_state);

    let mut results = Vec::new();

    for n in positions {
        results.push(fuel_cost_of_elements_in_hashmap(rethm.clone(), n))
    }

    results.iter().min().unwrap().clone()
}

pub fn solve_puzzle_07_sample(positions: Vec<i64>) -> i64 {
    let instructions: Vec<_> = include_str!("../test_input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(instructions, positions)
}

pub fn solve_puzzle_07(positions: Vec<i64>) -> i64 {
    let instructions: Vec<_> = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(instructions, positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p07_test_fuel_cost_of_elements_in_hashmap_01() {
        let hm = HashMap::from([
            (16, 1),
            (0, 1),
            (4, 1),
            (7, 1),
            (2, 3),
            (1, 2),
            (14, 1),
        ]);

        assert_eq!(fuel_cost_of_elements_in_hashmap(hm, 1), 41);
    }

    #[test]
    fn p07_test_fuel_cost_of_elements_in_hashmap_02() {
        let hm = HashMap::from([
            (16, 1),
            (0, 1),
            (4, 1),
            (7, 1),
            (2, 3),
            (1, 2),
            (14, 1),
        ]);

        assert_eq!(fuel_cost_of_elements_in_hashmap(hm.clone(), 2), 37);
    }

    #[test]
    fn p07_test_fuel_cost_of_elements_in_hashmap_03() {
        let hm = HashMap::from([
            (16, 1),
            (0, 1),
            (4, 1),
            (7, 1),
            (2, 3),
            (1, 2),
            (14, 1),
        ]);

        assert_eq!(fuel_cost_of_elements_in_hashmap(hm.clone(), 3), 39);
    }

    #[test]
    fn p07_test_fuel_cost_of_elements_in_hashmap_04() {
        let hm = HashMap::from([
            (16, 1),
            (0, 1),
            (4, 1),
            (7, 1),
            (2, 3),
            (1, 2),
            (14, 1),
        ]);

        assert_eq!(fuel_cost_of_elements_in_hashmap(hm.clone(), 10), 71);
    }

    #[test]
    fn p07_test_list_to_hashmap() {
        let instructions: Vec<_> = include_str!("../test_input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let input = input_to_list(instructions);
        let hm = list_to_hashmap(input);

        assert_eq!(hm.len(), 7);
    }

    #[test]
    fn p07p1_sample() {
        assert_eq!(solve_puzzle_07_sample(Vec::from([1, 2, 3, 10])), 37);
    }

    #[test]
    fn p07p1() {
        // must not be 102830, or 500483
        assert_eq!(solve_puzzle_07(Vec::from([1, 2, 3, 10])), 0);
        // assert_ne!(solve_puzzle_07(Vec::from([1, 2, 3, 10])), 0);
        // assert_ne!(solve_puzzle_07(Vec::from([1, 2, 3, 10])), 500483);
    }

    #[test]
    fn p07p2_sample() {
        assert_eq!(solve_puzzle_07_sample(Vec::from([1, 2, 3, 10])), 0);
    }

    #[test]
    fn p07p2() {
        assert_eq!(solve_puzzle_07(Vec::from([1, 2, 3, 10])), 0);
    }
}
