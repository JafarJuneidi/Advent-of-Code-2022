#![feature(iter_array_chunks)]
use anyhow::Result;

fn unique_items(s: &str) -> u64 {
    s.bytes()
        .map(|b| match b {
            b'a'..=b'z' => 1 + b - b'a',
            b'A'..=b'Z' => 27 + b - b'A',
            _ => unreachable!(),
        })
        .fold(0, |acc, b| acc | (1u64 << b))
}

fn main() -> Result<()> {
    let value_1: u32 = include_str!("./day3.txt")
        .lines()
        .map(|x| x.split_at(x.len() / 2))
        .map(|(l, r)| [l, r].map(unique_items))
        .map(|[l, r]| u64::trailing_zeros(l & r))
        .sum();

    let value_2: u32 = include_str!("./day3.txt")
        .lines()
        .array_chunks::<3>()
        .map(|chunk| chunk.map(unique_items))
        .map(|[a, b, c]| a & b & c)
        .map(u64::trailing_zeros)
        .sum();

    println!("part1 is :{:?}", value_1);
    println!("part2 is :{:?}", value_2);
    Ok(())
}
