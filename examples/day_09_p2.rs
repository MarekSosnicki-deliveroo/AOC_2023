use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 9!");
    let input = read_to_string("inputs/day_09/input").unwrap();

    // let input = "0 3 6 9 12 15\n\
    // 1 3 6 10 15 21\n\
    // 10 13 16 21 30 45";

    let result: i64 = input
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i64>>();

            let mut values_history = vec![values.clone()];

            while values_history.last().unwrap().iter().any(|v| *v != 0) {
                values_history.push(
                    values_history
                        .last()
                        .unwrap()
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| *b - *a)
                        .collect(),
                )
            }

            values_history
                .iter()
                .rev()
                .fold(0, |prediction, current_row| {
                    current_row.first().unwrap() - prediction
                })
        })
        .sum();

    println!("Result: {}", result);
}
