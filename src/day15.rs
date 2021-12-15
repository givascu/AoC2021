use std::{
    collections::{BTreeSet, HashMap},
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point2D {
    y: usize,
    x: usize,
}

impl Point2D {
    fn new(y: usize, x: usize) -> Point2D {
        Point2D { y, x }
    }

    fn neighbors(&self, y_max: usize, x_max: usize) -> Vec<Point2D> {
        let mut v = Vec::new();
        if self.y > 0 {
            v.push(Point2D::new(self.y - 1, self.x));
        }
        if self.x > 0 {
            v.push(Point2D::new(self.y, self.x - 1));
        }
        if self.y < y_max - 1 {
            v.push(Point2D::new(self.y + 1, self.x));
        }
        if self.x < x_max - 1 {
            v.push(Point2D::new(self.y, self.x + 1));
        }
        v
    }
}

struct Map2D<T> {
    map: HashMap<Point2D, T>,
    height: usize,
    width: usize,
}

impl<T> Deref for Map2D<T> {
    type Target = HashMap<Point2D, T>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<T> DerefMut for Map2D<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<T: std::fmt::Display> Map2D<T> {
    fn new() -> Map2D<T> {
        Map2D {
            map: HashMap::new(),
            height: 0,
            width: 0,
        }
    }

    fn _print(&self) {
        for (y, x) in itertools::iproduct!(0..self.height, 0..self.width) {
            print!("{}", self.get(&Point2D::new(y, x)).unwrap());
            if x == self.width - 1 {
                println!();
            }
        }
    }
}

fn build_input_map() -> Map2D<u32> {
    let mut map = Map2D::new();
    for (y, line) in include_str!("../input/15.txt").lines().enumerate() {
        for (x, val) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.insert(Point2D::new(y, x), val);
        }
        map.height += 1;
        map.width = line.chars().count();
    }
    map
}

fn dijkstra_shortest_paths(map: &Map2D<u32>, src: Point2D, dst: Point2D) -> u32 {
    // Keep a sorted set (priority queue) by distance to the src node.
    let mut pqueue: BTreeSet<(u32, Point2D)> = BTreeSet::new();
    let mut distance: HashMap<Point2D, u32> = map.keys().map(|p| (*p, u32::MAX)).collect();

    pqueue.insert((0, src));
    distance.entry(src).and_modify(|d| *d = 0);

    while !pqueue.is_empty() {
        let closest = *pqueue.iter().next().unwrap();
        pqueue.remove(&closest);

        let (dist_u, u) = closest;

        for v in u.neighbors(map.height, map.width) {
            let weight = *map.get(&v).unwrap();
            let dist_v = *distance.get(&v).unwrap();
            if dist_v > dist_u + weight {
                if dist_v != u32::MAX {
                    pqueue.remove(&(dist_v, v));
                }
                distance.entry(v).and_modify(|d| *d = dist_u + weight);
                pqueue.insert((dist_u + weight, v));
            }
        }
    }

    *distance.get(&dst).unwrap()
}

pub fn solve_1() -> u32 {
    let map1 = build_input_map();
    dijkstra_shortest_paths(
        &map1,
        Point2D::new(0, 0),
        Point2D::new(map1.height - 1, map1.width - 1),
    )
}

pub fn solve_2() -> u32 {
    let map = build_input_map();
    let steps = 5_usize;

    let mut big_map: Map2D<u32> = Map2D::new();
    big_map.height = map.height * steps;
    big_map.width = map.width * steps;

    for (y, x) in itertools::iproduct!(0..big_map.height, 0..big_map.width) {
        let (line, col) = (y / map.height, x / map.width);
        let delta = line + col;
        let val = *map
            .get(&Point2D::new(y % map.height, x % map.height))
            .unwrap();
        let mut new_val = val + u32::try_from(delta).unwrap();
        if new_val > 9 {
            // FIXME: Adjust this if steps > 9 (wraps aroud twice).
            new_val -= 9;
        }
        big_map.insert(Point2D::new(y, x), new_val);
    }

    dijkstra_shortest_paths(
        &big_map,
        Point2D::new(0, 0),
        Point2D::new(big_map.height - 1, big_map.width - 1),
    )
}
