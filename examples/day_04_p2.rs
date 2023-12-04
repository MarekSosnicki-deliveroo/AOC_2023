use sscanf::sscanf;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 4!");
    let input = read_to_string("inputs/day_04/input").unwrap();

    // let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
    //                    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
    //                    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
    //                    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
    //                    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
    //                    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    // let cards_to_winnings: Vec<usize> = input
    //     .lines()
    //     .map(|line| {
    //         let (_, winning_numbers_str, ticket_numbers_str) =
    //             sscanf!(line, "Card {str}: {str} | {str}").unwrap();
    //         let winning_numbers: HashSet<_> = winning_numbers_str.split_whitespace().collect();
    //         let no_of_winning_tickets: usize = ticket_numbers_str
    //             .split_whitespace()
    //             .filter(|ticket| winning_numbers.contains(ticket))
    //             .count();
    //         no_of_winning_tickets
    //     })
    //     .collect();
    //
    // let mut no_of_cards_per_game: Vec<usize> = vec![1; cards_to_winnings.len()];
    //
    // for game_no in 0..no_of_cards_per_game.len() {
    //     let winnings_for_game = cards_to_winnings[game_no];
    //     let no_of_cards_for_game = no_of_cards_per_game[game_no];
    //     // println!("Game no: {game_no} no of winnings {winnings_for_game} no of cards {no_of_cards_for_game}");
    //     for winning_card in (game_no + 1)..(game_no + 1 + winnings_for_game) {
    //         no_of_cards_per_game[winning_card] += no_of_cards_for_game;
    //     }
    // }
    //
    // let result = no_of_cards_per_game.iter().sum::<usize>();

    let result: usize = input
        .lines()
        .map(|line| {
            let (_, winning_numbers_str, ticket_numbers_str) =
                sscanf!(line, "Card {str}: {str} | {str}").unwrap();
            let winning_numbers: HashSet<_> = winning_numbers_str.split_whitespace().collect();
            let no_of_winning_tickets: usize = ticket_numbers_str
                .split_whitespace()
                .filter(|ticket| winning_numbers.contains(ticket))
                .count();
            no_of_winning_tickets
        })
        .fold(
            (0, Vec::<usize>::default()),
            |(current_result, mut winnings_to_pass), winnings_this_game| {
                let no_of_games = if winnings_to_pass.is_empty() {
                    1
                } else {
                    1 + winnings_to_pass.remove(0)
                };
                if winnings_to_pass.len() < winnings_this_game {
                    winnings_to_pass.resize(winnings_this_game, 0);
                }

                for i in 0..winnings_this_game {
                    winnings_to_pass[i] += no_of_games;
                }
                (current_result + no_of_games, winnings_to_pass)
            },
        )
        .0;

    println!("Result: {}", result);
}
