use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize, d: usize, x_2000: usize,
    };
    let mut x = x_2000;
    for _ in 1..=10 {
        x = r * x - d;
        println!("{}", x);
    }
}
