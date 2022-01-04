const NUMBERS: [i32; 5] = [2, 3, 5, 7, 9];

fn main() {
    for i_a in 0..5 {
        let a = NUMBERS[i_a];
        for i_b in 0..5 {
            if i_a == i_b {
                continue;
            }
            let b = NUMBERS[i_b];
            for i_c in 0..5 {
                if i_a == i_c || i_b == i_c {
                    continue;
                }
                let c = NUMBERS[i_c];
                for i_d in 0..5 {
                    if i_a == i_d || i_b == i_d || i_c == i_d {
                        continue;
                    }
                    let d = NUMBERS[i_d];
                    for i_e in 0..5 {
                        if i_a == i_e || i_b == i_e || i_c == i_e || i_d == i_e {
                            continue;
                        }
                        let e = NUMBERS[i_e];

                        if compute(a, b, c, d, e) == 399 {
                            println!("{} + {}x{}^2 + {}^3 - {} = 399", a, b, c, d, e);
                        }
                    }
                }
            }
        }
    }
}

fn compute(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    a + b * c * c + d * d * d - e
}
