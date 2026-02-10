fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let score = buf.trim().parse::<usize>().unwrap();

    if 90 <= score && score <= 100 {
        println!("A");
    }
    else if 80 <= score && score < 90 {
        println!("B");
    }
    else if 70 <= score && score < 80 {
        println!("C");
    }
    else if 60 <= score && score < 70 {
        println!("D");
    }
    else {
        println!("F");
    }
}