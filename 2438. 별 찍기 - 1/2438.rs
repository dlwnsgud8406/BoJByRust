use std::io::{self, stdin};

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("Input Error");

    let count: i32 = buf.trim().parse().expect("Not a Number");

    for i in 1..=count {
        println!("{}", "*".repeat(i as usize));
    }
}
