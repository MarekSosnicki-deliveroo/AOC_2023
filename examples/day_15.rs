use std::fs::read_to_string;
use std::ptr::hash;
use std::thread::current;

fn main() {
    println!("Hello day 15!");
    let input = read_to_string("inputs/day_15/input")
        .unwrap()
        .trim()
        .to_string();
    // let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    let result: u64 = input
        .split(',')
        .map(|sequence| {
            let hash = sequence.chars().fold(0u64, |hash, current| {
                let hash = hash + current as u64;
                (hash * 17) % 256
            });
            println!(
                "Hash for {sequence} is {hash}",
                sequence = sequence,
                hash = hash
            );
            hash
        })
        .sum();
    println!("Result: {}", result);
}
