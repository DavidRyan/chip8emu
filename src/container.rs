use crate::chip8::Chip8;
use std::{thread, time};

use notan::app::{AppFlow, Event, Plugin};
use notan::draw::*;
use notan::prelude::*;

pub struct Container {
    pub c8: Chip8,
}

impl Container {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut c8 = Chip8::new();
        c8.load_rom(&rom);
        Container { c8 }
    }
}

impl Plugin for Container {
    fn event(
        &mut self,
        app: &mut App,
        _assets: &mut Assets,
        event: &Event,
    ) -> Result<AppFlow, String> {
        if let Event::KeyUp { .. } = event {
            let key = app.keyboard.last_key_released.ok_or("No key pressed")?;
            let k = match key {
                KeyCode::Key1=> 0x1,
                KeyCode::Key2 => 0x2,
                KeyCode::Key3 => 0x3,
                KeyCode::Key4 => 0xC,
                KeyCode::Q => 0x4,
                KeyCode::W => 0x5,
                KeyCode::E => 0x6,
                KeyCode::R => 0xD,
                KeyCode::A => 0x7,
                KeyCode::S => 0x8,
                KeyCode::D => 0x9,
                KeyCode::F => 0xE,
                KeyCode::Z => 0xA,
                KeyCode::X => 0x0,
                KeyCode::C => 0xB,
                KeyCode::V => 0xF,
                _ => (0x0),
            };
            self.c8.key_up(k);
        }

        if let Event::KeyDown{ .. } = event {
            let key = app.keyboard.down.iter().nth(0).map(|(k, _)| k).ok_or("No key found")?;
            //let key = app.keyboard.pressed.iter().nth(0).unwrap();
            let k = match key {
                KeyCode::Key1 => 0x1,
                KeyCode::Key2 => 0x2,
                KeyCode::Key3 => 0x3,
                KeyCode::Key4 => 0xC,
                KeyCode::Q => 0x4,
                KeyCode::W => 0x5,
                KeyCode::E => 0x6,
                KeyCode::R => 0xD,
                KeyCode::A => 0x7,
                KeyCode::S => 0x8,
                KeyCode::D => 0x9,
                KeyCode::F => 0xE,
                KeyCode::Z => 0xA,
                KeyCode::X => 0x0,
                KeyCode::C => 0xB,
                KeyCode::V => 0xF,
                _ => (0x0),
            };
            self.c8.key_down(k);
        }

        Ok(notan::app::AppFlow::Next)
    }

    fn update(&mut self, _app: &mut App, _assets: &mut Assets) -> Result<AppFlow, String> {
        self.c8.tick_timers();
        self.c8.execute();
        Ok(notan::app::AppFlow::Next)
    }

    fn draw(
        &mut self,
        _app: &mut App,
        _assets: &mut Assets,
        gfx: &mut Graphics,
    ) -> Result<AppFlow, String> {
        let mut draw = gfx.create_draw();
        draw.clear(Color::BLACK);

        for x in 0..63 {
            for y in 0..31 {
                let i = y * 64 + x;
                if self.c8.graphics[i] {
                    let n_x = x * 10;
                    let n_y = y * 10;
                    draw.rect((n_x as f32, n_y as f32), (10.0, 10.0));
                }
            }
        }
        gfx.render(&draw);
        let ten_millis = time::Duration::from_millis(10);
        //thread::sleep(ten_millis);
        Ok(notan::app::AppFlow::Next)
    }
}
