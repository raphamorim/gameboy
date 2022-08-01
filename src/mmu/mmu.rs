// Memory Management Unit

use crate::gpu::gpu::Gpu;

use crate::mmu::timer::Timer;

pub const WRAM_SIZE: usize = 0x8000;
pub const ZRAM_SIZE: usize = 0x7F;

pub struct Mmu {
    // memory: u32,
    pub f_flag: u8,
    // pub inte: u8,
    // Flag indicating BIOS is mapped in
    // BIOS is unmapped with the first instruction above 0x00FF
    // _inbios: u16,

    // Memory regions (initialised at reset time)
    // Heap
    wram: Box<[u8; WRAM_SIZE]>,
    zram: Box<[u8; ZRAM_SIZE]>,
    rom: Vec<u8>,
    rombank: u16,
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
            wram: Box::new([0; WRAM_SIZE]),
            zram: Box::new([0; ZRAM_SIZE]),
            timer: Box::new(Timer::new()),
            gpu: Box::new(Gpu::new()),
            rombank: 1,
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

    // Read 8-bit byte to a given u8 address
    pub fn rr8b(&mut self, address: u8) -> u8 {
        self.r8b(address as u16)
    }

    // Read 8-bit word from a given address
    pub fn r8b(&mut self, address: u16) -> u8 {
        match address {
            // ROM0
            0x0000..=0x3FFF => self.rom[address as usize],
            // ROM1 (unbanked) (16k)
            0x4000..=0x7FFF => {
                self.rom[(((self.rombank as u32) << 14) | ((address as u32) & 0x3fff)) as usize]
            }
            // Graphics VRAM (8k)
            0x8000..=0x9FFF => self.r8b(address),
            // // External RAM (8k)
            // 0xA000..=0xBFFF => self.mbc.readram(address),
            // // Working RAM (8k) and RAM shadow
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF],
            // 0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wrambank * 0x1000) | address as usize & 0x0FFF],

            // // Graphics: object attribute memory
            0xFE00..=0xFE9F => self.r8b(address),
            // Zero-page
            // 0xFF00 => self.keypad.rb(),
            // 0xFF01..=0xFF02 => self.nop(address),
            // 0xFF04..=0xFF07 => self.nop(address),
            // 0xFF0F => self.nop(address),
            // 0xFF10..=0xFF3F => self.nop(address),
            // 0xFF4D => 0,
            // 0xFF40..=0xFF4F => self.nop(address),
            // 0xFF51..=0xFF55 => self.nop(address),
            // 0xFF68..=0xFF6B => self.nop(address),
            // 0xFF70 => self.nop(address),
            // 0xFF80..=0xFFFE => self.nop(address),
            0xFFFF => self.f_flag,
            _ => 0,
        }
    }

    // Read 16-bit word from a given address
    pub fn r16b(&mut self, address: u16) -> u16 {
        (self.r8b(address) as u16) | ((self.r8b(address + 1) as u16) << 8)
    }

    // Write 8-bit byte to a given address
    pub fn ww8b(&mut self, address: u8, value: u8) {
        self.w8b(address as u16, value);
    }

    // Write 8-bit byte to a given address
    pub fn w8b(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.nop(address),
            0x8000..=0x9FFF => self.nop(address),
            0xA000..=0xBFFF => self.nop(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.nop(address),
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.nop(address),
            0xFE00..=0xFE9F => self.nop(address),
            0xFF00 => self.nop(address),
            0xFF01..=0xFF02 => self.nop(address),
            0xFF04..=0xFF07 => self.nop(address),
            0xFF10..=0xFF3F => self.nop(address),
            0xFF46 => self.nop(address),
            0xFF4D => 0,
            0xFF40..=0xFF4F => self.nop(address),
            0xFF51..=0xFF55 => self.nop(address),
            0xFF68..=0xFF6B => self.nop(address),
            0xFF0F => self.nop(address),
            0xFF70 => 0,
            0xFF80..=0xFFFE => self.nop(address),
            0xFFFF => self.nop(address),
            _ => 0,
        };
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
