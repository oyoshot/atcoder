use proconio::input;

fn main() {
    input! {
     s: String
    }

    let mut ans = 0;
    let mut tmp = 0;
    for c in s.chars() {
        match &c {
            'A' | 'C' | 'G' | 'T' => {
                tmp += 1;
                ans = ans.max(tmp);
            }
            _ => {
                tmp = 0;
            }
        }
    }
    println!("{}", ans);
}
