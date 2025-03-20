mod chip8;
use crate::chip8::Chip8;

use std::env;
use std::fs;

use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    println!("File path {}", file_path);

    let win_config = WindowConfig::new().set_size(1200, 640).set_vsync(true);
    let _ = notan::init()
        .draw(draw)
        .add_config(DrawConfig)
        .add_config(win_config)
        .event(event)
        .build();

    let rom = fs::read(file_path).unwrap();

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom);
    chip8.execute();
}

fn event(app: &mut App, evt: Event) {
    if let Event::KeyUp{ .. } = evt {
        println!("Key down {:?}", app.keyboard.last_key_released);
    }
}

//1200 X 640
fn draw(gfx: &mut Graphics) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::BLACK);
    draw.rect((20.0, 20.0), (20.0, 20.0));
    gfx.render(&draw);
}
