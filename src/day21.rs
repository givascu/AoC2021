use std::collections::HashMap;

#[derive(Debug)]
struct Dice {
    value: u32,
    counter: u32,
}

impl Dice {
    fn new() -> Dice {
        Dice {
            value: 1,
            counter: 0,
        }
    }

    fn roll(&mut self) -> u32 {
        let retval = self.value;
        self.value = (self.value + 1) % 1000;
        self.counter += 1;
        retval
    }
}

#[derive(Debug, Clone)]
struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(pos: u32) -> Player {
        Player { pos, score: 0 }
    }

    fn advance(&mut self, steps: u32) {
        self.pos += steps % 10;
        if self.pos > 10 {
            self.pos %= 10;
        }
        self.score += self.pos;
    }
}

fn read_input() -> (Player, Player) {
    let positions = include_str!("../input/21.txt")
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .parse::<u32>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    assert!(positions.len() == 2);

    (Player::new(positions[0]), Player::new(positions[1]))
}

pub fn solve_1() -> u32 {
    let (mut p1, mut p2) = read_input();
    let mut dice = Dice::new();

    for round in 1..u32::MAX {
        let mut roll = 0;
        for _ in 0..3 {
            roll += dice.roll();
        }

        if round % 2 == 1 {
            p1.advance(roll);
            if p1.score >= 1000 {
                return p2.score * dice.counter;
            }
        } else {
            p2.advance(roll);
            if p2.score >= 1000 {
                return p1.score * dice.counter;
            }
        }
    }

    0
}

fn dice_rolls() -> HashMap<u32, u32> {
    itertools::iproduct!(1..=3, 1..=3, 1..=3).fold(HashMap::new(), |mut acc, (r1, r2, r3)| {
        *acc.entry(r1 + r2 + r3).or_default() += 1;
        acc
    })
}

fn count_wins(
    rolls: &HashMap<u32, u32>,
    round: u64,
    exp: u64,
    p1: &Player,
    p2: &Player,
    win1: &mut u64,
    win2: &mut u64,
) {
    if p1.score >= 21 {
        *win1 += exp;
        return;
    }
    if p2.score >= 21 {
        *win2 += exp;
        return;
    }
    for (&roll, &num) in rolls {
        let mut p1_clone = p1.clone();
        let mut p2_clone = p2.clone();
        if round % 2 == 1 {
            p1_clone.advance(roll);
        } else {
            p2_clone.advance(roll);
        }
        count_wins(
            rolls,
            round + 1,
            exp * u64::from(num),
            &p1_clone,
            &p2_clone,
            win1,
            win2,
        );
    }
}

pub fn solve_2() -> u64 {
    let (p1, p2) = read_input();

    let rolls = dice_rolls();
    let mut win1 = 0;
    let mut win2 = 0;

    count_wins(&rolls, 1, 1, &p1, &p2, &mut win1, &mut win2);

    std::cmp::max(win1, win2)
}
