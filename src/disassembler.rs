pub mod disassembler {
    use std::fmt;

    use byteorder::{BigEndian, ByteOrder};
    pub struct Disassembler<'a> {
        program: &'a [u8],
    }
    impl<'a> Disassembler<'a> {
        pub fn from_binary(program: &'a [u8]) -> Disassembler {
            Disassembler { program: program }
        }
    }

    impl<'a> fmt::Display for Disassembler<'a> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let instructions = self
                .program
                .chunks(2)
                .map(|bytes| BigEndian::read_u16(bytes))
                .collect::<Vec<_>>();
            let _res = writeln!(f, "{:4}    {:4}          {}", "PC", "Op", "Mnemonic");
            let mut pc = 0;
            for instr in instructions {
                let nimble = instr >> 12;
                match nimble {
                    // Clear screen and return from subroutine instructions
                    0 => match instr {
                        0x00E0 => {
                            let _res =
                                writeln!(f, "{:04X}    {:04X}          {}", pc, instr, "CLS");
                        }
                        0x00EE => {
                            let _res =
                                writeln!(f, "{:04X}    {:04X}          {}", pc, instr, "RTS");
                        }
                        _ => {
                            let _res =
                                writeln!(f, "{:04X}    {:04X}          {}", pc, instr, "NOP");
                        }
                    },
                    // absolute jumps to address
                    1 => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} ${:X}",
                            pc,
                            instr,
                            "JUMP",
                            instr & 0x0FFF
                        );
                    }
                    // call subroutine at address
                    2 => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} ${:X}",
                            pc,
                            instr,
                            "CALL",
                            instr & 0x0FFF
                        );
                    }
                    // Skip the next instruction if VX is equals to value NN
                    3 => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} V{:X}, #${:02X}",
                            pc,
                            instr,
                            "SKIP.EQ",
                            (instr & 0x0F00) >> 8,
                            instr & 0x00FF
                        );
                    }
                    // Skip the next instruction if VX is NOT equals to value NN
                    4 => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} V{:X}, #${:X}",
                            pc,
                            instr,
                            "SKIP.NE",
                            (instr & 0x0F00) >> 8,
                            instr & 0x00FF
                        );
                    }
                    // Skip the next instruction if VX is equals to VY
                    5 => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} V{:X}, V{:X}",
                            pc,
                            instr,
                            "SKIP.EQ",
                            (instr & 0x0F00) >> 8,
                            (instr & 0x00F0) >> 4
                        );
                    }
                    // Sets the value of VX register to NN
                    6 => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} V{:X}, $#{:02X}",
                            pc,
                            instr,
                            "SETR",
                            (instr & 0x0F00) >> 8,
                            instr & 0x00FF
                        );
                    }
                    // Adds the value of NN to register VX, carry flag is not changed
                    7 => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} V{:X}, $#{:02X}",
                            pc,
                            instr,
                            "ADDR",
                            (instr & 0x0F00) >> 8,
                            instr & 0x00FF
                        );
                    }
                    // Skip the next instruction if VX is NOT equals to VY
                    9 => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} V{:X}, V{:X}",
                            pc,
                            instr,
                            "SKIP.NE",
                            (instr & 0x0F00) >> 8,
                            (instr & 0x00F0) >> 4
                        );
                    }
                    8 => match instr & 0x000F {
                        // sets VX to the value of VY
                        0 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}, V{:X}",
                                pc,
                                instr,
                                "MOV",
                                (instr & 0x0F00) >> 8,
                                (instr & 0x00F0) >> 4
                            );
                        }
                        // sets VX to the value of VX | VY
                        1 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}, V{:X}",
                                pc,
                                instr,
                                "OR",
                                (instr & 0x0F00) >> 8,
                                (instr & 0x00F0) >> 4
                            );
                        }
                        // sets VX to the value of VX & VY
                        2 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}, V{:X}",
                                pc,
                                instr,
                                "AND",
                                (instr & 0x0F00) >> 8,
                                (instr & 0x00F0) >> 4
                            );
                        }
                        // sets VX to the value of VX ^ VY
                        3 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}, V{:X}",
                                pc,
                                instr,
                                "XOR",
                                (instr & 0x0F00) >> 8,
                                (instr & 0x00F0) >> 4
                            );
                        }
                        // sets VX with the value of VX + VY, VF is set to 1 when carry
                        // is present, or 0 otherwise
                        4 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}, V{:X}",
                                pc,
                                instr,
                                "ADD.",
                                (instr & 0x0F00) >> 8,
                                (instr & 0x00F0) >> 4
                            );
                        }
                        // sets VX with the value of VX - VY, VF is set to 0 when there's a borrow
                        // or 1 otherwise
                        5 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}, V{:X}",
                                pc,
                                instr,
                                "SUB.",
                                (instr & 0x0F00) >> 8,
                                (instr & 0x00F0) >> 4
                            );
                        }
                        // shifts right VY by one and stores the result inside VX
                        6 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}",
                                pc,
                                instr,
                                "SHR.",
                                (instr & 0x00F0) >> 4
                            );
                        }
                        // sets VX with the value of VY - VX, VF is set to 0 when there's a borrow
                        // or 1 otherwise
                        7 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}, V{:X}",
                                pc,
                                instr,
                                "SUBB.",
                                (instr & 0x0F00) >> 8,
                                (instr & 0x00F0) >> 4
                            );
                        }
                        // shifts left VY by one and stores the result inside VX
                        0xE => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}",
                                pc,
                                instr,
                                "SHL.",
                                (instr & 0x00F0) >> 4
                            );
                        }
                        _ => panic!(format!(
                            "unsupported instruction {:04X} within nimble: {:X}",
                            instr, nimble
                        )),
                    },
                    // Sets the I register to value NNN.
                    0xA => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} ${:03X}",
                            pc,
                            instr,
                            "SETI",
                            instr & 0x0FFF,
                        );
                    }
                    // Jump to address NNN + V0
                    0xB => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} ${:03X}",
                            pc,
                            instr,
                            "JUMP0",
                            instr & 0x0FFF,
                        );
                    }
                    // Jump to address NNN + V0
                    0xC => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} V{:X}, #${:02X}",
                            pc,
                            instr,
                            "RAND",
                            (instr & 0x0F00) >> 8,
                            instr & 0x00FF
                        );
                    }
                    // Draws a sprite at coordinates VX, VY with a width of 8 pixels and height of N
                    0xD => {
                        let _res = writeln!(
                            f,
                            "{:04X}    {:04X}          {} V{:X}, V{:X}, #${:X}",
                            pc,
                            instr,
                            "SPRITE",
                            (instr & 0x0F00) >> 8,
                            (instr & 0x00F0) >> 4,
                            instr & 0x000F
                        );
                    }
                    0xE => match instr & 0xFF {
                        // skips the next instruction if the key stored in VX is pressed
                        0x9E => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}",
                                pc,
                                instr,
                                "SKIP.KEY",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        // skips th enext instruction if the key stored in VX is NOT pressed
                        0xA1 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}",
                                pc,
                                instr,
                                "SKIP.NOKEY",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        _ => panic!(format!(
                            "unsupported instruction {:04X} within nimble: {:X}",
                            instr, nimble
                        )),
                    },
                    // sets VX to the value of the delay timer
                    0xF => match instr & 0xFF {
                        0x07 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}, DELAY",
                                pc,
                                instr,
                                "MOV",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        // A key press is awaited, and then stored in VX,
                        // blocking operation
                        0x0A => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}",
                                pc,
                                instr,
                                "WAITKEY",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        // sets the delay timer to VX
                        0x15 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} DELAY, V{:X}",
                                pc,
                                instr,
                                "MOV",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        // sets the sound timer to VX
                        0x18 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} SOUND, V{:X}",
                                pc,
                                instr,
                                "MOV",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        // adds VX to I
                        0x1E => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} I, V{:X}",
                                pc,
                                instr,
                                "ADD",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        // sets I to the location of the sprite for the character in VX
                        0x29 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}",
                                pc,
                                instr,
                                "SPRITECHAR",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        // stores the BCD representation of VX with the most significant
                        // digit at adress in I, the other digits at I + 1, I + 2
                        0x33 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V{:X}",
                                pc,
                                instr,
                                "MOVBCD",
                                (instr & 0x0F00) >> 8,
                            );
                        }
                        // stores V0 -> VX included in memory starting at adress I
                        0x55 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} (I), V0-VX",
                                pc, instr, "MOVM",
                            );
                        }
                        // fills V0 -> VX with values read from memory starting with address I
                        0x65 => {
                            let _res = writeln!(
                                f,
                                "{:04X}    {:04X}          {} V0-VX, (I)",
                                pc, instr, "MOVM",
                            );
                        }
                        _ => panic!(format!(
                            "unsupported instruction {:04X} within nimble: {:X}",
                            instr, nimble
                        )),
                    },
                    _ => panic!(format!("unsupported instruction with nimble: {:X}", nimble)),
                };
                pc += 2;
            }
            Ok(())
        }
    }
}
