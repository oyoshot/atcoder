fn main() {
    proconio::input! {
        a: String,
        b: String,
    }
    let x = format!("{}{}", a.trim(), b.trim());
    let n: i64 = x.parse().unwrap();
    let sq = (n as f64).sqrt() as i64;
    dbg!(sq * sq);
    dbg!(n);
    if sq * sq == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
