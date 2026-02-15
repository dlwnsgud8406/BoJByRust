use std::io;

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();
    let mut c_str = String::new();

    io::stdin().read_line(&mut a_str).unwrap();
    io::stdin().read_line(&mut b_str).unwrap();
    io::stdin().read_line(&mut c_str).unwrap();

    let a_str = a_str.trim().to_string();
    let b_str = b_str.trim().to_string();
    let c_str = c_str.trim().to_string();

    let a: i64 = a_str.parse().unwrap();
    let b: i64 = b_str.parse().unwrap();
    let c: i64 = c_str.parse().unwrap();

    println!("{}", a + b - c);

    let ab: i64 = format!("{}{}", a_str, b_str).parse().unwrap();
    println!("{}", ab - c);
}