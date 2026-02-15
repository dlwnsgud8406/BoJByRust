fn main() {
    let mut string = String::new();
    std::io::stdin().read_line(&mut string).unwrap();
    let words: Vec<&str> = string.split_whitespace().collect();

    println!("{}", words.len());
}
