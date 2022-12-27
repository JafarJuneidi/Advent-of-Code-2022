use anyhow::Result;
use std::cmp::{max, min};

#[derive(Clone, PartialEq)]
enum CavePoint {
    Air,
    Rock,
    Sand,
}

fn to_coord(str: &str) -> (usize, usize) {
    let (x, y) = str.split_once(',').unwrap();
    (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
}

fn show_grid(grid: &Vec<Vec<CavePoint>>) {
    for row in grid {
        for item in row {
            print!(
                "{}",
                match item {
                    CavePoint::Air => '.',
                    CavePoint::Rock => '#',
                    CavePoint::Sand => '*',
                }
            )
        }
        println!();
    }
}

fn get_grid() -> (Vec<Vec<CavePoint>>, usize) {
    let mut grid = vec![vec![CavePoint::Air; 1000]; 200];

    let mut y_max = 0;

    include_str!("./day14.txt").lines().for_each(|line| {
        let mut previous: Option<(usize, usize)> = None;

        line.split(" -> ").for_each(|range| {
            if previous == None {
                let prev = to_coord(range);
                if prev.1 > y_max {
                    y_max = prev.1;
                }
                previous = Some(prev);
                return;
            }

            let prev = previous.unwrap();
            let current = to_coord(range);

            if current.1 > y_max {
                y_max = current.1;
            }

            if prev.0 == current.0 {
                for i in min(prev.1, current.1)..=max(prev.1, current.1) {
                    grid[i][current.0] = CavePoint::Rock;
                }
            } else {
                for i in min(prev.0, current.0)..=max(prev.0, current.0) {
                    grid[current.1][i] = CavePoint::Rock;
                }
            }

            previous = Some(current);
        })
    });

    grid.truncate(y_max + 1);
    grid.push(vec![CavePoint::Air; 1000]);
    grid.push(vec![CavePoint::Rock; 1000]);
    return (grid, y_max);
}

fn main() -> Result<()> {
    let (mut grid, y_max) = get_grid();

    let mut done = false;
    let mut part_1_total = 0;
    let mut total = 0;

    loop {
        let mut sand_coord = (500, 0);

        loop {
            if sand_coord.1 >= y_max && part_1_total == 0 {
                part_1_total = total;
            }

            let down = (sand_coord.0, sand_coord.1 + 1);
            if grid[down.1][down.0] == CavePoint::Air {
                sand_coord = down;
                continue;
            }

            let down_left = (sand_coord.0 - 1, sand_coord.1 + 1);
            if grid[down_left.1][down_left.0] == CavePoint::Air {
                sand_coord = down_left;
                continue;
            }

            let down_right = (sand_coord.0 + 1, sand_coord.1 + 1);
            if grid[down_right.1][down_right.0] == CavePoint::Air {
                sand_coord = down_right;
                continue;
            }

            grid[sand_coord.1][sand_coord.0] = CavePoint::Sand;
            total += 1;
            if sand_coord == (500, 0) {
                done = true;
            }
            break;
        }

        if done {
            break;
        }
    }

    println!("part1 is :{:?}", part_1_total);
    println!("part2 is :{:?}", total);

    Ok(())
}
