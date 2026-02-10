use std::io::{self, BufRead};

fn main() {
    let mut buf = String::new();
    let _ = io::stdin().lock().read_line(&mut buf)
        .expect("reading console input error.");

    let input:Vec<i32> = buf.trim()
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    
    let sum_of_squares: i32 = input.iter().map(|&x| x * x).sum();

    println!("{}", sum_of_squares % 10);
    
}