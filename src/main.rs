mod chip8;
use crate::chip8::Chip8;

use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("File path {}", file_path);

    let rom = fs::read(file_path).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);
    chip8.run();
}


