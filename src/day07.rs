use std::cmp;

pub fn solve_2() -> i64 {
    let positions = include_str!("../input/07.txt")
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut best = i64::MAX;

    for (i, p1) in positions.iter().enumerate() {
        let mut curr = 0_i64;
        for (j, p2) in positions.iter().enumerate() {
            if i != j {
                let dist = (p2 - p1).abs();
                curr += dist * (dist + 1) / 2;
            }
        }
        best = cmp::min(best, curr);
    }

    best
}

pub fn solve_1() -> i64 {
    let positions = include_str!("../input/07.txt")
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut best = i64::MAX;

    for (i, p1) in positions.iter().enumerate() {
        let mut curr = 0_i64;
        for (j, p2) in positions.iter().enumerate() {
            if i != j {
                curr += (p2 - p1).abs();
            }
        }
        best = cmp::min(best, curr);
    }

    best
}
