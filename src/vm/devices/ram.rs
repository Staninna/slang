use crate::vm::dev_map::device::{Buffer, Device};

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

impl Device for Ram {
    fn read(&self, addr: u64) -> u8 {
        let offset = addr as usize;
        self.buffer.read(offset)
    }

    fn write(&mut self, addr: u64, value: u8) {
        let offset = addr as usize;
        self.buffer.write(offset, value);
    }

    fn size(&self) -> usize {
        self.buffer.size()
    }
}
