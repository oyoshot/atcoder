use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {a: usize, b: usize};

    for i in 0..=1009 {
        let tax_a = i * 8 / 100;
        let tax_b = i * 10 / 100;
        if tax_a == a && tax_b == b {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}
