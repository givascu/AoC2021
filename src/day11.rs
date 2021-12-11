use std::collections::HashSet;

type Grid = Vec<Vec<u32>>;
type Point = (usize, usize); // (y, x)

fn get_neighbors(p: Point, size: usize) -> Vec<Point> {
    let mut neighbors = Vec::new();

    if p.0 > 0 {
        neighbors.push((p.0 - 1, p.1)); // -1, 0
    }
    if p.0 < size - 1 {
        neighbors.push((p.0 + 1, p.1)); // +1, 0
    }
    if p.1 > 0 {
        neighbors.push((p.0, p.1 - 1)); // 0, -1
    }
    if p.1 < size - 1 {
        neighbors.push((p.0, p.1 + 1)); // 0, +1
    }
    if p.0 > 0 && p.1 > 0 {
        neighbors.push((p.0 - 1, p.1 - 1)); // -1, -1
    }
    if p.0 > 0 && p.1 < size - 1 {
        neighbors.push((p.0 - 1, p.1 + 1)); // -1, +1
    }
    if p.0 < size - 1 && p.1 > 0 {
        neighbors.push((p.0 + 1, p.1 - 1)); // +1, -1
    }
    if p.0 < size - 1 && p.1 < size - 1 {
        neighbors.push((p.0 + 1, p.1 + 1)); // +1, +1
    }

    neighbors
}

fn get_max_caps(grid: &[Vec<u32>]) -> Vec<Point> {
    let mut v = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x] > 9 {
                v.push((y, x));
            }
        }
    }
    v
}

fn advance_grid(grid: &mut Grid) -> i64 {
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            grid[y][x] += 1;
        }
    }

    let mut flashed = HashSet::new();
    loop {
        let maxcap = get_max_caps(grid);
        if maxcap.is_empty() {
            break;
        }
        for p in maxcap {
            if !flashed.contains(&p) {
                grid[p.0][p.1] = 0;
                for (y, x) in get_neighbors(p, grid.len())
                    .into_iter()
                    .filter(|neigh| !flashed.contains(neigh))
                {
                    grid[y][x] += 1;
                }
                flashed.insert(p);
            }
        }
    }

    flashed.len() as i64
}

pub fn solve_2() -> i64 {
    let mut grid = include_str!("../input/11.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Grid>();

    let mut k = 0i64;
    loop {
        k += 1;
        if advance_grid(&mut grid) == (grid.len() * grid.len()) as i64 {
            return k;
        }
    }
}

pub fn solve_1() -> i64 {
    let mut grid = include_str!("../input/11.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Grid>();

    let mut counter = 0;
    for _ in 0..100 {
        counter += advance_grid(&mut grid);
    }

    counter
}
