use std::collections::HashMap;

// #[derive(Copy, Clone, Debug)]
// #[derive(Clone, Debug)]
#[derive(Debug)]
pub struct Board {
    pub cells: HashMap<u8, bool>,
    pub won: bool,
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

        println!("board.rs:get_sum_of_unmarked_numbers () ret = {}", counter);
        counter as u64
    }

    pub fn mark_number(&mut self, number: u8) {
        for mut cell in self.cells.clone() {
            // println!("cell, 1/2: {:?}", cell);
            // help: consider dereferencing the borrow
            //     |
            // 54  |             if number == *cell.0 {
            //     |                          +
            if number != cell.0 {
                // println!("\n\nmark_number: miss {} \n\n", number);
            }
            if number == cell.0 {
                // 59 |                 cell.1 = true;
                //    |                 ------   ^^^^
                //    |                 |        |
                //    |                 |        expected `&bool`, found `bool`
                //    |                 |        help: consider borrowing here: `&true`
                //    |                 expected due to the type of this binding
                // cell.1 = &true;

                // *self.cells.get_mut(&number).unwrap() = true;
                *self.cells.get_mut(&number).unwrap() = !self.cells.get_key_value(&number).unwrap().1;
                // https://www.programiz.com/rust/hashmap
                // let _ = &self.cells.insert(number, true);
                // println!("{:?}", &self.cells.insert(number, true));
                // println!("{:?}", self.cells.get(&number));

                if cell.1 {
                    println!("\n\nmark_number: hit {} {:?}\n\n", number, &self.cells);
                    println!("\n\nmark_number: hit {} {:?}\n\n", number, cell);
                }
                // return;
            }
            // println!("cell, 2/2: {:?}", cell);
        }
        // println!("\n\nmark_number: self.cells = {:?}\n\n", self.cells);
    }

    pub fn has_winning_row_or_column(&mut self, horizontal: bool) -> bool {
        let list_of_cells = Vec::from_iter(self.cells.iter());

        // println!("list_of_cells = {:?}", list_of_cells);
        // println!("z.len() = {}", z.len());

        if horizontal {
            for n in (0..BOARD_DIMENSION.pow(2) + 1).step_by(BOARD_DIMENSION) {
                if n < list_of_cells.len() {
                    // println!("board.rs:55");
                    if *list_of_cells[n as usize + 0].1
                    && *list_of_cells[n as usize + 1].1
                    && *list_of_cells[n as usize + 2].1
                    && *list_of_cells[n as usize + 3].1
                    && *list_of_cells[n as usize + 4].1
                    {
                        println!("board.rs:has_winning_row_or_column() | horizontal: {} {} {} {} {} {}",
                            horizontal,
                                 list_of_cells[n as usize + 0].1,
                                 list_of_cells[n as usize + 1].1,
                                 list_of_cells[n as usize + 2].1,
                                 list_of_cells[n as usize + 3].1,
                                 list_of_cells[n as usize + 4].1,
                        );

                        self.won = true;
                        return true;
                    }
                }
            }
        } else {
            for n in 0..BOARD_DIMENSION {
                // println!("board.rs:69");
                if *list_of_cells[n as usize + 0 * BOARD_DIMENSION].1
                    && *list_of_cells[n as usize + 1 * BOARD_DIMENSION].1
                    && *list_of_cells[n as usize + 2 * BOARD_DIMENSION].1
                    && *list_of_cells[n as usize + 3 * BOARD_DIMENSION].1
                    && *list_of_cells[n as usize + 4 * BOARD_DIMENSION].1
                {
                    println!("board.rs:has_winning_row_or_column() | horizontal: {} {} {} {} {} {}",
                             horizontal,
                             list_of_cells[n as usize + 0].1,
                             list_of_cells[n as usize + 1].1,
                             list_of_cells[n as usize + 2].1,
                             list_of_cells[n as usize + 3].1,
                             list_of_cells[n as usize + 4].1,
                    );


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
