use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize
    }

    // エラトステネスのふるい
    let is_primes = eratosthenes(100_000);

    // 累積和
    let mut com_sum = vec![0; 100_001];
    for i in 1..com_sum.len() {
        if i % 2 == 1 && is_primes[i] && is_primes[(i + 1) / 2] {
            com_sum[i] = 1;
        }
    }
    for i in 1..com_sum.len() {
        com_sum[i] += com_sum[i - 1];
    }

    for _ in 0..q {
        input! {
            l: usize, r:usize,
        }

        println!("{}", com_sum[r] - com_sum[l - 1]);
    }
}

fn eratosthenes(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[1] = false;
    for i in 1..=n {
        if !is_prime[i] {
            continue;
        }
        for i in (i * 2..=n).step_by(i) {
            is_prime[i] = false;
        }
    }
    is_prime
}
