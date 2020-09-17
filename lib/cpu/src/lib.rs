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
    let n = ((opcode & 0x000F)) as usize;

    // kk or byte - An 8-bit value, the lowest 8 bits of the instruction
    let kk = (opcode & 0x00FF) as u8;

    // extract the op code nibbles
    let op_1 = ((opcode & 0xF000) >> 12) as usize;
    let op_2 = ((opcode & 0x0F00) >> 8) as usize;
    let op_3 = ((opcode & 0x00F0) >> 4) as usize;
    let op_4 = (opcode & 0x000F) as usize;
    
    return OpCode { x, y, nnn, n, kk, op_1, op_2, op_3, op_4 };
}

pub struct Cpu {
    // index 16 bit register
    pub i: u16,

    // program counter
    pub pc: u16,

    // memory
    pub memory: [u8; 4096],

    // registers
    pub v: [u8; 16],

    // peripherals
    // pub keypad: Keypad,
    // pub display: Display,

    // program stack
    pub stack: [u16; 16],

    // stack pointer
    pub sp: u8,

    // delay timer
    pub dt: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            i: 0,
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            // TODO: fix
            // display: Display::new(),
            // keypad: Keypad::new(),
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
        //self.rand = ComplementaryMultiplyWithCarryGen::new(1);
        // TODO: impl display
        //self.display.cls();
        // for i in 0..80 {
        //     self.memory[i] = FONT_SET[i];
        // }
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
        match (op_chunks.op_1, op_chunks.op_2, op_chunks.op_3, op_chunks.op_4) {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(op_chunks.nnn),
            (0x02, _, _, _) => self.op_2nnn(op_chunks.nnn),
            (0x03, _, _, _) => self.op_3xkk(op_chunks.x, op_chunks.kk),
            (0x04, _, _, _) => self.op_4xkk(op_chunks.x,op_chunks.kk),
            // (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            // (0x06, _, _, _) => self.op_6xkk(x, kk),
            // (0x07, _, _, _) => self.op_7xkk(x, kk),
            // (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            // (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            // (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            // (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            // (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            // (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            // (0x08, _, _, 0x06) => self.op_8x06(x),
            // (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            // (0x08, _, _, 0x0E) => self.op_8x0e(x),
            // (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            // (0x0A, _, _, _) => self.op_annn(nnn),
            // (0x0B, _, _, _) => self.op_bnnn(nnn),
            // (0x0C, _, _, _) => self.op_cxkk(x, kk),
            // (0x0D, _, _, _) => self.op_dxyn(x, y, n),
            // (0x0E, _, 0x09, 0x0E) => self.op_ex9e(x),
            // (0x0E, _, 0x0a, 0x01) => self.op_exa1(x),
            // (0x0F, _, 0x00, 0x07) => self.op_fx07(x),
            // (0x0F, _, 0x00, 0x0A) => self.op_fx0a(x),
            // (0x0F, _, 0x01, 0x05) => self.op_fx15(x),
            // (0x0F, _, 0x01, 0x08) => self.op_fx18(x),
            // (0x0F, _, 0x01, 0x0e) => self.op_fx1e(x),
            // (0x0F, _, 0x02, 0x09) => self.op_fx29(x),
            // (0x0F, _, 0x03, 0x03) => self.op_fx33(x),
            // (0x0F, _, 0x05, 0x05) => self.op_fx55(x),
            // (0x0F, _, 0x06, 0x05) => self.op_fx65(x),
            _ => (println!("not implemented instruction")),
        }
    }

    // SYS
    fn op_00e0(&mut self) {
        println!("attempted to use 0nnn, this is ignored on modern interpretes")
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
        if self.v[x] == kk { self.pc += 2 };
    }

    // SNE Vx kk
    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk { self.pc += 2 };
    }

    // SE Vx Vy
    // fn op_5xy0(&mut self, x: usize, y: usize) {

    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_jp(){
        let mut chip: Cpu = Cpu::new();
        chip.handle_opcode(0x1A2A);
        assert_eq!(chip.pc, 0x0A2A, "program counter was updated");
    }
}