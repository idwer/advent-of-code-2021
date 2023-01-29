mod part_01;
mod part_02;

fn main() {
    /*
    // co2: ['100111000110']
    // o2:  ['010110010011']

    // let co2 = "100111000110".split_whitespace();
    // let co2 = "100111000110".split_once();
    let co2 = "100a111000110".chars();

    // let co2 = "100111000110".split(char::is_numeric);


    // println!("{} {}", u32::, b)

    println!("{:?}", co2);

    //match co2 {
    //Ok(ok) => println!("ok: {}", ok),
    //_ => (),
    //}
    for e in co2 {
    println!("e = {}, radix = {:?}", e, u32::from_str_radix(e.to_string().as_str(), 2));
    }

    */

    /*
        let co2_sample = "01010";
        let o2_sample = "10111";
        let co2 = "100111000110";
        let o2 = "010110010011";

        let mut small = 0;
        let mut large = 0;

        match u32::from_str_radix(co2_sample.to_string().as_str(), 2) {
            Ok(ok) => small = ok,
            _ => (),
        }


        match u32::from_str_radix(o2_sample.to_string().as_str(), 2) {
            Ok(ok) => small = small * ok,
            _ => (),
        }

        match u32::from_str_radix(co2.to_string().as_str(), 2) {
            Ok(ok) => large = ok,
            _ => (),
        }


        match u32::from_str_radix(o2.to_string().as_str(), 2) {
            Ok(ok) => large = large * ok,
            _ => (),
        }


    //	let small =  * ;
    //	let large =  * ;
        println!("co2_sample radix = {:?}", u32::from_str_radix(co2_sample.to_string().as_str(), 2));
        println!("o2_sample radix = {:?}", u32::from_str_radix(o2_sample.to_string().as_str(), 2));
        println!("co2 radix = {:?}", u32::from_str_radix(co2.to_string().as_str(), 2));
        println!("o2 radix = {:?}", u32::from_str_radix(o2.to_string().as_str(), 2));


        println!("{} {}", small, large);
    */

    let result_part_1 = part_01::solve_part_1();
    let result_part_1_sample = part_01::solve_part_1_sample();

    let result_part_2 = part_02::solve_part_2();
    let result_part_2_sample = part_02::solve_part_2_sample();

    // println!("Solution for part 1: {}", result_part_1);
    // println!("Solution for part 1: {} - using test_input", result_part_1_sample);

    println!("Solution for part 2: {}", result_part_2);
    println!("Solution for part 2: {} - using test_input", result_part_2_sample);
}
