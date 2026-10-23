pub type StrResult<T> = Result<T, &'static str>;

use std::io::prelude::*;
use std::{fs, path};

mod mbc0;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;

pub trait MemoryBankController: Send {
    fn readrom(&self, a: u16) -> u8;
    fn readram(&self, a: u16) -> u8;
    fn writerom(&mut self, a: u16, v: u8);
    fn writeram(&mut self, a: u16, v: u8);

    /// Writes battery-backed RAM to the save file, if the cartridge has one.
    fn save(&self) -> StrResult<()> {
        Ok(())
    }

    /// Banking registers and cartridge RAM, for save states.
    fn save_state(&self, _w: &mut crate::state::Writer) {}
    fn load_state(&mut self, _r: &mut crate::state::Reader) -> crate::state::Result {
        Ok(())
    }

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
) -> StrResult<Box<dyn MemoryBankController + 'static>> {
    let file = filepath;

    if data.len() < 0x150 {
        return Err("ROM is too small to contain a cartridge header");
    }

    match data[0x147] {
        0x00 => {
            mbc0::MBC0::new(data).map(|v| Box::new(v) as Box<dyn MemoryBankController>)
        }
        0x01..=0x03 => mbc1::MBC1::new(data, file)
            .map(|v| Box::new(v) as Box<dyn MemoryBankController>),
        0x05..=0x06 => mbc2::MBC2::new(data, file)
            .map(|v| Box::new(v) as Box<dyn MemoryBankController>),
        0x0F..=0x13 => mbc3::MBC3::new(data, file)
            .map(|v| Box::new(v) as Box<dyn MemoryBankController>),
        0x19..=0x1E => mbc5::MBC5::new(data, file)
            .map(|v| Box::new(v) as Box<dyn MemoryBankController>),
        _ => Err("Unsupported MBC type"),
    }
}

/// Writes `chunks` to `path` through a temporary file, so a crash mid-write
/// never leaves a truncated save behind.
pub(crate) fn write_save(path: &path::Path, chunks: &[&[u8]]) -> StrResult<()> {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = path::PathBuf::from(tmp);
    let mut file = fs::File::create(&tmp).map_err(|_| "Could not create save file")?;
    for chunk in chunks {
        file.write_all(chunk)
            .map_err(|_| "Could not write save file")?;
    }
    file.sync_all().map_err(|_| "Could not flush save file")?;
    fs::rename(&tmp, path).map_err(|_| "Could not replace save file")
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

#[allow(dead_code)]
fn check_checksum(data: &[u8]) -> StrResult<()> {
    let mut value: u8 = 0;
    for item in data.iter().take(0x14D).skip(0x134) {
        value = value.wrapping_sub(*item).wrapping_sub(1);
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
