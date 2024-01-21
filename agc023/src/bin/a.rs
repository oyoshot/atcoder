use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut cum_sum = vec![0; n + 1];
    for i in 0..n {
        cum_sum[i + 1] += cum_sum[i] + a[i];
    }
    //dbg!(&cum_sum);

    let mut map = HashMap::new();
    for i in 0..n + 1 {
        *map.entry(cum_sum[i]).or_insert(0 as i64) += 1;
    }
    //dbg!(&map);

    let mut ans = 0;
    for (_, v) in map {
        //dbg!(v * (v - 1) / 2);

        // nC2
        ans += v * (v - 1) / 2;
    }
    println!("{}", ans);
}
