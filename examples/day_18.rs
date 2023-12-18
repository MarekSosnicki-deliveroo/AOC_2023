use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, sscanf::FromScanf)]
enum Direction {
    #[sscanf("U")]
    Up,
    #[sscanf("D")]
    Down,
    #[sscanf("L")]
    Left,
    #[sscanf("R")]
    Right,
}

impl Point {
    fn move_with(&self, direction: Direction) -> Point {
        let (new_row, new_column) = match direction {
            Direction::Up => (self.row - 1, self.column),
            Direction::Down => (self.row + 1, self.column),
            Direction::Left => (self.row, self.column - 1),
            Direction::Right => (self.row, self.column + 1),
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

    //     let input = "R 6 (#70c710)\n\
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

    let row_size = 1000;
    let column_size = 1000;

    let mut trenches_grid = vec![vec![false; column_size]; row_size];
    let start_point = Point {
        row: row_size / 2,
        column: column_size / 2,
    };

    let mut current_point: Point = start_point;
    trenches_grid[current_point.row][current_point.column] = true;

    input.lines().for_each(|line| {
        let (direction, steps, _) = sscanf::scanf!(line, "{Direction} {usize} {str}").unwrap();

        for _ in 0..steps {
            current_point = current_point.move_with(direction);
            trenches_grid[current_point.row][current_point.column] = true;
        }
    });

    println!("Result grid:");
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
    let mut to_visit = vec![Point { row: 0, column: 0 }];

    while let Some(current) = to_visit.pop() {
        if flood_grid[current.row][current.column] || trenches_grid[current.row][current.column] {
            continue;
        }
        flood_grid[current.row][current.column] = true;

        if current.row > 0 {
            to_visit.push(Point {
                row: current.row - 1,
                column: current.column,
            })
        }

        if current.column > 0 {
            to_visit.push(Point {
                row: current.row,
                column: current.column - 1,
            })
        }

        if current.row < flood_grid.len() - 1 {
            to_visit.push(Point {
                row: current.row + 1,
                column: current.column,
            })
        }

        if current.column < flood_grid[0].len() - 1 {
            to_visit.push(Point {
                row: current.row,
                column: current.column + 1,
            })
        }
    }

    println!("Result grid:");
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
    let result: usize = flood_grid
        .iter()
        .map(|row| row.iter().filter(|v| !*v).count())
        .sum();
    println!("Result: {}", result);
}
