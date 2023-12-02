use std::fs::read_to_string;

fn main() {
    println!("Hello day 1!");
    let input = read_to_string("inputs/day_01/input").unwrap();

    let result : usize = input.lines().map(|line| {
        let first = line.chars().find(|c| c.is_digit(10)).unwrap();
        let last = line.chars().rfind(|c| c.is_digit(10)).unwrap();
        println!("first '{first}' last '{last}'");
        (first as usize - '0' as usize) * 10 + (last as usize - '0' as usize)
    }).sum();
    println!("Result: {}", result);
}
