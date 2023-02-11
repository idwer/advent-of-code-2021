#[derive(Clone, Debug)]
pub struct Board {
    pub cells: Vec<(u64, bool)>,
    pub won: bool,
}

pub const BOARD_DIMENSION: usize = 5;

impl Board {
    pub fn get_sum_of_unmarked_numbers(&mut self) -> u64 {
        let mut counter = 0;

        // println!("get_sum_of_unmarked_numbers() | counter = {:?}", counter);

        for cell in &self.cells {
            // println!("board.rs:16 | get_sum_of_unmarked_numbers () cell = {:?}", cell);

            if !cell.1 {

                counter += cell.0;
            }
        }

        // println!("board.rs:get_sum_of_unmarked_numbers () ret = {}", counter);
        counter as u64
    }

    pub fn mark_number(&mut self, number: u64) {
        // println!("board.rs:mark_number() cells (before) = {:?}", &self.cells[0..13]);
        // println!("board.rs:mark_number() cells (before) = {:?}\n", &self.cells[13..]);

        let mut tmp = self.cells.clone();
        let mut index = 0;

        for cell in &self.cells {
            if cell.0 == number {
                let mut tmpcell = cell.clone();

                // println!("board.rs:mark_number() number = {:?} old = {:?} new = {:?}", number, tmpcell.1, !tmpcell.1);

                tmpcell.1 = !tmpcell.1;

                if tmpcell.1 {
                    // println!("board.rs:mark_number() number = {:?} cell.1 = {:?}", number, cell.1);
                }

                tmp[index] = tmpcell.clone();

                // if tmpcell.1 {
                //     println!("board.rs:mark_number() tmp (after) = {:?} ", tmp);
                //     println!("board.rs:mark_number() cells (after) = {:?} ", self.cells);
                //
                // }


                // panic!();
            }

            index += 1;
        }

        self.cells = tmp.clone();

        // println!("{}", self.cells.len());

        // println!("board.rs:mark_number() tmp (after) = {:?} ", tmp);
        // println!("board.rs:mark_number() cells (after) = {:?}", &self.cells[0..13]);
        // println!("board.rs:mark_number() cells (after) = {:?}\n", &self.cells[13..]);
        // panic!();
    }

    pub fn has_winning_row_or_column(&mut self, horizontal: bool) -> bool {
        let test = &self.cells;
        let list_of_cells = Vec::from_iter(test.iter());

        // println!("test: {:?}", test);
        // println!("list_of_cells: {:?}", list_of_cells);

        if horizontal {
            for n in (0..BOARD_DIMENSION.pow(2) + 1).step_by(BOARD_DIMENSION) {
                // println!("has_winning_row_or_column() | horizontal = {:?} | list_of_cells.len() = {:?} n = {:?} list_of_cells = {:?}",
                //          horizontal, list_of_cells.len(), n, list_of_cells);

                if n <= list_of_cells.len() - BOARD_DIMENSION {
                    // println!("board.rs:51");
                    // println!("board.rs:52 | has_winning_row_or_column() | len = {:?} n = {:?} | tuples: {:?} {:?} {:?} {:?} {:?}", list_of_cells.len(), n,
                    //          list_of_cells[n as usize + 0],
                    //          list_of_cells[n as usize + 1],
                    //          list_of_cells[n as usize + 2],
                    //          list_of_cells[n as usize + 3],
                    //          list_of_cells[n as usize + 4]
                    // );
                    // panic!("okee dag");
                    if list_of_cells[n as usize + 0].1
                    && list_of_cells[n as usize + 1].1
                    && list_of_cells[n as usize + 2].1
                    && list_of_cells[n as usize + 3].1
                    && list_of_cells[n as usize + 4].1
                    {
                        // println!("board.rs:97");
                        // println!("has_winning_row_or_column() | len = {:?} n = {:?} | tuples: {:?} {:?} {:?} {:?} {:?}", list_of_cells.len(), n,
                        //          list_of_cells[n as usize + 0],
                        //          list_of_cells[n as usize + 1],
                        //          list_of_cells[n as usize + 2],
                        //          list_of_cells[n as usize + 3],
                        //          list_of_cells[n as usize + 4]
                        // );

                        self.won = true;
                        return true;
                    }
                }
            }
        } else {
            for n in 0..BOARD_DIMENSION {
                // println!("has_winning_row_or_column() | horizontal = {:?} | list_of_cells.len() = {:?} n = {:?} list_of_cells = {:?}",
                //          horizontal, list_of_cells.len(), n, list_of_cells);
                if list_of_cells[n as usize + 0 * BOARD_DIMENSION].1
                && list_of_cells[n as usize + 1 * BOARD_DIMENSION].1
                && list_of_cells[n as usize + 2 * BOARD_DIMENSION].1
                && list_of_cells[n as usize + 3 * BOARD_DIMENSION].1
                && list_of_cells[n as usize + 4 * BOARD_DIMENSION].1
                {
                    // println!("board.rs:121");
                    // println!("board.rs:122 | has_winning_row_or_column() | len = {:?} n = {:?} | tuples: {:?} {:?} {:?} {:?} {:?}", list_of_cells.len(), n,
                    //          list_of_cells[n as usize + 0 * BOARD_DIMENSION],
                    //          list_of_cells[n as usize + 1 * BOARD_DIMENSION],
                    //          list_of_cells[n as usize + 2 * BOARD_DIMENSION],
                    //          list_of_cells[n as usize + 3 * BOARD_DIMENSION],
                    //          list_of_cells[n as usize + 4 * BOARD_DIMENSION]
                    // );

                    self.won = true;
                    return true;
                }
            }
        }

        false
    }

    // todo: return vector
    // todo: what datatype/struct is rows?
    // rows is a list of rows that represent the 5x5 block within the (test) input
    /*    fn parse_list(&self, rows: Vec<_>) -> Board {
    // fn parse_list(&self, rows: Vec<_>) -> Vec<_> {
    //     Vec::<_>::new() // placeholder
        Board {
            cells: Vec::from()  Cell {
                marked: false,
                number: 13
            },
            won: false
        },
    }*/

    // a/the constructor ( __init__() ) goes here?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_vec_was_updated() {
        let mut cells: Vec<(u64, bool)> = Vec::new();

        let mut board = Board {
            cells: { cells.clone() },
            won: false,
        };

        let mut tmp = cells.clone();

        tmp.push((13, false));
        // cells.push((13, false));
        tmp.push((1, false));
        tmp.push((2, false));
        tmp.push((7, false));
        tmp.push((4, false));

        board.cells = tmp;

        board.mark_number(13);
        board.mark_number(7);

        assert_eq!(board.cells[0].1, true);
        assert_eq!(board.cells[3].1, true);
        assert_eq!(board.cells[4].1, false);
    }

    #[test]
    fn calculate_correct_sum_of_unmarked_numbers() {
        let mut cells: Vec<(u64, bool)> = Vec::new();

        let mut board = Board {
            cells: { cells.clone() },
            won: false,
        };

        let mut tmp = cells;

        tmp.push((13, true));
        tmp.push((1, false));
        tmp.push((22, false));
        tmp.push((7, true));
        tmp.push((4, false));

        board.cells = tmp;

        assert_eq!(board.get_sum_of_unmarked_numbers(), 27);
    }

    #[test]
    fn board_can_mark_winning_col() {
        let mut cells: Vec<(u64, bool)> = Vec::new();
        let winning_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ]; // Vec::from(7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1);
        // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]
        // let x = winning_numbers.clone().sort();
        // println!("{:?}", x);

        let mut board = Board {
            cells: { cells.clone() },
            won: false,
        };

        let mut tmp = cells;

        tmp.push((22, false));
        tmp.push((13, false));
        tmp.push((17, false));
        tmp.push((11, false));
        tmp.push((0, false));

        tmp.push((8, false));
        tmp.push((2, false));
        tmp.push((23, false));
        tmp.push((4, false));
        tmp.push((24, false));

        tmp.push((21, false));
        tmp.push((9, false));
        tmp.push((14, false));
        tmp.push((16, false));
        tmp.push((7, false));

        tmp.push((6, false));
        tmp.push((10, false));
        tmp.push((3, false));
        tmp.push((18, false));
        tmp.push((5, false));

        tmp.push((1, false));
        tmp.push((12, false));
        tmp.push((20, false));
        tmp.push((15, false));
        tmp.push((19, false));

        board.cells = tmp;

        for n in winning_numbers {
            board.mark_number(n);
        }

        assert_eq!(board.has_winning_row_or_column(false), true);
    }

    #[test]
    fn board_can_mark_winning_row() {
        let mut cells: Vec<(u64, bool)> = Vec::new();
        let winning_numbers = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ]; // Vec::from(7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1);

        let mut board = Board {
            cells: { cells.clone() },
            won: false,
        };

        let mut tmp_cells = cells;

        tmp_cells.push((22, false));
        tmp_cells.push((13, false));
        tmp_cells.push((17, false));
        tmp_cells.push((11, false));
        tmp_cells.push((0, false));
        tmp_cells.push((8, false));
        tmp_cells.push((2, false));
        tmp_cells.push((23, false));
        tmp_cells.push((4, false));
        tmp_cells.push((24, false));
        tmp_cells.push((21, false));
        tmp_cells.push((9, false));
        tmp_cells.push((14, false));
        tmp_cells.push((16, false));
        tmp_cells.push((7, false));
        tmp_cells.push((6, false));
        tmp_cells.push((10, false));
        tmp_cells.push((3, false));
        tmp_cells.push((18, false));
        tmp_cells.push((5, false));
        tmp_cells.push((1, false));
        tmp_cells.push((12, false));
        tmp_cells.push((20, false));
        tmp_cells.push((15, false));
        tmp_cells.push((19, false));


        board.cells = tmp_cells;

        for n in winning_numbers {
            board.mark_number(n);
        }

        assert_eq!(board.has_winning_row_or_column(true), true);
    }
}