use itertools::iproduct;
use std::collections::{HashMap, HashSet};

pub fn solve_1() -> usize {
    let (_, beacons) = solver();
    beacons.len()
}

pub fn solve_2() -> i32 {
    let (scanners, _) = solver();
    let mut result = 0;
    for (s1, s2) in iproduct!(scanners.iter(), scanners.iter()) {
        result = result.max(s1.origin.manhattan(&s2.origin));
    }
    result
}

fn solver() -> (Vec<Scanner>, HashSet<Point>) {
    let mut scanners = build_input_scanners();
    scanners[0].fixed = true;

    let mut reference_beacons = scanners[0]
        .beacons
        .iter()
        .copied()
        .collect::<HashSet<Point>>();

    while !scanners.iter().all(|s| s.fixed) {
        for scanner in &mut scanners {
            if scanner.fixed {
                continue;
            }

            for rotation in ROTATIONS {
                let rotated_beacons = scanner
                    .beacons
                    .iter()
                    .map(|b| b.rotate(rotation))
                    .collect::<Vec<_>>();

                let (mut x_map, mut y_map, mut z_map) =
                    (HashMap::new(), HashMap::new(), HashMap::new());
                for (beacon, ref_beacon) in iproduct!(&rotated_beacons, &reference_beacons) {
                    *x_map.entry(ref_beacon.x - beacon.x).or_insert(0_i32) += 1;
                    *y_map.entry(ref_beacon.y - beacon.y).or_insert(0_i32) += 1;
                    *z_map.entry(ref_beacon.z - beacon.z).or_insert(0_i32) += 1;
                }

                let x_offset = x_map.iter().find(|(_, &v)| v >= 12).map(|(&k, _)| k);
                let y_offset = y_map.iter().find(|(_, &v)| v >= 12).map(|(&k, _)| k);
                let z_offset = z_map.iter().find(|(_, &v)| v >= 12).map(|(&k, _)| k);

                if x_offset.is_some() && y_offset.is_some() && z_offset.is_some() {
                    let x_new = x_map
                        .iter()
                        .max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
                        .unwrap()
                        .0;
                    let y_new = y_map
                        .iter()
                        .max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
                        .unwrap()
                        .0;
                    let z_new = z_map
                        .iter()
                        .max_by(|(_, lhs), (_, rhs)| lhs.cmp(rhs))
                        .unwrap()
                        .0;

                    scanner.origin = Point::new(*x_new, *y_new, *z_new);
                    scanner.beacons = rotated_beacons.clone();
                    scanner.fixed = true;

                    for beacon in &rotated_beacons {
                        reference_beacons.insert(beacon.translate(*x_new, *y_new, *z_new));
                    }

                    break;
                }
            }
        }
    }

    (scanners, reference_beacons)
}

fn build_input_scanners() -> Vec<Scanner> {
    include_str!("../input/19.txt")
        .split("\n\n")
        .map(|input| {
            let beacons = input
                .lines()
                .skip(1)
                .map(|line| {
                    let mut parts = line.split(',');
                    let x = parts.next().unwrap().parse().unwrap();
                    let y = parts.next().unwrap().parse().unwrap();
                    let z = parts.next().unwrap().parse().unwrap();
                    Point::new(x, y, z)
                })
                .collect();
            Scanner::new(Point::default(), beacons, false)
        })
        .collect()
}

type Tuple3 = (Point, Point, Point);

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Self { x, y, z }
    }

    fn rotate(&self, rotation: Tuple3) -> Point {
        Point::new(
            self.x * rotation.0.x + self.y * rotation.0.y + self.z * rotation.0.z,
            self.x * rotation.1.x + self.y * rotation.1.y + self.z * rotation.1.z,
            self.x * rotation.2.x + self.y * rotation.2.y + self.z * rotation.2.z,
        )
    }

    fn translate(&self, dx: i32, dy: i32, dz: i32) -> Point {
        Point::new(self.x + dx, self.y + dy, self.z + dz)
    }

    fn manhattan(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Debug)]
struct Scanner {
    origin: Point,
    beacons: Vec<Point>,
    fixed: bool,
}

impl Scanner {
    fn new(origin: Point, beacons: Vec<Point>, fixed: bool) -> Scanner {
        Self {
            origin,
            beacons,
            fixed,
        }
    }
}

const ROTATIONS: [Tuple3; 24] = [
    (
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: 0, z: 1 },
    ),
    (
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: 0, z: -1 },
        Point { x: 0, y: 1, z: 0 },
    ),
    (
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: -1, z: 0 },
        Point { x: 0, y: 0, z: -1 },
    ),
    (
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: 0, z: 1 },
        Point { x: 0, y: -1, z: 0 },
    ),
    (
        Point { x: 0, y: -1, z: 0 },
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: 0, z: 1 },
    ),
    (
        Point { x: 0, y: 0, z: 1 },
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: 1, z: 0 },
    ),
    (
        Point { x: 0, y: 1, z: 0 },
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: 0, z: -1 },
    ),
    (
        Point { x: 0, y: 0, z: -1 },
        Point { x: 1, y: 0, z: 0 },
        Point { x: 0, y: -1, z: 0 },
    ),
    (
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: -1, z: 0 },
        Point { x: 0, y: 0, z: 1 },
    ),
    (
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 0, z: -1 },
        Point { x: 0, y: -1, z: 0 },
    ),
    (
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: 0, z: -1 },
    ),
    (
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 0, z: 1 },
        Point { x: 0, y: 1, z: 0 },
    ),
    (
        Point { x: 0, y: 1, z: 0 },
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 0, z: 1 },
    ),
    (
        Point { x: 0, y: 0, z: 1 },
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: -1, z: 0 },
    ),
    (
        Point { x: 0, y: -1, z: 0 },
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 0, z: -1 },
    ),
    (
        Point { x: 0, y: 0, z: -1 },
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 1, z: 0 },
    ),
    (
        Point { x: 0, y: 0, z: -1 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: 1, y: 0, z: 0 },
    ),
    (
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: 0, z: 1 },
        Point { x: 1, y: 0, z: 0 },
    ),
    (
        Point { x: 0, y: 0, z: 1 },
        Point { x: 0, y: -1, z: 0 },
        Point { x: 1, y: 0, z: 0 },
    ),
    (
        Point { x: 0, y: -1, z: 0 },
        Point { x: 0, y: 0, z: -1 },
        Point { x: 1, y: 0, z: 0 },
    ),
    (
        Point { x: 0, y: 0, z: -1 },
        Point { x: 0, y: -1, z: 0 },
        Point { x: -1, y: 0, z: 0 },
    ),
    (
        Point { x: 0, y: -1, z: 0 },
        Point { x: 0, y: 0, z: 1 },
        Point { x: -1, y: 0, z: 0 },
    ),
    (
        Point { x: 0, y: 0, z: 1 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: -1, y: 0, z: 0 },
    ),
    (
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: 0, z: -1 },
        Point { x: -1, y: 0, z: 0 },
    ),
];
