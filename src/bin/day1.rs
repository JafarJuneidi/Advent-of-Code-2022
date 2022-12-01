fn main() {
    let mut output: Vec<usize> = include_str!("./day1.txt")
        .split("\n\n")
        .map(|x| x.lines().flat_map(str::parse::<usize>).sum::<usize>())
        .collect();

    output.sort_by(|a, b| b.cmp(a));

    println!("part1 is :{:?}", output[0]);
    println!("part2 is :{:?}", output.iter().take(3).sum::<usize>());
}
