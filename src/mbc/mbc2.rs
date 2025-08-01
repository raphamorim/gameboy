#[cfg(not(feature = "ffi"))]
use std::io::prelude::*;
#[cfg(not(feature = "ffi"))]
use std::{fs, io, path};
#[cfg(feature = "ffi")]
use alloc::vec::Vec;
#[cfg(feature = "ffi")]
use alloc::vec;

use crate::mbc::{rom_banks, MemoryBankController};
pub type StrResult<T> = Result<T, &'static str>;

pub struct MBC2 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_on: bool,
    rombank: usize,
    #[cfg(not(feature = "ffi"))]
    savepath: Option<path::PathBuf>,
    #[cfg(feature = "ffi")]
    savepath: Option<()>,
    rombanks: usize,
}

impl MBC2 {
    #[cfg(not(feature = "ffi"))]
    pub fn new(data: Vec<u8>, file: path::PathBuf) -> StrResult<MBC2> {
        let svpath = match data[0x147] {
            0x05 => None,
            0x06 => Some(file.with_extension("gbsave")),
            _ => None,
        };
        let rombanks = rom_banks(data[0x148]);

        let mut res = MBC2 {
            rom: data,
            ram: vec![0; 512],
            ram_on: false,
            rombank: 1,
            savepath: svpath,
            rombanks,
        };
        res.loadram().map(|_| res)
    }
    
    #[cfg(feature = "ffi")]
    pub fn new(data: Vec<u8>, _file: ()) -> StrResult<MBC2> {
        let rombanks = rom_banks(data[0x148]);

        Ok(MBC2 {
            rom: data,
            ram: vec![0; 512],
            ram_on: false,
            rombank: 1,
            savepath: None,
            rombanks,
        })
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
impl Drop for MBC2 {
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
impl Drop for MBC2 {
    fn drop(&mut self) {
        // No-op for no_std
    }
}

impl MemoryBankController for MBC2 {
    fn readrom(&self, a: u16) -> u8 {
        let bank = if a < 0x4000 { 0 } else { self.rombank };
        let idx = (bank * 0x4000) | ((a as usize) & 0x3FFF);
        *self.rom.get(idx).unwrap_or(&0xFF)
    }
    fn readram(&self, a: u16) -> u8 {
        if !self.ram_on {
            return 0xFF;
        }
        self.ram[(a as usize) & 0x1FF] | 0xF0
    }

    fn writerom(&mut self, a: u16, v: u8) {
        if let 0x0000..=0x3FFF = a {
            if a & 0x100 == 0 {
                self.ram_on = v & 0xF == 0xA;
            } else {
                self.rombank = match (v as usize) & 0x0F {
                    0 => 1,
                    n => n,
                } % self.rombanks;
            }
        }
    }

    fn writeram(&mut self, a: u16, v: u8) {
        if !self.ram_on {
            return;
        }
        self.ram[(a as usize) & 0x1FF] = v | 0xF0;
    }
}
