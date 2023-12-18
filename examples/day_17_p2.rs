use itertools::Itertools;
use std::cmp::min_by_key;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct ToVisit {
    row: i64,
    column: i64,
    direction: Direction,
    no_of_other_moves_in_this_direction: i64,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl ToVisit {
    fn move_with(&self, direction: Direction) -> Option<ToVisit> {
        let (new_row, new_column) = match direction {
            Direction::Up => (self.row - 1, self.column),
            Direction::Down => (self.row + 1, self.column),
            Direction::Left => (self.row, self.column - 1),
            Direction::Right => (self.row, self.column + 1),
        };

        match (self.direction, direction) {
            (Direction::Up, Direction::Down) => return None,
            (Direction::Down, Direction::Up) => return None,
            (Direction::Left, Direction::Right) => return None,
            (Direction::Right, Direction::Left) => return None,
            _ => {}
        }

        if self.direction == direction {
            if self.no_of_other_moves_in_this_direction > 8 {
                None
            } else {
                Some(ToVisit {
                    row: new_row,
                    column: new_column,
                    direction: direction,
                    no_of_other_moves_in_this_direction: self.no_of_other_moves_in_this_direction
                        + 1,
                })
            }
        } else {
            if self.no_of_other_moves_in_this_direction < 3 {
                None
            } else {
                Some(ToVisit {
                    row: new_row,
                    column: new_column,
                    direction: direction,
                    no_of_other_moves_in_this_direction: 0,
                })
            }
        }
    }
}

fn main() {
    println!("Hello day 17!");
    let input = read_to_string("inputs/day_17/input").unwrap();

    // let input = "2413432311323\n\
    // 3215453535623\n\
    // 3255245654254\n\
    // 3446585845452\n\
    // 4546657867536\n\
    // 1438598798454\n\
    // 4457876987766\n\
    // 3637877979653\n\
    // 4654967986887\n\
    // 4564679986453\n\
    // 1224686865563\n\
    // 2546548887735\n\
    // 4322674655533";

    //     let input = "111111111111\n\
    // 999999999991\n\
    // 999999999991\n\
    // 999999999991\n\
    // 999999999991";

    // let input = "241343231\n\
    // 321545353";
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c as i64 - '0' as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut distances_map: Vec<Vec<HashMap<(Direction, i64), i64>>> =
        vec![vec![HashMap::default(); grid[0].len()]; grid.len()];
    let mut prev_map: Vec<Vec<HashMap<(Direction, i64), ToVisit>>> =
        vec![vec![HashMap::default(); grid[0].len()]; grid.len()];
    let mut visited_map: Vec<Vec<HashSet<(Direction, i64)>>> =
        vec![vec![HashSet::default(); grid[0].len()]; grid.len()];

    let mut to_visit_queue = vec![
        (
            ToVisit {
                row: 0,
                column: 1,
                direction: Direction::Right,
                no_of_other_moves_in_this_direction: 0,
            },
            grid[0][1],
        ),
        (
            ToVisit {
                row: 1,
                column: 0,
                direction: Direction::Down,
                no_of_other_moves_in_this_direction: 0,
            },
            grid[1][0],
        ),
    ];

    distances_map[0][1].insert((Direction::Right, 0), grid[0][1]);
    distances_map[0][1].insert((Direction::Down, 0), grid[1][0]);

    while !to_visit_queue.is_empty() {
        to_visit_queue.sort_by_key(|(_v, distance_to)| *distance_to);
        let (visiting, distance_to) = to_visit_queue.remove(0);

        if visited_map[visiting.row as usize][visiting.column as usize].contains(&(
            visiting.direction,
            visiting.no_of_other_moves_in_this_direction,
        )) {
            continue;
        }

        println!("Visiting {visiting:?} distance_to: {distance_to}");

        visited_map[visiting.row as usize][visiting.column as usize].insert((
            visiting.direction,
            visiting.no_of_other_moves_in_this_direction,
        ));

        for direction in [
            Direction::Right,
            Direction::Left,
            Direction::Down,
            Direction::Up,
        ] {
            if let Some(new_point) = visiting.move_with(direction) {
                if new_point.row >= 0
                    && new_point.row < grid.len() as i64
                    && new_point.column >= 0
                    && new_point.column < grid[0].len() as i64
                {
                    let new_point_heat = grid[new_point.row as usize][new_point.column as usize];

                    let new_distance_to = distance_to + new_point_heat;

                    let distance_entry = distances_map[new_point.row as usize]
                        [new_point.column as usize]
                        .entry((
                            new_point.direction,
                            new_point.no_of_other_moves_in_this_direction,
                        ))
                        .or_insert(std::i64::MAX);

                    if *distance_entry >= new_distance_to {
                        to_visit_queue.push((new_point, new_distance_to));
                        *distance_entry = new_distance_to;
                        prev_map[new_point.row as usize][new_point.column as usize].insert(
                            (
                                new_point.direction,
                                new_point.no_of_other_moves_in_this_direction,
                            ),
                            visiting,
                        );
                    }
                }
            }
        }
    }

    // println!("Distances map:");
    // for row in distances_map.iter() {
    //     println!("{:?}", row);
    // }

    println!(
        "Distances of last {:?}",
        distances_map.last().unwrap().last().unwrap()
    );

    let ((last_dir, last_no_of_moves), result) = distances_map
        .last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .filter(|((_, no_of_moves), _)| *no_of_moves >= 3)
        .min_by_key(|(_, distance)| *distance)
        .unwrap();
    let mut grid_with_route = vec![vec!['.'; grid[0].len()]; grid.len()];

    let mut current = ToVisit {
        row: grid.len() as i64 - 1,
        column: grid[0].len() as i64 - 1,
        direction: *last_dir,
        no_of_other_moves_in_this_direction: *last_no_of_moves,
    };

    while current.row != 0 || current.column != 0 {
        grid_with_route[current.row as usize][current.column as usize] = '#';
        if let Some(prev) = prev_map[current.row as usize][current.column as usize].get(&(
            current.direction,
            current.no_of_other_moves_in_this_direction,
        )) {
            current = prev.clone();
        } else {
            break;
        }
    }

    println!(
        "Route grid: \n{}",
        grid_with_route
            .iter()
            .map(|row| row.iter().join(""))
            .join("\n")
    );

    println!("Result: {}", result);
}
