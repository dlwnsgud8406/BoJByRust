fn solve(string_buf: &str)->u64 {
    const R: u64 = 31;
    const M: u64 = 1_234_567_891;

    let mut answer: u64 = 0;
    let mut pow: u64 = 1;

    for b in string_buf.bytes() {
        let val = (b - b'a' + 1) as u64;
        answer = (answer + val * pow) % M;
        pow = (pow * R) % M;
    }
    answer
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    let mut string_buf = String::new();
    std::io::stdin().read_line(&mut string_buf).unwrap();

    println!("{}", solve(string_buf.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        assert_eq!(solve("abcde"), 4739715);
    }
    #[test]
    fn test_example2() {
        assert_eq!(solve("zzz"), 25818);
    }
    #[test]
    fn test_example3() {
        assert_eq!(solve("i"), 9);
    }
}