// I hate this, but I would have to restructure the workspace into a lib crate ¯\_(ツ)_/¯
use cpu::cpu::Cpu;
use std::{thread, time};
use std::env;

fn main() {
    // my CPU
    let mut cpu =  Cpu::new();
    let mut i = 0;

    // TODO: add logic to load game into cpu memory
    let args: Vec<String> = env::args().collect();
    let rom = &args[1];

    //println!("{} rom loaded", rom);
    cpu.load_game(rom);

    let mut output = String::new();
    
    // begin executing instructions and updating the display
    loop {
        i+=1;
        cpu.execute_cycle();

        // display logic
        for column in cpu.display.iter() {
            for value in column.iter() {
                if *value != 0 {
                    output.push('█');
                    //print!("{} ", '█');
                } else {
                    output.push('▒');
                    //print!("{} ", '▒');
                }
            }
            output.push('\n');
            //println!();
        }
        println!("{}", output);
        println!("Frame {}", i);
        output.clear();

        //TODO: detect keypress events, map to Chip-8 keyboard
        // update the chips keyboard state
    }
}

// convert a keypress into a chip 8 key value if possible
// fn char_to_key(key: char) -> Option<u8> {
//     if let Some(x) = key.to_digit(10) {
//         if x <= 16 {
//             return Some(x as u8)
//         }
//     }
//     None
// }


// display unicode values
// empty pixel
// ▒
// full pixel
// █
