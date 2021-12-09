use std::{cmp, error::Error};

use crate::utils;

pub fn solve_2() -> Result<i64, Box<dyn Error>> {
    let positions = utils::read_ints("data/07.in", ",")?;

    let mut best = i64::MAX;

    for (i, p1) in positions.iter().enumerate() {
        let mut curr = 0i64;
        for (j, p2) in positions.iter().enumerate() {
            if i != j {
                let dist = (p2 - p1).abs();
                curr += dist * (dist + 1) / 2;
            }
        }
        best = cmp::min(best, curr);
    }

    Ok(best)
}

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let positions = utils::read_ints("data/07.in", ",")?;

    let mut best = i64::MAX;

    for (i, p1) in positions.iter().enumerate() {
        let mut curr = 0i64;
        for (j, p2) in positions.iter().enumerate() {
            if i != j {
                curr += (p2 - p1).abs();
            }
        }
        best = cmp::min(best, curr);
    }

    Ok(best)
}
