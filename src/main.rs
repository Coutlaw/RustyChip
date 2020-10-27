use cpu::Cpu;
fn main() {
    let test =  Cpu::new();

//     // Set up render system and register input callbacks
//   setupGraphics();
//   setupInput();
 
//   // Initialize the Chip8 system and load the game into the memory  
//   myChip8.initialize();
//   myChip8.loadGame("pong");
 
//   // Emulation loop
//   for(;;)
//   {
//     // Emulate one cycle
//     myChip8.emulateCycle();
 
//     // If the draw flag is set, update the screen
//     if(myChip8.drawFlag)
//       drawGraphics();
 
//     // Store key press state (Press and Release)
//     myChip8.setKeys();	
//   }
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


// display unicode values
// empty pixel
// ▒
// full pixel
// █
