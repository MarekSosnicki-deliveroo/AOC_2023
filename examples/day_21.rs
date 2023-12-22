use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    row: usize,
    column: usize,
}
fn main() {
    println!("Hello day 1!");
    let input = read_to_string("inputs/day_21/input").unwrap();

    //     let input = "\
    // ...........\n\
    // .....###.#.\n\
    // .###.##..#.\n\
    // ..#.#...#..\n\
    // ....#.#....\n\
    // .##..S####.\n\
    // .##..#...#.\n\
    // .......##..\n\
    // .##.#.####.\n\
    // .##..##.##.\n\
    // ...........";

    let grid_char = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start_point = grid_char
        .iter()
        .enumerate()
        .find_map(|(row, column)| {
            column
                .iter()
                .position(|c| *c == 'S')
                .map(|column| Point { row, column })
        })
        .unwrap();

    println!("Start point is {:?}", start_point);

    let grid_with_rocks = grid_char
        .iter()
        .enumerate()
        .map(|(row, column)| {
            column
                .iter()
                .enumerate()
                .map(|(column, c)| if *c == '#' { true } else { false })
                .collect_vec()
        })
        .collect_vec();

    println!(
        "grid_with_rocks: {}",
        grid_with_rocks
            .iter()
            .map(|row| row.iter().map(|v| if *v { "#" } else { "." }).join(""))
            .join("\n")
    );

    let mut distances_map =
        vec![vec![std::usize::MAX; grid_with_rocks[0].len()]; grid_with_rocks.len()];

    distances_map[start_point.row][start_point.column] = 0;

    let mut even_distances_map =
        vec![vec![std::usize::MAX; grid_with_rocks[0].len()]; grid_with_rocks.len()];

    even_distances_map[start_point.row][start_point.column] = 0;

    let mut visited_map = vec![vec![false; grid_with_rocks[0].len()]; grid_with_rocks.len()];

    let mut to_visit_queue = vec![(start_point, 0)];

    let rows_length = grid_with_rocks.len();
    let columns_lenght = grid_with_rocks[0].len();

    while let Some((to_visit, distance)) = to_visit_queue.pop() {
        if visited_map[to_visit.row][to_visit.column] {
            continue;
        }
        visited_map[to_visit.row][to_visit.column] = true;

        for neighbour in get_neighbours(&to_visit, rows_length, columns_lenght) {
            if grid_with_rocks[neighbour.row][neighbour.column] {
                continue;
            }
            let distance_to_neighbour = distance + 1;
            distances_map[neighbour.row][neighbour.column] =
                distances_map[neighbour.row][neighbour.column].min(distance_to_neighbour);

            if distance_to_neighbour % 2 == 0 {
                even_distances_map[neighbour.row][neighbour.column] =
                    even_distances_map[neighbour.row][neighbour.column].min(distance_to_neighbour);
            }

            to_visit_queue.push((neighbour, distance_to_neighbour));
        }

        to_visit_queue.sort_by_key(|(_, d)| -(*d as i64));
    }

    println!(
        "Dijkstra distances:\n{}",
        distances_map
            .iter()
            .map(|row| row
                .iter()
                .map(|v| if *v == std::usize::MAX {
                    "XX".to_string()
                } else {
                    format!("{:2.}", v)
                })
                .join(","))
            .join("\n")
    );

    println!(
        "Even distances:\n{}",
        even_distances_map
            .iter()
            .map(|row| row
                .iter()
                .map(|v| if *v == std::usize::MAX {
                    "XX".to_string()
                } else {
                    format!("{:2.}", v)
                })
                .join(","))
            .join("\n")
    );

    let no_of_steps = 64;
    let result: usize = even_distances_map
        .iter()
        .map(|row| {
            row.iter()
                .filter(|distance| **distance <= no_of_steps)
                .count()
        })
        .sum();
    println!("Result: {}", result);
}

fn get_neighbours(point: &Point, row_len: usize, column_len: usize) -> Vec<Point> {
    let mut result = vec![];

    if point.row > 0 {
        result.push(Point {
            row: point.row - 1,
            column: point.column,
        });
    }
    if point.row < row_len - 1 {
        result.push(Point {
            row: point.row + 1,
            column: point.column,
        });
    }
    if point.column > 0 {
        result.push(Point {
            row: point.row,
            column: point.column - 1,
        });
    }
    if point.column < column_len - 1 {
        result.push(Point {
            row: point.row,
            column: point.column + 1,
        });
    }
    result
}
