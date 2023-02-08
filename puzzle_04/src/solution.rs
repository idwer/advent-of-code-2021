use std::borrow::Borrow;
use std::collections::HashMap;
use crate::board;
use crate::board::{Board, BOARD_DIMENSION};

fn get_number_of_boards(rows: Vec<&str>, block_size: usize) -> usize {
    rows[1..].len() / BOARD_DIMENSION
}

fn get_drawn_numbers(rows: Vec<&str>) -> Vec::<&str> {
    rows[0].split(',').filter(|e| e.parse::<u8>().is_ok()).collect::<Vec<&str>>()
}

fn generate_list_of_boards(number_of_boards: usize, board_data: Vec<&str>, block_length: usize) -> Vec<Board> {
    let mut boards = Vec::<Board>::new();

    for n in (0..board_data.len() + 1).step_by(BOARD_DIMENSION) {
        if n + BOARD_DIMENSION <= board_data.len() {
            let cells = &board_data[n..n + BOARD_DIMENSION];

            let mut board = Board {
                cells: HashMap::new(),
                won: false
            };
            for cells in cells {
                for cell in cells.split_whitespace() {
                    board.cells.insert(cell.parse::<u8>().unwrap(), false);
                }
            }
            board.cells.insert(board_data[n].parse::<u8>().is_ok().into(), false);
            board.cells.insert(board_data[n + 1].parse::<u8>().is_ok().into(), false);
            board.cells.insert(board_data[n + 2].parse::<u8>().is_ok().into(), false);
            board.cells.insert(board_data[n + 3].parse::<u8>().is_ok().into(), false);
            board.cells.insert(board_data[n + 4].parse::<u8>().is_ok().into(), false);

            boards.push(board);
        }
    }

    boards
}

fn solution(rows: &Vec<&str>, squid_must_win: bool) -> u64 {
    let number_of_boards = get_number_of_boards(rows.clone(), BOARD_DIMENSION + 1);
    let board_data = &rows[1..];
    let mut list_of_boards = generate_list_of_boards(number_of_boards, rows[1..].to_vec(), BOARD_DIMENSION + 1);
    let drawn_numbers = get_drawn_numbers(rows.clone());

    if !squid_must_win {
        for number in &drawn_numbers {
            for board in &mut list_of_boards {
                board.mark_number(number.parse::<u8>().unwrap());

                // https://www.google.com/search?q=%22so+the+data+it+refers+to+cannot+be+borrowed+as+mutable%22&client=ubuntu&hs=rau&channel=fs&ei=vMbiY9zIHNP4sAfr7LugBA&ved=0ahUKEwjcluWnsYT9AhVTPOwKHWv2DkQQ4dUDCA4&uact=5&oq=%22so+the+data+it+refers+to+cannot+be+borrowed+as+mutable%22&gs_lcp=Cgxnd3Mtd2l6LXNlcnAQAzIHCAAQHhDxBDIGCAAQBRAeMgYIABAFEB4yBggAEAUQHjoKCAAQRxDWBBCwAzoICAAQBxAeEBM6CwgAEAcQHhDxBBATOgkIABAeEPEEEBNKBAhBGABKBAhGGABQyBdYkyZg7CloAXABeACAAXOIAY4DkgEDMi4ymAEAoAEByAEIwAEB&sclient=gws-wiz-serp
                if board.has_winning_row_or_column(true) {
                    match u64::from_str_radix(number, 10) {
                        Ok(ok) => return ok * board.get_sum_of_unmarked_numbers(),
                        _ => ()
                    }
                }
            }
        }
    }

    // todo: unfinished
    if squid_must_win {
        let mut boards_in_winning_order: Vec<Board> = Vec::new();

        for number in drawn_numbers {
            if list_of_boards.len() == boards_in_winning_order.len() {
                break
            }

            for mut board in &list_of_boards {
                // board.mark_number(number);
                match u8::from_str_radix(number, 10) {
                    Ok(ok) => board.mark_number(ok),
                    _ => ()
                }
            }

        }
    }

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
