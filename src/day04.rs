use std::{
    error::Error,
    fs,
    io::{BufRead, BufReader},
};

#[derive(Debug, Default, Clone)]
pub struct Board {
    values: Vec<i64>,
    marked: Vec<i64>,
}

impl Board {
    const SIZE: usize = 5;

    pub fn new() -> Board {
        Board {
            values: Vec::new(),
            marked: Vec::new(),
        }
    }

    pub fn fill_in(&mut self, from: &[i64]) {
        self.values.clear();
        for value in from.iter().take(Board::SIZE * Board::SIZE) {
            self.values.push(*value);
        }
        self.marked = vec![0; Board::SIZE * Board::SIZE];
    }

    pub fn mark_one(&mut self, value: i64) {
        for (i, val) in self.values.iter().enumerate() {
            if val == &value {
                self.marked[i] = 1;
            }
        }
    }

    pub fn has_won(&self) -> bool {
        // Check lines
        for k in 0..Board::SIZE {
            let line = &self.marked[k * Board::SIZE..(k + 1) * Board::SIZE];
            if line.iter().sum::<i64>() == Board::SIZE as i64 {
                return true;
            }
        }
        // Check columns
        for k in 0..Board::SIZE {
            let mut sum = 0;
            for idx in (k..Board::SIZE * Board::SIZE).step_by(Board::SIZE) {
                sum += self.marked[idx];
            }
            if sum == Board::SIZE as i64 {
                return true;
            }
        }
        false
    }

    pub fn calculate_score(&self, multiplier: i64) -> i64 {
        let mut sum = 0;
        for (i, &mark) in self.marked.iter().enumerate() {
            if mark == 0 {
                sum += self.values[i];
            }
        }
        sum * multiplier
    }
}

fn read_bingo_input(filename: &str) -> Result<(Vec<i64>, Vec<i64>), Box<dyn Error>> {
    let file = fs::File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let marked_numbers = line
        .trim()
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut board_numbers = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            for number in line
                .trim()
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i64>().unwrap())
            {
                board_numbers.push(number);
            }
        }
    }

    Ok((marked_numbers, board_numbers))
}

fn build_bingo_input(filename: &str) -> Result<(Vec<i64>, Vec<Board>), Box<dyn Error>> {
    let (to_mark, board_numbers) = read_bingo_input(filename)?;

    let mut boards = Vec::new();
    for k in (0..board_numbers.len()).step_by(25) {
        let mut board = Board::new();
        board.fill_in(&board_numbers[k..k + 25]);
        boards.push(board);
    }

    Ok((to_mark, boards))
}

pub fn solve_2() -> Result<i64, Box<dyn Error>> {
    let (to_mark, mut boards) = build_bingo_input("input/04.txt")?;

    let mut last_board = Board::new();
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

    Ok(last_board.calculate_score(last_mark))
}

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let (to_mark, mut boards) = build_bingo_input("input/04.txt")?;

    for mark in to_mark {
        for board in &mut boards {
            board.mark_one(mark);
            if board.has_won() {
                return Ok(board.calculate_score(mark));
            }
        }
    }

    Err("Could not find last board to win!".into())
}
