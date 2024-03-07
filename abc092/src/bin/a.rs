use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize
    };
    let x = if a < b { a } else { b };
    let y = if c < d { c } else { d };
    println!("{}", x + y);
}
