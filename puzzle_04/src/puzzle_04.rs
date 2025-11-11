use crate::board::Board;
use crate::board::Cell;
use crate::board::BOARD_DIMENSION;

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

            let mut board = Board {
                cells: Vec::new(),
                won: false,
            };

            for cells in rows {
                for cell in cells.split_whitespace() {
                    board.cells.push( Cell { number: cell.parse::<u64>().unwrap(), marked: false });
                }
            }

            boards.push(board);
        }
    }

    boards
}

fn solution(rows: &Vec<&str>, part_two: bool) -> i64 {
    let mut list_of_boards = generate_list_of_boards(rows[1..].to_vec());
    let drawn_numbers = get_drawn_numbers(rows.clone());

    if !part_two {
        for number in &drawn_numbers {
            for board in list_of_boards.iter_mut() {
                board.mark_number(*number);

                if board.has_winning_row_or_column(true) || board.has_winning_row_or_column(false) {
                    return (number * board.get_sum_of_unmarked_numbers()).try_into().unwrap();
                }
            }
        }
    }

    if part_two {
        let mut boards_in_winning_order = Vec::new();
        let list_of_boards_len = list_of_boards.len();
        let mut boards_in_winning_order_len = boards_in_winning_order.len();

        let mut board_placeholder = Board {
            cells: Vec::new(),
            won: false
        };

        for number in &drawn_numbers {
            for board in list_of_boards.iter_mut() {
                board.mark_number(*number);

                if !board.won &&
                    (board.has_winning_row_or_column(true) ||
                    board.has_winning_row_or_column(false)) {
                    boards_in_winning_order.push(board.clone());
                    boards_in_winning_order_len = boards_in_winning_order.len();

                    board_placeholder = board.clone();
                }
            }

            if list_of_boards_len == boards_in_winning_order.len() &&
                (board_placeholder.has_winning_row_or_column(true) ||
                board_placeholder.has_winning_row_or_column(false)) {
                return (number * boards_in_winning_order[boards_in_winning_order_len - 1].get_sum_of_unmarked_numbers()).try_into().unwrap()
            }
        }
    }

    -1
}

pub fn solve_puzzle_04(squid_must_win: bool) -> i64 {
    let instructions: Vec<_> = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(&instructions, squid_must_win)
}

pub fn solve_puzzle_04_sample(squid_must_win: bool) -> i64 {
    let instructions: Vec<_> = include_str!("../test_input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(&instructions, squid_must_win)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_list_of_boards() {
        let instructions: Vec<_> = include_str!("../test_input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let list_of_boards = generate_list_of_boards(instructions[1..].to_vec());

        let board_count = (instructions.len() - 1) / BOARD_DIMENSION;

        // first cell on first board:
        assert_eq!(list_of_boards[0].cells[0].number, 22);
        // last cell on last board:
        assert_eq!(list_of_boards[board_count - 1].cells[BOARD_DIMENSION.pow(2) - 1].number, 7);

        let instructions: Vec<_> = include_str!("../input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let list_of_boards = generate_list_of_boards(instructions[1..].to_vec());

        let board_count = (instructions.len() - 1) / BOARD_DIMENSION;

        // first cell on first board:
        assert_eq!(list_of_boards[0].cells[0].number, 91);
        // last cell on last board:
        assert_eq!(list_of_boards[board_count - 1].cells[BOARD_DIMENSION.pow(2) - 1].number, 33);
    }

    #[test]
    fn test_get_board_state() {
        let instructions: Vec<_> = include_str!("../test_input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let list_of_boards = generate_list_of_boards(instructions[1..].to_vec());

        for board in &list_of_boards {
            assert_eq!(board.won, false);
        }

        for board in list_of_boards {
            for e in board.cells {
                assert_eq!(e.marked, false);
            }
        }

        let instructions: Vec<_> = include_str!("../input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let list_of_boards = generate_list_of_boards(instructions[1..].to_vec());

        for board in &list_of_boards {
            assert_eq!(board.won, false);
        }

        for board in list_of_boards {
            for e in board.cells {
                assert_eq!(e.marked, false);
            }
        }
    }

    #[test]
    fn p04p1_get_sum_of_unmarked_numbers() {
        assert_eq!(solve_puzzle_04(false), 39902);
    }

    #[test]
    fn p04p1_sample_get_sum_of_unmarked_numbers() {
        assert_eq!(solve_puzzle_04_sample(false), 4512);
    }

    #[test]
    fn p04p2_get_sum_of_unmarked_numbers() {
        assert_eq!(solve_puzzle_04(true), 26936);
    }

    #[test]
    fn p04p2_sample_get_sum_of_unmarked_numbers() {
        assert_eq!(solve_puzzle_04_sample(true), 1924);
    }
}
