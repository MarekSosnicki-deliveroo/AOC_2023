use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 1!");
    let input = read_to_string("inputs/day_13/input").unwrap();

    //     let input = "\
    // #.##..##.\n\
    // ..#.##.#.\n\
    // ##......#\n\
    // ##......#\n\
    // ..#.##.#.\n\
    // ..##..##.\n\
    // #.#.##.#.\n\
    // \n\
    // #...##..#\n\
    // #....#..#\n\
    // ..##..###\n\
    // #####.##.\n\
    // #####.##.\n\
    // ..##..###\n\
    // #....#..#";

    let result: u64 = input.split("\n\n").map(|puzzle| calculate(puzzle)).sum();

    println!("Result: {}", result);
}

fn calculate(puzzle: &str) -> u64 {
    println!("Input is\n{}", puzzle);

    let puzzle = puzzle
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { true } else { false })
                .collect_vec()
        })
        .collect_vec();

    // 0123
    // 1 -> 0 12
    // 2 -> 01 2

    for column_to_check_after in 0..(puzzle[0].len() - 1) {
        println!("Checking column after {:?}", column_to_check_after);

        let mut all_rows_reflect = true;
        for row in 0..puzzle.len() {
            let all_columns_to_left = (0..=column_to_check_after).rev();
            let all_columns_to_right = (column_to_check_after + 1)..puzzle[0].len();
            for (column_left, column_right) in all_columns_to_left.zip(all_columns_to_right) {
                if puzzle[row][column_left] != puzzle[row][column_right] {
                    all_rows_reflect = false;
                    break;
                }
            }
            if !all_rows_reflect {
                break;
            }
        }

        if all_rows_reflect {
            println!("Find reflecting column after {}", column_to_check_after);
            return (column_to_check_after + 1) as u64;
        }
    }

    for row_to_put_line_after in 0..(puzzle.len() - 1) {
        println!("Checking row after {:?}", row_to_put_line_after);

        let mut all_columns_reflect = true;
        for column in 0..puzzle[row_to_put_line_after].len() {
            let all_rows_above = (0..=row_to_put_line_after).rev();
            let all_rows_below = (row_to_put_line_after + 1)..puzzle.len();
            for (row_above, row_below) in all_rows_above.zip(all_rows_below) {
                // HERE

                // println!("Row above {row_above}, row below {row_below}");
                // println!(
                //     "Comparing column {column} between {:?}, and {:?}",
                //     puzzle[row_above], puzzle[row_below]
                // );
                if puzzle[row_above][column] != puzzle[row_below][column] {
                    all_columns_reflect = false;
                    break;
                }
            }
            if !all_columns_reflect {
                break;
            }
        }

        if all_columns_reflect {
            println!("Find reflecting row after {}", row_to_put_line_after);
            return 100 * (row_to_put_line_after + 1) as u64;
        }
    }

    panic!("No column found")
}
