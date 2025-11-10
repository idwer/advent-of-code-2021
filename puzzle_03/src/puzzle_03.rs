fn get_rate(rows: Vec<&str>, line_width: i8, gamma: bool) -> u64 {
    let mut rate = 0;

    for position in 0..line_width {
        let mut zeroes = 0;
        let mut ones = 0;

        for row in &rows {
            let tmp = row.chars().nth(position.try_into().unwrap());

            match tmp {
                Some(n) if n == '0' => zeroes += 1,
                Some(n) if n == '1' => ones += 1,
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

#[cfg(test)]
mod tests_p03p1 {
    use super::*;

    #[test]
    fn test_p03p1() {
        assert_eq!(solve_part_1(), 4147524);
    }

    #[test]
    fn test_p03p1_sample() {
        assert_eq!(solve_part_1_sample(), 198);
    }
}
fn parse_radix(radix: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let tmp = u64::from_str_radix(&radix, 2);
    match tmp {
        Ok(ok) => Ok(ok),
        Err(err) => Err(Box::new(err)),
    }
}

fn get_life_support_generator_rating(get_rating_for_co2: bool, rows: Vec<&str>, col_pos: u8) -> Vec<&str> {
    let mut ret = Vec::<_>::new();

    let mut nullen = Vec::<_>::new();
    let mut enen = Vec::<_>::new();

    for row in rows {
        let unwrapped_char = row.chars().nth(col_pos.try_into().unwrap());

        match unwrapped_char {
            Some(n) if n == '0' => {
                nullen.push(row);
            }
            Some(n) if n == '1' => {
                enen.push(row);
            }
            _ => (),
        }
    }

    if get_rating_for_co2 {
        if nullen.len() <= enen.len() {
            ret = nullen
        } else {
            ret = enen
        }
    } else {
        if enen.len() >= nullen.len() {
            ret = enen
        } else {
            ret = nullen
        }
    }

    if ret.len() > 1 {
        return get_life_support_generator_rating(get_rating_for_co2, ret, col_pos + 1);
    }

    ret
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

    match parse_radix(co2_rate[0]) {
        Ok(ok) => {
            co2 += ok;
        }
        _ => (),
    }

    match parse_radix(oxygen_rate[0]) {
        Ok(ok) => {
            o2 += ok;
        }
        _ => (),
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
mod tests_p03p2 {
    use super::*;

    #[test]
    fn test_p03p2() {
        assert_eq!(solve_part_2(), 3570354);
    }

    #[test]
    fn test_p03p2_sample() {
        assert_eq!(solve_part_2_sample(), 230);
    }
}
