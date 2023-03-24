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
}
