pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const SPRITE_CHARS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];


pub struct Chip8 {
    delay_timer: u8,
    sound_timer: u8,
    i: u16,
    registers: [u8; 16],
    memory: [u8; 4096],
    pub graphics: [bool; 64 * 32],
    stack: [u16; 8],
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
        self.opcode = ((w1 as u16) << 8) | (w2 as u16);
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
            stack: [0; 8],
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
            0x20 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.instruction.nnn;
            }
            0x30 => self.skip_if_reg_eq(),
            0x40 => self.skip_if_reg_neq(),
            0x50 => self.skip_if_reg_v_x_eq(),
            0x60 => self.op_set_reg(),
            0x70 => self.op_add(),
            0x80 => match self.instruction.n {
                0x0000 => {
                    self.registers[self.instruction.x as usize] = self.registers[self.instruction.y as usize];
                    self.pc += 2;   
                }
                0x0001 => {
                    self.registers[self.instruction.x as usize] |= self.registers[self.instruction.y as usize];
                    self.pc += 2;   
                }
                0x0002 => {
                    self.registers[self.instruction.x as usize] &= self.registers[self.instruction.y as usize];
                    self.pc += 2;   
                }
                0x0003 => {
                    self.registers[self.instruction.x as usize] ^= self.registers[self.instruction.y as usize];
                    self.pc += 2;   
                }
                0x0004 => {
                    let (res, overflow) = self.registers[self.instruction.x as usize].overflowing_add(self.registers[self.instruction.y as usize]);
                    self.registers[self.instruction.x as usize] = res;
                    if overflow {
                        self.registers[15] = 1;
                    } else {
                        self.registers[15] = 0;
                    }

                    let x = self.registers[self.instruction.x as usize] as u16;
                    let y = self.registers[self.instruction.y as usize] as u16;
                    if x + y > 255 {
                       // self.registers[15] = 1;
                    } else {
                        //self.registers[15] = 0;
                    }
                    self.pc += 2;   
                }
                0x0005 => {
                    let (res, overflow) = self.registers[self.instruction.x as usize].overflowing_sub(self.registers[self.instruction.y as usize]);
                    if overflow {
                        self.registers[15] = 1;
                    } else {
                        self.registers[15] = 0;
                    }
                    self.registers[self.instruction.x as usize] = res;
                    self.pc += 2;   
                }
                0x0006 => {
                    self.registers[15] = self.registers[self.instruction.x as usize] & 0x1;
                    self.registers[self.instruction.x as usize] >>= 1;
                    self.pc += 2;   
                }
                0x0007 => {
                    let (res, overflow) = self.registers[self.instruction.y as usize].overflowing_sub(self.registers[self.instruction.x as usize]);
                    if overflow {
                        self.registers[15] = 0;
                    } else {
                        self.registers[15] = 1;
                    }
                    self.registers[self.instruction.x as usize] = res;
                    self.pc += 2;   
                }
                0x000e => {
                    self.registers[15] = self.registers[self.instruction.x as usize] >> 7;
                    self.registers[self.instruction.x as usize] <<= 1;
                    self.pc += 2;
                }
                _ => {
                    println!()
                }
            }
            0x90 => {
                if self.registers[self.instruction.y as usize] != self.registers[self.instruction.x as usize] {
                    self.pc += 4;   
                } else {
                    self.pc += 2;   
                }
            }
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

    fn skip_if_reg_v_x_eq(&mut self) {
        if self.registers[self.instruction.y as usize] == self.registers[self.instruction.x as usize] {
            self.pc += 4;   
        } else {
            self.pc += 2;   
        }
    }

    fn skip_if_reg_neq(&mut self) {
        if self.registers[self.instruction.x as usize] == self.instruction.nn {
            self.pc += 2;   
        } else {
            self.pc += 4;   
        }
    }

    fn skip_if_reg_eq(&mut self) {
        if self.registers[self.instruction.x as usize] == self.instruction.nn {
            self.pc += 4;   
        } else {
            self.pc += 2;   
        }
    }

    fn op_clear_screen(&mut self) {
        println!("Clear Screen");
        for i in self.graphics.iter_mut() {
            *i = false;
        }
        self.pc += 2
    }

    fn op_return(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc += 2
    }

    fn op_jump_addr(&mut self) {
        println!("Jump Address");
        self.pc = self.instruction.nnn;
    }

    fn op_set_reg(&mut self) {
        println!(
            "Set Reg {:#x} as {:#x}",
            self.instruction.x, self.instruction.nn
        );
        self.registers[self.instruction.x as usize] = self.instruction.nn;
        self.pc += 2
    }

    // wasn't adding this correctly due to overflow'
    fn op_add(&mut self) {
        println!("Add");
        let (res, overflow) =
            self.registers[self.instruction.x as usize].overflowing_add(self.instruction.w2);
        if overflow {
            // do something
        }
        self.registers[self.instruction.x as usize] = res;
        self.pc += 2;
    }

    fn op_set_i_reg(&mut self) {
        println!("Set I Reg to {:#x}", self.instruction.nnn);
        self.i = self.instruction.nnn;
        println!("IIII: {:#x}", self.i);
        self.pc += 2
    }
    fn op_draw(&mut self) {
        println!("Draw");

        self.registers[15] = 0;

        let x_coord = self.registers[self.instruction.x as usize];
        let y_coord = self.registers[self.instruction.y as usize];
        let mut flipped = false;

        for y_line in 0..self.instruction.n {
            let sprite = self.memory[(self.i + y_line as u16) as usize];
            println!("I: {:#x} Sprite: {:#x}", self.i + y_line as u16, sprite);
            for x_line in 0..8 {
                let bit_on = sprite & (0b1000_0000 >> x_line);
                if bit_on != 0 {
                    let x = (x_coord + x_line) as usize % SCREEN_WIDTH;
                    let y = (y_coord + y_line) as usize % SCREEN_HEIGHT;
                    let idx = y * SCREEN_WIDTH + x;
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
        self.pc += 2;
    }
}
