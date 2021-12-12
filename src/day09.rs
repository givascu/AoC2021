use std::collections::{HashMap, HashSet, VecDeque};

type Grid = HashMap<(i32, i32), u32>;
type Point = (i32, i32);

fn get_neighbors(p: Point, y_max: i32, x_max: i32) -> Vec<Point> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|&(dy, dx)| (p.0 + dy, p.1 + dx))
        .filter(|&(y, x)| 0 <= y && y < y_max && 0 <= x && x < x_max)
        .collect()
}

fn get_lows(map: &Grid, y_max: i32, x_max: i32) -> Vec<Point> {
    let mut low_points = Vec::new();
    itertools::iproduct!(0..y_max, 0..x_max).for_each(|p| {
        if get_neighbors(p, y_max, x_max)
            .iter()
            .all(|neigh| map.get(neigh).unwrap() > map.get(&p).unwrap())
        {
            low_points.push(p);
        }
    });
    low_points
}

fn get_basin_size(map: &Grid, y_max: i32, x_max: i32, start: Point) -> i64 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if !visited.contains(&pos) {
            for next in get_neighbors(pos, y_max, x_max)
                .iter()
                .filter(|&p| *map.get(p).unwrap() != 9 && !visited.contains(p))
            {
                queue.push_back(*next);
            }
            visited.insert(pos);
        }
    }

    visited.len() as i64
}

pub fn solve_2() -> i64 {
    let mut map = Grid::new();
    let mut y_max = 0;
    let mut x_max = 0;

    for (y, line) in include_str!("../input/09.txt").lines().enumerate() {
        for (x, c) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.entry((y as i32, x as i32)).or_insert(c);
        }
        y_max += 1;
        x_max = line.chars().count() as i32;
    }

    let mut basins = get_lows(&map, y_max, x_max)
        .iter()
        .map(|p| get_basin_size(&map, y_max, x_max, *p))
        .collect::<Vec<_>>();
    basins.sort_unstable();

    basins.iter().rev().take(3).product()
}

pub fn solve_1() -> i64 {
    let mut map = Grid::new();
    let mut y_max = 0;
    let mut x_max = 0;

    for (y, line) in include_str!("../input/09.txt").lines().enumerate() {
        for (x, c) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            map.entry((y as i32, x as i32)).or_insert(c);
        }
        y_max += 1;
        x_max = line.chars().count() as i32;
    }

    get_lows(&map, y_max, x_max)
        .iter()
        .map(|p| (map.get(p).unwrap() + 1) as i64)
        .sum()
}
