use anyhow::Result;

fn sliding_window(input: &str, win_size: usize) -> usize {
    input
        .as_bytes()
        .windows(win_size)
        .position(|window| {
            !window
                .iter()
                .enumerate()
                .any(|(index, num)| window[..index].contains(num))
        })
        .unwrap()
        + win_size
}

fn main() -> Result<()> {
    let input = include_str!("./day6.txt");

    println!("part1 is :{:?}", sliding_window(input, 4));
    println!("part2 is :{:?}", sliding_window(input, 14));

    Ok(())
}
