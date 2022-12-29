fn bits_to_dec(bits: &[u8]) -> u64 {
    let mut num = 0;
    for (i, &b) in bits.iter().rev().enumerate() {
        if b == 1 {
            num += 2_u64.pow(i.try_into().unwrap());
        }
    }
    num
}

fn calculate_pkt(bits: &[u8], start: usize) -> (u64, usize) {
    let mut idx = start;
    idx += 3; // skip version

    let type_id = bits_to_dec(&bits[idx..idx + 3]);
    idx += 3;

    if type_id == 4 {
        let mut literal = Vec::new();
        for chunk in bits[idx..].chunks(5) {
            for &b in &chunk[1..5] {
                literal.push(b);
            }
            if chunk[0] == 0 {
                break;
            }
        }
        idx += (literal.len() / 4) * 5;
        return (bits_to_dec(&literal), idx);
    }

    let length_type_id = bits[idx];
    idx += 1;

    let mut values = Vec::new();

    if length_type_id == 0 {
        let subpkt_len = usize::try_from(bits_to_dec(&bits[idx..idx + 15])).unwrap();
        idx += 15;
        let orig_idx = idx;

        loop {
            let (val, new_idx) = calculate_pkt(bits, idx);
            assert!(
                new_idx <= orig_idx + subpkt_len,
                "Index out of range after recursion: idx = {}, subpkt_len = {}, new_idx = {}",
                idx,
                subpkt_len,
                new_idx
            );
            values.push(val);
            idx = new_idx;
            if orig_idx + subpkt_len == new_idx {
                break;
            }
        }
    } else if length_type_id == 1 {
        let subpkt_num = bits_to_dec(&bits[idx..idx + 11]);
        idx += 11;

        for _ in 0..subpkt_num {
            let (val, new_idx) = calculate_pkt(bits, idx);
            values.push(val);
            idx = new_idx;
        }
    } else {
        panic!("Unknown length_type_id = {length_type_id}");
    }

    match type_id {
        0 => (values.iter().sum(), idx),
        1 => (values.iter().product(), idx),
        2 => (*values.iter().min().unwrap(), idx),
        3 => (*values.iter().max().unwrap(), idx),
        5 => (u64::from(values[0] > values[1]), idx),
        6 => (u64::from(values[0] < values[1]), idx),
        7 => (u64::from(values[0] == values[1]), idx),
        _ => panic!("Unknown type_id = {type_id}"),
    }
}

fn accumulate_pkt_ver(bits: &[u8], start: usize) -> (u64, usize) {
    let mut idx = start;

    let mut version = bits_to_dec(&bits[idx..idx + 3]);
    idx += 3;

    let type_id = bits_to_dec(&bits[idx..idx + 3]);
    idx += 3;

    if type_id == 4 {
        let mut literal = Vec::new();
        for chunk in bits[idx..].chunks(5) {
            for &b in &chunk[1..5] {
                literal.push(b);
            }
            if chunk[0] == 0 {
                break;
            }
        }
        idx += (literal.len() / 4) * 5;
    } else {
        let length_type_id = bits[idx];
        idx += 1;

        if length_type_id == 0 {
            let subpkt_len = usize::try_from(bits_to_dec(&bits[idx..idx + 15])).unwrap();
            idx += 15;
            let orig_idx = idx;

            loop {
                let (ver, new_idx) = accumulate_pkt_ver(bits, idx);
                assert!(
                    new_idx <= orig_idx + subpkt_len,
                    "Index out of range after recursion: idx = {}, subpkt_len = {}, new_idx = {}",
                    idx,
                    subpkt_len,
                    new_idx
                );
                version += ver;
                idx = new_idx;
                if orig_idx + subpkt_len == new_idx {
                    break;
                }
            }
        } else if length_type_id == 1 {
            let subpkt_num = bits_to_dec(&bits[idx..idx + 11]);
            idx += 11;

            for _ in 0..subpkt_num {
                let (ver, new_idx) = accumulate_pkt_ver(bits, idx);
                version += ver;
                idx = new_idx;
            }
        } else {
            panic!("Unknown length_type_id = {length_type_id}");
        }
    }

    (version, idx)
}

fn build_input_bits() -> Vec<u8> {
    let bytes = hex::decode(include_str!("../input/16.txt")).unwrap();
    let mut bits = Vec::with_capacity(bytes.len() * 8);

    for b in bytes {
        for i in 0..8 {
            if b & (1 << (7 - i)) == 0 {
                bits.push(0);
            } else {
                bits.push(1);
            }
        }
    }

    bits
}

pub fn solve_1() -> u64 {
    let bits = build_input_bits();
    accumulate_pkt_ver(&bits, 0).0
}

pub fn solve_2() -> u64 {
    let bits = build_input_bits();
    calculate_pkt(&bits, 0).0
}
