use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader},
};

pub fn solve_2() -> Result<i64, Box<dyn Error>> {
    let file = fs::File::open("data/02.in")?;
    let reader = BufReader::new(file);

    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line?;
        let (direction, value) = line.split_once(" ").ok_or("split_once()")?;
        let value = value.parse::<i64>()?;

        if direction == "forward" {
            horiz += value;
            depth += aim * value;
        } else if direction == "down" {
            aim += value;
        } else if direction == "up" {
            aim -= value;
        }
    }

    Ok(horiz * depth)
}
