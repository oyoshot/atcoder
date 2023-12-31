use proconio::input;

fn main() {
    input! {
        n: usize,
        csf: [(usize, usize, usize); n-1],
    }

    for i in 0..n - 1 {
        let mut t = 0;
        for j in i..n - 1 {
            let (c, s, f) = csf[j];

            if t <= s {
                t = s;
            }
            t += (f - t % f) % f;
            t += c;
        }
        println!("{}", t);
    }
    println!("0");
}
