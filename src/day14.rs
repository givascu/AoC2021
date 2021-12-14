use std::collections::HashMap;

type PairCount = HashMap<(char, char), i64>;
type CharCount = HashMap<char, i64>;
type Rules = HashMap<(char, char), char>;

fn read_input() -> (PairCount, CharCount, Rules) {
    let input = include_str!("../input/14.txt");
    let first_line = input.lines().next().unwrap();

    let pair_count =
        first_line
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .fold(PairCount::new(), |mut acc, x| {
                *acc.entry((x[0], x[1])).or_default() += 1;
                acc
            });

    let char_count = first_line.chars().fold(CharCount::new(), |mut acc, c| {
        *acc.entry(c).or_default() += 1;
        acc
    });

    let rules = input
        .lines()
        .filter(|&line| line.contains("->"))
        .map(|line| {
            let parts = line
                .split_once("->")
                .map(|(x, y)| (x.trim(), y.trim()))
                .unwrap();
            (
                (
                    parts.0.chars().next().unwrap(),
                    parts.0.chars().nth(1).unwrap(),
                ),
                parts.1.chars().next().unwrap(),
            )
        })
        .collect::<Rules>();

    (pair_count, char_count, rules)
}

fn solve(pair_count: &PairCount, char_count: &CharCount, rules: &Rules, steps: usize) -> i64 {
    let mut pair_count_ = pair_count.clone();
    let mut char_count_ = char_count.clone();

    for _ in 1..=steps {
        let mut new_pair_count = PairCount::new();

        for (pair, count) in pair_count_ {
            if let Some(c) = rules.get(&pair) {
                *new_pair_count.entry((pair.0, *c)).or_default() += count;
                *new_pair_count.entry((*c, pair.1)).or_default() += count;
                *char_count_.entry(*c).or_default() += count;
            }
        }

        pair_count_ = new_pair_count;
    }

    char_count_.values().max().unwrap() - char_count_.values().min().unwrap()
}

pub fn solve_2() -> i64 {
    let (pairs, counters, rules) = read_input();
    solve(&pairs, &counters, &rules, 40)
}

pub fn solve_1() -> i64 {
    let (pairs, counters, rules) = read_input();
    solve(&pairs, &counters, &rules, 10)
}
