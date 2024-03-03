use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut cum_sum = vec![0];
    for (i, &v) in a.iter().enumerate() {
        cum_sum.push(cum_sum[i] + v);
    }
    let f = |l: usize, r: usize| cum_sum[r].abs_diff(cum_sum[l]);
    let pq = |l: usize, r: usize| {
        let a = *[f(l, r), f(0, l)].iter().max().unwrap();
        let b = *[f(l, r), f(0, l)].iter().min().unwrap();
        a - b
    };
    let rs = |l: usize, r: usize| {
        let a = *[f(l, r), f(r, n)].iter().max().unwrap();
        let b = *[f(l, r), f(r, n)].iter().min().unwrap();
        a - b
    };
    let calc = |l, c, r| {
        let x = [f(0, l), f(l, c), f(c, r), f(r, n)];
        x.iter().max().unwrap() - x.iter().min().unwrap()
    };

    let mut l = 0;
    let mut r = 2;
    let mut ans = usize::MAX;
    for i in 1..n - 1 {
        while pq(l, i) > pq(l + 1, i) {
            l += 1;
        }
        while rs(i, r) > rs(i, r + 1) {
            r += 1;
        }
        ans = ans.min(calc(l, i, r));
    }

    println!("{}", ans);
}
