use crate::dev_map::device::Device;

pub const STDOUT_SIZE: usize = 0x1000;

pub struct Stdout;

impl Stdout {
    pub fn new() -> Self {
        Self
    }
}

impl Device for Stdout {
    fn read(&self, _addr: u64) -> u8 {
        0
    }

    fn write(&mut self, _addr: u64, value: u8) {
        print!("{}", value as char);
    }

    fn size(&self) -> usize {
        STDOUT_SIZE
    }
}
