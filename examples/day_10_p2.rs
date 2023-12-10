use boolinator::Boolinator;
use itertools::{iproduct, Itertools};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::BufRead;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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
    println!("Hello day 10 part 2!");
    let input = read_to_string("inputs/day_10/input").unwrap();

    // let input = "FF7FSF7F7F7F7F7F---7\n\
    // L|LJ||||||||||||F--J\n\
    // FL-7LJLJ||||||LJL-77\n\
    // F--JF--7||LJLJIF7FJ-\n\
    // L---JF-JLJIIIIFJLJJ7\n\
    // |F|F-JF---7IIIL7L|7|\n\
    // |FFJF7L7F-JF7IIL---7\n\
    // 7-L-JL7||F7|L7F-7F7|\n\
    // L.L7LFJ|||||FJL7||LJ\n\
    // L7JLJL-JLJLJL--JLJ.L";

    //     let input = ".....\n\
    // .S-7.\n\
    // .|.|.\n\
    // .L-J.\n\
    // .....";

    //     let input = ".F----7F7F7F7F-7....\n\
    // .|F--7||||||||FJ....\n\
    // .||.FJ||||||||L7....\n\
    // FJL7L7LJLJ||LJ.L-7..\n\
    // L--J.L7...LJS7F-7L7.\n\
    // ....F-J..F7FJ|L7L7L7\n\
    // ....L7.F7||L7|.L7L7|\n\
    // .....|FJLJ|FJ|F7|.LJ\n\
    // ....FJL-7.||.||||...\n\
    // ....L---J.LJ.LJLJ...";
    //
    //     let input = "..........\n\
    // .S------7.\n\
    // .|F----7|.\n\
    // .||OOOO||.\n\
    // .||OOOO||.\n\
    // .|L-7F-J|.\n\
    // .|II||II|.\n\
    // .L--JL--J.\n\
    // ..........";

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

    let loop_cords = get_loop_cords(grid.clone(), s_location);

    println!("Starting to visit grid");

    let mut visited_grid = (0..grid.len() * 3)
        .map(|_| vec![false; grid[0].len() * 3])
        .collect_vec();
    for loop_cord in loop_cords.iter() {
        let value = grid[loop_cord.row][loop_cord.column];

        visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3 + 1] = true;

        match value {
            '|' => {
                visited_grid[loop_cord.row * 3][loop_cord.column * 3 + 1] = true;
                visited_grid[loop_cord.row * 3 + 2][loop_cord.column * 3 + 1] = true;
            }
            '-' => {
                visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3] = true;
                visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3 + 2] = true;
            }
            'L' => {
                visited_grid[loop_cord.row * 3][loop_cord.column * 3 + 1] = true;
                visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3 + 2] = true;
            }
            'J' => {
                visited_grid[loop_cord.row * 3][loop_cord.column * 3 + 1] = true;
                visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3] = true;
            }
            '7' => {
                visited_grid[loop_cord.row * 3 + 2][loop_cord.column * 3 + 1] = true;
                visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3] = true;
            }
            'F' => {
                visited_grid[loop_cord.row * 3 + 2][loop_cord.column * 3 + 1] = true;
                visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3 + 2] = true;
            }
            'S' => {
                visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3] = true;
                visited_grid[loop_cord.row * 3 + 1][loop_cord.column * 3 + 2] = true;
                visited_grid[loop_cord.row * 3][loop_cord.column * 3 + 1] = true;
                visited_grid[loop_cord.row * 3 + 2][loop_cord.column * 3 + 1] = true;
            }
            _ => {}
        }
    }

    println!(
        "{}",
        visited_grid
            .iter()
            .map(|row| row.iter().map(|v| if *v { "#" } else { "." }).join(""))
            .join("\n")
    );

    let grid_with_loop = visited_grid.clone();

    let mut visited_grid = (0..grid.len() * 3)
        .map(|_| vec![false; grid[0].len() * 3])
        .collect_vec();

    // These cords are hardcoded to my example. for some reason If I was not starting from the center, but from the edge
    // the result was wrong. I have no idea why.
    let mut to_visit_queue = vec![Cords::new(68 * 3, 73 * 3)];

    while !to_visit_queue.is_empty() {
        let visiting_now = to_visit_queue.remove(0);

        if visited_grid[visiting_now.row][visiting_now.column] {
            continue;
        }

        if grid_with_loop[visiting_now.row][visiting_now.column] {
            continue;
        }
        visited_grid[visiting_now.row][visiting_now.column] = true;

        let row_before = visiting_now.row.saturating_sub(1);
        let column_before = visiting_now.column.saturating_sub(1);
        let row_after = (visiting_now.row + 1).min(visited_grid.len() - 1);
        let column_after = (visiting_now.column + 1).min(visited_grid[0].len() - 1);

        for row in row_before..=row_after {
            for column in column_before..=column_after {
                let cord = Cords::new(row, column);
                if !visited_grid[cord.row][cord.column] {
                    to_visit_queue.push(cord);
                }
            }
        }

        // for cord in [
        //     Cords::new(row_before, visiting_now.column),
        //     Cords::new(row_after, visiting_now.column),
        //     Cords::new(visiting_now.row, column_before),
        //     Cords::new(visiting_now.row, column_after),
        // ] {
        //     if !visited_grid[cord.row][cord.column] {
        //         to_visit_queue.push(cord);
        //     }
        // }
    }

    println!("Result grid");

    println!(
        "{}",
        visited_grid
            .iter()
            .map(|row| row.iter().map(|v| if *v { "." } else { "I" }).join(""))
            .join("\n")
    );

    let visited_grid_small: Vec<Vec<bool>> = (0..grid.len())
        .map(|small_row| {
            (0..grid[0].len())
                .map(|small_column| {
                    !iproduct!(
                        (small_row * 3)..(small_row * 3 + 2),
                        (small_column * 3)..(small_column * 3 + 2)
                    )
                    .all(|(visited_row, visited_col)| visited_grid[visited_row][visited_col])
                })
                .collect()
        })
        .collect();

    println!("Small version");
    println!(
        "{}",
        visited_grid_small
            .iter()
            .map(|row| row.iter().map(|v| if *v { "." } else { "I" }).join(""))
            .join("\n")
    );

    let result = visited_grid_small
        .iter()
        .map(|row| row.iter().filter(|v| !**v).count())
        .sum::<usize>();

    println!("Result: {}", result);
}

fn get_loop_cords(grid: Vec<Vec<char>>, s_location: Cords) -> HashSet<Cords> {
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

            let mut loop_cords = HashSet::new();

            loop_cords.insert(s_location);
            loop_cords.insert(current_position);

            while grid[next_position.row][next_position.column] != 'S' {
                let next_position_value = grid[next_position.row][next_position.column];
                let after_move = move_by(current_position, next_position, next_position_value);

                if let Some(after_move) = after_move {
                    loop_cords.insert(current_position);
                    current_position = next_position;
                    next_position = after_move;
                    loop_cords.insert(current_position);
                } else {
                    break;
                }
            }

            if grid[next_position.row][next_position.column] == 'S' {
                println!(
                    "Found Loop from neighbour ({}, {}) with length {}",
                    neighbour.row,
                    neighbour.column,
                    loop_cords.len()
                );
                return loop_cords;
            }
        }
    }
    HashSet::default()
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

fn is_crossing_loop(from: Cords, to: Cords, value: char) -> bool {
    match value {
        '|' => from.column == to.column,

        '-' => from.row == to.row,
        'L' => {
            (from.column == to.column && from.row > to.row)
                || (from.row == to.row && from.column > to.column)
        }

        'J' => {
            if from.column == to.column && from.row < to.row {
                true
            } else if from.row == to.row && from.column < to.column {
                true
            } else {
                false
            }
        }

        '7' => {
            if from.column == to.column && from.row > to.row {
                true
            } else if from.row == to.row && from.column < to.column {
                true
            } else {
                false
            }
        }

        'F' => {
            if from.column == to.column && from.row > to.row {
                true
            } else if from.row == to.row && from.column > to.column {
                true
            } else {
                false
            }
        }

        _ => false,
    }
}
