use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        match line.trim() {
            "ping" => println!("pong"),
            "date" => println!("{}", chrono::Local::now().date_naive()),
            _ => println!("unknown command"),
        }
    }
}
