use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        s: [[Usize1]; m],
        p: [usize; m],
    };

    //dbg!(&s);
    //dbg!(&p);
    let mut ans = 0;
    for bit in 0..(1 << n) {
        let mut sw = vec![0; m];
        for i in 0..m {
            for &s in &s[i] {
                if bit >> s & 1 == 1 {
                    sw[i] ^= 1;
                }
            }
        }

        if sw == p {
            ans += 1;
        }
    }
    println!("{}", ans);
}
