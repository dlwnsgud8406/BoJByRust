fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let music: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut asc = music.clone();
    asc.sort();

    let mut desc = music.clone();
    desc.sort_by(|a, b| b.cmp(a));

    if music == asc {
        println!("ascending");
    }
    else if music == desc {
        println!("descending");
    }
    else {
        println!("mixed");
    }
}
