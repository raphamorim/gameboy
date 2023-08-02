use std::io::prelude::*;
use std::{path, fs, io};

use crate::mbc::{MBC, rom_banks};
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
            rombanks: rombanks,
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
                    Ok(..) => { self.ram = data; Ok(()) },
                }
            },
        }
    }
}

impl Drop for MBC2 {
    fn drop(&mut self) {
        match self.savepath {
            None => {},
            Some(ref path) =>
            {
                let _ = fs::File::create(path).and_then(|mut f| f.write_all(&*self.ram));
            },
        };
    }
}

impl MBC for MBC2 {
    fn readrom(&self, a: u16) -> u8 {
        let bank = if a < 0x4000 {
            0
        }
        else {
            self.rombank
        };
        let idx = bank * 0x4000 | ((a as usize) & 0x3FFF);
        *self.rom.get(idx).unwrap_or(&0xFF)
    }
    fn readram(&self, a: u16) -> u8 {
        if !self.ram_on { return 0xFF }
        self.ram[(a as usize) & 0x1FF] | 0xF0
    }

    fn writerom(&mut self, a: u16, v: u8) {
        match a {
            0x0000 ..= 0x3FFF => {
                if a & 0x100 == 0 {
                    self.ram_on = v & 0xF == 0xA;
                }
                else {
                    self.rombank = match (v as usize) & 0x0F {
                        0 => 1,
                        n => n,
                    } % self.rombanks;
                }
            },
            _ => {},
        }
    }

    fn writeram(&mut self, a: u16, v: u8) {
        if !self.ram_on { return }
        self.ram[(a as usize) & 0x1FF] = v | 0xF0;
    }
}
