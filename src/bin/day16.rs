// I still can't fully wrap my head around this
use anyhow::Result;
use pathfinding::prelude::{astar, dijkstra_all};
use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitSet(u64);

impl BitSet {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub fn insert(&mut self, k: usize) {
        self.0 |= 0b1 << k;
    }

    pub fn contains(&self, k: usize) -> bool {
        self.0 & (0b1 << k) != 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Valve<'a> {
    name: &'a str,
    flow: i64,
    exit: Vec<&'a str>,
}

#[derive(Debug)]
pub struct CompressedValve<'a> {
    name: &'a str,
    flow: i64,
    exit: Vec<(&'a str, i64)>,
}

#[derive(Debug)]
struct IdValve {
    name: usize,
    flow: i64,
    exit: Vec<(usize, i64)>,
}

struct IdCache<T> {
    cache: HashMap<T, usize>,
}

impl<T: PartialEq + Eq + Hash> IdCache<T> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn id(&mut self, key: T) -> usize {
        let id = self.cache.len() as usize;
        match self.cache.entry(key) {
            Entry::Vacant(e) => {
                e.insert(id);
                return id;
            }
            Entry::Occupied(e) => *e.get(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    remaining: i64,
    current: (usize, i64),
    opened: BitSet,
}

impl State {
    fn new(valves: &HashMap<usize, IdValve>, start: usize) -> Self {
        let mut opened = BitSet::new();
        if valves[&start].flow == 0 {
            opened.insert(start);
        }

        Self {
            remaining: 30,
            current: (start, 0),
            opened,
        }
    }

    fn opened(&self, valve: usize) -> bool {
        self.opened.contains(valve)
    }

    fn moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        let mut nexts = vec![];
        let (dest, distance) = self.current;

        if distance > 0 {
            let mut next = self.clone();
            next.remaining -= 1;
            next.current = (dest, distance - 1);
            nexts.push((next, 0));
            return nexts;
        }

        if !self.opened(dest) {
            let mut next = self.clone();
            next.remaining -= 1;
            next.opened.insert(dest);
            let cost = next.remaining * valves[&dest].flow;
            nexts.push((next, -cost));
        }

        for (next_dest, next_distance) in &valves[&dest].exit {
            let mut next = self.clone();
            next.remaining -= 1;
            next.current = (*next_dest, next_distance - 1);
            nexts.push((next, 0));
        }

        nexts
    }

    fn finished(&self, valves: &HashMap<usize, IdValve>) -> bool {
        debug_assert!(self.remaining >= 0);
        self.remaining == 0 || self.opened.len() == valves.len()
    }

    fn heuristic(&self, sorted_by_flow: &[(usize, i64)]) -> i64 {
        let mut h = 0;
        let mut r = self.remaining;
        let mut i = 0;

        while r >= 1 && i < sorted_by_flow.len() {
            let inc = sorted_by_flow[i..]
                .iter()
                .position(|(name, _)| !self.opened(*name));

            if let Some(inc) = inc {
                i += inc;
            } else {
                break;
            }

            r -= 1;
            h += sorted_by_flow[i].1 * r;
            i += 1;
            r -= 1;
        }

        -h
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct StateWithElephant {
    remaining: i64,
    actors: [(usize, i64); 2],
    opened: BitSet,
}

impl StateWithElephant {
    fn new(valves: &HashMap<usize, IdValve>, start: usize) -> Self {
        let mut opened = BitSet::new();
        if valves[&start].flow == 0 {
            opened.insert(start);
        }

        Self {
            remaining: 26,
            actors: [(start, 0), (start, 0)],
            opened,
        }
    }

    fn opened(&self, valve: usize) -> bool {
        self.opened.contains(valve)
    }

    fn finished(&self, valves: &HashMap<usize, IdValve>) -> bool {
        assert!(self.remaining >= 0);
        self.remaining == 0 || self.opened.len() == valves.len()
    }

    fn actor_moves(&self, valves: &HashMap<usize, IdValve>, actor: usize) -> Vec<(Self, i64)> {
        let mut nexts = vec![];
        let (dest, distance) = self.actors[actor];

        if distance > 0 {
            let mut next = *self;
            next.actors[actor] = (dest, distance - 1);
            nexts.push((next, 0));
            return nexts;
        }

        if !self.opened(dest) {
            let mut next = *self;
            next.opened.insert(dest);
            let cost = next.remaining * valves[&dest].flow;
            nexts.push((next, -cost));
        }

        for (next_dest, next_distance) in &valves[&dest].exit {
            let mut next = *self;
            next.actors[actor] = (*next_dest, next_distance - 1);
            nexts.push((next, 0));
        }

        nexts
    }

    fn you_moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        self.actor_moves(valves, 0)
    }

    fn elephant_moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        self.actor_moves(valves, 1)
    }

    fn moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        let mut moved = *self;
        moved.remaining -= 1;

        moved
            .you_moves(valves)
            .into_iter()
            .flat_map(|(n, c0)| {
                n.elephant_moves(valves)
                    .into_iter()
                    .map(move |(n, c1)| (n, c0 + c1))
            })
            .collect()
    }

    fn heuristic(&self, sorted_by_flow: &[(usize, i64)], valves: &HashMap<usize, IdValve>) -> i64 {
        self.actors
            .into_iter()
            .map(|actor| {
                let state = State {
                    remaining: self.remaining,
                    current: actor,
                    opened: self.opened,
                };
                one_actor_cost(&state, valves, sorted_by_flow)
            })
            .sum()
    }
}

// parsing logic using 'nom'
mod parser {

    use super::*;
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{self, line_ending},
        combinator::all_consuming,
        multi::separated_list1,
        sequence::preceded,
        IResult,
    };

    pub fn parse(input: &str) -> Vec<Valve> {
        let (_, valves) = all_consuming(parse_valves)(input).expect("valid complete parse");
        valves
    }

    fn parse_valves(input: &str) -> IResult<&str, Vec<Valve>> {
        separated_list1(line_ending, parse_valve)(input)
    }

    fn parse_valve(input: &str) -> IResult<&str, Valve> {
        let (input, name) = preceded(tag("Valve "), take(2usize))(input)?;
        let (input, flow) = preceded(tag(" has flow rate="), complete::i64)(input)?;
        let (input, exit) = preceded(
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
            separated_list1(tag(", "), take(2usize)),
        )(input)?;

        Ok((input, Valve { name, flow, exit }))
    }
}

fn compress<'a>(valves: &'a [Valve]) -> HashMap<&'a str, CompressedValve<'a>> {
    let valves: HashMap<&'a str, &Valve> = valves.iter().map(|v| (v.name, v)).collect();
    let mut compressed = HashMap::new();

    for &k in valves.keys().filter(|&&k| k == "AA" || valves[k].flow > 0) {
        let successors = dijkstra_all(&k, |n| {
            if *n != k && valves[n].flow > 0 {
                return vec![];
            }

            valves[n].exit.iter().map(|n| (*n, 1)).collect()
        });

        // println!("node: {} - successors: {:?}", k, successors);

        let exit = successors
            .iter()
            .filter(|(n, _)| valves[**n].flow > 0)
            .map(|(n, (_, c))| (*n, *c))
            .collect();

        // println!("node: {} - exit: {:?}", k, exit);

        compressed.insert(
            k,
            CompressedValve {
                name: k,
                flow: valves[k].flow,
                exit,
            },
        );
    }

    compressed
}

fn identifiers(valves: &HashMap<&str, CompressedValve>) -> (HashMap<usize, IdValve>, usize) {
    let mut cache = IdCache::new();

    for &name in valves.keys() {
        cache.id(name);
    }

    let id_valves = valves
        .values()
        .map(|val| {
            (
                cache.id(val.name),
                IdValve {
                    name: cache.id(val.name),
                    flow: val.flow,
                    exit: val.exit.iter().map(|(n, c)| (cache.id(n), *c)).collect(),
                },
            )
        })
        .collect();

    let start = cache.id("AA");

    (id_valves, start)
}

fn part_one(valves: &HashMap<usize, IdValve>, start: usize) -> i64 {
    let mut sorted_by_flow: Vec<(usize, i64)> = valves.values().map(|v| (v.name, v.flow)).collect();
    sorted_by_flow.sort_by_key(|(_, f)| *f);
    sorted_by_flow.reverse();

    -one_actor_cost(&State::new(valves, start), valves, &sorted_by_flow)
}

fn one_actor_cost(
    state: &State,
    valves: &HashMap<usize, IdValve>,
    sorted_by_flow: &[(usize, i64)],
) -> i64 {
    let (_, cost) = astar(
        state,
        |state| state.moves(valves),
        |state| state.heuristic(sorted_by_flow),
        |state| state.finished(valves),
    )
    .unwrap();
    cost
}

fn part_two(valves: &HashMap<usize, IdValve>, start: usize) -> i64 {
    let mut sorted_by_flow: Vec<(usize, i64)> = valves.values().map(|v| (v.name, v.flow)).collect();
    sorted_by_flow.sort_by_key(|(_, f)| *f);
    sorted_by_flow.reverse();

    let (_, cost) = astar(
        &StateWithElephant::new(valves, start),
        |state| state.moves(valves),
        |state| state.heuristic(&sorted_by_flow, valves),
        |state| state.finished(valves),
    )
    .unwrap();

    -cost
}

fn main() -> Result<()> {
    let valves = parser::parse(include_str!("./day16.txt").trim());
    let compressed = compress(&valves);

    // compressed
    //     .iter()
    //     .for_each(|(k, v)| println!("{:?} => {:?}", k, v));

    let (ids, start) = identifiers(&compressed);

    // ids.iter().for_each(|(k, v)| println!("{:?} => {:?}", k, v));

    println!("part1 is :{:?}", part_one(&ids, start));
    println!("part2 is :{:?}", part_two(&ids, start));
    Ok(())
}
