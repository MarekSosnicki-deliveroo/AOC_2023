use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 11!");
    let input = read_to_string("inputs/day_11/input").unwrap();

    let input = "...#......\n\
    .......#..\n\
    #.........\n\
    ..........\n\
    ......#...\n\
    .#........\n\
    .........#\n\
    ..........\n\
    .......#..\n\
    #...#.....";

    let galaxies_read: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { true } else { false })
                .collect()
        })
        .collect();

    println!("Galaxies read: ");
    print_galaxies(&galaxies_read);

    let galaxies_expanded = expand_galaxies(&galaxies_read);

    println!("Galaxies expanded: ");
    print_galaxies(&galaxies_expanded);

    let galaxy_coordinates = (0..galaxies_expanded.len())
        .cartesian_product(0..galaxies_expanded[0].len())
        .filter(|(row, column)| galaxies_expanded[*row][*column])
        .collect_vec();

    println!("Coordinates {:?}", galaxy_coordinates);

    let result: i64 = galaxy_coordinates
        .iter()
        .combinations(2)
        .map(|x| {
            let ((row1, col1), (row2, col2)) = (*x[0], *x[1]);
            let row_diff = (row1 as i64 - row2 as i64).abs();
            let column_diff = (col1 as i64 - col2 as i64).abs();
            row_diff + column_diff
        })
        .sum();

    println!("result\n: {:?}", result);
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
