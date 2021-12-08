use crate::utils;

fn solve(v: &[i32]) -> i32 {
    let mut count = 0;
    for i in 1 .. v.len() {
        if v[i] > v[i-1] {
            count += 1;
        }
    }
    count
}

pub fn solve_1() -> i32 {
    let v = utils::read_ints("data/01.in", "\n").unwrap();
    solve(&v)
}

pub fn solve_2() -> i32 {
    let v = utils::read_ints("data/01.in", "\n").unwrap();
    let mut w = Vec::new();
    for i in 0 .. v.len() {
        if i + 2 < v.len() {
            w.push(v[i] + v[i+1] + v[i+2]);
        }
    }
    solve(&w)
}
