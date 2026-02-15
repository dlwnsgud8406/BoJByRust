fn main() {
    let mut answer:Vec<u32> = Vec::new();

    // input 입력
    let mut T = String::new();
    std::io::stdin().read_line(&mut T).unwrap();
    let T = T.trim().parse().unwrap();

    for i in 0..T {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let H: u32 = iter.next().unwrap().parse().unwrap();
        let W: u32 = iter.next().unwrap().parse().unwrap();
        let N: u32 = iter.next().unwrap().parse().unwrap();

        let floor = (N - 1) % H + 1;
        let room = (N - 1) / H + 1;
        let answer = floor * 100 + room;
        println!("{}", answer);
    }

}
