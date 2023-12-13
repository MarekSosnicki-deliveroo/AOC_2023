use itertools::Itertools;
use std::fs::read_to_string;

/*
--- Part Two ---
You resume walking through the valley of mirrors and - SMACK! - run directly into one. Hopefully nobody was watching, because that must have been pretty embarrassing.

Upon closer inspection, you discover that every mirror has exactly one smudge: exactly one . or # should be the opposite type.

In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to be valid. (The old reflection line won't necessarily continue being valid after the smudge is fixed.)

Here's the above example again:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
The first pattern's smudge is in the top-left corner. If the top-left # were instead ., it would have a different, horizontal line of reflection:

1 ..##..##. 1
2 ..#.##.#. 2
3v##......#v3
4^##......#^4
5 ..#.##.#. 5
6 ..##..##. 6
7 #.#.##.#. 7
With the smudge in the top-left corner repaired, a new horizontal line of reflection between rows 3 and 4 now exists. Row 7 has no corresponding reflected row and can be ignored, but every other row matches exactly: row 1 matches row 6, row 2 matches row 5, and row 3 matches row 4.

In the second pattern, the smudge can be fixed by changing the fifth symbol on row 2 from . to #:

1v#...##..#v1
2^#...##..#^2
3 ..##..### 3
4 #####.##. 4
5 #####.##. 5
6 ..##..### 6
7 #....#..# 7
Now, the pattern has a different horizontal line of reflection between rows 1 and 2.

Summarize your notes as before, but instead use the new different reflection lines. In this example, the first pattern's new horizontal line has 3 rows above it and the second pattern's new horizontal line has 1 row above it, summarizing to the value 400.

In each pattern, fix the smudge and find the different line of reflection. What number do you get after summarizing the new reflection line in each pattern in your notes?
*/
fn main() {
    println!("Hello day 1!");
    let input = read_to_string("inputs/day_13/input").unwrap();
    //
    // let input = "\
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
        let mut not_matching_columns = 0;
        for row in 0..puzzle.len() {
            let all_columns_to_left = (0..=column_to_check_after).rev();
            let all_columns_to_right = (column_to_check_after + 1)..puzzle[0].len();
            for (column_left, column_right) in all_columns_to_left.zip(all_columns_to_right) {
                if puzzle[row][column_left] != puzzle[row][column_right] {
                    not_matching_columns += 1;
                }
            }
        }

        if not_matching_columns == 1 {
            println!("Find reflecting column after {}", column_to_check_after);
            return (column_to_check_after + 1) as u64;
        }
    }

    for row_to_put_line_after in 0..(puzzle.len() - 1) {
        let mut not_matching_rows = 0;
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
                    not_matching_rows += 1;
                }
            }
        }

        if not_matching_rows == 1 {
            println!("Find reflecting row after {}", row_to_put_line_after);
            return 100 * (row_to_put_line_after + 1) as u64;
        }
    }

    panic!("No column found")
}
