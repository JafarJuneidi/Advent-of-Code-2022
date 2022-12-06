use anyhow::Result;
use std::collections::HashMap;

fn sliding_window(input: &Vec<char>, win_size: usize) -> Result<usize, ()> {
    let mut map = HashMap::new();

    for i in 0..=input.len() - win_size {
        if i == 0 {
            for j in 0..win_size {
                map.entry(input[j])
                    .and_modify(|freq| *freq += 1)
                    .or_insert(1);
            }
        } else {
            if map.len() == win_size {
                return Ok(i + win_size - 1);
            }

            if map[&input[i - 1]] == 1 {
                map.remove(&input[i - 1]);
            } else {
                map.entry(input[i - 1]).and_modify(|freq| *freq -= 1);
            }
            map.entry(input[i + win_size - 1])
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        }
    }
    Err(())
}

fn main() -> Result<()> {
    let input = include_str!("./day6.txt")
        .trim()
        .chars()
        .collect::<Vec<char>>();

    println!("part1 is :{:?}", sliding_window(&input, 4));
    println!("part2 is :{:?}", sliding_window(&input, 14));

    Ok(())
}
