use std::usize;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SPRITE_CHARS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];



    fn lo_nib(b: u8) -> u8 {
        b & 0x0f
    }
macro_rules! nnn {
    ($w0:expr, $w1:expr) => {
        (($w0 & 0x0f) as u16) << 8 | $w1 as u16
    };
}

pub struct Chip8 {
    delay_timer: u8,
    sound_timer: u8,
    i: u16,
    registers: [u8; 16],
    memory: [u8; 4096],
    pub graphics: [bool; 64 * 32],
    stack: Vec<u16>,
    sp: u16,
    pc: u16,
    instruction: Instruction,
}

struct Instruction {
    pub instruction: u8,
    pub opcode: u16,
    pub w1: u8,
    pub w2: u8,
    pub x: u8,
    pub y: u8,
    pub n: u8,
    pub nn: u8,
    pub nnn: u16,
}

impl Instruction {
    fn read(&mut self, w1: u8, w2: u8) {
        println!("w1: {:#x}, w2: {:#X}", w1, w2);
        println!("Inst: {:#x}", ((w1 as u16) << 8) | (w2 as u16));
        self.opcode =((w1 as u16) << 8) | (w2 as u16);
        self.instruction = w1 & 0xf0;
        self.x = w1 & 0x0f;
        self.y = (w2 & 0xf0) >> 4;
        self.n = w2 & 0x0f;
        self.nn = w2;
        self.nnn = (((w1 & 0x0f) as u16) << 8) | w2 as u16;
        self.w1 = w1;
        self.w2 = w2
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut mem = [0_u8; 4096];
        mem[..80].copy_from_slice(&SPRITE_CHARS);
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
                opcode: 0,
                x: 0,
                y: 0,
                n: 0,
                nn: 0,
                nnn: 0,
                w1: 0,
                w2: 0,
            },
        }
    }

    pub fn execute(&mut self) {
        self.instruction.read(
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        );
        match self.instruction.instruction {
            0x00 => match self.instruction.w2 {
                0xe0 => self.op_clear_screen(),
                0xee => self.op_return(),
                _ => println!("No instruction found for {:#x}", self.instruction.w1),
            },
            0x10 => self.op_jump_addr(),
            0x60 => self.op_set_reg(),
            0x70 => self.op_add(),
            0xA0 => self.op_set_i_reg(),
            0xD0 => self.op_draw(),
            _ => {
                println!("No instruction found for {:#x}", self.instruction.w1)
            }
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        println!("Load Rom");
        
        self.memory[0x200..(0x200 as usize) + rom.len()].copy_from_slice(rom);
    }

    fn op_clear_screen(&mut self) {
        println!("Clear Screen");
        for i in self.graphics.iter_mut() {
            *i = false;
        }
        self.pc += 2
    }

    fn op_return(&mut self) {
        println!("Return");
        for i in self.graphics.iter_mut() {
            *i = false;
        }
        self.pc += 2
    }

    fn op_jump_addr(&mut self) {
        println!("Jump Address");
        self.pc = self.instruction.nnn;
    }

    fn op_set_reg(&mut self) {
        println!("Set Reg {:#x} as {:#x}", self.instruction.x, self.instruction.nn);
        self.registers[self.instruction.x as usize] = self.instruction.nn;
        self.pc += 2
    }

    fn op_add(&mut self) {
        println!("Add");
        let register_index = (self.instruction.w1 & 0x0f) as usize;
        let _ = self.registers[register_index].overflowing_add(self.instruction.w2); // TODO: Handle overflow?
        self.pc += 2;
    }

    fn op_set_i_reg(&mut self) {
        println!("Set I Reg to {:#x}", self.instruction.nnn);
        self.i = self.instruction.opcode & 0xFFF;//nnn!(self.instruction.w1, self.instruction.w2);
        println!("IIII: {:#x}", self.i);
        self.pc += 2
    }

    // Sprite is found correctly, with the locations provided, x/y coords seem to be incorrect when
    // drawwing
    fn op_draw(&mut self) {
        //println!("Draw");
        let x_coord = self.registers[((self.instruction.opcode & 0x0F00) >> 8) as usize];
        let y_coord = self.registers[((self.instruction.opcode & 0x00F0) >> 4) as usize];
        //let x_coord = self.registers[self.instruction.x as usize];
        //let y_coord = self.registers[self.instruction.y as usize];
        //println!("COORDINATES X: {:#} Y: {:#}", x_coord, y_coord);
        //self.registers[15] = 0;
        let mut flipped = false;
        let lines =  self.instruction.opcode & 0x000F;
        //println!("I as usign: {:#x} ", (self.i) as usize);
        //println!("LINES: {}", lines);
        for y_line in 0..self.instruction.n {
            // get ith byte of data from I from memory (sprinte)
            //println!("Drawing sprite at index {}", self.i + y_line as u16);
            let sprite = self.memory[(self.i + y_line as u16) as usize];
            //println!("I: {:#x} Sprite: {:#x}", self.i + y_line as u16, sprite);
            //println!("sprite {:#x}", self.i + y_line as u16);
            for x_line in 0..8 {
                let bit_on = sprite & (0b1000_0000 >> x_line);
                if bit_on != 0 {
                    // This is probable the issue
                    let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                    let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;
                    let idx = y * 64 + x;
                    flipped |= self.graphics[idx];
                    self.graphics[idx] ^= true;
                }
            }
        }
        if flipped {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        self.pc +=2;
    }
}
