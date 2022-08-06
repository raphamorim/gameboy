// Memory Management Unit

use crate::gpu::gpu::Gpu;

use crate::mmu::timer::Timer;

pub const WRAM_SIZE: usize = 0x8000;
pub const ZRAM_SIZE: usize = 0x7F;
pub const HIRAM_SIZE: usize = 0x7f;

pub struct Mmu {
    // memory: u32,
    pub ie_: u8,
    pub f_flag: u8,
    // pub inte: u8,
    // Flag indicating BIOS is mapped in
    // BIOS is unmapped with the first instruction above 0x00FF
    // _inbios: u16,

    // Memory regions (initialised at reset time)
    // Heap
    wram: Box<[u8; WRAM_SIZE]>,
    zram: Box<[u8; ZRAM_SIZE]>,
    hiram: Box<[u8; HIRAM_SIZE]>,
    ram: Vec<u8>,
    rom: Vec<u8>,
    rombank: u16,
    rambank: u8,
    wrambank: u8,
    mode: bool,
    ramon: bool,
    speedswitch: bool,
    sound_on: bool,
    // _bios: [],
    // _eram: [],
    pub gpu: Box<Gpu>,
    pub timer: Box<Timer>,
}

impl Mmu {
    pub fn new() -> Mmu {
        Mmu {
            f_flag: 0,
            rom: Vec::new(),
            ram: Vec::new(),
            wram: Box::new([0; WRAM_SIZE]),
            zram: Box::new([0; ZRAM_SIZE]),
            hiram: Box::new([0; HIRAM_SIZE]),
            timer: Box::new(Timer::new()),
            gpu: Box::new(Gpu::new()),
            rombank: 1,
            mode: false,
            speedswitch: false,
            sound_on: false,
            ie_: 0,
            rambank: 0,
            wrambank: 0,
            ramon: false,
        }
    }

    pub fn power_on(&mut self) {
        // See http://nocash.emubase.de/pandocs.htm#powerupsequence
        self.w8b(0xFF05, 0);
        self.w8b(0xFF06, 0);
        self.w8b(0xFF07, 0);
        self.w8b(0xFF10, 0x80);
        self.w8b(0xFF11, 0xBF);
        self.w8b(0xFF12, 0xF3);
        self.w8b(0xFF14, 0xBF);
        self.w8b(0xFF16, 0x3F);
        self.w8b(0xFF16, 0x3F);
        self.w8b(0xFF17, 0);
        self.w8b(0xFF19, 0xBF);
        self.w8b(0xFF1A, 0x7F);
        self.w8b(0xFF1B, 0xFF);
        self.w8b(0xFF1C, 0x9F);
        self.w8b(0xFF1E, 0xFF);
        self.w8b(0xFF20, 0xFF);
        self.w8b(0xFF21, 0);
        self.w8b(0xFF22, 0);
        self.w8b(0xFF23, 0xBF);
        self.w8b(0xFF24, 0x77);
        self.w8b(0xFF25, 0xF3);
        self.w8b(0xFF26, 0xF1);
        self.w8b(0xFF40, 0x91);
        self.w8b(0xFF42, 0);
        self.w8b(0xFF43, 0);
        self.w8b(0xFF45, 0);
        self.w8b(0xFF47, 0xFC);
        self.w8b(0xFF48, 0xFF);
        self.w8b(0xFF49, 0xFF);
        self.w8b(0xFF4A, 0);
        self.w8b(0xFF4B, 0);
    }

    // Read 8-bit byte from a given address
    pub fn nop(&mut self, address: u16) -> u8 {
        address as u8
    }

    /// Reads a byte at the given address
    pub fn r8b(&self, addr: u16) -> u8 {
        println!("-> getting... {:#06x}", addr);

        match addr >> 12 {
            // Always mapped in as first bank of cartridge
            0x0 | 0x1 | 0x2 | 0x3 => self.rom[addr as usize],

            // Swappable banks of ROM, there may be a total of more than 2^16
            // bytes in the ROM, so we use u32 here.
            0x4 | 0x5 | 0x6 | 0x7 => {
                self.rom[(((self.rombank as u32) << 14) | ((addr as u32) & 0x3fff)) as usize]
            }

            0x8 | 0x9 => self.gpu.vram()[(addr & 0x1fff) as usize],

            0xa | 0xb => {
                // Swappable banks of RAM
                if self.ramon {
                    // if self.rtc.current & 0x08 != 0 {
                    //     self.rtc.regs[(self.rtc.current & 0x7) as usize]
                    // } else {
                    self.ram[(((self.rambank as u16) << 12) | (addr & 0x1fff)) as usize]
                    // }
                } else {
                    0xff
                }
            }

            // e000-fdff same as c000-ddff
            0xe | 0xc => self.wram[(addr & 0xfff) as usize],
            0xd => self.wram[(((self.rombank as u16) << 12) | (addr & 0xfff)) as usize],

            0xf => {
                if addr < 0xfe00 {
                    // mirrored RAM
                    self.r8b(addr & 0xdfff)
                } else if addr < 0xfea0 {
                    // sprite attribute table (oam)
                    self.gpu.oam[(addr & 0xff) as usize]
                } else if addr < 0xff00 {
                    // unusable ram
                    0xff
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
            0x0 => {
                match addr & 0xf {
                    // 0x0 => self.input.r8b(addr),
                    0x4 => self.timer.div,
                    0x5 => self.timer.tima,
                    0x6 => self.timer.tma,
                    0x7 => self.timer.tac,
                    // 0xf => self.if_,

                    _ => 0xff,
                }
            }

            // Sound info: http://nocash.emubase.de/pandocs.htm#soundcontroller
            0x1 | 0x2 | 0x3 => 0xff,

            0x4 => {
                if addr == 0xff4d {
                    0x00 | (self.speedswitch as u8)
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

            _ => { 0xff }
        }
    }

    // Read 16-bit word from a given address
    pub fn r16b(&mut self, address: u16) -> u16 {
        (self.r8b(address) as u16) | ((self.r8b(address + 1) as u16) << 8)
    }

    /// Writes a byte at the given address
    pub fn w8b(&mut self, addr: u16, val: u8) {
        // More information about mappings can be found online at
        //      http://nocash.emubase.de/pandocs.htm#memorymap
        println!("<- saving... {:#06x} {}" ,addr, val);
        match addr >> 12 {
            0x0 | 0x1 => {
                self.ramon = val & 0xf == 0xa;
            }

            0x2 | 0x3 => {
                let val = val as u16;
                self.rombank = (self.rombank & 0x60) | (val & 0x1f);
                if self.rombank == 0 {
                    self.rombank = 1;
                }
            }

            0x4 | 0x5 => {
                if !self.mode {
                    // ROM banking mode
                    self.rombank = (self.rombank & 0x1f) | (((val as u16) & 0x3) << 5);
                } else {
                    // RAM banking mode
                    self.rambank = val & 0x3;
                }
            }

            0x6 | 0x7 => {
                self.mode = val & 0x1 != 0;
            }

            0x8 | 0x9 => {
                self.gpu.vram_mut()[(addr & 0x1fff) as usize] = val;
                if addr < 0x9800 {
                    self.gpu.update_tile(addr);
                }
            }

            0xa | 0xb => {
                if self.ramon {
                    // if self.rtc.current & 0x8 != 0 {
                    //     self.rtc.wb(addr, val);
                    // } else {
                    //     let val = if self.mbc == Mbc::Mbc2 {val & 0xf} else {val};
                        self.ram[(((self.rambank as u16) << 12) |
                                 (addr & 0x1fff)) as usize] = val;
                    // }
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
                    self.w8b(addr & 0xdfff, val); // mirrored RAM
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
            0x0 => {
                match addr & 0xf {
                    0x0 => {
                        // self.input.wb(addr, val);
                    }
                    0x4 => { self.timer.div = 0; }
                    0x5 => { self.timer.tima = val; }
                    0x6 => { self.timer.tma = val; }
                    0x7 => {
                        self.timer.tac = val;
                        self.timer.update();
                    }
                    // 0xf => { self.if_ = val; }
                    _ => {}
                }
            }

            // Sound info: http://nocash.emubase.de/pandocs.htm#soundcontroller
            // TODO: sound registers
            0x1 | 0x2 | 0x3 => {
                if addr == 0xff26 {
                    self.sound_on = val != 0;
                }
            }

            0x4 | 0x5 | 0x6 => {
                // See http://nocash.emubase.de/pandocs.htm#cgbregisters
                match addr {
                    // 0xff46 => gpu::Gpu::oam_dma_transfer(self, val),
                    // 0xff55 => gpu::Gpu::hdma_dma_transfer(self, val),
                    0xff4d => {
                        // if self.is_cgb {
                            if val & 0x01 != 0 {
                                self.speedswitch = true;
                            } else {
                                self.speedswitch = false;
                            }
                        // }
                    }
                    _ => self.gpu.wb(addr, val),
                }
            }

            // WRAM banks only for CGB mode, see
            //      http://nocash.emubase.de/pandocs.htm#cgbregisters
            0x7 => {
                if addr == 0xff70 {
                    let val = val & 0x7; /* only bits 0-2 are used */
                    self.wrambank = if val != 0 {val} else {1};
                }
            }

            _ => { 
                // dpanic!("unimplemented address {:x}", addr); 
            }
        }
    }

    // Write 16-bit byte to a given address
    pub fn w16b(&mut self, address: u16, value: u16) {
        self.w8b(address, (value & 0xFF) as u8);
        self.w8b(address + 1, (value >> 8) as u8);
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
    }
}
