use std::fmt;

pub struct Instruction(pub u16);

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let nimble = self.0 >> 12;
        match nimble {
            // Clear screen and return from subroutine instructions
            0 => match self.0 {
                0x00E0 => {
                    let _res = writeln!(f, "{:04X}          {}", self.0, "CLS");
                }
                0x00EE => {
                    let _res = writeln!(f, "{:04X}          {}", self.0, "RTS");
                }
                _ => {
                    let _res = writeln!(f, "{:04X}          {}", self.0, "NOP");
                }
            },
            // absolute jumps to address
            1 => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} ${:X}",
                    self.0,
                    "JUMP",
                    self.0 & 0x0FFF
                );
            }
            // call subroutine at address
            2 => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} ${:X}",
                    self.0,
                    "CALL",
                    self.0 & 0x0FFF
                );
            }
            // Skip the next instruction if VX is equals to value NN
            3 => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} V{:X}, #${:02X}",
                    self.0,
                    "SKIP.EQ",
                    (self.0 & 0x0F00) >> 8,
                    self.0 & 0x00FF
                );
            }
            // Skip the next instruction if VX is NOT equals to value NN
            4 => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} V{:X}, #${:X}",
                    self.0,
                    "SKIP.NE",
                    (self.0 & 0x0F00) >> 8,
                    self.0 & 0x00FF
                );
            }
            // Skip the next instruction if VX is equals to VY
            5 => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} V{:X}, V{:X}",
                    self.0,
                    "SKIP.EQ",
                    (self.0 & 0x0F00) >> 8,
                    (self.0 & 0x00F0) >> 4
                );
            }
            // Sets the value of VX register to NN
            6 => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} V{:X}, $#{:02X}",
                    self.0,
                    "SETR",
                    (self.0 & 0x0F00) >> 8,
                    self.0 & 0x00FF
                );
            }
            // Adds the value of NN to register VX, carry flag is not changed
            7 => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} V{:X}, $#{:02X}",
                    self.0,
                    "ADDR",
                    (self.0 & 0x0F00) >> 8,
                    self.0 & 0x00FF
                );
            }
            // Skip the next instruction if VX is NOT equals to VY
            9 => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} V{:X}, V{:X}",
                    self.0,
                    "SKIP.NE",
                    (self.0 & 0x0F00) >> 8,
                    (self.0 & 0x00F0) >> 4
                );
            }
            8 => match self.0 & 0x000F {
                // sets VX to the value of VY
                0 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}, V{:X}",
                        self.0,
                        "MOV",
                        (self.0 & 0x0F00) >> 8,
                        (self.0 & 0x00F0) >> 4
                    );
                }
                // sets VX to the value of VX | VY
                1 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}, V{:X}",
                        self.0,
                        "OR",
                        (self.0 & 0x0F00) >> 8,
                        (self.0 & 0x00F0) >> 4
                    );
                }
                // sets VX to the value of VX & VY
                2 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}, V{:X}",
                        self.0,
                        "AND",
                        (self.0 & 0x0F00) >> 8,
                        (self.0 & 0x00F0) >> 4
                    );
                }
                // sets VX to the value of VX ^ VY
                3 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}, V{:X}",
                        self.0,
                        "XOR",
                        (self.0 & 0x0F00) >> 8,
                        (self.0 & 0x00F0) >> 4
                    );
                }
                // sets VX with the value of VX + VY, VF is set to 1 when carry
                // is present, or 0 otherwise
                4 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}, V{:X}",
                        self.0,
                        "ADD.",
                        (self.0 & 0x0F00) >> 8,
                        (self.0 & 0x00F0) >> 4
                    );
                }
                // sets VX with the value of VX - VY, VF is set to 0 when there's a borrow
                // or 1 otherwise
                5 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}, V{:X}",
                        self.0,
                        "SUB.",
                        (self.0 & 0x0F00) >> 8,
                        (self.0 & 0x00F0) >> 4
                    );
                }
                // shifts right VY by one and stores the result inside VX
                6 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}",
                        self.0,
                        "SHR.",
                        (self.0 & 0x00F0) >> 4
                    );
                }
                // sets VX with the value of VY - VX, VF is set to 0 when there's a borrow
                // or 1 otherwise
                7 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}, V{:X}",
                        self.0,
                        "SUBB.",
                        (self.0 & 0x0F00) >> 8,
                        (self.0 & 0x00F0) >> 4
                    );
                }
                // shifts left VY by one and stores the result inside VX
                0xE => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}",
                        self.0,
                        "SHL.",
                        (self.0 & 0x00F0) >> 4
                    );
                }
                _ => panic!(format!(
                    "unsupported instruction {:04X} within nimble: {:X}",
                    self.0, nimble
                )),
            },
            // Sets the I register to value NNN.
            0xA => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} ${:03X}",
                    self.0,
                    "SETI",
                    self.0 & 0x0FFF,
                );
            }
            // Jump to address NNN + V0
            0xB => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} ${:03X}",
                    self.0,
                    "JUMP0",
                    self.0 & 0x0FFF,
                );
            }
            // Jump to address NNN + V0
            0xC => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} V{:X}, #${:02X}",
                    self.0,
                    "RAND",
                    (self.0 & 0x0F00) >> 8,
                    self.0 & 0x00FF
                );
            }
            // Draws a sprite at coordinates VX, VY with a width of 8 pixels and height of N
            0xD => {
                let _res = writeln!(
                    f,
                    "{:04X}          {} V{:X}, V{:X}, #${:X}",
                    self.0,
                    "SPRITE",
                    (self.0 & 0x0F00) >> 8,
                    (self.0 & 0x00F0) >> 4,
                    self.0 & 0x000F
                );
            }
            0xE => match self.0 & 0xFF {
                // skips the next instruction if the key stored in VX is pressed
                0x9E => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}",
                        self.0,
                        "SKIP.KEY",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                // skips th enext instruction if the key stored in VX is NOT pressed
                0xA1 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}",
                        self.0,
                        "SKIP.NOKEY",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                _ => panic!(format!(
                    "unsupported instruction {:04X} within nimble: {:X}",
                    self.0, nimble
                )),
            },
            // sets VX to the value of the delay timer
            0xF => match self.0 & 0xFF {
                0x07 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}, DELAY",
                        self.0,
                        "MOV",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                // A key press is awaited, and then stored in VX,
                // blocking operation
                0x0A => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}",
                        self.0,
                        "WAITKEY",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                // sets the delay timer to VX
                0x15 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} DELAY, V{:X}",
                        self.0,
                        "MOV",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                // sets the sound timer to VX
                0x18 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} SOUND, V{:X}",
                        self.0,
                        "MOV",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                // adds VX to I
                0x1E => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} I, V{:X}",
                        self.0,
                        "ADD",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                // sets I to the location of the sprite for the character in VX
                0x29 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}",
                        self.0,
                        "SPRITECHAR",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                // stores the BCD representation of VX with the most significant
                // digit at adress in I, the other digits at I + 1, I + 2
                0x33 => {
                    let _res = writeln!(
                        f,
                        "{:04X}          {} V{:X}",
                        self.0,
                        "MOVBCD",
                        (self.0 & 0x0F00) >> 8,
                    );
                }
                // stores V0 -> VX included in memory starting at adress I
                0x55 => {
                    let _res = writeln!(f, "{:04X}          {} (I), V0-VX", self.0, "MOVM",);
                }
                // fills V0 -> VX with values read from memory starting with address I
                0x65 => {
                    let _res = writeln!(f, "{:04X}          {} V0-VX, (I)", self.0, "MOVM",);
                }
                _ => panic!(format!(
                    "unsupported instruction {:04X} within nimble: {:X}",
                    self.0, nimble
                )),
            },
            _ => panic!(format!("unsupported instruction with nimble: {:X}", nimble)),
        };
        Ok(())
    }
}
