use sscanf::sscanf;
use std::collections::HashMap;
use std::fs::read_to_string;

struct Node {
    name: String,
    left: String,
    right: String,
}

fn main() {
    println!("Hello day 8!");
    let input = read_to_string("inputs/day_08/input").unwrap();

    // let input = "RL\n\
    //                     \n\
    //                     AAA = (BBB, CCC)\n\
    //                     BBB = (DDD, EEE)\n\
    //                     CCC = (ZZZ, GGG)\n\
    //                     DDD = (DDD, DDD)\n\
    //                     EEE = (EEE, EEE)\n\
    //                     GGG = (GGG, GGG)\n\
    //                     ZZZ = (ZZZ, ZZZ)";
    //
    // let input = "LLR\n\
    //             \n\
    //             AAA = (BBB, BBB)\n\
    //             BBB = (AAA, ZZZ)\n\
    //             ZZZ = (ZZZ, ZZZ)";

    let mut lines = input.lines();

    let instructions = lines.next().unwrap();

    let nodes: HashMap<String, Node> = lines
        .skip(1)
        .map(|line| {
            let (name, left, right) = sscanf!(line, "{str} = ({str}, {str})").unwrap();
            (
                name.to_string(),
                Node {
                    name: name.to_string(),
                    left: left.to_string(),
                    right: right.to_string(),
                },
            )
        })
        .collect();

    let mut current_node = nodes.get("AAA").unwrap();
    let mut no_of_steps = 0;
    let mut current_step = 0;

    while current_node.name != "ZZZ" {
        if instructions.chars().nth(current_step).unwrap() == 'R' {
            current_node = nodes.get(&current_node.right).unwrap();
        } else {
            current_node = nodes.get(&current_node.left).unwrap();
        }
        no_of_steps += 1;
        current_step = (current_step + 1) % instructions.len();
    }
    let result = no_of_steps;
    println!("Result: {}", result);
}
