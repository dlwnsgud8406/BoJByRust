fn solve(input: &str) ->&'static str {
    let rev: String = input.chars().rev().collect();
    if input == rev {
        "yes"
    }
    else {
        "no"
    }
}

fn main() {
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let mut input = buf.trim();

        if input == "0" {
            break
        }
        else {
            println!("{}", solve(input));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        assert_eq!("yes", solve("121"));
        assert_eq!("no", solve("1231"));
        assert_eq!("yes", solve("12421"));
    }
}