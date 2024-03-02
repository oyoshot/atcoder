use std::i64;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n : usize,
        a : [i64; n]
    };

    let mut aa: Vec<_> = a.iter().enumerate().map(|(i, &v)| v - i as i64).collect();
    aa.sort();
    let mp = aa[n / 2];
    let ans = aa.iter().map(|v| (mp - v).abs()).sum::<i64>();
    println!("{}", ans);
}
