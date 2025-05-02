static WIDTH: i8 = 32;
static HEIGHT: i8 = 32;

fn init() {
    crossterm::terminal::enable_raw_mode().expect("Failed to enter raw_mode");
    print!("\x1B[2J\x1B[3J\x1B[H"); // Clear the screen, clear the scroll buffer, reset cursor position.
    print!("ctrl c to quit\r\n");
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush");
}

// Clear the screen, clear the scroll buffer, reset cursor position.
fn clearscrn() {
    print!("\x1B[2J\x1B[3J\x1B[H");
}

fn moveCursorUp(distance: i32) {
    print!("\x1B[{}A", distance);
}

fn moveCursorDown(distance: i32) {
    print!("\x1B[{}B", distance);
}

fn moveCursorRight(distance: i32) {
    print!("\x1B[{}C", distance);
}

fn moveCursorLeft(distance: i32) {
    print!("\x1B[{}D", distance);
}

fn checkWin(gameState: u16, xo: u16) -> (bool, bool) {
    const WIN_MASKS: [u16; 8] = [
        0b111_000_000,
        0b000_111_000,
        0b000_000_111,
        0b100_100_100,
        0b010_010_010,
        0b001_001_001,
        0b100_010_001,
        0b001_010_001,
    ];

    for &mask in WIN_MASKS.iter(){
        if (gameState & mask) & xo == mask {
            return (true, true);
        }
        if (gameState & mask) & !xo == mask {
            return (true, false);
        }
    }
    (true,true)
}

fn main() {
    init();
    let mut xo: u16 = 0b000_000_000; //1 means X, 0 means empty or O, verify with gameState
    let mut gameState: u16 = 0b000_000_000;// 1 means space is occupied
    let mut position = (0, 0); // Stores cursor position.

    let stdin = std::io::stdin(); // capture stdin once
    for b in std::io::Read::bytes(&mut stdin.lock()) {
        let c = b.unwrap();
        clearscrn();
        print!("ctrl c to quit\r\n");
        print!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r\n", c, c as char);

        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush");

        if c == 3 {
            // Ctrl + C
            crossterm::terminal::disable_raw_mode().unwrap();
            break;
        }
    }
}
