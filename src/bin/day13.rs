use anyhow::Result;
use std::iter::Peekable;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Packet {
    Num(usize),
    List(Vec<Packet>),
}

impl Packet {
    fn parse(line: &str) -> Option<Packet> {
        Self::parse_rec(&mut line.chars().peekable())
    }

    fn parse_rec(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Packet> {
        match chars.next() {
            Some('[') => {
                let mut list = vec![];

                while let Some(term) = Self::parse_rec(chars) {
                    list.push(term);

                    if let Some(']') = chars.next() {
                        break;
                    }
                }

                return Some(Packet::List(list));
            }
            Some(c) if c.is_ascii_digit() => {
                if let Some('0') = chars.peek() {
                    chars.next();
                    return Some(Packet::Num(10));
                } else {
                    return Some(Packet::Num((c as u8 - b'0') as usize));
                }
            }
            _ => None,
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, right: &Self) -> std::cmp::Ordering {
        match (self, right) {
            (Packet::Num(left_num), Packet::Num(right_num)) => left_num.cmp(right_num),
            (Packet::List(left_list), Packet::List(right_list)) => left_list
                .iter()
                .zip(right_list.iter())
                .find_map(|(left_term, right_term)| {
                    let result = left_term.cmp(right_term);
                    result.is_ne().then_some(result)
                })
                .unwrap_or_else(|| left_list.len().cmp(&right_list.len())),
            (Packet::Num(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(right),
            (Packet::List(_), Packet::Num(_)) => self.cmp(&Packet::List(vec![right.clone()])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let part1 = include_str!("./day13.txt")
        .split("\n\n")
        .map(|split| split.split("\n").collect::<Vec<&str>>())
        .enumerate()
        .filter_map(
            |(i, chunk)| match (Packet::parse(chunk[0]), Packet::parse(chunk[1])) {
                (Some(a), Some(b)) if a < b => Some(i + 1),
                _ => None,
            },
        )
        .sum::<usize>();

    println!("part1 is :{:?}", part1);

    let mut terms: Vec<Packet> = include_str!("./day13.txt")
        .lines()
        .filter_map(Packet::parse)
        .collect();

    let (d2, d6) = (
        Packet::parse("[[2]]").unwrap(),
        Packet::parse("[[6]]").unwrap(),
    );
    terms.push(d2.clone());
    terms.push(d6.clone());

    terms.sort();

    let part2 = (1 + terms
        .iter()
        .enumerate()
        .find(|(_, term)| **term == d2)
        .unwrap()
        .0)
        * (1 + terms
            .iter()
            .enumerate()
            .find(|(_, term)| **term == d6)
            .unwrap()
            .0);

    println!("part2 is :{:?}", part2);

    Ok(())
}
