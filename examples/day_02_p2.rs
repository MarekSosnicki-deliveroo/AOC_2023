use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Hash, sscanf::FromScanf)]
enum Color {
    #[sscanf("red")]
    Red,
    #[sscanf("green")]
    Green,
    #[sscanf("blue")]
    Blue,
}

fn main() {
    println!("Hello day 2 part 2!");
    let input = read_to_string("inputs/day_02/input").unwrap();

    // let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
    //                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
    //                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
    //                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
    //                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let result: usize = input
        .lines()
        .map(|line| sscanf::sscanf!(line, "Game {usize}:{str}").unwrap())
        .map(|(game_id, game_data_str)| {
            let game_data = game_data_str
                .split(";")
                .map(|single_grab_string| {
                    single_grab_string
                        .split(",")
                        .map(|single_balls_str| {
                            let (num_of_balls, color) =
                                sscanf::sscanf!(single_balls_str, " {usize} {Color}").unwrap();
                            (color, num_of_balls)
                        })
                        .collect::<HashMap<Color, usize>>()
                })
                .collect_vec();
            (game_id, game_data)
        })
        .inspect(|(game_id, game_data)| println!("Game id is {game_id}, game is {game_data:?}"))
        .map(|(_, game_data)| {
            [Color::Blue, Color::Red, Color::Green]
                .into_iter()
                .map(|color| {
                    game_data
                        .iter()
                        .map(|single_grab| *single_grab.get(&color).unwrap_or(&0))
                        .max()
                        .unwrap_or(0)
                })
                .product::<usize>()
        })
        .inspect(|product| println!("Product is {product}"))
        .sum();

    println!("Result: {}", result);
}
