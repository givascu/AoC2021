use std::error::Error;

use crate::utils;

pub fn solve(days: i64) -> Result<i64, Box<dyn Error>> {
    let counters = utils::read_ints("data/06.in", ",")?;

    let mut frequency: Vec<i64> = vec![0; 9];
    for c in counters {
        frequency[c as usize] += 1;
    }

    for _ in 0..days {
        let spawners = frequency[0];
        for i in 1..frequency.len() {
            // After each day pass, the number of N counter moves to the N - 1 counter
            frequency[i - 1] = frequency[i];
        }
        frequency[6] += spawners; // counter 6 cycles back to counter 0
        frequency[8] = spawners; // spawnees have a starting counter of 8
    }

    Ok(frequency.iter().sum())
}
