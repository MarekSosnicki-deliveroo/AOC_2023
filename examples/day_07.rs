use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

fn strength(cards: &[Card]) -> usize {
    let num_of_cards_per_group = cards
        .iter()
        .sorted()
        .group_by(|c| *c)
        .into_iter()
        .map(|(_, group)| group.count())
        .sorted()
        .collect_vec();

    let no_of_cards_in_biggest_group = *num_of_cards_per_group.last().unwrap();

    match no_of_cards_in_biggest_group {
        5 => 7,
        4 => 6,
        3 => {
            let no_of_cards_in_second_biggest_group =
                num_of_cards_per_group.get(num_of_cards_per_group.len() - 2);

            if *no_of_cards_in_second_biggest_group.unwrap() == 2 {
                5
            } else {
                4
            }
        }
        2 => {
            let no_of_cards_in_second_biggest_group =
                num_of_cards_per_group.get(num_of_cards_per_group.len() - 2);

            if *no_of_cards_in_second_biggest_group.unwrap() == 2 {
                3
            } else {
                2
            }
        }
        1 => 1,
        _ => panic!("Invalid number of cards in biggest group"),
    }
}

#[derive(Copy, Clone, PartialOrd, Ord, sscanf::FromScanf, Eq, PartialEq)]
enum Card {
    #[sscanf("2")]
    N2,
    #[sscanf("3")]
    N3,
    #[sscanf("4")]
    N4,
    #[sscanf("5")]
    N5,
    #[sscanf("6")]
    N6,
    #[sscanf("7")]
    N7,
    #[sscanf("8")]
    N8,
    #[sscanf("9")]
    N9,
    #[sscanf("T")]
    T,
    #[sscanf("J")]
    J,
    #[sscanf("Q")]
    Q,
    #[sscanf("K")]
    K,
    #[sscanf("A")]
    A,
}

fn main() {
    println!("Hello day 7!");
    let input = read_to_string("inputs/day_07/input").unwrap();
    // let input = "32T3K 765\n\
    //                    T55J5 684\n\
    //                    KK677 28\n\
    //                    KTJJT 220\n\
    //                    QQQJA 483";

    let result: u64 = input
        .lines()
        .map(|line| sscanf!(line, "{str} {u64}").unwrap())
        .map(|(cards, bid)| {
            let cards = cards
                .chars()
                .map(|c| {
                    let c = c.to_string();
                    sscanf!(c, "{Card}").unwrap()
                })
                .collect_vec();
            // println!(
            //     "cards: {:?}, strength: {:?}, bid: {:?}",
            //     cards,
            //     strength(&cards),
            //     bid
            // );
            (cards.clone(), strength(&cards), bid)
        })
        .sorted_by_key(|(cards, strength, bid)| (*strength, cards.clone()))
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as u64 + 1) * bid)
        .sum();
    println!("Result: {}", result);
}
