// Memory Management Unit

pub struct Mmu {
    memory: u32
}

impl Mmu {
    // Read 8-bit byte from a given address
    pub fn rb(addr: u8) -> u8 {
        0
    }
    // Read 16-bit word from a given address
    pub fn rw(addr: u8) -> u8 {
        0
    }
    // Write 8-bit byte to a given address
    pub fn wb(addr: u8, val: u8) -> u8 {
        0
    }
    // Write 16-bit word to a given address
    pub fn ww(addr: u16, val: u16) -> u16 {
        0
    }
}