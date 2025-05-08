static WIDTH: i8 = 32;
static HEIGHT: i8 = 32;

fn main() {
    init();
    let mut xo: u16 = 0b000_000_000; //1 means X, 0 means empty or O, verify with gameState
    let mut gameState: u16 = 0b000_000_000;// 1 means space is occupied
    let mut position: (u16,u16) = (0, 0); // Stores cursor position. (x,y) coordinate
    let mut gameStart = true;
    let stdin = std::io::stdin(); // capture stdin once
    std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush");

    for b in std::io::Read::bytes(&mut stdin.lock()){
        clearscrn();
        if gameStart{
            print!("Welcome to tic-tac-toe\r\n");
            gameStart = false; 
            printGameFromState(xo, gameState, position);
            std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush");
            continue;
        }
        //print!("ctrl c to quit\r\n");
        if handleInput(b.expect("std in byte error"),&mut position,  &mut gameState,  &mut xo){
            break;
        }
        //print!("Position:{0},{1}\r\n",position.0,position.1);
        printGameFromState(xo, gameState, position);

        if checkWin(gameState, xo).1 == true{
            clearscrn();
            print!("WIN!\r\n");
            print!("Press anything to start over \r\n");                       
            std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush");
         
            gameState = 0b000_000_000;
            xo = 0b000_000_000;
            gameStart = true;
            continue;        
            
        }

        std::io::Write::flush(&mut std::io::stdout()).expect("Failed to flush");
    }
}

fn printGameFromState(xo: u16, gameState :u16, position: (u16,u16)){
    let mut mask : u16 = 0b0000_0001_0000_0000;
    let mut counter: u8 = 1;
    while mask > 0{
        if (mask & gameState > 0){
            print!("x");
        }
        else{
            if xo & mask > 0{
                print!("o");
            }
            else{
                print!("-");
            }
        }
        if counter % 3 == 0{
            print!("\r\n");
        }
        counter+= 1;
        mask = mask >> 1;
    }
    moveCursorUp(3);
    moveCursorRight(position.0);
    moveCursorDown(position.1);
}

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

fn moveCursorUp(distance: u16) {
    if distance > 0 {
        print!("\x1B[{}A", distance);
    }
}

fn moveCursorDown(distance: u16) {
    if distance > 0 {
        print!("\x1B[{}B", distance);
    }
}

fn moveCursorRight(distance: u16) {
    if distance > 0 {
        print!("\x1B[{}C", distance);
    }
}

fn moveCursorLeft(distance: u16) {
    if distance > 0 {       
    print!("\x1B[{}D", distance);
    }
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
        0b001_010_100,
    ];

    for &mask in WIN_MASKS.iter(){
        if (gameState & mask) & xo == mask {
            return (true, true);
        }
        if (gameState & mask) & !xo == mask {
            return (true, false);
        }
    }
    (false, false)
}

fn handleInput(inByte: u8, position: &mut(u16,u16), gameState:&mut u16, xo: &mut u16) -> bool{
 
    let c = inByte;
   
    if c == 3
        {
            // Ctrl + C
            crossterm::terminal::disable_raw_mode().unwrap();
            return true;
        }
    if c == 119{ 
        position.1 += 2;
        position.1 %= 3;
    }
    if c == 97{ 
        position.0 += 2;
        position.0 %= 3;
    }

   if c == 115{ 
        position.1 += 1;
        position.1 %= 3;
    }

   if c == 100{ 
        position.0 += 1;
        position.0 %= 3;
    }
    if c == 32{
        selectSquare(position.0,position.1, gameState, xo);
    }

    print!("handleInputLog: Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r\n", c, c as char);
       false
}

fn selectSquare(x: u16, y: u16, gameState: &mut u16, xo: &mut u16){
    let selected = x + 3 * y;// index of selection given x and y
    if *xo >> (8 - selected) & 1 > 0{
        //Area has already been set
        return;
    }
    let mut mask: u16 = 0b0000_0001_0000_0000;
    mask = mask >> selected; //mask for selecting the bit
    *gameState += mask;
    *xo += mask
    
}
