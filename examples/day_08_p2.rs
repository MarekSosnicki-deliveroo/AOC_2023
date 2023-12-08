use itertools::Itertools;
use sscanf::sscanf;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
struct PointData {
    first_iteration: u64,
    iteration_instruction_step: u64,
    repeated_after: Option<u64>,
}

fn main() {
    println!("Hello day 8!");
    let input = read_to_string("inputs/day_08/input").unwrap();

    // let input = "LR\n\
    //             \n\
    //             11A = (11B, XXX)\n\
    //             11B = (XXX, 11Z)\n\
    //             11Z = (11B, XXX)\n\
    //             22A = (22B, XXX)\n\
    //             22B = (22C, 22C)\n\
    //             22C = (22Z, 22Z)\n\
    //             22Z = (22B, 22B)\n\
    //             XXX = (XXX, XXX)";

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

    let mut current_nodes: Vec<Node> = nodes
        .values()
        .filter(|node| node.name.ends_with("A"))
        .cloned()
        .collect_vec();
    println!("Starting nodes size {}", current_nodes.len());
    println!("Instructions len {}", instructions.len());

    let mut nodes_data: Vec<Vec<PointData>> = vec![vec![]; current_nodes.len()];

    let mut no_of_steps: u64 = 0;
    let mut current_step = 0;

    let mut iteration_no = 0;
    loop {
        for (iteration_instruction_step, instruction) in
            instructions.chars().into_iter().enumerate()
        {
            if instruction == 'R' {
                current_nodes = current_nodes
                    .into_iter()
                    .map(|node| nodes.get(&node.right).unwrap().clone())
                    .collect_vec();
            } else {
                current_nodes = current_nodes
                    .into_iter()
                    .map(|node| nodes.get(&node.left).unwrap().clone())
                    .collect_vec();
            }

            for i in 0..current_nodes.len() {
                if current_nodes[i].name.ends_with("Z") {
                    let mut found_point = nodes_data[i].iter_mut().find(|point| {
                        point.iteration_instruction_step == iteration_instruction_step as u64
                    });

                    if let Some(point) = found_point {
                        if point.repeated_after.is_none() {
                            point.repeated_after = Some(iteration_no - point.first_iteration);
                        }
                    } else {
                        nodes_data[i].push(PointData {
                            first_iteration: iteration_no,
                            iteration_instruction_step: iteration_instruction_step as u64,
                            repeated_after: None,
                        });
                    }
                }
            }
        }

        iteration_no += 1;

        if iteration_no > 1000 {
            break;
        }
    }

    for node_data in nodes_data.first().unwrap() {
        let found_matching_points: Vec<_> = nodes_data
            .iter()
            .skip(1)
            .filter_map(|other_node_data| {
                other_node_data.iter().find(|point| {
                    point.iteration_instruction_step == node_data.iteration_instruction_step
                })
            })
            .collect();

        if found_matching_points.len() != nodes_data.len() - 1 {
            continue;
        } else {
            let mut all_matching_points = found_matching_points.clone();
            all_matching_points.push(node_data);
            println!("All points data {:?}", all_matching_points);

            all_matching_points.sort_by_key(|point| point.first_iteration);

            // n2 = (s1 - s2) / r2 + (r1 / r2) * n1

            let mut iteration_for_point: u64 = all_matching_points.last().unwrap().first_iteration;

            loop {
                if all_matching_points.iter().all(|point| {
                    (iteration_for_point + point.repeated_after.unwrap() - point.first_iteration)
                        % point.repeated_after.unwrap()
                        == 0
                }) {
                    let result = iteration_for_point * instructions.len() as u64
                        + node_data.iteration_instruction_step
                        + 1;
                    println!("Result iteration {iteration_for_point}");
                    println!("Result: {}", result);
                    break;
                }

                iteration_for_point += all_matching_points.last().unwrap().repeated_after.unwrap();
            }
        }
    }

    // let result = 0;
    // println!("Result: {}", result);
}

// 47: 10 .. 10 + 47 .. 10 + 47 + 47
// 67: 25 .. 25 + 67 ...25 + 67 + 67
// 43:
// 59

// s1 + r1 * n1 = c
// s2 + r2 * n2 = c
//
// s1 + r1 * n1 = s2 + r2 * n2
// s1 - s2 = r2 * n2 - r1 * n1
// (s1 - s2) / r2 = n2 - (r1 / r2) * n1

// n2 = (s1 - s2) / r2 + (r1 / r2) * n1

// point1 node_data.first_iteration + node_data.repeated_after * some_integer = result
// point2 found_matching_points[0].first_iteration + found_matching_points[0].repeated_after * some_integer = result

// f1 + r1 * x1 = y
// f2 + r2 * x2 = y

// f1 + r1 * x1 + r1*r2 = z
// f2 + r2 * x2 + r1*r2 = z

// f1 + r1 * (x1 + r2)
// f2 + r2 * (x2 + r1)
