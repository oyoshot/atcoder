use std::collections::BinaryHeap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        bc: [(usize, usize); m],
    };

    let mut heap = BinaryHeap::from(a);
    for &(b, c) in &bc {
        for _ in 0..b {
            heap.push(c);
        }
    }

    let mut ans = 0;
    for _ in 0..n {
        ans += heap.pop().unwrap();
    }
    println!("{}", ans);
}
