use anyhow::Result;
use std::str::FromStr;

struct HandPair1 {
    value: usize,
}

struct HandPair2 {
    value: usize,
}

const WIN_LOSE: [usize; 3] = [3, 6, 0];
const CHOICE_VALUE: [usize; 3] = [3, 1, 2];
// A                              X  Y  Z
// B                              Z  X  Y
// C                              Y  Z  X

impl FromStr for HandPair1 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (o, p) = match s.split_once(" ") {
            Some((o, p)) => (o, p),
            None => return Err(anyhow::anyhow!("invalid input")),
        };

        let o = to_number_1(o);
        let p = to_number_1(p);
        let score = p + WIN_LOSE[(2 + o + p) % WIN_LOSE.len()];

        return Ok(HandPair1 { value: score });
    }
}

impl FromStr for HandPair2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (o, p) = match s.split_once(" ") {
            Some((o, p)) => (o, p),
            None => return Err(anyhow::anyhow!("invalid input")),
        };

        let o = to_number_2(o);
        let p = to_number_2(p);
        let score = p * 3 + CHOICE_VALUE[(o + p) % CHOICE_VALUE.len()];

        return Ok(HandPair2 { value: score });
    }
}

fn to_number_1(c: &str) -> usize {
    return match c {
        "A" => 0,
        "B" => 2,
        "C" => 1,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!("try to get here, lol"),
    };
}

fn to_number_2(c: &str) -> usize {
    return match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!("try to get here, lol"),
    };
}

fn main() -> Result<()> {
    let values_1: usize = include_str!("./day2.txt")
        .lines()
        .flat_map(|x| x.parse::<HandPair1>())
        .map(|x| x.value)
        .sum();

    let values_2: usize = include_str!("./day2.txt")
        .lines()
        .flat_map(|x| x.parse::<HandPair2>())
        .map(|x| x.value)
        .sum();

    println!("part1 is :{:?}", values_1);
    println!("part1 is :{:?}", values_2);
    Ok(())
}
