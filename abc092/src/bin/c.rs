use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let a = [&[0], &a[..], &[0]].concat();
    let f = |a: i64, b: i64| (a - b).abs();
    let csum = a.windows(2).map(|v| f(v[0], v[1])).sum::<i64>();
    //dbg!(&csum);

    for i in 0..n {
        println!(
            "{}",
            csum - f(a[i], a[i + 1]) - f(a[i + 1], a[i + 2]) + f(a[i], a[i + 2])
        );
    }
}
