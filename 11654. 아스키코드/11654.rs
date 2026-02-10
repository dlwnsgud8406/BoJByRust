fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let alphabet = input.trim().chars().next().unwrap();
    println!("{}", alphabet as u8);
}

fn test_main(ch: char) -> u8 {
    ch as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ascii_code() {
        assert_eq!(test_main('A'), 65);
        assert_eq!(test_main('C'), 67);
        assert_eq!(test_main('0'), 48);
        assert_eq!(test_main('9'), 57);
        assert_eq!(test_main('a'), 97);
        assert_eq!(test_main('z'), 122);
    }
}