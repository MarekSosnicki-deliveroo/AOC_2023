use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::os::unix::raw::gid_t;
/*
The parabolic reflector dish deforms, but not in a way that focuses the beam. To do that, you'll need to move the rocks to the edges of the platform. Fortunately, a button on the side of the control panel labeled "spin cycle" attempts to do just that!

Each cycle tilts the platform four times so that the rounded rocks roll north, then west, then south, then east. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order.

Here's what happens in the example above after each of the first few cycles:

After 1 cycle:
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....

After 2 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O

After 3 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
This process should work if you leave it running long enough, but you're still worried about the north support beams. To make sure they'll survive for a while, you need to calculate the total load on the north support beams after 1000000000 cycles.

In the above example, after 1000000000 cycles, the total load on the north support beams is 64.

Run the spin cycle for 1000000000 cycles. Afterward, what is the total load on the north support beams?
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum CellValue {
    StillRock,
    MovingRock,
    Nothing,
}

fn main() {
    println!("Hello day 14!");
    let input = read_to_string("inputs/day_14/input").unwrap();
    // let input = "O....#....\n\
    // O.OO#....#\n\
    // .....##...\n\
    // OO.#O....O\n\
    // .O.....O#.\n\
    // O.#..O.#.#\n\
    // ..O..#O..O\n\
    // .......O..\n\
    // #....###..\n\
    // #OO..#....";
    //
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => CellValue::StillRock,
                    'O' => CellValue::MovingRock,
                    '.' => CellValue::Nothing,
                    _ => panic!("Unknown cell value {}", c),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut saved_grids: HashMap<Vec<Vec<CellValue>>, u64> = HashMap::new();

    let mut iteration = 0;
    let no_of_iterations_to_run = 1000000000;

    let mut moved_by_iterations = false;
    while iteration < no_of_iterations_to_run {
        if !moved_by_iterations && saved_grids.contains_key(&grid) {
            let previous_iteration = saved_grids.get(&grid).unwrap();
            println!("Found repeating iteration prev {previous_iteration} current {iteration}");
            let cycle_length = iteration - previous_iteration;
            let remaining_iterations = no_of_iterations_to_run - iteration;
            let remaining_cycles = remaining_iterations / cycle_length;
            println!("Moving by {remaining_cycles} cycles with length {cycle_length}");
            moved_by_iterations = true;
            iteration += remaining_cycles * cycle_length;
            println!(
                "Moved to iteration {iteration} remaining iterations {}",
                no_of_iterations_to_run - remaining_iterations
            )
        } else {
            saved_grids.insert(grid.clone(), iteration);
            roll_cycle(&mut grid);
            iteration += 1
        }
    }

    let result = calculate_value(&grid);
    println!("Result: {}", result);
}

fn roll_cycle(grid: &mut Vec<Vec<CellValue>>) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn calculate_value(grid: &[Vec<CellValue>]) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|cell| cell == &&CellValue::MovingRock)
                .count()
        })
        .enumerate()
        .map(|(index, no_of_rocks)| no_of_rocks * (grid[0].len() - index))
        .sum::<usize>()
}

fn roll_north(grid: &mut Vec<Vec<CellValue>>) {
    for column in 0..grid[0].len() {
        let mut first_free: Option<usize> = None;
        for row in 0..grid.len() {
            match grid[row][column] {
                CellValue::StillRock => {
                    first_free = None;
                }
                CellValue::Nothing => {
                    first_free = Some(first_free.unwrap_or(row));
                }
                CellValue::MovingRock => {
                    if let Some(first_free_unpacked) = first_free {
                        grid[row][column] = CellValue::Nothing;
                        grid[first_free_unpacked][column] = CellValue::MovingRock;
                        first_free = Some(first_free_unpacked + 1);
                    }
                }
            }
        }
    }
}

fn roll_south(grid: &mut Vec<Vec<CellValue>>) {
    for column in 0..grid[0].len() {
        let mut first_free: Option<usize> = None;
        for row in (0..grid.len()).rev() {
            match grid[row][column] {
                CellValue::StillRock => {
                    first_free = None;
                }
                CellValue::Nothing => {
                    first_free = Some(first_free.unwrap_or(row));
                }
                CellValue::MovingRock => {
                    if let Some(first_free_unpacked) = first_free {
                        grid[row][column] = CellValue::Nothing;
                        grid[first_free_unpacked][column] = CellValue::MovingRock;
                        first_free = Some(first_free_unpacked - 1);
                    }
                }
            }
        }
    }
}

fn roll_west(grid: &mut Vec<Vec<CellValue>>) {
    for row in 0..grid.len() {
        let mut first_free: Option<usize> = None;
        for column in 0..grid[0].len() {
            match grid[row][column] {
                CellValue::StillRock => {
                    first_free = None;
                }
                CellValue::Nothing => {
                    first_free = Some(first_free.unwrap_or(column));
                }
                CellValue::MovingRock => {
                    if let Some(first_free_unpacked) = first_free {
                        grid[row][column] = CellValue::Nothing;
                        grid[row][first_free_unpacked] = CellValue::MovingRock;
                        first_free = Some(first_free_unpacked + 1);
                    }
                }
            }
        }
    }
}

fn roll_east(grid: &mut Vec<Vec<CellValue>>) {
    for row in 0..grid.len() {
        let mut first_free: Option<usize> = None;
        for column in (0..grid[0].len()).rev() {
            match grid[row][column] {
                CellValue::StillRock => {
                    first_free = None;
                }
                CellValue::Nothing => {
                    first_free = Some(first_free.unwrap_or(column));
                }
                CellValue::MovingRock => {
                    if let Some(first_free_unpacked) = first_free {
                        grid[row][column] = CellValue::Nothing;
                        grid[row][first_free_unpacked] = CellValue::MovingRock;
                        first_free = Some(first_free_unpacked - 1);
                    }
                }
            }
        }
    }
}
fn print_grid(grid: &[Vec<CellValue>]) {
    println!("Grid is:\n");
    for row in grid {
        for cell in row {
            match cell {
                CellValue::StillRock => print!("#"),
                CellValue::MovingRock => print!("O"),
                CellValue::Nothing => print!("."),
            }
        }
        println!();
    }
    println!();
}
