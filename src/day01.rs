fn solve(v: &[i64]) -> i64 {
    let mut count = 0;
    for i in 1..v.len() {
        if v[i] > v[i - 1] {
            count += 1;
        }
    }
    count
}

pub fn solve_1() -> i64 {
    let v = include_str!("../input/01.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    solve(&v)
}

pub fn solve_2() -> i64 {
    let v = include_str!("../input/01.txt")
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut w = Vec::new();
    for i in 0..v.len() {
        if i + 2 < v.len() {
            w.push(v[i] + v[i + 1] + v[i + 2]);
        }
    }
    solve(&w)
}
