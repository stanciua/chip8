pub mod keyboard {

    use sdl2::event::Event;
    use sdl2::keyboard::Keycode;
    use sdl2::pixels::Color;
    use std::collections::HashSet;
    use std::fmt;
    use std::time::Duration;

    pub struct Keyboard<'b> {
        keys: [u8; 16],
        sdl_context: &'b ::sdl2::Sdl,
    }

    impl<'b> fmt::Debug for Keyboard<'b> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "keys: {:?}", self.keys)
        }
    }

    impl<'b> Keyboard<'b> {
        pub fn with_context(context: &'b ::sdl2::Sdl) -> Keyboard {
            Keyboard {
                keys: [0u8; 16],
                sdl_context: context,
            }
        }

        pub fn keys(&mut self) -> &[u8] {
            let mut event_pump = self.sdl_context.event_pump().unwrap();

            for event in event_pump.poll_iter() {
                match event {
                    // handle KeyDown events
                    Event::KeyDown {
                        keycode: Some(Keycode::Num0),
                        ..
                    } => self.keys[0] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num1),
                        ..
                    } => self.keys[1] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num2),
                        ..
                    } => self.keys[2] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num3),
                        ..
                    } => self.keys[3] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num4),
                        ..
                    } => self.keys[4] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num5),
                        ..
                    } => self.keys[5] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num6),
                        ..
                    } => self.keys[6] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num7),
                        ..
                    } => self.keys[7] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num8),
                        ..
                    } => self.keys[8] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::Num9),
                        ..
                    } => self.keys[9] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => self.keys[0xA] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::B),
                        ..
                    } => self.keys[0xB] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::C),
                        ..
                    } => self.keys[0xC] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::D),
                        ..
                    } => self.keys[0xD] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::E),
                        ..
                    } => self.keys[0xE] = 1,
                    Event::KeyDown {
                        keycode: Some(Keycode::F),
                        ..
                    } => self.keys[0xF] = 1,
                    // handle KeyUp events
                    Event::KeyUp {
                        keycode: Some(Keycode::Num0),
                        ..
                    } => self.keys[0] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num1),
                        ..
                    } => self.keys[1] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num2),
                        ..
                    } => self.keys[2] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num3),
                        ..
                    } => self.keys[3] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num4),
                        ..
                    } => self.keys[4] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num5),
                        ..
                    } => self.keys[5] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num6),
                        ..
                    } => self.keys[6] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num7),
                        ..
                    } => self.keys[7] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num8),
                        ..
                    } => self.keys[8] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::Num9),
                        ..
                    } => self.keys[9] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::A),
                        ..
                    } => self.keys[0xA] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::B),
                        ..
                    } => self.keys[0xB] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::C),
                        ..
                    } => self.keys[0xC] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::D),
                        ..
                    } => self.keys[0xD] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::E),
                        ..
                    } => self.keys[0xE] = 0,
                    Event::KeyUp {
                        keycode: Some(Keycode::F),
                        ..
                    } => self.keys[0xF] = 0,
                    _ => {}
                }
            }
            &self.keys
        }
    }
}
