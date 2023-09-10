pub type StrResult<T> = Result<T, &'static str>;

use std::path;

mod mbc0;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;

pub trait MBC: Send {
    fn readrom(&self, a: u16) -> u8;
    fn readram(&self, a: u16) -> u8;
    fn writerom(&mut self, a: u16, v: u8);
    fn writeram(&mut self, a: u16, v: u8);

    fn romname(&self) -> String {
        const TITLE_START: u16 = 0x134;
        const CGB_FLAG: u16 = 0x143;

        let title_size = match self.readrom(CGB_FLAG) & 0x80 {
            0x80 => 11,
            _ => 16,
        };

        let mut result = String::with_capacity(title_size as usize);

        for i in 0..title_size {
            match self.readrom(TITLE_START + i) {
                0 => break,
                v => result.push(v as char),
            }
        }

        result
    }
}

pub fn get_mbc(
    data: Vec<u8>,
    filepath: Option<path::PathBuf>,
) -> StrResult<Box<dyn MBC + 'static>> {
    if filepath.is_none() {
        return mbc1::MBC1::new_without_save(data).map(|v| Box::new(v) as Box<dyn MBC>);
    }

    let file = filepath.unwrap();

    match data[0x147] {
        0x00 => mbc0::MBC0::new(data).map(|v| Box::new(v) as Box<dyn MBC>),
        0x01..=0x03 => mbc1::MBC1::new(data, file).map(|v| Box::new(v) as Box<dyn MBC>),
        0x05..=0x06 => mbc2::MBC2::new(data, file).map(|v| Box::new(v) as Box<dyn MBC>),
        0x0F..=0x13 => mbc3::MBC3::new(data, file).map(|v| Box::new(v) as Box<dyn MBC>),
        0x19..=0x1E => mbc5::MBC5::new(data, file).map(|v| Box::new(v) as Box<dyn MBC>),
        _ => Err("Unsupported MBC type"),
    }
}

fn ram_banks(v: u8) -> usize {
    match v {
        1 =>
        // "Listed in various unofficial docs as 2 KiB. However, a 2 KiB RAM chip was never
        // used in a cartridge. The source of this value is unknown."
        // Needed by some test roms. As we only deal in whole banks, just make it 1 8KiB bank.
        {
            1
        }
        2 => 1,
        3 => 4,
        4 => 16,
        5 => 8,
        _ => 0,
    }
}

fn rom_banks(v: u8) -> usize {
    if v <= 8 {
        2 << v
    } else {
        0
    }
}

fn check_checksum(data: &[u8]) -> StrResult<()> {
    let mut value: u8 = 0;
    for i in 0x134..0x14D {
        value = value.wrapping_sub(data[i]).wrapping_sub(1);
    }
    match data[0x14D] == value {
        true => Ok(()),
        false => Err("Cartridge checksum is invalid"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn checksum_zero() {
        let mut data = vec![0; 0x150];
        data[0x14D] = -(0x14D_i32 - 0x134_i32) as u8;
        super::check_checksum(&data).unwrap();
    }

    #[test]
    fn checksum_ones() {
        let mut data = vec![1; 0x150];
        data[0x14D] = (-(0x14D_i32 - 0x134_i32) * 2) as u8;
        super::check_checksum(&data).unwrap();
    }
}
