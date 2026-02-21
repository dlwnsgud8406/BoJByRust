use std::collections::VecDeque;

fn solve(input: &str) -> VecDeque<i32> {
    let mut lines = input.lines();
    let buffer_size: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut buffer: VecDeque<i32> = VecDeque::new();

    for line in lines {
        let information: i32 = line.trim().parse().unwrap();
        if information == -1 {
            break;
        } else if information == 0 && !buffer.is_empty() {
            buffer.pop_front(); // BinaryHeap의 pop() → VecDeque의 pop_front()
        } else {
            if buffer.len() < buffer_size {
                buffer.push_back(information); // push(Reverse(...)) → push_back(...)
            }
        }
    }

    buffer
}

fn main() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let input: String = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    let answer = solve(&input);

    if answer.is_empty() {
        println!("empty");
    } else {
        for v in &answer {
            print!("{} ", v);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "5\n1\n2\n0\n3\n4\n0\n5\n6\n0\n0\n-1\n";
        let result: Vec<i32> = solve(input).into_iter().collect();
        println!("{:?}", result);
        assert_eq!(result, vec![5, 6]);
    }

    #[test]
    fn test_example2() {
        let input = "1\n1\n2\n3\n4\n5\n6\n7\n-1\n";
        let result: Vec<i32> = solve(input).into_iter().collect();
        println!("{:?}", result);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_example3() {
        let input = "1\n1\n2\n0\n3\n4\n0\n5\n6\n0\n7\n0\n-1\n";
        let result: Vec<i32> = solve(input).into_iter().collect();
        println!("{:?}", result);
        assert_eq!(result, vec![]);
    }
}
