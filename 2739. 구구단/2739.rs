use std::io::{self, stdin};

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let n = buf.trim().parse::<usize>().unwrap();
    
    for i in 1..10 {
        println!("{} * {} = {}", n, i, n * i);
    }
}
