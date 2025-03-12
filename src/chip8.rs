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
    pc: u16
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
            stack: Vec::new()
        }
    }

    pub fn run(&mut self) {
        while (self.sp as usize) < self.memory.len() {
            let word1 = self.memory[self.pc as usize];
            let word2 = self.memory[(self.pc + 1) as usize];
            match word1 & 0xf0 {
                0x00 => match word2 {
                    0xe0 => self.op_clear_screen(),
                    0xee => self.op_return(),
                    _ => println!("not found")
                },
                0x10 => self.op_jump_addr(word1, word2),
                0x60 => self.op_set_reg(word1, word2), 
                0x70 => self.op_add(word1, word2),
                0xA0 => self.op_set_i_reg(word2),
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
        self.registers[register_index as usize] = word2; // TODO: this as useize might not work
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






