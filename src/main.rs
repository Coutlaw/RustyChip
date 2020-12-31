use cpu::cpu::Cpu;
use std::io::{stdin, stdout, Read, Write};

fn main() {
    // my CPU
    let cpu =  Cpu::new();

    // TODO: add logic to load game into cpu memory
    
    
    // begin executing instructions and updating the display
    loop {
        cpu.execute_cycle();

        /*
Just a short answer because I‘m on mobile.

You don‘t need to index. This works:

   for (i, row) in grid.iter_mut().enumerate() {
        for (y, col) in row.iter_mut().enumerate() {
            println!("{}", col);
        }
    }
Edit:
If you want to avoid the nested loops and just want to do something with each element, you can use flat_map:

    for element in grid.iter_mut().flat_map(|r| r.iter_mut()) {
        println!("{}", element);
    }
        */
        // display logic
        for i in cpu.display.enumerate() {
            for value in column.enumerate() {
                if value != 0 {
                    print!("{}", '█');
                } else {
                    print!("{}", '▒');
                }
            }
        }

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
