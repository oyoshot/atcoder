use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        xa:f64, ya:f64, xb:f64, yb:f64, xc:f64, yc:f64,
    }

    let a = xa - xc;
    let b = yb - yc;
    let c = xb - xc;
    let d = ya - yc;

    println!("{:.1}", ((a * b) - (c * d)).abs() / 2.0);
}
