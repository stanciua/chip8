use byteorder::{BigEndian, ByteOrder};
use rand::thread_rng;
use rand::Rng;
use CHIP8_HEIGHT;
use CHIP8_RAM;
use CHIP8_WIDTH;

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

const INSTR_SIZE: usize = 2;

pub struct State<'a> {
    pub vram: &'a [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub vram_changed: bool,
    pub beep: bool,
}

pub struct Interpreter {
    vx: [u8; 16],
    stack: [u16; 16],
    i: u16,
    pc: usize,
    dt: u8,
    sp: usize,
    st: u8,
    memory: [u8; CHIP8_RAM],
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    vram_changed: bool,
    keyboard: [bool; 16],
    keyboard_waiting: bool,
    keyboard_register: usize,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut raw_memory = [0u8; CHIP8_RAM];
        Interpreter::init_fonts(&mut raw_memory);
        Interpreter {
            vx: [0u8; 16],
            stack: [0u16; 16],
            i: 0,
            pc: 0x200,
            dt: 0,
            sp: 0,
            st: 0,
            memory: raw_memory,
            vram: [[0u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
            vram_changed: false,
            keyboard: [false; 16],
            keyboard_waiting: false,
            keyboard_register: 0,
        }
    }

    pub fn init_fonts(memory: &mut [u8]) {
        // place the fonts sprites in memory starting with reserved address
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

    pub fn load(&mut self, program: &[u8]) {
        assert!(program.len() + 0x200 <= 4096);
        program
            .iter()
            .enumerate()
            .for_each(|(idx, &v)| self.memory[0x200 + idx] = v);
    }

    pub fn tick(&mut self, keyboard: [bool; 16]) -> State {
        self.keyboard = keyboard;
        self.vram_changed = false;

        if self.keyboard_waiting {
            let pos = self.keyboard.iter().position(|&v| v == true).unwrap();
            self.vx[self.keyboard_register] = pos as u8;
        } else {
            if self.dt > 0 {
                self.dt -= 1;
            }
            if self.st > 0 {
                self.st -= 1;
            }

            let instr = self.get_opcode();
            self.run(instr);
        }

        State {
            vram: &self.vram,
            vram_changed: self.vram_changed,
            beep: self.st > 0,
        }
    }

    fn get_opcode(&self) -> u16 {
        BigEndian::read_u16(&self.memory[self.pc..])
    }

    pub fn run(&mut self, instr: u16) {
        let nimbles = (
            (instr >> 12) as u8,
            ((instr >> 8) & 0xF) as u8,
            ((instr >> 4) & 0xF) as u8,
            (instr & 0xF) as u8,
        );
        println!("instr: {:X?}", nimbles);
        match nimbles {
            (0, 0, 0xE, 0) => {
                self.vram
                    .iter_mut()
                    .flat_map(|it| it.iter_mut())
                    .for_each(|v| *v = 0);
                self.pc += INSTR_SIZE;
                self.vram_changed = true;
            }
            (0, 0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp] as usize;
            }
            (0, 0, _, _l) => {
                // NOP
            }

            (1, _, _, _) => {
                self.pc = (instr & 0xFFF) as usize;
            }
            (2, _, _, _) => {
                self.stack[self.sp] = self.pc as u16 + INSTR_SIZE as u16;
                self.sp += 1;
                self.pc = (instr & 0xFFF) as usize;
            }
            (3, r, _, _) => {
                if self.vx[r as usize] == (instr & 0xFF) as u8 {
                    self.pc += 2 * INSTR_SIZE;
                } else {
                    self.pc += INSTR_SIZE;
                }
            }
            (4, r, _, _) => {
                if self.vx[r as usize] != (instr & 0xFF) as u8 {
                    self.pc += 2 * INSTR_SIZE;
                } else {
                    self.pc += INSTR_SIZE;
                }
            }
            (5, r1, r2, 0) => {
                if r1 == r2 {
                    self.pc += 2 * INSTR_SIZE;
                } else {
                    self.pc += INSTR_SIZE;
                }
            }
            (6, r, _, _) => {
                self.vx[r as usize] = (instr & 0xFF) as u8;
                self.pc += INSTR_SIZE;
            }
            (7, r, _, _) => {
                let vx = self.vx[r as usize] as u16;
                let val = (instr & 0xFF) as u16;
                let result = vx + val;
                self.vx[r as usize] = result as u8;
                self.pc += INSTR_SIZE;
            }
            (8, r1, r2, 0) => {
                self.vx[r1 as usize] = self.vx[r2 as usize];
                self.pc += INSTR_SIZE;
            }
            (8, r1, r2, 1) => {
                self.vx[r1 as usize] |= self.vx[r2 as usize];
                self.pc += INSTR_SIZE;
            }
            (8, r1, r2, 2) => {
                self.vx[r1 as usize] &= self.vx[r2 as usize];
                self.pc += INSTR_SIZE;
            }
            (8, r1, r2, 4) => {
                let rslt = self.vx[r1 as usize] as u16 + self.vx[r2 as usize] as u16;
                if rslt > 0xFF {
                    self.vx[0xF] = 1;
                } else {
                    self.vx[0xF] = 0;
                }

                self.vx[r1 as usize] = (rslt & 0xFF) as u8;
                self.pc += INSTR_SIZE;
            }
            (8, r1, r2, 5) => {
                if self.vx[r1 as usize] > self.vx[r2 as usize] {
                    self.vx[0xF] = 1;
                } else {
                    self.vx[0xF] = 0;
                }

                self.vx[r1 as usize] -= self.vx[r2 as usize];
                self.pc += INSTR_SIZE;
            }
            (8, r1, r2, 6) => {
                self.vx[0xF] = self.vx[r2 as usize] & 0x1;
                self.vx[r1 as usize] = self.vx[r2 as usize] >> 1;
                self.pc += INSTR_SIZE;
            }
            (8, r1, r2, 7) => {
                if self.vx[r2 as usize] > self.vx[r1 as usize] {
                    self.vx[0xF] = 1;
                } else {
                    self.vx[0xF] = 0;
                }

                self.vx[r1 as usize] = self.vx[r2 as usize] - self.vx[r1 as usize];
                self.pc += INSTR_SIZE;
            }
            (8, r1, r2, 0xE) => {
                self.vx[0xF] = self.vx[r2 as usize] >> 7;
                self.vx[r1 as usize] = self.vx[r2 as usize] << 1;
                self.pc += INSTR_SIZE;
            }
            (9, r1, r2, 0) => {
                if r1 != r2 {
                    self.pc += 2 * INSTR_SIZE;
                } else {
                    self.pc += INSTR_SIZE;
                }
            }
            (0xA, _, _, _) => {
                self.i = instr & 0xFFF;
                self.pc += INSTR_SIZE;
            }
            (0xB, _, _, _) => {
                self.pc = (self.vx[0] as u16 + instr & 0xFFF) as usize;
            }
            (0xC, r, _, _) => {
                self.vx[r as usize] = thread_rng().gen::<u8>() & (instr & 0xFF) as u8;
                self.pc += INSTR_SIZE;
            }
            (0xD, r1, r2, n) => {
                let sprites = (0..n)
                    .into_iter()
                    .map(|idx| {
                        Interpreter::byte_to_bits(self.memory[(self.i + idx as u16) as usize])
                    })
                    .collect::<Vec<_>>();

                self.vx[0xF] = 0;
                for i in 0..sprites.len() {
                    let x = (self.vx[r1 as usize] + i as u8) % CHIP8_HEIGHT as u8;
                    for j in 0..sprites[i].len() {
                        let y = (self.vx[r2 as usize] + j as u8) % CHIP8_WIDTH as u8;
                        if self.vram[x as usize][y as usize] == 1 && sprites[i][j] == 1 {
                            self.vx[0xF] = 1;
                        } else {
                            self.vx[0xF] = 0;
                        }
                        self.vram[x as usize][y as usize] ^= sprites[i][j];
                    }
                    // 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16
                    // 1 2 3 4
                    // 5 6 7 8
                    // 9 10 11 12
                    // 13 14 15 16
                }
                self.vram_changed = true;
                self.pc += INSTR_SIZE;
            }
            (0xE, r, 9, 0xE) => {
                if self.keyboard[self.vx[r as usize] as usize] {
                    self.pc += 2 * INSTR_SIZE;
                } else {
                    self.pc += INSTR_SIZE;
                }
            }
            (0xE, r, 0xA, 0x1) => {
                if self.keyboard[self.vx[r as usize] as usize] {
                    self.pc += 2 * INSTR_SIZE;
                } else {
                    self.pc += INSTR_SIZE;
                }
            }
            (0xF, r, 0, 7) => {
                self.vx[r as usize] = self.dt;
                self.pc += INSTR_SIZE;
            }
            (0xF, r, 0, 0xA) => loop {
                self.keyboard_waiting = true;
                self.keyboard_register = r as usize;
                self.pc += INSTR_SIZE;
            },
            (0xF, r, 1, 5) => {
                self.dt = self.vx[r as usize];
                self.pc += INSTR_SIZE;
            }
            (0xF, r, 1, 8) => {
                self.st = self.vx[r as usize];
                self.pc += INSTR_SIZE;
            }
            (0xF, r, 1, 0xE) => {
                self.i += self.vx[r as usize] as u16;
                self.pc += INSTR_SIZE;
            }
            (0xF, r, 2, 9) => {
                // font sprites are located starting with address 0x0000, each
                // sprite is 5 bytes long
                self.i = self.vx[r as usize] as u16 * 5;
                self.pc += INSTR_SIZE;
            }
            (0xF, r, 3, 3) => {
                let mut value = self.vx[r as usize];
                self.memory[self.i as usize] = value / 100;
                value %= 100;
                self.memory[(self.i + 1) as usize] = value / 10;
                value %= 10;
                self.memory[(self.i + 2) as usize] = value;
                self.pc += INSTR_SIZE;
            }
            (0xF, r, 5, 5) => {
                for idx in 0..=r {
                    self.memory[(self.i + idx as u16) as usize] = self.vx[idx as usize];
                }
                self.pc += INSTR_SIZE;
            }
            (0xF, r, 6, 5) => {
                for idx in 0..=r {
                    self.vx[idx as usize] = self.memory[(self.i + idx as u16) as usize];
                }
                self.pc += INSTR_SIZE;
            }
            _ => panic!(format!("unsupported instruction: {:04X}", instr)),
        }
    }
}
