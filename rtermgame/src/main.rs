#![allow(non_snake_case)]

fn main() {
    init();
    let stdin = std::io::stdin(); // capture stdin once

    for _b in std::io::Read::bytes(&mut stdin.lock()){
        clearscrn();

        println!("Hello, world!");

        flush();
    }
}

fn init() {
    crossterm::terminal::enable_raw_mode().expect("Failed to enter raw_mode");
    clearscrn(); // Clear the screen, clear the scroll buffer, reset cursor position.
    print!("ctrl c to quit\r\n");
    flush();
}
fn clearscrn() {
    print!("\x1B[2J\x1B[3J\x1B[H");
}


fn flush() {
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush");
}

