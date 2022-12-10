use anyhow::Result;
use std::collections::HashSet;

fn lead(dir: &str, head: &mut (i32, i32)) {
    match dir {
        "R" => head.0 += 1,
        "U" => head.1 += 1,
        "L" => head.0 -= 1,
        "D" => head.1 -= 1,
        _ => unreachable!("Yell at Advent of code!!"),
    }
}

fn follow(leader: (i32, i32), follower: &mut (i32, i32)) {
    let (dx, dy) = (leader.0 - follower.0, leader.1 - follower.1);
    if dx.abs() == 2 || dy.abs() == 2 {
        follower.0 += dx.signum();
        follower.1 += dy.signum();
    }
}

fn visit(knot_num: usize) -> usize {
    let mut rope = vec![(0, 0); knot_num];
    let mut visited = HashSet::from([(0, 0)]);

    include_str!("./day9.txt").lines().for_each(|line| {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<usize>().unwrap();

        for _ in 0..steps {
            lead(dir, &mut rope[0]);

            for i in 1..rope.len() {
                follow(rope[i - 1], &mut rope[i]);
            }

            visited.insert(*rope.last().unwrap());
        }
    });

    return visited.len();
}

fn main() -> Result<()> {
    println!("part1 is :{:?}", visit(2));
    println!("part2 is :{:?}", visit(10));
    Ok(())
}
