fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input:u32 = input.trim().parse().unwrap();

    for i in 1..=input {
        for _ in 0..(input - i) {
            print!(" ");
        }
        for _ in 0..i {
            print!("*");
        }
        println!();
    }

}