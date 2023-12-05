extern crate core;

use boolinator::Boolinator;
use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
struct Values {
    min: i64,
    range: i64,
}

#[derive(Debug)]
struct ValueRanges {
    ranges: Vec<Values>,
}

#[derive(Debug)]
struct Transformation {
    min_from: i64,
    min_to: i64,
    range: i64,
}

impl Transformation {
    fn transform(&self, values: Values) -> Option<(Values, Vec<Values>)> {
        let transformation_min = self.min_from;
        let transformation_max = self.min_from + self.range - 1;

        let to_transform_min = values.min;
        let to_transform_max = values.min + values.range - 1;

        let transformable_min = transformation_min.max(to_transform_min);
        let transformable_max = transformation_max.min(to_transform_max);

        if to_transform_max < transformation_min || to_transform_min > transformation_max {
            return None;
        }

        let transformed = Values {
            min: self.min_to + (transformable_min - transformation_min),
            range: transformable_max - transformable_min,
        };

        let mut lefrovers = vec![];

        if transformation_min > to_transform_min {
            lefrovers.push(Values {
                min: to_transform_min,
                range: transformable_min - to_transform_min,
            })
        }

        if transformation_max < to_transform_max {
            lefrovers.push(Values {
                min: transformation_max + 1,
                range: to_transform_max - transformation_max,
            })
        }

        Some((transformed, lefrovers))
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

    fn transform(&self, ranges: ValueRanges) -> ValueRanges {
        let mut result_values = vec![];
        let mut remaining_values = ranges.ranges;

        while !remaining_values.is_empty() {
            let values = remaining_values.remove(0);

            // println!("Transforming {values:?}");
            let (transformed, leftover) = self
                .transformations
                .iter()
                .find_map(|t| t.transform(values))
                .unwrap_or((values, vec![]));
            // println!("Got {transformed:?}");
            // println!("Leftover {leftover:?}");

            result_values.push(transformed);
            remaining_values.extend(leftover);
        }
        ValueRanges {
            ranges: result_values,
        }
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
        .tuples()
        .map(|(seed_start, range)| {
            println!("seed_start, seed_end {} {}", seed_start, range);
            let seed_start = seed_start.parse::<i64>().unwrap();
            let range = range.parse::<i64>().unwrap();
            ValueRanges {
                ranges: vec![Values {
                    min: seed_start,
                    range,
                }],
            }
        })
        .collect_vec();

    println!("Seeds {:?}", seeds);

    let transformation_groups = input_groups
        .map(|group| TransformationGroup::from_str(group))
        .collect_vec();

    // println!("Transformation groups {:?}", transformation_groups);

    let result = seeds
        .into_iter()
        .map(|seed_values| {
            println!("Seed values {:?}", seed_values);
            transformation_groups
                .iter()
                .fold(seed_values, |seed_values, group| {
                    println!("Transformation {:?}", group);

                    let transformation_result = group.transform(seed_values);
                    println!(
                        "Transforming into group {} with values {:?}",
                        group.name, transformation_result
                    );
                    transformation_result
                })
        })
        .flat_map(|value_ranges| value_ranges.ranges.into_iter())
        .map(|values| values.min)
        .inspect(|result| println!("Final value is {result}"))
        .min()
        .unwrap();

    println!("Result is {}", result);
}
