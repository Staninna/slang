pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0x00; size],
        }
    }

    // Reads a value from the buffer at an offset
    pub fn read(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    // Writes a value to the buffer at an offset
    pub fn write(&mut self, offset: usize, value: u8) {
        self.data[offset] = value;
    }

    // Returns the size of the buffer
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

// A trait for devices with generic bits.
pub trait Device {
    /// Reads a value from an address of this device 8 bits at a time.
    fn read(&self, addr: u64) -> u8;
    /// Writes a value to an address of this device 8 bits at a time.
    fn write(&mut self, addr: u64, value: u8);
    /// Returns the size of the buffer of this device.
    fn size(&self) -> usize;

    // Reads a value from an address of this device 64 bits at a time.
    fn read64(&self, addr: u64) -> u64 {
        let offset = addr;
        let mut value: u64 = 0;
        for i in 0..8 {
            value |= (self.read(offset + i) as u64) << (i * 8);
        }
        value
    }

    // Writes a value to an address of this device 64 bits at a time.
    fn write64(&mut self, addr: u64, value: u64) {
        let offset = addr;
        for i in 0..8 {
            self.write(offset + i, (value >> (i * 8)) as u8);
        }
    }
}
