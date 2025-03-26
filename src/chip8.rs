use rand::Rng;

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
    reg: [u8; 16],
    memory: [u8; 4096],
    pub graphics: [bool; 64 * 32],
    stack: [u16; 8],
    sp: u16,
    pc: u16,
    inst: Instruction,
    keys: [bool; 16],
}

struct Instruction {
    pub inst: u8,
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
        self.opcode = ((w1 as u16) << 8) | (w2 as u16);
        self.inst = w1 & 0xf0;
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
            reg: [0; 16],
            memory: mem,
            graphics: [false; 64 * 32],
            stack: [0; 8],
            inst: Instruction {
                inst: 0,
                opcode: 0,
                x: 0,
                y: 0,
                n: 0,
                nn: 0,
                nnn: 0,
                w1: 0,
                w2: 0,
            },
            keys: [false; 16],
        }
    }

    pub fn execute(&mut self) {
        self.inst.read(
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        );
        match self.inst.inst{
            0x00 => match self.inst.w2 {
                0xe0 => self.op_clear_screen(),
                0xee => self.op_return(),
                _ => println!("No inst found for {:#x}", self.inst.w1),
            },
            0x10 => self.op_jump_addr(),
            0x20 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.inst.nnn;
            }
            0x30 => self.skip_if_reg_eq(),
            0x40 => self.skip_if_reg_neq(),
            0x50 => self.skip_if_reg_v_x_eq(),
            0x60 => self.op_set_reg(),
            0x70 => self.op_add(),
            0x80 => self.bitwise_operations(),
            0x90 => {
                if self.reg[self.inst.y as usize] != self.reg[self.inst.x as usize] {
                    self.pc += 4;   
                } else {
                    self.pc += 2;   
                }
            }
            0xA0 => self.op_set_i_reg(),
            0xB0 => {
                self.pc = self.inst.nnn + self.reg[0] as u16;
            },
            0xC0 => {
                self.reg[self.inst.x as usize] = rand::rng().random_range(0..255) & self.inst.nn;
            }
            0xE0 => {
                match self.inst.w2 { //TODO: this could be wrong
                    0x90 => {
                        if self.keys[self.reg[self.inst.x as usize] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    0xA1 => {
                        if !self.keys[self.reg[self.inst.x as usize] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        println!("No inst found for {:#x}", self.inst.w1)
                    }
                }
            }
            0xF0 => {
                match self.inst.w2 {
                    0x07 => {
                        self.reg[self.inst.x as usize] = self.delay_timer;
                        self.pc += 2;
                    },
                    0x0A => {
                        let mut key_pressed = false;
                        for i in 0..16 {
                            if self.keys[i] {
                                self.reg[self.inst.x as usize] = i as u8;
                                key_pressed = true;
                            }
                        }
                        if !key_pressed {
                            return;
                        }
                        self.pc += 2;
                    },
                    0x0015 => {
                        self.delay_timer = self.reg[self.inst.x as usize];
                        self.pc += 2;
                    }
                    0x0018 => {
                        self.sound_timer = self.reg[self.inst.x as usize];
                        self.pc += 2;
                    }
                    0x001E => {
                        let (res, overflow) = self.i.overflowing_add(self.reg[self.inst.x as usize] as u16);
                        if overflow {
                            self.reg[15] = 1;
                        } else {
                            self.reg[15] = 0;
                        }
                        self.i = res;
                        self.pc += 2;
                    }
                    0x029 => {
                        self.i = self.reg[self.inst.x as usize] as u16 * 5;
                        self.pc += 2;
                    }
                    // below all wrong
                    0x0033 => {
                        let vx = self.reg[self.inst.x as usize] as f32;

                        let hundreds = (vx / 100.0).floor() as u8;
                        let tens = ((vx / 10.0) % 10.0).floor() as u8;
                        let ones = (vx % 10.0) as u8;

                        self.memory[self.i as usize] = hundreds;
                        self.memory[(self.i+ 1) as usize] = tens;
                        self.memory[(self.i+ 2) as usize] = ones;
                        self.pc += 2;
                    }
                    0x0055 => {
                        let x = self.inst.x as usize;
                        println!("x: {:#x}", x);
                        //let i = self.i as usize;
                        //println!("i: {:#x}", i);
                        for idx in 0..=x {
                            //println!("idx: {:#x}", idx);
                            //self.memory[i + idx] = self.reg[idx];
                        }
                        for i in 0..x + 1 {
                            self.memory[(self.i as usize) + i] = self.reg[i]; 
                        }
                        self.pc += 2;
                        //panic!("Not implemented");
                    }
                    0x0065 => {

                        let vx = self.inst.x as usize;
                        for i in 0..=vx {
                        }
                        //self.i = self.i + self.inst.x as u16 + 1;
                        self.pc += 2;
                    }
                    _ => {
                        println!("No inst found for {:#x}", self.inst.w1)
                    }
                }
            }
            0xD0 => self.op_draw(),
            _ => {
                println!("No inst found for {:#x}", self.inst.w1)
            }
        }
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {

            if self.sound_timer == 1 {
                // TODO: Implement sound
            }
            self.sound_timer -= 1;
        }
    }


    pub fn load_rom(&mut self, rom: &[u8]) {
        println!("Load Rom");

        self.memory[0x200..(0x200_usize) + rom.len()].copy_from_slice(rom);
    }

    fn bitwise_operations(&mut self) {
        match self.inst.n {
            0x0000 => {
                self.reg[self.inst.x as usize] = self.reg[self.inst.y as usize];
                self.pc += 2;   
            }
            0x0001 => {
                self.reg[self.inst.x as usize] |= self.reg[self.inst.y as usize];
                self.pc += 2;   
            }
            0x0002 => {
                self.reg[self.inst.x as usize] &= self.reg[self.inst.y as usize];
                self.pc += 2;   
            }
            0x0003 => {
                self.reg[self.inst.x as usize] ^= self.reg[self.inst.y as usize];
                self.pc += 2;   
            }
            0x0004 => {
                let (res, overflow) = self.reg[self.inst.x as usize].overflowing_add(self.reg[self.inst.y as usize]);
                self.reg[self.inst.x as usize] = res;
                if overflow {
                    self.reg[15] = 1;
                } else {
                    self.reg[15] = 0;
                }
                self.pc += 2;   
            }
            0x0005 => {
                let (res, overflow) = self.reg[self.inst.x as usize].overflowing_sub(self.reg[self.inst.y as usize]);
                if overflow {
                    self.reg[15] = 1;
                } else {
                    self.reg[15] = 0;
                }
                self.reg[self.inst.x as usize] = res;
                self.pc += 2;   
            }
            0x0006 => {
                self.reg[15] = self.reg[self.inst.x as usize] & 0x1;
                self.reg[self.inst.x as usize] >>= 1;
                self.pc += 2;   
            }
            0x0007 => {
                let (res, overflow) = self.reg[self.inst.y as usize].overflowing_sub(self.reg[self.inst.x as usize]);
                if overflow {
                    self.reg[15] = 0;
                } else {
                    self.reg[15] = 1;
                }
                self.reg[self.inst.x as usize] = res;
                self.pc += 2;   
            }
            0x000e => {
                self.reg[15] = self.reg[self.inst.x as usize] >> 7;
                self.reg[self.inst.x as usize] <<= 1;
                self.pc += 2;
            }
            _ => {
                println!()
            }

        }
    }

    fn skip_if_reg_v_x_eq(&mut self) {
        if self.reg[self.inst.y as usize] == self.reg[self.inst.x as usize] {
            self.pc += 4;   
        } else {
            self.pc += 2;   
        }
    }

    fn skip_if_reg_neq(&mut self) {
        if self.reg[self.inst.x as usize] == self.inst.nn {
            self.pc += 2;   
        } else {
            self.pc += 4;   
        }
    }

    fn skip_if_reg_eq(&mut self) {
        if self.reg[self.inst.x as usize] == self.inst.nn {
            self.pc += 4;   
        } else {
            self.pc += 2;   
        }
    }

    fn op_clear_screen(&mut self) {
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
        self.pc = self.inst.nnn;
    }

    fn op_set_reg(&mut self) {
        self.reg[self.inst.x as usize] = self.inst.nn;
        self.pc += 2
    }

    // wasn't adding this correctly due to overflow'
    fn op_add(&mut self) {
        let (res, overflow) =
            self.reg[self.inst.x as usize].overflowing_add(self.inst.w2);
        if overflow {
            // do something
        }
        self.reg[self.inst.x as usize] = res;
        self.pc += 2;
    }

    fn op_set_i_reg(&mut self) {
        println!("Set I to {:#x}", self.inst.nnn);
        self.i = self.inst.nnn;
        self.pc += 2
    }
    fn op_draw(&mut self) {

        self.reg[15] = 0;

        let x_coord = self.reg[self.inst.x as usize];
        let y_coord = self.reg[self.inst.y as usize];
        let mut flipped = false;

        for y_line in 0..self.inst.n {
            let sprite = self.memory[(self.i + y_line as u16) as usize];
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
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }
        self.pc += 2;
    }
}
