use proconio::{fastout, input, marker::Chars};

const ATCODER: [char; 7] = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut is_ok = true;
    for i in 0..s.len() {
        if s[i] == '@' && ATCODER.contains(&t[i]) {
            continue;
        }
        if t[i] == '@' && ATCODER.contains(&s[i]) {
            continue;
        }
        if s[i] == t[i] {
            continue;
        }
        is_ok = false;
        break;
    }
    println!(
        "{}",
        if is_ok {
            "You can win"
        } else {
            "You will lose"
        }
    );
}
