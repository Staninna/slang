use crate::dev_map::device::Buffer;

pub struct Registers {
    buffer: Buffer,
}

impl Registers {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: Buffer::new(size),
        }
    }

    pub fn read(&self, addr: u64) -> u64 {
        let offset = addr as usize;
        let mut bytes = [0; 8];
        for i in 0..8 {
            bytes[i] = self.buffer.read(offset + i as usize);
        }
        u64::from_le_bytes(bytes)
    }

    pub fn write(&mut self, addr: u64, value: u64) {
        let offset = addr as usize;
        for i in 0..8 {
            self.buffer.write(offset + i, (value >> (i * 8)) as u8);
        }
    }
}
