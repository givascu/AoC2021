use std::{cmp, collections::HashSet};

#[derive(Debug, Default)]
struct Paper {
    grid: HashSet<(u32, u32)>,
    y_max: u32,
    x_max: u32,
}

impl Paper {
    fn new() -> Paper {
        Paper {
            grid: HashSet::new(),
            y_max: (0),
            x_max: (0),
        }
    }

    fn print(&self) {
        itertools::iproduct!(0..self.y_max, 0..self.x_max).for_each(|(y, x)| {
            if self.grid.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
            if x == self.x_max - 1 {
                println!();
            }
        });
    }

    fn fold_y(&mut self, y_fold: u32) {
        itertools::iproduct!(y_fold + 1..self.y_max, 0..self.x_max).for_each(|(y, x)| {
            if self.grid.contains(&(y, x)) {
                self.grid.insert((self.y_max - 1 - y, x));
                self.grid.remove(&(y, x));
            }
        });
        self.y_max = y_fold;
    }

    fn fold_x(&mut self, x_fold: u32) {
        itertools::iproduct!(0..self.y_max, x_fold + 1..self.x_max).for_each(|(y, x)| {
            if self.grid.contains(&(y, x)) {
                self.grid.insert((y, self.x_max - 1 - x));
                self.grid.remove(&(y, x));
            }
        });
        self.x_max = x_fold;
    }
}

fn read_paper_input() -> Paper {
    let mut paper = Paper::new();

    for line in include_str!("../input/13.txt").lines() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let (x, y) = line
            .split_once(',')
            .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
            .unwrap();

        paper.y_max = cmp::max(y + 1, paper.y_max);
        paper.x_max = cmp::max(x + 1, paper.x_max);
        paper.grid.insert((y, x));
    }

    paper
}

pub fn solve_2() -> i64 {
    let mut paper = read_paper_input();

    include_str!("../input/13.txt")
        .lines()
        .filter(|&line| line.contains("fold along"))
        .for_each(|line| {
            let (mode, val) = line.split_once('=').unwrap();
            let val = val.parse::<u32>().unwrap();

            if mode == "fold along y" {
                paper.fold_y(val);
            } else if mode == "fold along x" {
                paper.fold_x(val);
            }
        });

    paper.print();

    -1
}

pub fn solve_1() -> i64 {
    let mut paper = read_paper_input();

    let first_fold = include_str!("../input/13.txt")
        .lines()
        .find(|&line| line.contains("fold along"))
        .unwrap();
    let (mode, val) = first_fold.split_once('=').unwrap();
    let val = val.parse::<u32>().unwrap();

    if mode == "fold along y" {
        paper.fold_y(val);
    } else if mode == "fold along x" {
        paper.fold_x(val);
    }

    paper.grid.len() as i64
}
