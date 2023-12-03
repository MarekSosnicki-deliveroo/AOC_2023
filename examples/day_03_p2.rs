use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 3!");
    let input = read_to_string("inputs/day_03/input").unwrap();

    // let input = "467..114..\n\
    //                   ...*......\n\
    //                   ..35..633.\n\
    //                   ......#...\n\
    //                   617*......\n\
    //                   .....+.58.\n\
    //                   ..592.....\n\
    //                   ......755.\n\
    //                   ...$.*....\n\
    //                   .664.598..";

    let characters: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Position of gear to the numbers it is adjacent to
    let mut numbers_adjacent_to_gears: HashMap<(usize, usize), Vec<usize>> = HashMap::default();

    for row in 0..characters.len() {
        let mut current_number = 0;
        let mut adjacent_gears: HashSet<(usize, usize)> = HashSet::default();
        for column in 0..characters[row].len() {
            if characters[row][column].is_ascii_digit() {
                current_number =
                    current_number * 10 + characters[row][column] as usize - '0' as usize;

                let min_row = if row == 0 { 0 } else { row - 1 };
                let max_row = (row + 1).min(characters.len() - 1);
                let min_column = if column == 0 { 0 } else { column - 1 };
                let max_column = (column + 1).min(characters[row].len() - 1);

                for row_index in min_row..=max_row {
                    for column_index in min_column..=max_column {
                        if characters[row_index][column_index] == '*' {
                            adjacent_gears.insert((row_index, column_index));
                        }
                    }
                }
            } else {
                if current_number != 0 {
                    println!("Adding numeber {} to {:?}", current_number, adjacent_gears);
                    for (gear_row, gear_column) in adjacent_gears {
                        numbers_adjacent_to_gears
                            .entry((gear_row, gear_column))
                            .or_default()
                            .push(current_number);
                    }
                    adjacent_gears = HashSet::default();
                    current_number = 0;
                }
            }
        }
        if current_number != 0 {
            println!("Adding numeber {} to {:?}", current_number, adjacent_gears);
            for (gear_row, gear_column) in adjacent_gears {
                numbers_adjacent_to_gears
                    .entry((gear_row, gear_column))
                    .or_default()
                    .push(current_number);
            }
        }
    }

    let mut result = 0;
    for numbers_adjacent_to_gear in numbers_adjacent_to_gears.values() {
        if numbers_adjacent_to_gear.len() == 2 {
            result += numbers_adjacent_to_gear[0] * numbers_adjacent_to_gear[1];
        }
    }

    println!("Result: {}", result);
}
