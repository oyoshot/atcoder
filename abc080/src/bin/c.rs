use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        f: [[usize; 10]; n],
        p: [[i64; 11]; n],
    };

    let mut ans = i64::MIN;
    for bit in 1..1 << 10 {
        let mut sum = 0;
        for i in 0..n {
            let mut pos = 0;
            for j in 0..10 {
                if f[i][j] == 1 && (bit >> j) & 1 == 1 {
                    pos += 1;
                }
            }
            sum += p[i][pos];
        }
        ans = ans.max(sum);
    }
    println!("{:?}", ans);
}
