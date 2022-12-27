use anyhow::Result;

fn is_contained(pair: &str) -> bool {
    let result = pair
        .replace("-", ",")
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let l1 = result[0];
    let r1 = result[1];
    let l2 = result[2];
    let r2 = result[3];

    (l2 >= l1 && r2 <= r1) || (l1 >= l2 && r1 <= r2)
}

fn is_overlapping(pair: &str) -> bool {
    let result = pair
        .replace("-", ",")
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    let l1 = result[0];
    let r1 = result[1];
    let l2 = result[2];
    let r2 = result[3];

    (l2 >= l1 && l2 <= r1)
        || (r2 >= l1 && r2 <= r1)
        || (l1 >= l2 && l1 <= r2)
        || (r1 >= l2 && r1 <= r2)
}

fn main() -> Result<()> {
    let value_1 = include_str!("./day4.txt")
        .lines()
        .filter(|x| is_contained(x))
        .count();

    let value_2 = include_str!("./day4.txt")
        .lines()
        .filter(|x| is_overlapping(x))
        .count();

    println!("part1 is :{:?}", value_1);
    println!("part2 is :{:?}", value_2);
    Ok(())
}
