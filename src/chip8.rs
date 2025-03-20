const SPRITE_CHARS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 
    0x20, 0x60, 0x20, 0x20, 0x70, 
    0xF0, 0x10, 0xF0, 0x80, 0xF0, 
    0xF0, 0x10, 0xF0, 0x10, 0xF0, 
    0x90, 0x90, 0xF0, 0x10, 0x10, 
    0xF0, 0x80, 0xF0, 0x10, 0xF0, 
    0xF0, 0x80, 0xF0, 0x90, 0xF0, 
    0xF0, 0x10, 0x20, 0x40, 0x40, 
    0xF0, 0x90, 0xF0, 0x90, 0xF0, 
    0xF0, 0x90, 0xF0, 0x10, 0xF0, 
    0xF0, 0x90, 0xF0, 0x90, 0x90, 
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0, 
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80, 
];

pub struct Chip8 {
    delay_timer: u8,
    sound_timer: u8,
    i: u16,
    registers: [u8; 16],
    memory: [u8; 4096],
    graphics: [bool; 64 * 32],
    stack: Vec<u16>,
    sp: u16,
    pc: u16,
    instruction: Instruction
}

struct Instruction {
    pub instruction: u8,
    pub w2: u8,
    pub w2: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

impl Instruction {
    fn init(&mut self, w1: u8, w2: u8) {
        self.instruction = w1 & 0xf0;
        self.x = w1 & 0x0f;
        self.y = (w2 & 0xf0) >> 4;
        self.n = w2 & 0x0f;
        self.nn = w2;
        self.nnn = (((w1 & 0x0f) as u16) << 8) | w2 as u16;
    }
}


impl Chip8 {
    pub fn new() -> Chip8 {
        let mut mem = [0_u8; 4096];
        for(i, sprint) in SPRITE_CHARS.iter().enumerate() {
            mem[i] = *sprint
        }
        Chip8 {
            sp: 0,
            pc: 0x200,
            delay_timer: 0,
            sound_timer: 8,
            i: 0,
            registers: [0; 16],
            memory: mem,
            graphics: [false; 64 * 32],
            stack: Vec::new(),
            instruction: Instruction {
                instruction: 0,
                x: 0,
                y: 0,
                n: 0,
                nn: 0,
                nnn: 0
            }
        }
    }

    pub fn execute(&mut self) {

        while (self.sp as usize) < self.memory.len() {
            self.instruction.init(self.memory[self.pc as usize], self.memory[(self.pc + 1) as usize]);
            match self.instruction.instruction {
                0x00 => match self.instruction.w2 {
                    0xe0 => self.op_clear_screen(),
                    0xee => self.op_return(),
                    _ => println!("not found")
                },
                0x10 => self.op_jump_addr(),
                0x60 => self.op_set_reg(), 
                0x70 => self.op_add(),
                0xA0 => self.op_set_i_reg(),
                0xD0 => self.op_draw(),
                _ =>  {},
            }
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(rom);
    }

    fn op_clear_screen(&mut self) {
        for i in self.graphics.iter_mut() {
            *i = false;
        }
        self.pc +=2
    }

    fn op_return(&mut self) {
        for i in self.graphics.iter_mut() {
            *i = false;
        }
        self.pc +=2
    }

    fn op_jump_addr(&mut self, word1: u8, word2: u8) {
        self.pc = (((word1 & 0x0f) as u16) << 8u16) | word2 as u16;
    }

    fn op_set_reg(&mut self, word1: u8, word2: u8) {
        let register_index = word1 & 0x0f;
        self.registers[register_index as usize] = word2; 
        self.pc +=2
    }

     fn op_add(&mut self, word1: u8, word2: u8) {
        let register_index = (word1 & 0x0f) as usize;
        let _ = self.registers[register_index].overflowing_add(word2); // TODO: Handle overflow?
        self.pc += 2; 
    }

    fn op_set_i_reg(&mut self, word2: u8) {
        self.i = word2 as u16; 
        self.pc +=2
    }

    fn op_draw(&mut self) {
        self.pc +=2;
    }
}






