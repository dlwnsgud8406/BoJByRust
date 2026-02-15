fn main() {
    let mut A = String::new();
    std::io::stdin().read_line(&mut A).unwrap();

    let mut B = String::new();
    std::io::stdin().read_line(&mut B).unwrap();

    let mut C = String::new();
    std::io::stdin().read_line(&mut C).unwrap();

    let A:usize = A.trim().parse().unwrap();
    let B:usize = B.trim().parse().unwrap();
    let C:usize = C.trim().parse().unwrap();

    let mut result_string = (A * B * C ).to_string();
    for i in 0..=9 {
        let digit = char::from_digit(i, 10).unwrap();
        println!("{}", result_string.chars().filter(|&c| c == digit).count());
    }
    
}
