use crate::dev_map::device::{Buffer, Device};

pub struct Rom {
    buffer: Buffer,
    writeable: bool,
}

impl Rom {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: Buffer::new(size),
            writeable: false,
        }
    }

    pub fn flash(&mut self, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.buffer.write(i, *byte);
        }
    }

    pub fn dump(&self, amount_bytes: usize) {
        let len = self.buffer.size();

        for i in 0..amount_bytes.min(len) {
            print!("{:02x} ", self.buffer.read(i));
            // Every 18 bytes, print a newline
            if i % 18 == 17 {
                println!();
            }
        }
        println!();
        println!();
    }
}

impl Device for Rom {
    fn read(&self, addr: u64) -> u8 {
        let offset = addr as usize;
        self.buffer.read(offset)
    }

    fn write(&mut self, addr: u64, value: u8) {
        if self.writeable {
            let offset = addr as usize;
            self.buffer.write(offset, value);
        } else {
            panic!("Attempted to write to ROM");
        }
    }

    fn size(&self) -> usize {
        self.buffer.size()
    }
}
