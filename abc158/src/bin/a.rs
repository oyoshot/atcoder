use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {s: String};

    println!(
        "{}",
        if s == "AAA" || s == "BBB" {
            "No"
        } else {
            "Yes"
        }
    );
}
