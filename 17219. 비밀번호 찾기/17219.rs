use std::{collections::HashMap};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    let input:Vec<String> = buf.split(' ').map(|s| s.to_string()).collect();
    let n = input[0].trim().parse::<usize>().unwrap();
    let k = input[1].trim().parse::<usize>().unwrap();
    
    let mut passwordBook:HashMap<String, String> = HashMap::new();

    for _ in 0..n {
        let mut bufPasswordBook = String::new();
        std::io::stdin().read_line(&mut bufPasswordBook).unwrap();
        let input:Vec<&str> = bufPasswordBook.trim().split_whitespace().collect();
        let siteAddress = input[0].to_string();
        let sitePassword = input[1].to_string();
        passwordBook.insert(siteAddress, sitePassword);
    }

    for _ in 0..k {
        let mut bufQuery = String::new();
        std::io::stdin().read_line(&mut bufQuery).unwrap();
        let site = bufQuery.trim();
        println!("{}", passwordBook.get(site).unwrap());

}