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

    let mut result = 0;

    for row in 0..characters.len() {
        let mut current_number = 0;
        let mut is_adjacent_to_symbol = false;
        for column in 0..characters[row].len() {
            if characters[row][column].is_ascii_digit() {
                current_number =
                    current_number * 10 + characters[row][column] as usize - '0' as usize;

                if !is_adjacent_to_symbol {
                    let min_row = if row == 0 { 0 } else { row - 1 };
                    let max_row = (row + 1).min(characters.len() - 1);
                    let min_column = if column == 0 { 0 } else { column - 1 };
                    let max_column = (column + 1).min(characters[row].len() - 1);

                    for row_index in min_row..=max_row {
                        for column_index in min_column..=max_column {
                            let character_to_check = characters[row_index][column_index];
                            if !character_to_check.is_ascii_digit() && character_to_check != '.' {
                                is_adjacent_to_symbol = true;
                            }
                        }
                    }
                }
            } else {
                if current_number != 0 {
                    println!("Found number {}", current_number);
                    if is_adjacent_to_symbol {
                        println!("Number {} is adjacent to symbol", current_number);
                        result += current_number;
                    }

                    current_number = 0;
                    is_adjacent_to_symbol = false;
                }
            }
        }
        if current_number != 0 {
            println!("Found number {}", current_number);
            if is_adjacent_to_symbol {
                println!("Number {} is adjacent to symbol", current_number);
                result += current_number;
            }

            current_number = 0;
            is_adjacent_to_symbol = false;
        }
    }

    println!("Result: {}", result);
}
