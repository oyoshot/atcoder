use proconio::input;

fn main() {
    input! {
        m: u32,
    }

    let vv = match m {
        0..=99 => 0,
        100..=5000 => m / 100,
        6000..=30000 => m / 1000 + 50,
        35000..=70000 => (m / 1000 - 30) / 5 + 80,
        70001.. => 89,
        _ => unreachable!(),
    };
    println!("{:02}", vv);
}
