use itertools::{all, enumerate};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::slice::SliceIndex;

type Brick = (usize, usize, usize, usize, usize, usize);
type BrickId = usize;
fn main() {
    println!("Hello day 22!");
    let input = read_to_string("inputs/day_22/input").unwrap();

    // let input = "1,0,1~1,2,1\n\
    // 0,0,2~2,0,2\n\
    // 0,2,3~2,2,3\n\
    // 0,0,4~0,2,4\n\
    // 2,0,5~2,2,5\n\
    // 0,1,6~2,1,6\n\
    // 1,1,8~1,1,9";

    const SIZE: usize = 1000;
    let mut grid = vec![vec![vec![usize::MAX; SIZE]; SIZE]; SIZE];

    let mut bricks: Vec<(BrickId, Brick)> = vec![];

    for (block_id, line) in input.lines().enumerate() {
        let (x1, y1, z1, x2, y2, z2) =
            sscanf::scanf!(line, "{usize},{usize},{usize}~{usize},{usize},{usize}").unwrap();
        println!(
            "x1 {} y1 {} z1 {} x2 {} y2 {} z2 {}",
            x1, y1, z1, x2, y2, z2
        );
        let brick = (x1, y1, z1, x2, y2, z2);
        bricks.push((block_id, brick));
        for (x, y, z) in brick_coordinates(brick) {
            grid[x][y][z] = block_id;
        }
    }
    let no_of_blocks = input.lines().count();

    bricks.sort_by_key(|(_, (_, _, z1, _, _, z2))| (*z1, *z2));

    // lower bricks
    for (brick_id, brick) in bricks.iter_mut() {
        while try_drop_brick(brick, *brick_id, &mut grid) {}
    }

    let mut support_bricks_per_brick: HashMap<BrickId, HashSet<BrickId>> = HashMap::default();
    for (brick_id, brick) in bricks.iter() {
        let support_bricks = get_support_bricks(&grid, *brick);
        println!(
            "Support bricks for brick {}: {:?}",
            brick_id, support_bricks
        );
        support_bricks_per_brick.insert(*brick_id, support_bricks);
    }

    let mut result = 0;
    for (brick_id, _) in bricks {
        if !support_bricks_per_brick.values().any(|supporting_bricks| {
            supporting_bricks.contains(&brick_id) && supporting_bricks.len() == 1
        }) {
            result += 1;
        }
    }
    println!("Result: {}", result);
}

fn brick_coordinates(brick: Brick) -> Vec<(usize, usize, usize)> {
    let (x1, y1, z1, x2, y2, z2) = brick;
    let mut coordinates = vec![];
    for x in x1..=x2 {
        for y in y1..=y2 {
            for z in z1..=z2 {
                coordinates.push((x, y, z));
            }
        }
    }
    coordinates
}

fn get_support_bricks(grid: &[Vec<Vec<usize>>], brick: Brick) -> HashSet<BrickId> {
    let mut support_bricks = HashSet::default();
    for (x, y, z) in brick_coordinates(brick) {
        let brick_id = grid[x][y][z - 1];
        if brick_id != usize::MAX && brick_id != grid[x][y][z] {
            support_bricks.insert(brick_id);
        }
    }
    support_bricks
}

fn try_drop_brick(brick: &mut Brick, brick_id: BrickId, grid: &mut Vec<Vec<Vec<usize>>>) -> bool {
    if brick.2 == 1 || !get_support_bricks(grid, *brick).is_empty() {
        return false;
    }

    for (x, y, z) in brick_coordinates(*brick) {
        grid[x][y][z] = usize::MAX;
    }

    // lower brick by 1
    brick.2 -= 1;
    brick.5 -= 1;

    for (x, y, z) in brick_coordinates(*brick) {
        grid[x][y][z] = brick_id;
    }

    true
}
