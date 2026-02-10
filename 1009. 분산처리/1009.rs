use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // 첫 줄에서 테스트 케이스 개수 읽기
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // t개의 테스트 케이스 처리
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let a = nums[0] as i64;
        let b = nums[1] as i64;
        
        // a^b의 마지막 자릿수를 구하는 함수
        let mut result = 1i64;
        let mut base = a % 10;
        let mut exp = b;
        
        while exp > 0 {
            if exp % 2 == 1 {
                result = (result * base) % 10;
            }
            base = (base * base) % 10;
            exp /= 2;
        }
        
        // 0이면 10번 컴퓨터
        if result == 0 {
            println!("10");
        } else {
            println!("{}", result);
        }
    }
}