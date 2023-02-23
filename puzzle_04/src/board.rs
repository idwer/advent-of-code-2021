#[derive(Clone, Debug)]
pub struct Board {
    pub cells: Vec<(u64, bool)>,
    pub won: bool,
}

pub const BOARD_DIMENSION: usize = 5;

impl Board {
    pub fn get_sum_of_unmarked_numbers(&mut self) -> u64 {
        let mut sum = 0;

        for cell in &self.cells {
            if !cell.1 {
                sum += cell.0;
            }
        }

        sum as u64
    }

    pub fn has_winning_row_or_column(&mut self, horizontal: bool) -> bool {
        let test = &self.cells;
        let list_of_cells = Vec::from_iter(test.iter());

        if horizontal {
            for n in (0..BOARD_DIMENSION.pow(2) + 1).step_by(BOARD_DIMENSION) {
                if n <= list_of_cells.len() - BOARD_DIMENSION {
                    if list_of_cells[n as usize + 0].1
                    && list_of_cells[n as usize + 1].1
                    && list_of_cells[n as usize + 2].1
                    && list_of_cells[n as usize + 3].1
                    && list_of_cells[n as usize + 4].1
                    {
                        self.won = true;
                        return self.won
                    }
                }
            }
        } else {
            for n in 0..BOARD_DIMENSION {
                if list_of_cells[n as usize + 0 * BOARD_DIMENSION].1
                && list_of_cells[n as usize + 1 * BOARD_DIMENSION].1
                && list_of_cells[n as usize + 2 * BOARD_DIMENSION].1
                && list_of_cells[n as usize + 3 * BOARD_DIMENSION].1
                && list_of_cells[n as usize + 4 * BOARD_DIMENSION].1
                {
                    self.won = true;
                    return self.won
                }
            }
        }

        false
    }

    pub fn mark_number(&mut self, number: u64) {
        let mut tmp = self.cells.clone();
        let mut index = 0;

        for cell in &self.cells {
            if cell.0 == number {
                let mut tmpcell = cell.clone();

                tmpcell.1 = !tmpcell.1;

                tmp[index] = tmpcell.clone();
            }

            index += 1;
        }

        self.cells = tmp.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sum_of_unmarked_numbers() {
        let list_of_cells: Vec<(u64, bool)> = Vec::new();

        let mut board = Board {
            cells: { list_of_cells.clone() },
            won: false,
        };

        let mut list_of_cells_writeonly = list_of_cells;

        list_of_cells_writeonly.push((13, true));
        list_of_cells_writeonly.push((1, false));
        list_of_cells_writeonly.push((22, false));
        list_of_cells_writeonly.push((7, true));
        list_of_cells_writeonly.push((4, false));

        board.cells = list_of_cells_writeonly;

        assert_eq!(board.get_sum_of_unmarked_numbers(), 27);
    }

    #[test]
    fn has_winning_row_or_column() {
        let list_of_cells: Vec<(u64, bool)> = Vec::new();
        let winning_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let mut board = Board {
            cells: { list_of_cells.clone() },
            won: false,
        };

        let mut list_of_cells_writeonly = list_of_cells;

        // first 5x5 block found in the file test_input
        list_of_cells_writeonly.push((22, false));
        list_of_cells_writeonly.push((13, false));
        list_of_cells_writeonly.push((17, false));
        list_of_cells_writeonly.push((11, false));
        list_of_cells_writeonly.push((0, false));

        list_of_cells_writeonly.push((8, false));
        list_of_cells_writeonly.push((2, false));
        list_of_cells_writeonly.push((23, false));
        list_of_cells_writeonly.push((4, false));
        list_of_cells_writeonly.push((24, false));

        list_of_cells_writeonly.push((21, false));
        list_of_cells_writeonly.push((9, false));
        list_of_cells_writeonly.push((14, false));
        list_of_cells_writeonly.push((16, false));
        list_of_cells_writeonly.push((7, false));

        list_of_cells_writeonly.push((6, false));
        list_of_cells_writeonly.push((10, false));
        list_of_cells_writeonly.push((3, false));
        list_of_cells_writeonly.push((18, false));
        list_of_cells_writeonly.push((5, false));

        list_of_cells_writeonly.push((1, false));
        list_of_cells_writeonly.push((12, false));
        list_of_cells_writeonly.push((20, false));
        list_of_cells_writeonly.push((15, false));
        list_of_cells_writeonly.push((19, false));

        board.cells = list_of_cells_writeonly;

        for number in winning_numbers {
            board.mark_number(number);
        }

        assert_eq!(board.has_winning_row_or_column(false), true);
        assert_eq!(board.has_winning_row_or_column(true), true);
    }

    #[test]
    fn mark_number() {
        let list_of_cells: Vec<(u64, bool)> = Vec::new();

        let mut board = Board {
            cells: { list_of_cells.clone() },
            won: false,
        };

        let mut list_of_cells_writeonly = list_of_cells.clone();

        list_of_cells_writeonly.push((13, false));
        list_of_cells_writeonly.push((1, false));
        list_of_cells_writeonly.push((2, false));
        list_of_cells_writeonly.push((7, false));
        list_of_cells_writeonly.push((4, false));

        board.cells = list_of_cells_writeonly;

        board.mark_number(13);
        board.mark_number(7);

        assert_eq!(board.cells[0].1, true);
        assert_eq!(board.cells[3].1, true);
    }
}