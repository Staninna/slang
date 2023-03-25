use super::device::{Buffer, Device8Bit};

pub struct Ram {
    buffer: Buffer,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: Buffer::new(size),
        }
    }
}

impl Device8Bit for Ram {
    fn read(&self, addr: u64) -> u8 {
        if !self.check_addr(addr) {
            panic!("Address out of bounds: {0:#x}", addr);
        }

        self.buffer.data[addr as usize]
    }

    fn write(&mut self, addr: u64, data: u8) {
        if !self.check_addr(addr) {
            panic!("Address out of bounds: {0:#x}", addr);
        }

        self.buffer.data[addr as usize] = data;
    }

    fn size(&self) -> usize {
        self.buffer.data.len()
    }
}
