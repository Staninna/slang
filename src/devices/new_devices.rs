trait BitsOps {
    // Returns a zero value of this type.
    fn zero() -> Self;
}

impl BitsOps for u8 {
    fn zero() -> Self {
        0
    }
}

impl BitsOps for u64 {
    fn zero() -> Self {
        0
    }
}

// A trait for devices with generic bits.
trait Device<Bits> {
    /// Reads a value from an address of this device.
    fn read(&self, addr: u64) -> Bits;
    /// Writes a value to an address of this device.
    fn write(&mut self, addr: u64, value: Bits);
    /// Returns the size of the buffer of this device.
    fn size(&self) -> usize;
}

// A region in the device mapper that contains a device and its address range.
struct Region<Bits> {
    device: Box<dyn Device<Bits>>,
    dev_name: String,
    start: u64,
    end: u64,
}

impl<Bits> Region<Bits> {
    // Creates a new region with a device and its starting address.
    fn new(device: Box<dyn Device<Bits>>, dev_name: String, start_addr: u64) -> Self {
        let end_addr = start_addr + device.size() as u64;
        Self {
            device,
            dev_name,
            start: start_addr,
            end: end_addr,
        }
    }

    // Checks if an address is within the address range of this region.
    fn check_addr(&self, addr: u64) -> bool {
        addr >= self.start && addr <= self.end
    }
}

// A device mapper that maps device regions to address ranges.
struct DeviceMapper<Bits> {
    regions: Vec<Region<Bits>>,
}

impl<Bits: BitsOps> DeviceMapper<Bits> {
    // Creates a new device mapper with no regions.
    fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    // Maps a device to an address range.
    fn map(&mut self, device: Box<dyn Device<Bits>>, dev_name: String, start: u64) {
        self.regions.push(Region::new(device, dev_name, start));
    }

    }

    // Finds the region that contains an address.
    fn find_region(&self, addr: u64) -> Option<&Region<Bits>> {
        self.regions.iter().find(|region| region.check_addr(addr))
    }

    // Finds the mutable reference to the region that contains an address.
    fn find_region_mut(&mut self, addr: u64) -> Option<&mut Region<Bits>> {
        self.regions
            .iter_mut()
            .find(|region| region.check_addr(addr))
    }

    // Reads a value from an address in the device mapper.
    fn read(&mut self, addr: u64) -> Bits {
        if let Some(region) = self.find_region(addr) {
            let offset = addr - region.start;
            region.device.read(offset)
        }
        // If no region is found, return 0.
        else {
            Bits::zero()
        }
    }

    // Writes a value to an address in the device mapper.
    fn write(&mut self, addr: u64, value: Bits) {
        if let Some(region) = self.find_region_mut(addr) {
            let offset = addr - region.start;
            region.device.write(offset, value)
        }
        // If no region is found, panic.
        else {
            panic!("No region/device found for address: {:#x}", addr);
        }
    }
}