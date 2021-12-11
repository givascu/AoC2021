use std::collections::HashSet;

type Grid = Vec<Vec<u32>>;
type Point = (usize, usize); // (y, x)

fn get_neighbors(p: Point, size: usize) -> Vec<Point> {
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
    .map(|&(dy, dx)| (p.0 as i32 + dy, p.1 as i32 + dx))
    .filter(|&(y, x)| (0..size).contains(&(y as usize)) && (0..size).contains(&(x as usize)))
    .map(|(y, x)| (y as usize, x as usize))
    .collect()
}

fn advance_grid(grid: &mut Grid) -> i64 {
    // Increase all by 1.
    *grid = grid
        .iter()
        .map(|line| line.iter().map(|val| *val + 1).collect())
        .collect();

    let mut flashed = HashSet::new();
    loop {
        let overcharged = itertools::iproduct!(0..grid.len(), 0..grid.len())
            .filter(|&(y, x)| grid[y][x] > 9)
            .collect::<Vec<Point>>();
        if overcharged.is_empty() {
            break;
        }

        overcharged.iter().for_each(|&p| {
            if !flashed.contains(&p) {
                grid[p.0][p.1] = 0;

                get_neighbors(p, grid.len())
                    .iter()
                    .filter(|&neigh| !flashed.contains(neigh))
                    .for_each(|&(y, x)| grid[y][x] += 1);

                flashed.insert(p);
            }
        });
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

    for k in 1..i64::MAX {
        if advance_grid(&mut grid) == (grid.len() * grid.len()) as i64 {
            return k;
        }
    }

    -1
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

    (0..100).fold(0, |acc, _| acc + advance_grid(&mut grid))
}
