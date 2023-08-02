use crate::mbc::{MBC, ram_banks, rom_banks};
pub type StrResult<T> = Result<T, &'static str>;

use std::fs::File;
use std::{io, path};
use std::io::prelude::*;

pub struct MBC5 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rombank: usize,
    rambank: usize,
    ram_on: bool,
    savepath: Option<path::PathBuf>,
    rombanks: usize,
    rambanks: usize,
}

impl MBC5 {
    pub fn new(data: Vec<u8>, file: path::PathBuf) -> StrResult<MBC5> {
        let subtype = data[0x147];
        let svpath = match subtype {
            0x1B | 0x1E => Some(file.with_extension("gbsave")),
            _ => None,
        };
        let rambanks = match subtype {
            0x1A | 0x1B | 0x1D | 0x1E => ram_banks(data[0x149]),
            _ => 0,
        };
        let ramsize = 0x2000 * rambanks;
        let rombanks = rom_banks(data[0x148]);

        let mut res = MBC5 {
            rom: data,
            ram: ::std::iter::repeat(0u8).take(ramsize).collect(),
            rombank: 1,
            rambank: 0,
            ram_on: false,
            savepath: svpath,
            rombanks: rombanks,
            rambanks: rambanks,
        };
        res.loadram().map(|_| res)
    }

    fn loadram(&mut self) -> StrResult<()> {
        match self.savepath {
            None => Ok(()),
            Some(ref savepath) => {
                let mut data = vec![];
                match File::open(&savepath).and_then(|mut f| f.read_to_end(&mut data)) {
                    Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
                    Err(_) => Err("Could not read RAM"),
                    Ok(..) => { self.ram = data; Ok(()) },
                }
            },
        }
    }
}

impl Drop for MBC5 {
    fn drop(&mut self) {
        match self.savepath {
            None => {},
            Some(ref path) =>
            {
                let _ = File::create(path).and_then(|mut f| f.write_all(&*self.ram));
            },
        };
    }
}

impl MBC for MBC5 {
    fn readrom(&self, a: u16) -> u8 {
        let idx = if a < 0x4000 { a as usize }
        else { self.rombank * 0x4000 | ((a as usize) & 0x3FFF) };
        *self.rom.get(idx).unwrap_or(&0)
    }
    fn readram(&self, a: u16) -> u8 {
        if !self.ram_on { return 0 }
        self.ram[self.rambank * 0x2000 | ((a as usize) & 0x1FFF)]
    }
    fn writerom(&mut self, a: u16, v: u8) {
        match a {
            0x0000 ..= 0x1FFF => self.ram_on = v & 0x0F == 0x0A,
            0x2000 ..= 0x2FFF => self.rombank = ((self.rombank & 0x100) | (v as usize)) % self.rombanks,
            0x3000 ..= 0x3FFF => self.rombank = ((self.rombank & 0x0FF) | (((v & 0x1) as usize) << 8)) % self.rombanks,
            0x4000 ..= 0x5FFF => self.rambank = ((v & 0x0F) as usize) % self.rambanks,
            0x6000 ..= 0x7FFF => { /* ? */ },
            _ => panic!("Could not write to {:04X} (MBC5)", a),
        }
    }
    fn writeram(&mut self, a: u16, v: u8) {
        if self.ram_on == false { return }
        self.ram[self.rambank * 0x2000 | ((a as usize) & 0x1FFF)] = v;
    }
}
