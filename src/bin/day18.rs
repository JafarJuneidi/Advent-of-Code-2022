use std::collections::HashSet;

use glam::IVec3;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult, Parser,
};

fn points(input: &str) -> IResult<&str, Vec<IVec3>> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::i32).map(|vec| IVec3::new(vec[0], vec[1], vec[2])),
    )(input)
}

fn process_part1(input: &str) -> String {
    let (_, points) = points(input).unwrap();
    let points: HashSet<IVec3> = HashSet::from_iter(points.into_iter());

    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            // number of free sides
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);

            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .count()
        })
        .sum::<usize>();

    surface_area.to_string()
}

fn process_part2(input: &str) -> String {
    let (_, points) = points(input).unwrap();
    let points: HashSet<IVec3> = HashSet::from_iter(points.into_iter());

    let surface_area = points
        .iter()
        .map(|&IVec3 { x, y, z }| {
            // number of free sides
            let x_low = IVec3::new(x - 1, y, z);
            let x_high = IVec3::new(x + 1, y, z);
            let y_low = IVec3::new(x, y - 1, z);
            let y_high = IVec3::new(x, y + 1, z);
            let z_low = IVec3::new(x, y, z - 1);
            let z_high = IVec3::new(x, y, z + 1);
            [x_low, x_high, y_low, y_high, z_low, z_high]
                .iter()
                .filter(|ivec| points.get(ivec).is_none())
                .map(|ivec| {
                    if is_interior_block(&ivec, &points) {
                        let IVec3 { x, y, z } = *ivec;
                        let x_low = IVec3::new(x - 1, y, z);
                        let x_high = IVec3::new(x + 1, y, z);
                        let y_low = IVec3::new(x, y - 1, z);
                        let y_high = IVec3::new(x, y + 1, z);
                        let z_low = IVec3::new(x, y, z - 1);
                        let z_high = IVec3::new(x, y, z + 1);
                        // (interior wall, exterior wall)
                        let is_really_exterior_block =
                            [x_low, x_high, y_low, y_high, z_low, z_high]
                                .iter()
                                .filter(|ivec| points.get(ivec).is_none())
                                .any(|block| process_block(block, &points) >= 1);
                        if is_really_exterior_block {
                            (0, 1)
                        } else {
                            (1, 0)
                        }
                    } else {
                        (0, 1)
                    }
                })
                .map(|(_interior, exterior)| exterior)
                .sum::<usize>()
        })
        .sum::<usize>();
    surface_area.to_string()
}

fn process_block(&IVec3 { x, y, z }: &IVec3, points: &HashSet<IVec3>) -> usize {
    // number of free sides
    let x_low = IVec3::new(x - 1, y, z);
    let x_high = IVec3::new(x + 1, y, z);
    let y_low = IVec3::new(x, y - 1, z);
    let y_high = IVec3::new(x, y + 1, z);
    let z_low = IVec3::new(x, y, z - 1);
    let z_high = IVec3::new(x, y, z + 1);
    [x_low, x_high, y_low, y_high, z_low, z_high]
        .iter()
        .filter(|ivec| points.get(ivec).is_none())
        .map(|ivec| {
            if is_interior_block(&ivec, &points) {
                // (interior wall, exterior wall)
                (1, 0)
            } else {
                (0, 1)
            }
        })
        .map(|(_interior, exterior)| exterior)
        .sum::<usize>()
}

fn is_interior_block(&IVec3 { x, y, z }: &IVec3, points: &HashSet<IVec3>) -> bool {
    let bounded_x_pos = points
        .iter()
        .find(|point| point.x > x && point.y == y && point.z == z)
        .is_some();
    let bounded_x_neg = points
        .iter()
        .find(|point| point.x < x && point.y == y && point.z == z)
        .is_some();
    let bounded_y_pos = points
        .iter()
        .find(|point| point.x == x && point.y > y && point.z == z)
        .is_some();
    let bounded_y_neg = points
        .iter()
        .find(|point| point.x == x && point.y < y && point.z == z)
        .is_some();
    let bounded_z_pos = points
        .iter()
        .find(|point| point.x == x && point.y == y && point.z > z)
        .is_some();
    let bounded_z_neg = points
        .iter()
        .find(|point| point.x == x && point.y == y && point.z < z)
        .is_some();
    [
        bounded_x_pos,
        bounded_x_neg,
        bounded_y_pos,
        bounded_y_neg,
        bounded_z_pos,
        bounded_z_neg,
    ]
    .iter()
    .all(|v| *v)
}
fn main() {
    let input = include_str!("./day18.txt");
    let part1 = process_part1(input);
    let part2 = process_part2(input);

    println!("part1 is: {}", part1);
    println!("part2 is: {}", part2);
}
