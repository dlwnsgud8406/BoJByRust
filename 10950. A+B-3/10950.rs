fn main() {
    let stdin = std::io::stdin();
    
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let T: i32 = buf.trim().parse::<i32>().unwrap();

    for _ in 0..T {
        buf.clear();
        stdin.read_line(&mut buf).expect("input error");
        let nums: Vec<i32> = buf.trim().split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        println!("{}", nums[0] + nums[1]);
    }

}