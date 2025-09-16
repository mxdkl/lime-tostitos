#[derive(Debug, Clone, Copy)]
pub struct Permission(u8);

impl Permission {
    const READ:    u8 = 0b001;
    const WRITE:   u8 = 0b010;
    const EXECUTE: u8 = 0b100;

    fn check(&self, mask: u8) -> bool {
        (self.0 & mask) == mask
    }

}


#[derive(Debug)]
pub struct Memory {
    data:  Vec<u8>,
    perms: Vec<Permission>,
}

impl Memory {
    /// Constructor
    pub fn new(size: usize) -> Self {
        Memory {
            data:  vec![0; size],
            perms: vec![Permission(Permission::READ | Permission::WRITE); size],
        }
    }

    pub fn read8(&self, addr: u64) -> u8 {
        let addr = addr as usize;
        if addr >= self.data.len() {
            panic!("Memory read out of bounds: addr=0x{:x}, size=0x{:x}", addr, self.data.len());
        }
        if !self.perms[addr].check(Permission::READ) {
            panic!("Memory read permission denied at addr=0x{:x}", addr);
        }
        self.data[addr]
    }

    pub fn read16(&self, addr: u64) -> u16 {
        let addr = addr as usize;
        if addr + 1 >= self.data.len() {
            panic!("Memory read out of bounds: addr=0x{:x}, size=0x{:x}", addr, self.data.len());
        }
        if !self.perms[addr].check(Permission::READ) || !self.perms[addr + 1].check(Permission::READ) {
            panic!("Memory read permission denied at addr=0x{:x}", addr);
        }
        u16::from_le_bytes([self.data[addr], self.data[addr + 1]])
    }

    pub fn read32(&self, addr: u64) -> u32 {
        let addr = addr as usize;
        if addr + 3 >= self.data.len() {
            panic!("Memory read out of bounds: addr=0x{:x}, size=0x{:x}", addr, self.data.len());
        }
        if !self.perms[addr].check(Permission::READ) || !self.perms[addr + 1].check(Permission::READ) ||
           !self.perms[addr + 2].check(Permission::READ) || !self.perms[addr + 3].check(Permission::READ) {
            panic!("Memory read permission denied at addr=0x{:x}", addr);
        }
        u32::from_le_bytes([self.data[addr], self.data[addr + 1], self.data[addr + 2], self.data[addr + 3]])
    }

    pub fn write8(&mut self, addr: u64, val: u8) {
        let addr = addr as usize;
        if addr >= self.data.len() {
            panic!("Memory write out of bounds: addr=0x{:x}, size=0x{:x}", addr, self.data.len());
        }
        if !self.perms[addr].check(Permission::WRITE) {
            panic!("Memory write permission denied at addr=0x{:x}", addr);
        }
        self.data[addr] = val;
    }

    pub fn write16(&mut self, addr: u64, val: u16) {
        let addr = addr as usize;
        if addr + 1 >= self.data.len() {
            panic!("Memory write out of bounds: addr=0x{:x}, size=0x{:x}", addr, self.data.len());
        }
        if !self.perms[addr].check(Permission::WRITE) || !self.perms[addr + 1].check(Permission::WRITE) {
            panic!("Memory write permission denied at addr=0x{:x}", addr);
        }
        let bytes = val.to_le_bytes();
        self.data[addr] = bytes[0];
        self.data[addr + 1] = bytes[1];
    }

    pub fn write32(&mut self, addr: u64, val: u32) {
        let addr = addr as usize;
        if addr + 3 >= self.data.len() {
            panic!("Memory write out of bounds: addr=0x{:x}, size=0x{:x}", addr, self.data.len());
        }
        if !self.perms[addr].check(Permission::WRITE) || !self.perms[addr + 1].check(Permission::WRITE) ||
           !self.perms[addr + 2].check(Permission::WRITE) || !self.perms[addr + 3].check(Permission::WRITE) {
            panic!("Memory write permission denied at addr=0x{:x}", addr);
        }
        let bytes = val.to_le_bytes();
        self.data[addr] = bytes[0];
        self.data[addr + 1] = bytes[1];
        self.data[addr + 2] = bytes[2];
        self.data[addr + 3] = bytes[3];
    }
}

