use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        xy: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![false; n]; n];
    for &(x, y) in &xy {
        graph[x][y] = true;
        graph[y][x] = true;
    }

    let mut ans = 0;
    for bit in 0..(1 << n) {
        let mut cnt = 0;
        let mut is_ok = true;
        for i in 0..n {
            if (bit >> i) & 1 == 1 {
                cnt += 1;
            }
            for j in 0..i {
                if ((bit >> i) & (bit >> j) & 1) == 1 && !graph[i][j] {
                    is_ok = false;
                }
            }
        }
        if is_ok {
            ans = ans.max(cnt);
        }
    }
    println!("{}", ans);
}
