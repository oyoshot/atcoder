use proconio::input;
use regex::Regex;

fn main() {
    input! {
        a: usize, b: usize,
        s: String
    }

    let p = format!(r"^\d{{{}}}-\d{{{}}}$", a, b);
    dbg!(&p);
    let re = Regex::new(&p).unwrap();

    println!("{}", if re.is_match(s.as_str()) { "Yes" } else { "No" });
}
