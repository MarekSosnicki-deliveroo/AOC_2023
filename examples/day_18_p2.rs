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

    let input = "R 6 (#70c710)\n\
    D 5 (#0dc571)\n\
    L 2 (#5713f0)\n\
    D 2 (#d2c081)\n\
    R 2 (#59c680)\n\
    D 2 (#411b91)\n\
    L 5 (#8ceee2)\n\
    U 2 (#caa173)\n\
    L 1 (#1b58a2)\n\
    U 2 (#caa171)\n\
    R 2 (#7807d2)\n\
    U 3 (#a77fa3)\n\
    L 2 (#015232)\n\
    U 2 (#7a21e3)";

    let mut corner_points = vec![Point { row: 0, column: 0 }];

    let mut veritcal_lines: Vec<VerticalLine> = Vec::default();
    let mut horizontal_lines: Vec<HorizontalLine> = Vec::default();

    let mut current_point: Point = Point { row: 0, column: 0 };

    let use_v1_reading = true;

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

        match direction {
            Direction::Left | Direction::Right => horizontal_lines.push(HorizontalLine {
                row: current_point.row,
                column_from: current_point.column.min(new_point.column),
                column_to: current_point.column.max(new_point.column),
            }),
            _ => {}
        }

        corner_points.push(new_point);
        current_point = new_point;
    });

    println!("Corner points: {:?}", corner_points);

    horizontal_lines.sort_by_key(|line| (line.row, line.column_from));
    veritcal_lines.sort_by_key(|line| line.column);

    println!("Vertical lines {:?}", veritcal_lines);
    println!("Horizontal lines {:?}", horizontal_lines);

    let highest_row = horizontal_lines.first().unwrap().row;
    let lowest_row = horizontal_lines.last().unwrap().row;

    println!("Highest row {}", highest_row);
    println!("Lowest row {}", lowest_row);

    let mut current_row = highest_row;

    let mut current_filling_columns: Vec<CurrentFilling> = vec![];

    let mut digged_area: i64 = 0;

    loop {
        // find all trenches ranges in this row

        println!("===");
        println!("Current row {}", current_row);

        let all_horizontal_lines_on_this_row = horizontal_lines
            .iter()
            .filter(|line| line.row == current_row)
            .collect_vec();

        println!(
            "Found horizontal lines {:?}",
            all_horizontal_lines_on_this_row
        );

        println!("Current fillings are {:?}", current_filling_columns);
        println!("Dig area {:?}", digged_area);

        for current_filling in current_filling_columns.iter_mut() {
            digged_area += (current_filling.column_end - current_filling.column_start + 1)
                * (current_row - current_filling.row_start + 1);
            current_filling.row_start = current_row + 1;
        }

        println!("Dig area after {:?}", digged_area);

        for line in all_horizontal_lines_on_this_row {
            let mut used_ranges = vec![];

            while let Some(current_filling_that_is_intersecting_position) = current_filling_columns
                .iter()
                .position(|current_filling: &CurrentFilling| {
                    (current_filling.column_start >= line.column_from
                        && current_filling.column_start < line.column_to)
                        || (current_filling.column_end > line.column_from
                            && current_filling.column_end <= line.column_to)
                })
            {
                let found_filling: CurrentFilling =
                    current_filling_columns.remove(current_filling_that_is_intersecting_position);
                // filling inside the line
                let fill_column_start = found_filling.column_start.max(line.column_from);
                let fill_column_end = found_filling.column_end.min(line.column_to);

                if fill_column_start > found_filling.column_start {
                    current_filling_columns.push(CurrentFilling {
                        row_start: found_filling.row_start,
                        column_start: found_filling.column_start,
                        column_end: fill_column_start,
                    });
                }

                if fill_column_end < found_filling.column_end {
                    current_filling_columns.push(CurrentFilling {
                        row_start: found_filling.row_start,
                        column_start: fill_column_end,
                        column_end: found_filling.column_end,
                    });
                }
                used_ranges.push((fill_column_start, fill_column_end));
            }

            used_ranges.sort_by_key(|(c1, c2)| *c1);

            let mut current_range_start = line.column_from;

            println!("Used ranges {:?}", used_ranges);

            while let Some((used_range_start, used_range_end)) = used_ranges.pop() {
                if used_range_start > current_range_start {
                    current_filling_columns.push(CurrentFilling {
                        row_start: current_row + 1,
                        column_start: current_range_start,
                        column_end: used_range_start - 1,
                    });
                    println!(
                        "!! Adding dig area {}",
                        used_range_start - current_range_start - 1
                    );
                    digged_area += used_range_start - current_range_start - 1;
                }

                current_range_start = used_range_end + 1;
            }

            if current_range_start < line.column_to {
                current_filling_columns.push(CurrentFilling {
                    row_start: current_row + 1,
                    column_start: current_range_start,
                    column_end: line.column_to,
                });
                println!(
                    "END Adding dig area {}",
                    line.column_to - current_range_start + 1
                );
                digged_area += line.column_to - current_range_start + 1;
            }

            current_filling_columns.sort_by_key(|filling| filling.column_start);
            current_filling_columns =
                current_filling_columns
                    .iter()
                    .fold(Vec::new(), |mut folded, current| {
                        if let Some(last) = folded.last_mut() {
                            if last.column_end == current.column_start {
                                last.column_end = current.column_end;
                            } else {
                                folded.push(current.clone());
                            }
                            folded
                        } else {
                            folded.push(current.clone());
                            folded
                        }
                    })
        }

        println!("Fillings after {:?}", current_filling_columns);
        println!("Dig area after {:?}", digged_area);

        if let Some(next_row) = horizontal_lines.iter().find(|line| line.row > current_row) {
            current_row = next_row.row;
        } else {
            break;
        }
    }

    println!("Result is {}", digged_area);
}
