pub type StrResult<T> = Result<T, &'static str>;
use crate::mbc::MBC;

pub struct MBC0 {
    rom: Vec<u8>,
}

impl MBC0 {
    pub fn new(data: Vec<u8>) -> StrResult<MBC0> {
        Ok(MBC0 { rom: data })
    }
}

impl MBC for MBC0 {
    fn readrom(&self, a: u16) -> u8 { self.rom[a as usize] }
    fn readram(&self, _a: u16) -> u8 { 0 }
    fn writerom(&mut self, _a: u16, _v: u8) { () }
    fn writeram(&mut self, _a: u16, _v: u8) { () }
}
