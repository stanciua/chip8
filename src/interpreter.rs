pub mod interpreter {
    use byteorder::{BigEndian, ByteOrder};
    use keyboard::keyboard::Keyboard;
    use rand::thread_rng;
    use rand::Rng;
    use screen::screen::Screen;
    use std::time::Duration;

    pub const FONTS: [[u8; 5]; 16] = [
        // zero
        [0xF0, 0x90, 0x90, 0x90, 0xF0],
        // one
        [0x20, 0x60, 0x20, 0x20, 0x70],
        // two
        [0xF0, 0x10, 0xF0, 0x80, 0xF0],
        // three
        [0xF0, 0x10, 0xF0, 0x10, 0xF0],
        // four
        [0x90, 0x90, 0xF0, 0x10, 0x10],
        // five
        [0xF0, 0x80, 0xF0, 0x10, 0xF0],
        // six
        [0xF0, 0x80, 0xF0, 0x90, 0xF0],
        // seven
        [0xF0, 0x10, 0x20, 0x40, 0x40],
        // eight
        [0xF0, 0x90, 0xF0, 0x90, 0xF0],
        // nine
        [0xF0, 0x90, 0xF0, 0x10, 0xF0],
        // A
        [0xF0, 0x90, 0xF0, 0x90, 0x90],
        // B
        [0xE0, 0x90, 0xE0, 0x90, 0xE0],
        // C
        [0xF0, 0x80, 0x80, 0x80, 0xF0],
        // D
        [0xE0, 0x90, 0x90, 0x90, 0xE0],
        // E
        [0xF0, 0x80, 0xF0, 0x80, 0xF0],
        // F
        [0xF0, 0x80, 0xF0, 0x80, 0x80],
    ];

    #[derive(Debug)]
    pub struct Interpreter<'a, 'b> {
        vx: [u8; 16],
        stack: [u16; 16],
        i: u16,
        pc: u16,
        dt: u8,
        sp: usize,
        st: u8,
        memory: Vec<u8>,
        program: &'a mut [u8],
        screen: &'a mut Screen<'a>,
        keyboard: &'b mut Keyboard<'b>,
    }

    impl<'a, 'b> Interpreter<'a, 'b> {
        pub fn new(
            keyboard: &'b mut Keyboard<'b>,
            program: &'a mut [u8],
            screen: &'a mut Screen<'a>,
        ) -> Interpreter<'a, 'b> {
            let mut raw_memory = vec![0u8; 4096];
            Interpreter::init_fonts(&mut raw_memory);
            Interpreter {
                vx: [0u8; 16],
                stack: [0u16; 16],
                i: 0,
                pc: 0,
                dt: 0,
                sp: 0,
                st: 0,
                memory: raw_memory,
                program: program,
                screen: screen,
                keyboard: keyboard,
            }
        }

        pub fn init_fonts(memory: &mut [u8]) {
            // place the fonts sprite in memory starting with reserved address
            // 0x0000
            for (idx, b) in FONTS.iter().flat_map(|arr| arr.iter()).enumerate() {
                memory[idx] = *b;
            }
        }

        fn byte_to_bits(byte: u8) -> [u8; 8] {
            let mut bits = [0u8; 8];
            let mut byte = byte;
            for i in (0..bits.len()).into_iter().rev() {
                bits[i] = byte & 0x1;
                byte >>= 1;
            }
            bits
        }
        pub fn run(&mut self) {
            let instructions = self
                .program
                .chunks(2)
                .map(|bytes| BigEndian::read_u16(bytes))
                .collect::<Vec<_>>();
            for instr in instructions {
                let nimbles = (
                    (instr >> 12) as u8,
                    ((instr >> 8) & 0xF) as u8,
                    ((instr >> 4) & 0xF) as u8,
                    (instr & 0xF) as u8,
                );
                match nimbles {
                    (0, 0, 0xE, 0) => {
                        self.screen.screen().iter_mut().for_each(|v| *v = 0);
                        self.screen.render();
                    }
                    (0, 0, 0xE, 0xE) => {
                        self.pc = self.stack[self.sp];
                        self.sp -= 1;
                    }
                    (0, 0, _, _l) => {
                        // NOP
                    }

                    (1, _, _, _) => {
                        self.pc = instr & 0xFFF;
                    }
                    (2, _, _, _) => {
                        self.stack[self.sp] = self.pc;
                        self.sp += 1;
                        self.pc = instr & 0xFFF;
                    }
                    (3, r, _, _) => {
                        if self.vx[r as usize] == (instr & 0xFF) as u8 {
                            self.pc += 2;
                        }
                    }
                    (4, r, _, _) => {
                        if self.vx[r as usize] != (instr & 0xFF) as u8 {
                            self.pc += 2;
                        }
                    }
                    (5, r1, r2, 0) => {
                        if r1 == r2 {
                            self.pc += 2;
                        }
                    }
                    (6, r, _, _) => {
                        self.vx[r as usize] = (instr & 0xFF) as u8;
                    }
                    (7, r, _, _) => {
                        self.vx[r as usize] =
                            ((self.vx[r as usize] as u16 + (instr & 0xFF)) & 0xFF) as u8;
                    }
                    (8, r1, r2, 0) => {
                        self.vx[r1 as usize] = self.vx[r2 as usize];
                    }
                    (8, r1, r2, 1) => {
                        self.vx[r1 as usize] |= self.vx[r2 as usize];
                    }
                    (8, r1, r2, 2) => {
                        self.vx[r1 as usize] &= self.vx[r2 as usize];
                    }
                    (8, r1, r2, 4) => {
                        let rslt = self.vx[r1 as usize] as u16 + self.vx[r2 as usize] as u16;
                        if rslt > 0xFF {
                            self.vx[0xF] = 1;
                        } else {
                            self.vx[0xF] = 0;
                        }

                        self.vx[r1 as usize] = (rslt & 0xFF) as u8;
                    }
                    (8, r1, r2, 5) => {
                        if self.vx[r1 as usize] > self.vx[r2 as usize] {
                            self.vx[0xF] = 1;
                        } else {
                            self.vx[0xF] = 0;
                        }

                        self.vx[r1 as usize] -= self.vx[r2 as usize];
                    }
                    (8, r1, r2, 6) => {
                        self.vx[0xF] = self.vx[r2 as usize] & 0x1;
                        self.vx[r1 as usize] = self.vx[r2 as usize] >> 1;
                    }
                    (8, r1, r2, 7) => {
                        if self.vx[r2 as usize] > self.vx[r1 as usize] {
                            self.vx[0xF] = 1;
                        } else {
                            self.vx[0xF] = 0;
                        }

                        self.vx[r1 as usize] = self.vx[r2 as usize] - self.vx[r1 as usize];
                    }
                    (8, r1, r2, 0xE) => {
                        self.vx[0xF] = self.vx[r2 as usize] >> 7;
                        self.vx[r1 as usize] = self.vx[r2 as usize] << 1;
                    }
                    (9, r1, r2, 0) => {
                        if r1 != r2 {
                            self.pc += 2;
                        }
                    }
                    (0xA, _, _, _) => {
                        self.i = instr & 0xFFF;
                    }
                    (0xB, _, _, _) => {
                        self.pc = self.vx[0] as u16 + instr & 0xFFF;
                    }
                    (0xC, r, _, _) => {
                        self.vx[r as usize] = thread_rng().gen::<u8>() & (instr & 0xFF) as u8;
                    }
                    (0xD, r1, r2, n) => {
                        let sprite = (0..n)
                            .into_iter()
                            .map(|idx| {
                                Interpreter::byte_to_bits(
                                    self.memory[(self.i + idx as u16) as usize],
                                )
                            })
                            .collect::<Vec<_>>();

                        self.vx[0xF] = 0;
                        for i in 0..sprite.len() {
                            let x = (r1 + i as u8) % self.screen.height() as u8;
                            for j in 0..sprite[i].len() {
                                let y = (r2 + j as u8) % self.screen.width() as u8;
                                let heigth = self.screen.height();
                                let width = self.screen.width();
                                if self.screen.screen()[x as usize * heigth + y as usize * width]
                                    == 1 && sprite[i][j] == 1
                                {
                                    self.vx[0xF] = 1
                                }
                                self.screen.screen()[x as usize * heigth + y as usize * width] ^=
                                    sprite[i][j];
                            }
                        }
                        self.screen.render();
                    }
                    (0xE, r, 9, 0xE) => {
                        if self.keyboard.keys()[self.vx[r as usize] as usize] == 1 {
                            self.pc += 2;
                        }
                    }
                    (0xE, r, 0xA, 0x1) => {
                        if self.keyboard.keys()[self.vx[r as usize] as usize] == 0 {
                            self.pc += 2;
                        }
                    }
                    (0xF, r, 0, 7) => {
                        self.vx[r as usize] = self.dt;
                    }
                    (0xF, r, 0, 0xA) => loop {
                        if let Some(idx) = self.keyboard.keys().iter().position(|k| *k == 1) {
                            self.vx[r as usize] = idx as u8;
                            break;
                        }
                        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
                    },
                    (0xF, r, 1, 5) => {
                        self.dt = self.vx[r as usize];
                    }
                    (0xF, r, 1, 8) => {
                        self.st = self.vx[r as usize];
                    }
                    (0xF, r, 1, 0xE) => {
                        self.i += self.vx[r as usize] as u16;
                    }
                    (0xF, r, 2, 9) => {
                        // font sprites are located starting with address 0x0000, each
                        // sprite is 5 bytes long
                        self.i = self.vx[r as usize] as u16 * 5;
                    }
                    (0xF, r, 3, 3) => {
                        let mut value = self.vx[r as usize];
                        self.memory[self.i as usize] = value / 100;
                        value %= 100;
                        self.memory[(self.i + 1) as usize] = value / 10;
                        value %= 10;
                        self.memory[(self.i + 2) as usize] = value;
                    }
                    (0xF, r, 5, 5) => {
                        for idx in 0..=r {
                            self.memory[(self.i + idx as u16) as usize] = self.vx[idx as usize];
                        }
                    }
                    (0xF, r, 6, 5) => {
                        for idx in 0..=r {
                            self.vx[idx as usize] = self.memory[(self.i + idx as u16) as usize];
                        }
                    }
                    _ => panic!(format!("unsupported instruction: {:04X}", instr)),
                }
            }
        }
    }
}
