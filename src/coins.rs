use itertools::Itertools;

const NUMBERS: [i32; 5] = [2, 3, 5, 7, 9];

fn main() {
    for p in NUMBERS.iter().copied().permutations(5) {
        if let [a, b, c, d, e] = p[0..5] {
            if compute(a, b, c, d, e) == 399 {
                println!("{} + {}x{}^2 + {}^3 - {} = 399", a, b, c, d, e);
            }
        }
    }
}

fn compute(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    a + b * c * c + d * d * d - e
}
