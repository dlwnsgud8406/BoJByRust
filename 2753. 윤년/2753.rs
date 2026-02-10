fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let n = buf.trim().parse::<usize>().unwrap();

    if (n % 4 == 0 && n % 100 != 0) || n % 400 == 0 {
        println!("1");
    } else {
        println!("0");
    }
}