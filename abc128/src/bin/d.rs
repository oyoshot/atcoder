use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize , k:usize,
        v: [i32; n],
    };

    let mut ans = 0;
    for l in 0..=k {
        for r in 0..=k - l {
            if l + r > n {
                continue;
            }

            let mut sum = 0;
            let mut gems = vec![];

            for i in 0..l {
                sum += v[i];
                gems.push(v[i]);
            }

            for i in n - r..n {
                sum += v[i];
                gems.push(v[i]);
            }

            gems.sort();
            for i in 0..k - l - r {
                if i >= gems.len() {
                    break;
                }
                if gems[i] > 0 {
                    break;
                }
                sum -= gems[i];
            }

            ans = ans.max(sum);
        }
    }

    println!("{}", ans);
}
