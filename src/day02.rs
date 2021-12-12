pub fn solve_2() -> i64 {
    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    include_str!("../input/02.txt").lines().for_each(|line| {
        let (dir, val) = line.split_once(' ').unwrap();
        let val = val.parse::<i64>().unwrap();

        if dir == "forward" {
            horiz += val;
            depth += aim * val;
        } else if dir == "down" {
            aim += val;
        } else if dir == "up" {
            aim -= val;
        }
    });

    horiz * depth
}
