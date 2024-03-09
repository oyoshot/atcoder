use proconio::{fastout, input};

const H: usize = 100;
const W: usize = 100;

#[fastout]
fn main() {
    input! { a: usize, b: usize,};

    let mut grid = vec![['.'; W]; H];
    for i in 0..H / 2 {
        for j in 0..W {
            grid[i][j] = '#';
        }
    }

    let mut row = 0;
    let mut col = 0;
    for _ in 0..a - 1 {
        if col >= W {
            row += 2;
            col = 0;
        }
        grid[row][col] = '.';
        col += 2;
    }

    row = 51;
    col = 0;
    for _ in 0..b - 1 {
        if col >= W {
            row += 2;
            col = 0;
        }
        grid[row][col] = '#';
        col += 2;
    }

    println!("{} {}", H, W);
    for v in grid {
        println!("{}", v.iter().collect::<String>());
    }
}
