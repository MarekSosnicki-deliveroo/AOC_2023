use itertools::Itertools;
use std::fs::read_to_string;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

#[derive(Debug, Clone)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    x_velocity: i64,
    y_velocity: i64,
    z_velocity: i64,
}

// Z3 logic from arthomnix's part2: https://github.com/arthomnix/aoc23/blob/master/src/days/day24.rs
fn main() {
    println!("Hello day 24!");
    let input = read_to_string("inputs/day_24/input").unwrap();

    // let input = "19, 13, 30 @ -2, 1, -2\n\
    // 18, 19, 22 @ -1, -1, -2\n\
    // 20, 25, 34 @ -2, -2, -4\n\
    // 12, 31, 28 @ -1, -2, -1\n\
    // 20, 19, 15 @ 1, -5, -3";
    // const MIN_RANGE: i64 = 7.;
    // const MAX_RANGE: i64 = 27.;

    let hailstones: Vec<Hailstone> = input
        .lines()
        .map(|line| {
            let (x, y, z, x_velocity, y_velocity, z_velocity) =
                sscanf::scanf!(line, "{i64}, {i64}, {i64} @ {i64}, {i64}, {i64}").unwrap();

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

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    let max_vx = Int::from_i64(&ctx, 1000);
    let max_vy = Int::from_i64(&ctx, 1000);

    solver.assert(&(&vx.le(&max_vx)));
    solver.assert(&(&vy.le(&max_vy)));

    for hailstone in hailstones {
        let pxn = Int::from_i64(&ctx, hailstone.x);
        let pyn = Int::from_i64(&ctx, hailstone.y);
        let pzn = Int::from_i64(&ctx, hailstone.z);
        let vxn = Int::from_i64(&ctx, hailstone.x_velocity);
        let vyn = Int::from_i64(&ctx, hailstone.y_velocity);
        let vzn = Int::from_i64(&ctx, hailstone.z_velocity);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    println!("Starting check");

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();

    let result = x + y + z;

    println!("result: {}", result);
}
