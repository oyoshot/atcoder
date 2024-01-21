use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, m : usize,
        lr : [(usize, usize); m],
    };

    let mut gates = vec![0; n + 2];
    for &(l, r) in &lr {
        gates[l] += 1;
        gates[r + 1] -= 1;
    }
    //dbg!(&gates);

    for i in 0..n {
        gates[i + 1] += gates[i];
    }

    let mut ans = 0;
    for gate in gates {
        if gate == m as i32 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
