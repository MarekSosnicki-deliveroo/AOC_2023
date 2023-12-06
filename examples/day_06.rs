use std::fs::read_to_string;

fn main() {
    println!("Hello day 6!");
    let input = read_to_string("inputs/day_06/input").unwrap();

    //     let input = "Time:      7  15   30
    // Distance:  9  40  200";

    let mut lines = input.lines();

    let times: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect();
    let distances: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect();

    println!("Times {:?}", times);
    println!("Distances {:?}", distances);

    let result: i32 = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| calculate(*t, *d))
        .product();

    println!("Result: {}", result);
}

fn calculate(time_i32: i32, distance_i32: i32) -> i32 {
    let time = time_i32 as f64;
    let distance = distance_i32 as f64;
    // speed * duration = distance

    // wait * (time - wait) - distance = 0
    // - wait ^ 2 + time * wait - distance = 0
    // - x^2 + b*x - c = 0
    // x^2 - b*x + c = 0
    // x = (b - sqrt(b^2 - 4*c)) / 2

    // ax^2 + bx + c = 0
    // x = (-b ± sqrt(b^2 - 4ac)) / 2a

    // (x - a) ^ 2 = x^2 - 2*x*a + a^2
    //

    //let delta = b^2 - 4ac;
    let delta = time * time - 4.0 * distance;
    let x1 = (-time + delta.sqrt()) / -2.;
    let x2 = (-time - delta.sqrt()) / -2.;

    let (time_min, time_max): (f64, f64) = if x1 < x2 { (x1, x2) } else { (x2, x1) };

    let time_min = time_min.ceil() as i32;
    let time_max = time_max.floor() as i32;

    time_max - time_min + 1;
}
