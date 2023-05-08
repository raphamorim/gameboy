use crate::cpu::registers::Registers;
use crate::cpu::{data, ld, misc, stack};
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
    pub registers: Registers,
    pub ime: u32,
    pub halt: u32,
    pub stop: u32,
    pub memory: Mmu,
    delay: u32,
    ticks: u32,

    // To debug
    _executed_operations: Vec<u8>,
}

impl Cpu {
    pub fn new(memory: Mmu) -> Self {
        Cpu {
            registers: Registers::new(),
            memory: memory,
            ime: 0,
            halt: 0,
            stop: 0,
            ticks: 0,
            delay: 0,
            _executed_operations: Vec::new(),
        }
    }
    fn mut_find_or_insert<T: PartialEq>(vec: &mut Vec<T>, val: T) -> &mut T {
        if let Some(i) = vec.iter().position(|each| *each == val) {
            &mut vec[i]
        } else {
            vec.push(val);
            vec.last_mut().unwrap()
        }
    }
    pub fn debug(&mut self, op: u8) {
        Cpu::mut_find_or_insert(&mut self._executed_operations, op);
        // println!("{} {:#01x} {}", op, op, format!("{:?}", self.registers));
        println!("{} {}", op, format!("{:?}", self.registers));
        println!("{:?}", self._executed_operations);
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
    pub fn fz(&mut self, _i: u8, _cond: u8) {
        panic!("fix it");
        // self.registers.f = 0;
        // if !((i & 255) > 0) {
        //     // self.registers.f |= 128;
        // }
        // if cond > 0 {
        //     // self.registers.f |= 0x40;
        // } else {
        //     // self.registers.f |= 0;
        // }
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
            self.operation()
        } else {
            if self.stop != 0 && self.memory.speedswitch {
                self.memory.switch_speed();
                self.stop = 0;
            }
            1
        };

        // See http://bgb.bircd.org/pandocs.htm#interrupts
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

    fn operation(&mut self) -> u32 {
        let op = self.get_byte();
        // self.debug(op);
        match op {
            0x00 => 1,
            0x01 => {
                ld::bcnn(self);
                3
            }
            0x02 => {
                ld::bcm_a(self);
                2
            }
            0x03 => {
                data::incbc(self);
                2
            }
            0x04 => {
                data::incr_b(self);
                1
            }
            0x05 => {
                data::decr_b(self);
                1
            }
            0x06 => {
                ld::rr_b(self);
                2
            }
            0x07 => {
                misc::rlca(self);
                1
            }
            0x08 => {
                ld::mmsp(self);
                5
            }
            0x09 => {
                data::addhlbc(self);
                2
            }
            0x0A => {
                ld::abcm(self);
                2
            }
            0x0B => {
                data::decbc(self);
                2
            }
            0x0C => {
                data::incr_c(self);
                1
            }
            13 => {
                data::decr_c(self);
                1
            }
            0x0E => {
                ld::rr_c(self);
                2
            }
            0x0F => {
                misc::rrca(self);
                1
            }
            0x10 => {
                // stack::djnzn(self);
                self.stop = 1;
                1
            }
            0x11 => {
                ld::denn(self);
                3
            }
            18 => {
                ld::dem_a(self);
                2
            }
            0x13 => {
                data::incde(self);
                2
            }
            0x14 => {
                data::incr_d(self);
                1
            }
            0x15 => {
                data::decr_d(self);
                1
            }
            0x16 => {
                ld::rr_d(self);
                2
            }
            23 => {
                misc::rla(self);
                1
            }
            0x18 => {
                stack::jrn(self);
                3
            }
            0x19 => {
                data::addhlde(self);
                2
            }
            0x1A => {
                ld::adem(self);
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
            0x1D => {
                data::decr_e(self);
                1
            }
            0x1E => {
                ld::rr_e(self);
                2
            }
            31 => {
                misc::rra(self);
                1
            }
            0x20 => stack::jrnzn(self),
            0x21 => {
                ld::hlnn(self);
                3
            }
            0x22 => {
                ld::hlia(self);
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
                ld::rr_h(self);
                1
            }
            0x28 => stack::jrzn(self),
            41 => {
                data::addhlhl(self);
                1
            }
            0x2A => {
                ld::ahli(self);
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
                ld::rr_l(self);
                1
            }
            0x2F => {
                data::cpl(self);
                1
            }
            0x30 => stack::jrncn(self),
            0x31 => {
                ld::spnn(self);
                3
            }
            0x32 => {
                ld::hld_a(self);
                2
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
            0x36 => {
                ld::hlmn(self);
                3
            }
            0x37 => {
                data::scf(self);
                1
            }
            0x38 => stack::jrcn(self),
            57 => {
                data::addhlsp(self);
                1
            }
            58 => {
                ld::ahld(self);
                1
            }
            59 => {
                data::decsp(self);
                1
            }
            0x3C => {
                data::incr_a(self);
                1
            }
            0x3D => {
                data::decr_a(self);
                1
            }
            0x3E => {
                ld::rr_a(self);
                2
            }
            0x3F => {
                data::ccf(self);
                1
            }
            0x40 => {
                // ld::rr_bb(self);
                1
            }
            65 => {
                ld::rr_bc(self);
                1
            }
            66 => {
                ld::rr_bd(self);
                1
            }
            67 => {
                ld::rr_be(self);
                1
            }
            0x44 => {
                ld::rr_bh(self);
                1
            }
            69 => {
                ld::rr_bl(self);
                1
            }
            0x46 => {
                ld::r_hlm_b(self);
                2
            }
            0x47 => {
                ld::rr_ba(self);
                1
            }
            72 => {
                ld::rr_cb(self);
                1
            }
            0x49 => {
                // ld::rr_cc(self);
                1
            }
            74 => {
                ld::rr_cd(self);
                1
            }
            75 => {
                ld::rr_ce(self);
                1
            }
            76 => {
                ld::rr_ch(self);
                1
            }
            0x4D => {
                ld::rr_cl(self);
                1
            }
            0x4E => {
                ld::r_hlm_c(self);
                2
            }
            79 => {
                ld::rr_ca(self);
                1
            }
            80 => {
                ld::rr_db(self);
                1
            }
            81 => {
                ld::rr_dc(self);
                1
            }
            0x52 => {
                // ld::rr_dd(self);
                1
            }
            83 => {
                ld::rr_de(self);
                1
            }
            84 => {
                ld::rr_dh(self);
                1
            }
            85 => {
                ld::rr_dl(self);
                1
            }
            0x56 => {
                ld::r_hlm_d(self);
                2
            }
            87 => {
                ld::rr_da(self);
                1
            }
            88 => {
                ld::rr_eb(self);
                1
            }
            89 => {
                ld::rr_ec(self);
                1
            }
            90 => {
                ld::rr_ed(self);
                1
            }
            0x5B => {
                // ld::rr_ee(self);
                1
            }
            92 => {
                ld::rr_eh(self);
                1
            }
            93 => {
                ld::rr_el(self);
                1
            }
            0x5E => {
                ld::r_hlm_e(self);
                2
            }
            95 => {
                ld::rr_ea(self);
                1
            }
            96 => {
                ld::rr_hb(self);
                1
            }
            0x61 => {
                ld::rr_hc(self);
                1
            }
            98 => {
                ld::rr_hd(self);
                1
            }
            99 => {
                ld::rr_he(self);
                1
            }
            100 => {
                ld::rr_hh(self);
                1
            }
            101 => {
                ld::rr_hl(self);
                1
            }
            0x66 => {
                ld::r_hlm_h(self);
                2
            }
            103 => {
                ld::rr_ha(self);
                1
            }
            104 => {
                ld::rr_lb(self);
                1
            }
            105 => {
                ld::rr_lc(self);
                1
            }
            106 => {
                ld::rr_ld(self);
                1
            }
            107 => {
                ld::rr_le(self);
                1
            }
            108 => {
                ld::rr_lh(self);
                1
            }
            0x6D => {
                // ld::rr_ll(self);
                1
            }
            0x6E => {
                ld::r_hlm_l(self);
                2
            }
            111 => {
                ld::rr_la(self);
                1
            }
            112 => {
                ld::hlmr_b(self);
                1
            }
            113 => {
                ld::hlmr_c(self);
                1
            }
            114 => {
                ld::hlmr_d(self);
                1
            }
            115 => {
                ld::hlmr_e(self);
                1
            }
            116 => {
                ld::hlmr_h(self);
                1
            }
            117 => {
                ld::hlmr_l(self);
                1
            }
            0x76 => {
                self.halt = 1;
                1
            }
            119 => {
                ld::hlmr_a(self);
                2
            }
            0x78 => {
                ld::rr_ab(self);
                1
            }
            121 => {
                ld::rr_ac(self);
                1
            }
            122 => {
                ld::rr_ad(self);
                1
            }
            123 => {
                ld::rr_ae(self);
                1
            }
            124 => {
                ld::rr_ah(self);
                1
            }
            125 => {
                ld::rr_al(self);
                1
            }
            0x7E => {
                ld::r_hlm_a(self);
                2
            }
            0x7F => {
                // ld::rr_aa(self);
                1
            }
            0x80 => {
                data::addr_b(self);
                1
            }
            0x81 => {
                data::addr_c(self);
                1
            }
            0x82 => {
                data::addr_d(self);
                1
            }
            0x83 => {
                data::addr_e(self);
                1
            }
            0x84 => {
                data::addr_h(self);
                1
            }
            0x85 => {
                data::addr_l(self);
                1
            }
            0x86 => {
                data::addhl(self);
                2
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
            0x90 => {
                data::subr_b(self);
                1
            }
            0x91 => {
                data::subr_c(self);
                1
            }
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
            }
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
            0xA0 => {
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
            0xA8 => {
                data::xorr_b(self);
                1
            }
            0xA9 => {
                data::xorr_c(self);
                1
            }
            0xAA => {
                data::xorr_d(self);
                1
            }
            0xAB => {
                data::xorr_e(self);
                1
            }
            0xAC => {
                data::xorr_h(self);
                1
            }
            0xAD => {
                data::xorr_l(self);
                1
            }
            0xAE => {
                data::xorhl(self);
                2
            }
            0xAF => {
                data::xorr_a(self);
                1
            }
            0xB0 => {
                data::orr_b(self);
                1
            }
            0xB1 => {
                data::orr_c(self);
                1
            }
            0xB2 => {
                data::orr_d(self);
                1
            }
            0xB3 => {
                data::orr_e(self);
                1
            }
            0xB4 => {
                data::orr_h(self);
                1
            }
            0xB5 => {
                data::orr_l(self);
                1
            }
            0xB6 => {
                data::orhl(self);
                2
            }
            0xB7 => {
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
            0xC1 => {
                stack::popbc(self);
                3
            }
            0xC2 => stack::jpnznn(self),
            0xC3 => {
                stack::jpnn(self);
                4
            }
            196 => {
                stack::callnznn(self);
                1
            }
            0xC5 => {
                stack::pushbc(self);
                4
            }
            0xC6 => {
                data::addn(self);
                2
            }
            199 => {
                stack::rst(self, 0x00);
                1
            }
            0xC8 => stack::retz(self),
            0xC9 => {
                stack::ret(self);
                4
            }
            0xCA => stack::jpznn(self),
            0xCB => misc::cbmap(self),
            0xCC => stack::callznn(self),
            0xCD => {
                stack::callnn(self);
                6
            }
            0xCE => {
                data::adcn(self);
                2
            }
            0xCF => {
                stack::rst(self, 0x08);
                4
            }
            0xD0 => stack::retnc(self),
            0xD1 => {
                stack::popde(self);
                3
            }
            0xD2 => stack::jpncnn(self),
            212 => {
                stack::callncnn(self);
                1
            }
            0xD5 => {
                stack::pushde(self);
                4
            }
            0xD6 => {
                data::subn(self);
                2
            }
            215 => {
                stack::rst(self, 0x10);
                1
            }
            216 => {
                stack::retc(self);
                1
            }
            0xD9 => {
                stack::reti(self);
                4
            }
            218 => {
                stack::jpcnn(self);
                1
            }
            0xDC => stack::callcnn(self),
            0xDE => {
                data::sbcn(self);
                2
            }
            0xDF => {
                stack::rst(self, 0x18);
                4
            }
            0xE0 => {
                ld::ion_a(self);
                3
            }
            0xE1 => {
                stack::pophl(self);
                3
            }
            226 => {
                ld::ioca(self);
                1
            }
            0xE5 => {
                stack::pushhl(self);
                4
            }
            0xE6 => {
                data::andn(self);
                2
            }
            231 => {
                stack::rst(self, 0x20);
                1
            }
            232 => {
                data::addspn(self);
                1
            }
            0xE9 => {
                stack::jphl(self);
                1
            }
            0xEA => {
                ld::mm_a(self);
                4
            }
            0xEE => {
                data::orn(self);
                2
            }
            0xEF => {
                stack::rst(self, 0x28);
                4
            }
            0xF0 => {
                ld::aion(self);
                3
            }
            0xF1 => {
                stack::popaf(self);
                3
            }
            242 => {
                ld::aioc(self);
                2
            }
            0xF3 => {
                self.ime = 0;
                self.delay = 0;
                1
            }
            0xF5 => {
                stack::pushaf(self);
                4
            }
            246 => {
                data::xorn(self);
                2
            }
            247 => {
                stack::rst(self, 0x30);
                4
            }
            248 => {
                ld::hlspn(self);
                1
            }
            0xFA => {
                ld::amm(self);
                4
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
                stack::rst(self, 0x38);
                4
            }
            _ => {
                println!(
                    "Instruction at {:#01x} | {:#06x} | {} not implemented, stopping.",
                    op, op, op
                );
                self.stop = 1;
                panic!("{:#06x} not implemented", op);
            }
        }
    }
}
