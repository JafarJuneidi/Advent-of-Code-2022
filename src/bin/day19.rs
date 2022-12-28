mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{self, line_ending},
        multi::separated_list1,
        sequence::{delimited, separated_pair},
        IResult, Parser,
    };

    use super::*;

    fn blueprint(input: &str) -> IResult<&str, Blueprint> {
        let (input, id) = delimited(tag("Blueprint "), complete::u64, tag(":"))(input)?;

        let (input, ore) =
            delimited(tag(" Each ore robot costs "), complete::u64, tag(" ore."))(input)?;

        let (input, clay) =
            delimited(tag(" Each clay robot costs "), complete::u64, tag(" ore."))(input)?;

        let (input, obsidian) = delimited(
            tag(" Each obsidian robot costs "),
            separated_pair(complete::u64, tag(" ore and "), complete::u64).map(|pair| {
                ObsidianRequirements {
                    ore: pair.0 as usize,
                    clay: pair.1 as usize,
                }
            }),
            tag(" clay."),
        )(input)?;

        let (input, geode) = delimited(
            tag(" Each geode robot costs "),
            separated_pair(complete::u64, tag(" ore and "), complete::u64).map(|pair| {
                GeodeRequirements {
                    ore: pair.0 as usize,
                    obsidian: pair.1 as usize,
                }
            }),
            tag(" obsidian."),
        )(input)?;

        Ok((
            input,
            Blueprint {
                id: id as usize,
                ore: ore as usize,
                clay: clay as usize,
                obsidian,
                geode,
            },
        ))
    }

    pub fn blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
        separated_list1(line_ending, blueprint)(input)
    }
}

use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
struct ObsidianRequirements {
    ore: usize,
    clay: usize,
}

#[derive(Debug)]
struct GeodeRequirements {
    ore: usize,
    obsidian: usize,
}

#[derive(Debug)]
pub struct Blueprint {
    id: usize,
    ore: usize,
    clay: usize,
    obsidian: ObsidianRequirements,
    geode: GeodeRequirements,
}

#[derive(Debug, Clone)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_bots: usize,
    clay_bots: usize,
    obsidian_bots: usize,
    geode_bots: usize,
}
impl Resources {
    fn run(&mut self) {
        self.ore += self.ore_bots;
        self.clay += self.clay_bots;
        self.obsidian += self.obsidian_bots;
        self.geode += self.geode_bots;
    }
    fn try_build_geode(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore >= blueprint.geode.ore && self.obsidian >= blueprint.geode.obsidian {
            let mut new_resources = self.clone();
            new_resources.ore -= blueprint.geode.ore;
            new_resources.obsidian -= blueprint.geode.obsidian;
            new_resources.run();
            new_resources.geode_bots += 1;
            Some(new_resources)
        } else {
            None
        }
    }
    fn try_build_obsidian(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore >= blueprint.obsidian.ore
            && self.clay >= blueprint.obsidian.clay
            && self.obsidian_bots < blueprint.geode.obsidian
        {
            let mut new_resources = self.clone();
            new_resources.ore -= blueprint.obsidian.ore;
            new_resources.clay -= blueprint.obsidian.clay;
            new_resources.run();
            new_resources.obsidian_bots += 1;
            Some(new_resources)
        } else {
            None
        }
    }
    fn try_build_clay(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore >= blueprint.clay && self.clay_bots < blueprint.obsidian.clay {
            let mut new_resources = self.clone();
            new_resources.ore -= blueprint.clay;
            new_resources.run();
            new_resources.clay_bots += 1;
            Some(new_resources)
        } else {
            // println!(
            //     "couldn't buy obsidian requiring {:?}\n with,\n {:?}",
            //     blueprint.obsidian, self
            // );
            None
        }
    }
    fn try_build_ore(&self, blueprint: &Blueprint) -> Option<Resources> {
        if self.ore >= blueprint.ore
            && self.ore_bots
                < blueprint
                    .clay
                    .max(blueprint.obsidian.ore)
                    .max(blueprint.geode.ore)
        {
            let mut new_resources = self.clone();
            new_resources.ore -= blueprint.ore;
            new_resources.run();
            new_resources.ore_bots += 1;
            Some(new_resources)
        } else {
            None
        }
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            ore: Default::default(),
            clay: Default::default(),
            obsidian: Default::default(),
            geode: Default::default(),
            ore_bots: 1,
            clay_bots: Default::default(),
            obsidian_bots: Default::default(),
            geode_bots: Default::default(),
        }
    }
}

fn step_blueprint(blueprint: &Blueprint, resources: Resources, time_left: usize) -> Vec<Resources> {
    if let Some(time_left) = time_left.checked_sub(1) {
        // build,
        // collect
        // plus new bots
        let new_resources = match (
            resources.try_build_geode(blueprint),
            resources.try_build_obsidian(blueprint),
            resources.try_build_clay(blueprint),
        ) {
            (Some(resources), ..) => {
                // println!("Bought geode");
                Some(resources)
            }
            (None, Some(resources), ..) => {
                // println!("Bought obsidian");
                Some(resources)
            }
            (None, None, Some(resources), ..) => {
                // println!("Bought clay");
                Some(resources)
            }

            _ => None,
        };
        [
            resources
                .try_build_ore(blueprint)
                .map(|new_resources| step_blueprint(blueprint, new_resources, time_left)),
            new_resources.map(|new_resources| step_blueprint(blueprint, new_resources, time_left)),
            Some(step_blueprint(
                blueprint,
                {
                    let mut new_resources = resources.clone();
                    new_resources.run();
                    new_resources
                },
                time_left,
            )),
        ]
        .into_iter()
        .filter_map(|v| v)
        .flatten()
        .collect()
    } else {
        vec![resources]
    }
}

fn part1(input: &str) -> String {
    let (_, blueprints) = parser::blueprints(input).unwrap();

    let maxes: usize = blueprints
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let max = step_blueprint(&blueprint, Resources::default(), 24)
                .iter()
                .map(|v| v.geode)
                .max()
                .unwrap();
            let max = (i + 1) * max;
            max
        })
        .sum::<usize>();

    maxes.to_string()
}

fn part2(input: &str) -> String {
    let (_, blueprints) = parser::blueprints(input).unwrap();

    let maxes: usize = blueprints[0..3]
        .par_iter()
        .enumerate()
        .map(|(i, blueprint)| {
            let max = step_blueprint(&blueprint, Resources::default(), 32)
                .iter()
                .map(|v| v.geode)
                .max()
                .unwrap();
            let max = (i + 1) * max;
            max
        })
        .sum::<usize>();

    maxes.to_string()
}

fn main() {
    let input = include_str!("./day19.txt");
    let part1 = part1(input);
    // let part2 = part2(input);

    println!("part1 : {}", part1);
    // println!("part2 : {}", part2);
}
