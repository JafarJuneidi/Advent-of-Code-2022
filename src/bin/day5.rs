use std::collections::VecDeque;

use anyhow::Result;

fn main() -> Result<()> {
    let (start, moves) = include_str!("./day5.txt").split_once("\n\n").unwrap();

    let mut start = start.lines().rev();
    let mut stacks_1: Vec<VecDeque<char>> = start
        .next()
        .unwrap()
        .split_whitespace()
        .map(|_| VecDeque::<char>::new())
        .collect();
    let mut stacks_2 = stacks_1.clone();

    start
        .flat_map(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_, c)| *c != ' ')
                .collect::<Vec<(usize, char)>>()
        })
        .for_each(|(i, c)| {
            stacks_1[i].push_front(c);
            stacks_2[i].push_front(c);
        });

    moves
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|c| c.parse::<usize>().ok())
                .collect::<Vec<usize>>()
        })
        .map(|line| (line[0], line[1] - 1, line[2] - 1))
        .for_each(|(num, from, to)| {
            let m_1: Vec<char> = (0..num)
                .map(|_| stacks_1[from].pop_front().unwrap())
                .collect();
            m_1.iter().for_each(|c| stacks_1[to].push_front(*c));

            let m_2: Vec<char> = (0..num)
                .map(|_| stacks_2[from].pop_front().unwrap())
                .collect();
            m_2.iter().rev().for_each(|c| stacks_2[to].push_front(*c));
        });

    println!(
        "part1 is :{:?}",
        stacks_1.iter().map(|s| s[0]).collect::<String>()
    );

    println!(
        "part2 is :{:?}",
        stacks_2.iter().map(|s| s[0]).collect::<String>()
    );
    Ok(())
}
