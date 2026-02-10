use std::io::{self, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_line(&mut buf).expect("input error");

    let n = buf.trim().parse::<usize>().unwrap();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let mut output = String::new();
    
    for i in 1..=n {
        output.push_str(&format!("{}\n", i));
    }
    
    handle.write_all(output.as_bytes()).unwrap();

}