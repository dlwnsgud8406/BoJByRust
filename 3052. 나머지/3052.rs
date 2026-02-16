use std::collections::HashSet;

fn main() {
    let mut answer_set: HashSet<i32> = HashSet::new();
    
    for i in 0..10 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let number:i32 = input.trim().parse().unwrap();
        answer_set.insert(number % 42);
    }
    println!("{}", answer_set.len());
}
