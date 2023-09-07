use crate::mbc::{ram_banks, MBC};
pub type StrResult<T> = Result<T, &'static str>;

use std::io::prelude::*;
use std::path;
use std::{fs, io, time};

pub struct MBC3 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rombank: usize,
    rambank: usize,
    rambanks: usize,
    selectrtc: bool,
    ram_on: bool,
    savepath: Option<path::PathBuf>,
    rtc_ram: [u8; 5],
    rtc_ram_latch: [u8; 5],
    rtc_zero: Option<u64>,
}

impl MBC3 {
    pub fn new(data: Vec<u8>, file: path::PathBuf) -> StrResult<MBC3> {
        let subtype = data[0x147];
        let svpath = match subtype {
            0x0F | 0x10 | 0x13 => Some(file.with_extension("gbsave")),
            _ => None,
        };
        let rambanks = match subtype {
            0x10 | 0x12 | 0x13 => ram_banks(data[0x149]),
            _ => 0,
        };
        let ramsize = rambanks * 0x2000;
        let rtc = match subtype {
            0x0F | 0x10 => Some(0),
            _ => None,
        };

        let mut res = MBC3 {
            rom: data,
            ram: ::std::iter::repeat(0u8).take(ramsize).collect(),
            rombank: 1,
            rambank: 0,
            rambanks: rambanks,
            selectrtc: false,
            ram_on: false,
            savepath: svpath,
            rtc_ram: [0u8; 5],
            rtc_ram_latch: [0u8; 5],
            rtc_zero: rtc,
        };
        res.loadram().map(|_| res)
    }

    fn loadram(&mut self) -> StrResult<()> {
        match self.savepath {
            None => Ok(()),
            Some(ref savepath) => {
                let mut file = match fs::File::open(savepath) {
                    Ok(f) => f,
                    Err(ref e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
                    Err(..) => return Err("Could not read existing save file"),
                };
                let mut rtc_bytes = [0; 8];
                file.read_exact(&mut rtc_bytes)
                    .map_err(|_| "Could not read RTC")?;
                let rtc = u64::from_be_bytes(rtc_bytes);
                if self.rtc_zero.is_some() {
                    self.rtc_zero = Some(rtc);
                }
                let mut data = vec![];
                match file.read_to_end(&mut data) {
                    Err(..) => Err("Could not read ROM"),
                    Ok(..) => {
                        self.ram = data;
                        Ok(())
                    }
                }
            }
        }
    }

    fn latch_rtc_reg(&mut self) {
        self.calc_rtc_reg();
        self.rtc_ram_latch.clone_from_slice(&self.rtc_ram);
    }

    fn calc_rtc_reg(&mut self) {
        // Do not modify regs when halted
        if self.rtc_ram[4] & 0x40 == 0x40 {
            return;
        }

        let tzero = match self.rtc_zero {
            Some(t) => time::UNIX_EPOCH + time::Duration::from_secs(t),
            None => return,
        };

        if self.compute_difftime() == self.rtc_zero {
            // No time has passed. Do not alter registers
            return;
        }

        let difftime = match time::SystemTime::now().duration_since(tzero) {
            Ok(n) => n.as_secs(),
            _ => 0,
        };
        self.rtc_ram[0] = (difftime % 60) as u8;
        self.rtc_ram[1] = ((difftime / 60) % 60) as u8;
        self.rtc_ram[2] = ((difftime / 3600) % 24) as u8;
        let days = difftime / (3600 * 24);
        self.rtc_ram[3] = days as u8;
        self.rtc_ram[4] = (self.rtc_ram[4] & 0xFE) | (((days >> 8) & 0x01) as u8);
        if days >= 512 {
            self.rtc_ram[4] |= 0x80;
            self.calc_rtc_zero();
        }
    }

    fn compute_difftime(&self) -> Option<u64> {
        if self.rtc_zero.is_none() {
            return None;
        }
        let mut difftime = match time::SystemTime::now().duration_since(time::UNIX_EPOCH)
        {
            Ok(t) => t.as_secs(),
            Err(_) => {
                panic!("System clock is set to a time before the unix epoch (1970-01-01)")
            }
        };
        difftime -= self.rtc_ram[0] as u64;
        difftime -= (self.rtc_ram[1] as u64) * 60;
        difftime -= (self.rtc_ram[2] as u64) * 3600;
        let days = ((self.rtc_ram[4] as u64 & 0x1) << 8) | (self.rtc_ram[3] as u64);
        difftime -= days * 3600 * 24;
        Some(difftime)
    }

    fn calc_rtc_zero(&mut self) {
        self.rtc_zero = self.compute_difftime();
    }
}

impl Drop for MBC3 {
    fn drop(&mut self) {
        match self.savepath {
            None => {}
            Some(ref path) => {
                let mut file = match fs::File::create(path) {
                    Ok(f) => f,
                    Err(..) => return,
                };
                let rtc = match self.rtc_zero {
                    Some(t) => t,
                    None => 0,
                };
                let mut ok = true;
                if ok {
                    let rtc_bytes = rtc.to_be_bytes();
                    ok = file.write_all(&rtc_bytes).is_ok();
                };
                if ok {
                    let _ = file.write_all(&*self.ram);
                };
            }
        };
    }
}

impl MBC for MBC3 {
    fn readrom(&self, a: u16) -> u8 {
        let idx = if a < 0x4000 {
            a as usize
        } else {
            self.rombank * 0x4000 | ((a as usize) & 0x3FFF)
        };
        *self.rom.get(idx).unwrap_or(&0xFF)
    }
    fn readram(&self, a: u16) -> u8 {
        if !self.ram_on {
            return 0xFF;
        }
        if !self.selectrtc && self.rambank < self.rambanks {
            self.ram[self.rambank * 0x2000 | ((a as usize) & 0x1FFF)]
        } else if self.selectrtc && self.rambank < 5 {
            self.rtc_ram_latch[self.rambank]
        } else {
            0xFF
        }
    }
    fn writerom(&mut self, a: u16, v: u8) {
        match a {
            0x0000..=0x1FFF => self.ram_on = (v & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                self.rombank = match v & 0x7F {
                    0 => 1,
                    n => n as usize,
                }
            }
            0x4000..=0x5FFF => {
                self.selectrtc = v & 0x8 == 0x8;
                self.rambank = (v & 0x7) as usize;
            }
            0x6000..=0x7FFF => self.latch_rtc_reg(),
            _ => panic!("Could not write to {:04X} (MBC3)", a),
        }
    }
    fn writeram(&mut self, a: u16, v: u8) {
        if !self.ram_on {
            return;
        }
        if !self.selectrtc && self.rambank < self.rambanks {
            self.ram[self.rambank * 0x2000 | ((a as usize) & 0x1FFF)] = v;
        } else if self.selectrtc && self.rambank < 5 {
            self.calc_rtc_reg();
            let vmask = match self.rambank {
                0 | 1 => 0x3F,
                2 => 0x1F,
                4 => 0xC1,
                _ => 0xFF,
            };
            self.rtc_ram[self.rambank] = v & vmask;
            self.calc_rtc_zero();
        }
    }
}
