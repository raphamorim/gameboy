use crate::cpu::registers::CpuFlag::{C, H, N, Z};
use crate::cpu::registers::{Clock, Registers};
use crate::cpu::{bit, data, ldrr, stack, swap};
use crate::mmu::mmu::{Mmu, Speed};

#[allow(dead_code)]
pub enum Interrupt {
    Vblank = 0x01,
    LCDStat = 0x02,
    Timer = 0x04,
    Serial = 0x08,
    Joypad = 0x10,
}

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers, // registers
    pub clock: Clock,
    pub ime: u32,
    pub halt: u32,
    pub stop: u32,
    pub memory: Mmu,
    delay: u32,
    ticks: u32,
}

impl Cpu {
    pub fn new(memory: Mmu) -> Self {
        Cpu {
            registers: Registers::new(),
            halt: 0,
            ticks: 0,
            memory: memory,
            stop: 0,
            ime: 0,
            delay: 0,
            clock: Clock { m: 0, t: 0 },
        }
    }
    pub fn get_byte(&mut self) -> u8 {
        let pc = self.memory.rb(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        pc
    }

    pub fn get_word(&mut self) -> u16 {
        let w = self.memory.rw(self.registers.pc);
        self.registers.pc += 2;
        w
    }
    pub fn fz(&mut self, i: u8, cond: u8) {
        self.registers.f = 0;
        if !((i & 255) > 0) {
            self.registers.f |= 128;
        }
        if cond > 0 {
            self.registers.f |= 0x40;
        } else {
            self.registers.f |= 0;
        }
    }
    fn sbcn(&mut self) {
        let b = self.get_byte();
        let c = if self.registers.getflag(C) { 1 } else { 0 };
        let a = self.registers.a;
        let r = a.wrapping_sub(b).wrapping_sub(c);
        self.registers.flag(Z, r == 0);
        self.registers.flag(H, (a & 0x0F) < (b & 0x0F) + c);
        self.registers.flag(N, true);
        self.registers.flag(C, (a as u16) < (b as u16) + (c as u16));
        self.registers.a = r;
    }
    fn ldr_hlm_b(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.registers.b = self.memory.rb(addr);
    }
    fn ldr_hlm_c(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.registers.c = self.memory.rb(addr);
    }
    fn ldr_hlm_d(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.registers.d = self.memory.rb(addr);
    }
    fn ldr_hlm_e(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.registers.e = self.memory.rb(addr);
    }
    fn ldr_hlm_h(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.registers.h = self.memory.rb(addr);
    }
    fn ldr_hlm_l(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.registers.l = self.memory.rb(addr);
    }
    fn ldr_hlm_a(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.registers.a = self.memory.rb(addr);
    }
    fn ld_hlmr_b(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, self.registers.b);
    }
    fn ld_hlmr_c(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, self.registers.c);
    }
    fn ld_hlmr_d(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, self.registers.d);
    }
    fn ld_hlmr_e(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, self.registers.e);
    }
    fn ld_hlmr_h(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, self.registers.h);
    }
    fn ld_hlmr_l(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, self.registers.l);
    }
    fn ld_hlmr_a(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, self.registers.a);
    }
    fn ld_hlmn(&mut self) {
        let value = self.memory.rb(self.registers.pc);
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, value);
        self.registers.pc += 1;
    }
    fn ld_bcm_a(&mut self) {
        let addr = ((self.registers.b as u16) << 8) + self.registers.c as u16;
        self.memory.wb(addr, self.registers.a);
    }
    fn ld_dem_a(&mut self) {
        let addr = ((self.registers.d as u16) << 8) + self.registers.e as u16;
        self.memory.wb(addr, self.registers.a);
    }
    fn ldmm_a(&mut self) {
        let addr = self.get_word();
        self.memory.wb(addr, self.registers.a);
    }
    fn ld_abcm(&mut self) {
        let addr = ((self.registers.b as u16) << 8) + self.registers.c as u16;
        self.registers.a = self.memory.rb(addr);
    }
    fn ld_adem(&mut self) {
        let addr = ((self.registers.d as u16) << 8) + self.registers.e as u16;
        self.registers.a = self.memory.rb(addr);
    }
    fn ld_amm(&mut self) {
        let addr = self.get_word();
        self.registers.a = self.memory.rb(addr);
    }
    fn ld_bcnn(&mut self) {
        let value = self.get_word();
        self.registers.b = (value >> 8) as u8;
        self.registers.c = (value & 0x00FF) as u8;
    }
    fn ld_denn(&mut self) {
        self.registers.e = self.memory.rb(self.registers.pc);
        self.registers.d = self.memory.rb(self.registers.pc + 1);
        self.registers.pc += 2;
    }
    fn ld_hlnn(&mut self) {
        let v = self.get_word();
        self.registers.h = (v >> 8) as u8;
        self.registers.l = (v & 0x00FF) as u8;
    }
    fn ld_spnn(&mut self) {
        self.registers.sp = self.get_word();
    }
    fn ld_hlia(&mut self) {
        let mut hl = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
        hl += 1;
        self.registers.h = (hl >> 8) as u8;
        self.registers.l = (hl & 0x00FF) as u8;
        self.memory.wb(hl, self.registers.a);
    }
    fn ld_ahli(&mut self) {
        let mut addr = ((self.registers.h as u16) << 8) | (self.registers.l as u16);
        self.registers.a = self.memory.rb(addr);
        addr += 1;
        self.registers.h = (addr >> 8) as u8;
        self.registers.l = (addr & 0x00FF) as u8;
    }
    fn ld_hld_a(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.memory.wb(addr, self.registers.a);
        self.registers.l = (self.registers.l - 1) & 255;
        if self.registers.l == 255 {
            self.registers.h = (self.registers.h - 1) & 255;
        }
    }
    fn ld_ahld(&mut self) {
        let addr = ((self.registers.h as u16) << 8) + self.registers.l as u16;
        self.registers.a = self.memory.rb(addr);
        self.registers.l = (self.registers.l - 1) & 255;
        if self.registers.l == 255 {
            self.registers.h = (self.registers.h - 1) & 255;
        }
    }
    fn ld_aion(&mut self) {
        let addr = 0xFF00 | self.get_byte() as u16;
        self.registers.a = self.memory.rb(addr);
    }
    fn ld_ion_a(&mut self) {
        let a = 0xFF00 | self.get_byte() as u16;
        self.memory.wb(a, self.registers.a);
    }
    fn ld_aioc(&mut self) {
        let addr: u16 = (0xFF00 + self.registers.c as u16).into();
        self.registers.a = self.memory.rb(addr);
    }
    fn ld_ioca(&mut self) {
        let addr: u16 = (0xFF00 + self.registers.c as u16).into();
        self.memory.wb(addr, self.registers.a);
    }
    fn ld_hlspn(&mut self) {
        let mut i: u8 = self.memory.rb(self.registers.pc);
        if i > 127 {
            // i=-(!i+1);
            // i = (i - (self.registers.sp as u8)) + 1;
            i = 1;
        }
        self.registers.pc += 1;
        i += self.registers.sp as u8;
        // self.registers.h = ((i >> 8) as u8) & 255;
        self.registers.h = i;
        self.registers.l = i & 255;
    }
    fn ldmmsp(&mut self) {
        let addr = self.get_word();
        self.memory.ww(addr, self.registers.sp);
    }
    fn rla(&mut self) {
        let mut ci = 0;
        let mut co = 0;
        if (self.registers.f & 0x10) > 0 {
            ci = 1;
        }
        if (self.registers.a & 0x80) > 0 {
            co = 0x10;
        }
        self.registers.a = (self.registers.a << 1) + ci;
        self.registers.f = (self.registers.f & 0xEF) + co;
    }
    fn rlca(&mut self) {
        let mut ci = 0;
        let mut co = 0;
        if (self.registers.a & 0x80) > 0 {
            ci = 1;
            co = 0x10;
        }

        self.registers.a = (self.registers.a << 1) + ci;
        self.registers.f = (self.registers.f & 0xEF) + co;
    }
    fn rra(&mut self) {
        let mut ci = 0;
        let mut co = 0;
        if (self.registers.f & 0x10) > 0 {
            ci = 0x80;
        }
        if (self.registers.a & 1) > 0 {
            co = 0x10;
        }

        self.registers.a = (self.registers.a >> 1) + ci;
        self.registers.f = (self.registers.f & 0xEF) + co;
    }
    fn rrca(&mut self) {
        let mut ci = 0;
        let mut co = 0;
        if (self.registers.a & 1) > 0 {
            ci = 0x80;
            co = 0x10;
        }

        self.registers.a = (self.registers.a >> 1) + ci;
        self.registers.f = (self.registers.f & 0xEF) + co;
    }

    pub fn exec(&mut self) -> u32 {
        match self.delay {
            0 => {}
            1 => {
                self.delay = 0;
                self.ime = 1;
            }
            2 => {
                self.delay = 1;
            }
            _ => {}
        }

        let mut ticks = if self.halt == 0 && self.stop == 0 {
            self.exec_current_operation() as u32
        } else {
            if self.stop != 0 && self.memory.speedswitch {
                self.memory.switch_speed();
                self.stop = 0;
            }
            1
        };

        // See http://nocash.emubase.de/pandocs.htm#interrupts
        if self.ime != 0 || self.halt != 0 {
            let ints = self.memory.if_ & self.memory.ie_;

            if ints != 0 {
                let i = ints.trailing_zeros();
                if self.ime != 0 {
                    self.memory.if_ &= !(1 << (i as u32));
                }
                self.ime = 0;
                self.halt = 0;
                self.stop = 0;
                match i {
                    0 => {
                        stack::rst(self, 0x40);
                    }
                    1 => {
                        stack::rst(self, 0x48);
                    }
                    2 => {
                        stack::rst(self, 0x50);
                    }
                    3 => {
                        stack::rst(self, 0x58);
                    }
                    4 => {
                        stack::rst(self, 0x60);
                    }
                    _ => {}
                }
                ticks += 1;
            }
        }

        match self.memory.speed {
            Speed::Normal => {
                // ticks *= 4;
            }
            Speed::Double => {
                ticks *= 2;
            }
        }
        self.ticks += ticks;
        return ticks;
    }

    fn exec_current_operation(&mut self) -> u32 {
        let op = self.get_byte();
        println!("{} {:#01x} {}", op, op, format!("{:?}", self.registers));
        match op {
            0x00 => 1,
            0x01 => {
                self.ld_bcnn();
                3
            }
            0x02 => {
                self.ld_bcm_a();
                2
            }
            0x03 => {
                data::incbc(self);
                2
            } // todo
            0x04 => {
                data::incr_b(self);
                1
            }
            0x05 => {
                data::decr_b(self);
                1
            }
            0x06 => {
                ldrr::b(self);
                2
            }
            0x07 => {
                self.rlca();
                1
            }
            0x08 => {
                self.ldmmsp();
                5
            }
            0x09 => {
                data::addhlbc(self);
                1
            }
            0x0A => {
                self.ld_abcm();
                2
            }
            0x0B => {
                data::decbc(self);
                2
            }
            12 => {
                data::incr_c(self);
                1
            }
            13 => {
                data::decr_c(self);
                1
            }
            0x0E => {
                ldrr::c(self);
                2
            }
            0x0F => {
                self.rrca();
                1
            }
            0x10 => {
                // stack::djnzn(self);
                self.stop = 1;
                1
            } // 16
            0x11 => {
                self.ld_denn();
                3
            } // 17
            18 => {
                self.ld_dem_a();
                2
            }
            19 => {
                data::incde(self);
                2
            }
            20 => {
                data::incr_d(self);
                1
            }
            0x15 => {
                data::decr_d(self);
                1
            } // 21
            0x16 => {
                ldrr::d(self);
                2
            } // 22
            23 => {
                self.rla();
                1
            }
            0x18 => {
                stack::jrn(self);
                3
            } // 24
            0x19 => {
                data::addhlde(self);
                2
            } // 25
            26 => {
                self.ld_adem();
                2
            }
            27 => {
                data::decde(self);
                2
            }
            28 => {
                data::incr_e(self);
                1
            }
            29 => {
                data::decr_e(self);
                1
            } // 29
            0x1E => {
                ldrr::e(self);
                2
            } // 30
            31 => {
                self.rra();
                1
            }
            0x20 => {
                stack::jrnzn(self);
                3
            } // todo
            0x21 => {
                self.ld_hlnn();
                3
            }
            0x22 => {
                self.ld_hlia();
                2
            }
            0x23 => {
                data::inchl(self);
                2
            }
            36 => {
                data::incr_h(self);
                1
            }
            37 => {
                data::decr_h(self);
                1
            }
            38 => {
                ldrr::h(self);
                1
            }
            0x28 => {
                stack::jrzn(self);
                3
            }
            41 => {
                data::addhlhl(self);
                1
            }
            0x2a => {
                self.ld_ahli();
                2
            }
            43 => {
                data::dechl(self);
                1
            }
            44 => {
                data::incr_l(self);
                1
            }
            45 => {
                data::decr_l(self);
                1
            }
            46 => {
                ldrr::l(self);
                1
            }
            0x2f => {
                data::cpl(self);
                1
            }
            48 => {
                stack::jrncn(self);
                1
            }
            0x31 => {
                self.ld_spnn();
                3
            }
            50 => {
                self.ld_hld_a();
                1
            }
            51 => {
                data::incsp(self);
                1
            }
            52 => {
                data::inchlm(self);
                1
            }
            0x35 => {
                data::dechlm(self);
                3
            }
            54 => {
                self.ld_hlmn();
                1
            }
            0x37 => {
                data::scf(self);
                1
            }
            0x38 => {
                stack::jrcn(self); // todo
                3
            }
            57 => {
                data::addhlsp(self);
                1
            }
            58 => {
                self.ld_ahld();
                1
            }
            59 => {
                data::decsp(self);
                1
            }
            0x3c => {
                data::incr_a(self);
                1
            }
            61 => {
                data::decr_a(self);
                1
            }
            0x3e => {
                ldrr::a(self);
                2
            }
            0x3F => {
                data::ccf(self);
                1
            }
            64 => {
                ldrr::bb(self);
                1
            }
            65 => {
                ldrr::bc(self);
                1
            }
            66 => {
                ldrr::bd(self);
                1
            }
            67 => {
                ldrr::be(self);
                1
            }
            0x44 => {
                ldrr::bh(self);
                2
            }
            69 => {
                ldrr::bl(self);
                1
            }
            70 => {
                self.ldr_hlm_b();
                1
            }
            0x47 => {
                ldrr::ba(self);
                2
            }
            72 => {
                ldrr::cb(self);
                1
            }
            73 => {
                // ldrr::cc(self);
                1
            }
            74 => {
                ldrr::cd(self);
                1
            }
            75 => {
                ldrr::ce(self);
                1
            }
            76 => {
                ldrr::ch(self);
                1
            }
            77 => {
                ldrr::cl(self);
                2
            }
            78 => {
                self.ldr_hlm_c();
                3
            }
            79 => {
                ldrr::ca(self);
                1
            }
            80 => {
                ldrr::db(self);
                1
            }
            81 => {
                ldrr::dc(self);
                1
            }
            82 => {
                // ldrr::dd(self);
                1
            }
            83 => {
                ldrr::de(self);
                1
            }
            84 => {
                ldrr::dh(self);
                1
            }
            85 => {
                ldrr::dl(self);
                1
            }
            86 => {
                self.ldr_hlm_d();
                1
            }
            87 => {
                ldrr::da(self);
                1
            }
            88 => {
                ldrr::eb(self);
                1
            }
            89 => {
                ldrr::ec(self);
                1
            }
            90 => {
                ldrr::ed(self);
                1
            }
            91 => {
                // ldrr::ee(self);
                1
            }
            92 => {
                ldrr::eh(self);
                1
            }
            93 => {
                ldrr::el(self);
                1
            }
            94 => {
                self.ldr_hlm_e();
                1
            }
            95 => {
                ldrr::ea(self);
                1
            }
            96 => {
                ldrr::hb(self);
                1
            }
            97 => {
                ldrr::hc(self);
                1
            }
            98 => {
                ldrr::hd(self);
                1
            }
            99 => {
                ldrr::he(self);
                1
            }
            100 => {
                ldrr::hh(self);
                1
            }
            101 => {
                ldrr::hl(self);
                1
            }
            102 => {
                self.ldr_hlm_h();
                1
            }
            103 => {
                ldrr::ha(self);
                1
            }
            104 => {
                ldrr::lb(self);
                1
            }
            105 => {
                ldrr::lc(self);
                1
            }
            106 => {
                ldrr::ld(self);
                1
            }
            107 => {
                ldrr::le(self);
                1
            }
            108 => {
                ldrr::lh(self);
                1
            }
            109 => {
                // ldrr::ll(self);
                1
            }
            110 => {
                self.ldr_hlm_l();
                1
            }
            111 => {
                ldrr::la(self);
                1
            }
            112 => {
                self.ld_hlmr_b();
                1
            }
            113 => {
                self.ld_hlmr_c();
                1
            }
            114 => {
                self.ld_hlmr_d();
                1
            }
            115 => {
                self.ld_hlmr_e();
                1
            }
            116 => {
                self.ld_hlmr_h();
                1
            }
            117 => {
                self.ld_hlmr_l();
                1
            }
            0x76 => {
                self.halt = 1;
                1
            }
            119 => {
                self.ld_hlmr_a();
                1
            }
            120 => {
                ldrr::ab(self);
                1
            }
            121 => {
                ldrr::ac(self);
                1
            }
            122 => {
                ldrr::ad(self);
                1
            }
            123 => {
                ldrr::ae(self);
                1
            }
            124 => {
                ldrr::ah(self);
                1
            }
            125 => {
                ldrr::al(self);
                1
            }
            126 => {
                self.ldr_hlm_a();
                1
            }
            0x7F => {
                // ldrr::aa(self);
                1
            }
            128 => {
                data::addr_b(self);
                1
            }
            129 => {
                data::addr_c(self);
                1
            }
            130 => {
                data::addr_d(self);
                1
            }
            131 => {
                data::addr_e(self);
                1
            }
            132 => {
                data::addr_h(self);
                1
            }
            133 => {
                data::addr_l(self);
                1
            }
            134 => {
                data::addhl(self);
                1
            }
            135 => {
                data::addr_a(self);
                1
            }
            136 => {
                data::adcr_b(self);
                1
            }
            137 => {
                data::adcr_c(self);
                1
            }
            138 => {
                data::adcr_d(self);
                1
            }
            139 => {
                data::adcr_e(self);
                1
            }
            140 => {
                data::adcr_h(self);
                1
            }
            141 => {
                data::adcr_l(self);
                1
            }
            142 => {
                data::adchl(self);
                1
            }
            143 => {
                data::adcr_a(self);
                1
            }
            144 => {
                data::subr_b(self);
                1
            }
            0x91 => {
                data::subr_c(self);
                1
            } // 145
            146 => {
                data::subr_d(self);
                1
            }
            147 => {
                data::subr_e(self);
                1
            }
            148 => {
                data::subr_h(self);
                1
            }
            149 => {
                data::subr_l(self);
                1
            }
            150 => {
                data::subhl(self);
                1
            }
            0x97 => {
                data::subr_a(self);
                1
            } // 151
            152 => {
                data::sbcr_b(self);
                1
            }
            153 => {
                data::sbcr_c(self);
                1
            }
            154 => {
                data::sbcr_d(self);
                1
            }
            155 => {
                data::sbcr_e(self);
                1
            }
            156 => {
                data::sbcr_h(self);
                1
            }
            157 => {
                data::sbcr_l(self);
                1
            }
            158 => {
                data::sbchl(self);
                1
            }
            159 => {
                data::sbcr_a(self);
                1
            }
            160 => {
                data::andr_b(self);
                1
            }
            161 => {
                data::andr_c(self);
                1
            }
            162 => {
                data::andr_d(self);
                1
            }
            163 => {
                data::andr_e(self);
                1
            }
            164 => {
                data::andr_h(self);
                1
            }
            165 => {
                data::andr_l(self);
                1
            }
            166 => {
                data::andhl(self);
                1
            }
            167 => {
                data::andr_a(self);
                1
            }
            168 => {
                data::xorr_b(self);
                1
            }
            169 => {
                data::xorr_c(self);
                1
            }
            170 => {
                data::xorr_d(self);
                1
            }
            171 => {
                data::xorr_e(self);
                1
            }
            172 => {
                data::xorr_h(self);
                1
            }
            173 => {
                data::xorr_l(self);
                1
            }
            174 => {
                data::xorhl(self);
                1
            }
            175 => {
                data::xorr_a(self);
                1
            }
            176 => {
                data::orr_b(self);
                1
            }
            177 => {
                data::orr_c(self);
                1
            }
            178 => {
                data::orr_d(self);
                1
            }
            179 => {
                data::orr_e(self);
                1
            }
            180 => {
                data::orr_h(self);
                1
            }
            181 => {
                data::orr_l(self);
                1
            }
            182 => {
                data::orhl(self);
                1
            }
            183 => {
                data::orr_a(self);
                1
            }
            184 => {
                data::cpr_b(self);
                1
            }
            185 => {
                data::cpr_c(self);
                1
            }
            186 => {
                data::cpr_d(self);
                1
            }
            187 => {
                data::cpr_e(self);
                1
            }
            188 => {
                data::cpr_h(self);
                1
            }
            189 => {
                data::cpr_l(self);
                1
            }
            190 => {
                data::cphl(self);
                1
            }
            191 => {
                data::cpr_a(self);
                1
            }
            192 => {
                stack::retnz(self);
                1
            }
            193 => {
                stack::popbc(self);
                1
            }
            0xc2 => {
                stack::jpnznn(self);
                1
            }
            0xc3 => {
                stack::jpnn(self);
                1
            }
            196 => {
                stack::callnznn(self);
                1
            }
            197 => {
                stack::pushbc(self);
                1
            }
            0xC6 => {
                data::addn(self);
                2
            }
            199 => {
                stack::rst00(self);
                1
            }
            200 => {
                stack::retz(self);
                1
            }
            201 => {
                stack::ret(self);
                1
            }
            202 => {
                stack::jpznn(self);
                1
            }
            0xcb => self.cbmap(),
            204 => {
                stack::callznn(self);
                1
            }
            0xcd => {
                stack::callnn(self);
                2
            }
            206 => {
                data::adcn(self);
                1
            }
            207 => {
                stack::rst08(self);
                1
            }
            208 => {
                stack::retnc(self);
                1
            }
            209 => {
                stack::popde(self);
                1
            }
            210 => {
                stack::jpncnn(self);
                1
            }
            212 => {
                stack::callncnn(self);
                1
            }
            213 => {
                stack::pushde(self);
                1
            }
            0xD6 => {
                data::subn(self);
                2
            }
            215 => {
                stack::rst10(self);
                1
            }
            216 => {
                stack::retc(self);
                1
            }
            217 => {
                stack::reti(self);
                1
            }
            218 => {
                stack::jpcnn(self);
                1
            }
            220 => {
                stack::callcnn(self);
                1
            }
            0xDE => {
                self.sbcn();
                4
            }
            223 => {
                stack::rst18(self);
                1
            }
            0xe0 => {
                self.ld_ion_a();
                3
            }
            225 => {
                stack::pophl(self);
                1
            }
            226 => {
                self.ld_ioca();
                1
            }
            229 => {
                stack::pushhl(self);
                1
            }
            0xe6 => {
                data::andn(self);
                4
            }
            231 => {
                stack::rst20(self);
                1
            }
            232 => {
                data::addspn(self);
                1
            }
            233 => {
                stack::jphl(self);
                1
            }
            0xea => {
                self.ldmm_a();
                1
            }
            238 => {
                data::orn(self);
                1
            }
            239 => {
                stack::rst28(self);
                1
            }
            0xf0 => {
                self.ld_aion();
                1
            }
            241 => {
                stack::popaf(self);
                1
            }
            242 => {
                self.ld_aioc();
                1
            }
            0xF3 => {
                self.ime = 0;
                self.delay = 0;
                1
            }
            245 => {
                stack::pushaf(self);
                1
            }
            246 => {
                data::xorn(self);
                1
            }
            247 => {
                stack::rst30(self);
                1
            }
            248 => {
                self.ld_hlspn();
                1
            }
            250 => {
                self.ld_amm();
                1
            }
            0xFB => {
                if self.delay == 2 || self.memory.rb(self.registers.pc) == 0x76 {
                    self.delay = 1;
                } else {
                    self.delay = 2;
                }
                self.ime = 1;
                1
            }
            0xFE => {
                data::cpn(self);
                2
            }
            0xFF => {
                stack::rst38(self);
                4
            }
            _ => {
                println!(
                    "Instruction at {:#01x} | {:#06x} | {} not implemented, stopping.",
                    op, op, op
                );
                self.stop = 1;
                0
            } // 255
        }
    }

    fn cbmap(&mut self) -> u32 {
        let op = self.get_byte();
        match op {
            // CB00
            // 0 => self.RLCr_b,
            // 1 => self.RLCr_c,
            // 2 => self.RLCr_d,
            // 3 => self.RLCr_e,
            // 4 => self.RLCr_h,
            // 5 => self.RLCr_l,
            // 6 => self.RLCHL,
            // 7 => self.RLCr_a,
            // 8 => self.RRCr_b,
            // 9 => self.RRCr_c,
            // 10 => self.RRCr_d,
            // 11 => self.RRCr_e,
            // 12 => self.RRCr_h,
            // 13 => self.RRCr_l,
            // 14 => self.RRCHL,
            // 15 => self.RRCr_a,
            // 16 => self.RLr_b,
            // 17 => self.RLr_c,
            18 => {
                let a = self.registers.d;
                let c = a & 0x80 == 0x80;
                let r = (a << 1) | (if c { 1 } else { 0 });
                
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                self.registers.flag(Z, r == 0);
                self.registers.flag(C, c);

                self.registers.d = r;
                2
            },
            // 19 => self.RLr_e,
            // 20 => self.RLr_h,
            // 21 => self.RLr_l,
            // 22 => self.RLHL,
            // 23 => self.RLr_a,
            // 24 => self.RRr_b,
            // 25 => self.RRr_c,
            // 26 => self.RRr_d,
            // 27 => self.RRr_e,
            28 => { 
                let a = self.registers.h;
                let c = a & 0x01 == 0x01;
                let r = (a >> 1) | (if self.registers.getflag(C) { 0x80 } else { 0 });
                
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                self.registers.flag(Z, r == 0);
                self.registers.flag(C, c);

                self.registers.h = r; 
                2 
            },
            29 => {
                let a = self.registers.l;
                let c = a & 0x01 == 0x01;
                let r = (a >> 1) | (if self.registers.getflag(C) { 0x80 } else { 0 });
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                self.registers.flag(Z, r == 0);
                self.registers.flag(C, c);

                self.registers.l = r; 
                2
            },
            // 30 => self.RRHL,
            // 31 => self.RRr_a,
            // 32 => self.SLAr_b,
            // 33 => self.SLAr_c,
            // 34 => self.SLAr_d,
            35 => {
                let a = self.registers.e; 
                let c = a & 0x80 == 0x80;
                let r = a << 1;

                self.registers.flag(H, false);
                self.registers.flag(N, false);
                self.registers.flag(Z, r == 0);
                self.registers.flag(C, c);

                self.registers.e = r;
                2
            },
            // 36 => self.SLAr_h,
            // 37 => self.SLAr_l,
            39 => {
                let a = self.registers.a;
                let c = a & 0x80 == 0x80;
                let r = a << 1;
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                self.registers.flag(Z, r == 0);
                self.registers.flag(C, c);
                self.registers.a = r;
                2
            },
            // 40 => self.SRAr_b,
            // 41 => self.SRAr_c,
            // 42 => self.SRAr_d,
            // 43 => self.SRAr_e,
            // 44 => self.SRAr_h,
            // 45 => self.SRAr_l,
            // 47 => { self.SRAr_a; 2 },
            48 => {
                swap::r_b(self);
                2
            }
            49 => {
                swap::r_c(self);
                2
            }
            50 => {
                swap::r_d(self);
                2
            }
            51 => {
                swap::r_e(self);
                2
            }
            52 => {
                swap::r_h(self);
                2
            }
            53 => {
                swap::r_l(self);
                2
            }
            55 => {
                swap::r_a(self);
                2
            }
            // 56 => { self.SRLr_b; 2 },
            // 57 => { self.SRLr_c; 2 },
            // 58 => { self.SRLr_d; 2 },
            // 59 => { self.SRLr_e; 2 },
            60 => { 
                let a = self.registers.h;
                let c = a & 0x01 == 0x01;
                let r = a >> 1;
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                self.registers.flag(Z, r == 0);
                self.registers.flag(C, c);
                self.registers.h = r;

                2
            },
            61 => { 
                let a = self.registers.l;
                let c = a & 0x01 == 0x01;
                let r = a >> 1;
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                self.registers.flag(Z, r == 0);
                self.registers.flag(C, c);
                self.registers.l = r;
                2 
            },
            63 => { 
                let a = self.registers.a;
                let c = a & 0x01 == 0x01;
                let r = a >> 1;
                
                self.registers.flag(H, false);
                self.registers.flag(N, false);
                self.registers.flag(Z, r == 0);
                self.registers.flag(C, c);

                self.registers.a = r;
                2 
            },
            64 => {
                bit::bit0b(self);
                2
            }
            65 => {
                bit::bit0c(self);
                2
            }
            66 => {
                bit::bit0d(self);
                2
            }
            67 => {
                bit::bit0e(self);
                2
            }
            68 => {
                bit::bit0h(self);
                2
            }
            69 => {
                bit::bit0l(self);
                2
            }
            70 => {
                bit::bit0m(self);
                2
            }
            71 => {
                bit::bit0a(self);
                2
            }
            72 => {
                bit::bit1b(self);
                2
            }
            73 => {
                bit::bit1c(self);
                2
            }
            74 => {
                bit::bit1d(self);
                2
            }
            75 => {
                bit::bit1e(self);
                2
            }
            76 => {
                bit::bit1h(self);
                2
            }
            77 => {
                bit::bit1l(self);
                2
            }
            78 => {
                bit::bit1m(self);
                2
            }
            79 => {
                bit::bit1a(self);
                2
            }
            80 => {
                bit::bit2b(self);
                2
            }
            81 => {
                bit::bit2c(self);
                2
            }
            82 => {
                bit::bit2d(self);
                2
            }
            83 => {
                bit::bit2e(self);
                2
            }
            84 => {
                bit::bit2h(self);
                2
            }
            85 => {
                bit::bit2l(self);
                2
            }
            86 => {
                bit::bit2m(self);
                2
            }
            87 => {
                bit::bit2a(self);
                2
            }
            88 => {
                bit::bit3b(self);
                2
            }
            89 => {
                bit::bit3c(self);
                2
            }
            90 => {
                bit::bit3d(self);
                2
            }
            91 => {
                bit::bit3e(self);
                2
            }
            92 => {
                bit::bit3h(self);
                2
            }
            93 => {
                bit::bit3l(self);
                2
            }
            94 => {
                bit::bit3m(self);
                2
            }
            95 => {
                bit::bit3a(self);
                2
            }
            96 => {
                bit::bit4b(self);
                2
            }
            97 => {
                bit::bit4c(self);
                2
            }
            98 => {
                bit::bit4d(self);
                2
            }
            99 => {
                bit::bit4e(self);
                2
            }
            100 => {
                bit::bit4h(self);
                2
            }
            101 => {
                bit::bit4l(self);
                2
            }
            102 => {
                bit::bit4m(self);
                2
            }
            103 => {
                bit::bit4a(self);
                2
            }
            104 => {
                bit::bit5b(self);
                2
            }
            105 => {
                bit::bit5c(self);
                2
            }
            106 => {
                bit::bit5d(self);
                2
            }
            107 => {
                bit::bit5e(self);
                2
            }
            108 => {
                bit::bit5h(self);
                2
            }
            109 => {
                bit::bit5l(self);
                2
            }
            110 => {
                bit::bit5m(self);
                2
            }
            111 => {
                bit::bit5a(self);
                2
            }
            112 => {
                bit::bit6b(self);
                2
            }
            113 => {
                bit::bit6c(self);
                2
            }
            114 => {
                bit::bit6d(self);
                2
            }
            115 => {
                bit::bit6e(self);
                2
            }
            116 => {
                bit::bit6h(self);
                2
            }
            117 => {
                bit::bit6l(self);
                2
            }
            118 => {
                bit::bit6m(self);
                2
            }
            119 => {
                bit::bit6a(self);
                2
            }
            120 => {
                bit::bit7b(self);
                2
            }
            121 => {
                bit::bit7c(self);
                2
            }
            122 => {
                bit::bit7d(self);
                2
            }
            123 => {
                bit::bit7e(self);
                2
            }
            124 => {
                bit::bit7h(self);
                2
            }
            125 => {
                bit::bit7l(self);
                2
            }
            126 => {
                bit::bit7m(self);
                2
            }
            127 => {
                bit::bit7a(self);
                2
            }
            _ => {
                println!(
                    "cbmap -> Instruction at {:#01x} | {} not implemented, stopping.",
                    op, op
                );
                self.stop = 1;
                0
            }
        }
    }
}
