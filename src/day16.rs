fn bits_to_dec(bits: &[u8]) -> u32 {
    let mut num = 0;
    for (i, &b) in bits.iter().rev().enumerate() {
        if b == 1 {
            num += 2_u32.pow(i.try_into().unwrap());
        }
    }
    num
}

fn acc_packet_ver(bits: &[u8], start: usize) -> (u32, usize) {
    if start >= bits.len() {
        return (0, 0);
    }

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
        let pktlen = 6 + (literal.len() / 4) * 5;
        // if pktlen % 8 != 0 {
        //     pktlen = pktlen + 8 - (pktlen % 8); // skip the padding
        // }
        idx += pktlen - 6; // the header is already counted
    } else {
        let length_type_id = bits[idx];
        idx += 1;

        if length_type_id == 0 {
            let subpkt_len = bits_to_dec(&bits[idx..idx + 15]) as usize;
            idx += 15;
            let idx_orig = idx;

            loop {
                let (ver, adv) = acc_packet_ver(bits, idx);
                if adv > idx_orig + subpkt_len {
                    panic!(
                        "Index out of range after parse: idx = {}, subpkt_len = {}, adv = {}",
                        idx, subpkt_len, adv
                    );
                }
                let should_break = idx_orig + subpkt_len == adv;
                version += ver;
                idx = adv;
                if should_break {
                    break;
                }
            }
        } else if length_type_id == 1 {
            let subpkt_num = bits_to_dec(&bits[idx..idx + 11]);
            idx += 11;

            for _ in 0..subpkt_num {
                let (ver, adv) = acc_packet_ver(bits, idx);
                version += ver;
                idx = adv;
            }
        } else {
            panic!("Unknown length_type_id = {}", length_type_id);
        }
    }

    (version, idx)
}

pub fn solve_1() -> u32 {
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

    acc_packet_ver(&bits, 0).0
}

pub fn solve_2() -> i64 {
    -1
}
