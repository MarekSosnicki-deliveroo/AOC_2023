use itertools::{all, enumerate};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::slice::SliceIndex;

/*
--- Part Two ---
Disintegrating bricks one at a time isn't going to be fast enough. While it might sound dangerous, what you really need is a chain reaction.

You'll need to figure out the best brick to disintegrate. For each brick, determine how many other bricks would fall if that brick were disintegrated.

Using the same example as above:

Disintegrating brick A would cause all 6 other bricks to fall.
Disintegrating brick F would cause only 1 other brick, G, to fall.
Disintegrating any other brick would cause no other bricks to fall. So, in this example, the sum of the number of other bricks that would fall as a result of disintegrating each brick is 7.

For each brick, determine how many other bricks would fall if that brick were disintegrated. What is the sum of the number of other bricks that would fall?
*/
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

    let mut bricks_to_bricks_they_support: HashMap<BrickId, HashSet<BrickId>> = bricks
        .iter()
        .map(|(brick_id, _)| (*brick_id, HashSet::default()))
        .collect();
    let mut support_bricks_per_brick: HashMap<BrickId, HashSet<BrickId>> =
        bricks_to_bricks_they_support.clone();
    for (brick_id, brick) in bricks.iter() {
        let support_bricks = get_support_bricks(&grid, *brick);
        println!(
            "Support bricks for brick {}: {:?}",
            brick_id, support_bricks
        );
        support_bricks_per_brick.insert(*brick_id, support_bricks.clone());
        for support_brick in support_bricks {
            bricks_to_bricks_they_support
                .entry(support_brick)
                .or_default()
                .insert(*brick_id);
        }
    }

    let mut result = 0;
    for (brick_id, brick) in bricks.iter() {
        let mut removed_bricks = HashSet::new();
        let mut queue = vec![brick_id];
        while let Some(brick_id) = queue.pop() {
            removed_bricks.insert(brick_id);
            for supported_brick_id in bricks_to_bricks_they_support.get(brick_id).unwrap() {
                if support_bricks_per_brick
                    .get(supported_brick_id)
                    .unwrap()
                    .iter()
                    .all(|id| removed_bricks.contains(id))
                {
                    queue.push(supported_brick_id);
                }
            }
        }
        result += removed_bricks.len() - 1;
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
