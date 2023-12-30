use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};
use itertools::{chain, enumerate, iproduct, Itertools};
use std::env::vars;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    x_velocity: f64,
    y_velocity: f64,
    z_velocity: f64,
}

fn main() {
    println!("Hello day 1!");
    let input = read_to_string("inputs/day_24/input").unwrap();
    const MIN_RANGE: f64 = 200000000000000.;
    const MAX_RANGE: f64 = 400000000000000.;

    // let input = "19, 13, 30 @ -2, 1, -2\n\
    // 18, 19, 22 @ -1, -1, -2\n\
    // 20, 25, 34 @ -2, -2, -4\n\
    // 12, 31, 28 @ -1, -2, -1\n\
    // 20, 19, 15 @ 1, -5, -3";
    // const MIN_RANGE: f64 = 7.;
    // const MAX_RANGE: f64 = 27.;

    let hailstones: Vec<Hailstone> = input
        .lines()
        .map(|line| {
            let (x, y, z, x_velocity, y_velocity, z_velocity) =
                sscanf::scanf!(line, "{f64}, {f64}, {f64} @ {f64}, {f64}, {f64}").unwrap();

            Hailstone {
                x,
                y,
                z,
                x_velocity,
                y_velocity,
                z_velocity,
            }
        })
        .collect_vec();

    let mut min_x_velocity = 0.;
    let mut min_y_velocity = 0.;
    let mut min_z_velocity = 0.;

    for hailstone in hailstones.iter() {
        for other in hailstones.iter() {
            if hailstone == other {
                continue;
            }

            if hailstone.z < other.z && hailstone.z_velocity < 0.0 && other.z_velocity > 0. {
                let min_z_velocity_for_pair = other.z_velocity - hailstone.z_velocity;
                if min_z_velocity_for_pair > min_z_velocity {
                    min_z_velocity = min_z_velocity_for_pair;
                }
            }

            if hailstone.x < other.x && hailstone.x_velocity < 0.0 && other.x_velocity > 0. {
                let min_x_velocity_for_pair = other.x_velocity - hailstone.x_velocity;
                if min_x_velocity_for_pair > min_x_velocity {
                    min_x_velocity = min_x_velocity_for_pair;
                }
            }

            if hailstone.y < other.y && hailstone.y_velocity < 0.0 && other.y_velocity > 0. {
                let min_y_velocity_for_pair = other.y_velocity - hailstone.y_velocity;
                if min_y_velocity_for_pair > min_y_velocity {
                    min_y_velocity = min_y_velocity_for_pair;
                }
            }
        }
    }

    let min_x_velocity = min_x_velocity.round() as i32 + 1;
    let min_y_velocity = min_y_velocity.round() as i32 + 1;

    println!("Min x is {}", min_x_velocity);
    println!("Min y is {}", min_y_velocity);
    println!("Min z is {}", min_z_velocity);
    // let x_vel = 10;
    // let y_vel = 10;
    // let z_vel = 10;

    let range_to_check = 200;

    let x_velocities_to_check = ((-min_x_velocity - range_to_check)..(-min_x_velocity))
        .into_iter()
        .chain(min_x_velocity..(min_x_velocity + range_to_check));

    let y_velocities_to_check = ((-min_y_velocity - range_to_check)..(-min_y_velocity))
        .into_iter()
        .chain(min_y_velocity..(min_y_velocity + range_to_check));

    // let mut best_solution = f64::MAX;
    // let mut best_velocities = (0, 0, 0);
    //
    // let result = minimize_without_accurate(&hailstones[0..2], x_vel, y_vel, z_vel);
    //
    for (x_vel, y_vel) in iproduct!(x_velocities_to_check, y_velocities_to_check) {
        println!("Testing {} {}", x_vel, y_vel);

        if let Some(solution) = solve_for_velocities(&hailstones, x_vel, y_vel) {
            println!("Found velocities {:?}", solution);
            return;
        } else {
            println!("Failed to minimize ?");
        }
    }
    //
    // println!("best velocities {:?}", best_velocities);

    // println!("Hailstones: {:?}", hailstones);
}

fn solve_for_velocities(hailstones: &[Hailstone], x_vel: i32, y_vel: i32) -> Option<(i64, i64)> {
    let start = std::time::Instant::now();
    let mut vars = variables!();

    let x_var = vars.add(variable().integer().name("x"));
    let y_var = vars.add(variable().integer().name("y"));

    let mut collision_variables = vec![];

    for (hailstone_index, hailstone) in hailstones.iter().enumerate() {
        let collision_t = vars.add(
            variable()
                .integer()
                .name(format!("hailstone_collision_{hailstone_index}"))
                .min(0),
        );

        collision_variables.push(collision_t);
    }

    let mut solver = vars.minimise(collision_variables[0]).using(default_solver);

    for (hailstone_index, hailstone) in hailstones.iter().enumerate() {
        let collision_t = collision_variables[hailstone_index];
        solver = solver
            .with(constraint!(
                x_var + x_vel * collision_t == hailstone.x + hailstone.x_velocity * collision_t
            ))
            .with(constraint!(
                y_var + y_vel * collision_t == hailstone.y + hailstone.y_velocity * collision_t
            ))
    }

    if let Ok(solution) = solver.solve() {
        let x_value = solution.value(x_var).round();
        let y_value = solution.value(y_var).round();

        println!("Solutions found {x_value}, {y_value}");

        Some((x_value as i64, y_value as i64))
    } else {
        println!(
            "Failed to find solution, elapsed {}",
            start.elapsed().as_secs_f32()
        );
        None
    }
}

fn minimize_without_accurate(
    hailstones: &[Hailstone],
    x_vel: i32,
    y_vel: i32,
    z_vel: i32,
) -> Option<f64> {
    let allowed_range = 1000000000000.0;
    let start = std::time::Instant::now();
    let mut vars = variables!();

    let x_var = vars.add(variable().integer().name("x"));
    let y_var = vars.add(variable().integer().name("y"));
    let z_var = vars.add(variable().integer().name("z"));

    let mut collision_variables = vec![];

    for (hailstone_index, hailstone) in hailstones.iter().enumerate() {
        let collision_t = vars.add(
            variable()
                .integer()
                .name(format!("hailstone_collision_{hailstone_index}"))
                .min(0),
        );

        collision_variables.push(collision_t);
    }

    let mut expression = Expression::default();

    for (hailstone_index, hailstone) in hailstones.iter().enumerate() {
        let collision_t = collision_variables[hailstone_index];

        expression +=
            x_var + x_vel * collision_t - hailstone.x - hailstone.x_velocity * collision_t
                + y_var
                + y_vel * collision_t
                - hailstone.y
                - hailstone.y_velocity * collision_t
                + z_var
                + z_vel * collision_t
                - hailstone.z
                - hailstone.z_velocity * collision_t;
    }
    let mut solver = vars.minimise(expression.clone()).using(default_solver);

    for (hailstone_index, hailstone) in hailstones.iter().enumerate() {
        let collision_t = collision_variables[hailstone_index];

        solver = solver
            .with(constraint!(
                x_var + x_vel * collision_t
                    <= hailstone.x + hailstone.x_velocity * collision_t + allowed_range
            ))
            .with(constraint!(
                x_var + x_vel * collision_t
                    >= hailstone.x + hailstone.x_velocity * collision_t - allowed_range
            ))
            .with(constraint!(
                y_var + y_vel * collision_t
                    <= hailstone.y + hailstone.y_velocity * collision_t + allowed_range
            ))
            .with(constraint!(
                y_var + y_vel * collision_t
                    >= hailstone.y + hailstone.y_velocity * collision_t - allowed_range
            ))
            .with(constraint!(
                z_var + z_vel * collision_t
                    <= hailstone.z + hailstone.z_velocity * collision_t + allowed_range
            ))
            .with(constraint!(
                z_var + z_vel * collision_t
                    >= hailstone.z + hailstone.z_velocity * collision_t - allowed_range
            ));
    }

    if let Ok(solution) = solver.solve() {
        let solution_value = solution.eval(expression);

        let x_value = solution.value(x_var).round();
        let y_value = solution.value(y_var).round();
        let z_value = solution.value(z_var).round();
        println!("Solutions found {x_value}, {y_value}, {z_value}");

        println!("Solution value {}", solution_value);

        Some(solution_value)
    } else {
        println!(
            "Failed to find solution, elapsed {}",
            start.elapsed().as_secs_f32()
        );
        None
    }
}
