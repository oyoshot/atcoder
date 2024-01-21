fn main() {
    proconio::input! {
        n: i64,
        txys: [[i32; 3];n],
    }

    let mut t = 0;
    let mut x = 0;
    let mut y = 0;

    let mut ret = "Yes";
    for txy in txys {
        t = txy[0] - t;
        x = (txy[1] - x).abs();
        y = (txy[2] - y).abs();
        let xy = x + y;

        if !(t >= xy && t % 2 == xy % 2) {
            ret = "No";
            break;
        }
    }

    println!("{}", ret);
}
