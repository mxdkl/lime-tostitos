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

    /// Read byte
    pub fn read_byte(&self, addr: usize) -> Option<u8> {
        if addr < self.data.len() && self.perms[addr].check(Permission::READ) {
            Some(self.data[addr])
        } else {
            None
        }
    }
}

