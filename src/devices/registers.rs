use crate::dev_map::device::{Buffer, Device};

pub struct Registers {
    buffer: Buffer,
}

impl Registers {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: Buffer::new(size),
        }
    }
}

impl Device<u64> for Registers {
    fn read(&self, addr: u64) -> u64 {
        let offset = addr as usize;
        let mut value: u64 = 0;
        for i in 0..8 {
            value |= (self.buffer.read(offset + i) as u64) << (i * 8);
        }
        value
    }

    fn write(&mut self, addr: u64, value: u64) {
        let offset = addr as usize;
        for i in 0..8 {
            self.buffer.write(offset + i, (value >> (i * 8)) as u8);
        }
    }

    fn size(&self) -> usize {
        self.buffer.size()
    }
}
