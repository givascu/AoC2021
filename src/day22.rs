use std::{cmp, collections::HashSet};

type Point3D = (i64, i64, i64);
type Instruction = (bool, (Segment, Segment, Segment));

#[derive(Debug, Copy, Clone)]
struct Segment {
    x0: i64,
    x1: i64,
}

impl Segment {
    fn new(x0: i64, x1: i64) -> Segment {
        Segment { x0, x1 }
    }

    fn length(&self) -> i64 {
        self.x1 - self.x0 + 1
    }

    fn is_intersecting(&self, other: &Segment) -> bool {
        (self.x0..=self.x1).contains(&other.x0)
            || (self.x0..=self.x1).contains(&other.x1)
            || (other.x0..=other.x1).contains(&self.x0)
            || (other.x0..=other.x1).contains(&self.x1)
    }

    fn get_intersection(&self, other: &Segment) -> Segment {
        assert!(self.is_intersecting(other));
        Segment::new(
            if other.x0 < self.x0 {
                self.x0
            } else {
                other.x0
            },
            if other.x1 < self.x1 {
                other.x1
            } else {
                self.x1
            },
        )
    }
}

fn read_input() -> Vec<Instruction> {
    include_str!("../input/22.txt")
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
                    Segment::new(
                        parts.0.parse::<i64>().unwrap(),
                        parts.1.parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            assert!(parts.len() == 3);

            (light, (parts[0], parts[1], parts[2]))
        })
        .collect()
}

pub fn solve_1() -> usize {
    let instructions = read_input();

    let mut lights: HashSet<Point3D> = HashSet::new();
    for ins in instructions {
        let rx = (cmp::max(ins.1 .0.x0, -50), cmp::min(ins.1 .0.x1, 50));
        let ry = (cmp::max(ins.1 .1.x0, -50), cmp::min(ins.1 .1.x1, 50));
        let rz = (cmp::max(ins.1 .2.x0, -50), cmp::min(ins.1 .2.x1, 50));

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

struct Cuboid {
    x: Segment,
    y: Segment,
    z: Segment,
    off: Vec<Cuboid>,
}

impl Cuboid {
    fn new(x: Segment, y: Segment, z: Segment) -> Cuboid {
        Cuboid {
            x,
            y,
            z,
            off: Vec::new(),
        }
    }

    fn is_intersecting(&self, other: &Cuboid) -> bool {
        self.x.is_intersecting(&other.x)
            && self.y.is_intersecting(&other.y)
            && self.z.is_intersecting(&other.z)
    }

    fn subtract(&mut self, other: &Cuboid) {
        if self.is_intersecting(other) {
            let x = self.x.get_intersection(&other.x);
            let y = self.y.get_intersection(&other.y);
            let z = self.z.get_intersection(&other.z);

            for o in &mut self.off {
                o.subtract(other);
            }

            self.off.push(Cuboid::new(x, y, z));
        }
    }

    fn num_lights(&self) -> i64 {
        self.x.length() * self.y.length() * self.z.length()
            - self.off.iter().map(Cuboid::num_lights).sum::<i64>()
    }
}

pub fn solve_2() -> i64 {
    let instructions = read_input();

    let mut cuboids: Vec<Cuboid> = Vec::new();
    for ins in instructions {
        let cuboid = Cuboid::new(ins.1 .0, ins.1 .1, ins.1 .2);

        for c in &mut cuboids {
            c.subtract(&cuboid);
        }

        if ins.0 {
            cuboids.push(cuboid);
        }
    }

    cuboids.iter().map(Cuboid::num_lights).sum()
}
