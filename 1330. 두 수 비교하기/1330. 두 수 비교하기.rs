use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if nums.len() == 2 {
            let (a, b) = (nums[0], nums[1]);
            if a > b {
                println!(">");
            } else if a < b {
                println!("<");
            } else {
                println!("==");
            }
        }
    }
}