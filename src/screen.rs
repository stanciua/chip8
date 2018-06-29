use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub mod screen {
    pub struct Screen {
        canvas: Canvas<Window>,
    }

    pub const SCALE: u8 = 20;
    pub const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE;
    pub const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE;

    impl Screen {
        pub fn new(sdl_context: &sdl2::Sdl) -> Self {
            let video_sys = sdl_context.video().unwrap();
            let window = video_sys
                .window("CHIP-8", SCREEN_WIDTH, SCREEN_HEIGHT)
                .position_centered()
                .opengl()
                .build()
                .unwrap();

            let mut canvas = window.into_canvas().build.unwrap();

            canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.present();

            Screen { canvas: canvas }
        }

        pub fn draw(&mut self, pixels: &[u8]) {
            unimplemented!();
        }
    }

}
