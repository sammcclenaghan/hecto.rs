use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let ch = b.unwrap() as char;
        if ch == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
        println!("{}", ch);
    }
}
