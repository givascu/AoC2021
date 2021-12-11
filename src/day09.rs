use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

type Point = (usize, usize);

fn get_neighbors(p: Point, y_max: usize, x_max: usize) -> Vec<Point> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|&(dy, dx)| (p.0 as i32 + dy, p.1 as i32 + dx))
        .filter(|&(y, x)| (0..y_max).contains(&(y as usize)) && (0..x_max).contains(&(x as usize)))
        .map(|(y, x)| (y as usize, x as usize))
        .collect()
}

fn get_lows(map: &[String]) -> Vec<Point> {
    let mut low_points = Vec::new();
    itertools::iproduct!(0..map.len(), 0..map[0].len()).for_each(|(y, x)| {
        if get_neighbors((y, x), map.len(), map[0].len())
            .iter()
            .all(|p| map[p.0].chars().nth(p.1).unwrap() > map[y].chars().nth(x).unwrap())
        {
            low_points.push((y, x));
        }
    });
    low_points
}

fn get_basin_size(map: &[String], start: Point) -> i64 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if !visited.contains(&pos) {
            for next in get_neighbors(pos, map.len(), map[0].len())
                .iter()
                .filter(|(y, x)| {
                    map[*y].chars().nth(*x).unwrap() != '9' && !visited.contains(&(*y, *x))
                })
            {
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

    let mut basins = get_lows(&map)
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

    Ok(get_lows(&map)
        .iter()
        .map(|p| (map[p.0].chars().nth(p.1).unwrap().to_digit(10).unwrap() + 1) as i64)
        .sum())
}
