use cpu::Cpu;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    // my CPU
    let cpu =  Cpu::new();

    // TODO: add logic to load game into cpu memory
    
    
    // begin executing instructions and updating the display
    while true {
        cpu.execute_cycle();

        // display, an array of bits
        //cpu.display

        //TODO: detect keypress events, map to Chip-8 keyboard
        // update the chips keyboard state
    }
}

// convert a keypress into a chip 8 key value if possible
fn char_to_key(key: char) -> Option<u8> {
    if let Some(x) = key.to_digit(10) {
        if x <= 16 {
            return Some(x as u8)
        }
    }
    None
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}


// display unicode values
// empty pixel
// ▒
// full pixel
// █
