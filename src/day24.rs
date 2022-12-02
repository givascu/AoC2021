struct Alu {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Alu {
    fn new() -> Alu {
        Alu {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    fn _print(&self) {
        println!(
            "(w: {}, x: {}, y: {}, z: {})",
            self.w, self.x, self.y, self.z
        );
    }

    fn var_ref(&mut self, var: &str) -> &mut i64 {
        match var {
            "w" => &mut self.w,
            "x" => &mut self.x,
            "y" => &mut self.y,
            "z" => &mut self.z,
            _ => panic!("Unknown variable: {}", var),
        }
    }

    fn var_val(&self, var: &str) -> i64 {
        match var {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            _ => panic!("Unknown variable: {}", var),
        }
    }

    fn exec_lines(&mut self, lines: &[&str], input: &[i64]) {
        let mut input_idx = 0_usize;

        for &line in lines {
            let mut parts = line.split(' ');

            let op = parts.next().unwrap();
            let param1 = parts.next().unwrap();
            let val1 = self.var_val(param1);

            match op {
                "inp" => {
                    assert!(input_idx < input.len());
                    *self.var_ref(param1) = input[input_idx];
                    input_idx += 1;
                }
                "add" | "mul" | "div" | "mod" | "eql" => {
                    let param2 = parts.next().unwrap();
                    let val2 = if let Ok(num) = param2.parse::<i64>() {
                        num
                    } else {
                        self.var_val(param2)
                    };
                    let res = match op {
                        "add" => val1 + val2,
                        "mul" => val1 * val2,
                        "div" => val1 / val2,
                        "mod" => val1 % val2,
                        "eql" => i64::from(val1 == val2),
                        _ => panic!("Unknown instruction: {}", op),
                    };
                    *self.var_ref(param1) = res;
                }
                _ => panic!("Unknown instruction: {}", op),
            }
        }
    }
}

pub fn solve_1() -> i64 {
    let lines = include_str!("../input/24.txt").lines().collect::<Vec<_>>();

    // Solving the input instructions by hand, and assuming 1 for each eql instruction,
    // the following equations result for the fourteen digits (a,b,c,d,e,f,g,h,i,j,k,l,m,n):
    //
    // d = e
    // f + 1 = g
    // h + 2 = i
    // c + 7 = j
    // k - 1 = l
    // b + d = m
    // a - 2 = n
    //
    // Hence, the number looks like: a,b,c,d,d,f,f+1,h,h+2,c+7,k,k-1,b+4,a-2
    //
    // Solving this by maximizing/minimizing the number, the following numbers result:
    // 95299897999897 - MAX
    // 31111121382151 - MIN

    let input_str = "95299897999897";
    let input = input_str
        .chars()
        .map(|c| i64::from(c.to_digit(10).unwrap()))
        .collect::<Vec<_>>();

    let mut alu = Alu::new();
    alu.exec_lines(&lines, &input);
    assert!(alu.z == 0);

    input_str.parse::<i64>().unwrap()
}

pub fn solve_2() -> i64 {
    let lines = include_str!("../input/24.txt").lines().collect::<Vec<_>>();

    let input_str = "31111121382151";
    let input = input_str
        .chars()
        .map(|c| i64::from(c.to_digit(10).unwrap()))
        .collect::<Vec<_>>();

    let mut alu = Alu::new();
    alu.exec_lines(&lines, &input);
    assert!(alu.z == 0);

    input_str.parse::<i64>().unwrap()
}
