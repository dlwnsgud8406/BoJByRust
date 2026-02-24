fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn lcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    let g = gcd(a, b);
    // (a / g) * b 순서로 계산하면 중간 overflow 위험이 줄어듦
    (a / g) * b
}

fn solve(array:Vec<u64>) -> (u64, u64) {

    (gcd(array[0], array[1]), lcd(array[0], array[1]))
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut array = buffer.trim().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();

    let answer = solve(array);
    println!("{}\n{}", answer.0, answer.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        assert_eq!(solve(vec![24, 18]), (6, 72))
    }
}