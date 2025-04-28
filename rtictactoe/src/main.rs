use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use std::io::Write;

fn main() {
    enable_raw_mode().expect("Failed to enter raw_mode");
    for b in io::stdin().bytes() {
        let c = b.unwrap();
        print!("\x1B[2J\x1B[3J\x1B[H");
        print!("Binary: {0:08b} ASCII: {0:#03}\r\n", c);
        print!("\x1B[5A"); // Move cursor up 5 lines

        print!("\x1B[3C"); // Move cursor right 3 columns
        

        io::stdout().flush().expect("Failed to flush");

        if c as char == 'q' {
			disable_raw_mode().unwrap();
            break;
        }
    }
}

