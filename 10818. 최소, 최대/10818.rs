fn main() {
    let mut N = String::new();
    std::io::stdin().read_line(&mut N).unwrap();
    let N: u32 = N.trim().parse().unwrap();

    let mut N_array = String::new();
    std::io::stdin().read_line(&mut N_array).unwrap();
    let N_array:Vec<u32> = N_array.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{} {}", N_array.iter().min().unwrap(), N_array.iter().max().unwrap());
}
