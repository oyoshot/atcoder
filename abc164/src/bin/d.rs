use proconio::{fastout, input, marker::Chars};

const MOD: usize = 2019;

#[fastout]
fn main() {
    input! {s: Chars};

    let mut num = 0;
    let mut d = 1;
    let mut counts = vec![0usize; MOD];
    counts[num] += 1;
    for &v in s.iter().rev()  {
        let add = v.to_digit(10).unwrap() as usize;
        num = (num + add * d) % MOD;
        d = (d * 10) % MOD;
        //dbg!(num);
        counts[num] += 1;
    }

    let mut ans = 0;
    for count in counts {
        ans += count * count.saturating_sub(1) / 2;
    }
    println!("{}", ans);
}
