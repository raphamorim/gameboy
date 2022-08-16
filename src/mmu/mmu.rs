use crate::gameboy;
use crate::gpu::gpu::Gpu;
use crate::mmu::rtc::Rtc;
use std::iter::repeat;

use crate::mmu::timer::Timer;

use crate::input;

pub const WRAM_SIZE: usize = 32 << 10; // CGB has 32K (8 banks * 4 KB/bank), GB has 8K
pub const HIRAM_SIZE: usize = 0x7f;

#[derive(Copy, Debug, Clone)]
pub enum Speed {
    Normal,
    Double,
}

#[derive(Debug)]
pub struct Mmu {
    // memory: u32,
    pub if_: u8,
    pub ie_: u8,
    // pub inte: u8,
    // Flag indicating BIOS is mapped in
    // BIOS is unmapped with the first instruction above 0x00FF
    // _inbios: u16,

    // Memory regions (initialised at reset time)
    // Heap
    wram: Box<[u8; WRAM_SIZE]>,
    hiram: Box<[u8; HIRAM_SIZE]>,
    battery: bool,
    ram: Vec<u8>,
    rom: Vec<u8>,
    rombank: u16,
    target: gameboy::Target,
    rambank: u8,
    wrambank: u8,
    mode: bool,
    ramon: bool,
    /// The speed that the gameboy is operating at and whether a switch has been
    /// requested
    pub speed: Speed,
    pub speedswitch: bool,
    sound: bool,
    mbc: Mbc,
    /// Flag if this is a CGB cartridge or not
    is_cgb: bool,
    /// Flag if this is a SGB cartridge or not
    is_sgb: bool,
    // _bios: [],
    // _eram: [],
    pub input: Box<input::Input>,
    pub gpu: Box<Gpu>,
    pub timer: Box<Timer>,
    pub rtc: Box<Rtc>,
    // pub sgb: Option<Box<Sgb>>,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Mbc {
    Unknown,
    Omitted,
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
}

impl Mmu {
    pub fn new(target: gameboy::Target) -> Mmu {
        Mmu {
            target: target,
            rom: Vec::new(),
            ram: Vec::new(),
            wram: Box::new([0; WRAM_SIZE]),
            hiram: Box::new([0; HIRAM_SIZE]),
            input: Box::new(input::Input::new()),
            rtc: Box::new(Rtc::new()),
            timer: Box::new(Timer::new()),
            gpu: Box::new(Gpu::new()),
            is_cgb: false,
            is_sgb: false,
            rombank: 1,
            battery: false,
            mode: false,
            speed: Speed::Normal,
            speedswitch: false,
            sound: false,
            ie_: 0,
            if_: 0,
            mbc: Mbc::Unknown,
            rambank: 0,
            wrambank: 0,
            ramon: false,
            // sgb: None,
        }
    }

    pub fn power_on(&mut self) {
        // See http://nocash.emubase.de/pandocs.htm#powerupsequence
        self.wb(0xFF05, 0);
        self.wb(0xFF06, 0);
        self.wb(0xFF07, 0);
        self.wb(0xFF10, 0x80);
        self.wb(0xFF11, 0xBF);
        self.wb(0xFF12, 0xF3);
        self.wb(0xFF14, 0xBF);
        self.wb(0xFF16, 0x3F);
        self.wb(0xFF16, 0x3F);
        self.wb(0xFF17, 0);
        self.wb(0xFF19, 0xBF);
        self.wb(0xFF1A, 0x7F);
        self.wb(0xFF1B, 0xFF);
        self.wb(0xFF1C, 0x9F);
        self.wb(0xFF1E, 0xFF);
        self.wb(0xFF20, 0xFF);
        self.wb(0xFF21, 0);
        self.wb(0xFF22, 0);
        self.wb(0xFF23, 0xBF);
        self.wb(0xFF24, 0x77);
        self.wb(0xFF25, 0xF3);
        self.wb(0xFF26, 0xF1);
        self.wb(0xFF40, 0x91);
        self.wb(0xFF42, 0);
        self.wb(0xFF43, 0);
        self.wb(0xFF45, 0);
        self.wb(0xFF47, 0xFC);
        self.wb(0xFF48, 0xFF);
        self.wb(0xFF49, 0xFF);
        self.wb(0xFF4A, 0);
        self.wb(0xFF4B, 0);
    }

    // Read 8-bit byte from a given address
    pub fn nop(&mut self, address: u16) -> u8 {
        address as u8
    }

    /// Reads a byte at the given address
    pub fn rb(&self, addr: u16) -> u8 {
        // println!("-> getting... {:#06x}", addr);

        match addr >> 12 {
            // Always mapped in as first bank of cartridge
            0x0 | 0x1 | 0x2 | 0x3 => self.rom[addr as usize],

            // Swappable banks of ROM, there may be a total of more than 2^16
            // bytes in the ROM, so we use u32 here.
            0x4 | 0x5 | 0x6 | 0x7 => {
                self.rom[(((self.rombank as u32) << 14) | ((addr as u32) & 0x3fff)) as usize]
            }

            0x8 | 0x9 => self.gpu.vram()[(addr & 0x1FFF) as usize],

            0xa | 0xb => {
                // Swappable banks of RAM
                if self.ramon {
                    if self.rtc.current & 0x08 != 0 {
                        self.rtc.regs[(self.rtc.current & 0x7) as usize]
                    } else {
                        self.ram[(((self.rambank as u16) << 12) | (addr & 0x1FFF)) as usize]
                    }
                } else {
                    0xff
                }
            }

            // e000-fdff same as c000-ddff
            0xe | 0xc => self.wram[(addr & 0x1FFF) as usize],
            0xd => self.wram[(((self.rombank as u16) << 12) | (addr & 0x1FFF)) as usize],

            0xf => {
                if addr < 0xfe00 {
                    // mirrored RAM
                    // self.rb(addr & 0xdfff)
                    self.wram[(addr & 0x1FFF) as usize]
                } else if addr < 0xFEA0 {
                    // sprite attribute table (oam)
                    self.gpu.oam[(addr & 0xFF) as usize]
                } else if addr < 0xff00 {
                    // unusable ram
                    0
                } else if addr < 0xff80 {
                    // I/O ports
                    self.ioreg_rb(addr)
                } else if addr < 0xffff {
                    // High RAM
                    self.hiram[(addr & 0x7f) as usize]
                } else {
                    // 0xff
                    self.ie_
                }
            }

            _ => 0,
        }
    }

    /// Reads a value from a known IO type register
    fn ioreg_rb(&self, addr: u16) -> u8 {
        match (addr >> 4) & 0xf {
            // joypad data, http://nocash.emubase.de/pandocs.htm#joypadinput
            // interrupts, http://nocash.emubase.de/pandocs.htm#interrupts
            // timer, http://nocash.emubase.de/pandocs.htm#timeranddividerregisters
            //
            // TODO: serial data transfer
            // http://nocash.emubase.de/pandocs.htm#serialdatatransferlinkcable
            0x0 => match addr & 0xf {
                0x0 => self.input.rb(addr),
                0x4 => self.timer.div,
                0x5 => self.timer.tima,
                0x6 => self.timer.tma,
                0x7 => self.timer.tac,
                0xf => self.if_,
                _ => 0xff,
            },

            // Sound info: http://nocash.emubase.de/pandocs.htm#soundcontroller
            0x1 | 0x2 | 0x3 => 0xff,

            0x4 => {
                if addr == 0xff4d {
                    let b = match self.speed {
                        Speed::Normal => 0x00,
                        Speed::Double => 0x80,
                    };
                    b | (self.speedswitch as u8)
                } else {
                    self.gpu.rb(addr)
                }
            }
            0x5 | 0x6 => self.gpu.rb(addr),

            0x7 => {
                if addr == 0xff70 {
                    self.wrambank as u8
                } else {
                    0xff
                }
            }

            _ => 0xff,
        }
    }

    // Read 16-bit word from a given address
    pub fn rw(&mut self, address: u16) -> u16 {
        (self.rb(address) as u16) | ((self.rb(address + 1) as u16) << 8)
    }

    /// Writes a byte at the given address
    pub fn wb(&mut self, addr: u16, val: u8) {
        // More information about mappings can be found online at
        //      http://nocash.emubase.de/pandocs.htm#memorymap
        // println!("<- saving... {:#06x} {}" ,addr, val);
        match addr >> 12 {
            0x0 | 0x1 => match self.mbc {
                Mbc::Mbc1 | Mbc::Mbc3 | Mbc::Mbc5 => {
                    self.ramon = val & 0xf == 0xa;
                }
                Mbc::Mbc2 => {
                    if addr & 0x100 == 0 {
                        self.ramon = !self.ramon;
                    }
                }
                Mbc::Unknown | Mbc::Omitted => {}
            },

            0x2 | 0x3 => {
                let val = val as u16;
                match self.mbc {
                    Mbc::Mbc1 => {
                        self.rombank = (self.rombank & 0x60) | (val & 0x1f);
                        if self.rombank == 0 {
                            self.rombank = 1;
                        }
                    }
                    Mbc::Mbc2 => {
                        if addr & 0x100 != 0 {
                            self.rombank = val & 0xf;
                        }
                    }
                    Mbc::Mbc3 => {
                        let val = val & 0x7f;
                        self.rombank = val + if val != 0 { 0 } else { 1 };
                    }
                    Mbc::Mbc5 => {
                        if addr >> 12 == 0x2 {
                            self.rombank = (self.rombank & 0xff00) | val;
                        } else {
                            let val = (val & 1) << 8;
                            self.rombank = (self.rombank & 0x00ff) | val;
                        }
                    }
                    Mbc::Unknown | Mbc::Omitted => {}
                }
            }

            0x4 | 0x5 => {
                match self.mbc {
                    Mbc::Mbc1 => {
                        if !self.mode {
                            // ROM banking mode
                            self.rombank = (self.rombank & 0x1f) | (((val as u16) & 0x3) << 5);
                        } else {
                            // RAM banking mode
                            self.rambank = val & 0x3;
                        }
                    }
                    Mbc::Mbc3 => {
                        self.rtc.current = val & 0xf;
                        self.rambank = val & 0x3
                    }
                    Mbc::Mbc5 => {
                        self.rambank = val & 0xf;
                    }
                    Mbc::Unknown | Mbc::Omitted | Mbc::Mbc2 => {}
                }
            }

            0x6 | 0x7 => match self.mbc {
                Mbc::Mbc1 => {
                    self.mode = val & 0x1 != 0;
                }
                Mbc::Mbc3 => {
                    self.rtc.latch(val);
                }
                _ => {}
            },

            0x8 | 0x9 => {
                self.gpu.vram_mut()[(addr & 0x1fff) as usize] = val;
                if addr < 0x9800 {
                    self.gpu.update_tile(addr);
                }
            }

            0xa | 0xb => {
                if self.ramon {
                    if self.rtc.current & 0x8 != 0 {
                        self.rtc.wb(addr, val);
                    } else {
                        let val = if self.mbc == Mbc::Mbc2 {
                            val & 0xf
                        } else {
                            val
                        };
                        self.ram[(((self.rambank as u16) << 12) | (addr & 0x1fff)) as usize] = val;
                    }
                }
            }

            0xc | 0xe => {
                self.wram[(addr & 0xfff) as usize] = val;
            }
            0xd => {
                self.wram[(((self.wrambank as u16) << 12) | (addr & 0xfff)) as usize] = val;
            }

            0xf => {
                if addr < 0xfe00 {
                    self.wb(addr & 0xdfff, val); // mirrored RAM
                } else if addr < 0xfea0 {
                    self.gpu.oam[(addr & 0xff) as usize] = val;
                } else if addr < 0xff00 {
                    // unusable ram
                } else if addr < 0xff80 {
                    self.ioreg_wb(addr, val);
                } else if addr < 0xffff {
                    self.hiram[(addr & 0x7f) as usize] = val;
                } else {
                    self.ie_ = val;
                }
            }

            _ => {}
        }
    }

    fn ioreg_wb(&mut self, addr: u16, val: u8) {
        // debug!("ioreg_wb {:x} {:x}", addr, val);
        match (addr >> 4) & 0xf {
            // TODO: serial data transfer
            // http://nocash.emubase.de/pandocs.htm#serialdatatransferlinkcable
            0x0 => match addr & 0xf {
                0x0 => {
                    self.input.wb(addr, val);
                }
                0x4 => {
                    self.timer.div = 0;
                }
                0x5 => {
                    self.timer.tima = val;
                }
                0x6 => {
                    self.timer.tma = val;
                }
                0x7 => {
                    self.timer.tac = val;
                    self.timer.update();
                }
                0xf => {
                    self.if_ = val;
                }
                _ => {}
            },

            // Sound info: http://nocash.emubase.de/pandocs.htm#soundcontroller
            // TODO: sound registers
            0x1 | 0x2 | 0x3 => {
                if addr == 0xff26 {
                    self.sound = val != 0;
                }
            }

            0x4 | 0x5 | 0x6 => {
                // See http://nocash.emubase.de/pandocs.htm#cgbregisters
                match addr {
                    0xff46 => Gpu::oam_dma_transfer(self, val),
                    0xff55 => Gpu::hdma_dma_transfer(self, val),
                    0xff4d => {
                        if self.is_cgb {
                            if val & 0x01 != 0 {
                                self.speedswitch = true;
                            } else {
                                self.speedswitch = false;
                            }
                        }
                    }
                    _ => self.gpu.wb(addr, val),
                }
            }

            // WRAM banks only for CGB mode, see
            //      http://nocash.emubase.de/pandocs.htm#cgbregisters
            0x7 => {
                if addr == 0xff70 {
                    let val = val & 0x7; /* only bits 0-2 are used */
                    self.wrambank = if val != 0 { val } else { 1 };
                }
            }

            _ => {
                panic!("address {:?} not implemented", addr);
            }
        }
    }

    pub fn switch_speed(&mut self) {
        self.speedswitch = false;
        self.speed = match self.speed {
            Speed::Normal => Speed::Double,
            Speed::Double => Speed::Normal,
        };
    }

    // Write 16-bit byte to a given address
    pub fn ww(&mut self, address: u16, value: u16) {
        self.wb(address, (value & 0xFF) as u8);
        self.wb(address + 1, (value >> 8) as u8);
    }

    pub fn ram_size(&self) -> usize {
        // See http://nocash.emubase.de/pandocs.htm#thecartridgeheader
        match self.rom[0x0149] {
            0x00 => 0,
            0x01 => 2 << 10,  // 2KB
            0x02 => 8 << 10,  // 8KB
            0x03 => 32 << 10, // 32KB
            _ => {
                panic!("Unknown ram size");
                #[allow(unreachable_code)]
                0
            }
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
        self.battery = true;
        self.mbc = Mbc::Unknown;
        match self.rom[0x0147] {
            0x00 |      // rom only
            0x08 => {   // rom + ram
                self.battery = false;
                self.mbc = Mbc::Omitted;
            }

            0x09 => {   // rom + ram + battery
                self.mbc = Mbc::Omitted;
            }

            0x01 |      // rom + mbc1
            0x02 => {   // rom + mbc1 + ram
                self.battery = false;
                self.mbc = Mbc::Mbc1;
            }
            0x03 => {   // rom + mbc1 + ram + batt
                self.mbc = Mbc::Mbc1;
            }

            0x05 => {   // rom + mbc2
                self.battery = false;
                self.mbc = Mbc::Mbc2;
            }
            0x06 => {   // rom + mbc2 + batt
                self.mbc = Mbc::Mbc2;
            }

            0x11 |      // rom + mbc3
            0x12 => {   // rom + mbc3 + ram
                self.battery = false;
                self.mbc = Mbc::Mbc3;
            }
            0x0f |      // rom + mbc3 + timer + batt
            0x10 |      // rom + mbc3 + timer + ram + batt
            0x13 => {   // rom + mbc3 + ram + batt
                self.mbc = Mbc::Mbc3;
            }

            0x19 |      // <>
            0x1a |      // ram
            0x1c |      // rumble
            0x1d => {   // rumble + ram
                self.battery = false;
                self.mbc = Mbc::Mbc5;
            }
            0x1b |      // ram + battery
            0x1e => {   // rumble + ram + batter
                self.mbc = Mbc::Mbc5;
            }

            n => { panic!("unknown cartridge inserted: {:x}", n); }
        }

        self.ram = repeat(0u8).take(self.ram_size()).collect();
        if self.target == gameboy::GameBoyColor {
            self.is_cgb = self.rom[0x0143] & 0x80 != 0;
            self.gpu.is_cgb = self.is_cgb;
        }

        if self.target == gameboy::SuperGameBoy || self.target == gameboy::GameBoyColor {
            // self.is_sgb = self.rom[0x0146] == 0x03;
            // if self.is_sgb {
            //     self.sgb = Some(Box::new(Sgb::new()));
            //     self.gpu.is_sgb = self.is_sgb;
            // }
        }
    }
}
