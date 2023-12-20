use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq, Clone)]
enum ModuleType {
    Broadcaster {
        destinations: Vec<String>,
    },
    FlipFlopButton {
        turned_on: bool,
        destinations: Vec<String>,
    },
    Conjunction {
        most_recent_pulse: HashMap<String, bool>,
        destinations: Vec<String>,
    },
    Nothing,
}

fn main() {
    println!("Hello day 20!");
    let input = read_to_string("inputs/day_20/input").unwrap();

    //     let input = "broadcaster -> a, b, c\n\
    // %a -> b\n\
    // %b -> c\n\
    // %c -> inv\n\
    // &inv -> a";

    //     let input = "broadcaster -> a\n\
    // %a -> inv, con\n\
    // &inv -> b\n\
    // %b -> con\n\
    // &con -> output";

    // &jh -> zz, lr, vl, fc, nz, fk, qg
    let mut modules: HashMap<String, ModuleType> = input
        .lines()
        .map(|line| {
            let (module_name_with_prefix, destinations_str) =
                sscanf::scanf!(line, "{str} -> {str}").unwrap();

            let destinations = destinations_str
                .split(", ")
                .map(|d| d.to_string())
                .collect_vec();

            let (module_name, module) = if module_name_with_prefix == "broadcaster" {
                (
                    "broadcaster".to_string(),
                    ModuleType::Broadcaster { destinations },
                )
            } else if module_name_with_prefix.starts_with("%") {
                (
                    module_name_with_prefix[1..].to_string(),
                    ModuleType::FlipFlopButton {
                        turned_on: false,
                        destinations,
                    },
                )
            } else if module_name_with_prefix.starts_with("&") {
                (
                    module_name_with_prefix[1..].to_string(),
                    ModuleType::Conjunction {
                        most_recent_pulse: HashMap::new(),
                        destinations,
                    },
                )
            } else {
                panic!("Unknown module type '{}'", module_name_with_prefix);
            };
            (module_name, module)
        })
        .collect();

    println!("Modules {:?}", modules);

    for (module_name, module) in modules.clone().iter() {
        match module {
            ModuleType::Broadcaster { destinations }
            | ModuleType::FlipFlopButton { destinations, .. }
            | ModuleType::Conjunction { destinations, .. } => {
                for destination in destinations {
                    if let Some(destination_module) = modules.get_mut(destination) {
                        if let ModuleType::Conjunction {
                            most_recent_pulse, ..
                        } = destination_module
                        {
                            most_recent_pulse.insert(module_name.to_string(), false);
                        }
                    } else {
                        modules.insert(destination.to_string(), ModuleType::Nothing);
                    }
                }
            }
            _ => {}
        }
    }

    println!("Modules {:?}", modules);

    let mut high_pulses_counter: i64 = 0;
    let mut low_pulses_counter: i64 = 0;

    for iteration in 0..1000 {
        // println!("=====");
        // println!("Starting iteration {}", iteration);
        let mut pulses_queue = vec![(false, "button".to_string(), "broadcaster".to_string())];

        while !pulses_queue.is_empty() {
            let (current_pulse, from_module, current_module) = pulses_queue.remove(0);
            let module = modules.get_mut(&current_module).unwrap();

            if current_pulse {
                high_pulses_counter += 1;
            } else {
                low_pulses_counter += 1;
            }

            match module {
                ModuleType::Broadcaster { destinations } => {
                    for destination in destinations {
                        // println!("{} -{}- to {}", current_module, current_pulse, destination);
                        pulses_queue.push((
                            current_pulse,
                            current_module.clone(),
                            destination.to_string(),
                        ));
                    }
                }
                ModuleType::FlipFlopButton {
                    turned_on,
                    destinations,
                } => {
                    if !current_pulse {
                        *turned_on = !*turned_on;
                        for destination in destinations {
                            pulses_queue.push((
                                *turned_on,
                                current_module.clone(),
                                destination.to_string(),
                            ));
                            // println!("{} -{}- to {}", current_module, *turned_on, destination);
                        }
                    }
                }
                ModuleType::Conjunction {
                    most_recent_pulse,
                    destinations,
                } => {
                    most_recent_pulse.insert(from_module.clone(), current_pulse);
                    let pulse_to_send = !most_recent_pulse.values().all(|v| *v);
                    for destination in destinations {
                        pulses_queue.push((
                            pulse_to_send,
                            current_module.clone(),
                            destination.to_string(),
                        ));
                        // println!("{} -{}- to {}", current_module, pulse_to_send, destination);
                    }
                }
                ModuleType::Nothing => {}
            }
        }
    }

    println!("=====");

    println!("High pulses {}", high_pulses_counter);
    println!("Low pulses {}", low_pulses_counter);

    println!("Result: {}", high_pulses_counter * low_pulses_counter);
}
