fn main() {
    let mut T = String::new();
    std::io::stdin().read_line(&mut T).unwrap();
    let T = T.trim().parse().unwrap();

    for i in 0..T {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        
        let mut iter = line.split_whitespace();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let s = iter.next().unwrap();
        let mut answer = String::new();


        for ch in s.chars() {
            for _ in 0..r {
                answer.push(ch);
            }
        }
        println!("{}", answer);
    }
}
