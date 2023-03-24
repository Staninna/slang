use super::device::{Buffer, Device64Bit};

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

impl Device64Bit for Registers {
    fn read(&self, addr: u64) -> u64 {
        if !self.check_addr(addr) {
            panic!("Invalid address: {}", addr)
        }

        let mut data: u64 = 0;
        for i in 0..std::mem::size_of::<u64>() {
            // Add the data from the next 8 bits to the data by shifting to the right place and oring it
            data |= (self.buffer.data[(addr + i as u64) as usize] as u64) << (i * 8);
        }
        data
    }

    fn write(&mut self, addr: u64, data: u64) {
        if !self.check_addr(addr) {
            panic!("Invalid address: {}", addr)
        }

        for i in 0..std::mem::size_of::<u64>() {
            // Write the data to the next 8 bits by shifting to the right place and replacing the data
            self.buffer.data[(addr + i as u64) as usize] = (data >> (i * 8)) as u8;
        }
    }

    fn size(&self) -> usize {
        self.buffer.data.len()
    }
}
