use crate::vm::dev_map::device::Device;

pub const STDOUT_SIZE: usize = 0x1000;
pub const STDOUT_NEWLINE: u8 = 0xFF;

pub struct Stdout;

impl Stdout {
    pub fn new() -> Self {
        Self
    }
}

impl Device for Stdout {
    fn read(&self, _addr: u64) -> u8 {
        panic!("Cannot read from stdout");
    }

    fn write(&mut self, _addr: u64, value: u8) {
        match value {
            STDOUT_NEWLINE => print!("\n"),
            _ => print!("{}", value as char),
        }
    }

    fn size(&self) -> usize {
        STDOUT_SIZE
    }
}
