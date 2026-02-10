use std::io::{self};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lines() {
        let line = line.unwrap();
        let nums: Vec<f64> = line.trim().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        if nums.len() == 2 {
            let a = nums[0];
            let b = nums[1];
            println!("{:.32}", a / b);  // 긴 소수점 출력
        }
    }
}