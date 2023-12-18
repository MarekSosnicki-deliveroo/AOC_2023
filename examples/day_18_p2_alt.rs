use itertools::Itertools;
use std::fs::read_to_string;

/*

--- Part Two ---
The Elves were right to be concerned; the planned lagoon would be much too small.

After a few minutes, someone realizes what happened; someone swapped the color and instruction parameters when producing the dig plan. They don't have time to fix the bug; one of them asks if you can extract the correct instructions from the hexadecimal codes.

Each hexadecimal code is six hexadecimal digits long. The first five hexadecimal digits encode the distance in meters as a five-digit hexadecimal number. The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.

So, in the above example, the hexadecimal codes can be converted into the true instructions:

#70c710 = R 461937
#0dc571 = D 56407
#5713f0 = R 356671
#d2c081 = D 863240
#59c680 = R 367720
#411b91 = D 266681
#8ceee2 = L 577262
#caa173 = U 829975G
#1b58a2 = L 112010
#caa171 = D 829975
#7807d2 = L 491645
#a77fa3 = U 686074
#015232 = L 5411
#7a21e3 = U 500254
 */
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    row: i64,
    column: i64,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct UPoint {
    row: usize,
    column: usize,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct VerticalLine {
    column: i64,
    row_from: i64,
    row_to: i64,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct HorizontalLine {
    row: i64,
    column_from: i64,
    column_to: i64,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct CurrentFilling {
    row_start: i64,
    column_start: i64,
    column_end: i64,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, sscanf::FromScanf)]
enum Direction {
    #[sscanf("3")]
    Up,
    #[sscanf("1")]
    Down,
    #[sscanf("2")]
    Left,
    #[sscanf("0")]
    Right,
}

impl Point {
    fn move_with(&self, direction: Direction, steps: i64) -> Point {
        let (new_row, new_column) = match direction {
            Direction::Up => (self.row - steps, self.column),
            Direction::Down => (self.row + steps, self.column),
            Direction::Left => (self.row, self.column - steps),
            Direction::Right => (self.row, self.column + steps),
        };

        Point {
            row: new_row,
            column: new_column,
        }
    }
}

fn main() {
    println!("Hello day 18!");
    let input = read_to_string("inputs/day_18/input").unwrap();

    // let input = "R 6 (#70c710)\n\
    // D 5 (#0dc571)\n\
    // L 2 (#5713f0)\n\
    // D 2 (#d2c081)\n\
    // R 2 (#59c680)\n\
    // D 2 (#411b91)\n\
    // L 5 (#8ceee2)\n\
    // U 2 (#caa173)\n\
    // L 1 (#1b58a2)\n\
    // U 2 (#caa171)\n\
    // R 2 (#7807d2)\n\
    // U 3 (#a77fa3)\n\
    // L 2 (#015232)\n\
    // U 2 (#7a21e3)";

    let mut corner_points = vec![Point { row: 0, column: 0 }];

    let mut current_point: Point = Point { row: 0, column: 0 };

    let use_v1_reading = false;

    let mut trenches_dig_area = 0;

    input.lines().for_each(|line| {
        let (dir_str, steps_old, steps, direction) =
            sscanf::scanf!(line, "{str} {i64} (#{i64:x}{Direction})").unwrap();

        let direction = if use_v1_reading {
            match dir_str {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction {}", dir_str),
            }
        } else {
            direction
        };

        let steps = if use_v1_reading { steps_old } else { steps };
        println!("Applying {steps} in direction {direction:?}");
        let new_point = current_point.move_with(direction, steps);

        println!("New point {:?}", new_point);

        trenches_dig_area += steps;

        corner_points.push(new_point);
        current_point = new_point;
    });

    println!("Current points are {:?}", corner_points);

    let row_values = corner_points
        .iter()
        .map(|point| point.row)
        .flat_map(|point| [point - 1, point, point + 1])
        .sorted()
        .unique()
        .collect_vec();

    let column_values = corner_points
        .iter()
        .map(|point| point.column)
        .flat_map(|point| [point - 1, point, point + 1])
        .sorted()
        .unique()
        .collect_vec();

    println!("Row values are {:?}", row_values);
    println!("Column values are {:?}", column_values);

    let column_size = column_values.len() + 2;
    let row_size = row_values.len() + 2;
    let mut trenches_grid = vec![vec![false; column_size]; row_size];

    let mut current_point: Point = Point { row: 0, column: 0 };
    let mut current_point_row_value_position = row_values
        .iter()
        .position(|&r| r == current_point.row)
        .unwrap()
        + 1;

    let mut current_point_column_value_position = column_values
        .iter()
        .position(|&c| c == current_point.column)
        .unwrap()
        + 1;

    input.lines().for_each(|line| {
        let (dir_str, steps_old, steps, direction) =
            sscanf::scanf!(line, "{str} {i64} (#{i64:x}{Direction})").unwrap();

        let direction = if use_v1_reading {
            match dir_str {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction {}", dir_str),
            }
        } else {
            direction
        };

        let steps = if use_v1_reading { steps_old } else { steps };
        println!("Applying {steps} in direction {direction:?}");
        let new_point = current_point.move_with(direction, steps);

        let new_point_row_value_position =
            row_values.iter().position(|&r| r == new_point.row).unwrap() + 1;
        let new_point_column_value_position = column_values
            .iter()
            .position(|&c| c == new_point.column)
            .unwrap()
            + 1;

        match direction {
            Direction::Up => {
                for row in new_point_row_value_position..=current_point_row_value_position {
                    trenches_grid[row][current_point_column_value_position] = true;
                }
            }
            Direction::Down => {
                for row in current_point_row_value_position..=new_point_row_value_position {
                    trenches_grid[row][current_point_column_value_position] = true;
                }
            }
            Direction::Left => {
                for column in new_point_column_value_position..=current_point_column_value_position
                {
                    trenches_grid[current_point_row_value_position][column] = true;
                }
            }
            Direction::Right => {
                for column in current_point_column_value_position..=new_point_column_value_position
                {
                    trenches_grid[current_point_row_value_position][column] = true;
                }
            }
        }

        println!("New point {:?}", new_point);
        current_point = new_point;
        current_point_column_value_position = new_point_column_value_position;
        current_point_row_value_position = new_point_row_value_position;
    });

    println!("Trenches grid:");
    println!(
        "{}",
        trenches_grid
            .iter()
            .map(|row| row
                .iter()
                .map(|v| if *v { '#' } else { '.' })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );

    // let mut dig_grid = trenches_grid.clone();
    //
    // for row in dig_grid.iter_mut() {
    //     let mut previous_was_trench = false;
    //     let mut is_inside_trenches = false;
    //     for cell in row.iter_mut() {
    //         let is_trench = *cell;
    //
    //         if previous_was_trench && !is_trench {
    //             is_inside_trenches = !is_inside_trenches;
    //         }
    //
    //         if is_inside_trenches {
    //             *cell = true;
    //         }
    //
    //         previous_was_trench = is_trench;
    //     }
    // }

    let mut flood_grid = vec![vec![false; column_size]; row_size];
    let mut to_visit = vec![UPoint { row: 0, column: 0 }];

    while let Some(current) = to_visit.pop() {
        if flood_grid[current.row][current.column] || trenches_grid[current.row][current.column] {
            continue;
        }
        flood_grid[current.row][current.column] = true;

        if current.row > 0 {
            to_visit.push(UPoint {
                row: current.row - 1,
                column: current.column,
            })
        }

        if current.column > 0 {
            to_visit.push(UPoint {
                row: current.row,
                column: current.column - 1,
            })
        }

        if current.row < flood_grid.len() - 1 {
            to_visit.push(UPoint {
                row: current.row + 1,
                column: current.column,
            })
        }

        if current.column < flood_grid[0].len() - 1 {
            to_visit.push(UPoint {
                row: current.row,
                column: current.column + 1,
            })
        }
    }

    println!("Flood grid:");
    println!(
        "{}",
        flood_grid
            .iter()
            .map(|row| row
                .iter()
                .map(|v| if *v { '#' } else { '.' })
                .collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    );

    let mut dig_area = 0;

    for row in 1..(flood_grid.len() - 1) {
        for column in 1..(flood_grid[0].len() - 1) {
            if flood_grid[row][column] {
                continue;
            } else {
                let current_row_value = row_values[row - 1];
                let current_column_value = column_values[column - 1];
                let next_row_value = row_values[row];
                let next_column_value = column_values[column];

                // println!("Adding flood grid at position {row} {column} with values {current_row_value} {current_column_value} {next_row_value} {next_column_value}");

                dig_area += (next_row_value - current_row_value)
                    * (next_column_value - current_column_value);
            }
        }
    }

    println!("Result: {}", dig_area);
}
