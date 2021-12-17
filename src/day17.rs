type Coord = (i64, i64);
type Target = (Coord, Coord);

fn height_max(vy: i64) -> i64 {
    if vy % 2 == 0 {
        vy / 2 * vy + vy / 2
    } else {
        (vy + 1) / 2 * vy
    }
}

fn flight_hits(pos: Coord, target: Target, vel: (i64, i64)) -> bool {
    let (mut x, mut y) = pos;
    let (mut vx, mut vy) = vel;
    let ((x_min, y_min), (x_max, y_max)) = target;

    if (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y) {
        return true; // hit target
    }
    if x > x_max || y < y_min {
        return false; // got past target
    }

    x += vx;
    y += vy;

    match vx.cmp(&0) {
        std::cmp::Ordering::Greater => vx -= 1,
        std::cmp::Ordering::Less => vx += 1,
        std::cmp::Ordering::Equal => {}
    }
    vy -= 1;

    flight_hits((x, y), target, (vx, vy))
}

fn read_target() -> Target {
    let r = include_str!("../input/17.txt")
        .trim()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_once(',')
        .map(|p| (p.0.trim(), p.1.trim()))
        .unwrap();

    let (x_min, x_max) =
        r.0.split_once('=')
            .unwrap()
            .1
            .split_once("..")
            .map(|x| (x.0.parse::<i64>().unwrap(), x.1.parse::<i64>().unwrap()))
            .unwrap();
    let (y_min, y_max) =
        r.1.split_once('=')
            .unwrap()
            .1
            .split_once("..")
            .map(|y| (y.0.parse::<i64>().unwrap(), y.1.parse::<i64>().unwrap()))
            .unwrap();

    ((x_min, y_min), (x_max, y_max))
}

pub fn solve_1() -> i64 {
    let t = read_target();
    let ((_, y_min), (x_max, _)) = t;

    itertools::iproduct!(0..=x_max, -y_min.abs()..y_min.abs())
        .filter(|&(vx, vy)| flight_hits((0, 0), t, (vx, vy)))
        .map(|v| height_max(v.1))
        .max()
        .unwrap()
}

pub fn solve_2() -> usize {
    let t = read_target();
    let ((_, y_min), (x_max, _)) = t;

    itertools::iproduct!(0..=x_max, -y_min.abs()..y_min.abs())
        .filter(|&(vx, vy)| flight_hits((0, 0), t, (vx, vy)))
        .count()
}
