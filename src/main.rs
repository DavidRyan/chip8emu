mod chip8;
use crate::chip8::Chip8;
use std::{thread, time};

use std::env;
use std::fs;
use std::sync::LazyLock;
use std::sync::Mutex;

use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

#[derive(AppState)]
struct State {
    graphics: [bool; 64 * 32],
}

struct Container {
    graphics: [bool; 64 * 32],
}

static CHIP8: LazyLock<Mutex<Chip8>> = LazyLock::new(|| Mutex::new(Chip8::new()));

fn main() {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[1];

        println!("File path {}", file_path);


        let args: Vec<String> = env::args().collect();
        let file_path = &args[1];

        let rom = fs::read(file_path).unwrap();
        CHIP8.lock().unwrap().load_rom(&rom);

        let win_config = WindowConfig::new().set_size(1200, 640).set_vsync(true);
        let _ = notan::init()
            .draw(draw)
            .update(update)
            .add_config(DrawConfig)
            .add_config(win_config)
            .event(event)
            .build();

    }

fn setup(gfx: &mut Graphics) -> State {

    State {
        graphics: [false; 64 * 32],
    }
}

fn update(app: &mut App) {
    println!("Executing");
    CHIP8.lock().unwrap().execute();
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

    for x in 0..63 {
        for y in 0..31 {
            // i=y*W + x
            let i = y * 64 + x;
            if (CHIP8.lock().unwrap().graphics[i]) {
                //print!("1")
            } else {
                //print!("0")
            }
            if CHIP8.lock().unwrap().graphics[i] {
                let n_x = x * 20;
                let n_y = y * 20;
                draw.rect((n_x as f32, n_y as f32), (20.0, 20.0));
            }
        }
        //println!()
    }
    gfx.render(&draw);

    let ten_millis = time::Duration::from_millis(100);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

}
