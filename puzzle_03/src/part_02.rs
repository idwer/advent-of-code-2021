// fn get_rate(rows: Vec<&str>, line_width: i8, gamma: bool) -> u64 {
fn get_life_support_generator_rating(get_rating_for_co2: bool, rows: Vec<&str>, line_width: u8, col_pos: u8) -> Vec<&str> {
    // let mut rate = 0;
    let mut ret = Vec::<&str>::new();

    let mut list_with_ones_common = Vec::<&str>::new();
    let mut list_with_zeros_common = Vec::<&str>::new();

    // for row in rows {
    //     row.chars().pos
    // }

    // for position in 0..line_width {
        let mut zeroes = 0;
        let mut ones = 0;

        for row in &rows {
            // let tmp = row.chars().nth(position.try_into().unwrap() );
            let tmp = row.chars().nth(col_pos.try_into().unwrap() );

            match tmp {
                Some(n) if n == '0' => zeroes += 1,
                Some(n) if n == '1' => ones += 1,
                _ => todo!(),
            }
        } // for row in rows

        // if position > 0 {
        //     rate <<= 1;
        // }

        // if !gamma && ones < zeroes || gamma && ones > zeroes {
        //     rate |= 1
        // }
    // } // for position in ...

    if get_rating_for_co2 {
        if list_with_zeros_common.len() <= list_with_ones_common.len() {
            ret = list_with_zeros_common;
        } else {
            ret = list_with_ones_common;
        }
    } else {
        if list_with_ones_common.len() <= list_with_zeros_common.len() {
            ret = list_with_ones_common;
        } else {
            ret = list_with_zeros_common;
        }
    }

    if ret.len() > 1 {
        println!("@ recursion");
        get_life_support_generator_rating(get_rating_for_co2, rows, line_width, col_pos + 1);
    }

    ret
}

fn solution2(rows: &Vec<&str>) -> u64 {
    let line_width = (rows[0].len()).try_into().unwrap();

    // println!("debug: solution2() rows = \t{:?}", rows);

    let co2_rate = get_life_support_generator_rating(true, rows.to_vec(), line_width, 0);
    let oxygen_rate = get_life_support_generator_rating(false, rows.to_vec(), line_width, 0);

    println!("debug: solution2() co2_rate = \t{:?}", co2_rate);
    println!("debug: solution2() oxygen_rate = \t{:?}", oxygen_rate);

    // co2_rate * oxygen_rate
    0
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
