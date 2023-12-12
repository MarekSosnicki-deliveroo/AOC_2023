use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 12!");
    let input = read_to_string("inputs/day_12/input").unwrap();

    //     let input = "???.### 1,1,3\n\
    // .??..??...?##. 1,1,3\n\
    // ?#?#?#?#?#?#?#? 1,3,1,6\n\
    // ????.#...#... 4,1,1\n\
    // ????.######..#####. 1,6,5\n\
    // ?###???????? 3,2,1";

    let start = std::time::Instant::now();

    let result: usize = input.lines().map(|line| no_of_possibilities(line)).sum();

    println!("Finished in {} us", start.elapsed().as_micros());

    println!("Result: {}", result);
}

fn no_of_possibilities(line: &str) -> usize {
    let (group_str, expected_groups_str) = sscanf!(line, "{str} {str}").unwrap();

    let row_with_unknowns = group_str.chars().collect::<Vec<_>>();
    let expected_groups = expected_groups_str
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let positions_of_unknowns = row_with_unknowns
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '?')
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let no_of_hashes = row_with_unknowns.iter().filter(|c| **c == '#').count();
    let missing_no_of_hashes = expected_groups.iter().sum::<usize>() - no_of_hashes;

    positions_of_unknowns
        .iter()
        .combinations(missing_no_of_hashes)
        .filter(|combination| {
            let mut row_copy = row_with_unknowns.clone();
            for position in combination {
                row_copy[**position] = '#';
            }
            no_of_groups(&row_copy) == expected_groups
        })
        .count()
}

fn no_of_groups(row: &[char]) -> Vec<usize> {
    let mut result = vec![];
    let mut no_of_hashes = 0;
    for c in row {
        if *c == '#' {
            no_of_hashes += 1;
        } else if no_of_hashes > 0 {
            result.push(no_of_hashes);
            no_of_hashes = 0;
        }
    }
    if no_of_hashes > 0 {
        result.push(no_of_hashes);
    }
    result
}
