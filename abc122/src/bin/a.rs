use proconio::input;

fn main() {
    input! {
        b: char
    };
    dbg!(b);
    match b {
        'A' => {
            println!("T")
        }
        'T' => {
            println!("A")
        }
        'C' => {
            println!("G")
        }
        'G' => {
            println!("C")
        }
        _ => {}
    }
}
