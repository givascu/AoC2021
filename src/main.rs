use std::{fs::{self}, io::{self, BufRead, BufReader}};

fn read_ints(filename: &str) -> io::Result<Vec<i32>> {
    Ok(fs::read_to_string(filename)?
        .split('\n').map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    )
}

fn main() {
    let v = read_ints("01.in").unwrap();
    println!("{}", _solve_01_1(&v).unwrap());
    println!("{}", _solve_01_2(&v).unwrap());

    println!("{}", _solve_02_1("02.in"));
    println!("{}", _solve_02_2("02.in"));
}

fn _solve_02_2(filename: &str) -> i32 {
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        if direction == "forward" {
            horiz += value;
            depth += aim * value;
        } else if direction == "down" {
            aim += value;
        } else if direction == "up" {
            aim -= value;
        }
    }

    horiz * depth
}

fn _solve_02_1(filename: &str) -> i32 {
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut horiz = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        if direction == "forward" {
            horiz += value;
        } else if direction == "down" {
            depth += value;
        } else if direction == "up" {
            depth -= value;
        }
    }

    horiz * depth
}

fn _solve_01_2(v: &[i32]) -> Option<i32> {
    let mut w = Vec::new();
    for i in 0..v.len() {
        if i + 2 < v.len() {
            w.push(v[i] + v[i+1] + v[i+2]);
        }
    }
    _solve_01_1(&w)
}

fn _solve_01_1(v: &[i32]) -> Option<i32> {
    if v.len() <= 1 {
        return None;
    }

    let mut count = 0;
    for i in 1..v.len() {
        if v[i] > v[i-1] {
            count += 1;
        }
    }

    Some(count)
}
