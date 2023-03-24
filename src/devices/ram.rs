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

    pub fn read64(&self, addr: u64) -> u64 {
        if !self.check_addr(addr) {
            panic!("Invalid address: {0:#x}", addr);
        }

        let mut data: u64 = 0;
        for i in 0..std::mem::size_of::<u64>() {
            // Add the data from the next 8 bits to the data by shifting to the right place and oring it
            data |= (self.read(addr + i as u64) as u64) << (i * 8);
        }
        data
    }

    pub fn write64(&mut self, addr: u64, data: u64) {
        if !self.check_addr(addr) {
            panic!("Invalid address: {0:#x}", addr);
        }

        for i in 0..std::mem::size_of::<u64>() {
            // Write the data to the next 8 bits by shifting to the right place and replacing the data
            self.write(addr + i as u64, (data >> (i * 8)) as u8);
        }
    }
}

impl Device8Bit for Ram {
    fn read(&self, addr: u64) -> u8 {
        if !self.check_addr(addr) {
            panic!("Invalid address: {0:#x}", addr);
        }

        self.buffer.data[addr as usize]
    }

    fn write(&mut self, addr: u64, data: u8) {
        if !self.check_addr(addr) {
            panic!("Invalid address: {0:#x}", addr);
        }

        self.buffer.data[addr as usize] = data;
    }

    fn size(&self) -> usize {
        self.buffer.data.len()
    }
}
