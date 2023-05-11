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
pub trait Device<Bits> {
    /// Reads a value from an address of this device.
    fn read(&self, addr: u64) -> Bits;
    /// Writes a value to an address of this device.
    fn write(&mut self, addr: u64, value: Bits);
    /// Returns the size of the buffer of this device.
    fn size(&self) -> usize;
}
