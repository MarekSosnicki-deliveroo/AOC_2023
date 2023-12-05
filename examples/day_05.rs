use boolinator::Boolinator;
use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

#[derive(Debug)]

struct Transformation {
    min_from: i64,
    min_to: i64,
    range: i64,
}

impl Transformation {
    fn transform(&self, input: i64) -> Option<i64> {
        let away_from_min = input - self.min_from;
        (away_from_min >= 0 && away_from_min < self.range)
            .as_some_from(|| self.min_to + away_from_min)
    }
}

#[derive(Debug)]
struct TransformationGroup {
    name: String,
    transformations: Vec<Transformation>,
}

impl TransformationGroup {
    fn new(name: String, transformations: Vec<Transformation>) -> Self {
        Self {
            transformations,
            name,
        }
    }

    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();

        let first_line = lines.next().unwrap();
        let name = sscanf!(first_line, "{str}-to-{str} map:")
            .unwrap()
            .1
            .to_string();

        TransformationGroup::new(
            name,
            lines
                .map(|line| {
                    let (min_to, min_from, range) = sscanf!(line, "{i64} {i64} {i64}").unwrap();
                    Transformation {
                        min_from,
                        min_to,
                        range,
                    }
                })
                .collect(),
        )
    }

    fn transform(&self, input: i64) -> i64 {
        self.transformations
            .iter()
            .find_map(|t| t.transform(input))
            .unwrap_or(input)
    }
}

fn main() {
    println!("Hello day 5!");
    let input = read_to_string("inputs/day_05/input").unwrap();
    // let input = "seeds: 79 14 55 13\n\
    //                    \n\
    //                    seed-to-soil map:\n\
    //                    50 98 2\n\
    //                    52 50 48\n\
    //                    \n\
    //                    soil-to-fertilizer map:\n\
    //                    0 15 37\n\
    //                    37 52 2\n\
    //                    39 0 15\n\
    //                    \n\
    //                    fertilizer-to-water map:\n\
    //                    49 53 8\n\
    //                    0 11 42\n\
    //                    42 0 7\n\
    //                    57 7 4\n\
    //                    \n\
    //                    water-to-light map:\n\
    //                    88 18 7\n\
    //                    18 25 70\n\
    //                    \n\
    //                    light-to-temperature map:\n\
    //                    45 77 23\n\
    //                    81 45 19\n\
    //                    68 64 13\n\
    //                    \n\
    //                    temperature-to-humidity map:\n\
    //                    0 69 1\n\
    //                    1 0 69\n\
    //                    \n\
    //                    humidity-to-location map:\n\
    //                    60 56 37\n\
    //                    56 93 4";

    let mut input_groups = input.split("\n\n");

    let seeds_str = input_groups.next().unwrap();
    let seeds = sscanf!(seeds_str, "seeds: {str}")
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect_vec();

    println!("Seeds {:?}", seeds);

    let transformation_groups = input_groups
        .map(|group| TransformationGroup::from_str(group))
        .collect_vec();

    println!("Transformation groups {:?}", transformation_groups);

    let result = seeds
        .into_iter()
        .map(|seed| {
            println!("Seed value {}", seed);
            transformation_groups.iter().fold(seed, |seed, group| {
                println!(
                    "Transforming into group {} with value {}",
                    group.name,
                    group.transform(seed),
                );
                group.transform(seed)
            })
        })
        .min()
        .unwrap();

    println!("Result is {}", result);
}
