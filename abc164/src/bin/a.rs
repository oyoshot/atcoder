use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: usize, w: usize,
    };

    println!("{}", if w >= s { "unsafe" } else { "safe" });
}
