use std::{cmp, collections::HashSet};

type MyRange = (i64, i64);
type Point3D = (i64, i64, i64);
type Instruction = (bool, (MyRange, MyRange, MyRange));

pub fn solve_1() -> usize {
    let instructions: Vec<Instruction> = include_str!("../input/22.txt")
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();
            let light = parts.0 == "on";

            let parts = parts
                .1
                .trim()
                .split(',')
                .map(|x| {
                    let parts = x.split_once('=').unwrap().1.split_once("..").unwrap();
                    (
                        parts.0.parse::<i64>().unwrap(),
                        parts.1.parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            assert!(parts.len() == 3);

            (light, (parts[0], parts[1], parts[2]))
        })
        .collect();

    let mut lights: HashSet<Point3D> = HashSet::new();
    for ins in instructions {
        let rx = (cmp::max(ins.1 .0 .0, -50), cmp::min(ins.1 .0 .1, 50));
        let ry = (cmp::max(ins.1 .1 .0, -50), cmp::min(ins.1 .1 .1, 50));
        let rz = (cmp::max(ins.1 .2 .0, -50), cmp::min(ins.1 .2 .1, 50));

        for (x, y, z) in itertools::iproduct!(rx.0..=rx.1, ry.0..=ry.1, rz.0..=rz.1) {
            if ins.0 {
                lights.insert((x, y, z));
            } else {
                lights.remove(&(x, y, z));
            }
        }
    }

    lights.len()
}

pub fn solve_2() -> u64 {
    0
}
