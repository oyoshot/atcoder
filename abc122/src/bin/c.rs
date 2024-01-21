use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, q: usize,
        s: Chars,
    }

    // 累積和
    let mut cum_sum = vec![0; n + 1];
    for i in 0..(n - 1) {
        cum_sum[i + 1] += cum_sum[i];
        if s[i] == 'A' && s[i + 1] == 'C' {
            cum_sum[i + 2] += 1;
        }
    }
    cum_sum[n] += cum_sum[n - 1];
    dbg!(&cum_sum);

    for _ in 0..q {
        input! {
            l: usize, r: usize
        }
        println!("{}", cum_sum[r] - cum_sum[l]);
    }
}
