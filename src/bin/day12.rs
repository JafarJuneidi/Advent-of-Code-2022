use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};

type Position = (usize, usize);

fn bfs<F, G>(graph: &HashMap<Position, u8>, start: Position, goal: F, reachable: G) -> Option<usize>
where
    F: Fn(&Position) -> bool,
    G: Fn(&Position, &Position) -> bool,
{
    let mut queue: VecDeque<(Position, usize)> = VecDeque::new();
    let mut visited = HashSet::new();
    visited.insert(start);
    queue.push_back((start, 0));

    while let Some((node, cost)) = queue.pop_front() {
        if goal(&node) {
            return Some(cost);
        }

        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = (
                (node.0 as isize + di) as usize,
                (node.1 as isize + dj) as usize,
            );

            if !graph.contains_key(&next) || visited.contains(&next) {
                continue;
            }

            if !reachable(&node, &next) {
                continue;
            }

            queue.push_back((next, cost + 1));
            visited.insert(next);
        }
    }
    None
}

fn main() -> Result<()> {
    let input = include_str!("./day12.txt");

    let mut start = (0, 0);
    let mut end = (0, 0);

    let graph: HashMap<Position, u8> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| line.chars().enumerate().map(move |(j, c)| ((i, j), c)))
        .map(|(pos, c)| {
            let c = match c {
                'S' => {
                    start = pos;
                    'a'
                }
                'E' => {
                    end = pos;
                    'z'
                }
                _ => c,
            };
            (pos, c as u8)
        })
        .collect();

    // PART 1
    let solution = bfs(
        &graph,
        start,
        |node| node == &end,                         // goal
        |node, next| graph[next] <= graph[node] + 1, // reachable adjacents
    )
    .unwrap();
    println!("part1 is :{:?}", solution);

    // PART 2
    let solution = bfs(
        &graph,
        end,
        |node| graph[node] == 'a' as u8,             // goal
        |node, next| graph[node] - 1 <= graph[next], // reachable adjacents
    )
    .unwrap();
    println!("part2 is :{:?}", solution);
    Ok(())
}
