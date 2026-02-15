use std::io::Read;

fn main() {
    // 입력 받기
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();


    // 문자열
    let a_str = it.next().unwrap().to_string();
    let b_str = it.next().unwrap().to_string();
    let c_str = it.next().unwrap().to_string();

    // 숫자
    let a: u32 = a_str.parse().unwrap();
    let b: u32 = b_str.parse().unwrap();
    let c: u32 = c_str.parse().unwrap();

    // 숫자열 계산
    println!("{}", a + b - c);

    // 문자열 계산
    let ab: u32 = format!("{}{}", a_str, b_str).parse().unwrap();
    println!("{}", ab - c);
}
