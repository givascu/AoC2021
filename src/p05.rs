use std::{fs, io::{BufReader, BufRead}, collections::HashMap, cmp};

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

pub fn solve_2() -> i32 {
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
        let x_min = cmp::min(l.p1.0, l.p2.0);
        let x_max = cmp::max(l.p1.0, l.p2.0);
        let y_min = cmp::min(l.p1.1, l.p2.1);
        let y_max = cmp::max(l.p1.1, l.p2.1);

        if l.p1.0 == l.p2.0 {
            // X coordinate is the same
            for y in y_min .. (y_max + 1) {
                let p = crossed.entry((l.p1.0, y)).or_insert(0);
                *p += 1;
            }
        } else if l.p1.1 == l.p2.1 {
            // Y coordinate is the same
            for x in x_min .. (x_max + 1) {
                let p = crossed.entry((x, l.p1.1)).or_insert(0);
                *p += 1;
            }
        } else if (l.p1.0 - l.p2.0).abs() == (l.p1.1 - l.p2.1).abs() {
            // 45 degree line
            let slope = (l.p2.1 - l.p1.1) / (l.p2.0 - l.p1.0);
            let b = l.p1.1 - slope * l.p1.0;
            for x in x_min .. (x_max + 1) {
                let y = slope * x + b;
                let p = crossed.entry((x, y)).or_insert(0);
                *p += 1;
            }
        }
    }

    crossed.iter().filter(|&(_, v)| *v >= 2).count() as i32
}
