use op_code::{OpCode, Operations};

enum ProgramCounter {
    Next,
    Skip,
    Jump(usize),
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
        let operation = OpCode::parse_op_codes_from_word(op_code);

        // TODO: match opcodes to instructions
        match operation.operations {
            Operations{ op_1: 0, op_2: 0, op_3: 0xE, op_4: 0} => println!("clear screen"),
            _ => println!("not implemented"),
        }

    }
}