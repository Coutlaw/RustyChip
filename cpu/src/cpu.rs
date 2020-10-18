use crate::keyboard::Keyboard;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

// display unicode values
// empty pixel
// ▒
// full pixel
// █
const ACTIVE_PIXEL: char = '█';
const EMPTY_PIXEL: char = '▒';

pub struct OpCode {
    // processed opcodes
    pub op_1: usize,
    pub op_2: usize,
    pub op_3: usize,
    pub op_4: usize,

    // operation code variables
    pub nnn: usize,
    pub kk: u8,
    pub x: usize,
    pub y: usize,
    pub n: usize,
}

pub fn parse_op_codes_from_word(opcode: u16) -> OpCode {
    // opcode params
    // take the op code, mask its position, shift to the 0th place of the instruction

    // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
    let x = ((opcode & 0x0F00) >> 8) as usize;

    // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
    let y = ((opcode & 0x00F0) >> 4) as usize;

    // nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
    let nnn = (opcode & 0x0FFF) as usize;

    // n or nibble - A 4-bit value, the lowest 4 bits of the instruction
    let n = (opcode & 0x000F) as usize;

    // kk or byte - An 8-bit value, the lowest 8 bits of the instruction
    let kk = (opcode & 0x00FF) as u8;

    // extract the op code nibbles
    let op_1 = ((opcode & 0xF000) >> 12) as usize;
    let op_2 = ((opcode & 0x0F00) >> 8) as usize;
    let op_3 = ((opcode & 0x00F0) >> 4) as usize;
    let op_4 = (opcode & 0x000F) as usize;

    return OpCode {
        x,
        y,
        nnn,
        n,
        kk,
        op_1,
        op_2,
        op_3,
        op_4,
    };
}

pub struct Cpu {
    // index 16 bit register
    i: u16,

    // program counter
    pc: u16,

    // memory
    memory: [u8; 4096],

    // registers
    v: [u8; 16],

    // peripherals
    pub keyboard: Keyboard,
    pub display: [[char; SCREEN_HEIGHT]; SCREEN_WIDTH],

    // program stack
    stack: [u16; 16],

    // stack pointer
    sp: u8,

    // delay timer
    dt: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            display: [['▒'; SCREEN_HEIGHT]; SCREEN_WIDTH],
            keyboard: Keyboard::new(),
            stack: [0; 16],
            sp: 0,
            dt: 0,
        }
    }

    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = 0x200;
        self.memory = [0; 4096];
        self.v = [0; 16];
        self.stack = [0; 16];
        self.sp = 0;
        self.dt = 0;
        self.display = [['▒'; SCREEN_HEIGHT]; SCREEN_WIDTH]
    }

    pub fn execute_cycle(&mut self) {
        let opcode = self.read_word();
        self.handle_opcode(opcode);

        // Increment the PC by two 8 bit ops, or 1 word
        self.pc += 2;
    }

    // fn decrement_timers(&mut self) {
    //     if self.dt > 0 {
    //         self.dt -= 1;
    //     }
    // }

    // a word is 16 bits, so we combine two 8 bit chunks of memory to form one word
    fn read_word(&mut self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16)
    }

    fn handle_opcode(&mut self, opcode: u16) {
        let op_chunks = parse_op_codes_from_word(opcode);

        // match opcodes to a function that updates the CPU state
        match (
            op_chunks.op_1,
            op_chunks.op_2,
            op_chunks.op_3,
            op_chunks.op_4,
        ) {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(op_chunks.nnn),
            (0x02, _, _, _) => self.op_2nnn(op_chunks.nnn),
            (0x03, _, _, _) => self.op_3xkk(op_chunks.x, op_chunks.kk),
            (0x04, _, _, _) => self.op_4xkk(op_chunks.x, op_chunks.kk),
            (0x05, _, _, 0x00) => self.op_5xy0(op_chunks.x, op_chunks.y),
            (0x06, _, _, _) => self.op_6xkk(op_chunks.x, op_chunks.kk),
            (0x07, _, _, _) => self.op_7xkk(op_chunks.x, op_chunks.kk),
            (0x08, _, _, 0x00) => self.op_8xy0(op_chunks.x, op_chunks.y),
            (0x08, _, _, 0x01) => self.op_8xy1(op_chunks.x, op_chunks.y),
            (0x08, _, _, 0x02) => self.op_8xy2(op_chunks.x, op_chunks.y),
            (0x08, _, _, 0x03) => self.op_8xy3(op_chunks.x, op_chunks.y),
            (0x08, _, _, 0x04) => self.op_8xy4(op_chunks.x, op_chunks.y),
            (0x08, _, _, 0x05) => self.op_8xy5(op_chunks.x, op_chunks.y),
            (0x08, _, _, 0x06) => self.op_8x06(op_chunks.x),
            (0x08, _, _, 0x07) => self.op_8xy7(op_chunks.x, op_chunks.y),
            (0x08, _, _, 0x0E) => self.op_8x0e(op_chunks.x),
            (0x09, _, _, 0x00) => self.op_9xy0(op_chunks.x, op_chunks.y),
            (0x0A, _, _, _) => self.op_annn(op_chunks.nnn),
            (0x0B, _, _, _) => self.op_bnnn(op_chunks.nnn),
            (0x0C, _, _, _) => self.op_cxkk(op_chunks.x, op_chunks.kk),
            (0x0D, _, _, _) => self.op_dxyn(op_chunks.x, op_chunks.y, op_chunks.n),
            (0x0E, _, 0x09, 0x0E) => self.op_ex9e(op_chunks.x),
            (0x0E, _, 0x0A, 0x01) => self.op_exa1(op_chunks.x),
            // (0x0F, _, 0x00, 0x07) => self.op_fx07(op_chunks.x),
            // (0x0F, _, 0x00, 0x0A) => self.op_fx0a(op_chunks.x),
            // (0x0F, _, 0x01, 0x05) => self.op_fx15(op_chunks.x),
            // (0x0F, _, 0x01, 0x08) => self.op_fx18(op_chunks.x),
            // (0x0F, _, 0x01, 0x0e) => self.op_fx1e(op_chunks.x),
            // (0x0F, _, 0x02, 0x09) => self.op_fx29(op_chunks.x),
            // (0x0F, _, 0x03, 0x03) => self.op_fx33(op_chunks.x),
            // (0x0F, _, 0x05, 0x05) => self.op_fx55(op_chunks.x),
            // (0x0F, _, 0x06, 0x05) => self.op_fx65(op_chunks.x),
            _ => (println!("not implemented instruction")),
        }
    }

    // SYS
    fn op_00e0(&mut self) {
        println!("attempted to use 0nnn, this is ignored on modern interpreters")
    }

    // RET
    fn op_00ee(&mut self) {
        self.sp = self.sp - 1;
        self.pc = self.stack[self.sp as usize];
    }

    // Jp
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn as u16;
    }

    // CALL
    fn op_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp as usize] = self.pc;
        self.sp = self.sp + 1;
        self.pc = nnn as u16;
    }

    // SE Vx KK
    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 2
        };
    }

    // SNE Vx kk
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 2
        };
    }

    // SE Vx Vy
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2
        };
    }

    // LD Vx, byte
    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    //ADD Vx, byte
    fn op_7xkk(&mut self, x: usize, kk: u8) {
        self.v[x] += kk;
    }

    // LD Vx, Vy
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    // OR Vx, Vy
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] | self.v[y];
    }

    // AND Vx, Vy
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] & self.v[y];
    }

    // XOR Vx, Vy
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[x] ^ self.v[y];
    }

    // ADD Vx, Vy
    fn op_8xy4(&mut self, x: usize, y: usize) {
        match self.v[x].checked_add(self.v[y]) {
            Some(res) => {
                self.v[0xF] = 0;
                self.v[x] = res as u8;
            }
            None => {
                self.v[0xF] = 1;
                // We need the lower 8 bits of the result, so calculate as a u16 and convert
                self.v[x] = (self.v[x] as u16 + self.v[y] as u16) as u8;
            }
        }
    }

    // SUB Vx, Vy
    fn op_8xy5(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);

        // update Vf to NOT BORROW, meaning true if there was no borrow, false otherwise
        self.v[0xF] = !overflow as u8;

        // only take the 8 bit value
        self.v[x] = res as u8;
    }

    // SHR Vx {, Vy}
    fn op_8x06(&mut self, x: usize) {
        // find the bit value of the rightmost bit, convert to bool
        self.v[0xF] = self.v[x] & 1;
        // only take the 8 bit value
        self.v[x] = (self.v[x] / 2) as u8;
    }

    // SUBN Vx, Vy
    fn op_8xy7(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);

        // update Vf to NOT BORROW, meaning true if there was no borrow, false otherwise
        self.v[0xF] = !overflow as u8;

        // only take the 8 bit value
        self.v[x] = res as u8;
    }

    // SHL Vx {, Vy}
    fn op_8x0e(&mut self, x: usize) {
        // find the bit value of the leftmost bit (right 7 spaces for 8 bit int), convert to bool
        // if it is a 1, then set Vf to 1, else 0
        self.v[0xF] = (self.v[x] & (1 << 7)) >> 7;
        // only take the 8 bit value
        self.v[x] = (self.v[x] as u16 * 2) as u8;
    }

    // SNE Vx, Vy
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2
        }
    }

    // LD I, addr
    fn op_annn(&mut self, nnn: usize) {
        self.i = nnn as u16;
    }

    // JP V0, addr
    fn op_bnnn(&mut self, nnn: usize) {
        self.pc += self.v[0x0] as u16 + nnn as u16;
    }

    // RND Vx, byte
    fn op_cxkk(&mut self, x: usize, kk: u8) {
        use rand::Rng;
        // generate random value between 0-255, max range of u8
        let rand_bit: u8 = rand::thread_rng().gen();

        self.v[x] = kk & rand_bit
    }

    // DRW Vx, Vy, n
    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        // Vf self.v[0xF]

        // get the sprite out of memory by borrowing a slice of memory from i to i + n
        let sprite = &self.memory[self.i as usize .. (self.i + n as u16) as usize];

        // traverse ever memory address (each represents a row)
        for j in 0..sprite.len() {
            let row = &sprite[j];
            
            // go through each bit of the address
            for i in 0..8 {
                // starting with the left most bit, shift the bit all the way right
                // determine if its a 1 or 0
                let bit_value = row >> (7 - i) & 0x01;
                let pixel = if bit_value == 1 {
                    ACTIVE_PIXEL
                } else {
                    EMPTY_PIXEL
                };

                // determine the coordinates for the pixel
                // and check if it needs to wrap around the display
                let x_target = if x + i > SCREEN_WIDTH {
                    i
                } else {
                    x + i
                };
                let y_target = if y + i > SCREEN_HEIGHT {
                    i
                } else {
                    y + i
                };

                let display_target_pixel = &self.display[x_target][y_target];
                let target_is_active = display_target_pixel == &ACTIVE_PIXEL;
                let new_pixel_is_active = pixel == ACTIVE_PIXEL;

                // detect collision
                self.v[0xF] = (target_is_active && new_pixel_is_active) as u8;

                // draw value on the display
                self.display[x_target][y_target] = if (target_is_active as u8 ^ new_pixel_is_active as u8) == 1 {
                    ACTIVE_PIXEL
                } else {
                    EMPTY_PIXEL
                };
                
            }
        }
    }

    // SKP Vx
    fn op_ex9e(&mut self, x: usize) {
        if self.keyboard.key_is_pressed(self.v[x]) {
            self.pc += 2;
        }
    }

    // SKNP Vx
    fn op_exa1(&mut self, x: usize) {
        if !self.keyboard.key_is_pressed(self.v[x]) {
            self.pc += 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_jp() {
        let mut chip: Cpu = Cpu::new();
        chip.handle_opcode(0x1A2B);
        assert_eq!(chip.pc, 0x0A2B, "program counter was updated");
    }

    #[test]
    fn opcode_ret() {
        let mut chip: Cpu = Cpu::new();
        chip.sp += 1;
        chip.stack[0] = 1234;

        chip.handle_opcode(0x00EE);
        assert_eq!(chip.sp, 0x0000, "stack pointer was updated");
        assert_eq!(chip.pc, 1234, "program counter was updated");
    }

    #[test]
    fn opcode_2nnn() {
        let opcode = 0x2123;
        let nnn = (opcode & 0x0FFF) as u16;
        let mut chip: Cpu = Cpu::new();
        chip.pc += 10;
        chip.sp += 1;
        chip.stack[10] = 1234;

        chip.handle_opcode(opcode);
        assert_eq!(chip.stack[1], 10, "stack was updated");
        assert_eq!(chip.sp, 2, "stack pointer was updated");
        assert_eq!(chip.pc, nnn, "program counter was updated");
    }

    #[test]
    fn opcode_8xy4() {
        let opcode = 0x8124;
        let mut chip: Cpu = Cpu::new();
        chip.v[1] = 254;
        chip.v[2] = 3;

        chip.handle_opcode(opcode);
        assert_eq!(chip.v[0xF], 1, "overflow was detected, vf was updated");
        assert_eq!(chip.v[1], 1, "register Vx was updated");

        chip.reset();
        chip.v[1] = 251;
        chip.v[2] = 1;
        chip.handle_opcode(opcode);
        assert_eq!(chip.v[0xF], 0, "no overflow occurred, vf was updated");
        assert_eq!(chip.v[1], 252, "register Vx was updated");
    }

    #[test]
    fn opcode_8xy5() {
        let opcode = 0x8125;
        let mut chip: Cpu = Cpu::new();
        chip.v[1] = 0;
        chip.v[2] = 1;

        chip.handle_opcode(opcode);
        assert_eq!(
            chip.v[0xF], 0,
            "overflow was detected, vf was updated to NOT BORROW"
        );
        assert_eq!(chip.v[1], 255, "register Vx was updated");

        chip.reset();
        chip.v[1] = 3;
        chip.v[2] = 1;
        chip.handle_opcode(opcode);
        assert_eq!(
            chip.v[0xF], 1,
            "no overflow occurred, vf was updated to NOT BORROW"
        );
        assert_eq!(chip.v[1], 2, "register Vx was updated");
    }

    #[test]
    fn opcode_8xy6() {
        let opcode = 0x8126;
        let mut chip: Cpu = Cpu::new();
        chip.v[1] = 5;

        chip.handle_opcode(opcode);
        assert_eq!(chip.v[0xF], 1, "least significant bit is 1, Vf was updated");
        assert_eq!(chip.v[1], 2, "register Vx was updated");

        chip.reset();
        chip.v[1] = 2;
        chip.handle_opcode(opcode);
        assert_eq!(chip.v[0xF], 0, "lest significant bit is 0, Vf was updated");
        assert_eq!(chip.v[1], 1, "register Vx was updated");
    }

    #[test]
    fn opcode_8xy7() {
        let opcode = 0x8127;
        let mut chip: Cpu = Cpu::new();
        chip.v[1] = 1;
        chip.v[2] = 0;

        chip.handle_opcode(opcode);
        assert_eq!(
            chip.v[0xF], 0,
            "overflow was detected, vf was updated to NOT BORROW"
        );
        assert_eq!(chip.v[1], 255, "register Vx was updated");

        chip.reset();
        chip.v[1] = 1;
        chip.v[2] = 3;
        chip.handle_opcode(opcode);
        assert_eq!(
            chip.v[0xF], 1,
            "no overflow occurred, vf was updated to NOT BORROW"
        );
        assert_eq!(chip.v[1], 2, "register Vx was updated");
    }

    #[test]
    fn opcode_8xye() {
        let opcode = 0x812e;
        let mut chip: Cpu = Cpu::new();
        chip.v[1] = 128;

        chip.handle_opcode(opcode);
        assert_eq!(chip.v[0xF], 1, "Most significant bit is 1, Vf was updated");
        assert_eq!(
            chip.v[1], 0,
            "There was an overflow, register Vx was updated"
        );

        chip.reset();
        chip.v[1] = 2;
        chip.handle_opcode(opcode);
        assert_eq!(chip.v[0xF], 0, "most significant bit is 0, Vf was updated");
        assert_eq!(chip.v[1], 4, "register Vx was updated");
    }
}
