use std::{fs, io};

pub fn read_ints(filename: &str, delim: &str) -> io::Result<Vec<i32>> {
    Ok(
        fs::read_to_string(filename)?
        .split(delim).map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    )
}

pub fn read_strings(filename: &str, delim: &str) -> io::Result<Vec<String>> {
    Ok(
        fs::read_to_string(filename)?
        .split(delim).map(|x| x.to_string()).collect()
    )
}
