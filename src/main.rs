use std::{env, error::Error};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("01: {} {}", day01::solve_1()?, day01::solve_2()?);
            println!("02: {}", day02::solve_2()?);
            println!("03: {} {}", day03::solve_1()?, day03::solve_2()?);
            println!("04: {} {}", day04::solve_1()?, day04::solve_2()?);
            println!("05: {}", day05::solve_2()?);
            println!("06: {} {}", day06::solve(80)?, day06::solve(256)?);
            println!("07: {} {}", day07::solve_1()?, day07::solve_2()?);
            println!("08: {} {}", day08::solve_1()?, day08::solve_2()?);
            println!("09: {} {}", day09::solve_1()?, day09::solve_2()?);
            println!("10: {} {}", day10::solve_1()?, day10::solve_2()?);
            println!("11: {} {}", day11::solve_1(), day11::solve_2());
        }
        _ => match args[1].as_str() {
            "1" => println!("{} {}", day01::solve_1()?, day01::solve_2()?),
            "2" => println!("{}", day02::solve_2()?),
            "3" => println!("{} {}", day03::solve_1()?, day03::solve_2()?),
            "4" => println!("{} {}", day04::solve_1()?, day04::solve_2()?),
            "5" => println!("{}", day05::solve_2()?),
            "6" => println!("{} {}", day06::solve(80)?, day06::solve(256)?),
            "7" => println!("{} {}", day07::solve_1()?, day07::solve_2()?),
            "8" => println!("{} {}", day08::solve_1()?, day08::solve_2()?),
            "9" => println!("{} {}", day09::solve_1()?, day09::solve_2()?),
            "10" => println!("{} {}", day10::solve_1()?, day10::solve_2()?),
            "11" => println!("{} {}", day11::solve_1(), day11::solve_2()),
            _ => return Err(format!("Invalid day: '{}'", args[1]).into()),
        },
    }
    Ok(())
}
