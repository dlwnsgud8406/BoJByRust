fn solve(tc: Vec<i32>) -> &'static str {
    if tc[2]  * tc[2] == tc[0] * tc[0] + tc[1] * tc[1] {
        "right"
    }
    else {
        "wrong"
    }
}

fn main() {
    while true {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut tc: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        if tc[0] == 0 && tc[1] == 0 && tc[2] == 0 {
            break;
        }
        else {
            tc.sort();
            let mut answer: &str = solve(tc);
            println!("{}", answer);
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = vec![6,8,10];
        let mut result = solve(input);
        println!("{}", result);
        assert_eq!("right", result);
    }

    #[test]
    fn test_example2() {
        let input = vec![25,52,60];
        let mut result = solve(input);
        println!("{}", result);
        assert_eq!("wrong", result);
    }

    #[test]
    fn test_example3() {
        let input = vec![5,12,13];
        let mut result = solve(input);
        println!("{}", result);
        assert_eq!("right", result);
    }

    #[test]
    fn test_example4() {
        let input = vec![0,0,0];
        let mut result = solve(input);
        println!("{}", result);
        assert_eq!("", result);
    }
}