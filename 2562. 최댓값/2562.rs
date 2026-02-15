fn main() {
    let mut input_array:Vec<u32> = Vec::new();
    for i in 0..9 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input_array.push(input.trim().parse().unwrap());
    }
    let maxValue = *input_array.iter().max().unwrap();

    let maxValueIdx = input_array.iter().position(|&x| x == maxValue).unwrap() + 1; 

    println!("{}\n{}", maxValue, maxValueIdx);
}
