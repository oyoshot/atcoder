use std::collections::VecDeque;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        q: usize,
    };

    let mut ans: VecDeque<char> = s.into_iter().collect();

    let mut is_rev = false;
    for _ in 0..q {
        input! { t: usize};

        if t == 1 {
            is_rev = !is_rev;
        } else {
            input! {f: usize, c: char};

            if !is_rev {
                match f {
                    1 => ans.push_front(c),
                    2 => ans.push_back(c),
                    _ => unreachable!(),
                }
            } else {
                match f {
                    1 => ans.push_back(c),
                    2 => ans.push_front(c),
                    _ => unreachable!(),
                }
            }
        }
    }
    println!(
        "{}",
        if is_rev {
            ans.iter().rev().collect::<String>()
        } else {
            ans.iter().collect::<String>()
        }
    );
}
