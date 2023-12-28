use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
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
    // println!("Hailstones: {:?}", hailstones);

    let result = hailstones
        .iter()
        .tuple_combinations()
        .map(|(left, right)| {
            // left_y = left_o + left_a * left_x
            let left_o = left.x - left.x_velocity * left.y / left.y_velocity;
            let left_a = left.x_velocity / left.y_velocity;

            // right_y = right_o + right_a * right_x
            let right_o = right.x - right.x_velocity * right.y / right.y_velocity;
            let right_a = right.x_velocity / right.y_velocity;

            let crossing_x = (left_o - right_o) / (right_a - left_a);
            let crossing_y = left_o + left_a * crossing_x;

            let t_left = (crossing_x - left.y) / left.y_velocity;
            let t_right = (crossing_x - right.y) / right.y_velocity;

            println!("Hailstone A {:?}", left);
            println!("Hailstone B {:?}", right);

            println!(
                "Crossing x={crossing_x} crossing y={crossing_y} t_right={t_right} t_left={t_left}"
            );

            (t_right, t_left, crossing_x, crossing_y)
        })
        .filter(|&(t_right, t_left, x, y)| {
            t_right > 0.
                && t_left > 0.
                && x >= MIN_RANGE
                && x <= MAX_RANGE
                && y >= MIN_RANGE
                && y <= MAX_RANGE
        })
        .inspect(|_| println!("Passed!"))
        .count();

    println!("Result: {}", result);
}
