use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

/*
--- Part Two ---
To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.

J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind. However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.

Now, the above example goes very differently:

32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
KK677 is now the only two pair, making it the second-weakest hand.
T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
With the new joker rule, the total winnings in this example are 5905.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?

*/
fn strength(cards: &[Card]) -> usize {
    let jokers = cards.iter().filter(|c| **c == Card::J).count();

    let num_of_cards_per_group = cards
        .iter()
        .filter(|c| **c != Card::J)
        .sorted()
        .group_by(|c| *c)
        .into_iter()
        .map(|(_, group)| group.count())
        .sorted()
        .collect_vec();

    let no_of_cards_in_biggest_group = *num_of_cards_per_group.last().unwrap_or(&0) + jokers;

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
    #[sscanf("J")]
    J,
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
    #[sscanf("Q")]
    Q,
    #[sscanf("K")]
    K,
    #[sscanf("A")]
    A,
}

fn main() {
    println!("Hello day 7 part2!");
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
