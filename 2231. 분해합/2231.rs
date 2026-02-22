fn digit_sum(mut x: u32) -> u32 {
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn solve(num: u32) -> u32 {
    let mut tmp = num;
    let mut digits = 0;
    while tmp > 0 {
        digits += 1;
        tmp /=10;
    }
    let max_add = 9 * digits;
    let start = if num > max_add {num - max_add} else {1};

    for m in start..num {
        if m + digit_sum(m) == num {
            return m;
        }
    }
    0
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut number:u32 = buf.trim().parse::<u32>().unwrap();

    println!("{}", solve(number));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        assert_eq!(198, solve(216));
    }

}