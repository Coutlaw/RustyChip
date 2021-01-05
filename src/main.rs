extern crate minifb;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
// I hate this, but I would have to restructure the workspace into a lib crate ¯\_(ツ)_/¯
use cpu::cpu::Cpu;
use std::env;
use std::{
    thread,
    time::{Duration, Instant},
};

const EXECUTION_RATE: f32 = 0.06; // 60 hertz

fn get_chip8_keycode_for(key: Option<Key>) -> Option<u8> {
    match key {
        Some(Key::Key1) => Some(0x1),
        Some(Key::Key2) => Some(0x2),
        Some(Key::Key3) => Some(0x3),
        Some(Key::Key4) => Some(0xC),

        Some(Key::Q) => Some(0x4),
        Some(Key::W) => Some(0x5),
        Some(Key::E) => Some(0x6),
        Some(Key::R) => Some(0xD),

        Some(Key::A) => Some(0x7),
        Some(Key::S) => Some(0x8),
        Some(Key::D) => Some(0x9),
        Some(Key::F) => Some(0xE),

        Some(Key::Z) => Some(0xA),
        Some(Key::X) => Some(0x0),
        Some(Key::C) => Some(0xB),
        Some(Key::V) => Some(0xF),
        _ => None,
    }
}

fn main() {
    let width = 640;
    let height = 320;

    //ARGB buffer
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "Rust Chip8 emulator",
        width,
        height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Window creation failed: {:?}", e);
    });


    // my CPU
    let mut cpu =  Cpu::new();
    let mut i = 0;

    // TODO: add logic to load game into cpu memory
    let args: Vec<String> = env::args().collect();
    let rom = &args[1];

    //println!("{} rom loaded", rom);
    cpu.load_game(rom);
    
    // begin executing instructions and updating the display
    loop {
        // // tracking start time of cycle
        // let dur = Duration::from_secs_f32(EXECUTION_RATE);
        // let start = Instant::now();

        i+=1;
        cpu.execute_cycle();


            for y in 0..height {
                let y_coord = y / 10;
                for x in 0..width {
                    let x_cord = x / 10;
                    let pixel = cpu.display[y_coord][x_cord];
                    
                    let color_pixel = match pixel {
                        0 => 0x0,
                        1 => 0xffffff,
                        _ => {println!("pixel value {}", pixel); 0x0},
                    };
                    buffer[(y * width) + x] = color_pixel;
                }
            }

            let _ = window.update_with_buffer(&buffer);
        //TODO: detect keypress events, map to Chip-8 keyboard
        // update the chips keyboard state

        // // calculate the time it took to execute the instruction
        // let runtime = start.elapsed();

        // // limit functions to 60 hertz
        // if let Some(remaining) = dur.checked_sub(runtime) {
        //     thread::sleep(remaining);
        // }
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
