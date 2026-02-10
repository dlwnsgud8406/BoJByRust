fn main() {
    let stdin = std::io::stdin();
    
    for line in stdin.lines() {
        let line = line.unwrap();
        let pair: Vec<i32> = line.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = pair[0];
        let k = pair[1];
        if n == 0 && k == 0 {
            return;
        }
        else {
            println!("{}", n + k);
        }
    }

}