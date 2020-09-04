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
        self.process_opcode(opcode);
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
    }

    fn process_opcode(&mut self, opcode: u16) {

        // opcode params
        // take the op code, mask its position, shift to the 0th place of the instruction

        // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        let x = ((opcode & 0x0F00) >> 8) as usize;

        // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        let y = ((opcode & 0x00F0) >> 4) as usize;

        // nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
        let nnn = (opcode & 0x0FFF);

        // n or nibble - A 4-bit value, the lowest 4 bits of the instruction
        let n = ((opcode & 0x000F)) as u8;

        // kk or byte - An 8-bit value, the lowest 8 bits of the instruction
        let kk = (opcode & 0x00FF) as u8;

        // extract the smaller nibbles
        let op_1 = ((opcode & 0xF000) >> 12) as u8;
        let op_2 = ((opcode & 0x0F00) >> 8)as u8;
        let op_3 = ((opcode & 0x00F0) >> 4)as u8;
        let op_4 = (opcode & 0x000F) as u8;

        // TODO: match opcodes to instructions


    }
}