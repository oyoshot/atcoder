use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        mut xyc: [(usize, usize, char); n],
    }

    let mut black = vec![vec![0; k + 1]; k + 1];
    let mut white = vec![vec![0; k + 1]; k + 1];
    for (x, y, c) in xyc {
        let mut x = x % 2 * k;
        if k <= x {
            x -= k;
        }
        let mut y = y % 2 * k;
        if k <= y {
            y -= k;
        }
        let arr = if c == 'B' { &mut black } else { &mut white };
        arr[x + 1][y + 1] += 1;
    }
    dbg!(&black, &white);
}
