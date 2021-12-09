use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader},
};

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let file = fs::File::open("data/08.in")?;
    let reader = BufReader::new(file);

    let mut counter = 0i64;
    let basic_digits_lengths = vec![2usize, 3, 4, 7];

    for line in reader.lines() {
        let line = line?;
        let (_, digits) = line.split_once('|').ok_or("split_once()")?;
        let digits = digits
            .trim()
            .split(' ')
            .filter(|x| basic_digits_lengths.contains(&x.len()))
            .count();
        counter += digits as i64;
    }

    Ok(counter)
}
