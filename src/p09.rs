use std::error::Error;

fn get_neighbors(map: &[String], pos: (usize, usize)) -> Vec<u32> {
    let mut neighbors = Vec::new();

    if pos.0 > 0 {
        neighbors.push(
            map[pos.0 - 1]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_digit(10)
                .unwrap(),
        );
    }
    if pos.0 < map.len() - 1 {
        neighbors.push(
            map[pos.0 + 1]
                .chars()
                .nth(pos.1)
                .unwrap()
                .to_digit(10)
                .unwrap(),
        );
    }
    if pos.1 > 0 {
        neighbors.push(
            map[pos.0]
                .chars()
                .nth(pos.1 - 1)
                .unwrap()
                .to_digit(10)
                .unwrap(),
        );
    }
    if pos.1 < map[0].len() - 1 {
        neighbors.push(
            map[pos.0]
                .chars()
                .nth(pos.1 + 1)
                .unwrap()
                .to_digit(10)
                .unwrap(),
        );
    }

    neighbors
}

pub fn solve_1() -> Result<i64, Box<dyn Error>> {
    let map = include_str!("../data/09.in")
        .split('\n')
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    let height = map.len();
    let width = map[0].len();
    let mut sum = 0i64;

    for y in 0..height {
        for x in 0..width {
            let value = map[y].chars().nth(x).unwrap().to_digit(10).unwrap();
            if get_neighbors(&map, (y, x))
                .iter()
                .all(|neigh| *neigh > value)
            {
                sum += (value + 1) as i64;
            }
        }
    }

    Ok(sum)
}
