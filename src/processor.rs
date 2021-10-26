use crate::CHIP_8_HEIGHT;
use crate::CHIP_8_RAM_SIZE;
use crate::CHIP_8_WIDTH;
use rand::Rng;

enum ProgramCounter {
    Next,
    Skip,
    Jump(u16),
}

impl ProgramCounter {
    fn skip_if(condition: bool) -> ProgramCounter {
        if condition {
            ProgramCounter::Skip
        } else {
            ProgramCounter::Next
        }
    }
}

struct Cpu {
    v: [u8; 16],
    i: u16,
    dt: u8,
    st: u8,
    pc: u16,
    sp: usize,
    ram: [u8; CHIP_8_RAM_SIZE],
    stack: [u16; 16],
    display: [[u8; 64]; 32],
}

/// An instruction is like:
/// **** nnnn nnnn nnnn (nnn or addr)
/// **** **** **** nnnn (n or nibble)
/// **** xxxx **** **** (x)
/// **** **** yyyy **** (y)
/// **** **** kkkk kkkk (kk or byte)
impl Cpu {
    fn i00e0(&mut self) -> ProgramCounter {
        for h in 0..CHIP_8_HEIGHT {
            for w in 0..CHIP_8_WIDTH {
                self.display[h][w] = 0x00;
            }
        }
        ProgramCounter::Next
    }

    fn i00ee(&mut self) -> ProgramCounter {
        let addr = self.stack[self.sp];
        self.sp -= 1;

        ProgramCounter::Jump(addr)
    }

    fn i1nnn(&mut self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }

    fn i2nnn(&mut self, nnn: u16) -> ProgramCounter {
        self.sp += 1;
        self.stack[self.sp] = self.pc;

        ProgramCounter::Jump(nnn)
    }

    fn i3xkk(&self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] == kk)
    }

    fn i4xkk(&self, x: usize, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] != kk)
    }

    fn i5xy0(&self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] == self.v[y])
    }

    fn i6xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.v[x] = kk;

        ProgramCounter::Next
    }

    fn i7xkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        self.v[x] += kk;

        ProgramCounter::Next
    }

    fn i8xy0(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] = self.v[y];

        ProgramCounter::Next
    }

    fn i8xy1(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] |= self.v[y];

        ProgramCounter::Next
    }

    fn i8xy2(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] &= self.v[y];

        ProgramCounter::Next
    }

    fn i8xy3(&mut self, x: usize, y: usize) -> ProgramCounter {
        self.v[x] ^= self.v[y];

        ProgramCounter::Next
    }

    fn i8xy4(&mut self, x: usize, y: usize) -> ProgramCounter {
        let sum = self.v[x] as u16 + self.v[y] as u16;

        self.v[0xF] = if sum > 0xFF { 1 } else { 0 };

        self.v[x] = sum as u8;

        ProgramCounter::Next
    }

    fn i8xy5(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx = self.v[x];
        let vy = self.v[y];

        self.v[0xF] = if vx > vy { 1 } else { 0 };

        self.v[x] = vx.wrapping_sub(vy);

        ProgramCounter::Next
    }

    fn i8xy6(&mut self, x: usize) -> ProgramCounter {
        self.v[0xF] = self.v[x] & 0x01;

        self.v[x] >>= 1;

        ProgramCounter::Next
    }

    fn i8xy7(&mut self, x: usize, y: usize) -> ProgramCounter {
        let vx = self.v[x];
        let vy = self.v[y];

        self.v[0xF] = if vy > vx { 1 } else { 0 };

        self.v[x] = vy.wrapping_sub(vx);

        ProgramCounter::Next
    }

    #[allow(non_snake_case)]
    fn i8x0E(&mut self, x: usize) -> ProgramCounter {
        self.v[0xF] = (self.v[x] & 0x80) >> 7;

        self.v[x] <<= 1;

        ProgramCounter::Next
    }

    fn i9xy0(&self, x: usize, y: usize) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x] != self.v[y])
    }

    #[allow(non_snake_case)]
    fn iAnnn(&mut self, nnn: u16) -> ProgramCounter {
        self.i = nnn;

        ProgramCounter::Next
    }

    #[allow(non_snake_case)]
    fn iBnnn(&self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(nnn + self.v[0x00] as u16)
    }

    #[allow(non_snake_case)]
    fn iCxkk(&mut self, x: usize, kk: u8) -> ProgramCounter {
        let random_byte: u8 = rand::thread_rng().gen_range(0..=255);

        self.v[x] = random_byte & kk;

        ProgramCounter::Next
    }

    #[allow(non_snake_case)]
    fn iDxyn(&mut self, x: usize, y: usize, n: usize) -> ProgramCounter {
        self.v[0xF] = 0x0;

        for byte_increment in 0..n {
            let sprite = self.ram[self.i as usize + byte_increment as usize];
            let y = (self.v[y] as usize + byte_increment) % CHIP_8_HEIGHT;

            for bit in 0..8 {
                let pixel = sprite & (0x80 >> (7 - bit));

                if pixel != 0x1 {
                    let x = (self.v[x] as usize + bit) % CHIP_8_WIDTH;

                    if self.display[y][x] == 0x1 {
                        self.v[0xF] = 0x1;
                    }

                    self.display[y][x] ^= pixel;
                }
            }
        }

        ProgramCounter::Next
    }
}
