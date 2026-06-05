// 16 glyphs, one per hex digit
const CHIP8_FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pub memory: [u8; 4096],
    pub display: [bool; 64 * 32],
    pub stack: [u16; 16],
    pub v: [u8; 16],      // registers V0–VF
    pub i: u16,           // index register
    pub pc: u16,          // program counter
    pub sp: u8,           // stack pointer
    pub dt: u8,           // delay timer
    pub st: u8,           // sound timer
    pub keys: [bool; 16], // hex keypad state
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            display: [false; 64 * 32],
            stack: [0; 16],
            v: [0; 16],
            i: 0,
            pc: 0x200, // ROMs always load at 0x200
            sp: 0,
            dt: 0,
            st: 0,
            keys: [false; 16],
        };
        chip8.load_fonts();
        chip8
    }

    pub fn load_fonts(&mut self) {
        self.memory[0x000..0x050].copy_from_slice(&CHIP8_FONT);
    }

    pub fn load_rom(&mut self, path: &str) {
        let rom = std::fs::read(path).expect("Failed to read ROM");
        self.memory[0x200..(0x200 + rom.len())].copy_from_slice(&rom);
    }

    pub fn fetch(&mut self) -> u16 {
        let pc = self.pc as usize;
        let high = self.memory[pc] as u16; // 0x0X
        let low = self.memory[pc + 1] as u16; // 0x0Y
        self.pc += 2;
        (high << 8) | low // 0xXY
    }

    pub fn execute(&mut self, opcode: u16) {
        let n1 = (opcode & 0xF000) >> 12;
        let n2 = (opcode & 0x0F00) >> 8;
        let n3 = (opcode & 0x00F0) >> 4;
        let n4 = opcode & 0x000F;

        let x = n2 as usize;
        let y = n3 as usize;
        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = n4 as u8;

        match (n1, n2, n3, n4) {
            // 00E0 - clear screen
            (0x0, 0x0, 0xE, 0x0) => {
                self.display = [false; 64 * 32];
            }

            // 00EE - return from subroutine
            (0x0, 0x0, 0xE, 0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
            }

            // 1NNN - jump to NNN
            (0x1, _, _, _) => {
                self.pc = nnn;
            }

            // 2NNN - call subroutine at NNN
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }

            // 3XKK - skip if VX == KK
            (0x3, _, _, _) => {
                if self.v[x] == kk {
                    self.pc += 2;
                }
            }

            // 4XKK - skip if VX != KK
            (0x4, _, _, _) => {
                if self.v[x] != kk {
                    self.pc += 2;
                }
            }

            // 5XY0 - skip if VX == VY
            (0x5, _, _, 0x0) => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }

            // 6XKK - set VX = KK
            (0x6, _, _, _) => {
                self.v[x] = kk;
            }

            // 7XKK - set VX = VX + KK
            (0x7, _, _, _) => {
                self.v[x] = self.v[x].wrapping_add(kk);
            }

            // 8XY0 - set VX = VY
            (0x8, _, _, 0x0) => {
                self.v[x] = self.v[y];
            }

            // 8XY1 - set VX = VX OR VY
            (0x8, _, _, 0x1) => {
                self.v[x] |= self.v[y];
                self.v[0xF] = 0; // CHIP-8 quirk
            }

            // 8XY2 - set VX = VX AND VY
            (0x8, _, _, 0x2) => {
                self.v[x] &= self.v[y];
                self.v[0xF] = 0; // CHIP-8 quirk
            }

            // 8XY3 - set VX = VX XOR VY
            (0x8, _, _, 0x3) => {
                self.v[x] ^= self.v[y];
                self.v[0xF] = 0; // CHIP-8 quirk
            }

            // 8XY4 - set VX = VX + VY, VF = carry
            (0x8, _, _, 0x4) => {
                let (result, carry) = self.v[x].overflowing_add(self.v[y]);
                self.v[x] = result;
                self.v[0xF] = carry as u8;
            }

            // 8XY5 - set VX = VX - VY, VF = NOT borrow
            (0x8, _, _, 0x5) => {
                let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[x] = result;
                self.v[0xF] = !borrow as u8;
            }

            // 8XY6 - set VX = VX >> 1, VF = shifted out bit
            (0x8, _, _, 0x6) => {
                let bit = self.v[x] & 0x1;
                self.v[x] >>= 1;
                self.v[0xF] = bit;
            }

            // 8XY7 - set VX = VY - VX, VF = NOT borrow
            (0x8, _, _, 0x7) => {
                let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[x] = result;
                self.v[0xF] = !borrow as u8;
            }

            // 8XYE - set VX = VX << 1, VF = shifted out bit
            (0x8, _, _, 0xE) => {
                let bit = (self.v[x] & 0x80) >> 7;
                self.v[x] <<= 1;
                self.v[0xF] = bit;
            }

            // 9XY0 - skip if VX != VY
            (0x9, _, _, 0x0) => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }

            // ANNN - set I = NNN
            (0xA, _, _, _) => {
                self.i = nnn;
            }

            // BNNN - jump to NNN + V0
            (0xB, _, _, _) => {
                self.pc = nnn + self.v[0] as u16;
            }

            // CXKK - set VX = random byte AND KK
            (0xC, _, _, _) => {
                let rand: u8 = rand::random();
                self.v[x] = rand & kk;
            }

            // DXYN - draw sprite at (VX, VY), N rows tall, VF = collision
            (0xD, _, _, _) => {
                let x_pos = self.v[x] as usize % 64;
                let y_pos = self.v[y] as usize % 32;
                self.v[0xF] = 0;

                for row in 0..n as usize {
                    let sprite_byte = self.memory[self.i as usize + row];
                    for col in 0..8 {
                        let sprite_pixel = (sprite_byte >> (7 - col)) & 0x1;
                        let screen_x = x_pos + col;
                        let screen_y = y_pos + row;

                        if screen_x >= 64 || screen_y >= 32 {
                            continue; // clip at edges
                        }

                        let idx = screen_y * 64 + screen_x;
                        if sprite_pixel == 1 {
                            if self.display[idx] {
                                self.v[0xF] = 1; // collision
                            }
                            self.display[idx] ^= true;
                        }
                    }
                }
            }

            // EX9E - skip if key VX is pressed
            (0xE, _, 0x9, 0xE) => {
                if self.keys[self.v[x] as usize] {
                    self.pc += 2;
                }
            }

            // EXA1 - skip if key VX is NOT pressed
            (0xE, _, 0xA, 0x1) => {
                if !self.keys[self.v[x] as usize] {
                    self.pc += 2;
                }
            }

            // FX07 - set VX = delay timer
            (0xF, _, 0x0, 0x7) => {
                self.v[x] = self.dt;
            }

            // FX0A - wait for key press, store in VX (blocking)
            (0xF, _, 0x0, 0xA) => {
                let pressed = self.keys.iter().position(|&k| k);
                match pressed {
                    Some(key) => self.v[x] = key as u8,
                    None => self.pc -= 2, // rewind PC, retry next cycle
                }
            }

            // FX15 - set delay timer = VX
            (0xF, _, 0x1, 0x5) => {
                self.dt = self.v[x];
            }

            // FX18 - set sound timer = VX
            (0xF, _, 0x1, 0x8) => {
                self.st = self.v[x];
            }

            // FX1E - set I = I + VX
            (0xF, _, 0x1, 0xE) => {
                self.i += self.v[x] as u16;
            }

            // FX29 - set I = address of font sprite for digit VX
            (0xF, _, 0x2, 0x9) => {
                self.i = self.v[x] as u16 * 5;
            }

            // FX33 - store BCD of VX in memory at I, I+1, I+2
            (0xF, _, 0x3, 0x3) => {
                self.memory[self.i as usize] = self.v[x] / 100;
                self.memory[self.i as usize + 1] = (self.v[x] / 10) % 10;
                self.memory[self.i as usize + 2] = self.v[x] % 10;
            }

            // FX55 - store V0..VX in memory starting at I
            (0xF, _, 0x5, 0x5) => {
                for reg in 0..=x {
                    self.memory[self.i as usize + reg] = self.v[reg];
                }
            }

            // FX65 - load V0..VX from memory starting at I
            (0xF, _, 0x6, 0x5) => {
                for reg in 0..=x {
                    self.v[reg] = self.memory[self.i as usize + reg];
                }
            }

            _ => panic!("Unknown opcode: {:#06X}", opcode),
        }
    }

    pub fn set_key(&mut self, key_index: usize) {
        self.keys[key_index] = true;
    }

    pub fn unset_key(&mut self, key_index: usize) {
        self.keys[key_index] = false;
    }
}
