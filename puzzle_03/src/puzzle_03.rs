fn get_rate(rows: Vec<&str>, line_width: u8, gamma: bool) -> u64 {
    let mut rate = 0;

    for position in 0..line_width {
        let mut zeroes = 0;
        let mut ones = 0;

        for row in &rows {
            let tmp = row.chars().nth(position.try_into().unwrap());

            match tmp {
                Some('0') => zeroes += 1,
                Some('1') => ones += 1,
                _ => todo!(),
            }
        }

        if position > 0 {
            rate <<= 1;
        }

        if !gamma && ones < zeroes || gamma && ones > zeroes {
            rate |= 1
        }
    }

    rate
}

fn parse_radix(radix: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let tmp = u64::from_str_radix(radix, 2);
    match tmp {
        Ok(ok) => Ok(ok),
        Err(err) => Err(Box::new(err)),
    }
}

fn get_life_support_generator_rating(get_rating_for_co2: bool, rows: Vec<&str>, col_pos: u8) -> Vec<&str> {
    let mut ret = Vec::<_>::new();

    let mut zeroes = Vec::<_>::new();
    let mut ones = Vec::<_>::new();

    for row in rows {
        let unwrapped_char = row.chars().nth(col_pos.into());

        match unwrapped_char {
            Some('0') => {
                zeroes.push(row);
            }

            Some('1') => {
                ones.push(row);
            }
            _ => (),
        }
    }

    if get_rating_for_co2 {
        if zeroes.len() <= ones.len() {
            ret = zeroes
        } else {
            ret = ones
        }
    } else if ones.len() >= zeroes.len() {
            ret = ones
    } else {
            ret = zeroes
    }

    if ret.len() > 1 {
        return get_life_support_generator_rating(get_rating_for_co2, ret, col_pos + 1);
    }

    ret
}

fn solution1(rows: &Vec<&str>) -> u64 {
    let line_width = (rows[0].len()).try_into().unwrap();

    let epsilon_rate = get_rate(rows.to_vec(), line_width, false);
    let gamma_rate = get_rate(rows.to_vec(), line_width, true);

    epsilon_rate * gamma_rate
}

pub fn solve_part_1() -> u64 {
    let instructions: Vec<_> = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution1(&instructions)
}

pub fn solve_part_1_sample() -> u64 {
    let instructions: Vec<_> = include_str!("../test_input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution1(&instructions)
}

fn solution2(rows: &Vec<&str>) -> u64 {
    let co2_rate = get_life_support_generator_rating(true, rows.to_vec(), 0);
    let oxygen_rate = get_life_support_generator_rating(false, rows.to_vec(), 0);

    let mut co2: u64 = 0;
    let mut o2: u64 = 0;

    if co2_rate.len() > 1 {
        panic!("too many elements in co2_rate");
    }

    if oxygen_rate.len() > 1 {
        panic!("too many elements in o2_rate");
    }

    if let Ok(ok) = parse_radix(co2_rate[0]) {
        co2 += ok;
    }

    if let Ok(ok) = parse_radix(oxygen_rate[0]) {
        o2 += ok;
    }

    co2 * o2
}

pub fn solve_part_2() -> u64 {
    let instructions: Vec<_> = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution2(&instructions)
}

pub fn solve_part_2_sample() -> u64 {
    let instructions: Vec<_> = include_str!("../test_input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution2(&instructions)
}

#[cfg(test)]
mod tests_p03 {
    use super::*;

    #[test]
    fn test_get_epsilon_rate() {
        let instructions: Vec<&str> = include_str!("../test_input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let line_width = (instructions[0].len()).try_into().unwrap();

        let epsilon_rate = get_rate(instructions.to_vec(), line_width, false);

        assert_eq!(epsilon_rate, 9);
    }

    #[test]
    fn test_get_gamma_rate() {
        let instructions: Vec<&str> = include_str!("../test_input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let line_width = (instructions[0].len()).try_into().unwrap();

        let gamma_rate = get_rate(instructions.to_vec(), line_width, true);

        assert_eq!(gamma_rate, 22);
    }

    #[test]
    fn test_p03_p1() {
        assert_eq!(solve_part_1(), 4147524);
    }

    #[test]
    fn test_p03p1_sample() {
        assert_eq!(solve_part_1_sample(), 198);
    }

    #[test]
    fn test_p03p2() {
        assert_eq!(solve_part_2(), 3570354);
    }

    #[test]
    fn test_p03p2_sample() {
        assert_eq!(solve_part_2_sample(), 230);
    }
}
