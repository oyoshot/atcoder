use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { n: usize, }

    let mut ans = 0;
    for i in 1..=n {
        ans += i * 10000;
    }
    println!("{}", ans / n);
}
