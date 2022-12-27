use anyhow::Result;
use sscanf;
use std::cmp::max;

fn get_grid() -> Vec<((isize, isize), (isize, isize))> {
    let mut grid = vec![];

    include_str!("./day15.txt").lines().for_each(|line| {
        let (sensor_x, sensor_y, beacon_x, beacon_y) = sscanf::sscanf!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            isize,
            isize,
            isize,
            isize
        )
        .unwrap();

        let sensor = (sensor_x, sensor_y);
        let beacon = (beacon_x, beacon_y);

        grid.push((sensor, beacon));
    });

    return grid;
}

fn get_desired_row(
    grid: &Vec<((isize, isize), (isize, isize))>,
    row: isize,
    included: bool,
) -> Vec<(isize, isize)> {
    let mut ranges = vec![];

    grid.iter().for_each(|pair| {
        let sensor = pair.0;
        let beacon = pair.1;

        let manhattan_distance = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
        let dy = sensor.1.abs_diff(row);

        if dy < manhattan_distance {
            let dx = (manhattan_distance - dy) as isize;

            if included {
                ranges.push((sensor.0 - dx, sensor.0 + dx));
            } else {
                ranges.push(((sensor.0 - dx).max(0), (sensor.0 + dx).min(4_000_000)))
            }
        }
    });

    ranges.sort();

    let mut combined_ranges = Vec::from([ranges[0]]);

    for i in 1..ranges.len() {
        let current = combined_ranges.last().unwrap();
        let next = ranges[i];

        if next.0 > current.1 {
            combined_ranges.push(next);
        } else {
            *combined_ranges.last_mut().unwrap() = (current.0, max(current.1, next.1));
        }
    }

    return combined_ranges;
}

fn main() -> Result<()> {
    let grid = get_grid();

    let mut ranges = get_desired_row(&grid, 2_000_000, true);
    let part1 = ranges.iter().map(|(start, end)| end - start).sum::<isize>();
    let mut part2: u64 = 0;

    for i in 0..4_000_000 {
        ranges = get_desired_row(&grid, i, false);

        if ranges.len() > 1 {
            part2 = 4_000_000u64 * (ranges.first().unwrap().1 + 1) as u64 + i as u64;
            break;
        }
    }

    println!("part1 is :{:?}", part1);
    println!("part2 is :{:?}", part2);
    Ok(())
}
