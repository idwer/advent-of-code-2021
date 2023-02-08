use std::collections::HashMap;

// #[derive(Copy, Clone, Debug)]
#[derive(Clone)]
// #[derive(Debug)]
pub struct Board {
    pub cells: HashMap<u8, bool>,
    pub won: bool
}

pub const BOARD_DIMENSION: usize = 5;

impl Board {
   pub fn get_sum_of_unmarked_numbers(&self) -> u64 {
       let mut counter = 0;

       for cell in &self.cells {
           if !cell.1 {
               counter += cell.0;
           }
       }

       counter as u64
    }

    pub fn mark_number(&self, number: u8) {
        for mut cell in &self.cells {
            // help: consider dereferencing the borrow
            //     |
            // 54  |             if number == *cell.0 {
            //     |                          +
                if number == *cell.0 {
                    // 59 |                 cell.1 = true;
                    //    |                 ------   ^^^^
                    //    |                 |        |
                    //    |                 |        expected `&bool`, found `bool`
                    //    |                 |        help: consider borrowing here: `&true`
                    //    |                 expected due to the type of this binding
                    cell.1 = &true;
                    // return;
            }
        }
    }

    pub fn has_winning_row_or_column(&mut self, horizontal: bool) -> bool {
        let z = Vec::from_iter(self.cells.iter());

        if horizontal {
            for n in (0..BOARD_DIMENSION.pow(2) + 1).step_by(BOARD_DIMENSION) {
                if n < z.len() {
                    if *z[n as usize + 0].1 &&
                        *z[n as usize + 1].1 &&
                        *z[n as usize + 2].1 &&
                        *z[n as usize + 3].1 &&
                        *z[n as usize + 4].1 {
                        self.won = true;
                        return true;
                    }
                }
            }

        } else {
            for n in 0..BOARD_DIMENSION {
                    if *z[n as usize + 0 * BOARD_DIMENSION].1 &&
                        *z[n as usize + 1 * BOARD_DIMENSION].1 &&
                        *z[n as usize + 2 * BOARD_DIMENSION].1 &&
                        *z[n as usize + 3 * BOARD_DIMENSION].1 &&
                        *z[n as usize + 4 * BOARD_DIMENSION].1 {
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
