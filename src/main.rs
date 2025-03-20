mod chip8;
use crate::chip8::Chip8;

use std::env;
use std::fs;

use notan::prelude::*;
use notan::draw::*;

fn main() {

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("File path {}", file_path);

    let win_config = WindowConfig::new().set_size(1200, 640).set_vsync(true);
    notan::init().draw(draw)
        .add_config(DrawConfig)
        .add_config(win_config)
        .build();

    let rom = fs::read(file_path).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);
    //chip8.run();
    let d = DrawConfig;
}

//1200 X 640
fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.rect((20.0,20.0),(20.0,20.0));
    gfx.render(&draw);
}


