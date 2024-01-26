use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize, p: usize,
        s: Chars,
    };

    if 2 % p == 0 || 5 % p == 0 {
        let mut ans = 0;
        for i in 0..n {
            let s = s[i].to_digit(10).unwrap() as usize;
            if s % p == 0 {
                ans += i + 1;
            }
        }
        println!("{}", ans);
        return;
    }

    let mut num = 0;
    let mut ten = 1;
    let mut counts = vec![0usize; p];
    counts[num] += 1;
    for i in (0..n).rev() {
        let add = s[i].to_digit(10).unwrap() as usize;
        num = (num + add * ten) % p;
        ten = (ten * 10) % p;
        counts[num] += 1;
    }

    let mut ans = 0;
    for count in counts {
        ans += count * count.saturating_sub(1) / 2;
    }
    println!("{}", ans);
}
