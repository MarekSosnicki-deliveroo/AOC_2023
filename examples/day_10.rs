use boolinator::Boolinator;
use itertools::Itertools;
use std::fs::read_to_string;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Hash)]
struct Cords {
    row: usize,
    column: usize,
}

impl Cords {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

fn main() {
    println!("Hello day 1!");
    let input = read_to_string("inputs/day_10/input").unwrap();

    //     let input = ".....\n\
    // .S-7.\n\
    // .|.|.\n\
    // .L-J.\n\
    // .....";

    //     let input = "-L|F7\n\
    // 7S-7|\n\
    // L|7||\n\
    // -L-J|\n\
    // L|-JF";

    //     let input = "..F7.\n\
    // .FJ|.\n\
    // SJ.L7\n\
    // |F--J\n\
    // LJ...";

    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    println!("{:?}", grid);

    let s_location = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    println!("S location is {:?}", s_location);

    let s_location = Cords::new(s_location.1, s_location.0);

    let mut neighbours = vec![];
    if s_location.row > 0 {
        neighbours.push(Cords::new(s_location.row - 1, s_location.column));
    }
    if s_location.column > 0 {
        neighbours.push(Cords::new(s_location.row, s_location.column - 1));
    }
    if s_location.row < grid.len() - 1 {
        neighbours.push(Cords::new(s_location.row + 1, s_location.column));
    }
    if s_location.column < grid[0].len() - 1 {
        neighbours.push(Cords::new(s_location.row, s_location.column + 1));
    }

    for neighbour in neighbours.iter() {
        let neighbour_value = grid[neighbour.row][neighbour.column];
        println!(
            "Starting from neighbour ({}, {}) with value {}",
            neighbour.row, neighbour.column, neighbour_value
        );
        let moved_once = move_by(s_location, *neighbour, neighbour_value);

        if let Some(moved_once) = moved_once {
            let mut current_position = *neighbour;
            let mut next_position = moved_once;

            let mut loop_length = 1;

            while grid[next_position.row][next_position.column] != 'S' {
                let next_position_value = grid[next_position.row][next_position.column];
                let after_move = move_by(current_position, next_position, next_position_value);

                if let Some(after_move) = after_move {
                    current_position = next_position;
                    next_position = after_move;
                    loop_length += 1;
                } else {
                    break;
                }
            }

            if grid[next_position.row][next_position.column] == 'S' {
                println!(
                    "Found Loop from neighbour ({}, {}) with length {}",
                    neighbour.row, neighbour.column, loop_length
                );

                println!("Result is {}", (loop_length + 1) / 2);
                return;
            }
        }
    }

    // println!("Result: {}", result);
}

fn move_by(start: Cords, cell: Cords, cell_value: char) -> Option<Cords> {
    (start.row == cell.row && start.column.abs_diff(cell.column) == 1
        || start.column == cell.column && start.row.abs_diff(cell.row) == 1)
        .as_option()?;

    println!(
        " *  Moving from ({}, {}) to ({}, {}) with value {}",
        start.row, start.column, cell.row, cell.column, cell_value
    );

    match cell_value {
        '|' => {
            if start.column == cell.column {
                if start.row < cell.row {
                    Some(Cords::new(cell.row + 1, cell.column))
                } else {
                    Some(Cords::new(cell.row - 1, cell.column))
                }
            } else {
                println!(
                    "Forbidden move | from ({}, {}) to ({}, {})",
                    start.row, start.column, cell.row, cell.column
                );
                None
            }
        }

        '-' => {
            if start.row == cell.row {
                if start.column < cell.column {
                    Some(Cords::new(cell.row, cell.column + 1))
                } else {
                    Some(Cords::new(cell.row, cell.column - 1))
                }
            } else {
                println!(
                    "Forbidden move - from ({}, {}) to ({}, {})",
                    start.row, start.column, cell.row, cell.column
                );
                None
            }
        }

        'L' => {
            if start.column == cell.column && start.row < cell.row {
                Some(Cords::new(cell.row, cell.column + 1))
            } else if start.row == cell.row && start.column > cell.column {
                Some(Cords::new(cell.row - 1, cell.column))
            } else {
                println!(
                    "Forbidden move L from ({}, {}) to ({}, {})",
                    start.row, start.column, cell.row, cell.column
                );
                None
            }
        }

        'J' => {
            if start.column == cell.column && start.row < cell.row {
                Some(Cords::new(cell.row, cell.column - 1))
            } else if start.row == cell.row && start.column < cell.column {
                Some(Cords::new(cell.row - 1, cell.column))
            } else {
                println!(
                    "Forbidden move J from ({}, {}) to ({}, {})",
                    start.row, start.column, cell.row, cell.column
                );
                None
            }
        }

        '7' => {
            if start.column == cell.column && start.row > cell.row {
                Some(Cords::new(cell.row, cell.column - 1))
            } else if start.row == cell.row && start.column < cell.column {
                Some(Cords::new(cell.row + 1, cell.column))
            } else {
                println!(
                    "Forbidden move J from ({}, {}) to ({}, {})",
                    start.row, start.column, cell.row, cell.column
                );
                None
            }
        }

        'F' => {
            if start.column == cell.column && start.row > cell.row {
                Some(Cords::new(cell.row, cell.column + 1))
            } else if start.row == cell.row && start.column > cell.column {
                Some(Cords::new(cell.row + 1, cell.column))
            } else {
                println!(
                    "Forbidden move F from ({}, {}) to ({}, {})",
                    start.row, start.column, cell.row, cell.column
                );
                None
            }
        }

        _ => {
            println!(
                "Forbidden move {} from ({}, {}) to ({}, {})",
                cell_value, start.row, start.column, cell.row, cell.column
            );
            None
        }
    }
}
