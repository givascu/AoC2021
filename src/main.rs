mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;

mod utils;

fn main() {
    println!("01: {} {}", p01::solve_1(), p01::solve_2());
    println!("02: {}", p02::solve_2());
    println!("03: {} {}", p03::solve_1(), p03::solve_2());
    println!("04: {} {}", p04::solve_1(), p04::solve_2());
    println!("05: {}", p05::solve_2());
    println!(
        "06: {} {}",
        p06::solve(80).unwrap_or(-1),
        p06::solve(256).unwrap_or(-1)
    );
}
