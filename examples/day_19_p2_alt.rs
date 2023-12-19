use itertools::Itertools;
use rayon::iter::split;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum WorkflowAction {
    IfLowerThenDo {
        variable_name: String,
        value: i64,
        action: Box<WorkflowAction>,
    },
    IfHigherThenDo {
        variable_name: String,
        value: i64,
        action: Box<WorkflowAction>,
    },
    DoAction {
        action_name: String,
    },
    AcceptAction,
    RejectAction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PartsRange {
    ranges: HashMap<String, (i64, i64)>,
}

impl PartsRange {
    fn split_at_value(&self, name: &str, value: i64) -> (Option<Self>, Option<Self>) {
        let range_for_name = self
            .ranges
            .get(name)
            .expect("Failed to find range with the name");

        let (start, end) = *range_for_name;

        // 3456, start 3 end 6

        if end <= value {
            // split at 6,7,8...
            (Some(self.clone()), None)
        } else if start >= value {
            // split at 0,1,2,3
            (None, Some(self.clone()))
        } else {
            let mut left_part = self.ranges.clone();
            left_part.insert(name.to_string(), (start, value));

            let mut right_part = self.ranges.clone();
            right_part.insert(name.to_string(), (value, end));

            (
                Some(Self { ranges: left_part }),
                Some(Self { ranges: right_part }),
            )
        }
    }
}

impl PartsRange {
    fn size(&self) -> i64 {
        self.ranges
            .values()
            .map(|(start, end)| end - start)
            .product()
    }
}

fn do_workflow_action(
    parts_ranges: &[PartsRange],
    action: &WorkflowAction,
    all_actions: &HashMap<String, Vec<WorkflowAction>>,
) -> (i64, Vec<PartsRange>) {
    let mut remaining_parts_ranges = vec![];
    let mut accepted = 0i64;
    for part_range in parts_ranges {
        match action {
            WorkflowAction::IfLowerThenDo {
                variable_name,
                value,
                action,
            } => {
                let (left, right) = part_range.split_at_value(&variable_name, *value - 1);

                if let Some(left) = left {
                    let (action_accepted, action_remaining) =
                        do_workflow_action(&[left], action, &all_actions);
                    accepted += action_accepted;
                    remaining_parts_ranges.extend(action_remaining);
                }
                if let Some(right) = right {
                    remaining_parts_ranges.push(right)
                }
            }
            WorkflowAction::IfHigherThenDo {
                variable_name,
                value,
                action,
            } => {
                let (left, right) = part_range.split_at_value(&variable_name, *value);

                if let Some(left) = left {
                    remaining_parts_ranges.push(left)
                }
                if let Some(right) = right {
                    let (action_accepted, action_remaining) =
                        do_workflow_action(&[right], action, &all_actions);
                    accepted += action_accepted;
                    remaining_parts_ranges.extend(action_remaining);
                }
            }
            WorkflowAction::DoAction { action_name } => {
                let action = all_actions.get(action_name).unwrap();
                let (accepted_from_actions, remaining) =
                    do_workflow_actions(vec![part_range.clone()], action.as_ref(), all_actions);
                accepted += accepted_from_actions;
                remaining_parts_ranges.extend(remaining);
            }
            WorkflowAction::AcceptAction => accepted += part_range.size(),
            WorkflowAction::RejectAction => {}
        }
    }
    (accepted, remaining_parts_ranges)
}

fn do_workflow_actions(
    parts: Vec<PartsRange>,
    actions: &[WorkflowAction],
    all_actions: &HashMap<String, Vec<WorkflowAction>>,
) -> (i64, Vec<PartsRange>) {
    let mut remaining_parts_ranges = parts;
    let mut accepted = 0i64;
    for action in actions {
        let (accepted_for_action, remaining_parts_ranges_from_action) =
            do_workflow_action(&remaining_parts_ranges, action, all_actions);
        accepted += accepted_for_action;
        remaining_parts_ranges = remaining_parts_ranges_from_action;
    }

    if !remaining_parts_ranges.is_empty() {
        println!(
            "There are some remaining parts ranges: {:?}",
            remaining_parts_ranges
        );
    }
    (accepted, remaining_parts_ranges)
}

fn main() {
    println!("Hello day 19!");
    let input = read_to_string("inputs/day_19/input")
        .unwrap()
        .trim()
        .to_string();
    // let input = "px{a<2006:qkq,m>2090:A,rfg}\n\
    // pv{a>1716:R,A}\n\
    // lnx{m>1548:A,A}\n\
    // rfg{s<537:gd,x>2440:R,A}\n\
    // qs{s>3448:A,lnx}\n\
    // qkq{x<1416:A,crn}\n\
    // crn{x>2662:A,R}\n\
    // in{s<1351:px,qqz}\n\
    // qqz{s>2770:qs,m<1801:hdj,R}\n\
    // gd{a>3333:R,R}\n\
    // hdj{m>838:A,pv}\n\
    // \n\
    // {x=787,m=2655,a=1222,s=2876}\n\
    // {x=1679,m=44,a=2067,s=496}\n\
    // {x=2036,m=264,a=79,s=2244}\n\
    // {x=2461,m=1339,a=466,s=291}\n\
    // {x=2127,m=1623,a=2188,s=1013}";

    let mut parts = input.split("\n\n");

    let action_definitions_string = parts.next().unwrap();

    let all_actions: HashMap<String, Vec<WorkflowAction>> = action_definitions_string
        .split("\n")
        .map(|line| {
            let (label, actions_string) = sscanf::scanf!(line, "{str}{{{str}}}").unwrap();

            let actions = actions_string
                .split(",")
                .map(|action_string| {
                    get_simple_action_from_str(action_string).unwrap_or_else(|| {
                        let (variable_name, comparator, value, action) =
                            sscanf::scanf!(action_string, "{str}{char}{i64}:{str}").unwrap();

                        let simple_action = get_simple_action_from_str(action).unwrap();

                        match comparator {
                            '<' => WorkflowAction::IfLowerThenDo {
                                variable_name: variable_name.to_string(),
                                value,
                                action: Box::new(simple_action),
                            },
                            '>' => WorkflowAction::IfHigherThenDo {
                                variable_name: variable_name.to_string(),
                                value,
                                action: Box::new(simple_action),
                            },
                            _ => panic!("Unknown comparator '{}'", comparator),
                        }
                    })
                })
                .collect();

            (label.to_string(), actions)
        })
        .collect();

    println!("All actions: {:?}", all_actions);

    let initial_part_range = PartsRange {
        ranges: ["x", "m", "a", "s"]
            .into_iter()
            .map(|name| (name.to_string(), (0, 4000)))
            .collect(),
    };

    let (result, remaining) = do_workflow_actions(
        vec![initial_part_range],
        &all_actions.get("in").unwrap(),
        &all_actions,
    );

    println!("Remaining parts ranges: {:?}", remaining);

    println!("Result: {}", result);
}

fn get_simple_action_from_str(action_string: &str) -> Option<WorkflowAction> {
    if action_string == "A" {
        Some(WorkflowAction::AcceptAction)
    } else if action_string == "R" {
        Some(WorkflowAction::RejectAction)
    } else if !action_string.contains(":") {
        Some(WorkflowAction::DoAction {
            action_name: action_string.to_string(),
        })
    } else {
        None
    }
}
