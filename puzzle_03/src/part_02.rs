fn parse_radix(radix: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let tmp = u64::from_str_radix(&radix, 2);
    match tmp {
        Ok(ok) => Ok(ok),
        Err(err) => Err(Box::new(err)),
    }
}

fn reduce_vector(get_rating_for_co2: bool, rows: Vec<&str>, line_width: u8, col_pos: u8) -> Vec<&str> {
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
            nullen
        } else {
            enen
        }
    } else {
        if enen.len() >= nullen.len() {
            enen
        } else {
            nullen
        }
    }
}

fn get_life_support_generator_rating<'a>(get_rating_for_co2: bool, rows: Vec<&str>, line_width: u8, col_pos: u8) -> Vec<&str> {
    let mut x = reduce_vector(get_rating_for_co2, rows.clone(), line_width, col_pos);

    if x.len() > 1 {
        return get_life_support_generator_rating(get_rating_for_co2, x.clone(), line_width, col_pos + 1);
    }

    x
}

fn solution2(rows: &Vec<&str>) -> u64 {
    let line_width = (rows[0].len()).try_into().unwrap();

    let co2_rate = get_life_support_generator_rating(true, rows.to_vec(), line_width, 0);
    let oxygen_rate = get_life_support_generator_rating(false, rows.to_vec(), line_width, 0);

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
