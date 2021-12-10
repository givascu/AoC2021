use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

type Point = (usize, usize);

fn get_neighbor_points(map: &[String], pos: Point) -> Vec<Point> {
    let mut neighbors = Vec::new();

    if pos.0 > 0 {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if pos.0 < map.len() - 1 {
        neighbors.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        neighbors.push((pos.0, pos.1 - 1));
    }
    if pos.1 < map[0].len() - 1 {
        neighbors.push((pos.0, pos.1 + 1));
    }

    neighbors
}

fn get_low_points(map: &[String]) -> Vec<Point> {
    let mut low_points = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if get_neighbor_points(map, (y, x))
                .iter()
                .all(|p| map[p.0].chars().nth(p.1).unwrap() > map[y].chars().nth(x).unwrap())
            {
                low_points.push((y, x));
            }
        }
    }

    low_points
}

fn get_basin_size(map: &[String], start: Point) -> i64 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if !visited.contains(&pos) {
            for next in get_neighbor_points(map, pos).iter().filter(|(y, x)| {
                map[*y].chars().nth(*x).unwrap() != '9' && !visited.contains(&(*y, *x))
            }) {
                queue.push_back(*next);
            }
            visited.insert(pos);
        }
    }

    visited.len() as i64
}

pub fn solve_2() -> Result<i64, Box<dyn Error>> {
    let map = include_str!("../input/09.txt")
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let mut basins = get_low_points(&map)
        .iter()
        .map(|p| get_basin_size(&map, *p))
        .collect::<Vec<_>>();
    basins.sort_unstable();

    Ok(basins.iter().rev().take(3).product())
}

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let map = include_str!("../input/09.txt")
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    Ok(get_low_points(&map)
        .iter()
        .map(|p| (map[p.0].chars().nth(p.1).unwrap().to_digit(10).unwrap() + 1) as i64)
        .sum())
}
