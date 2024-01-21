use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize, b: usize,
    };

    let ans = match a {
        0..=5 => 0,
        6..=12 => b / 2,
        _ => b,
    };
    println!("{}", ans);
}
