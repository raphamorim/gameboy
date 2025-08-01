#[cfg(not(feature = "ffi"))]
use std::io::prelude::*;
#[cfg(not(feature = "ffi"))]
use std::{fs, io, path};
#[cfg(feature = "ffi")]
use alloc::vec::Vec;
#[cfg(feature = "ffi")]
use alloc::vec;

use crate::mbc::{ram_banks, rom_banks, MemoryBankController};
pub type StrResult<T> = Result<T, &'static str>;

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_on: bool,
    banking_mode: u8,
    rombank: usize,
    rambank: usize,
    #[cfg(not(feature = "ffi"))]
    savepath: Option<path::PathBuf>,
    #[cfg(feature = "ffi")]
    savepath: Option<()>,
    rombanks: usize,
    rambanks: usize,
}

impl MBC1 {
    #[cfg(not(feature = "ffi"))]
    pub fn new(data: Vec<u8>, file: path::PathBuf) -> StrResult<MBC1> {
        let (svpath, rambanks) = match data[0x147] {
            0x02 => (None, ram_banks(data[0x149])),
            0x03 => (Some(file.with_extension("gbsave")), ram_banks(data[0x149])),
            _ => (None, 0),
        };
        let rombanks = rom_banks(data[0x148]);
        let ramsize = rambanks * 0x2000;

        let mut res = MBC1 {
            rom: data,
            ram: ::core::iter::repeat(0u8).take(ramsize).collect(),
            ram_on: false,
            banking_mode: 0,
            rombank: 1,
            rambank: 0,
            savepath: svpath,
            rombanks,
            rambanks,
        };
        res.loadram().map(|_| res)
    }
    
    #[cfg(feature = "ffi")]
    pub fn new(data: Vec<u8>, _file: ()) -> StrResult<MBC1> {
        let rambanks = match data[0x147] {
            0x02 | 0x03 => ram_banks(data[0x149]),
            _ => 0,
        };
        let rombanks = rom_banks(data[0x148]);
        let ramsize = rambanks * 0x2000;

        Ok(MBC1 {
            rom: data,
            ram: vec![0u8; ramsize],
            ram_on: false,
            banking_mode: 0,
            rombank: 1,
            rambank: 0,
            savepath: None,
            rombanks,
            rambanks,
        })
    }

    #[allow(dead_code)]
    pub fn new_without_save(data: Vec<u8>) -> StrResult<MBC1> {
        let (svpath, rambanks) = match data[0x147] {
            0x02 => (None, ram_banks(data[0x149])),
            0x03 => (None, ram_banks(data[0x149])),
            _ => (None, 0),
        };
        let rombanks = rom_banks(data[0x148]);
        let ramsize = rambanks * 0x2000;

        let res = MBC1 {
            rom: data,
            ram: ::core::iter::repeat(0u8).take(ramsize).collect(),
            ram_on: false,
            banking_mode: 0,
            rombank: 1,
            rambank: 0,
            savepath: svpath,
            rombanks,
            rambanks,
        };
        // res.loadram().map(|_| res)
        Ok(res)
    }

    #[cfg(not(feature = "ffi"))]
    fn loadram(&mut self) -> StrResult<()> {
        match self.savepath {
            None => Ok(()),
            Some(ref savepath) => {
                let mut data = vec![];
                match fs::File::open(savepath).and_then(|mut f| f.read_to_end(&mut data))
                {
                    Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
                    Err(_) => Err("Could not open save file"),
                    Ok(..) => {
                        self.ram = data;
                        Ok(())
                    }
                }
            }
        }
    }
    
    #[cfg(feature = "ffi")]
    fn loadram(&mut self) -> StrResult<()> {
        Ok(())
    }
}

#[cfg(not(feature = "ffi"))]
impl Drop for MBC1 {
    fn drop(&mut self) {
        match self.savepath {
            None => {}
            Some(ref path) => {
                let _ = fs::File::create(path).and_then(|mut f| f.write_all(&self.ram));
            }
        };
    }
}

#[cfg(feature = "ffi")]
impl Drop for MBC1 {
    fn drop(&mut self) {
        // No-op for no_std
    }
}

impl MemoryBankController for MBC1 {
    fn readrom(&self, a: u16) -> u8 {
        let bank = if a < 0x4000 {
            if self.banking_mode == 0 {
                0
            } else {
                self.rombank & 0xE0
            }
        } else {
            self.rombank
        };
        let idx = (bank * 0x4000) | ((a as usize) & 0x3FFF);
        *self.rom.get(idx).unwrap_or(&0xFF)
    }
    fn readram(&self, a: u16) -> u8 {
        if !self.ram_on {
            return 0xFF;
        }
        let rambank = if self.banking_mode == 1 {
            self.rambank
        } else {
            0
        };
        self.ram[(rambank * 0x2000) | ((a & 0x1FFF) as usize)]
    }

    fn writerom(&mut self, a: u16, v: u8) {
        match a {
            0x0000..=0x1FFF => {
                self.ram_on = v & 0xF == 0xA;
            }
            0x2000..=0x3FFF => {
                let lower_bits = match (v as usize) & 0x1F {
                    0 => 1,
                    n => n,
                };
                self.rombank = ((self.rombank & 0x60) | lower_bits) % self.rombanks;
            }
            0x4000..=0x5FFF => {
                if self.rombanks > 0x20 {
                    let upper_bits = (v as usize & 0x03) % (self.rombanks >> 5);
                    self.rombank = self.rombank & 0x1F | (upper_bits << 5)
                }
                if self.rambanks > 1 {
                    self.rambank = (v as usize) & 0x03;
                }
            }
            0x6000..=0x7FFF => {
                self.banking_mode = v & 0x01;
            }
            _ => panic!("Could not write to {:04X} (MBC1)", a),
        }
    }

    fn writeram(&mut self, a: u16, v: u8) {
        if !self.ram_on {
            return;
        }
        let rambank = if self.banking_mode == 1 {
            self.rambank
        } else {
            0
        };
        let address = (rambank * 0x2000) | ((a & 0x1FFF) as usize);
        if address < self.ram.len() {
            self.ram[address] = v;
        }
    }
}
