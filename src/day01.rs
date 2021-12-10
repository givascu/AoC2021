use std::error::Error;

use crate::utils;

fn solve(v: &[i64]) -> i64 {
    let mut count = 0;
    for i in 1..v.len() {
        if v[i] > v[i - 1] {
            count += 1;
        }
    }
    count
}

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let v = utils::read_ints("input/01.txt", "\n")?;
    Ok(solve(&v))
}

pub fn solve_2() -> Result<i64, Box<dyn Error>> {
    let v = utils::read_ints("input/01.txt", "\n")?;
    let mut w = Vec::new();
    for i in 0..v.len() {
        if i + 2 < v.len() {
            w.push(v[i] + v[i + 1] + v[i + 2]);
        }
    }
    Ok(solve(&w))
}
