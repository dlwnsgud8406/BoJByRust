fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim(); // 개행 제거

    let mut input_index = String::new();
    std::io::stdin().read_line(&mut input_index).unwrap();
    let input_index: usize = input_index.trim().parse().unwrap(); // 숫자로 변환

    println!("{}", input.chars().nth(input_index-1).unwrap());
}
