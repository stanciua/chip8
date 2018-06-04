extern crate byteorder;
extern crate rand;
extern crate sdl2;

use std::fs::metadata;
use std::fs::File;
use std::io::Read;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::collections::HashSet;
use std::time::Duration;

pub mod disassembler;
pub mod interpreter;
pub mod keyboard;
pub mod screen;

// use disassembler::disassembler::Disassembler;
use interpreter::interpreter::Interpreter;
use keyboard::keyboard::Keyboard;
use screen::screen::Screen;

pub fn main() -> std::io::Result<()> {
    let size = metadata("TETRIS")?.len();
    let mut file = File::open("TETRIS")?;
    let mut program = Vec::with_capacity(size as usize);
    let _bytes_read = file.read_to_end(&mut program)?;
    // let disassembly = Disassembler::from_binary(&program);
    let mut display = [0u8; 64 * 32];
    let mut screen = Screen::with_display(&mut display);
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    {
        let mut keyboard = Keyboard::with_context(&sdl_context);
        let mut interpreter = Interpreter::new(&mut keyboard, &mut program, &mut screen);
        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        interpreter.run();
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }
    Ok(())
}
