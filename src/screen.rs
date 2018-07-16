use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub struct Screen {
    canvas: Canvas<Window>,
}

use CHIP8_HEIGHT;
use CHIP8_WIDTH;

pub const SCALE: u32 = 20;
pub const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE;
pub const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE;

impl Screen {
    pub fn new(sdl_context: &Sdl) -> Self {
        let video_sys = sdl_context.video().unwrap();
        let window = video_sys
            .window("CHIP-8", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Screen { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[[u8; CHIP8_WIDTH]; CHIP8_HEIGHT]) {
        for (i, row) in pixels.iter().enumerate() {
            for (j, p) in row.iter().enumerate() {
                let x = (i as u32) * SCALE;
                let y = (j as u32) * SCALE;

                self.canvas.set_draw_color(if *p == 0 {
                    pixels::Color::RGB(0, 0, 0)
                } else {
                    pixels::Color::RGB(0, 250, 0)
                });

                let _ = self
                    .canvas
                    .fill_rect(Rect::new(x as i32, y as i32, SCALE, SCALE));
            }
        }
        self.canvas.present();
    }
}
