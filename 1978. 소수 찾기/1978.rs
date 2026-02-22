fn solve(input: Vec<u64>) -> u32 {
    if input.is_empty() {
        return 0;
    }

    let max = *input.iter().max().unwrap() as usize;

    if max < 2 {
        return 0;
    }

    let mut sieve = vec![true; max + 1];
    sieve[0] = false;
    sieve[1] = false;

    let limit = (max as f64).sqrt() as usize;

    for i in 2..=limit {
        if sieve[i] {
            for j in (i * i..=max).step_by(i) {
                sieve[j] = false;
            }
        }
    }

    input.iter()
        .filter(|&&num| num >= 2 && sieve[num as usize])
        .count() as u32
}

fn main() {
    let mut line1 = String::new();
    let mut line2 = String::new();

    std::io::stdin().read_line(&mut line1).unwrap(); // N
    std::io::stdin().read_line(&mut line2).unwrap(); // 숫자들

    let input: Vec<u64> = line2
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    println!("{}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        let answer = solve(vec![1,3,5,7]);
        assert_eq!(answer, 3);
    }
}