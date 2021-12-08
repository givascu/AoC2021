#[derive(Debug,Default,Clone)]
pub struct BingoBoard {
    values: Vec<i32>,
    marked: Vec<i32>,
}

impl BingoBoard {
    const SIZE: usize = 5;

    pub fn new() -> BingoBoard {
        BingoBoard {
            values: Vec::new(),
            marked: Vec::new(),
        }
    }

    pub fn fill_in(&mut self, from: &[i32]) {
        self.values.clear();
        for value in from.iter().take(BingoBoard::SIZE * BingoBoard::SIZE) {
            self.values.push(*value);
        }
        self.marked = vec![0; BingoBoard::SIZE * BingoBoard::SIZE];
    }

    pub fn mark_one(&mut self, value: i32) {
        for (i, val) in self.values.iter().enumerate() {
            if val == &value {
                self.marked[i] = 1;
            }
        }
    }

    pub fn has_won(&self) -> bool {
        // Check lines
        for k in 0 .. BingoBoard::SIZE {
            let line = &self.marked[k * BingoBoard::SIZE .. (k+1) * BingoBoard::SIZE];
            if line.iter().sum::<i32>() == BingoBoard::SIZE as i32 {
                return true
            }
        }
        // Check columns
        for k in 0 .. BingoBoard::SIZE {
            let mut sum = 0;
            for idx in (k .. BingoBoard::SIZE * BingoBoard::SIZE).step_by(BingoBoard::SIZE) {
                sum += self.marked[idx];
            }
            if sum == BingoBoard::SIZE as i32 {
                return true
            }
        }
        false
    }

    pub fn calculate_score(&self, multiplier: i32) -> i32 {
        let mut sum = 0;
        for (i, &mark) in self.marked.iter().enumerate() {
            if mark == 0 {
                sum += self.values[i];
            }
        }
        sum * multiplier
    }
}
