use std::{fs, io::{self, BufRead, BufReader}, vec, collections::HashMap, cmp};

use crate::bingo::BingoBoard;

pub mod bingo;

fn _read_ints(filename: &str, delim: &str) -> io::Result<Vec<i32>> {
    Ok(
        fs::read_to_string(filename)?
        .split(delim).map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
    )
}

fn read_strings(filename: &str, delim: &str) -> io::Result<Vec<String>> {
    Ok(
        fs::read_to_string(filename)?
        .split(delim).map(|x| x.to_string()).collect()
    )
}

#[derive(Debug,Default)]
struct Line {
    p1: (i32, i32),
    p2: (i32, i32),
}

impl Line {
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        Line {
            p1: (x1, y1),
            p2: (x2, y2)
        }
    }
}

pub fn solve_05_1() -> i32 {
    let file = fs::File::open("data/05.in").unwrap();
    let reader = BufReader::new(file);

    let mut crossed = HashMap::new();
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split("->");
        let p1 = parts.next().unwrap().trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let p2 = parts.next().unwrap().trim().split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        lines.push(Line::new(p1[0], p1[1], p2[0], p2[1]));
    }

    for l in lines {
        if l.p1.0 == l.p2.0 {
            // X coordinate is the same
            let y_lower = cmp::min(l.p1.1, l.p2.1);
            let y_upper = cmp::max(l.p1.1, l.p2.1);
            for y in y_lower .. (y_upper + 1) {
                let p = crossed.entry((l.p1.0, y)).or_insert(0);
                *p += 1;
            }
        } else if l.p1.1 == l.p2.1 {
            // Y coordinate is the same
            let x_lower = cmp::min(l.p1.0, l.p2.0);
            let x_upper = cmp::max(l.p1.0, l.p2.0);
            for x in x_lower .. (x_upper + 1) {
                let p = crossed.entry((x, l.p1.1)).or_insert(0);
                *p += 1;
            }
        }
    }

    crossed.iter().filter(|&(_, v)| *v >= 2).count() as i32
}

fn read_bingo_input(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let file = fs::File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let marked_numbers = line
        .trim().split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut board_numbers = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if !line.is_empty() {
            for number in line.trim().split(' ').filter(|x| !x.is_empty()).map(|x| x.parse::<i32>().unwrap()) {
                board_numbers.push(number);
            }
        }
    }

    (marked_numbers, board_numbers)
}

fn build_bingo_input(filename: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let (to_mark, board_numbers) = read_bingo_input(filename);

    let mut boards = Vec::new();
    for k in (0 .. board_numbers.len()).step_by(25) {
        let mut board = BingoBoard::new();
        board.fill_in(&board_numbers[k .. k + 25]);
        boards.push(board);
    }

    (to_mark, boards)
}

pub fn solve_04_2() -> i32 {
    let (to_mark, mut boards) = build_bingo_input("data/04.in");

    let mut last_board = BingoBoard::new();
    let mut last_mark = 0;

    for mark in to_mark {
        for board in &mut boards {
            let already_won = board.has_won();
            board.mark_one(mark);
            if !already_won && board.has_won() {
                last_board = board.clone();
                last_mark = mark;
            }
        }
    }

    last_board.calculate_score(last_mark)
}

pub fn solve_04_1() -> i32 {
    let (to_mark, mut boards) = build_bingo_input("data/04.in");

    for mark in to_mark {
        for board in &mut boards {
            board.mark_one(mark);
            if board.has_won() {
                return board.calculate_score(mark);
            }
        }
    }

    0
}

fn get_bit_one_frequency(diagnosis: &[String]) -> Vec<usize> {
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

fn get_dominant_bits(diagnosis: &[String]) -> Vec<i32> {
    let n = diagnosis.len();
    let mut result = vec![0; diagnosis[0].len()];

    for (i, f) in get_bit_one_frequency(diagnosis).iter().enumerate() {
        match f.cmp(&(n - f)) {
            std::cmp::Ordering::Less => result[i] = 0,
            std::cmp::Ordering::Greater => result[i] = 1,
            std::cmp::Ordering::Equal => result[i] = 2,
        }
    }

    result
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

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for b in get_dominant_bits(&diagnosis) {
        match b {
            1 => {
                gamma.push('1');
                epsilon.push('0');
            }
            _ => {
                gamma.push('0');
                epsilon.push('1');
            }
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