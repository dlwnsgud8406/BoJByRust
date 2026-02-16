fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    let mut positions = vec![-1; 26];

    for (index, ch) in input.chars().enumerate() {
        if let Some(pos) = ch.to_ascii_lowercase().to_digit(36) {
            let pos = pos as usize - 10;
            if positions[pos] == -1 {
                positions[pos] = index as i32;
            }
        }
    }
    for pos in positions {
        print!("{} ", pos);
    }
}
