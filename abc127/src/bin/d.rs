use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        bc: [(usize, usize); m],
    };

    let mut heap = vec![];
    for &a in &a {
        heap.push((1, a as i64));
    }
    for &(b, c) in &bc {
        heap.push((b as i64, c as i64));
    }
    heap.sort_by(|a, b| a.1.cmp(&b.1));

    let mut ans = 0;
    for _ in 0..n {
        let (b, c) = heap.pop().unwrap();
        ans += c;
        if b > 1 {
            heap.push((b - 1, c));
        }
    }
    println!("{}", ans);
}

// TLE
//fn main() {
//    input! {
//        n: usize, m: usize,
//        a: [usize; n],
//        bc: [(usize, usize); m],
//    };
//
//    let mut heap = BinaryHeap::from(a);
//    for &(b, c) in &bc {
//        for _ in 0..b {
//            heap.push(c);
//        }
//    }
//
//    let mut ans = 0;
//    for _ in 0..n {
//        ans += heap.pop().unwrap();
//    }
//    println!("{}", ans);
//}
