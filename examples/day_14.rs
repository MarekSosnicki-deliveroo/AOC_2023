use itertools::Itertools;
use std::fs::read_to_string;
use std::os::unix::raw::gid_t;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellValue {
    StillRock,
    MovingRock,
    Nothing,
}

fn main() {
    println!("Hello day 14!");
    let input = read_to_string("inputs/day_14/input").unwrap();
    //     let input = "O....#....\n\
    // O.OO#....#\n\
    // .....##...\n\
    // OO.#O....O\n\
    // .O.....O#.\n\
    // O.#..O.#.#\n\
    // ..O..#O..O\n\
    // .......O..\n\
    // #....###..\n\
    // #OO..#....";

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

    for column in 0..grid[0].len() {
        let mut first_free_above: Option<usize> = None;
        for row in 0..grid.len() {
            match grid[row][column] {
                CellValue::StillRock => {
                    first_free_above = None;
                }
                CellValue::Nothing => {
                    first_free_above = Some(first_free_above.unwrap_or(row));
                }
                CellValue::MovingRock => {
                    if let Some(first_free_above_unpacked) = first_free_above {
                        grid[row][column] = CellValue::Nothing;
                        grid[first_free_above_unpacked][column] = CellValue::MovingRock;
                        first_free_above = Some(first_free_above_unpacked + 1);
                    }
                }
            }
        }
    }

    print_grid(&grid);

    let result = grid
        .iter()
        .map(|row| {
            row.iter()
                .filter(|cell| cell == &&CellValue::MovingRock)
                .count()
        })
        .enumerate()
        .map(|(index, no_of_rocks)| no_of_rocks * (grid[0].len() - index))
        .sum::<usize>();
    println!("Result: {}", result);
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
