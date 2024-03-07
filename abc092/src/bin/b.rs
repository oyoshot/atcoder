use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize, x: usize,
        a:[usize; n],
    };

    let mut ans = x;
    for d in 1..=d {
        for a in &a {
            if (d - 1) % a == 0 {
                ans += 1;
            }
        }
    }
    println!("{:?}", ans);
}
