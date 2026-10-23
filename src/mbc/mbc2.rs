use std::io::prelude::*;
use std::{fs, io, path};

use crate::mbc::{rom_banks, write_save, MemoryBankController};
pub type StrResult<T> = Result<T, &'static str>;

pub struct MBC2 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    ram_on: bool,
    rombank: usize,
    savepath: Option<path::PathBuf>,
    rombanks: usize,
}

impl MBC2 {
    pub fn new(data: Vec<u8>, file: Option<path::PathBuf>) -> StrResult<MBC2> {
        let svpath = match data[0x147] {
            0x05 => None,
            0x06 => file.map(|f| f.with_extension("gbsave")),
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
}

impl Drop for MBC2 {
    fn drop(&mut self) {
        let _ = self.save();
    }
}

impl MemoryBankController for MBC2 {
    fn save_state(&self, w: &mut crate::state::Writer) {
        use crate::state::State;
        self.ram.save_state(w);
        self.ram_on.save_state(w);
        self.rombank.save_state(w);
    }

    fn load_state(&mut self, r: &mut crate::state::Reader) -> crate::state::Result {
        use crate::state::State;
        self.ram.load_state(r)?;
        self.ram_on.load_state(r)?;
        self.rombank.load_state(r)?;
        Ok(())
    }

    fn save(&self) -> StrResult<()> {
        match self.savepath {
            None => Ok(()),
            Some(ref path) => write_save(path, &[&self.ram]),
        }
    }
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
