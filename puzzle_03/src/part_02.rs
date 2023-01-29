fn parse_radix(radix: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let tmp = u64::from_str_radix(&radix, 2);
    match tmp {
        Ok(ok) => Ok(ok),
        Err(err) => Err(Box::new(err)),
    }
}

// the return type marking "Vec<&'a &'a str>" apparently is required? is this okay at all? it does compile ...
//
// the marking "Vec<&'a &'a str>" solved the following error:
// "returns a value referencing data owned by the current function"
// solution found at, and introduced from, https://old.reddit.com/r/learnrust/comments/r2pzbd/problem_leading_to_returning_a_value_referencing/
// fn reduce_vector<'a>(get_rating_for_co2: bool, rows: &'a Vec<&str>, line_width: u8, col_pos: u8) -> Vec<&'a &'a str> {
fn reduce_vector(get_rating_for_co2: bool, rows: Vec<&str>, line_width: u8, col_pos: u8, ) -> Vec<&str> {
// fn reduce_vector<'a>(get_rating_for_co2: bool, rows: Vec<&'a &'a str>, line_width: u8, col_pos: u8, ) -> Vec<&'a &'a str> {
    println!("debug | reduce_vector() | get_rating_for_co2 = {} rows = {:?} line_width = {} col_pos = {}", get_rating_for_co2, rows, line_width, col_pos);

    let mut nullen = Vec::<_>::new();
    let mut enen = Vec::<_>::new();

    for row in rows {
//        println!("debug | reduce_vector() | row = {:?}", row);

        let unwrapped_char = row.chars().nth(col_pos.try_into().unwrap());

        match unwrapped_char {
            Some(n) if n == '0' => {
               // println!(
               //     "debug | reduce_vector() | nullen | unwrapped_char = {:?} | push {} to {:?}",
               //     unwrapped_char, row, nullen
               // );
                nullen.push(row);
            }
            Some(n) if n == '1' => {
               // println!(
               //     "debug | reduce_vector() | enen | unwrapped_char = {:?} | push {} to {:?}",
               //     unwrapped_char, row, enen
               // );
                enen.push(row);
            }
            _ => (),
        }
    }

    println!("debug | reduce_vector() | location marker 0 | nullen = {:?} | enen = {:?}", nullen, enen);

    if get_rating_for_co2 {
        if nullen.len() <= enen.len() {
            println!("debug | reduce_vector() | location marker 1 | nullen.len() <= enen.len() | fn would return: {:?}", nullen);
            nullen
        } else {
            println!("debug | reduce_vector() | location marker 2 | fn would return: {:?}", enen);
            enen
        }
    } else {
        if enen.len() >= nullen.len() {
            println!("debug | reduce_vector() | location marker 3 | enen.len() >= nullen.len() | fn would return: {:?}", enen);
            enen
        } else {
            println!("debug | reduce_vector() | location marker 4 | fn would return: {:?}", nullen);
            nullen
        }
    }
}

// fn get_life_support_generator_rating(get_rating_for_co2: bool, rows: Vec<&str>, line_width: u8, col_pos: u8, ) -> Vec<char> {
fn get_life_support_generator_rating<'a>(get_rating_for_co2: bool, rows: Vec<&str>, line_width: u8, col_pos: u8, ) -> Vec<&str> {
// fn get_life_support_generator_rating<'a>(get_rating_for_co2: bool, rows: Vec<&str>, line_width: u8, col_pos: u8, ) -> Vec<char> {
    // let mut ret = Vec::<char>::new();
    // let mut ret = Vec::<_>::new();

    // just check for o2 ratings first
    // if col_pos < line_width { // otherwise we'd see "col_pos = 5" which is wrong
    // if col_pos < line_width && !get_rating_for_co2 {
        println!("\tdebug | get_life_support_generator_rating() | get_rating_for_co2 = {} rows = {:?} line_width = {} col_pos = {}", get_rating_for_co2, rows, line_width, col_pos);

        // let mut nullen = Vec::<&&str>::new();
        // let mut enen = Vec::<&&str>::new();
        //
        // for row in &rows {
        //     let unwrapped_char = row.chars().nth(col_pos.try_into().unwrap());
        //
        //     println!("debug | line 97 | get_life_support_generator_rating() | row = {:?} unwrapped_char = {:?}", row, unwrapped_char);
        //
        //     match unwrapped_char {
        //         Some(n) if n == '0' => {
        //             nullen.push(row);
        //
        //         }
        //         Some(n) if n == '1' => {
        //             enen.push(row);
        //
        //         }
        //         _ => (),
        //     }
        // }

        // println!("\ndebug | line 118| get_life_support_generator_rating() | nullen = {:?} enen = {:?}\n", nullen, enen);

        // println!("test: {:?}", reduce_vector(get_rating_for_co2, nullen.clone(), line_width, col_pos));
        // println!("test: {:?}", reduce_vector(get_rating_for_co2, enen.clone(), line_width, col_pos));

/*        if get_rating_for_co2 {
            if nullen.len() <= enen.len() {
                println!("\tdebug | get_life_support_generator_rating() | get_rating_for_co2 = {} | nullen in de minderheid/gelijk = {:?}", get_rating_for_co2, nullen);

                // todo: return some i64 value

                println!("debug | get_life_support_generator_rating() | location 1 | nullen: {:?}", nullen);
                println!("\tdebug | get_life_support_generator_rating() | calling reduce_vector() | reducing {:?} to {:?}", rows, reduce_vector(get_rating_for_co2, nullen.clone(), line_width, col_pos));

            } else {
                println!("\tdebug | get_life_support_generator_rating() | get_rating_for_co2 = {} | enen in de minderheid/gelijk   = {:?}", get_rating_for_co2, enen);
                // todo: return some i64 value

                println!("\tdebug | get_life_support_generator_rating() | location 2 | enen: {:?}", enen);

            }
        } else {
            if enen.len() <= nullen.len() {
                println!("\tdebug | get_life_support_generator_rating() | get_rating_for_co2 = {} | nullen in de meerderheid/gelijk   = {:?}", get_rating_for_co2, nullen);
                // todo: return some i64 value
                // ret = list_with_ones_common;

                // ret = enen;

                println!("debug | get_life_support_generator_rating() | location 3 | nullen: {:?}", nullen);
                //           println!("\t\t\t\t\t\tdebug | reduce_vector() | calling reduce_vector() | reducing {:?} to {:?}", rows, reduce_vector(get_rating_for_co2, &enen, line_width, col_pos));

            } else {
                //            println!("debug | col_pos = {} list_with_zeros_common = {:?} ", col_pos, list_with_zeros_common);
                println!("\tdebug | get_life_support_generator_rating() | get_rating_for_co2 = {} | enen in de meerderheid/gelijk = {:?}", get_rating_for_co2, enen);
                // todo: return some i64 value
                // ret = list_with_zeros_common;

                // ret = nullen;

                println!("debug | get_life_support_generator_rating() | location 4 | enen: {:?}", enen);
                //           println!("\t\t\t\t\t\tdebug | reduce_vector() | calling reduce_vector() | reducing {:?} to {:?}", rows, reduce_vector(get_rating_for_co2, &nullen, line_width, col_pos));

            }
        }*/

        // let mut x = _;
        // let mut x = Vec::<&&str>::new();
        // let mut x = Vec::<&str>::new();

        // let mut x = rows.clone();
    // println!("rows.len = {}", rows.len());
    // println!("rows = {:?}", rows);
    let mut x = reduce_vector(get_rating_for_co2, rows.clone(), line_width, col_pos);
    // println!("rows.len = {} x.len() = {} ", rows.len(), x.len());
    // println!("x = {:?} ", x);


        // let mut x = Vec::<&'a &'a str>::new();
/*        while x.len() > 1 {
            // x = reduce_vector(get_rating_for_co2, rows.clone(), line_width, col_pos);
            x = reduce_vector(get_rating_for_co2, x, line_width, col_pos);
            println!("x.len() = {} x = {:?}", x.len(), x);
        }*/


    if x.len() == 1 {
        println!("x.len() = {} x = {:?}", x.len(), x);
    }


            // if ret.len() > 1 {
            if x.len() > 1 {
                // println!("will recurse; ret.len() = {}", ret.len());
                println!("will recurse; ret.len() = {}", x.len());
            // get_life_support_generator_rating(get_rating_for_co2, rows, line_width, col_pos + 1);
            return get_life_support_generator_rating(get_rating_for_co2, x.clone(), line_width, col_pos + 1);
        }
    // }

    // ret
    x
}

fn solution2(rows: &Vec<&str>) -> u64 {
    let line_width = (rows[0].len()).try_into().unwrap();

    // println!("debug: solution2() rows = \t{:?}", rows);

    let co2_rate = get_life_support_generator_rating(true, rows.to_vec(), line_width, 0);
    let oxygen_rate = get_life_support_generator_rating(false, rows.to_vec(), line_width, 0);
    //    let co2_rate = get_life_support_generator_rating(true, &rows.to_vec(), line_width, 0);
    //    let oxygen_rate = get_life_support_generator_rating(false, &rows.to_vec(), line_width, 0);

       println!("\ndebug | solution2() co2_rate = \t{:?}", co2_rate);
       println!("debug | solution2() oxygen_rate = \t{:?}", oxygen_rate);

    let mut co2: u64 = 0;
    let mut o2: u64 = 0;

    if co2_rate.len() > 1 {
        panic!("too many elements in co2_rate");
    }
    if oxygen_rate.len() > 1 {
        panic!("too many elements in o2_rate");
    }

    match u64::from_str_radix(co2_rate[0].to_string().as_str(), 2) {
        Ok(ok) => {
            //		println!("co2_rate radix = {}", ok);
            // add matching radix stuff to co2;
            co2 += ok;
        } // Ok()
        _ => (),
    } // match

/*    for e in co2_rate {
        match u64::from_str_radix(e.to_string().as_str(), 2) {
            Ok(ok) => {
                //		println!("co2_rate radix = {}", ok);
                // add matching radix stuff to co2;
                co2 += ok;
            } // Ok()
            _ => (),
        } // match
    }*/

    match u64::from_str_radix(oxygen_rate[0].to_string().as_str(), 2) {
        Ok(ok) => {
            //		println!("o2_rate radix = {}", ok);
            o2 += ok;
        } // Ok() // , // ,
        // add matching radix stuff to o2;
        _ => (),
    } // match
/*    for e in oxygen_rate {
        match u64::from_str_radix(e.to_string().as_str(), 2) {
            Ok(ok) => {
                //		println!("o2_rate radix = {}", ok);
                o2 += ok;
            } // Ok() // , // ,
            // add matching radix stuff to o2;
            _ => (),
        } // match
    }*/
    println!("co2 = {}", co2);
    println!("o2 = {}", o2);

    // co2_rate * oxygen_rate
    co2 * o2
    // 0
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
