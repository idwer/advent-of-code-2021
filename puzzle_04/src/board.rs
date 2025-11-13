#[derive(Clone)]
pub struct Cell {
    pub number: u64,
    pub marked: bool,
}

#[derive(Clone)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub won: bool,
}

pub const BOARD_DIMENSION: usize = 5;

impl Board {
    pub fn sum_of_unmarked_numbers(&mut self) -> u64 {
        let mut sum = 0;

        for cell in &self.cells {
            if !cell.marked {
                sum += cell.number;
            }
        }

        sum
    }

    pub fn has_winning_row_or_column(&mut self, horizontal: bool) -> bool {
        let test = &self.cells;
        let cells = Vec::from_iter(test.iter());

        if horizontal {
            for n in (0..BOARD_DIMENSION.pow(2) + 1).step_by(BOARD_DIMENSION) {
                if n <= cells.len() - BOARD_DIMENSION {
                    if cells[n as usize + 0].marked
                    && cells[n as usize + 1].marked
                    && cells[n as usize + 2].marked
                    && cells[n as usize + 3].marked
                    && cells[n as usize + 4].marked
                    {
                        self.won = true;

                        return self.won
                    }
                }
            }
        } else {
            for n in 0..BOARD_DIMENSION {
                if cells[n as usize + 0 * BOARD_DIMENSION].marked
                && cells[n as usize + 1 * BOARD_DIMENSION].marked
                && cells[n as usize + 2 * BOARD_DIMENSION].marked
                && cells[n as usize + 3 * BOARD_DIMENSION].marked
                && cells[n as usize + 4 * BOARD_DIMENSION].marked
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
            if cell.number == number {
                let mut tmpcell = cell.clone();

                tmpcell.marked = !tmpcell.marked;

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
    fn test_sum_of_unmarked_numbers() {
        let cells: Vec<Cell> = Vec::new();

        let mut board = Board {
            cells: { cells.clone() },
            won: false,
        };

        let mut cells_writeonly = cells;

        cells_writeonly.push(Cell {number: 13, marked: true});
        cells_writeonly.push(Cell {number: 1, marked: false});
        cells_writeonly.push(Cell {number: 22, marked: false});
        cells_writeonly.push(Cell {number: 7, marked: true});
        cells_writeonly.push(Cell {number: 4, marked:false});

        board.cells = cells_writeonly;

        assert_eq!(board.sum_of_unmarked_numbers(), 27);
    }

    #[test]
    fn has_winning_row_or_column() {
        let cells: Vec<Cell> = Vec::new();

        let winning_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];

        let mut board = Board {
            cells: { cells.clone() },
            won: false,
        };

        let mut cells_writeonly = cells;

        // first 5x5 block found in the file test_input
        cells_writeonly.push(Cell { number: 22, marked: false });
        cells_writeonly.push(Cell { number: 13, marked: false });
        cells_writeonly.push(Cell { number: 17, marked:false });
        cells_writeonly.push(Cell { number: 11, marked:false });
        cells_writeonly.push(Cell { number: 0, marked:false });

        cells_writeonly.push(Cell { number: 8, marked:false });
        cells_writeonly.push(Cell { number: 2, marked: false });
        cells_writeonly.push(Cell { number: 23, marked: false });
        cells_writeonly.push(Cell { number: 4, marked: false });
        cells_writeonly.push(Cell { number: 24, marked: false });

        cells_writeonly.push(Cell { number: 21, marked: false });
        cells_writeonly.push(Cell { number: 9, marked: false });
        cells_writeonly.push(Cell { number: 14, marked: false });
        cells_writeonly.push(Cell { number: 16, marked: false });
        cells_writeonly.push(Cell { number: 7, marked: false });

        cells_writeonly.push(Cell { number: 6, marked: false });
        cells_writeonly.push(Cell { number: 10, marked: false });
        cells_writeonly.push(Cell { number: 3, marked: false });
        cells_writeonly.push(Cell { number: 18, marked: false });
        cells_writeonly.push(Cell { number: 5, marked: false });

        cells_writeonly.push(Cell { number: 1, marked: false });
        cells_writeonly.push(Cell { number: 12, marked: false });
        cells_writeonly.push(Cell { number: 20, marked: false });
        cells_writeonly.push(Cell { number: 15, marked: false });
        cells_writeonly.push(Cell { number: 19, marked: false });

        board.cells = cells_writeonly;

        for number in winning_numbers {
            board.mark_number(number);
        }

        assert_eq!(board.has_winning_row_or_column(false), true);
        assert_eq!(board.has_winning_row_or_column(true), true);
    }

    #[test]
    fn mark_number() {
        let cells: Vec<Cell> = Vec::new();

        let mut board = Board {
            cells: { cells.clone() },
            won: false,
        };

        let mut cells_writeonly = cells.clone();

        cells_writeonly.push(Cell { number: 13, marked: false });
        cells_writeonly.push(Cell { number: 1, marked: false });
        cells_writeonly.push(Cell { number: 2, marked: false });
        cells_writeonly.push(Cell { number: 7, marked: false });
        cells_writeonly.push(Cell { number: 4, marked: false });

        board.cells = cells_writeonly;

        board.mark_number(13);
        board.mark_number(7);

        assert_eq!(board.cells[0].marked, true);
        assert_eq!(board.cells[3].marked, true);
    }
}
