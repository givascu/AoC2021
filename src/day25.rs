use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
};

#[derive(Hash, Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Point2D {
    y: usize,
    x: usize,
}

impl Point2D {
    fn new(y: usize, x: usize) -> Point2D {
        Point2D { y, x }
    }

    fn right(&self, width: usize) -> Point2D {
        if self.x == width - 1 {
            Point2D::new(self.y, 0)
        } else {
            Point2D::new(self.y, self.x + 1)
        }
    }

    fn down(&self, height: usize) -> Point2D {
        if self.y == height - 1 {
            Point2D::new(0, self.x)
        } else {
            Point2D::new(self.y + 1, self.x)
        }
    }
}

struct Map2D {
    map: HashMap<Point2D, char>,
    height: usize,
    width: usize,
}

impl Deref for Map2D {
    type Target = HashMap<Point2D, char>;

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
    fn new(height: usize, width: usize) -> Map2D {
        Map2D {
            map: HashMap::new(),
            height,
            width,
        }
    }

    fn run_iteration(&mut self) -> usize {
        let mut moves = 0;

        let map_orig = self.map.clone();
        for (y, x) in itertools::iproduct!(0..self.height, (0..self.width).rev()) {
            let p = Point2D::new(y, x);
            if *map_orig.get(&p).unwrap() == '>' {
                let q = p.right(self.width);
                if *map_orig.get(&q).unwrap() == '.' {
                    self.entry(p).and_modify(|c| *c = '.');
                    self.entry(q).and_modify(|c| *c = '>');
                    moves += 1;
                }
            }
        }

        let map_orig = self.map.clone();
        for (y, x) in itertools::iproduct!((0..self.height).rev(), 0..self.width) {
            let p = Point2D::new(y, x);
            if *map_orig.get(&p).unwrap() == 'v' {
                let q = p.down(self.height);
                if *map_orig.get(&q).unwrap() == '.' {
                    self.entry(p).and_modify(|c| *c = '.');
                    self.entry(q).and_modify(|c| *c = 'v');
                    moves += 1;
                }
            }
        }

        moves
    }

    fn _print(&self) {
        for (y, x) in itertools::iproduct!(0..self.height, 0..self.width) {
            print!("{}", self.map.get(&Point2D::new(y, x)).unwrap());
            if x == self.width - 1 {
                println!();
            }
        }
        println!();
    }
}

fn read_input() -> Map2D {
    let input = include_str!("../input/25.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut map = Map2D::new(height, width);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(Point2D::new(y, x), c);
        }
    }
    map
}

pub fn solve_1() -> u64 {
    let mut map = read_input();

    for k in 1..=u64::MAX {
        let moves = map.run_iteration();
        if moves == 0 {
            return k;
        }
    }

    0
}
