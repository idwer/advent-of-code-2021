fn highest_value_from_input(input: Vec<&str>) -> usize {
    let mut dimension = 0;
    let mut maximum_values = Vec::<i64>::new();

    for line in input {
        for coordinates in line.split(" -> ") {
            let positions = coordinates.split(',').collect::<Vec<&str>>();

            if let Some(position) = positions.iter().max() {
                if let Ok(n) = position.parse::<i64>() {
                    maximum_values.push(n)
                }
            }
        }
    }

    if let Some(s) = maximum_values.iter().max() {
        dimension = *s as usize
    }

    dimension
}

fn generate_diagram(size: usize) -> Vec<Vec<u8>> {
    vec![vec![0; size]; size]
}

fn mark_diagram(diagram: &mut [Vec<u8>], input: Vec<&str>, part_two: bool) -> Vec<Vec<u8>> {
    let mut list_of_coordinates: Vec<_> = Vec::new();

    // split a line into two parts:
    // 0,9 -> 5,9 will build a list starting with 0,9 and 5,9
    for line in input {
        list_of_coordinates.push(line.split(" -> "));
    }

    for coordinate_to_split in list_of_coordinates {
        // horizontal start
        let mut x_lefthand = 0;
        // vertical start
        let mut y_lefthand = 0;
        // horizontal end
        let mut x_righthand = 0;
        // vertical end
        let mut y_righthand = 0;

        let mut _end = 0;
        let mut _start = 0;
        let mut _step = 0;

        // split every first line in two parts:
        // 0,9
        if let Some(coordinate) = coordinate_to_split.clone().nth(0) {
            match coordinate.split_once(',') {
                None => (),
                Some(coordinates) => {
                    x_lefthand = coordinates.0.parse::<i16>().unwrap();
                    y_lefthand = coordinates.1.parse::<i16>().unwrap()
                }
            }
        }

        // split every second line in two parts:
        // 5,9
        if let Some(coordinate) = coordinate_to_split.clone().nth(1) {
            match coordinate.split_once(',') {
                None => (),
                Some(coordinates) => {
                    x_righthand = coordinates.0.parse::<i16>().unwrap();
                    y_righthand = coordinates.1.parse::<i16>().unwrap()
                }
            }
        }

        // mark horizontal lines
        if let true = y_lefthand == y_righthand {
            _end = x_righthand;
            _start = x_lefthand;

            match x_lefthand < x_righthand {
                true => _step = 1,
                false => _step = -1,
            }

            match _step {
                -1 => {
                    for n in _end..=_start {
                        diagram[y_lefthand as usize][n as usize] += 1;
                    }
                }
                1 => {
                    for n in _start..=_end {
                        diagram[y_lefthand as usize][n as usize] += 1;
                    }
                }
                _ => (),
            }

            continue;
        }

        // mark vertical lines
        if let true = x_lefthand == x_righthand {
            _end = y_righthand;
            _start = y_lefthand;

            match y_lefthand < y_righthand {
                true => _step = 1,
                false => _step = -1,
            }

            match _step {
                -1 => {
                    for n in _end..=_start {
                        diagram[n as usize][x_lefthand as usize] += 1;
                    }
                }
                1 => {
                    for n in _start..=_end {
                        diagram[n as usize][x_lefthand as usize] += 1;
                    }
                }
                _ => (),
            }

            continue;
        }

        // mark diagonal lines
        if let true = part_two {
            let mut steps = 0;
            let mut step_method = 0;
            let mut x = x_lefthand;
            let mut y = y_lefthand;

            match x_lefthand > x_righthand && y_lefthand < y_righthand {
                true => {
                    steps = x_lefthand - x_righthand;
                    step_method = 1;
                }
                _ => (),
            }

            match x_lefthand > x_righthand && y_lefthand > y_righthand {
                true => {
                    steps = x_lefthand - x_righthand;
                    step_method = 2;
                }
                _ => (),
            }

            match x_lefthand < x_righthand && y_lefthand < y_righthand {
                true => {
                    steps = x_righthand - x_lefthand;
                    step_method = 3;
                }
                _ => (),
            }

            match x_lefthand < x_righthand && y_lefthand > y_righthand {
                true => {
                    steps = x_righthand - x_lefthand;
                    step_method = 4;
                }
                _ => (),
            }

            for _ in 0..=steps {
                diagram[y as usize][x as usize] += 1;

                match step_method {
                    1 => {
                        x -= 1;
                        y += 1
                    }
                    2 => {
                        x -= 1;
                        y -= 1
                    }
                    3 => {
                        x += 1;
                        y += 1
                    }
                    4 => {
                        x += 1;
                        y -= 1
                    }
                    _ => (),
                }
            }

            continue;
        }
    }

    diagram.to_vec()
}

fn parse_diagram(diagram: Vec<Vec<u8>>) -> i64 {
    let mut counter = 0;

    for row in diagram.iter() {
        for cell in row {
            if let 2.. = cell {
                counter += 1;
            }
        }
    }

    counter
}

fn solution(rows: Vec<&str>, part_two: bool) -> i64 {
    let diagram_size = 1 + highest_value_from_input(rows.clone());
    let mut diagram = generate_diagram(diagram_size);

    match part_two {
        false => diagram = mark_diagram(&mut diagram, rows.clone(), false),
        true => diagram = mark_diagram(&mut diagram, rows.clone(), true),
    }

    parse_diagram(diagram)
}

pub fn solve_puzzle_05(part_two: bool) -> i64 {
    let instructions: Vec<_> = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(instructions, part_two)
}

pub fn solve_puzzle_05_sample(part_two: bool) -> i64 {
    let instructions: Vec<_> = include_str!("../test_input")
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    solution(instructions, part_two)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagram_dimension_sample() {
        let instructions: Vec<_> = include_str!("../test_input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let max = highest_value_from_input(instructions);

        assert_eq!(max, 9);
    }

    #[test]
    fn test_diagram_dimension() {
        let instructions: Vec<_> = include_str!("../input")
            .lines()
            .filter(|l| !l.is_empty())
            .collect();

        let max = highest_value_from_input(instructions);
        assert_eq!(max, 990);
    }

    #[test]
    fn p05p1_sample() {
        assert_eq!(solve_puzzle_05_sample(false), 5);
    }

    #[test]
    fn p05p1() {
        assert_eq!(solve_puzzle_05(false), 7269);
    }

    #[test]
    fn p05p2_sample() {
        assert_eq!(solve_puzzle_05_sample(true), 12);
    }

    #[test]
    fn p05p2() {
        assert_eq!(solve_puzzle_05(true), 21140);
    }
}
