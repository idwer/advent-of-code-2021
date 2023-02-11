use crate::board::{Board, BOARD_DIMENSION};

fn get_drawn_numbers(rows: Vec<&str>) -> Vec<u64> {
    let mut drawn_numbers: Vec<u64> = Vec::new();

    for e in rows[0].split(',') {
        match u64::from_str_radix(e, 10) {
            Ok(number) => drawn_numbers.push(number),
            _ => (),
        }
    }

    drawn_numbers
}

fn generate_list_of_boards(board_data: Vec<&str>) -> Vec<Board> {
    let mut boards = Vec::<Board>::new();

    for n in (0..board_data.len() + 1).step_by(BOARD_DIMENSION) {
        if n + BOARD_DIMENSION <= board_data.len() {
            let rows = &board_data[n..n + BOARD_DIMENSION];
            // dbg!(rows);

            let mut board = Board {
                cells: Vec::new(),
                won: false,
            };

            for cells in rows {
                for cell in cells.split_whitespace() {
                    board.cells.push((cell.parse::<u64>().unwrap(), false));
                }
            }

            boards.push(board);
        }
    }

    // println!("generate_list_of_boards() | boards: {:?}", boards);
    boards
}

fn solution(rows: &Vec<&str>, part_two: bool) -> u64 {
    let mut list_of_boards = generate_list_of_boards(rows[1..].to_vec());
    let drawn_numbers = get_drawn_numbers(rows.clone());

    if !part_two {
        for number in &drawn_numbers {
            for board in list_of_boards.iter_mut() {

                // let b = board.mark_number(*number);
                // if b.has_winning_row_or_column(true) || b.has_winning_row_or_column(false) {
                    // println!("{:?} {:?} {:?}", squid_must_win, number, board.get_sum_of_unmarked_numbers());
                    // return *number as u64 * b.get_sum_of_unmarked_numbers();
                // }

                board.mark_number(*number);

                // println!("b.hwroc: {:?} {:?}", board.has_winning_row_or_column(true), number);

                // println!("{:?} {:?} {:?}", squid_must_win, number, board.get_sum_of_unmarked_numbers());
                if board.has_winning_row_or_column(true) || board.has_winning_row_or_column(false) {
                    // println!("solution.rs:63 | {:?} {:?} {:?} | {}", part_two, number, board.get_sum_of_unmarked_numbers(), number * board.get_sum_of_unmarked_numbers());
                    return *number as u64 * board.get_sum_of_unmarked_numbers();
                }
            }
        }
    }

    if part_two {
        let mut boards_in_winning_order: Vec<Board> = Vec::new();
        // let mut list_of_boards_len = list_of_boards.len();
        let list_of_boards_len = list_of_boards.len();
        // let mut boards_in_winning_order_len = boards_in_winning_order.len();

        let mut boards_in_winning_order_len = boards_in_winning_order.len();

        for number in drawn_numbers {
            if list_of_boards_len == boards_in_winning_order.len() {
                break;
            }

            for board in list_of_boards.iter_mut() {
                board.mark_number(number);

                if !board.won
                && (board.has_winning_row_or_column(true)
                || board.has_winning_row_or_column(false))
                {
                    boards_in_winning_order.push(board.clone());
                    boards_in_winning_order_len = boards_in_winning_order.len();
                }

                // println!("{:?} {:?} {:?}", squid_must_win, number, board.get_sum_of_unmarked_numbers());
                if list_of_boards_len == boards_in_winning_order_len
                && board.has_winning_row_or_column(true)
                || board.has_winning_row_or_column(false)
                {
                    // println!("{:?} {:?}", squid_must_win, number);
                    println!("solution.rs:100 | {:?} {:?} {:?} | {}", part_two, number, board.get_sum_of_unmarked_numbers(), number * board.get_sum_of_unmarked_numbers());

                    // let boards_in_winning_order_len = boards_in_winning_order.len();
                    // boards_in_winning_order_len = boards_in_winning_order.len();
                    // println!("solution.rs:103 len {:?}", boards_in_winning_order);
                    // println!("solution.rs:104 len {}", boards_in_winning_order.len());

                    // println!("solution.rs:104 board = {:?}", board);
                    println!("solution.rs:104 board = {:?}", boards_in_winning_order[boards_in_winning_order_len - 1]);

                    // dbg!(boards_in_winning_order.clone());

                    return number as u64
                    * boards_in_winning_order[boards_in_winning_order_len - 1]
                    // * boards_in_winning_order[list_of_boards_len - 1]
                    // * boards_in_winning_order[*boards_in_winning_order.len().borrow_mut() - 1]
                            .get_sum_of_unmarked_numbers();
                }
            }
        }
    }

    // println!("\n\nlist_of_boards, 3/3 afterwards: {:?}\n", list_of_boards);

    0
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
mod tests {
    // use std::assert_matches::assert_matches;
    use super::*;

    #[test]
    fn test_get_drawn_numbers() {
        let instructions: Vec<_> = include_str!("../test_input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();


        let list_of_boards = generate_list_of_boards(instructions[1..].to_vec());

        /*        assert_eq!(list_of_boards[0],
            [
                [(22, false)],
                [(13, false)],
                [(17, false)],
                [(11, false)],
                [(0, false)],

                [(8, false)],
                [(2, false)],
                [(23, false)],
                [(4, false)],
                [(24, false)],

                [(21, false)],
                [(9, false)],
                [(14, false)],
                [(16, false)],
                [(7, false)],

                [(6, false)],
                [(10, false)],
                [(3, false)],
                [(18, false)],
                [(5, false)],

                [(1, false)],
                [(12, false)],
                [(20, false)],
                [(15, false)],
                [(19, false)],


            ], false
        );*/

        // debug_assert_eq!(list_of_boards[0],
        // [Board { cells: [(22, false), (13, false), (17, false), (11, false), (0, false), (8, false), (2, false), (23, false), (4, false), (24, false), (21, false), (9, false), (14, false), (16, false), (7, false), (6, false), (10, false), (3, false), (18, false), (5, false), (1, false), (12, false), (20, false), (15, false), (19, false)], won: false }]
        // [Board { cells: [(22, false), (13, false), (17, false), (11, false), (0, false), (8, false), (2, false), (23, false), (4, false), (24, false), (21, false), (9, false), (14, false), (16, false), (7, false), (6, false), (10, false), (3, false), (18, false), (5, false), (1, false), (12, false), (20, false), (15, false), (19, false)].to_vec(), won: false }]
        // );

        // assert_eq!(list_of_boards[0], { Board {}});

        // assert_eq!(list_of_boards[0]
        assert_eq!(list_of_boards[0].cells, [(22, false), (13, false), (17, false), (11, false), (0, false), (8, false), (2, false), (23, false), (4, false), (24, false), (21, false), (9, false), (14, false), (16, false), (7, false), (6, false), (10, false), (3, false), (18, false), (5, false), (1, false), (12, false), (20, false), (15, false), (19, false)]);
        assert_eq!(list_of_boards[0].won, false);
    }

    #[test]
    fn test_generate_list_of_boards() {
        let instructions: Vec<_> = include_str!("../test_input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();


        // let mut list_of_boards = generate_list_of_boards(rows[1..].to_vec());
        let list_of_boards = generate_list_of_boards(instructions[1..].to_vec());
        // let drawn_numbers = get_drawn_numbers(rows.clone());
    }

    #[test]
    fn p04p1_outcome() {
        assert_eq!(solve_puzzle_04(false), 39902);
    }

    #[test]
    fn p04p1_sample_outcome() {
        assert_eq!(solve_puzzle_04_sample(false), 4512);
    }

    #[test]
    fn p04p2_outcome() {
        assert_eq!(solve_puzzle_04(true), 26936);
    }

    #[test]
    fn p04p2_sample_outcome() {
        assert_eq!(solve_puzzle_04_sample(true), 1924);
    }
}
