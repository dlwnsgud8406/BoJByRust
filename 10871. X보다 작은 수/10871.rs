fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let input_n_x: Vec<i32> = buf
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect();

    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let input_array: Vec<i32> = buf
    .trim()
    .split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect();

    let a_count = input_n_x[0] as usize;
    let x = input_n_x[1];

    for i in 0 .. a_count {
        if input_array[i] < x {
            print!("{} ", input_array[i])
        }
    }

}
