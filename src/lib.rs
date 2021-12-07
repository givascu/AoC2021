use std::{fs, io::{self, BufRead, BufReader}};

pub fn read_ints(filename: &str, delim: &str) -> io::Result<Vec<i32>> {
    Ok(
        fs::read_to_string(filename)?
        .split(delim).map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    )
}

pub fn read_strings(filename: &str, delim: &str) -> io::Result<Vec<String>> {
    Ok(
        fs::read_to_string(filename)?
        .split(delim).map(|x| x.to_string()).collect()
    )
}

pub fn solve_03_1() -> i32 {
    let diagnosis = read_strings("data/03.in", "\n").unwrap();
    let mut frequency = vec![0; diagnosis[0].len()];

    for d in &diagnosis {
        for (i, c) in d.chars().enumerate() {
            if c == '1' {
                frequency[i] += 1;
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for f in frequency {
        if 2 * f > diagnosis.len() {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_dec = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_dec = i32::from_str_radix(&epsilon, 2).unwrap();

    gamma_dec * epsilon_dec
}

pub fn solve_02_2() -> i32 {
    let file = fs::File::open("data/02.in").unwrap();
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
    for i in 0 .. v.len() {
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
    for i in 1 .. v.len() {
        if v[i] > v[i-1] {
            count += 1;
        }
    }

    Some(count)
}