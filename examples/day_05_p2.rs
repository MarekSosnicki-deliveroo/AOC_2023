use sscanf::sscanf;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 5!");
    let input = read_to_string("inputs/day_05/input").unwrap();

    let mut categories = input.split("\n\n");
    let seeds = sscanf!(categories.next().unwrap(), "seeds: {str}").unwrap();
    let seeds: Vec<_> = seeds
        .split(" ")
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .collect();

    let mut intervals: Vec<_> = merge_intervals(
        seeds
            .iter()
            .step_by(2)
            .cloned()
            .zip(seeds.iter().skip(1).step_by(2).cloned())
            .collect(),
    );

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

    for map in maps {
        intervals = apply(map, intervals);
    }

    let result = intervals[0].0;

    println!("Result: {:?}", result);
}

fn merge_intervals(mut intervals: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    intervals.sort_by_key(|(x, _)| *x);
    let mut i: usize = 0;
    let mut j: usize = 0;
    while j < intervals.len() {
        if i < j {
            intervals[i] = intervals[j];
        }
        let mut interval_end = intervals[i].0 + intervals[i].1;
        j += 1;
        while j < intervals.len() && intervals[j].0 <= interval_end {
            let end_j = intervals[j].0 + intervals[j].1;
            if end_j > interval_end {
                interval_end = end_j;
                intervals[i].1 = interval_end - intervals[i].0;
            }
            j += 1;
        }
        i += 1;
    }
    intervals.truncate(i);
    intervals
}

fn apply(map: Vec<(usize, usize, usize)>, intervals: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut map = map.into_iter().peekable();
    for (mut s, mut length) in intervals {
        while map.next_if(|(_, x, l)| x + l <= s).is_some() {}
        while let Some((y, x, l2)) = map.peek() {
            let mut x = *x;
            if x > s + length {
                break;
            }
            let mut l2 = *l2;
            let mut y = *y;
            if x < s {
                let diff = s - x;
                y += diff;
                x = s;
                l2 -= diff;
            } else if x > s {
                // no interval is matching this s => identity map
                let diff = x - s;
                result.push((s, diff));
                s = x;
                length -= diff;
            }
            // x == s from here on
            if l2 < length {
                result.push((y, l2));
                s += l2;
                length -= l2;
            } else {
                result.push((y, length));
                length = 0;
                break;
            }

            map.next();
        }
        if length > 0 {
            result.push((s, length));
        }
    }
    merge_intervals(result)
}
