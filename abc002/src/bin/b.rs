use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w: String,
    }

    println!("{}", w.replace(&['a', 'i', 'u', 'e', 'o'], ""));
}
