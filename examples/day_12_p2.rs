use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

/*
--- Part Two ---
As you look out at the field of springs, you feel like there are way more springs than the condition records list. When you examine the records, you discover that they were actually folded up this whole time!

To unfold the records, on each row, replace the list of spring conditions with five copies of itself (separated by ?) and replace the list of contiguous groups of damaged springs with five copies of itself (separated by ,).

So, this row:

.# 1
Would become:

.#?.#?.#?.#?.# 1,1,1,1,1
The first line of the above example would become:

???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3
In the above example, after unfolding, the number of possible arrangements for some rows is now much larger:

???.### 1,1,3 - 1 arrangement
.??..??...?##. 1,1,3 - 16384 arrangements
?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
????.#...#... 4,1,1 - 16 arrangements
????.######..#####. 1,6,5 - 2500 arrangements
?###???????? 3,2,1 - 506250 arrangements
After unfolding, adding all of the possible arrangement counts together produces 525152.

Unfold your condition records; what is the new sum of possible arrangement counts?
 */

// ???.###????.###????.###????.###????.###
// 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3

// ???.###????.### (x4)

/// #.?  1,1 N
/// #.? ? #.? ? #.? ? #.? ? #.?
/// assume ? = . => N^5
/// assume ? = # => (#.? ?) => M or (? #.?)
/// ???????  // 1

fn main() {
    println!("Hello day 12!");
    let input = read_to_string("inputs/day_12/input").unwrap();

    // let input = "???.### 1,1,3\n\
    // .??..??...?##. 1,1,3\n\
    // ?#?#?#?#?#?#?#? 1,3,1,6\n\
    // ????.#...#... 4,1,1\n\
    // ????.######..#####. 1,6,5\n\
    // ?###???????? 3,2,1";

    let start = std::time::Instant::now();

    let result: usize = input.lines().map(|line| no_of_possibilities(line)).sum();

    println!("Finished in {} us", start.elapsed().as_secs());

    println!("Result: {}", result);
}

fn no_of_possibilities(line: &str) -> usize {
    let (group_str, expected_groups_str) = sscanf!(line, "{str} {str}").unwrap();

    let group_str = vec![group_str; 5].join("?");
    let expected_groups_str = vec![expected_groups_str; 5].join(",");

    println!("{:?}", group_str);
    println!("{:?}", expected_groups_str);

    let row_with_unknowns = group_str.chars().collect::<Vec<_>>();
    let expected_groups = expected_groups_str
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let no_of_hashes = row_with_unknowns.iter().filter(|c| **c == '#').count();
    let missing_no_of_hashes = expected_groups.iter().sum::<usize>() - no_of_hashes;

    let result = recursive_find_no(
        &row_with_unknowns,
        &expected_groups,
        0,
        0,
        0,
        missing_no_of_hashes,
    );

    println!("Result for {line} is <<{result}>>");

    result
}

fn recursive_find_no(
    row: &[char],
    expected_groups: &[usize],
    mut current_character_index: usize,
    mut current_group_size: usize,
    mut current_group_index: usize,
    missing_no_of_hashes: usize,
) -> usize {
    while current_character_index < row.len() && row[current_character_index] != '?' {
        if row[current_character_index] == '#' {
            current_group_size += 1;
        } else if current_group_size > 0 {
            if expected_groups.get(current_group_index) != Some(&current_group_size) {
                return 0;
            }
            current_group_index += 1;
            current_group_size = 0;
        }
        current_character_index += 1;
    }

    if current_character_index < row.len() {
        assert_eq!(
            row[current_character_index], '?',
            "something wrong with passing through the row"
        );

        let current_group_index_with_dot = if current_group_size > 0 {
            if expected_groups.get(current_group_index) != Some(&current_group_size) {
                None
            } else {
                Some(current_group_index + 1)
            }
        } else {
            Some(current_group_index)
        };
        let no_of_combinations_when_current_is_dot = current_group_index_with_dot
            .map(|current_groups_with_dot| {
                recursive_find_no(
                    row,
                    expected_groups,
                    current_character_index + 1,
                    0,
                    current_groups_with_dot,
                    missing_no_of_hashes,
                )
            })
            .unwrap_or(0);

        if missing_no_of_hashes == 0 {
            no_of_combinations_when_current_is_dot
        } else {
            let no_of_combinations_when_curren_is_hash = recursive_find_no(
                row,
                expected_groups,
                current_character_index + 1,
                current_group_size + 1,
                current_group_index,
                missing_no_of_hashes - 1,
            );

            no_of_combinations_when_curren_is_hash + no_of_combinations_when_current_is_dot
        }
    } else {
        if current_group_size > 0 {
            if expected_groups.get(current_group_index) == Some(&current_group_size)
                && (current_group_index == expected_groups.len() - 1)
            {
                1
            } else {
                0
            }
        } else {
            if current_group_index == expected_groups.len() {
                1
            } else {
                0
            }
        }
    }
}
