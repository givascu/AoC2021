use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

fn bits_to_dec(bits: &[u8]) -> usize {
    let mut num = 0;
    for (i, &b) in bits.iter().rev().enumerate() {
        if b == 1 {
            num += 2_usize.pow(i.try_into().unwrap());
        }
    }
    num
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point2D {
    y: i32,
    x: i32,
}

impl Point2D {
    fn new(y: i32, x: i32) -> Point2D {
        Point2D { y, x }
    }

    fn neighbors_with_self(&self) -> Vec<Point2D> {
        vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .iter()
        .map(|&(dy, dx)| Point2D::new(self.y + dy, self.x + dx))
        .collect::<Vec<_>>()
    }
}

struct Map2D {
    map: HashMap<Point2D, u8>,
    pmax: Point2D,
    pmin: Point2D,
}

impl Deref for Map2D {
    type Target = HashMap<Point2D, u8>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Map2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl Map2D {
    fn new(height: i32, width: i32) -> Map2D {
        Map2D {
            map: HashMap::new(),
            pmax: Point2D::new(height, width),
            pmin: Point2D::new(0, 0),
        }
    }

    fn _print(&self) {
        println!(
            "pmin: {:?}, pmax: {:?}, abs=({}, {})",
            self.pmin,
            self.pmax,
            self.pmax.y - self.pmin.y,
            self.pmax.x - self.pmin.x
        );
        for (y, x) in itertools::iproduct!(self.pmin.y..self.pmax.y, self.pmin.x..self.pmax.x) {
            let val = self.map.get(&Point2D::new(y, x)).unwrap();
            print!("{}", if *val == 1 { '#' } else { '.' });
            if x == self.pmax.x - 1 {
                println!();
            }
        }
    }

    fn apply_pattern(&mut self, pattern: &[u8], default: u8) {
        let mut new_map: HashMap<Point2D, u8> = HashMap::new();

        for (y, x) in
            itertools::iproduct!(self.pmin.y - 1..=self.pmax.y, self.pmin.x - 1..=self.pmax.x)
        {
            let mut nvals: Vec<u8> = Vec::new();

            for n in Point2D::new(y, x).neighbors_with_self() {
                let val = self.map.get(&n);
                match val {
                    Some(val) => nvals.push(*val),
                    None => nvals.push(default),
                }
            }

            let idx = bits_to_dec(&nvals);
            new_map.insert(Point2D::new(y, x), pattern[idx]);
        }

        self.map = new_map;
        self.pmin = Point2D::new(self.pmin.y - 1, self.pmin.x - 1);
        self.pmax = Point2D::new(self.pmax.y + 1, self.pmax.x + 1);
    }

    fn count_lights(&self) -> usize {
        self.map.iter().filter(|x| *x.1 == 1).count()
    }
}

fn read_input() -> (Map2D, Vec<u8>) {
    let input = include_str!("../input/20.txt");

    let pattern = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1_u8 } else { 0 })
        .collect::<Vec<_>>();

    let size = input.lines().nth(2).unwrap().chars().count();
    let mut map = Map2D::new(size.try_into().unwrap(), size.try_into().unwrap());

    for (y, line) in input.lines().skip(2).enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                Point2D::new(y.try_into().unwrap(), x.try_into().unwrap()),
                if c == '#' { 1 } else { 0 },
            );
        }
    }

    (map, pattern)
}

pub fn solve_1() -> usize {
    let (mut map, pattern) = read_input();
    for k in 0..2 {
        map.apply_pattern(&pattern, k & 1);
    }
    map.count_lights()
}

pub fn solve_2() -> usize {
    let (mut map, pattern) = read_input();
    for k in 0..50 {
        map.apply_pattern(&pattern, k & 1);
    }
    map.count_lights()
}
