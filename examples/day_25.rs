use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::Deref;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Node {
    connections: HashSet<String>,
}

fn main() {
    println!("Hello day 25!");
    let input = read_to_string("inputs/day_25/input").unwrap();

    //     let input = "jqt: rhn xhk nvd\n\
    // rsh: frs pzl lsr\n\
    // xhk: hfx\n\
    // cmg: qnr nvd lhk bvb\n\
    // rhn: xhk bvb hfx\n\
    // bvb: xhk hfx\n\
    // pzl: lsr hfx nvd\n\
    // qnr: nvd\n\
    // ntq: jqt hfx bvb xhk\n\
    // nvd: lhk\n\
    // lsr: lhk\n\
    // rzs: qnr cmg lsr rsh\n\
    // frs: qnr lhk lsr";

    let mut graph: HashMap<String, Node> = HashMap::new();

    input.lines().for_each(|line| {
        let mut line_split = line.split(": ");

        let node_name = line_split.next().unwrap().to_string();

        let connections = line_split
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect_vec();

        for connection in &connections {
            graph
                .entry(connection.clone())
                .or_default()
                .connections
                .insert(node_name.clone());
        }
        graph
            .entry(node_name.clone())
            .or_default()
            .connections
            .extend(connections);
    });

    let mut visit_queue = vec![graph.keys().next().unwrap()];

    let node_with_most_connections = graph
        .iter()
        .max_by_key(|(_, node)| node.connections.len())
        .unwrap()
        .0;

    let mut nodes_in_current_group = HashSet::new();
    nodes_in_current_group.insert(node_with_most_connections.clone());

    let mut outgoing_connections_in_current_group = HashSet::new();
    for connection in graph
        .get(node_with_most_connections)
        .unwrap()
        .connections
        .iter()
    {
        outgoing_connections_in_current_group
            .insert((node_with_most_connections.clone(), connection.clone()));
    }

    while outgoing_connections_in_current_group.len() != 3
        && nodes_in_current_group.len() != graph.len()
    {
        // println!("Nodes in current group: {:?}", nodes_in_current_group);
        // println!(
        //     "Outgoing connections in current group: {:?}",
        //     outgoing_connections_in_current_group
        // );

        // Find connected nodes
        let connected_nodes: Vec<_> = outgoing_connections_in_current_group
            .iter()
            .map(|(_, connection)| connection.clone())
            .unique()
            .collect();

        let chosen_node = connected_nodes
            .iter()
            .map(|node_name| {
                let node_connections = &graph.get(node_name).unwrap().connections;

                let removed_connections = outgoing_connections_in_current_group
                    .iter()
                    .filter(|(_, connection)| connection == node_name)
                    .count() as i32;

                let added_connections = node_connections
                    .iter()
                    .filter(|connection| !nodes_in_current_group.contains(*connection))
                    .count() as i32;

                let connections_change = added_connections - removed_connections;
                (connections_change, node_name)
            })
            .min_by_key(|(connections_change, _)| *connections_change)
            .unwrap()
            .1
            .clone();
        for connection in graph.get(&chosen_node).unwrap().connections.iter() {
            if !nodes_in_current_group.contains(connection) {
                outgoing_connections_in_current_group
                    .insert((chosen_node.clone(), connection.clone()));
            }
        }
        outgoing_connections_in_current_group = outgoing_connections_in_current_group
            .into_iter()
            .filter(|(_, connection)| *connection != chosen_node)
            .collect();

        nodes_in_current_group.insert(chosen_node);
    }

    println!("Nodes in current group: {:?}", nodes_in_current_group);

    println!(
        "Outgoing connections in current group: {:?}",
        outgoing_connections_in_current_group
    );

    let result = nodes_in_current_group.len() * (graph.len() - nodes_in_current_group.len());

    println!("Result: {}", result);
}
