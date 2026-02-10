fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let pair: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = pair[0];
    let k = pair[1];

    println!("{}", n * k);
}