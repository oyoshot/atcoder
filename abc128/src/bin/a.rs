use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { a: usize, p: usize};

    let piece = (a * 3) + p;
    println!("{}", piece / 2);
}
