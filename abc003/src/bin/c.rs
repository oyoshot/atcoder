use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut r: [usize; n],
    }

    r.sort();

    let mut ans = 0.0;
    for i in n - k..n {
        ans = (ans + r[i] as f64) / 2.0;
    }
    println!("{:.9}", ans);
}
