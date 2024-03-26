use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        sp: [(String, usize); n]
    };

    let mut sp = sp
        .iter()
        .enumerate()
        .map(|(i, (s, p))| (s.clone(), Reverse(*p), i))
        .collect_vec();
    sp.sort();
    for &(_, _, i) in sp.iter() {
        println!("{}", i + 1);
    }
}
