fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    // 입력 전체를 trim()하여 공백과 개행 제거 후 split_whitespace()로 분리
    let input: Vec<i32> = buf
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let n = input[0];
    let k = input[1];

    println!("{}", n + k);
    println!("{}", n - k);
    println!("{}", n * k);
    println!("{}", n / k);
    println!("{}", n % k);
}