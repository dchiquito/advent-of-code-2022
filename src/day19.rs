use regex::Regex;
use std::ops::Add;
use std::ops::Mul;

use crate::advent;

#[derive(Debug, PartialEq, Eq)]
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
    fn heuristic(&self, _blueprint: &Blueprint, depth: u32) -> u32 {
        let ticks = depth - self.tick;
        let geodes = self.geodes + (self.geode_robots * ticks) + (0..ticks).fold(0, u32::add);
        geodes
    }
}

fn find_max_geodes(blueprint: &Blueprint, depth: u32) -> u32 {
    let max_ore_robots = blueprint
        .ore
        .max(blueprint.clay)
        .max(blueprint.obsidian.0)
        .max(blueprint.geode.0);
    let max_clay_robots = blueprint.obsidian.1;
    let max_obsidian_robots = blueprint.geode.1;
    println!(
        "{} {} {}",
        max_ore_robots, max_clay_robots, max_obsidian_robots
    );
    let mut stack: Vec<State> = Vec::new();
    stack.push(State::new());
    let mut max_geodes = 0;
    while let Some(state) = stack.pop() {
        // println!("Stackin up {:?}", state);
        if state.tick == depth {
            max_geodes = max_geodes.max(state.geodes);
            continue;
        }
        if state.heuristic(blueprint, depth) < max_geodes {
            continue;
        }
        stack.push(state.minute(blueprint, &None));
        for material in vec![
            Material::Ore,
            Material::Clay,
            Material::Obsidian,
            Material::Geode,
        ] {
            // don't bother with geodes if there are no obsidian bots
            if (material == Material::Geode && state.obsidian_robots == 0) ||
            // don't bother with obsidian if there are no clay bots or we already have enough
            (material == Material::Obsidian
                && (state.clay_robots == 0 || state.obsidian_robots >= max_obsidian_robots)) ||
            // don't bother with clay if there are already enough
            (material == Material::Clay && state.clay_robots >= max_clay_robots) || 
            // don't bother with ore if there are already enough
            (material == Material::Ore && state.ore_robots >= max_ore_robots) {
                break;
            }
            let mut temp_state = state.clone();
            // wait until the material is buildable
            while !temp_state.can_build(blueprint, &material) {
                temp_state = temp_state.minute(blueprint, &None);
            }
            temp_state = temp_state.minute(blueprint, &Some(material));
            // check to make sure we haven't waited until after the max depth
            if temp_state.tick <= depth {
                stack.push(temp_state);
            }
        }
    }
    max_geodes
}
fn solve_1() -> u32 {
    let blueprints = read_blueprints();
    let mut total_quality = 0;
    for blueprint in blueprints.iter() {
        let geodes = find_max_geodes(blueprint, 24);
        println!("{:?} {}", blueprint, geodes);
        total_quality += blueprint.id * geodes;
    }
    total_quality
}

fn _solve_2() -> u32 {
    let blueprints = read_blueprints();
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            let g = find_max_geodes(blueprint, 32);
            println!("{:?} -> {}", blueprint, g);
            g
        })
        .fold(1, u32::mul)
}
pub fn solve() {
    // runs in 81 seconds
    // also it's wrong :((((
    println!("{}", solve_1());
    // println!("{}", solve_2());
}
