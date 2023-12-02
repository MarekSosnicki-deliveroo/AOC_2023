use std::cmp::max_by_key;
use std::fs::read_to_string;
use boolinator::Boolinator;

fn main() {
    println!("Hello day 1 part 2!");
    let input = read_to_string("inputs/day_01/input").unwrap();

    let numbers_as_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
//     let input = r#"two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen"#;

    let result: usize = input.lines().map(|line| {
        let first = numbers_as_words.iter().enumerate().filter_map(|(number, word)| {
            line.find(word).map(|index| (index, number + 1))
        }).chain(
            line.chars().position(|c| c.is_ascii_digit()).map(|position| (position, line.chars().nth(position).unwrap() as usize - '0' as usize)).into_iter()
        ).min_by_key(|(position, value)| *position).unwrap().1;

        let last = numbers_as_words.iter().enumerate().filter_map(|(number, word)| {
            line.rfind(word).map(|index| (index, number + 1))
        }).chain(
            line.chars().enumerate().filter_map(|(position, c)| c.is_digit(10).as_some_from(|| (position, c as usize - '0' as usize))).into_iter()
        ).max_by_key(|(position, value)| *position).unwrap().1;

        let value = first * 10 + last;

        value
    }).sum();
    println!("Result: {}", result);
}
