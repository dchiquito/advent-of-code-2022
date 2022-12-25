use regex::Regex;
use std::ops::Mul;

use crate::advent;

#[derive(Debug, PartialEq, Eq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug)]
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

#[derive(Debug, Clone)]
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
    fn can_build(&self, blueprint: &Blueprint, material: &Material) -> bool {
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
    fn build(&mut self, blueprint: &Blueprint, material: &Material) {
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
    fn minute(&self, blueprint: &Blueprint, material: &Option<Material>) -> State {
        if let Some(material) = material {
            let mut state = self.minute(blueprint, &None);
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
}

// A holder for the recursion context
struct Ctx {
    blueprint: Blueprint,
    depth: u32,
    max_geodes: u32,
}
impl Ctx {
    fn recurse(&mut self, state: &State) {
        if state.tick == self.depth {
            self.max_geodes = self.max_geodes.max(state.geodes);
            return;
        }
        let mut minutes_remaining = self.depth - state.tick;
        // Assuming we manufacture an obsidian bot every turn, how long until we have enough for a
        // geode bot?
        // obsidian <= (first+last)*n/2 = (bot + (bot + t-1)) * t / 2
        if state.geode_robots == 0 {
            let mut obsidian_time = 0;
            let mut obsidian = state.obsidian;
            while obsidian < self.blueprint.geode.1 {
                obsidian += state.obsidian_robots + obsidian_time;
                obsidian_time += 1;
            }
            minutes_remaining = minutes_remaining - obsidian_time;
            if minutes_remaining <= 0 {
                return;
            }
        }
        let max_possible_geodes = state.geodes
            + (state.geode_robots * minutes_remaining)
            + ((minutes_remaining - 1) * minutes_remaining / 2);
        //println!("max possible geodes {}", max_possible_geodes);
        if max_possible_geodes <= self.max_geodes {
            return;
        }
        if state.can_build(&self.blueprint, &Material::Geode) {
            self.recurse(&state.minute(&self.blueprint, &Some(Material::Geode)));
        }
        if state.can_build(&self.blueprint, &Material::Obsidian) {
            self.recurse(&state.minute(&self.blueprint, &Some(Material::Obsidian)));
        }
        if state.can_build(&self.blueprint, &Material::Clay) {
            self.recurse(&state.minute(&self.blueprint, &Some(Material::Clay)));
        }
        if state.can_build(&self.blueprint, &Material::Ore) {
            self.recurse(&state.minute(&self.blueprint, &Some(Material::Ore)));
        }
        self.recurse(&state.minute(&self.blueprint, &None));
    }
}

fn find_max_geodes(blueprint: &Blueprint, depth: u32) -> u32 {
    let mut ctx = Ctx {
        blueprint: blueprint.clone(),
        depth,
        max_geodes: 0,
    };
    ctx.recurse(&State::new());
    ctx.max_geodes
}
fn solve_1() -> u32 {
    let blueprints = read_blueprints();
    let mut total_quality = 0;
    for blueprint in blueprints.iter() {
        let geodes = find_max_geodes(blueprint, 24);
        total_quality += blueprint.id * geodes;
    }
    total_quality
}

fn solve_2() -> u32 {
    let blueprints = read_blueprints();
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            let g = find_max_geodes(blueprint, 32);
            g
        })
        .fold(1, u32::mul)
}
pub fn solve() {
    // runs in 81 seconds
    // also it's wrong :((((
    println!("{}", solve_1());
    println!("{}", solve_2());
}
