fn solve(N: usize, K: i32, cards:Vec<i32>) -> i32 {
    let mut answer = 0;
    for i in 0..N {
        for j in i+1..N {
            for k in j+1.. N {
                let sum = cards[i] + cards[j] + cards[k];
                if sum <= K {
                    answer = answer.max(sum);
                }
            }
        }
    }
    answer
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let mut rules:Vec<i32> = buf.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    let mut N = rules[0] as usize;
    let mut K = rules[1];

    let mut cards_buf = String::new();
    std::io::stdin().read_line(&mut cards_buf).unwrap();
    let mut cards:Vec<i32> = cards_buf.trim().split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    println!("{}", solve(N, K, cards));

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        assert_eq!(solve(5, 21, vec![5,6,7,8,9]), 21);
    }
    #[test]
    fn test_example2() {
        assert_eq!(solve(10, 500, vec![93,181,245,214,315,36,185,138,216,295]), 497);
    }
}