use sscanf::sscanf;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 5!");
    let input = read_to_string("inputs/day_05/input").unwrap();

    let mut categories = input.split("\n\n");
    let seeds = sscanf!(categories.next().unwrap(), "seeds: {str}").unwrap();
    let seeds = seeds
        .split(" ")
        .map(|s| usize::from_str_radix(s, 10).unwrap());

    let maps: Vec<_> = categories
        .map(|s| {
            let mut ranges: Vec<_> = s
                .split("\n")
                .skip(1)
                .map(|l| sscanf!(l, "{usize} {usize} {usize}").unwrap())
                .collect();

            ranges.sort_by_key(|(_, s, _)| *s);

            ranges
        })
        .collect();

    let result = seeds
        .map(|mut s| {
            for map in &maps {
                println!("Apply map to {}", s);
                s = apply(map, s);
                println!("Result: {}", s);
            }
            s
        })
        .min()
        .unwrap();

    println!("Result: {:?}", result);
}

fn apply(map: &[(usize, usize, usize)], s: usize) -> usize {
    map.iter()
        .take_while(|(_, x, _)| *x <= s)
        .find(|(_, x, l)| *x + *l > s)
        .map(|(y, x, _)| y + s - x)
        .unwrap_or(s)
}
