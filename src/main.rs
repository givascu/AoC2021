use std::{env, error::Error};

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;

mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("01: {} {}", p01::solve_1()?, p01::solve_2()?);
            println!("02: {}", p02::solve_2()?);
            println!("03: {} {}", p03::solve_1()?, p03::solve_2()?);
            println!("04: {} {}", p04::solve_1()?, p04::solve_2()?);
            println!("05: {}", p05::solve_2()?);
            println!("06: {} {}", p06::solve(80)?, p06::solve(256)?);
            println!("07: {} {}", p07::solve_1()?, p07::solve_2()?);
            println!("08: {} {}", p08::solve_1()?, p08::solve_2()?);
            println!("09: {} {}", p09::solve_1()?, p09::solve_2()?);
            println!("10: {} {}", p10::solve_1()?, p10::solve_2()?);
        }
        _ => match args[1].as_str() {
            "1" => println!("{} {}", p01::solve_1()?, p01::solve_2()?),
            "2" => println!("{}", p02::solve_2()?),
            "3" => println!("{} {}", p03::solve_1()?, p03::solve_2()?),
            "4" => println!("{} {}", p04::solve_1()?, p04::solve_2()?),
            "5" => println!("{}", p05::solve_2()?),
            "6" => println!("{} {}", p06::solve(80)?, p06::solve(256)?),
            "7" => println!("{} {}", p07::solve_1()?, p07::solve_2()?),
            "8" => println!("{} {}", p08::solve_1()?, p08::solve_2()?),
            "9" => println!("{} {}", p09::solve_1()?, p09::solve_2()?),
            "10" => println!("{} {}", p10::solve_1()?, p10::solve_2()?),
            _ => return Err(format!("Invalid day: '{}'", args[1]).into()),
        },
    }
    Ok(())
}
