use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        s: [String; n],
    };

    let ans = s.iter().unique().count();
    println!("{}", ans);
}
