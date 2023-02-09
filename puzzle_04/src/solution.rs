use std::borrow::Borrow;
use std::cell::RefCell;
use crate::board;
use crate::board::{Board, BOARD_DIMENSION};
// use std::borrow::Borrow;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;

// fn get_number_of_boards(rows: Vec<&str>) -> usize {
fn get_number_of_boards(rows: Vec<&str>, block_size: usize) -> usize {
    // println!("get_number_of_boards() | board count = {:?}", rows[1..].len() / BOARD_DIMENSION);

    rows[1..].len() / BOARD_DIMENSION
}

fn get_drawn_numbers(rows: Vec<&str>) -> Vec<u8> {
// fn get_drawn_numbers(rows: Vec<&str>) -> Vec<&str> {
    // println!("get_drawn_numbers() | ret = {:?}", rows[0].split(',').filter(|e| e.parse::<u8>().is_ok()).collect::<Vec<&str>>());

/*    rows[0]
        .split(',')
        .filter(|e| e.parse::<u8>().is_ok())
        .collect::<Vec<&str>>()
*/        // .collect::<Vec<u8>>()

    let mut v: Vec<u8> = Vec::new();

    for e in rows[0].split(',') {
        match u8::from_str_radix(e, 10) {
            Ok(ok) => v.push(ok),
            _ => ()
        }
    }

    v
}

// fn generate_list_of_boards(number_of_boards: usize, board_data: Vec<&str>) -> Vec<Board> {
fn generate_list_of_boards(
    number_of_boards: usize,
    board_data: Vec<&str>,
    block_length: usize,
) -> Vec<Board> {
    let mut boards = Vec::<Board>::new();

    for n in (0..board_data.len() + 1).step_by(BOARD_DIMENSION) {
        if n + BOARD_DIMENSION <= board_data.len() {
            let cells = &board_data[n..n + BOARD_DIMENSION];

            let mut board = Board {
                // cells: HashMap::new(),
                // cells: BTreeMap::new(),
                cells: Rc::new(RefCell::new(BTreeMap::new())),
                // cells: BTreeMap::new(),
                won: false,
            };

            for cells in cells {
                for cell in cells.split_whitespace() {
                    // board.cells.insert(cell.parse::<u8>().unwrap(), false);
                    board.cells.into_inner().insert(cell.parse::<u8>().unwrap(), false);
                }
            }

            boards.push(board);
        }
    }

    // println!("generate_list_of_boards() | boards: {:?}", boards);
    boards
}

fn solution(rows: &Vec<&str>, squid_must_win: bool) -> u64 {
    let number_of_boards = get_number_of_boards(rows.clone(), BOARD_DIMENSION + 1);
    let board_data = &rows[1..];
    let mut list_of_boards =
        generate_list_of_boards(number_of_boards, rows[1..].to_vec(), BOARD_DIMENSION + 1);
    let drawn_numbers = get_drawn_numbers(rows.clone());

    // println!("rows: {:?}", rows);
    // println!("number_of_boards: {:?}", number_of_boards);
    // println!("board_data: {:?}", board_data);
    println!("list_of_boards: {:?}", list_of_boards);
    println!("\ndrawn_numbers: {:?}", drawn_numbers);

    if !squid_must_win {
        for number in &drawn_numbers {
            // println!(
            //     "solution() squid_must_win: {} number: {}",
            //     squid_must_win, number,
            // );

            for board in list_of_boards.iter_mut() {

                // board.mark_number(number.parse::<u8>().unwrap());
                board.mark_number(*number);

                // https://www.google.com/search?q=%22so+the+data+it+refers+to+cannot+be+borrowed+as+mutable%22&client=ubuntu&hs=rau&channel=fs&ei=vMbiY9zIHNP4sAfr7LugBA&ved=0ahUKEwjcluWnsYT9AhVTPOwKHWv2DkQQ4dUDCA4&uact=5&oq=%22so+the+data+it+refers+to+cannot+be+borrowed+as+mutable%22&gs_lcp=Cgxnd3Mtd2l6LXNlcnAQAzIHCAAQHhDxBDIGCAAQBRAeMgYIABAFEB4yBggAEAUQHjoKCAAQRxDWBBCwAzoICAAQBxAeEBM6CwgAEAcQHhDxBBATOgkIABAeEPEEEBNKBAhBGABKBAhGGABQyBdYkyZg7CloAXABeACAAXOIAY4DkgEDMi4ymAEAoAEByAEIwAEB&sclient=gws-wiz-serp
                if board.has_winning_row_or_column(true) || board.has_winning_row_or_column(false) {
                    // match u64::from_str_radix(number, 10) {
                    //     Ok(ok) => return ok * board.get_sum_of_unmarked_numbers(),
                    //     _ => ()
                    // }

                    // println!(
                    //     "solution() squid_must_win: {} number: {} {} {}",
                    //     squid_must_win, number,
                    //     board.has_winning_row_or_column(true),
                    //     board.has_winning_row_or_column(false)
                    // );

                    // return number.parse::<u64>().unwrap();
                    println!("\n\nlist_of_boards, 1/3 afterwards: {:?}\n", list_of_boards);
                    return *number as u64
                }
                // if board.has_winning_row_or_column(false) {
                //     match u64::from_str_radix(number, 10) {
                //         Ok(ok) => return ok * board.get_sum_of_unmarked_numbers(),
                //         _ => ()
                //     }
                // }
            }
        }
    }

    // todo: unfinished
    if squid_must_win {
        let mut boards_in_winning_order: Vec<Board> = Vec::new();
        let mut list_of_boards_len = list_of_boards.len();
        let mut boards_in_winning_order_len = boards_in_winning_order.len();

        for number in drawn_numbers {
            // println!(
            //     "solution() squid_must_win: {} number: {}",
            //     squid_must_win, number,
            // );



            // if list_of_boards.len() == boards_in_winning_order.len() {
            if list_of_boards_len == boards_in_winning_order.len() {
                println!("breaking");
                break;
            }

            for board in list_of_boards.iter_mut() {
                // println!("o hi");
                // board.mark_number(number);

                // match u8::from_str_radix(number, 10) {
                //     Ok(ok) => board.mark_number(ok),
                //     _ => (),
                // }
                board.mark_number(number);

                // println!(
                //     "solution() squid_must_win: {} board.won: {}",
                //     squid_must_win, board.won
                // );

                // println!(
                //     "solution() squid_must_win: {} number: {} {} {}",
                //     squid_must_win, number,
                //     board.has_winning_row_or_column(true),
                //     board.has_winning_row_or_column(false)
                // );

                if !board.won
                && (board.has_winning_row_or_column(true)
                || board.has_winning_row_or_column(false))
                {
                    // println!(
                    //     "solution() squid_must_win: {} number: {} {} {}",
                    //     squid_must_win, number,
                    //     board.has_winning_row_or_column(true),
                    //     board.has_winning_row_or_column(false)
                    // );
                    boards_in_winning_order.push(board.clone());
                }

                // if list_of_boards.len() == boards_in_winning_order.len() &&
                // if list_of_boards_len == boards_in_winning_order.len() &&
                if list_of_boards_len == boards_in_winning_order_len &&
                    board.has_winning_row_or_column(true) ||
                    board.has_winning_row_or_column(false) {

                    // return number.parse::<u64>().unwrap()
                    println!("\n\nlist_of_boards, 2/3 afterwards: {:?}\n", list_of_boards);
                    return number as u64
                        // * boards_in_winning_order[boards_in_winning_order.len()]
                        * boards_in_winning_order[boards_in_winning_order_len - 1]
                        // * boards_in_winning_order[boards_in_winning_order.len() - 1]
                        .get_sum_of_unmarked_numbers();
                }

                // }
            }
        }
    }

    println!("\n\nlist_of_boards, 3/3 afterwards: {:?}\n", list_of_boards);

    0
    // -1
}

pub fn solve_puzzle_04(squid_must_win: bool) -> u64 {
    let instructions: Vec<_> = include_str!("../input")
    .lines()
    .filter(|l| !l.is_empty())
        .collect();

    solution(&instructions, squid_must_win)
}

pub fn solve_puzzle_04_sample(squid_must_win: bool) -> u64 {
    let instructions: Vec<_> = include_str!("../test_input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(&instructions, squid_must_win)
}

#[cfg(test)]
mod tests_p04p1 {
    use super::*;

    #[test]
    fn test_p04p1() {
        assert_eq!(solve_puzzle_04(false), 39902);
    }

    #[test]
    fn test_p04p1_sample() {
        assert_eq!(solve_puzzle_04_sample(false), 4512);
    }

    #[test]
    fn test_p04p2() {
        assert_eq!(solve_puzzle_04(true), 39902);
    }

    #[test]
    fn test_p04p2_sample() {
        assert_eq!(solve_puzzle_04_sample(true), 4512);
    }
}
