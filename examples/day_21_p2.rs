use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

/*
--- Part Two ---
The Elf seems confused by your answer until he realizes his mistake: he was reading from a list of his favorite numbers that are both perfect squares and perfect cubes, not his step counter.

The actual number of steps he needs to get today is exactly 26501365.

He also points out that the garden plots and rocks are set up so that the map repeats infinitely in every direction.

So, if you were to look one additional map-width or map-height out from the edge of the example map above, you would find that it keeps repeating:

.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##..S####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
.................................
.....###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#....
.##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#.
.......##.........##.........##..
.##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##.
.................................
This is just a tiny three-map-by-three-map slice of the inexplicably-infinite farm layout; garden plots and rocks repeat as far as you can see. The Elf still starts on the one middle tile marked S, though - every other repeated S is replaced with a normal garden plot (.).

Here are the number of reachable garden plots in this new infinite version of the example map for different numbers of steps:

In exactly 6 steps, he can still reach 16 garden plots.
In exactly 10 steps, he can reach any of 50 garden plots.
In exactly 50 steps, he can reach 1594 garden plots.
In exactly 100 steps, he can reach 6536 garden plots.
In exactly 500 steps, he can reach 167004 garden plots.
In exactly 1000 steps, he can reach 668697 garden plots.
In exactly 5000 steps, he can reach 16733044 garden plots.
However, the step count the Elf needs is much larger! Starting from the garden plot marked S on your infinite map, how many garden plots could the Elf reach in exactly 26501365 steps?
*/
#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    row: usize,
    column: usize,
}

/*


*/

fn main() {
    println!("Hello day 1!");
    let no_of_steps = 26501365;
    // let no_of_steps = 1000;

    let input = read_to_string("inputs/day_21/input")
        .unwrap()
        .trim()
        .to_string();

    // let input = "\
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

    /*
    0001222
    0001222
    0001222     [[0], [0,0], [0,0,0
    333X444
    5556777
    5556777

     */

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

    assert_eq!(
        grid_with_rocks.len(),
        grid_with_rocks[0].len(),
        "Grid is not square that breaks some assumptions"
    );

    // for row_index in 1..(grid_with_rocks.len() - 1) {
    //     if grid_with_rocks[row_index].iter().all(|v| !*v) {
    //         panic!("Row {} is empty", row_index);
    //     }
    // }

    // for column_index in 1..(grid_with_rocks[1].len() - 1) {
    //     for row_index in 0..grid_with_rocks.len() {
    //         if grid_with_rocks[row_index][column_index] {
    //             break;
    //         }
    //         if row_index == grid_with_rocks.len() - 1 {
    //             panic!("Column {} is empty", column_index);
    //         }
    //     }
    // }

    println!(
        "grid_with_rocks:\n {}",
        grid_with_rocks
            .iter()
            .map(|row| row.iter().map(|v| if *v { "#" } else { "." }).join(""))
            .join("\n")
    );

    let no_of_rows_in_grid = grid_with_rocks.len() as i64;
    let no_of_coulmns_in_grid = grid_with_rocks[0].len() as i64;
    println!(
        "Grid with rocks size: {}x{}",
        no_of_rows_in_grid, no_of_coulmns_in_grid
    );

    let no_of_added_grids_for_each_side = 0;

    // We will start by making grid with 10 added to each side, so 21 total
    // let bigger_grid = make_bigger_grid(&grid_with_rocks, no_of_added_grids_for_each_side);
    let bigger_grid = grid_with_rocks.clone();

    let distances_map = run_dijkstra(start_point.clone(), &grid_with_rocks);

    println!(
        "Dijkstra distances:\n{}",
        distances_map
            .iter()
            .map(|row| row
                .iter()
                .map(|v| if *v == i64::MAX {
                    "XX".to_string()
                } else {
                    format!("{:2.}", v)
                })
                .join(","))
            .join("\n")
    );

    let start_point_for_bigger_grid = Point {
        row: start_point.row + (no_of_rows_in_grid as usize) * no_of_added_grids_for_each_side,
        column: start_point.column
            + (no_of_coulmns_in_grid as usize) * no_of_added_grids_for_each_side,
    };

    println!("Calculating distances for bigger grid");

    let distances_for_bigger_grid = run_dijkstra(start_point_for_bigger_grid, &bigger_grid);

    let no_of_even_distances_in_grid = distances_for_bigger_grid
        .iter()
        .flatten()
        .filter(|v| **v != i64::MAX && **v % 2 == 0)
        .count() as i64;

    let no_of_odd_distances_in_grid = distances_for_bigger_grid
        .iter()
        .flatten()
        .filter(|v| **v != i64::MAX && **v % 2 == 1)
        .count() as i64;

    println!(
        "No of even distances in grid: {}",
        no_of_even_distances_in_grid
    );

    println!(
        "No of odd distances in grid: {}",
        no_of_odd_distances_in_grid
    );

    // Lets find the furthest grid to the right + 1 above

    let no_of_possible_visits_top_right =
        top_right_corner(&bigger_grid, &distances_for_bigger_grid, no_of_steps);

    println!(
        "No of possible visits top right: {}",
        no_of_possible_visits_top_right
    );

    let no_of_possible_visits_top_left =
        top_left_corner(&bigger_grid, &distances_for_bigger_grid, no_of_steps);

    println!(
        "No of possible visits top left: {}",
        no_of_possible_visits_top_left
    );

    let no_of_possible_visits_bottom_right =
        bottom_right_corner(&bigger_grid, &distances_for_bigger_grid, no_of_steps);

    println!(
        "No of possible visits bottom right: {}",
        no_of_possible_visits_bottom_right
    );

    let no_of_possible_visits_bottom_left =
        bottom_left_corner(&bigger_grid, &distances_for_bigger_grid, no_of_steps);

    println!(
        "No of possible visits bottom left: {}",
        no_of_possible_visits_bottom_left
    );

    let top_grids = top_grids(&bigger_grid, &distances_for_bigger_grid, no_of_steps);
    println!("No of possible visits top grids: {}", top_grids);

    let bottom_grids = bottom_grids(&bigger_grid, &distances_for_bigger_grid, no_of_steps);
    println!("No of possible visits bottom grids: {}", bottom_grids);

    let left_grids = left_grids(&bigger_grid, &distances_for_bigger_grid, no_of_steps);
    println!("No of possible visits left grids: {}", left_grids);

    let right_grids = right_grids(&bigger_grid, &distances_for_bigger_grid, no_of_steps);
    println!("No of possible visits right grids: {}", right_grids);

    // Now we need to find top/bottom left/right most grid

    let mut no_of_possible_visits = if no_of_steps % 2 == 0 {
        no_of_even_distances_in_grid
    } else {
        no_of_odd_distances_in_grid
    };

    no_of_possible_visits += no_of_possible_visits_top_right
        + no_of_possible_visits_bottom_right
        + no_of_possible_visits_top_left
        + no_of_possible_visits_bottom_left
        + top_grids
        + bottom_grids
        + left_grids
        + right_grids;

    println!("Result is {}", no_of_possible_visits);
}

fn top_grids(
    bigger_grid: &Vec<Vec<bool>>,
    distances_for_bigger_grid: &[Vec<i64>],
    no_of_steps: i64,
) -> i64 {
    // We enter from the middle cause it is empty
    let distance_to_edge = bigger_grid.len() / 2;
    let distances_map_from_edge = run_dijkstra(
        Point {
            row: bigger_grid.len() - 1,
            column: bigger_grid[0].len() / 2,
        },
        &bigger_grid,
    );

    visits_not_corner(
        bigger_grid,
        no_of_steps,
        distance_to_edge as i64,
        distances_map_from_edge,
    )
}

fn bottom_grids(
    bigger_grid: &Vec<Vec<bool>>,
    distances_for_bigger_grid: &[Vec<i64>],
    no_of_steps: i64,
) -> i64 {
    // We enter from the middle cause it is empty
    let distance_to_edge = bigger_grid.len() / 2;
    let distances_map_from_edge = run_dijkstra(
        Point {
            row: 0,
            column: bigger_grid[0].len() / 2,
        },
        &bigger_grid,
    );

    visits_not_corner(
        bigger_grid,
        no_of_steps,
        distance_to_edge as i64,
        distances_map_from_edge,
    )
}

fn left_grids(
    bigger_grid: &Vec<Vec<bool>>,
    distances_for_bigger_grid: &[Vec<i64>],
    no_of_steps: i64,
) -> i64 {
    // We enter from the middle cause it is empty
    let distance_to_edge = bigger_grid.len() / 2;
    let distances_map_from_edge = run_dijkstra(
        Point {
            row: bigger_grid[0].len() / 2,
            column: bigger_grid[0].len() - 1,
        },
        &bigger_grid,
    );

    visits_not_corner(
        bigger_grid,
        no_of_steps,
        distance_to_edge as i64,
        distances_map_from_edge,
    )
}

fn right_grids(
    bigger_grid: &Vec<Vec<bool>>,
    distances_for_bigger_grid: &[Vec<i64>],
    no_of_steps: i64,
) -> i64 {
    // We enter from the middle cause it is empty
    let distance_to_edge = bigger_grid.len() / 2;
    let distances_map_from_edge = run_dijkstra(
        Point {
            row: bigger_grid[0].len() / 2,
            column: 0,
        },
        &bigger_grid,
    );

    visits_not_corner(
        bigger_grid,
        no_of_steps,
        distance_to_edge as i64,
        distances_map_from_edge,
    )
}

fn visits_not_corner(
    bigger_grid: &Vec<Vec<bool>>,
    no_of_steps: i64,
    distance_to_edge: i64,
    distances_map_from_edge: Vec<Vec<i64>>,
) -> i64 {
    let no_of_steps_is_even = no_of_steps % 2 == 0;
    let mut no_of_possible_visits = 0;

    let (
        cumulative_even_distances_lookup_map,
        cumulative_odd_distances_lookup_map,
        no_of_even_distances_from_corner,
        no_of_odd_distances_from_corner,
    ) = get_distances_lookup(distances_map_from_edge);

    let distance_to_next_grid = 1 + distance_to_edge;
    let no_of_grids_to_the_end =
        (no_of_steps - distance_to_next_grid) / bigger_grid[0].len() as i64 + 1;
    let distance_to_the_last_grid =
        distance_to_next_grid + (no_of_grids_to_the_end - 1) * bigger_grid[0].len() as i64;

    {
        let remaining_distance_in_last_grid = no_of_steps - distance_to_the_last_grid;

        if remaining_distance_in_last_grid % 2 == 0 {
            no_of_possible_visits += *cumulative_even_distances_lookup_map
                .get(&(remaining_distance_in_last_grid as usize))
                .expect(&format!(
                    "Failed to find distance {} in cumulative even distances",
                    remaining_distance_in_last_grid
                ));
        } else {
            no_of_possible_visits += *cumulative_odd_distances_lookup_map
                .get(&(remaining_distance_in_last_grid as usize))
                .expect(&format!(
                    "Failed to find distance {} in cumulative odd distances",
                    remaining_distance_in_last_grid
                ));
        }
    }

    let distance_to_the_second_to_last_grid =
        distance_to_the_last_grid - bigger_grid[0].len() as i64;
    {
        let remaining_distance_in_second_to_last_grid =
            no_of_steps - distance_to_the_second_to_last_grid;

        if remaining_distance_in_second_to_last_grid % 2 == 0 {
            no_of_possible_visits += *cumulative_even_distances_lookup_map
                .get(&(remaining_distance_in_second_to_last_grid as usize))
                .unwrap_or(&no_of_even_distances_from_corner);
        } else {
            no_of_possible_visits += *cumulative_odd_distances_lookup_map
                .get(&(remaining_distance_in_second_to_last_grid as usize))
                .unwrap_or(&no_of_odd_distances_from_corner);
        }
    }

    // Strong assumption that only last and one before last grid can be not fully filled
    let no_of_full_grids = no_of_grids_to_the_end - 2;

    no_of_possible_visits += (no_of_full_grids / 2)
        * (no_of_even_distances_from_corner + no_of_odd_distances_from_corner);
    if no_of_full_grids % 2 == 1 {
        if !no_of_steps_is_even {
            no_of_possible_visits += no_of_even_distances_from_corner;
        } else {
            no_of_possible_visits += no_of_odd_distances_from_corner;
        }
    }

    no_of_possible_visits
}

fn top_left_corner(
    bigger_grid: &Vec<Vec<bool>>,
    distances_for_bigger_grid: &[Vec<i64>],
    no_of_steps: i64,
) -> i64 {
    let distance_to_corner = *distances_for_bigger_grid.first().unwrap().first().unwrap();
    let distances_map_from_corner = run_dijkstra(
        Point {
            row: bigger_grid.len() - 1,
            column: bigger_grid[0].len() - 1,
        },
        &bigger_grid,
    );
    no_of_visits_for_corner(
        bigger_grid,
        no_of_steps,
        distance_to_corner,
        distances_map_from_corner,
    )
}

fn top_right_corner(
    bigger_grid: &Vec<Vec<bool>>,
    distances_for_bigger_grid: &[Vec<i64>],
    no_of_steps: i64,
) -> i64 {
    let distance_to_corner = *distances_for_bigger_grid.first().unwrap().last().unwrap();
    let distances_map_from_corner = run_dijkstra(
        Point {
            row: bigger_grid.len() - 1,
            column: 0,
        },
        &bigger_grid,
    );
    no_of_visits_for_corner(
        bigger_grid,
        no_of_steps,
        distance_to_corner,
        distances_map_from_corner,
    )
}

fn bottom_right_corner(
    bigger_grid: &Vec<Vec<bool>>,
    distances_for_bigger_grid: &[Vec<i64>],
    no_of_steps: i64,
) -> i64 {
    let distance_to_corner = *distances_for_bigger_grid.last().unwrap().last().unwrap();
    let distances_map_from_corner = run_dijkstra(Point { row: 0, column: 0 }, &bigger_grid);
    no_of_visits_for_corner(
        bigger_grid,
        no_of_steps,
        distance_to_corner,
        distances_map_from_corner,
    )
}

fn bottom_left_corner(
    bigger_grid: &Vec<Vec<bool>>,
    distances_for_bigger_grid: &[Vec<i64>],
    no_of_steps: i64,
) -> i64 {
    let distance_to_corner = *distances_for_bigger_grid.last().unwrap().first().unwrap();
    let distances_map_from_corner = run_dijkstra(
        Point {
            row: 0,
            column: bigger_grid[0].len() - 1,
        },
        &bigger_grid,
    );
    no_of_visits_for_corner(
        bigger_grid,
        no_of_steps,
        distance_to_corner,
        distances_map_from_corner,
    )
}

fn no_of_visits_for_corner(
    bigger_grid: &Vec<Vec<bool>>,
    no_of_steps: i64,
    distance_to_corner: i64,
    distances_map_from_corner: Vec<Vec<i64>>,
) -> i64 {
    let no_of_rows_in_bigger_grid = bigger_grid.len() as i64;
    let no_of_columns_in_bigger_grid = bigger_grid[0].len() as i64;

    let mut no_of_possible_visits = 0;
    let mut rows_to_add = 0;

    let no_of_steps_is_even = no_of_steps % 2 == 0;

    let (
        cumulative_even_distances_lookup_map,
        cumulative_odd_distances_lookup_map,
        no_of_even_distances_from_corner,
        no_of_odd_distances_from_corner,
    ) = get_distances_lookup(distances_map_from_corner);

    loop {
        // Go to the corner
        // Go one more step to the right
        // Go one more step up
        // Go row_above * grid rows up
        // Calculate no of

        let distance_to_the_proper_row =
            1 + 1 + distance_to_corner + rows_to_add * no_of_rows_in_bigger_grid;
        if no_of_steps < distance_to_the_proper_row {
            println!("Break after checking {} rows above", rows_to_add);
            break;
        }

        let no_of_grids_in_row_to_the_right_to_the_last_grid =
            (no_of_steps - distance_to_the_proper_row) / no_of_columns_in_bigger_grid + 1;

        let distance_to_the_last_grid_in_row = distance_to_the_proper_row
            + (no_of_grids_in_row_to_the_right_to_the_last_grid - 1) * no_of_columns_in_bigger_grid;

        // Add the visited in the last grid
        {
            let remaining_distance_in_last_grid = no_of_steps - distance_to_the_last_grid_in_row;

            if remaining_distance_in_last_grid % 2 == 0 {
                no_of_possible_visits += *cumulative_even_distances_lookup_map
                    .get(&(remaining_distance_in_last_grid as usize))
                    .expect(&format!(
                        "Failed to find distance {} in cumulative even distances",
                        remaining_distance_in_last_grid
                    ));
            } else {
                no_of_possible_visits += *cumulative_odd_distances_lookup_map
                    .get(&(remaining_distance_in_last_grid as usize))
                    .expect(&format!(
                        "Failed to find distance {} in cumulative odd distances",
                        remaining_distance_in_last_grid
                    ));
            }
        }

        if no_of_grids_in_row_to_the_right_to_the_last_grid > 1 {
            let distance_to_the_second_to_last_grid_to_the_right =
                distance_to_the_last_grid_in_row - no_of_columns_in_bigger_grid;

            // Add the previous to last visited
            {
                let remaining_distance_in_second_to_last_grid =
                    no_of_steps - distance_to_the_second_to_last_grid_to_the_right;

                if remaining_distance_in_second_to_last_grid % 2 == 0 {
                    no_of_possible_visits += *cumulative_even_distances_lookup_map
                        .get(&(remaining_distance_in_second_to_last_grid as usize))
                        .unwrap_or(&no_of_even_distances_from_corner);
                } else {
                    no_of_possible_visits += *cumulative_odd_distances_lookup_map
                        .get(&(remaining_distance_in_second_to_last_grid as usize))
                        .unwrap_or(&no_of_odd_distances_from_corner);
                }
            }

            // Strong assumption that only last and one before last grid can be not fully filled
            let no_of_full_grids = no_of_grids_in_row_to_the_right_to_the_last_grid - 2;

            no_of_possible_visits += (no_of_full_grids / 2)
                * (no_of_even_distances_from_corner + no_of_odd_distances_from_corner);
            if no_of_full_grids % 2 == 1 {
                if !no_of_steps_is_even {
                    no_of_possible_visits += no_of_even_distances_from_corner;
                } else {
                    no_of_possible_visits += no_of_odd_distances_from_corner;
                }
            }
        }

        rows_to_add += 1;
    }
    no_of_possible_visits
}

fn get_distances_lookup(
    distances_map_from_corner: Vec<Vec<i64>>,
) -> (HashMap<usize, i64>, HashMap<usize, i64>, i64, i64) {
    let biggest_distance_in_grid = *distances_map_from_corner
        .iter()
        .flatten()
        .filter(|v| **v != i64::MAX)
        .max()
        .unwrap() as usize;

    let mut distances_lookup_map = vec![0i64; biggest_distance_in_grid + 1];

    for distance in distances_map_from_corner.iter().flatten() {
        if distance != &i64::MAX {
            distances_lookup_map[*distance as usize] += 1;
        }
    }

    let mut cumulative_even_distances_lookup_map: HashMap<usize, i64> = HashMap::default();
    let mut cumulative_odd_distances_lookup_map: HashMap<usize, i64> = HashMap::default();

    let mut even_distances_sum = 0;
    let mut odd_distances_sum = 0;

    for i in 0..=biggest_distance_in_grid {
        if i % 2 == 0 {
            even_distances_sum += distances_lookup_map[i];
            cumulative_even_distances_lookup_map.insert(i, even_distances_sum);
        } else {
            odd_distances_sum += distances_lookup_map[i];
            cumulative_odd_distances_lookup_map.insert(i, odd_distances_sum);
        }
    }

    println!(
        "Cumulative even distances: {:?}",
        cumulative_even_distances_lookup_map
    );
    println!(
        "Cumulative odd distances: {:?}",
        cumulative_odd_distances_lookup_map
    );

    let no_of_even_distances_from_corner = even_distances_sum;
    let no_of_odd_distances_from_corner = odd_distances_sum;
    (
        cumulative_even_distances_lookup_map,
        cumulative_odd_distances_lookup_map,
        no_of_even_distances_from_corner,
        no_of_odd_distances_from_corner,
    )
}

fn make_bigger_grid(
    grid_with_rocks: &Vec<Vec<bool>>,
    no_of_added_grids_for_each_side: usize,
) -> Vec<Vec<bool>> {
    let row_result = grid_with_rocks
        .iter()
        .map(|row| {
            vec![row.clone(); no_of_added_grids_for_each_side * 2 + 1]
                .into_iter()
                .flatten()
                .collect_vec()
        })
        .collect_vec();

    vec![row_result.clone(); no_of_added_grids_for_each_side * 2 + 1]
        .into_iter()
        .flatten()
        .collect_vec()
}

fn run_dijkstra(start_point: Point, grid_with_rocks: &[Vec<bool>]) -> Vec<Vec<i64>> {
    let mut distances_map = vec![vec![i64::MAX; grid_with_rocks[0].len()]; grid_with_rocks.len()];

    distances_map[start_point.row][start_point.column] = 0;

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

            to_visit_queue.push((neighbour, distance_to_neighbour));
        }

        to_visit_queue.sort_by_key(|(_, d)| -(*d as i64));
    }
    distances_map
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
