use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter;

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
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum ResultOfAction {
    Accept,
    Reject,
    Continue,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Parts {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Parts {
    fn get_part_value(&self, name: &str) -> i64 {
        match name {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Unknown variable name '{}'", name),
        }
    }
}

fn do_workflow_action(
    parts: &Parts,
    action: &WorkflowAction,
    all_actions: &HashMap<String, Vec<WorkflowAction>>,
) -> ResultOfAction {
    match action {
        WorkflowAction::IfLowerThenDo {
            variable_name,
            value,
            action,
        } => {
            if parts.get_part_value(&variable_name) < *value {
                do_workflow_action(parts, action, all_actions)
            } else {
                ResultOfAction::Continue
            }
        }
        WorkflowAction::IfHigherThenDo {
            variable_name,
            value,
            action,
        } => {
            if parts.get_part_value(&variable_name) > *value {
                do_workflow_action(parts, &action, all_actions)
            } else {
                ResultOfAction::Continue
            }
        }
        WorkflowAction::DoAction { action_name } => {
            let action = all_actions.get(action_name).unwrap();
            do_workflow_actions(parts, action.as_ref(), all_actions)
        }
        WorkflowAction::AcceptAction => ResultOfAction::Accept,
        WorkflowAction::RejectAction => ResultOfAction::Reject,
    }
}

fn do_workflow_actions(
    parts: &Parts,
    actions: &[WorkflowAction],
    all_actions: &HashMap<String, Vec<WorkflowAction>>,
) -> ResultOfAction {
    for action in actions {
        match do_workflow_action(parts, action, all_actions) {
            ResultOfAction::Accept => return ResultOfAction::Accept,
            ResultOfAction::Reject => return ResultOfAction::Reject,
            ResultOfAction::Continue => {}
        }
    }
    panic!("Not accept or reject for the parts {:?}", parts);
}

// Not working unfortunately :( too big for the input
fn main() {
    println!("Hello day 1!");
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

    let mut values_for_variables: HashMap<String, Vec<i64>> = HashMap::new();

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
                            '<' => {
                                values_for_variables
                                    .entry(variable_name.to_string())
                                    .or_default()
                                    .push(value);

                                WorkflowAction::IfLowerThenDo {
                                    variable_name: variable_name.to_string(),
                                    value,
                                    action: Box::new(simple_action),
                                }
                            }
                            '>' => {
                                values_for_variables
                                    .entry(variable_name.to_string())
                                    .or_default()
                                    .push(value + 1);

                                WorkflowAction::IfHigherThenDo {
                                    variable_name: variable_name.to_string(),
                                    value,
                                    action: Box::new(simple_action),
                                }
                            }
                            _ => panic!("Unknown comparator '{}'", comparator),
                        }
                    })
                })
                .collect();

            (label.to_string(), actions)
        })
        .collect();

    println!("All actions: {:?}", all_actions);

    println!("Values for variables: {:?}", values_for_variables);

    let values_for_variables_with_sizes: HashMap<String, Vec<(i64, i64)>> = values_for_variables
        .into_iter()
        .map(|(label, values)| {
            let values: Vec<i64> = iter::once(1)
                .chain(values.into_iter().collect_vec())
                .chain(iter::once(4001).into_iter())
                .sorted()
                .unique()
                .collect();

            let values = values
                .iter()
                .tuple_windows()
                .map(|(a, b)| (*a, *b - *a))
                .collect_vec();

            (label, values)
        })
        .collect();

    let mut result = 0;
    let x_iter = values_for_variables_with_sizes.get("x").unwrap().iter();
    let m_iter = values_for_variables_with_sizes.get("m").unwrap().iter();
    let a_iter = values_for_variables_with_sizes.get("a").unwrap().iter();
    let s_iter = values_for_variables_with_sizes.get("s").unwrap().iter();
    println!(
        "PRODUCT OF ITERS LENGTH: {:?}",
        x_iter.count() * m_iter.count() * a_iter.count() * s_iter.count()
    );

    let x_iter = values_for_variables_with_sizes.get("x").unwrap().iter();
    let m_iter = values_for_variables_with_sizes.get("m").unwrap().iter();
    let a_iter = values_for_variables_with_sizes.get("a").unwrap().iter();
    let s_iter = values_for_variables_with_sizes.get("s").unwrap().iter();

    for ((x, x_size), (m, m_size), (a, a_size), (s, s_size)) in
        iproduct!(x_iter, m_iter, a_iter, s_iter)
    {
        let part = Parts {
            x: *x,
            m: *m,
            a: *a,
            s: *s,
        };
        let actions_result =
            do_workflow_actions(&part, &all_actions.get("in").unwrap(), &all_actions);

        if actions_result == ResultOfAction::Accept {
            result += x_size * m_size * a_size * s_size;
        }
    }

    println!(
        "Values for variables: {:?}",
        values_for_variables_with_sizes
    );

    println!("Result is {result}")
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
