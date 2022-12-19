use regex::Regex;
use std::ops::Add;

use crate::advent;

#[derive(Debug)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}

fn read_blueprints() -> Vec<Blueprint> {
    let lines = advent::read_input(19);
    let re = Regex::new(r"Blueprint (\d+).*(\d+) ore.*(\d+) ore.*(\d+) ore and (\d+) clay.* (\d+) ore and (\d+) obsidian").unwrap();
    lines
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Blueprint {
                id: (&captures[1]).parse().unwrap(),
                ore: (&captures[2]).parse().unwrap(),
                clay: (&captures[3]).parse().unwrap(),
                obsidian: (
                    (&captures[4]).parse().unwrap(),
                    (&captures[5]).parse().unwrap(),
                ),
                geode: (
                    (&captures[6]).parse().unwrap(),
                    (&captures[7]).parse().unwrap(),
                ),
            }
        })
        .collect()
}

#[derive(Debug)]
struct State {
    tick: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl State {
    fn new() -> State {
        State {
            tick: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
    fn can_build(&self, blueprint: &Blueprint, material: Material) -> bool {
        match material {
            Material::Ore => self.ore >= blueprint.ore,
            Material::Clay => self.ore >= blueprint.clay && self.clay_robots < blueprint.obsidian.1,
            Material::Obsidian => {
                self.ore >= blueprint.obsidian.0
                    && self.clay >= blueprint.obsidian.1
                    && self.obsidian_robots < blueprint.geode.1
            }
            Material::Geode => self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1,
        }
    }
    fn build(&mut self, blueprint: &Blueprint, material: Material) {
        match material {
            Material::Ore => {
                self.ore -= blueprint.ore;
                self.ore_robots += 1
            }
            Material::Clay => {
                self.ore -= blueprint.clay;
                self.clay_robots += 1
            }
            Material::Obsidian => {
                self.ore -= blueprint.obsidian.0;
                self.clay -= blueprint.obsidian.1;
                self.obsidian_robots += 1;
            }
            Material::Geode => {
                self.ore -= blueprint.geode.0;
                self.obsidian -= blueprint.geode.1;
                self.geode_robots += 1;
            }
        }
    }
    fn minute(&self, blueprint: &Blueprint, material: Option<Material>) -> State {
        if let Some(material) = material {
            let mut state = self.minute(blueprint, None);
            state.build(blueprint, material);
            state
        } else {
            State {
                tick: self.tick + 1,
                ore: self.ore + self.ore_robots,
                clay: self.clay + self.clay_robots,
                obsidian: self.obsidian + self.obsidian_robots,
                geodes: self.geodes + self.geode_robots,
                ore_robots: self.ore_robots,
                clay_robots: self.clay_robots,
                obsidian_robots: self.obsidian_robots,
                geode_robots: self.geode_robots,
            }
        }
    }
    fn heuristic(&self, _blueprint: &Blueprint) -> u32 {
        let ticks = 24 - self.tick;
        let geodes = self.geodes + (self.geode_robots * ticks) + (0..ticks).fold(0, u32::add);
        geodes
    }
}

fn find_max_geodes(blueprint: &Blueprint, state: &State, best_so_far: u32) -> u32 {
    //println!(
    //    "finding {:?} {} {}",
    //    state,
    //    best_so_far,
    //    state.heuristic(blueprint)
    //);
    if state.tick > 23 {
        // println!("Bottomed out {:?}", state);
        state.geodes
    } else if state.heuristic(blueprint) < best_so_far {
        //println!("abort {:?}", state);
        0
    } else {
        let mut max_geodes = best_so_far;
        if state.can_build(blueprint, Material::Geode) {
            max_geodes = max_geodes.max(find_max_geodes(
                blueprint,
                &state.minute(blueprint, Some(Material::Geode)),
                max_geodes,
            ));
        }
        if state.can_build(blueprint, Material::Obsidian) {
            max_geodes = max_geodes.max(find_max_geodes(
                blueprint,
                &state.minute(blueprint, Some(Material::Obsidian)),
                max_geodes,
            ));
        }
        if state.can_build(blueprint, Material::Clay) {
            max_geodes = max_geodes.max(find_max_geodes(
                blueprint,
                &state.minute(blueprint, Some(Material::Clay)),
                max_geodes,
            ));
        }
        if state.can_build(blueprint, Material::Ore) {
            max_geodes = max_geodes.max(find_max_geodes(
                blueprint,
                &state.minute(blueprint, Some(Material::Ore)),
                max_geodes,
            ));
        }
        {
            max_geodes = max_geodes.max(find_max_geodes(
                blueprint,
                &state.minute(blueprint, None),
                best_so_far,
            ));
        }
        if max_geodes == 100 {
            println!(
                "{} [{} {} {} {}] robots [{} {} {} {}] {}",
                state.tick,
                state.ore,
                state.clay,
                state.obsidian,
                state.geodes,
                state.ore_robots,
                state.clay_robots,
                state.obsidian_robots,
                state.geode_robots,
                state.heuristic(blueprint),
            );
        }
        max_geodes
    }
}
fn solve_1() -> u32 {
    let blueprints = read_blueprints();
    let state = State::new();
    let mut total_quality = 0;
    for blueprint in blueprints.iter() {
        let geodes = find_max_geodes(blueprint, &state, 0);
        println!("{:?} {}", blueprint, geodes);
        total_quality += blueprint.id * geodes;
    }
    total_quality
}
pub fn solve() {
    // runs in 5.5 minutes :(
    println!("{}", solve_1());
}
