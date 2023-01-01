use glam::IVec2;
use itertools::Itertools;
use itertools::MinMaxResult::*;
use nom::{
    branch::alt,
    character::complete::{line_ending, one_of},
    combinator::{eof, iterator},
    multi::many1,
    sequence::terminated,
    *,
};
use std::collections::{HashMap, HashSet};

fn map(input: &str) -> IResult<&str, HashSet<IVec2>> {
    let mut it = iterator(
        input,
        terminated(many1(one_of(".#")), alt((line_ending, eof))),
    );
    let elves = it
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .filter_map(move |(x, c)| match c {
                    '.' => None,
                    '#' => Some(IVec2::new(x as i32, y as i32)),
                    _ => panic!("unknown char"),
                })
        })
        .collect::<HashSet<IVec2>>();

    let res: IResult<_, _> = it.finish();
    Ok((res.unwrap().0, elves))
}

fn part1(input: &str) -> String {
    let (_, mut field) = map(input).unwrap();
    let checks = vec![
        [IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1)],
        [IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1)],
        [IVec2::new(-1, -1), IVec2::new(-1, 0), IVec2::new(-1, 1)],
        [IVec2::new(1, -1), IVec2::new(1, 0), IVec2::new(1, 1)],
    ];
    let checks_iter = checks.iter().cycle();

    for i in 0..10 {
        let local_checks = checks_iter.clone().skip(i).take(4);

        let mut proposed_moves: HashMap<IVec2, Vec<IVec2>> = HashMap::new();

        for elf in field.iter() {
            if local_checks
                .clone()
                .flat_map(|v| v.iter().map(|vec| *vec + *elf))
                .unique()
                .all(|value| field.get(&value).is_none())
            {
                proposed_moves.entry(*elf).or_insert(vec![*elf]);
                continue;
            };

            let possible_move = local_checks.clone().find_map(|checks| {
                let output = checks
                    .iter()
                    .all(|position| field.get(&(*position + *elf)).is_none())
                    .then_some(checks[1] + *elf);
                output
            });
            if let Some(r#move) = possible_move {
                proposed_moves
                    .entry(r#move)
                    .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            } else {
                proposed_moves.entry(*elf).or_insert(vec![*elf]);
            }
        }

        field = proposed_moves
            .into_iter()
            .flat_map(|(desired_position, elves_to_move)| {
                if elves_to_move.len() == 1 {
                    vec![desired_position]
                } else {
                    elves_to_move
                }
            })
            .collect::<HashSet<IVec2>>();
    }

    let minmax_x = field.iter().map(|v| v.x).minmax();
    let minmax_y = field.iter().map(|v| v.y).minmax();
    let (MinMax(x1,x2), MinMax(y1,y2)) = (minmax_x,minmax_y) else {
        panic!("");
    };

    let min_box_size = (x2 - x1 + 1) * (y2 - y1 + 1);
    (min_box_size as usize - field.len()).to_string()
}

fn print_field(field: &HashSet<IVec2>) {
    let minmax_x = field.iter().map(|v| v.x).minmax();
    let minmax_y = field.iter().map(|v| v.y).minmax();
    let (MinMax(x1,x2), MinMax(y1,y2)) = (minmax_x,minmax_y) else {
        panic!("");
    };
    let output = (y1..=y2)
        .cartesian_product(x1..=x2)
        .map(|(y, x)| match field.get(&IVec2 { x, y }) {
            Some(_) => "#",
            None => ".",
        })
        .chunks((x2 - x1 + 1) as usize)
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .join("\n");
    println!("{}", output);
}

fn part2(input: &str) -> String {
    let (_, mut field) = map(input).unwrap();
    let checks = vec![
        [IVec2::new(-1, -1), IVec2::new(0, -1), IVec2::new(1, -1)],
        [IVec2::new(-1, 1), IVec2::new(0, 1), IVec2::new(1, 1)],
        [IVec2::new(-1, -1), IVec2::new(-1, 0), IVec2::new(-1, 1)],
        [IVec2::new(1, -1), IVec2::new(1, 0), IVec2::new(1, 1)],
    ];
    let checks_iter = checks.iter().cycle();

    let mut rounds = 0;

    for i in 0.. {
        let local_checks = checks_iter.clone().skip(i).take(4);

        let mut proposed_moves: HashMap<IVec2, Vec<IVec2>> = HashMap::new();

        for elf in field.iter() {
            if local_checks
                .clone()
                .flat_map(|v| v.iter().map(|vec| *vec + *elf))
                .unique()
                .all(|value| field.get(&value).is_none())
            {
                proposed_moves
                    .entry(*elf)
                    .or_insert(vec![*elf]);
                continue;
            };
            let possible_move = local_checks.clone().find_map(|checks| {
                let output = checks
                    .iter()
                    .all(|position| field.get(&(*position + *elf)).is_none())
                    .then_some(checks[1] + *elf);
                output
            });
            if let Some(r#move) = possible_move {
                proposed_moves
                    .entry(r#move)
                    .and_modify(|value| value.push(*elf))
                    .or_insert(vec![*elf]);
            } else {
                proposed_moves
                    .entry(*elf)
                    .or_insert(vec![*elf]);
            }
        }

        let new_field = proposed_moves
            .into_iter()
            .flat_map(|(desired_position, elves_to_move)| {
                if elves_to_move.len() == 1 {
                    vec![desired_position]
                } else {
                    elves_to_move
                }
            })
            .collect::<HashSet<IVec2>>();
        if field == new_field {
            rounds = i;
            break;
        } else {
            field = new_field
        }
    }

    (rounds + 1).to_string()
}

fn main() {

    let input = include_str!("./day23.txt");
    let part1 = part1(input);
    let part2 = part2(input);

    println!("part1 is: {}", part1);
    println!("part2 is: {}", part2);
}
