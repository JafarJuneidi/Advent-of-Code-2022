use anyhow::Result;

struct Monkey {
    items: Vec<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    divisible: usize,
    yay: usize,
    nay: usize,
    inspections: usize,
}

pub fn main() -> Result<()> {
    let mut monkies: Vec<_> = include_str!("./day11.txt")
        .split("\n\n")
        .map(|monkey| {
            let lines: Vec<_> = monkey
                .lines()
                .map(|line| line.split(": ").last().unwrap())
                .collect();
            Monkey {
                items: lines[1]
                    .split(", ")
                    .map(|num| num.parse().unwrap())
                    .collect(),
                operation: {
                    let op: Vec<_> = lines[2].rsplit_once("= ").unwrap().1.split(' ').collect();
                    match op[2] {
                        "old" => Box::new(|old| old * old),
                        b => match (op[1], b.parse::<usize>().unwrap()) {
                            ("+", n) => Box::new(move |old| old + n),
                            ("*", n) => Box::new(move |old| old * n),
                            _ => unreachable!(),
                        },
                    }
                },
                divisible: lines[3].rsplit_once(' ').unwrap().1.parse().unwrap(),
                yay: lines[4].rsplit_once(' ').unwrap().1.parse().unwrap(),
                nay: lines[5].rsplit_once(' ').unwrap().1.parse().unwrap(),
                inspections: 0,
            }
        })
        .collect();

    let (mo, mut items): (usize, _) = (
        monkies.iter().map(|monkey| monkey.divisible).product(),
        vec![vec![]; monkies.len()],
    );

    (0..10_000).for_each(|_| {
        monkies.iter_mut().enumerate().for_each(|(i, monkey)| {
            monkey.items.append(&mut items[i]);
            monkey.items.drain(..).for_each(|mut item| {
                item = (monkey.operation)(item) % mo;
                items[if item % monkey.divisible == 0 {
                    monkey.yay
                } else {
                    monkey.nay
                }]
                .push(item);
                monkey.inspections += 1;
            });
        });
    });

    let mut result = monkies
        .iter()
        .map(|monkey| monkey.inspections)
        .collect::<Vec<usize>>();
    result.sort_by(|a, b| b.cmp(a));

    println!("{}", result.iter().take(2).product::<usize>());
    Ok(())
}
