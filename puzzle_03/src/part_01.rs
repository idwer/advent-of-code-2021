fn get_rate(rows: Vec<&str>, line_width: i8, gamma: bool) -> u64 {
    let mut rate = 0;

    for position in 0..line_width {
        let mut zeroes = 0;
        let mut ones = 0;

        for row in &rows {
            let tmp = row.chars().nth(position.try_into().unwrap() );

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
