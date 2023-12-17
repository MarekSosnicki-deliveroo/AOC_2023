use itertools::Itertools;
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
            if self.no_of_other_moves_in_this_direction > 1 {
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
            Some(ToVisit {
                row: new_row,
                column: new_column,
                direction: direction,
                no_of_other_moves_in_this_direction: 0,
            })
        }
    }
}

fn main() {
    println!("Hello day 17!");
    let input = read_to_string("inputs/day_17/input").unwrap();

    let input = "2413432311323\n\
    3215453535623\n\
    3255245654254\n\
    3446585845452\n\
    4546657867536\n\
    1438598798454\n\
    4457876987766\n\
    3637877979653\n\
    4654967986887\n\
    4564679986453\n\
    1224686865563\n\
    2546548887735\n\
    4322674655533";

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

    let mut distances_map = vec![vec![vec![i64::MAX; 3]; grid[0].len()]; grid.len()];
    let mut prev_map: Vec<Vec<Vec<Option<(ToVisit, i64)>>>> =
        vec![vec![vec![None; 3]; grid[0].len()]; grid.len()];
    let mut visited_map = vec![vec![vec![false; 3]; grid[0].len()]; grid.len()];

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

    distances_map[0][1][0] = 0;
    distances_map[1][0][0] = 0;

    while !to_visit_queue.is_empty() {
        to_visit_queue.sort_by_key(|(_, distance_to)| *distance_to);
        let (visiting, distance_to) = to_visit_queue.remove(0);

        if visited_map[visiting.row as usize][visiting.column as usize]
            [visiting.no_of_other_moves_in_this_direction as usize]
        {
            continue;
        }

        println!("Visiting {visiting:?} distance_to: {distance_to}");

        let current_heat = grid[visiting.row as usize][visiting.column as usize];

        visited_map[visiting.row as usize][visiting.column as usize]
            [visiting.no_of_other_moves_in_this_direction as usize] = true;

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
                    let new_distance_to = distance_to + current_heat;

                    // if let Some(found) = to_visit_queue.iter_mut().find(|p| p.0 == new_point) {
                    //     found.1 = found.1.min(new_distance_to);
                    // } else {
                    // }

                    if distances_map[new_point.row as usize][new_point.column as usize]
                        [new_point.no_of_other_moves_in_this_direction as usize]
                        > new_distance_to
                    {
                        to_visit_queue.push((new_point, new_distance_to));

                        distances_map[new_point.row as usize][new_point.column as usize]
                            [new_point.no_of_other_moves_in_this_direction as usize] =
                            new_distance_to;

                        prev_map[new_point.row as usize][new_point.column as usize]
                            [new_point.no_of_other_moves_in_this_direction as usize] =
                            Some((visiting, new_distance_to));
                    }
                }
            }
        }
    }

    println!("Distances map:");
    for row in distances_map.iter() {
        println!("{:?}", row);
    }

    let (no_of_moves, result) = distances_map
        .last()
        .unwrap()
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .min_by_key(|(_, distance)| *distance)
        .unwrap();

    let mut grid_with_route = vec![vec!['.'; grid[0].len()]; grid.len()];

    let mut current = ToVisit {
        row: grid.len() as i64 - 1,
        column: grid[0].len() as i64 - 1,
        direction: Direction::Right,
        no_of_other_moves_in_this_direction: no_of_moves as i64,
    };

    while current.row != 0 || current.column != 0 {
        grid_with_route[current.row as usize][current.column as usize] = '#';
        if let Some((prev, _)) = prev_map[current.row as usize][current.column as usize]
            [current.no_of_other_moves_in_this_direction as usize]
        {
            current = prev;
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
