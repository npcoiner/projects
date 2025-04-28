use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use std::io::Write;

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        print!("{}\r\n", c);
        io::stdout().flush().unwrap();

        if c == 'q' {
			disable_raw_mode().unwrap();
            break;
        }
    }
}

