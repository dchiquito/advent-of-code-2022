use std::collections::HashSet;
use std::ops::Add;

use crate::advent;

type Pos = (i32, i32, i32);
type Volume = HashSet<Pos>;

fn read_input() -> Volume {
    let lines = advent::read_input(18);
    lines.iter().map(|line| {
        let mut arr = line.split(",");
        (arr.next().unwrap().parse().unwrap(), arr.next().unwrap().parse().unwrap(), arr.next().unwrap().parse().unwrap())
    }).collect()
}

fn adjacents((x, y, z): &Pos) -> Vec<Pos> {
    let (x, y, z) = (*x, *y, *z);
    vec![
        (x-1, y, z),
        (x+1, y, z),
        (x, y-1, z),
        (x, y+1, z),
        (x, y, z-1),
        (x, y, z+1),
    ]
}

fn surface_area(voxels: &Volume) -> u32 {
    voxels
        .iter()
        .map(|pos| 
             adjacents(&pos)
             .iter()
             .filter(|adj| !voxels.contains(adj)).count() as u32)
        .fold(0, u32::add)
}
pub fn solve_1() -> u32 {
    let voxels = read_input();
    surface_area(&voxels)
}

fn max_pos(volume: &Volume) -> Pos {
    let (mut mx, mut my, mut mz) = (0, 0, 0);
    for (x, y, z) in volume.iter() {
        mx = mx.max(*x);
        my = my.max(*y);
        mz = mz.max(*z);
    }
    (mx, my, mz)
}

fn invert(volume: &Volume) -> Volume {
    let mut inversion = HashSet::new();
    let (mx, my, mz) = max_pos(volume);
    for x in 0..mx+1 {
        for y in 0..my+1 {
            for z in 0..mz+1 {
                if !volume.contains(&(x,y,z)) {
                    inversion.insert((x,y,z));
                }
            }
        }
    }
    inversion
}

fn find_subset(volume: &Volume, start: &Pos, subset: &mut Volume) {
    for adj in adjacents(start) {
        if volume.contains(&adj) && !subset.contains(&adj) {
            subset.insert(adj.clone());
            find_subset(volume, &adj, subset);
        }
    }
}

fn solve_2() -> u32 {
    let voxels = read_input();
    let inversion = invert(&voxels);
    let mut outside = HashSet::new();
    find_subset(&inversion, &(0,0,0), &mut outside);
    let filled_voxels = invert(&outside);
    surface_area(&filled_voxels)
}

pub fn solve() {
    println!("{}", solve_1());
    println!("{}", solve_2());
}
