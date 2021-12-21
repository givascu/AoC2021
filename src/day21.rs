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

    fn counter(&self) -> u32 {
        self.counter
    }
}

#[derive(Debug)]
struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(pos: u32) -> Player {
        Player { pos, score: 0 }
    }

    fn score(&self) -> u32 {
        self.score
    }

    fn advance(&mut self, steps: u32) {
        self.pos += steps % 10;
        if self.pos > 10 {
            self.pos %= 10;
        }
        self.score += self.pos;
    }
}

pub fn solve_1() -> u32 {
    let players = include_str!("../input/21.txt")
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
    assert!(players.len() == 2);

    let mut p1 = Player::new(players[0]);
    let mut p2 = Player::new(players[1]);
    let mut dice = Dice::new();

    for round in 1..u32::MAX {
        let mut roll = 0;
        for _ in 0..3 {
            roll += dice.roll();
        }

        if round % 2 == 1 {
            p1.advance(roll);
            if p1.score() >= 1000 {
                return p2.score() * dice.counter();
            }
        } else {
            p2.advance(roll);
            if p2.score() >= 1000 {
                return p1.score() * dice.counter();
            }
        }
    }

    0
}

pub fn solve_2() -> i64 {
    -1
}
