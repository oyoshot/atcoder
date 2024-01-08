use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { c: [[char; 4]; 4]}

    for i in 0..c.len() {
        for j in 0..c[i].len() {
            print!(
                "{}{}",
                c[c.len() - 1 - i][c[i].len() - 1 - j],
                if j != c[i].len() { " " } else { "" }
            );
        }
        println!();
    }
}
