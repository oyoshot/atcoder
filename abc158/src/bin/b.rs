use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: u64, a: u64, b: u64};

    let mut ans = (n / (a + b)) * a;
    ans += (n % (a + b)).min(a);
    println!("{}", ans);
}
