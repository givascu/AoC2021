use std::{fs, io::{self, BufRead, BufReader}, vec};

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

fn get_bit_one_frequency(diagnosis: &Vec<String>) -> Vec<usize> {
    let mut frequency = vec![0; diagnosis[0].len()];
    for d in diagnosis {
        for (i, c) in d.chars().enumerate() {
            if c == '1' {
                frequency[i] += 1;
            }
        }
    }
    frequency
}

fn get_dominant_bits(diagnosis: &Vec<String>) -> Vec<i32> {
    let frequency = get_bit_one_frequency(&diagnosis);
    let mut dominant_bits = vec![0; frequency.len()];
    for (i, f) in frequency.iter().enumerate() {
        // 0 for bit 0 most common
        // 1 for bit 1 most common
        // 2 for equal number of bits 0 and bits 1
        if f > &(diagnosis.len() - f) {
            dominant_bits[i] = 1;
        } else if f < &(diagnosis.len() - f) {
            dominant_bits[i] = 0;
        } else {
            dominant_bits[i] = 2;
        }
    }
    dominant_bits
}

pub fn solve_03_2() -> i32 {
    let diagnosis = read_strings("data/03.in", "\n").unwrap();

    let mut ogr = diagnosis.clone(); // oxygen generator rating
    let mut csr = diagnosis.clone(); // co2 scrubber rating

    for i in 0 .. diagnosis[0].len() {
        if ogr.len() == 1 && csr.len() == 1 {
            break;
        }

        if ogr.len() > 1 {
            let ogr_bits = get_dominant_bits(&ogr);
            let ogr_bit = if ogr_bits[i] == 0 { '0' } else { '1' };
            let mut ogr_copy = Vec::new();
            for item in &ogr {
                if item.chars().nth(i).unwrap() == ogr_bit {
                    ogr_copy.push(item.clone());
                }
            }
            ogr = ogr_copy;
        }

        if csr.len() > 1 {
            let csr_bits = get_dominant_bits(&csr);
            let csr_bit = if csr_bits[i] == 0 { '1' } else { '0' };
            let mut csr_copy = Vec::new();

            for item in &csr {
                if item.chars().nth(i).unwrap() == csr_bit {
                    csr_copy.push(item.clone());
                }
            }
            csr = csr_copy;
        }
    }

    let ogr = i32::from_str_radix(&ogr[0], 2).unwrap();
    let csr = i32::from_str_radix(&csr[0], 2).unwrap();

    ogr * csr
}

pub fn solve_03_1() -> i32 {
    let diagnosis = read_strings("data/03.in", "\n").unwrap();
    let frequency = get_bit_one_frequency(&diagnosis);

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for f in frequency {
        if 2 * f > diagnosis.len() {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

pub fn solve_02_2() -> i32 {
    let file = fs::File::open("data/02.in").unwrap();
    let reader = BufReader::new(file);

    let mut horiz = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split(' ');
        let direction = parts.next().unwrap();
        let value = parts.next().unwrap().parse::<i32>().unwrap();

        if direction == "forward" {
            horiz += value;
            depth += aim * value;
        } else if direction == "down" {
            aim += value;
        } else if direction == "up" {
            aim -= value;
        }
    }

    horiz * depth
}

pub fn solve_01_2(v: &[i32]) -> Option<i32> {
    let mut w = Vec::new();
    for i in 0 .. v.len() {
        if i + 2 < v.len() {
            w.push(v[i] + v[i+1] + v[i+2]);
        }
    }
    solve_01_1(&w)
}

pub fn solve_01_1(v: &[i32]) -> Option<i32> {
    if v.len() <= 1 {
        return None;
    }

    let mut count = 0;
    for i in 1 .. v.len() {
        if v[i] > v[i-1] {
            count += 1;
        }
    }

    Some(count)
}