pub struct Ram {
    buf: [u8; 4096],
}

pub fn new_ram() -> Ram {
    Ram { buf: [0; 4096] }
}

impl Ram {}
