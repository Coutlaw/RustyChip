use op_code::{OpCode, Operations};

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
    pub keypad: Keypad,
    pub display: Display,

    // program stack
    pub stack: [u16; 16],

    // stack pointer
    pub sp: u8,

    // delay timer
    pub dt: u8,
}

// a word is 16 bits, so we combine two 8 bit chunks of memory to form one word
fn read_word(memory: [u8; 4096], index: u16) -> u16 {
    (memory[index as usize] as u16) << 8 | (memory[(index + 1) as usize] as u16)
}

impl Cpu {
    pub fn New() -> Cpu {
        Cpu {
            i: 0,
            pc: 0,
            memory: [0; 4096],
            v: [0; 16],
            display: Display::new(),
            keypad: Keypad::new(),
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
        self.rand = ComplementaryMultiplyWithCarryGen::new(1);
        self.display.cls();
        for i in 0..80 {
            self.memory[i] = FONT_SET[i];
        }
    }

    pub fn execute_cycle(&mut self) {
        let opcode = read_word(self.memory, self.pc);
        self.handle_opcode(opcode);
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    fn handle_opcode(&mut self, opcode: u16) {
        let OpCode { op_1, op_2, op_3, op_4, nnn, x, kk, y, n } = OpCode::parse_op_codes_from_word(op_code);

        self.pc += 2;

        // match opcodes to a function that updates the CPU state
        match (op_1, op_2, op_3, op_4) {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(nnn),
            (0x02, _, _, _) => self.op_2nnn(nnn),
            (0x03, _, _, _) => self.op_3xkk(x, kk),
            (0x04, _, _, _) => self.op_4xkk(x, kk),
            (0x05, _, _, 0x00) => self.op_5xy0(x, y),
            (0x06, _, _, _) => self.op_6xkk(x, kk),
            (0x07, _, _, _) => self.op_7xkk(x, kk),
            (0x08, _, _, 0x00) => self.op_8xy0(x, y),
            (0x08, _, _, 0x01) => self.op_8xy1(x, y),
            (0x08, _, _, 0x02) => self.op_8xy2(x, y),
            (0x08, _, _, 0x03) => self.op_8xy3(x, y),
            (0x08, _, _, 0x04) => self.op_8xy4(x, y),
            (0x08, _, _, 0x05) => self.op_8xy5(x, y),
            (0x08, _, _, 0x06) => self.op_8x06(x),
            (0x08, _, _, 0x07) => self.op_8xy7(x, y),
            (0x08, _, _, 0x0E) => self.op_8x0e(x),
            (0x09, _, _, 0x00) => self.op_9xy0(x, y),
            (0x0A, _, _, _) => self.op_annn(nnn),
            (0x0B, _, _, _) => self.op_bnnn(nnn),
            (0x0C, _, _, _) => self.op_cxkk(x, kk),
            (0x0D, _, _, _) => self.op_dxyn(x, y, n),
            (0x0E, _, 0x09, 0x0E) => self.op_ex9e(x),
            (0x0E, _, 0x0a, 0x01) => self.op_exa1(x),
            (0x0F, _, 0x00, 0x07) => self.op_fx07(x),
            (0x0F, _, 0x00, 0x0A) => self.op_fx0a(x),
            (0x0F, _, 0x01, 0x05) => self.op_fx15(x),
            (0x0F, _, 0x01, 0x08) => self.op_fx18(x),
            (0x0F, _, 0x01, 0x0e) => self.op_fx1e(x),
            (0x0F, _, 0x02, 0x09) => self.op_fx29(x),
            (0x0F, _, 0x03, 0x03) => self.op_fx33(x),
            (0x0F, _, 0x05, 0x05) => self.op_fx55(x),
            (0x0F, _, 0x06, 0x05) => self.op_fx65(x),
            _ => (),
        }
    }

    // RET
    fn op_00ee(&mut self) {
        self.sp = self.sp - 1;
        self.pc = self.stack[self.sp as usize];
    }

    // Jp
    fn op_1nnn(&mut self, nnn: usize) {
        self.pc = nnn;
    }

    // CALL
    fn op_2nnn(&mut self, nnn: usize) {
        self.stack[self.sp as uzise] = self.pc;
        self.sp = self.sp + 1;
        self.pc = nnn;
    }

    // SE Vx KK
    fn op_3xkk(&mut self, x: usize, kk: usize) {
        self.pc += if self.v[x] == kk { 2 } else { 0 };
    }
}