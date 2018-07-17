extern crate byteorder;
extern crate rand;
extern crate sdl2;

use std::fs::metadata;
use std::fs::File;
use std::io::Read;

use std::thread;
use std::time::Duration;

pub mod audio;
pub mod disassembler;
pub mod interpreter;
pub mod keyboard;
pub mod screen;

use audio::Audio;
use interpreter::Interpreter;
use keyboard::Keyboard;
use screen::Screen;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
const CHIP8_RAM: usize = 4096;

pub fn main() -> std::io::Result<()> {
    let sleep_duration = Duration::from_millis(2);
    let size = metadata("brix.ch8")?.len();
    let mut file = File::open("brix.ch8")?;
    let mut program = Vec::with_capacity(size as usize);
    let _bytes_read = file.read_to_end(&mut program)?;
    {
        // let _disassembly = Disassembler::from_binary(&program);
        // println!("{}", _disassembly);
    }
    let sdl_context = sdl2::init().unwrap();
    let mut keyboard = Keyboard::new(&sdl_context);
    let mut screen = Screen::new(&sdl_context);
    let audio = Audio::new(&sdl_context);
    let mut interpreter = Interpreter::default();
    interpreter.load(&program);

    while let Ok(keys) = keyboard.poll() {
        let output = interpreter.tick(keys);

        if output.vram_changed {
            screen.draw(output.vram);
        }

        if output.beep {
            audio.start_beep();
        } else {
            audio.stop_beep();
        }

        thread::sleep(sleep_duration);
    }
    Ok(())
}
