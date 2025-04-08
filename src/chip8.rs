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

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0x200..(0x200_usize) + rom.len()].copy_from_slice(rom);
    }

    pub fn key_down(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn key_up(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                //sound not implemented
            }
            self.sound_timer -= 1;
        }
    }

    pub fn execute(&mut self) {
        self.inst.read(
            self.memory[self.pc as usize],
            self.memory[(self.pc + 1) as usize],
        );
        match self.inst.inst {
            0x00 => match self.inst.w2 {
                0xe0 => {
                    for i in self.graphics.iter_mut() {
                        *i = false;
                    }
                    self.pc += 2
                }
                0xee => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2
                }
                _ => println!("No inst found for {:#x}", self.inst.w1),
            },
            0x10 => {
                self.pc = self.inst.opcode & 0x0FFF;
            }
            0x20 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.inst.nnn;
            }
            0x30 => {
                if self.reg[self.inst.x as usize] == self.inst.nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x40 => {
                if self.reg[self.inst.x as usize] == self.inst.nn {
                    self.pc += 2;
                } else {
                    self.pc += 4;
                }
            }
            0x50 => {
                if self.reg[self.inst.y as usize] == self.reg[self.inst.x as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x60 => {
                self.reg[self.inst.x as usize] = self.inst.nn;
                self.pc += 2
            }
            0x70 => {
                let (res, overflow) = self.reg[self.inst.x as usize].overflowing_add(self.inst.w2);
                if overflow {
                    // do something
                }
                self.reg[self.inst.x as usize] = res;
                self.pc += 2;
            }
            0x80 => match self.inst.n {
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
                    let x = ((self.inst.opcode & 0x0F00) >> 8) as usize;
                    let y = ((self.inst.opcode & 0x00F0) >> 4) as usize;
                    let (sum, carry) = self.reg[x].overflowing_add(self.reg[y]);
                    self.reg[x] = sum;
                    self.reg[0xF] = if carry { 1 } else { 0 };
                    self.pc += 2;
                }
                0x0005 => {
                    let x = ((self.inst.opcode & 0x0F00) >> 8) as usize;
                    let y = ((self.inst.opcode & 0x00F0) >> 4) as usize;
                    let (result, borrow) = self.reg[x].overflowing_sub(self.reg[y]);
                    self.reg[x] = result;
                    self.reg[0xF] = if borrow { 0 } else { 1 };
                    self.pc += 2;
                }
                0x0006 => {
                    let x = self.inst.x as usize;
                    self.reg[0xf] = self.reg[x] & 0x01;
                    self.reg[x] >>= 1;
                    self.pc += 2;
                }
                0x0007 => {
                    let (res, overflow) = self.reg[self.inst.y as usize]
                        .overflowing_sub(self.reg[self.inst.x as usize]);
                    self.reg[self.inst.x as usize] = res;
                    if overflow {
                        self.reg[15] = 0;
                    } else {
                        self.reg[15] = 1;
                    }
                    self.pc += 2;
                }
                0x000e => {
                    let x = self.inst.x as usize;
                    self.reg[0xF] = (self.reg[x] & 0x80) >> 7;
                    self.reg[x] <<= 1;
                    self.pc += 2;
                }
                _ => {
                    println!()
                }
            },
            0x90 => {
                if self.reg[self.inst.y as usize] != self.reg[self.inst.x as usize] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA0 => {
                self.i = self.inst.nnn;
                self.pc += 2
            }
            0xB0 => {
                self.pc = self.inst.nnn + self.reg[0] as u16;
            }
            0xC0 => {
                let kk = (self.inst.opcode & 0x00FF) as u8;
                let mut rng = rand::thread_rng();
                self.reg[self.inst.x as usize] = rng.gen::<u8>() & kk;
                self.pc += 2;
            }
            0xE0 => {
                match self.inst.w2 {
                    //TODO: this could be wrong
                    0x9e => {
                        let x = self.inst.x as usize;
                        if self.keys[self.reg[x] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0x90 => {
                        if self.keys[self.reg[self.inst.x as usize] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        if !self.keys[self.reg[self.inst.x as usize] as usize] {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        println!("No HERE inst found for {:#x}", self.inst.w2)
                    }
                }
            }
            0xF0 => match self.inst.w2 {
                0x07 => {
                    self.reg[self.inst.x as usize] = self.delay_timer;
                    self.pc += 2;
                }
                0x0A => {
                    if let Some(k) = self.keys.iter().position(|x| *x == true) {
                        self.keys[k] = false;
                        self.reg[self.inst.x as usize] = k as u8;
                        self.pc += 2;
                    }
                }
                0x099 => {
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
                }
                0x15 => {
                    self.delay_timer = self.reg[self.inst.x as usize];
                    self.pc += 2;
                }
                0x18 => {
                    self.sound_timer = self.reg[self.inst.x as usize];
                    self.pc += 2;
                }
                0x1E => {
                    let (res, overflow) = self
                        .i
                        .overflowing_add(self.reg[self.inst.x as usize] as u16);
                    self.i = res;
                    if overflow {
                        self.reg[15] = 1;
                    } else {
                        self.reg[15] = 0;
                    }
                    self.pc += 2;
                }
                0x29 => {
                    self.i = self.reg[self.inst.x as usize] as u16 * 5;
                    self.pc += 2;
                }
                0x33 => {
                    let x = ((self.inst.opcode & 0x0F00) >> 8) as usize;
                    let vx = self.reg[x];

                    self.memory[self.i as usize] = vx / 100;
                    self.memory[(self.i + 1) as usize] = (vx % 100) / 10;
                    self.memory[(self.i + 2) as usize] = vx % 10;
                    self.pc += 2;
                }
                0x55 => {
                    let x = self.inst.x;
                    for index in 0..=x as usize {
                        self.memory[self.i as usize + index] = self.reg[index];
                    }
                    self.pc += 2;
                }
                0x65 => {
                    let x = ((self.inst.opcode >> 8) & 0xF) as usize;
                    let mut tmp = self.i as usize;
                    for i in 0..=x {
                        self.reg[i] = self.memory[tmp];
                        tmp += 1;
                    }
                    self.pc += 2;
                }
                _ => {
                    println!("No inst found for {:#x}", self.inst.w1)
                }
            },
            0xD0 => {
                self.reg[15] = 0;

                let x_coord = self.reg[self.inst.x as usize];
                let y_coord = self.reg[self.inst.y as usize];
                let mut flipped = false;

                for y_line in 0..self.inst.n {
                    let sprite = self.memory[(self.i + y_line as u16) as usize];
                    for x_line in 0..8 {
                        let bit_on = sprite & (0b1000_0000 >> x_line);
                        if bit_on != 0 {
                            if (x_coord >= SCREEN_WIDTH as u8) || (y_coord >= SCREEN_HEIGHT as u8) {
                                continue;
                            }
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

            _ => {
                println!("No inst found for {:#x}", self.inst.w1)
            }
        }
    }
}
