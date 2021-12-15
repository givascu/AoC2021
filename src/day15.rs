use std::collections::{BTreeSet, HashMap};

type Point = (i64, i64);

#[derive(Debug, Default)]
struct Map {
    grid: HashMap<Point, u32>,
    height: i64,
    width: i64,
}

impl Map {
    fn new() -> Map {
        Map {
            grid: HashMap::new(),
            height: 0,
            width: 0,
        }
    }

    fn _print(&self) {
        itertools::iproduct!(0..self.height, 0..self.width).for_each(|(y, x)| {
            print!("{}", self.grid.get(&(y, x)).unwrap());
            if x == self.width - 1 {
                println!();
            }
        });
    }
}

fn get_neighbors(p: Point, p_max: Point) -> Vec<Point> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|&(dy, dx)| (p.0 + dy, p.1 + dx))
        .filter(|&(y, x)| 0 <= y && y < p_max.0 && 0 <= x && x < p_max.1)
        .collect()
}

fn get_input_map() -> Map {
    let mut map = Map::new();
    for (y, line) in include_str!("../input/15.txt").lines().enumerate() {
        for (x, c) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.grid.insert((y as i64, x as i64), c);
        }
        map.height += 1;
        map.width = line.chars().count() as i64;
    }
    map
}

fn dijkstra_shortest_paths(map: &Map, src: Point, dst: Point) -> u32 {
    // Keep a sorted set (priority queue) by distance to the src node.
    let mut pqueue: BTreeSet<(u32, Point)> = BTreeSet::new();
    let mut distance = map
        .grid
        .keys()
        .map(|p| (*p, u32::MAX))
        .collect::<HashMap<_, _>>();

    pqueue.insert((0, src));
    distance.entry(src).and_modify(|d| *d = 0);

    while !pqueue.is_empty() {
        let closest = *pqueue.iter().next().unwrap();
        pqueue.remove(&closest);

        let (dist_u, u) = closest;

        for v in get_neighbors(u, (map.height, map.width)) {
            let weight = *map.grid.get(&v).unwrap();
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
    let map = get_input_map();
    dijkstra_shortest_paths(&map, (0, 0), (map.height - 1, map.width - 1))
}

pub fn solve_2() -> u32 {
    let map = get_input_map();
    let steps = 5_i64;

    let mut big_map = Map::new();
    big_map.height = map.height * steps;
    big_map.width = map.width * steps;

    for y in 0..big_map.height {
        let line = y / map.height;
        for x in 0..big_map.width {
            let col = x / map.width;
            let delta = line + col;
            let val = *map.grid.get(&(y % map.height, x % map.width)).unwrap();
            let mut new_val = val + u32::try_from(delta).unwrap();
            if new_val > 9 {
                // FIXME: Adjust this if steps > 9 (wraps aroud twice).
                new_val -= 9;
            }
            big_map.grid.insert((y, x), new_val);
        }
    }

    dijkstra_shortest_paths(&big_map, (0, 0), (big_map.height - 1, big_map.width - 1))
}
