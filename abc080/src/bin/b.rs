use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize};

    let sum = f(n);
    println!("{}", if n % sum == 0 { "Yes" } else { "No" });
}

fn f(n: usize) -> usize {
    if n < 10 {
        return n;
    }
    f(n / 10) + n % 10
}
