use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
struct Point {
    row: usize,
    column: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Edge {
    to: Point,
    distance: i64,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash, Default)]
struct Node {
    destinations: Vec<Edge>,
}

// 4546 too low
// This is running very slow for the real input, but the result is correct :)
fn main() {
    println!("Hello day 23!");
    let input = read_to_string("inputs/day_23/input").unwrap();

    // let input = "#.#####################\n\
    // #.......#########...###\n\
    // #######.#########.#.###\n\
    // ###.....#.>.>.###.#.###\n\
    // ###v#####.#v#.###.#.###\n\
    // ###.>...#.#.#.....#...#\n\
    // ###v###.#.#.#########.#\n\
    // ###...#.#.#.......#...#\n\
    // #####.#.#.#######.#.###\n\
    // #.....#.#.#.......#...#\n\
    // #.#####.#.#.#########v#\n\
    // #.#...#...#...###...>.#\n\
    // #.#.#v#######v###.###v#\n\
    // #...#.>.#...>.>.#.###.#\n\
    // #####v#.#.###v#.#.###.#\n\
    // #.....#...#...#.#.#...#\n\
    // #.#########.###.#.#.###\n\
    // #...###...#...#...#.###\n\
    // ###.###.#.###v#####v###\n\
    // #...#...#.#.>.>.#.>.###\n\
    // #.###.###.#.###.#.#v###\n\
    // #.....###...###...#...#\n\
    // #####################.#";

    let mut grid_char = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = Point {
        row: 0,
        column: grid_char[0].iter().position(|c| *c == '.').unwrap(),
    };

    let end = Point {
        row: grid_char.len() - 1,
        column: grid_char[grid_char.len() - 1]
            .iter()
            .rposition(|c| *c == '.')
            .unwrap(),
    };

    println!("Start point {:?}", start);
    println!("End point {:?}", end);

    let graph = build_graph(start, end, &grid_char);

    println!("-----");
    println!("Result graph {:?}", graph);

    let all_paths = find_all_paths(&graph, start, end);

    let result = all_paths.iter().map(|(_, d)| *d).max().unwrap();
    println!("Result: {}", result);
}

fn get_neighbours(point: &Point, grid: &[Vec<char>]) -> Vec<Point> {
    let mut result = vec![];
    let row_len = grid.len();
    let column_len = grid[0].len();

    if point.row > 0 {
        let point = Point {
            row: point.row - 1,
            column: point.column,
        };
        if grid[point.row][point.column] != '#' {
            result.push(point);
        }
    }
    if point.row < row_len - 1 {
        let point = Point {
            row: point.row + 1,
            column: point.column,
        };
        if grid[point.row][point.column] != '#' {
            result.push(point);
        }
    }
    if point.column > 0 {
        let point = Point {
            row: point.row,
            column: point.column - 1,
        };
        if grid[point.row][point.column] != '#' {
            result.push(point);
        }
    }
    if point.column < column_len - 1 {
        let point = Point {
            row: point.row,
            column: point.column + 1,
        };
        if grid[point.row][point.column] != '#' {
            result.push(point);
        }
    }
    result
}

fn build_graph(start_point: Point, end_point: Point, grid: &[Vec<char>]) -> HashMap<Point, Node> {
    let mut graph: HashMap<Point, Node> = HashMap::default();
    graph.insert(start_point, Node::default());
    graph.insert(end_point, Node::default());

    let mut no_of_cells_that_can_lead_to_this_cell = vec![vec![0; grid[0].len()]; grid.len()];

    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            if grid[row][column] != '#' {
                for neighbour in get_neighbours(&Point { row, column }, &grid) {
                    no_of_cells_that_can_lead_to_this_cell[neighbour.row][neighbour.column] += 1;
                }
            }
        }
    }

    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            if no_of_cells_that_can_lead_to_this_cell[row][column] > 2 {
                let point = Point { row, column };
                graph.insert(point, Node::default());
            }
        }
    }

    println!("graph nodes {:?}", graph.keys().sorted());

    let mut visited_map = vec![vec![false; grid[0].len()]; grid.len()];
    visited_map[start_point.row][start_point.column] = true;
    let start_neighbours = get_neighbours(&start_point, &grid);
    assert_eq!(start_neighbours.len(), 1);
    let mut to_visit_queue = vec![(start_neighbours[0], start_point, 1)];

    while let Some((to_visit, from_point, distance_from_point)) = to_visit_queue.pop() {
        if visited_map[to_visit.row][to_visit.column] {
            continue;
        }
        visited_map[to_visit.row][to_visit.column] = true;

        let neighbours = get_neighbours(&to_visit, &grid);

        for neighbour in neighbours {
            if graph.contains_key(&neighbour) {
                graph.get_mut(&from_point).unwrap().destinations.push(Edge {
                    to: neighbour,
                    distance: distance_from_point + 1,
                });
                graph.get_mut(&neighbour).unwrap().destinations.push(Edge {
                    to: from_point,
                    distance: distance_from_point + 1,
                });
                to_visit_queue.push((neighbour, neighbour, 0));
            } else {
                to_visit_queue.push((neighbour, from_point, distance_from_point + 1));
            }
        }
    }
    graph
}

fn find_all_paths(
    graph: &HashMap<Point, Node>,
    start_point: Point,
    end_point: Point,
) -> Vec<(Vec<Point>, i64)> {
    let mut paths: Vec<(Vec<Point>, i64)> = vec![];

    let mut to_visit_queue = vec![(start_point, vec![start_point], 0)];

    while let Some((to_visit, path, current_distance)) = to_visit_queue.pop() {
        let destinations = graph.get(&to_visit).unwrap().destinations.clone();

        if to_visit == end_point {
            paths.push((path, current_distance));
        } else {
            for destination in destinations {
                if path.contains(&destination.to) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(destination.to);
                let path_length = current_distance + destination.distance;
                to_visit_queue.push((destination.to, new_path, path_length));
            }
        }
    }

    println!("Found {} paths", paths.len());

    paths
}
