use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut se: [String; n],
    }

    se.sort();

    let mut times: Vec<(i32, i32)> = vec![];
    for v in se.iter() {
        let v: Vec<&str> = v.split('-').collect();

        let mut s: i32 = v[0].parse().unwrap();
        s -= s % 5;

        let mut e: i32 = v[1].parse().unwrap();
        // (0m, 5m) * (1 ~ 4m) + (60m - m)
        e += ((4 + e % 5) / 5) * (5 - e % 5) + (e % 100 / 56 * 40);

        times.push((s, e));
    }
    //dbg!(&times);

    let mut ps = -1;
    let mut pe = -1;
    for time in times {
        let (s, e) = time;
        if ps == -1 {
            ps = s;
            pe = e;
        } else if s <= pe {
            if pe <= e {
                pe = e;
            }
        } else {
            println!("{:04}-{:04}", ps, pe);
            ps = s;
            pe = e;
        }
    }
    println!("{:04}-{:04}", ps, pe);
}
