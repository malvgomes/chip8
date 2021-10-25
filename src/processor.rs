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
    sp: u8,
    stack: [u16; 16],
    display: [[bool; 64]; 32],
}

/// An instruction is like:
/// **** nnnn nnnn nnnn (nnn or addr)
/// **** **** **** nnnn (n or nibble)
/// **** xxxx **** **** (x)
/// **** **** yyyy **** (y)
/// **** **** kkkk kkkk (kk or byte)
impl Cpu {
    fn i00e0(&mut self) -> ProgramCounter {
        for h in 0..32 {
            for w in 0..64 {
                self.display[h][w] = false;
            }
        }
        ProgramCounter::Next
    }

    fn i00ee(&mut self) -> ProgramCounter {
        let addr = self.stack[self.sp as usize];
        self.sp -= 1;

        ProgramCounter::Jump(addr)
    }

    fn i1nnn(&mut self, nnn: u16) -> ProgramCounter {
        ProgramCounter::Jump(nnn)
    }

    fn i2nnn(&mut self, nnn: u16) -> ProgramCounter {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;

        ProgramCounter::Jump(nnn)
    }

    fn i3xkk(&self, x: u8, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x as usize] == kk)
    }

    fn i4xkk(&self, x: u8, kk: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x as usize] != kk)
    }

    fn i5xy0(&self, x: u8, y: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x as usize] == self.v[y as usize])
    }

    fn i6xkk(&mut self, x: u8, kk: u8) -> ProgramCounter {
        self.v[x as usize] = kk;

        ProgramCounter::Next
    }

    fn i7xkk(&mut self, x: u8, kk: u8) -> ProgramCounter {
        self.v[x as usize] += kk;

        ProgramCounter::Next
    }

    fn i8xy0(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.v[x as usize] = self.v[y as usize];

        ProgramCounter::Next
    }

    fn i8xy1(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.v[x as usize] |= self.v[y as usize];

        ProgramCounter::Next
    }

    fn i8xy2(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.v[x as usize] &= self.v[y as usize];

        ProgramCounter::Next
    }

    fn i8xy3(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.v[x as usize] ^= self.v[y as usize];

        ProgramCounter::Next
    }

    fn i8xy4(&mut self, x: u8, y: u8) -> ProgramCounter {
        let sum = self.v[x as usize] as u16 + self.v[y as usize] as u16;

        self.v[0xF] = if sum > 0xFF { 1 } else { 0 };

        self.v[x as usize] = sum as u8;

        ProgramCounter::Next
    }

    fn i8xy5(&mut self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        self.v[0xF] = if vx > vy { 1 } else { 0 };

        self.v[x as usize] = vx.wrapping_sub(vy);

        ProgramCounter::Next
    }

    fn i8xy6(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.v[0xF] = self.v[x as usize] & 0x01;

        self.v[x as usize] >>= 1;

        ProgramCounter::Next
    }

    fn i8xy7(&mut self, x: u8, y: u8) -> ProgramCounter {
        let vx = self.v[x as usize];
        let vy = self.v[y as usize];

        self.v[0xF] = if vy > vx { 1 } else { 0 };

        self.v[x as usize] = vy.wrapping_sub(vx);

        ProgramCounter::Next
    }

    #[allow(non_snake_case)]
    fn i8x0E(&mut self, x: u8, y: u8) -> ProgramCounter {
        self.v[0xF] = self.v[x as usize] * 0x01;

        self.v[x as usize] <<= 1;

        ProgramCounter::Next
    }

    fn i9xy0(&self, x: u8, y: u8) -> ProgramCounter {
        ProgramCounter::skip_if(self.v[x as usize] != self.v[y as usize])
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
    fn iCxkk(&mut self, x: u8, kk: u8) -> ProgramCounter {
        let random_byte: u8 = rand::thread_rng().gen_range(0..=255);

        self.v[x as usize] = random_byte & kk;

        ProgramCounter::Next
    }
}
