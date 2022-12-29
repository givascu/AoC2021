use itertools::iproduct;

pub fn solve_1() -> u32 {
    let mut numbers = build_input_numbers();

    let mut result = numbers.remove(0);
    while !numbers.is_empty() {
        result.add(&numbers.remove(0));
        result.reduce(0);
    }

    result.magnitude(&mut 0, 1)
}

pub fn solve_2() -> u32 {
    let numbers = build_input_numbers();

    let mut maxsum = u32::MIN;
    for (i, j) in iproduct!(0..numbers.len(), 0..numbers.len()) {
        if i != j {
            let mut lhs = numbers[i].clone();
            lhs.add(&numbers[j]);
            lhs.reduce(0);
            maxsum = maxsum.max(lhs.magnitude(&mut 0, 1));
        }
    }

    maxsum
}

fn build_input_numbers() -> Vec<Number> {
    include_str!("../input/18.txt")
        .lines()
        .map(|line| {
            line.chars()
                .fold((0_u8, Number::new()), |(mut depth, mut number), c| {
                    if c == '[' {
                        depth += 1;
                    } else if c == ']' {
                        depth -= 1;
                    } else if c.is_ascii_digit() {
                        number
                            .values
                            .push((depth, u8::try_from(c.to_digit(10).unwrap()).unwrap()));
                    }
                    (depth, number)
                })
                .1
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Number {
    values: Vec<(u8, u8)>, // (depth, value)
}

impl Number {
    fn new() -> Number {
        Number { values: Vec::new() }
    }

    fn add(&mut self, other: &Number) {
        for x in &other.values {
            self.values.push(*x);
        }
        for (depth, _) in &mut self.values {
            *depth += 1;
        }
    }

    fn reduce(&mut self, idx: usize) {
        for i in idx..self.values.len() - 1 {
            // Check explode.
            if self.values[i].0 == 5 {
                let (lhs, rhs) = (self.values[i].1, self.values[i + 1].1);
                self.values[i] = (4, 0);
                self.values.remove(i + 1);
                if i > 0 {
                    self.values[i - 1].1 += lhs;
                }
                if i < self.values.len() - 1 {
                    self.values[i + 1].1 += rhs;
                }
                return self.reduce(i);
            }
        }
        for i in 0..self.values.len() {
            // Check split.
            let (depth, value) = self.values[i];
            if value > 9 {
                self.values[i] = (depth + 1, value / 2);
                self.values.insert(i + 1, (depth + 1, (value + 1) / 2));
                return self.reduce(i);
            }
        }
    }

    fn magnitude(&self, idx: &mut usize, depth: u8) -> u32 {
        let (lhs_depth, lhs_value) = self.values[*idx];
        let lhs = if lhs_depth == depth {
            *idx += 1;
            u32::from(lhs_value)
        } else {
            self.magnitude(idx, depth + 1)
        };
        let (rhs_depth, rhs_value) = self.values[*idx];
        let rhs = if rhs_depth == depth {
            *idx += 1;
            u32::from(rhs_value)
        } else {
            self.magnitude(idx, depth + 1)
        };
        lhs * 3 + rhs * 2
    }
}
