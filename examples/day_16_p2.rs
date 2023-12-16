use itertools::Itertools;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point {
    row: i32,
    column: i32,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Point {
    fn move_to(&self, direction: Direction) -> Point {
        match direction {
            Direction::Up => Point {
                row: self.row - 1,
                column: self.column,
            },
            Direction::Down => Point {
                row: self.row + 1,
                column: self.column,
            },
            Direction::Left => Point {
                row: self.row,
                column: self.column - 1,
            },
            Direction::Right => Point {
                row: self.row,
                column: self.column + 1,
            },
        }
    }
}

fn main() {
    println!("Hello day 16!");
    let input = read_to_string("inputs/day_16/input").unwrap();

    // let input = ".|...\\....\n\
    //              |.-.\\.....\n\
    //              .....|-...\n\
    //              ........|.\n\
    //              ..........\n\
    //              .........\\\n\
    //              ..../.\\\\..\n\
    //              .-.-/..|..\n\
    //              .|....-|.\\\n\
    //              ..//.|....";

    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let result = (0..grid.len())
        .map(|row| {
            (
                Point {
                    row: row as i32,
                    column: 0,
                },
                Direction::Right,
            )
        })
        .chain((0..grid.len()).map(|row| {
            (
                Point {
                    row: row as i32,
                    column: grid[0].len() as i32 - 1,
                },
                Direction::Left,
            )
        }))
        .chain((0..grid[0].len()).map(|column| {
            (
                Point {
                    row: 0,
                    column: column as i32,
                },
                Direction::Down,
            )
        }))
        .chain((0..grid[0].len()).map(|column| {
            (
                Point {
                    row: grid.len() as i32 - 1,
                    column: column as i32,
                },
                Direction::Up,
            )
        }))
        .map(|(entry_point, entry_direction)| energize_grid(&grid, entry_point, entry_direction))
        .max()
        .unwrap();

    println!("Result is {result}");
}

fn energize_grid(grid: &[Vec<char>], entry_point: Point, entry_direction: Direction) -> usize {
    let mut visited_grid = vec![vec![false; grid[0].len()]; grid.len()];

    let mut points_queue = vec![(entry_point, entry_direction)];

    let mut visited_points_with_directions: HashSet<(Point, Direction)> = HashSet::new();

    while !points_queue.is_empty() {
        let (point, from_direction) = points_queue.pop().unwrap();
        if point.row < 0
            || point.row >= grid.len() as i32
            || point.column < 0
            || point.column >= grid[0].len() as i32
        {
            // println!("Out of bounds");
            continue;
        }

        // println!("Visiting point row: {} column: {}", point.row, point.column);
        // println!(
        //     "Point sign {}",
        //     grid[point.row as usize][point.column as usize]
        // );
        if visited_points_with_directions.contains(&(point, from_direction)) {
            // println!("Already visited");
            continue;
        } else {
            visited_points_with_directions.insert((point, from_direction));
        }

        visited_grid[point.row as usize][point.column as usize] = true;

        match grid[point.row as usize][point.column as usize] {
            '.' => {
                points_queue.push((point.move_to(from_direction), from_direction));
            }
            '/' => {
                let new_direction = match from_direction {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                };
                points_queue.push((point.move_to(new_direction), new_direction));
            }
            '\\' => {
                let new_direction = match from_direction {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                };
                points_queue.push((point.move_to(new_direction), new_direction));
            }
            '|' => match from_direction {
                Direction::Up | Direction::Down => {
                    points_queue.push((point.move_to(from_direction), from_direction));
                }
                Direction::Left | Direction::Right => {
                    points_queue.push((point.move_to(Direction::Up), Direction::Up));
                    points_queue.push((point.move_to(Direction::Down), Direction::Down));
                }
            },
            '-' => match from_direction {
                Direction::Up | Direction::Down => {
                    points_queue.push((point.move_to(Direction::Left), Direction::Left));
                    points_queue.push((point.move_to(Direction::Right), Direction::Right));
                }
                Direction::Left | Direction::Right => {
                    points_queue.push((point.move_to(from_direction), from_direction));
                }
            },
            _ => panic!(
                "Unexpected value in grid: {}",
                grid[point.row as usize][point.column as usize]
            ),
        }
    }

    // println!("Visited_grid:");
    // println!(
    //     "{}",
    //     visited_grid
    //         .iter()
    //         .map(|row| row
    //             .iter()
    //             .map(|v| if *v { "#" } else { "." })
    //             .collect::<String>())
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );

    let result = visited_grid
        .iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum::<usize>();
    result
}
