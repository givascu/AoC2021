use std::error::Error;

mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;

mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    println!("01: {} {}", p01::solve_1()?, p01::solve_2()?);
    println!("02: {}", p02::solve_2()?);
    println!("03: {} {}", p03::solve_1()?, p03::solve_2()?);
    println!("04: {} {}", p04::solve_1()?, p04::solve_2()?);
    println!("05: {}", p05::solve_2()?);
    println!("06: {} {}", p06::solve(80)?, p06::solve(256)?);
    println!("07: {} {}", p07::solve_1()?, p07::solve_2()?);

    Ok(())
}
