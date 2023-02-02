// use std::cell::Cell;
use std::collections::HashMap;
// use std::hash::Hash;

/*pub struct Cell {
    // pub item: HashMap<i8, bool>,
    pub number: i8,
    // pub number: usize,
    pub marked: bool
}*/

// #[derive(Copy, Clone, Debug)]
#[derive(Clone)]
// #[derive(Debug)]
pub struct Board {
    // pub cells: Vec<Cell>,
    pub cells: HashMap<u8, bool>,
    // pub test: HashMap<Cell>,
    pub won: bool
}

// const BOARD_DIMENSION: u8 = 5;
pub const BOARD_DIMENSION: usize = 5;

// pub trait Draw {
//     fn mark)
// }

// impl Draw for Board {
/*impl Cell {
    // impl Board {
    // fn toggle(&mut self) {
    fn toggle(&mut self) {
    // fn toggle(&mut self, cell: Cell) {
        // self.cells[0].marked = !self.cells[0].marked;

        // self.marked = !self.marked;
        self.item.se
        // self.cell.marked = !self.cell.marked;
    }
}*/

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
        // placeholder
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
        // if horizontal {
        // goal: step in sizes of 5 (the board square)
        //     // for n in 0..u8::pow(5, 2)
        //     // for n in 0..5.pow(2)
        //     // https://www.geeksforgeeks.org/rust-for-and-range/
        //     for e in self.cells.iter().enumerate().step_by(5) {
        //         if self.cells[] {
        //             self.won = true;
        //             return true;
        //         }
        //     }
        // } else {
        //     self.won = true;
        //     return true;
        // }

        let z = Vec::from_iter(self.cells.iter());

        if horizontal {
            // let z = &self.cells;
            // z.iter().map(|e| !e.1).
            // for n in z {
            // }


            // let z = &self.cells.

            // https://www.dotnetperls.com/convert-hashmap-vec-rust
            // let z = Vec::from_iter(self.cells.iter());

            // println!("{:?}", z[0]);

            // google: rust range steps
            // so: 27893223
            // for n in (0..5_u8.pow(2) + 1).step_by(5) {

            for n in (0..BOARD_DIMENSION.pow(2) + 1).step_by(BOARD_DIMENSION) {
            // for n in (0..BOARD_DIMENSION_u8.pow(2) + 1).step_by(BOARD_DIMENSION) {
                if *z[n as usize + 0].1 &&
                    *z[n as usize + 1].1 &&
                    *z[n as usize + 2].1 &&
                    *z[n as usize + 3].1 &&
                    *z[n as usize + 4].1 {
                    self.won = true;
                    // println!("self.won = {}", self.won);
                    return true;
                }
            }

        } else {
            // for n in 0..5 {
            for n in 0..BOARD_DIMENSION {
                // if *z[n as usize + 0 * 5].1 &&
                // *z[n as usize + 1 * 5].1 &&
                // *z[n as usize + 2 * 5].1 &&
                // *z[n as usize + 3 * 5].1 &&
                // *z[n as usize + 4 * 5].1 {
                    if *z[n as usize + 0 * BOARD_DIMENSION].1 &&
                        *z[n as usize + 1 * BOARD_DIMENSION].1 &&
                        *z[n as usize + 2 * BOARD_DIMENSION].1 &&
                        *z[n as usize + 3 * BOARD_DIMENSION].1 &&
                        *z[n as usize + 4 * BOARD_DIMENSION].1 {
                        self.won = true;
                        // println!("self.won = {}", self.won);
                        return true;
                    }
                }
            }

        // }

        // for n in (0.. 5_u8.pow(2) + 1).step_by(5) {
        //
        //     println!("{}", n);
        // }

        false // placeholder
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
