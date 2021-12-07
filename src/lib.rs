use std::{fs, io::{self, BufRead, BufReader}};

pub fn read_ints(filename: &str, delim: &str) -> io::Result<Vec<i32>> {
    Ok(fs::read_to_string(filename)?
        .split(delim).map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    )
}

pub fn solve_02_2(filename: &str) -> i32 {
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

pub fn solve_01_2(v: &[i32]) -> Option<i32> {
    let mut w = Vec::new();
    for i in 0..v.len() {
        if i + 2 < v.len() {
            w.push(v[i] + v[i+1] + v[i+2]);
        }
    }
    solve_01_1(&w)
}

pub fn solve_01_1(v: &[i32]) -> Option<i32> {
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