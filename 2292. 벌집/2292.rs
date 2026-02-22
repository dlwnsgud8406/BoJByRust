fn solve(N: u64) -> u64 {
    let mut current_max_number = 1;
    let mut loop_count = 1;
    while current_max_number < N {
        current_max_number += (loop_count * 6);
        loop_count += 1;
    }
    loop_count
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut input:u64 = buf.trim().parse::<u64>().unwrap();
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        assert_eq!(solve(13), 3);
        assert_eq!(solve(58), 5);
    }
}