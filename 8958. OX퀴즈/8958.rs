fn main() {
    let mut T = String::new();
    std::io::stdin().read_line(&mut T).unwrap();
    let T:i32 = T.trim().parse().unwrap();

    for i in 0..T {
        // 입력 받기
        let mut answer = 0;
        let mut continuous = 1;

        let mut result = String::new();
        std::io::stdin().read_line(&mut result).unwrap();
        let result:String = result.trim().parse().unwrap();

        // 탐색하면서 연산
        for ch in result.chars() {
            if ch == 'O' {
                answer += continuous;
                continuous += 1;
            }
            else {
                continuous = 1;
            }
        }
        println!("{}", answer);
    }
}
