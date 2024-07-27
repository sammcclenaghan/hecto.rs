use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let ch = b as char;
        if ch.is_control() {
            println!("This is a control character \r")
        } else {
            println!("This is a normal character \r")
        }
        if ch == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
        println!("{}", ch);
    }
}
