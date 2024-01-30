use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    };

    a.sort();
    println!("{}", a[a.len() - 1] - a[0]);
}
