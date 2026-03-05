fn solve(input: Vec<u32>) -> u32 {
    let mut answer = 0;
    let mut A = input[0];
    let mut B = input[1];
    let mut V = input[2];

    let interval = A - B;

    (V - B + interval - 1) / interval
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut input:Vec<u32> = buffer.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        assert_eq!(solve(vec![2, 1, 5]), 4);
    }
    fn test_example2() {
        assert_eq!(solve(vec![5, 1, 6]), 2);
    }
    fn test_example3() {
        assert_eq!(solve(vec![100, 99, 1000000000]), 999999901);
    }
}