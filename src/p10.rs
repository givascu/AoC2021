use std::error::Error;

fn get_score(c: char) -> i64 {
    match c {
        '(' | ')' => 3,
        '[' | ']' => 57,
        '{' | '}' => 1197,
        '<' | '>' => 25137,
        _ => 0,
    }
}

fn is_opening(c: char) -> bool {
    matches!(c, '(' | '[' | '{' | '<')
}

fn is_closing(c: char) -> bool {
    matches!(c, ')' | ']' | '}' | '>')
}

fn is_matching(c1: char, c2: char) -> bool {
    match c1 {
        '(' => c2 == ')',
        '[' => c2 == ']',
        '{' => c2 == '}',
        '<' => c2 == '>',
        _ => false,
    }
}

fn line_corrupted_at(line: &str) -> Option<char> {
    let mut stack = Vec::new();

    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else if is_closing(c) {
            let k = stack.pop().unwrap();
            if !is_matching(k, c) {
                return Some(c);
            }
        }
    }

    None
}

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let lines = include_str!("../data/10.in").lines().collect::<Vec<_>>();

    let mut score = 0i64;
    for line in lines {
        if let Some(c) = line_corrupted_at(line) {
            score += get_score(c);
        }
    }

    Ok(score)
}
