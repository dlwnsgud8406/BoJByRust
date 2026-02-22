struct Order {
    participant_number: u32,
    current_tshirt_count: Vec<u32>,
    per_tshirt: u32,
    per_pen: u32
}

impl Order {
    fn new() -> Self {
        Self {
            participant_number: 0,
            current_tshirt_count: Vec::new(),
            per_tshirt: 0,
            per_pen: 0,
        }
    }

    fn tshirt_cal(&self) -> u32 {
        self.current_tshirt_count.iter().map(|&size| (size + self.per_tshirt - 1) / self.per_tshirt).sum()
    }

    fn pen_cal(&self) -> (u32, u32) {
        let bundle = self.participant_number / self.per_pen;
        let single = self.participant_number % self.per_pen;
        (bundle, single)
    }
}


fn main() {
    let mut arena_order = Order::new();

    let mut first_line = String::new();
    std::io::stdin().read_line(&mut first_line).unwrap();
    let participant_number:u32 = first_line.trim().parse().unwrap();

    let mut second_line = String::new();
    std::io::stdin().read_line(&mut second_line).unwrap();
    let current_tshirt_count:Vec<u32> = second_line.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();

    let mut third_line = String::new();
    std::io::stdin().read_line(&mut third_line).unwrap();
    let per_count:Vec<u32> = third_line.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();

    arena_order.participant_number = participant_number;
    arena_order.current_tshirt_count = current_tshirt_count;
    arena_order.per_tshirt = per_count[0];
    arena_order.per_pen = per_count[1];

    let (arena_bundle, arena_single) = arena_order.pen_cal();

    println!("{}", arena_order.tshirt_cal());
    println!("{} {}", arena_bundle, arena_single);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        let mut test_order = Order::new();
        test_order.participant_number = 23;
        test_order.current_tshirt_count = vec![3, 1, 4, 1, 5, 9];
        test_order.per_tshirt = 5;
        test_order.per_pen = 7;
        
        let T = test_order.tshirt_cal();
        let (test_bundle, test_single) = test_order.pen_cal();
        
        println!("{}", T);
        println!("{} {}", test_bundle, test_single);

        assert_eq!(7, T);
        assert_eq!(3, test_bundle);
        assert_eq!(2, test_single);
    }
}