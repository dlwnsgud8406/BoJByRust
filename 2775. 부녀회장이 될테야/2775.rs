use std::io::{self, Read};

fn solve(a: i32, b: i32) -> i32 {
    let k = a as usize;
    let n = b as usize;
    
    let mut dp = vec![0i32; n + 1];
    
    for i in 1..=n {
        dp[i] = i as i32;
    }
    
    for _floor in 1..=k {
        for room in 2..=n {
            dp[room] = dp[room] + dp[room - 1];
        }
    }
    dp[n]
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();
    for _ in 0..t {
        let k: i32 = it.next().unwrap().parse().unwrap();
        let n: i32 = it.next().unwrap().parse().unwrap();
        out.push_str(&format!("{}\n", solve(k, n)));
    }
    print!("{}", out);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        assert_eq!(solve(1, 3), 6);
    }
    fn test_example2() {
        assert_eq!(solve(2, 3), 10);
    }
}