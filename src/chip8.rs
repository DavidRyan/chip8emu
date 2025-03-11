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
    sp: u16
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut mem = [0_u8; 4096];
        for(i, sprint) in SPRITE_CHARS.iter().enumerate() {
            mem[i] = *sprint
        }
        Chip8 {
            sp: 0x200,
            delay_timer: 0,
            sound_timer: 8,
            i: 0,
            registers: [0; 16],
            memory: mem,
            graphics: [false; 64 * 32],
            stack: Vec::new()
        }
    }

    pub fn run(&mut self, rom: Vec<u8>) {
        // instruction is frst half (nib) of rom[0]
        // part of data can be first word second nib
        // data in second word
        let instruction = self.memory[self.sp as usize] & 0xf0;
        match instruction & 0xf0 {
            0x00 => println!("0"),
            0x10 => println!("1"),
            0x60 => println!("6"),
            0x70 => println!("7"),
            0xA0 => println!("a"),
            0xD0 => println!("d"),
            _ =>  println!("null: {:#x}", instruction),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory[0x200..0x200 + rom.len()].copy_from_slice(rom);
    }
}

