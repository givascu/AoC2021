fn get_bit_one_frequency(diagnosis: &[String]) -> Vec<usize> {
    let mut frequency = vec![0; diagnosis[0].len()];
    for d in diagnosis {
        for (i, c) in d.chars().enumerate() {
            if c == '1' {
                frequency[i] += 1;
            }
        }
    }
    frequency
}

fn get_dominant_bits(diagnosis: &[String]) -> Vec<i64> {
    let n = diagnosis.len();
    let mut result = vec![0; diagnosis[0].len()];

    for (i, f) in get_bit_one_frequency(diagnosis).iter().enumerate() {
        match f.cmp(&(n - f)) {
            std::cmp::Ordering::Less => result[i] = 0,
            std::cmp::Ordering::Greater => result[i] = 1,
            std::cmp::Ordering::Equal => result[i] = 2,
        }
    }

    result
}

pub fn solve_2() -> i64 {
    let diagnosis = include_str!("../input/03.txt")
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let mut ogr = diagnosis.clone(); // oxygen generator rating
    let mut csr = diagnosis.clone(); // co2 scrubber rating

    for i in 0..diagnosis[0].len() {
        if ogr.len() == 1 && csr.len() == 1 {
            break;
        }

        if ogr.len() > 1 {
            let ogr_bits = get_dominant_bits(&ogr);
            let ogr_bit = if ogr_bits[i] == 0 { '0' } else { '1' };
            let mut ogr_copy = Vec::new();
            for item in &ogr {
                if item.chars().nth(i).unwrap() == ogr_bit {
                    ogr_copy.push(item.clone());
                }
            }
            ogr = ogr_copy;
        }

        if csr.len() > 1 {
            let csr_bits = get_dominant_bits(&csr);
            let csr_bit = if csr_bits[i] == 0 { '1' } else { '0' };
            let mut csr_copy = Vec::new();

            for item in &csr {
                if item.chars().nth(i).unwrap() == csr_bit {
                    csr_copy.push(item.clone());
                }
            }
            csr = csr_copy;
        }
    }

    let ogr = i64::from_str_radix(&ogr[0], 2).unwrap();
    let csr = i64::from_str_radix(&csr[0], 2).unwrap();

    ogr * csr
}

pub fn solve_1() -> i64 {
    let diagnosis = include_str!("../input/03.txt")
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for b in get_dominant_bits(&diagnosis) {
        if b == 1 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}
