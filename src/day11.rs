use std::collections::{HashMap, HashSet};

type Grid = HashMap<(i64, i64), u32>;
type Point = (i64, i64);

fn get_neighbors(p: Point, y_max: i64, x_max: i64) -> Vec<Point> {
    [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ]
    .iter()
    .map(|&(dy, dx)| (p.0 + dy, p.1 + dx))
    .filter(|&(y, x)| 0 <= y && y < y_max && 0 <= x && x < x_max)
    .collect()
}

fn advance_grid(grid: &mut Grid, y_max: i64, x_max: i64) -> i64 {
    grid.iter_mut().for_each(|(_, v)| *v += 1);

    let mut flashed = HashSet::new();
    loop {
        let overcharged = itertools::iproduct!(0..y_max, 0..x_max)
            .filter(|p| *grid.get(p).unwrap() > 9)
            .collect::<Vec<Point>>();
        if overcharged.is_empty() {
            break;
        }

        for p in overcharged {
            if !flashed.contains(&p) {
                grid.entry(p).and_modify(|v| *v = 0);

                get_neighbors(p, y_max, x_max)
                    .iter()
                    .filter(|&p| !flashed.contains(p))
                    .for_each(|p| {
                        grid.entry(*p).and_modify(|v| *v += 1);
                    });

                flashed.insert(p);
            }
        }
    }

    flashed.len() as i64
}

pub fn solve_2() -> i64 {
    let mut grid = Grid::new();
    let mut y_max = 0;
    let mut x_max = 0;

    for (y, line) in include_str!("../input/11.txt").lines().enumerate() {
        for (x, c) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            grid.entry((y as i64, x as i64)).or_insert(c);
        }
        y_max += 1;
        x_max = line.chars().count() as i64;
    }

    for k in 1..i64::MAX {
        if advance_grid(&mut grid, y_max, x_max) == grid.len() as i64 {
            return k;
        }
    }

    0
}

pub fn solve_1() -> i64 {
    let mut grid = Grid::new();
    let mut y_max = 0;
    let mut x_max = 0;

    for (y, line) in include_str!("../input/11.txt").lines().enumerate() {
        for (x, c) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
            grid.entry((y as i64, x as i64)).or_insert(c);
        }
        y_max += 1;
        x_max = line.chars().count() as i64;
    }

    (0..100).fold(0, |acc, _| acc + advance_grid(&mut grid, y_max, x_max))
}
