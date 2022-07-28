// Memory Management Unit

const WRAM_SIZE: usize = 0x8000;
const ZRAM_SIZE: usize = 0x7F;

pub struct Mmu {
    memory: u32,

    pub intf: u8,
    pub inte: u8,
    // Flag indicating BIOS is mapped in
    // BIOS is unmapped with the first instruction above 0x00FF
    _inbios: u16,

    // Memory regions (initialised at reset time)
    wram: [u8; WRAM_SIZE],
    zram: [u8; ZRAM_SIZE],
    // _rom: [],
    // _bios: [],
    // _eram: [],
}

impl Mmu {
    // Read 8-bit byte from a given address
    // pub fn rb(addr: u16) -> u16 {
    //     switch(addr & 0xF000)
    // {
    //     // BIOS (256b)/ROM0
    //     case 0x0000:
    //         if(MMU._inbios)
    //     {
    //         if(addr < 0x0100)
    //             return MMU._bios[addr];
    //         else if(Z80._r.pc == 0x0100)
    //             MMU._inbios = 0;
    //     }

    //     return MMU._rom[addr];

    //     // ROM0
    //     case 0x1000:
    //     case 0x2000:
    //     case 0x3000:
    //         return MMU._rom[addr];

    //     // ROM1 (unbanked) (16k)
    //     case 0x4000:
    //     case 0x5000:
    //     case 0x6000:
    //     case 0x7000:
    //         return MMU._rom[addr];

    //     // Graphics: VRAM (8k)
    //     case 0x8000:
    //     case 0x9000:
    //         return GPU._vram[addr & 0x1FFF];

    //     // External RAM (8k)
    //     case 0xA000:
    //     case 0xB000:
    //         return MMU._eram[addr & 0x1FFF];

    //     // Working RAM (8k)
    //     case 0xC000:
    //     case 0xD000:
    //         return MMU._wram[addr & 0x1FFF];

    //     // Working RAM shadow
    //     case 0xE000:
    //         return MMU._wram[addr & 0x1FFF];

    //     // Working RAM shadow, I/O, Zero-page RAM
    //     case 0xF000:
    //         switch(addr & 0x0F00)
    //     {
    //         // Working RAM shadow
    //         case 0x000: case 0x100: case 0x200: case 0x300:
    //         case 0x400: case 0x500: case 0x600: case 0x700:
    //         case 0x800: case 0x900: case 0xA00: case 0xB00:
    //         case 0xC00: case 0xD00:
    //             return MMU._wram[addr & 0x1FFF];

    //         // Graphics: object attribute memory
    //         // OAM is 160 bytes, remaining bytes read as 0
    //         case 0xE00:
    //             if(addr < 0xFEA0)
    //             return GPU._oam[addr & 0xFF];
    //         else
    //             return 0;

    //         // Zero-page
    //         case 0xF00:
    //             if(addr >= 0xFF80)
    //         {
    //             return MMU._zram[addr & 0x7F];
    //         }
    //         else
    //         {
    //             // I/O control handling
    //             // Currently unhandled
    //             return 0;
    //         }
    //     }
    // }

    fn set_initial(&mut self) {
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

    pub fn nop(&mut self, address: u16) -> u8 {
        address as u8
    }

    // Read 8-bit word from a given address
    pub fn rb(&mut self, address: u16) -> u8 {
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
            0xFF0F => self.nop(address),
            0xFF10..=0xFF3F => self.nop(address),
            0xFF4D => 0,
            0xFF40..=0xFF4F => self.nop(address),
            0xFF51..=0xFF55 => self.nop(address),
            0xFF68..=0xFF6B => self.nop(address),
            0xFF70 => self.nop(address),
            0xFF80..=0xFFFE => self.nop(address),
            0xFFFF => self.inte,
            _ => 0,
        }
    }

    // Read 16-bit word from a given address
    pub fn rw(&mut self, address: u16) -> u16 {
        (self.rb(address) as u16) | ((self.rb(address + 1) as u16) << 8)
    }

    // Write 8-bit byte to a given address
    pub fn wb(&mut self, address: u16, value: u8) {
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
    pub fn ww(&mut self, address: u16, value: u16) {
        self.wb(address, (value & 0xFF) as u8);
        self.wb(address + 1, (value >> 8) as u8);
    }
}
