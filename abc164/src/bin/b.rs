use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: i64,  b: i64, mut  c: i64,  d: i64,
    };

    while a > 0 && c > 0 {
        c -= b;
        if c <= 0 {
            println!("Yes");
            return;
        }
        a -= d;
    }
    println!("No");
}
