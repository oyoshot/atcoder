use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize, a:usize, b:usize};
    println!("{}", (n * a).min(b));
}
