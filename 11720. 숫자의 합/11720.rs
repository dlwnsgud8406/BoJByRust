fn main() {
    let mut N = String::new();
    std::io::stdin().read_line(&mut N).unwrap();
    let N: usize = N.trim().parse().unwrap(); // N을 usize으로 변환

    let mut N_num = String::new();
    std::io::stdin().read_line(&mut N_num).unwrap();
    let N_num = N_num.trim();

    let mut answer = 0;

    for i in 0..N {
        answer += N_num.chars().nth(i).unwrap().to_digit(10).unwrap();
    }
    println!("{}", answer);
}