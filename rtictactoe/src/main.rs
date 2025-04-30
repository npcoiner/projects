use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use std::io::Write;

static WIDTH: i8 = 32;
static HEIGHT: i8 = 32;

fn init(){
     enable_raw_mode().expect("Failed to enter raw_mode");
    print!("\x1B[2J\x1B[3J\x1B[H");//Clear the screen, clear the scroll buffer, reset cursor
                                   //position.
    print!("ctrl c to quit\r\n");
    io::stdout().flush().expect("Failed to flush");
}

fn clearscrn(){
     print!("\x1B[2J\x1B[3J\x1B[H");//Clear the screen, clear the scroll buffer, reset cursor
                                       //position.
}



fn main() {
    init();

    for b in io::stdin().bytes() {
        let c = b.unwrap();
        clearscrn();
        print!("ctrl c to quit\r\n");
        print!("Binary: {0:08b} ASCII: {0:#03}\r\n", c);
        //print!("\x1B[5A"); // Move cursor up 5 lines

        //print!("\x1B[3C"); // Move cursor right 3 columns
        

        io::stdout().flush().expect("Failed to flush");

        if c == 3 {
			disable_raw_mode().unwrap();
            break;
        }
    }
}

