fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let mut hour: i32 = iter.next().unwrap().parse().unwrap();
    let mut minute: i32 = iter.next().unwrap().parse().unwrap();

    // 00시 인 경우, 계산에 편의상 24시로 설정
    if hour == 0 {
        hour = 24;
    }

    // 실제 연산 로직
    if minute < 45 {
        hour -= 1;
        minute = minute + 60 - 45
    }
    else {
        minute -= 45;
    }

    // 편의상 설정한것을 원복
    if hour == 24 {
        hour = 0;
    }

    println!("{} {}", hour, minute);

}
