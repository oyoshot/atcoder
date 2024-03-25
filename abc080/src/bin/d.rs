use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: usize,
    }
    let mut st = vec![vec![]; c];
    for _ in 0..n {
        input! {s: usize, t: usize, c: usize, }
        st[c - 1].push((s, t));
    }

    for i in 0..c {
        st[i].sort();

        let mut tmp: Vec<(usize, usize)> = vec![];
        for &(s, t) in &st[i] {
            if tmp.len() > 0 && tmp.last().unwrap().1 == s {
                tmp.last_mut().unwrap().1 = t;
            } else {
                tmp.push((s, t));
            }
        }
        st[i] = tmp;
    }
    //dbg!(&st);

    let mut csum = vec![0; 100_010];
    for i in 0..c {
        for &(s, t) in &st[i] {
            csum[s] += 1;
            csum[t + 1] -= 1;
        }
    }

    let mut ans = 0;
    for i in 0..100_000 {
        csum[i + 1] += csum[i];
        ans = ans.max(csum[i])
    }
    println!("{}", ans);
}
