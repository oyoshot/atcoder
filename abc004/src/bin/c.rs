use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize };

    let n = n % 30;
    let mut cards = (1..=6).collect_vec();
    for i in 0..n {
        let l = i % 5;
        let r = i % 5 + 1;
        cards.swap(l, r);
    }
    println!("{}", cards.iter().join(""));
}
