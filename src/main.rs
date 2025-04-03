mod chip8;
mod container;

use crate::container::Container;

use std::env;
use std::fs;

use notan::draw::*;
use notan::prelude::*;



fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let rom = fs::read(file_path).unwrap();

    let win_config = WindowConfig::new().set_size(640, 320).set_vsync(true);
    let _ = notan::init()
        .add_plugin(Container::new(rom))
        .add_config(DrawConfig)
        .add_config(win_config)
        .build();
}



