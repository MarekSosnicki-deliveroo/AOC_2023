use itertools::Itertools;
use std::cmp;
use std::fs::read_to_string;

/*
--- Part Two ---
The galaxies are much older (and thus much farther apart) than the researcher initially estimated.

Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.

(In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)

Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
 */
fn main() {
    println!("Hello day 11!");
    let input = read_to_string("inputs/day_11/input").unwrap();

    // let input = "...#......\n\
    // .......#..\n\
    // #.........\n\
    // ..........\n\
    // ......#...\n\
    // .#........\n\
    // .........#\n\
    // ..........\n\
    // .......#..\n\
    // #...#.....";

    let galaxies: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { true } else { false })
                .collect()
        })
        .collect();

    let (row_sizes, column_sizes) = galaxy_sizes(&galaxies);

    println!("Galaxies read: ");
    print_galaxies(&galaxies);

    println!("Galaxies expanded: ");
    print_galaxies(&galaxies);

    let galaxy_coordinates = (0..galaxies.len())
        .cartesian_product(0..galaxies[0].len())
        .filter(|(row, column)| galaxies[*row][*column])
        .collect_vec();

    println!("Coordinates {:?}", galaxy_coordinates);

    let result: i64 = galaxy_coordinates
        .iter()
        .combinations(2)
        .map(|x| {
            let ((row1, col1), (row2, col2)) = (*x[0], *x[1]);

            let row_min = cmp::min(row1, row2);
            let row_max = cmp::max(row1, row2);
            let row_diff: usize = row_sizes[row_min..row_max].iter().sum();

            let col_min = cmp::min(col1, col2);
            let col_max = cmp::max(col1, col2);
            let col_diff: usize = column_sizes[col_min..col_max].iter().sum();

            row_diff as i64 + col_diff as i64
        })
        .sum();

    println!("Result is: {:?}", result);
}

fn expand_galaxies(input: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut expanded = input
        .iter()
        .map(|row| {
            if row.iter().all(|v| !*v) {
                vec![row.clone(), row.clone()]
            } else {
                vec![row.clone()]
            }
        })
        .flatten()
        .collect_vec();

    for column in (0..expanded[0].len()).rev() {
        if (0..expanded.len()).all(|row| expanded[row][column] == false) {
            for row in 0..expanded.len() {
                expanded[row].insert(column, false)
            }
        }
    }

    expanded
}

fn galaxy_sizes(input: &Vec<Vec<bool>>) -> (Vec<usize>, Vec<usize>) {
    let expanded_size = 10;

    let row_sizes = input
        .iter()
        .map(|row| {
            if row.iter().all(|v| !*v) {
                1_000_000
            } else {
                1
            }
        })
        .collect_vec();

    let column_sizes = (0..input[0].len())
        .map(|column| {
            if input.iter().all(|row| !row[column]) {
                1_000_000
            } else {
                1
            }
        })
        .collect();
    (row_sizes, column_sizes)
}

fn print_galaxies(input: &Vec<Vec<bool>>) {
    for row in input {
        for col in row {
            if *col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
