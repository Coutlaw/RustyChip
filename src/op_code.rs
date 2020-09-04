struct OpCode {
    pub operations: Operations,

    // operation code variables
    pub nnn: usize,
    pub kk: u8,
    pub x: usize,
    pub y: usize,
    pub n: usize,
}

struct Operations {
    // 4 decoded ops from the word
    pub op_1: usize,
    pub op_2: usize,
    pub op_3: usize,
    pub op_4: usize,
}

impl Operations {
    fn parse_operations(&mut self, opcode: u16) {
        self.op_1 = ((opcode & 0xF000) >> 12) as u8;
        self.op_2 = ((opcode & 0x0F00) >> 8)as u8;
        self.op_3 = ((opcode & 0x00F0) >> 4)as u8;
        self.op_4 = (opcode & 0x000F) as u8;
    }
}

impl OpCode {
    pub fn parse_op_codes_from_word(&mut self, opcode: u16) -> OpCode {
        // opcode params
        // take the op code, mask its position, shift to the 0th place of the instruction

        // x - A 4-bit value, the lower 4 bits of the high byte of the instruction
        self.x = ((opcode & 0x0F00) >> 8) as usize;

        // y - A 4-bit value, the upper 4 bits of the low byte of the instruction
        self.y = ((opcode & 0x00F0) >> 4) as usize;

        // nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
        self.nnn = (opcode & 0x0FFF);

        // n or nibble - A 4-bit value, the lowest 4 bits of the instruction
        self.n = ((opcode & 0x000F)) as u8;

        // kk or byte - An 8-bit value, the lowest 8 bits of the instruction
        self.kk = (opcode & 0x00FF) as u8;

        // extract the smaller nibbles
        self.operations = Operations::parse_operations(op_code);
        
        self;
    }
}