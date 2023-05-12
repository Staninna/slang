use crate::dev_map::device::{Buffer, Device};

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

impl Device<u64> for Ram {
    fn read(&self, addr: u64) -> u64 {
        let offset = addr as usize;
        self.buffer.read(offset) as u64
    }

    fn write(&mut self, addr: u64, value: u64) {
        let offset = addr as usize;
        self.buffer.write(offset, value as u8);
    }

    fn size(&self) -> usize {
        self.buffer.size()
    }
}
