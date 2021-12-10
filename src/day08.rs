use std::{
    collections::HashSet,
    error::Error,
    fs,
    io::{BufRead, BufReader},
    ops::BitAnd,
};

type Signal = HashSet<char>;

fn pop_one<F>(from: &mut Vec<Signal>, predicate: F) -> Signal
where
    F: Fn(&Signal) -> bool,
{
    let signal = from.iter().find(|x| predicate(x)).unwrap().clone();
    from.retain(|x| !predicate(x));
    signal
}

pub fn solve_2() -> Result<i64, Box<dyn Error>> {
    let file = fs::File::open("input/08.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0i64;

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split('|').map(|x| {
            x.trim()
                .split(' ')
                .map(|y| y.chars().collect::<Signal>())
                .collect::<Vec<_>>()
        });
        let mut all = parts.next().ok_or("next all")?;
        let digits = parts.next().ok_or("next digits")?;

        // 1 is the only 2-segment digit
        let n1 = pop_one(&mut all, |x| x.len() == 2);
        // 4 is the only 4-segment digit
        let n4 = pop_one(&mut all, |x| x.len() == 4);
        // 7 is the only 3-segment digit
        let n7 = pop_one(&mut all, |x| x.len() == 3);
        // 8 is the only 7-segment digit
        let n8 = pop_one(&mut all, |x| x.len() == 7);
        // 3 is the only 5-segment digit that shares two segments with 1
        let n3 = pop_one(&mut all, |x| x.len() == 5 && x.bitand(&n1).len() == 2);
        // 5 is the only 5-segment digit that shares three segments with 4
        let n5 = pop_one(&mut all, |x| x.len() == 5 && x.bitand(&n4).len() == 3);
        // 2 is the only 5-segment digit left
        let n2 = pop_one(&mut all, |x| x.len() == 5);
        // 9 is the only 6-segment digit that shares four segments with 4
        let n9 = pop_one(&mut all, |x| x.len() == 6 && x.bitand(&n4).len() == 4);
        // 6 is the only 6-segment digit that shares one segment with 1
        let n6 = pop_one(&mut all, |x| x.len() == 6 && x.bitand(&n1).len() == 1);
        // 0 is the only 6-segment digit left
        let n0 = pop_one(&mut all, |x| x.len() == 6);

        let all = vec![n0, n1, n2, n3, n4, n5, n6, n7, n8, n9];

        let mut number = 0i64;
        for (i, d) in digits.iter().rev().enumerate() {
            let (n, _) = all.iter().enumerate().find(|(_, x)| *x == d).unwrap();
            number += (n * 10usize.pow(i as u32)) as i64;
        }
        sum += number;
    }

    Ok(sum)
}

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let file = fs::File::open("input/08.txt")?;
    let reader = BufReader::new(file);

    let mut counter = 0i64;

    for line in reader.lines() {
        let line = line?;
        let (_, digits) = line.split_once('|').ok_or("split_once()")?;
        let digits = digits
            .trim()
            .split(' ')
            .filter(|x| matches!(x.len(), 2 | 3 | 4 | 7))
            .count();
        counter += digits as i64;
    }

    Ok(counter)
}
