pub struct Buffer {
    pub data: Vec<u8>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0x00; size],
        }
    }
}

pub trait Device64Bit {
    fn read(&self, addr: u64) -> u64;
    fn write(&mut self, addr: u64, data: u64);
    fn size(&self) -> usize;
    fn check_addr(&self, addr: u64) -> bool {
        return addr < self.size() as u64;
    }
}

pub trait Device8Bit {
    fn read(&self, addr: u64) -> u8;
    fn write(&mut self, addr: u64, data: u8);
    fn size(&self) -> usize;
    fn check_addr(&self, addr: u64) -> bool {
        return addr < self.size() as u64;
    }

    // Read 64 bits from a 8 bit device
    fn read64(&self, addr: u64) -> u64 {
        if !self.check_addr(addr) || !self.check_addr(addr + 7) {
            panic!("Invalid address: {0:#x}", addr);
        }

        let mut data: u64 = 0;
        for i in 0..std::mem::size_of::<u64>() {
            // Add the data from the next 8 bits to the data by shifting to the right place and oring it
            data |= (self.read(addr + i as u64) as u64) << (i * 8);
        }
        data
    }

    // Write 64 bits to a 8 bit device
    fn write64(&mut self, addr: u64, data: u64) {
        if !self.check_addr(addr) || !self.check_addr(addr + 7) {
            panic!("Invalid address: {0:#x}", addr);
        }

        for i in 0..std::mem::size_of::<u64>() {
            // Write the data to the next 8 bits by shifting to the right place and replacing the data
            self.write(addr + i as u64, (data >> (i * 8)) as u8);
        }
    }
}
