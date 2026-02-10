fn main() {
    let stdin = std::io::stdin();

    for line in stdin.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        println!("{}", nums[0] + nums[1]);
    }
}