use itertools::Itertools;
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

fn main() {
    println!("Hello day 1!");
    let input = read_to_string("inputs/day_19/input")
        .unwrap()
        .trim()
        .to_string();
    //     let input = "px{a<2006:qkq,m>2090:A,rfg}
    // pv{a>1716:R,A}
    // lnx{m>1548:A,A}
    // rfg{s<537:gd,x>2440:R,A}
    // qs{s>3448:A,lnx}
    // qkq{x<1416:A,crn}
    // crn{x>2662:A,R}
    // in{s<1351:px,qqz}
    // qqz{s>2770:qs,m<1801:hdj,R}
    // gd{a>3333:R,R}
    // hdj{m>838:A,pv}
    //
    // {x=787,m=2655,a=1222,s=2876}
    // {x=1679,m=44,a=2067,s=496}
    // {x=2036,m=264,a=79,s=2244}
    // {x=2461,m=1339,a=466,s=291}
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

    let parts_string = parts.next().unwrap();

    let parts = parts_string
        .split("\n")
        .map(|line| {
            let (x, m, a, s) = sscanf::scanf!(line, "{{x={i64},m={i64},a={i64},s={i64}}}").unwrap();
            Parts { x, m, a, s }
        })
        .collect_vec();

    println!("All parts: {:?}", parts);

    let result = parts
        .iter()
        .filter(|part| {
            let result = do_workflow_actions(part, &all_actions.get("in").unwrap(), &all_actions);
            println!("Result: {:?}", result);
            result == ResultOfAction::Accept
        })
        .map(|part| part.a + part.x + part.s + part.m)
        .sum::<i64>();

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
